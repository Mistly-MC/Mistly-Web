//! Demandes d'ami entre joueurs Mistly (page Friends du launcher).
//!
//! Le launcher envoie le jeton Mistly que Mistly Client a obtenu en prouvant son compte
//! (`/api/client/login`) : personne ne peut donc demander ni accepter au nom d'un autre.
//!
//! - `GET /api/client/friends` (jeton) → `{ "incoming": [{ "uuid", "name" }], "outgoing": […],
//!   "friends": […] }`
//! - `POST /api/client/friends/request` (jeton) `{ "uuid": "…" }` : demande à ce joueur ; s'il
//!   avait déjà demandé, c'est accepté tout de suite.
//! - `POST /api/client/friends/respond` (jeton) `{ "uuid": "…", "accept": true }` : accepter ou
//!   refuser une demande reçue.
//! - `POST /api/client/friends/remove` (jeton) `{ "uuid": "…" }` : retirer un ami ou annuler une
//!   demande envoyée.
//!
//! Tout est gardé dans `amis.json` (dans `MISTY_CACHE_DIR`) : seulement des UUID et des dates.

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::AppState;
use crate::client::{avec_tirets, uuid_normalise};
use crate::registre::maintenant;

/// Demandes en attente qu'un joueur peut avoir envoyées (protection contre le spam).
const MAX_ENVOYEES: usize = 100;
/// Amis par joueur.
const MAX_AMIS: usize = 1000;
/// Une demande sans réponse est oubliée au bout de 30 jours.
const DUREE_DEMANDE: u64 = 30 * 24 * 3600;
/// Au-delà, plus aucune demande n'est acceptée (protection mémoire).
const MAX_TOTAL: usize = 500_000;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Demande {
    de: String,
    a: String,
    date: u64,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
#[serde(default)]
struct Base {
    demandes: Vec<Demande>,
    /// Paires d'amis (UUID en minuscules sans tirets, le plus petit d'abord)
    amis: Vec<(String, String)>,
}

pub struct Amis {
    base: Mutex<Base>,
    fichier: Option<PathBuf>,
}

/// Ce que `demander` a fait.
#[derive(Debug, PartialEq)]
enum Resultat {
    Envoyee,
    Acceptee,
    DejaAmis,
    Refusee,
}

fn paire(a: &str, b: &str) -> (String, String) {
    if a <= b { (a.to_string(), b.to_string()) } else { (b.to_string(), a.to_string()) }
}

impl Amis {
    pub fn new() -> Self {
        let fichier = (!cfg!(test)).then(|| std::env::var_os("MISTY_CACHE_DIR").map(|d| PathBuf::from(d).join("amis.json"))).flatten();
        let base = fichier.as_ref().and_then(|f| std::fs::read(f).ok()).and_then(|o| serde_json::from_slice(&o).ok()).unwrap_or_default();
        Self { base: Mutex::new(base), fichier }
    }

    fn ecrire(&self, base: &Base) {
        let Some(f) = &self.fichier else { return };
        let Ok(texte) = serde_json::to_vec(base) else { return };
        if let Some(d) = f.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let temporaire = f.with_extension("tmp");
        if std::fs::write(&temporaire, texte).is_ok() {
            let _ = std::fs::rename(&temporaire, f);
        }
    }

    fn nettoyer(base: &mut Base) {
        let t = maintenant();
        base.demandes.retain(|d| t.saturating_sub(d.date) < DUREE_DEMANDE);
    }

    fn demander(&self, de: &str, a: &str) -> Resultat {
        if de == a {
            return Resultat::Refusee;
        }
        let mut base = self.base.lock().unwrap();
        Self::nettoyer(&mut base);
        let p = paire(de, a);
        if base.amis.contains(&p) {
            return Resultat::DejaAmis;
        }
        // il avait déjà demandé : accepté
        if let Some(i) = base.demandes.iter().position(|d| d.de == a && d.a == de) {
            base.demandes.remove(i);
            base.amis.push(p);
            self.ecrire(&base);
            return Resultat::Acceptee;
        }
        if base.demandes.iter().any(|d| d.de == de && d.a == a) {
            return Resultat::Envoyee;
        }
        let envoyees = base.demandes.iter().filter(|d| d.de == de).count();
        let amis = base.amis.iter().filter(|(x, y)| x == de || y == de).count();
        if envoyees >= MAX_ENVOYEES || amis >= MAX_AMIS || base.demandes.len() + base.amis.len() >= MAX_TOTAL {
            return Resultat::Refusee;
        }
        base.demandes.push(Demande { de: de.to_string(), a: a.to_string(), date: maintenant() });
        self.ecrire(&base);
        Resultat::Envoyee
    }

    /// Réponse à la demande de `de` reçue par `moi`. false : pas de demande.
    fn repondre(&self, moi: &str, de: &str, accepter: bool) -> bool {
        let mut base = self.base.lock().unwrap();
        let Some(i) = base.demandes.iter().position(|d| d.de == de && d.a == moi) else {
            return false;
        };
        base.demandes.remove(i);
        if accepter && !base.amis.contains(&paire(moi, de)) {
            base.amis.push(paire(moi, de));
        }
        self.ecrire(&base);
        true
    }

    /// Retire un ami, ou annule une demande dans un sens comme dans l'autre.
    fn retirer(&self, moi: &str, autre: &str) {
        let mut base = self.base.lock().unwrap();
        let p = paire(moi, autre);
        base.amis.retain(|x| *x != p);
        base.demandes.retain(|d| !((d.de == moi && d.a == autre) || (d.de == autre && d.a == moi)));
        self.ecrire(&base);
    }

    /// (reçues, envoyées, amis) de ce joueur.
    fn liste(&self, moi: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
        let mut base = self.base.lock().unwrap();
        Self::nettoyer(&mut base);
        let recues = base.demandes.iter().filter(|d| d.a == moi).map(|d| d.de.clone()).collect();
        let envoyees = base.demandes.iter().filter(|d| d.de == moi).map(|d| d.a.clone()).collect();
        let amis = base
            .amis
            .iter()
            .filter_map(|(x, y)| if x == moi { Some(y.clone()) } else if y == moi { Some(x.clone()) } else { None })
            .collect();
        (recues, envoyees, amis)
    }
}

/// Le compte du jeton, ou la réponse à rendre (trop de requêtes, jeton inconnu).
fn compte(state: &AppState, entetes: &HeaderMap) -> Result<String, Response> {
    let c = &state.client;
    if !c.autorise_public(&crate::presence::adresse(entetes)) {
        return Err(StatusCode::TOO_MANY_REQUESTS.into_response());
    }
    c.compte_public(entetes).ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())
}

fn uuid_du_corps(corps: &Value) -> Result<String, Response> {
    corps["uuid"].as_str().and_then(uuid_normalise).ok_or_else(|| (StatusCode::BAD_REQUEST, Json(json!({ "error": "bad request" }))).into_response())
}

fn joueurs(state: &AppState, uuids: Vec<String>) -> Vec<Value> {
    uuids
        .into_iter()
        .map(|u| {
            let nom = state.registre.fiche(&u).and_then(|f| f.name).unwrap_or_default();
            json!({ "uuid": avec_tirets(&u), "name": nom })
        })
        .collect()
}

pub async fn liste(State(state): State<Arc<AppState>>, entetes: HeaderMap) -> Response {
    let moi = match compte(&state, &entetes) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let (recues, envoyees, amis) = state.amis.liste(&moi);
    Json(json!({
        "incoming": joueurs(&state, recues),
        "outgoing": joueurs(&state, envoyees),
        "friends": joueurs(&state, amis),
    }))
    .into_response()
}

pub async fn demander(State(state): State<Arc<AppState>>, entetes: HeaderMap, Json(corps): Json<Value>) -> Response {
    let (moi, autre) = match (compte(&state, &entetes), uuid_du_corps(&corps)) {
        (Ok(m), Ok(a)) => (m, a),
        (Err(r), _) | (_, Err(r)) => return r,
    };
    let etat = match state.amis.demander(&moi, &autre) {
        Resultat::Envoyee => "sent",
        Resultat::Acceptee => "accepted",
        Resultat::DejaAmis => "friends",
        Resultat::Refusee => return StatusCode::FORBIDDEN.into_response(),
    };
    Json(json!({ "status": etat })).into_response()
}

pub async fn repondre(State(state): State<Arc<AppState>>, entetes: HeaderMap, Json(corps): Json<Value>) -> Response {
    let (moi, autre) = match (compte(&state, &entetes), uuid_du_corps(&corps)) {
        (Ok(m), Ok(a)) => (m, a),
        (Err(r), _) | (_, Err(r)) => return r,
    };
    if state.amis.repondre(&moi, &autre, corps["accept"].as_bool() == Some(true)) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn retirer(State(state): State<Arc<AppState>>, entetes: HeaderMap, Json(corps): Json<Value>) -> Response {
    let (moi, autre) = match (compte(&state, &entetes), uuid_du_corps(&corps)) {
        (Ok(m), Ok(a)) => (m, a),
        (Err(r), _) | (_, Err(r)) => return r,
    };
    state.amis.retirer(&moi, &autre);
    StatusCode::NO_CONTENT.into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALEX: &str = "ec561538f3fd461daff5086b22154bce";
    const STEVE: &str = "8667ba71b85a4004af54457a9734eed7";

    #[test]
    fn demande_puis_acceptation() {
        let a = Amis::new();
        assert_eq!(a.demander(ALEX, STEVE), Resultat::Envoyee);
        assert_eq!(a.demander(ALEX, STEVE), Resultat::Envoyee);
        assert_eq!(a.liste(STEVE).0, vec![ALEX.to_string()]);
        assert_eq!(a.liste(ALEX).1, vec![STEVE.to_string()]);
        assert!(a.repondre(STEVE, ALEX, true));
        assert_eq!(a.liste(ALEX).2, vec![STEVE.to_string()]);
        assert_eq!(a.liste(STEVE).2, vec![ALEX.to_string()]);
        assert!(a.liste(STEVE).0.is_empty());
        assert_eq!(a.demander(STEVE, ALEX), Resultat::DejaAmis);
        a.retirer(STEVE, ALEX);
        assert!(a.liste(ALEX).2.is_empty());
    }

    #[test]
    fn demandes_croisees_et_refus() {
        let a = Amis::new();
        assert_eq!(a.demander(ALEX, ALEX), Resultat::Refusee);
        assert_eq!(a.demander(ALEX, STEVE), Resultat::Envoyee);
        // Steve demande aussi : accepté tout de suite
        assert_eq!(a.demander(STEVE, ALEX), Resultat::Acceptee);
        a.retirer(ALEX, STEVE);
        assert_eq!(a.demander(ALEX, STEVE), Resultat::Envoyee);
        assert!(a.repondre(STEVE, ALEX, false));
        assert!(a.liste(ALEX).2.is_empty());
        assert!(!a.repondre(STEVE, ALEX, true));
    }
}
