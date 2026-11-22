//! Tableau de bord de l'équipe (`/admin`) : joueurs en ligne, fiche de chaque joueur (donner ou
//! retirer des cosmétiques, bannir, expulser, note), règles des cosmétiques, rapports de logs,
//! journal des changements et éditeur de la base.
//!
//! Protégé par le mot de passe `ADMIN_PASSWORD` (12 caractères ou plus ; sans lui, `/admin`
//! n'existe pas). La session est un cookie signé (HMAC-SHA256, clé tirée du mot de passe : le
//! changer déconnecte tout le monde), `HttpOnly`, `Secure`, `SameSite=Strict`, valable 12 heures.
//! Les formulaires vérifient en plus l'en-tête `Origin`. Les essais de mot de passe sont limités
//! par adresse.

use axum::Form;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::AppState;
use crate::client::{avec_tirets, uuid_normalise};
use crate::registre::{self, Regle};
use crate::views::admin as vue;

const COOKIE: &str = "mistly_admin";
/// Durée d'une session.
const SESSION: u64 = 12 * 3600;
/// Essais de mot de passe par adresse, sur 15 minutes.
const MAX_ESSAIS: u32 = 8;
/// Joueurs par page dans la liste.
pub const PAR_PAGE: usize = 50;

pub struct Admin {
    mot_de_passe: Option<String>,
    /// Clé des cookies de session, tirée du mot de passe
    cle: Vec<u8>,
    /// adresse → (début de la fenêtre, essais ratés)
    essais: Mutex<HashMap<String, (Instant, u32)>>,
}

impl Admin {
    pub fn new() -> Self {
        let mot_de_passe = std::env::var("ADMIN_PASSWORD").ok().filter(|m| m.chars().count() >= 12);
        if std::env::var("ADMIN_PASSWORD").is_ok_and(|m| !m.is_empty()) && mot_de_passe.is_none() {
            eprintln!("ADMIN_PASSWORD trop court (12 caractères au moins) : tableau de bord désactivé");
        }
        Self::avec(mot_de_passe)
    }

    fn avec(mot_de_passe: Option<String>) -> Self {
        let cle = mot_de_passe
            .as_deref()
            .map(|m| Sha256::digest(format!("mistly-admin-session:{m}")).to_vec())
            .unwrap_or_default();
        Self { mot_de_passe, cle, essais: Mutex::default() }
    }

    fn actif(&self) -> bool {
        self.mot_de_passe.is_some()
    }

    fn signer(&self, texte: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.cle).expect("HMAC accepte toute clé");
        mac.update(texte.as_bytes());
        mac.finalize().into_bytes().iter().map(|b| format!("{b:02x}")).collect()
    }

    fn nouvelle_session(&self) -> String {
        let fin = registre::maintenant() + SESSION;
        format!("{fin}.{}", self.signer(&fin.to_string()))
    }

    fn session_valide(&self, valeur: &str) -> bool {
        let Some((fin, sig)) = valeur.split_once('.') else { return false };
        egal(&self.signer(fin), sig) && fin.parse::<u64>().is_ok_and(|f| f > registre::maintenant())
    }

    fn connecte(&self, entetes: &HeaderMap) -> bool {
        self.actif() && cookie(entetes, COOKIE).is_some_and(|v| self.session_valide(&v))
    }

    /// Encore droit à un essai ? (n'en compte pas)
    fn peut_essayer(&self, adresse: &str) -> bool {
        let mut e = self.essais.lock().unwrap();
        e.retain(|_, (debut, _)| debut.elapsed() < Duration::from_secs(900));
        e.get(adresse).is_none_or(|(_, n)| *n < MAX_ESSAIS)
    }

    fn rate(&self, adresse: &str) {
        let mut e = self.essais.lock().unwrap();
        e.entry(adresse.to_string()).or_insert((Instant::now(), 0)).1 += 1;
    }

    fn mot_de_passe_juste(&self, essai: &str) -> bool {
        self.mot_de_passe.as_deref().is_some_and(|m| {
            // comparaison des empreintes : même durée quelle que soit la longueur de l'essai
            egal(&hex(&Sha256::digest(m)), &hex(&Sha256::digest(essai)))
        })
    }
}

fn hex(o: &[u8]) -> String {
    o.iter().map(|b| format!("{b:02x}")).collect()
}

/// Comparaison en temps constant.
fn egal(a: &str, b: &str) -> bool {
    a.len() == b.len() && a.bytes().zip(b.bytes()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn cookie(entetes: &HeaderMap, nom: &str) -> Option<String> {
    entetes
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|p| p.trim().split_once('='))
        .find(|(k, _)| *k == nom)
        .map(|(_, v)| v.to_string())
}

/// Un formulaire envoyé depuis une autre origine est refusé (en plus de `SameSite=Strict`).
fn meme_origine(entetes: &HeaderMap) -> bool {
    let Some(origine) = entetes.get(header::ORIGIN).and_then(|v| v.to_str().ok()) else {
        return true;
    };
    let hote = entetes
        .get("x-forwarded-host")
        .or_else(|| entetes.get(header::HOST))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    origine.split_once("://").is_some_and(|(_, h)| h == hote)
}

/// Les pages du tableau de bord : jamais en cache, jamais indexées, jamais dans un cadre.
fn page(markup: maud::Markup) -> Response {
    let mut res = markup.into_response();
    entetes_prives(res.headers_mut());
    res
}

fn entetes_prives(h: &mut HeaderMap) {
    h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    h.insert("x-robots-tag", HeaderValue::from_static("noindex, nofollow"));
    h.insert("x-frame-options", HeaderValue::from_static("DENY"));
    h.insert(header::REFERRER_POLICY, HeaderValue::from_static("same-origin"));
}

fn redirection(vers: &str) -> Response {
    let mut res = StatusCode::SEE_OTHER.into_response();
    if let Ok(v) = HeaderValue::from_str(vers) {
        res.headers_mut().insert(header::LOCATION, v);
    }
    entetes_prives(res.headers_mut());
    res
}

/// Retour à une page avec un message (`?ok=` ou `?err=`).
fn retour(chemin: &str, ok: bool, message: &str) -> Response {
    let sep = if chemin.contains('?') { '&' } else { '?' };
    redirection(&format!("{chemin}{sep}{}={}", if ok { "ok" } else { "err" }, encoder(message)))
}

fn encoder(texte: &str) -> String {
    texte
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// Garde de chaque page : 404 sans mot de passe configuré, page de connexion sans session.
macro_rules! garde {
    ($state:expr, $entetes:expr) => {
        if !$state.admin.actif() {
            return StatusCode::NOT_FOUND.into_response();
        }
        if !$state.admin.connecte(&$entetes) {
            return redirection("/admin/login");
        }
    };
    ($state:expr, $entetes:expr, post) => {
        garde!($state, $entetes);
        if !meme_origine(&$entetes) {
            return (StatusCode::FORBIDDEN, "Origin refused").into_response();
        }
    };
}

#[derive(Deserialize, Default)]
pub struct Message {
    ok: Option<String>,
    err: Option<String>,
}

impl Message {
    fn vue(&self) -> vue::Flash<'_> {
        vue::Flash { ok: self.ok.as_deref(), err: self.err.as_deref() }
    }
}

// ---------- Connexion ----------

pub async fn page_connexion(State(state): State<Arc<AppState>>, entetes: HeaderMap, Query(m): Query<Message>) -> Response {
    if !state.admin.actif() {
        return StatusCode::NOT_FOUND.into_response();
    }
    if state.admin.connecte(&entetes) {
        return redirection("/admin");
    }
    page(vue::connexion(m.err.as_deref(), &state.scene))
}

#[derive(Deserialize)]
pub struct Identifiants {
    password: String,
}

pub async fn connexion(State(state): State<Arc<AppState>>, entetes: HeaderMap, Form(f): Form<Identifiants>) -> Response {
    let a = &state.admin;
    if !a.actif() {
        return StatusCode::NOT_FOUND.into_response();
    }
    if !meme_origine(&entetes) {
        return (StatusCode::FORBIDDEN, "Origin refused").into_response();
    }
    let adresse = crate::presence::adresse(&entetes);
    if !a.peut_essayer(&adresse) {
        return retour("/admin/login", false, "Too many attempts. Try again in 15 minutes.");
    }
    if !a.mot_de_passe_juste(&f.password) {
        a.rate(&adresse);
        eprintln!("Tableau de bord : mot de passe refusé");
        return retour("/admin/login", false, "Wrong password.");
    }
    state.registre.modifier("Signed in to the dashboard", |_| ());
    let mut res = redirection("/admin");
    let c = format!("{COOKIE}={}; Path=/; Max-Age={SESSION}; HttpOnly; Secure; SameSite=Strict", a.nouvelle_session());
    if let Ok(v) = HeaderValue::from_str(&c) {
        res.headers_mut().insert(header::SET_COOKIE, v);
    }
    res
}

pub async fn deconnexion(State(state): State<Arc<AppState>>, entetes: HeaderMap) -> Response {
    garde!(state, entetes, post);
    let mut res = redirection("/admin/login");
    let c = format!("{COOKIE}=; Path=/; Max-Age=0; HttpOnly; Secure; SameSite=Strict");
    if let Ok(v) = HeaderValue::from_str(&c) {
        res.headers_mut().insert(header::SET_COOKIE, v);
    }
    res
}

// ---------- Vue d'ensemble ----------

pub async fn accueil(State(state): State<Arc<AppState>>, entetes: HeaderMap, Query(m): Query<Message>) -> Response {
    garde!(state, entetes);
    let base = state.registre.copie();
    let (launchers, en_partie) = state.presence.compte();
    let record = state.presence.record().0;
    let en_ligne = joueurs_en_ligne(&state, &base);
    let t = state.telemetrie.dossier();
    let telemetrie = tokio::task::spawn_blocking(move || crate::telemetrie::compter(&t, 30)).await.ok();
    let maintenant = registre::maintenant();
    let fiches = base.players.values();
    let depuis = |s: u64| fiches.clone().filter(|f| f.first_seen.is_some_and(|v| v + s > maintenant)).count();
    let actifs = base.players.values().filter(|f| f.last_seen.is_some_and(|v| v + 86_400 > maintenant)).count();
    let chiffres = vue::Chiffres {
        launchers,
        en_partie,
        record,
        client: en_ligne.len(),
        joueurs: base.players.len(),
        nouveaux_24h: depuis(86_400),
        nouveaux_7j: depuis(7 * 86_400),
        actifs_24h: actifs,
        bannis: base.players.values().filter(|f| f.banned).count(),
        version: state.github.en_cache().and_then(|l| crate::github::latest(&l).map(|r| r.tag_name.clone())),
        dispo: state.status.dispo_globale(),
    };
    let journal: Vec<_> = base.log.iter().rev().take(8).cloned().collect();
    page(vue::accueil(&m.vue(), &chiffres, &en_ligne, telemetrie.as_ref(), &journal))
}

fn joueurs_en_ligne(state: &AppState, base: &registre::Base) -> Vec<vue::Ligne> {
    state
        .client
        .liste_en_ligne()
        .into_iter()
        .map(|j| {
            let f = base.players.get(&j.uuid);
            vue::Ligne {
                nom: f.and_then(|f| f.name.clone()),
                uuid: j.uuid,
                cosmetique: j.cosmetique,
                chapeau: j.chapeau,
                depuis: j.depuis,
                note: f.is_some_and(|f| !f.note.is_empty()),
            }
        })
        .collect()
}

/// `GET /admin/online.json` : chiffres en direct pour la vue d'ensemble.
pub async fn en_ligne_json(State(state): State<Arc<AppState>>, entetes: HeaderMap) -> Response {
    garde!(state, entetes);
    let (launchers, en_partie) = state.presence.compte();
    let base = state.registre.copie();
    let joueurs: Vec<_> = joueurs_en_ligne(&state, &base)
        .into_iter()
        .map(|l| serde_json::json!({ "uuid": l.uuid, "name": l.nom, "cosmetic": l.cosmetique, "hat": l.chapeau, "since": l.depuis }))
        .collect();
    let mut res = axum::Json(serde_json::json!({ "launchers": launchers, "playing": en_partie, "players": joueurs })).into_response();
    entetes_prives(res.headers_mut());
    res
}

// ---------- Joueurs ----------

#[derive(Deserialize, Default)]
pub struct Recherche {
    q: Option<String>,
    filter: Option<String>,
    page: Option<usize>,
    ok: Option<String>,
    err: Option<String>,
}

pub async fn joueurs(State(state): State<Arc<AppState>>, entetes: HeaderMap, Query(r): Query<Recherche>) -> Response {
    garde!(state, entetes);
    let base = state.registre.copie();
    let en_ligne: std::collections::HashSet<String> = state.client.liste_en_ligne().into_iter().map(|j| j.uuid).collect();
    let q = r.q.as_deref().unwrap_or("").trim().to_ascii_lowercase();
    let q_uuid: String = q.chars().filter(|c| *c != '-').collect();
    let filtre = r.filter.as_deref().unwrap_or("all");
    let mut liste: Vec<(&String, &registre::Fiche)> = base
        .players
        .iter()
        .filter(|(u, f)| {
            q.is_empty() || u.starts_with(&q_uuid) || f.name.as_deref().is_some_and(|n| n.to_ascii_lowercase().contains(&q))
        })
        .filter(|(u, f)| match filtre {
            "online" => en_ligne.contains(*u),
            "banned" => f.banned,
            "granted" => !f.granted.is_empty() || !f.revoked.is_empty(),
            "notes" => !f.note.is_empty(),
            "manual" => f.first_seen.is_none(),
            _ => true,
        })
        .collect();
    // en ligne d'abord, puis du plus récemment vu au plus ancien
    liste.sort_by_key(|(u, f)| (!en_ligne.contains(*u), std::cmp::Reverse(f.last_seen.unwrap_or(0))));
    let total = liste.len();
    let pages = total.div_ceil(PAR_PAGE).max(1);
    let p = r.page.unwrap_or(1).clamp(1, pages);
    let lignes: Vec<vue::Fiche> = liste
        .into_iter()
        .skip((p - 1) * PAR_PAGE)
        .take(PAR_PAGE)
        .map(|(u, f)| vue::Fiche { uuid: u.clone(), fiche: f.clone(), en_ligne: en_ligne.contains(u) })
        .collect();
    let flash = vue::Flash { ok: r.ok.as_deref(), err: r.err.as_deref() };
    page(vue::joueurs(&flash, r.q.as_deref().unwrap_or(""), filtre, &lignes, total, p, pages))
}

#[derive(Deserialize)]
pub struct Ajout {
    name: String,
}

/// Ajoute un joueur par son pseudo ou son UUID (pour lui donner un cosmétique avant qu'il ne joue).
pub async fn ajouter(State(state): State<Arc<AppState>>, entetes: HeaderMap, Form(a): Form<Ajout>) -> Response {
    garde!(state, entetes, post);
    let saisie = a.name.trim();
    let (uuid, nom) = if let Some(uuid) = uuid_normalise(saisie) {
        (uuid, None)
    } else if crate::views::shop::valid_username(saisie) {
        match state.skins.lookup(state.github.http(), saisie, false).await {
            crate::skins::Lookup::Found(p) => match p.uuid.as_deref().and_then(uuid_normalise) {
                Some(u) => (u, Some(p.name.clone())),
                None => return retour("/admin/players", false, "Mojang isn't answering, try again in a moment."),
            },
            crate::skins::Lookup::NotFound => return retour("/admin/players", false, &format!("No Minecraft account is called {saisie}.")),
            crate::skins::Lookup::Unavailable => return retour("/admin/players", false, "Mojang isn't answering, try again in a moment."),
        }
    } else {
        return retour("/admin/players", false, "Enter a Minecraft username or a UUID.");
    };
    let affiche = format!("{} ({})", nom.as_deref().unwrap_or("?"), avec_tirets(&uuid));
    state.registre.modifier(&format!("Player added: {affiche}"), |b| {
        let f = b.players.entry(uuid.clone()).or_default();
        if f.name.is_none() {
            f.name = nom;
        }
    });
    retour(&format!("/admin/players/{uuid}"), true, "Player added.")
}

pub async fn fiche(State(state): State<Arc<AppState>>, entetes: HeaderMap, Path(uuid): Path<String>, Query(m): Query<Message>) -> Response {
    garde!(state, entetes);
    let Some(uuid) = uuid_normalise(&uuid) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(f) = state.registre.fiche(&uuid) else {
        return retour("/admin/players", false, "This player is not in the database.");
    };
    let en_ligne = state.client.liste_en_ligne().into_iter().find(|j| j.uuid == uuid);
    let early = state.registre.early_avant();
    let cosmetiques: Vec<vue::EtatCosmetique> = registre::CATALOGUE
        .iter()
        .map(|c| vue::EtatCosmetique {
            c,
            regle: state.registre.regle(c.id),
            donne: f.granted.contains(c.id),
            retire: f.revoked.contains(c.id),
            permis: state.registre.peut_porter(&uuid, c.id),
            porte: en_ligne.as_ref().is_some_and(|j| j.cosmetique == c.id || j.chapeau == c.id),
        })
        .collect();
    let journal: Vec<_> = state
        .registre
        .copie()
        .log
        .into_iter()
        .rev()
        .filter(|e| e.text.contains(&avec_tirets(&uuid)))
        .take(20)
        .collect();
    page(vue::fiche(&m.vue(), &uuid, &f, en_ligne.as_ref(), &cosmetiques, early, &journal))
}

#[derive(Deserialize)]
pub struct Action {
    action: String,
    cosmetic: Option<String>,
    note: Option<String>,
}

pub async fn agir(State(state): State<Arc<AppState>>, entetes: HeaderMap, Path(uuid): Path<String>, Form(a): Form<Action>) -> Response {
    garde!(state, entetes, post);
    let Some(uuid) = uuid_normalise(&uuid) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(f) = state.registre.fiche(&uuid) else {
        return retour("/admin/players", false, "This player is not in the database.");
    };
    let chemin = format!("/admin/players/{uuid}");
    let qui = format!("{} ({})", f.name.as_deref().unwrap_or("?"), avec_tirets(&uuid));
    let r = &state.registre;
    let cosmetique = a.cosmetic.as_deref().and_then(registre::cosmetique);
    let message = match (a.action.as_str(), cosmetique) {
        ("grant" | "revoke" | "default", Some(c)) => {
            let verbe = match a.action.as_str() {
                "grant" => "Given",
                "revoke" => "Taken",
                _ => "Reset to default",
            };
            r.modifier(&format!("{verbe} to {qui}: {}", c.nom), |b| {
                if let Some(f) = b.players.get_mut(&uuid) {
                    f.granted.remove(c.id);
                    f.revoked.remove(c.id);
                    match a.action.as_str() {
                        "grant" => f.granted.insert(c.id.to_string()),
                        "revoke" => f.revoked.insert(c.id.to_string()),
                        _ => false,
                    };
                }
            });
            state.client.reverifier(r, &uuid);
            format!("{verbe}: {}.", c.nom)
        }
        ("ban", _) => {
            r.modifier(&format!("Banned: {qui}"), |b| b.players.get_mut(&uuid).map(|f| f.banned = true));
            state.client.expulser(&uuid);
            "Player banned from the Mistly network.".into()
        }
        ("unban", _) => {
            r.modifier(&format!("Unbanned: {qui}"), |b| b.players.get_mut(&uuid).map(|f| f.banned = false));
            "Ban lifted.".into()
        }
        ("kick", _) => {
            r.modifier(&format!("Kicked: {qui}"), |_| ());
            state.client.expulser(&uuid);
            "Player disconnected from the Mistly network (their game reconnects by itself).".into()
        }
        ("note", _) => {
            let note: String = a.note.unwrap_or_default().trim().chars().take(registre::MAX_NOTE).collect();
            r.modifier(&format!("Note changed: {qui}"), |b| b.players.get_mut(&uuid).map(|f| f.note = note));
            "Note saved.".into()
        }
        ("forget", _) => {
            r.modifier(&format!("Record deleted: {qui}"), |b| b.players.remove(&uuid));
            state.client.expulser(&uuid);
            return retour("/admin/players", true, "Record deleted.");
        }
        _ => return retour(&chemin, false, "Unknown action."),
    };
    retour(&chemin, true, &message)
}

// ---------- Cosmétiques ----------

pub async fn cosmetiques(State(state): State<Arc<AppState>>, entetes: HeaderMap, Query(m): Query<Message>) -> Response {
    garde!(state, entetes);
    let base = state.registre.copie();
    let en_ligne = state.client.liste_en_ligne();
    let lignes: Vec<vue::RegleCosmetique> = registre::CATALOGUE
        .iter()
        .map(|c| vue::RegleCosmetique {
            c,
            regle: state.registre.regle(c.id),
            donnes: base.players.values().filter(|f| f.granted.contains(c.id)).count(),
            retires: base.players.values().filter(|f| f.revoked.contains(c.id)).count(),
            portes: en_ligne.iter().filter(|j| j.cosmetique == c.id || j.chapeau == c.id).count(),
        })
        .collect();
    let early = vue::Early {
        date: base.early_before.clone().or_else(|| std::env::var("EARLY_BEFORE").ok().filter(|d| registre::date_unix(d).is_some())),
        depuis_la_base: state.registre.early_dans_la_base(),
        eligibles: state.registre.early_avant().map(|l| base.players.values().filter(|f| f.first_seen.is_some_and(|v| v < l)).count()),
    };
    page(vue::cosmetiques(&m.vue(), &lignes, &early))
}

#[derive(Deserialize)]
pub struct ChangementRegle {
    cosmetic: Option<String>,
    rule: Option<String>,
    early_before: Option<String>,
}

pub async fn changer_regle(State(state): State<Arc<AppState>>, entetes: HeaderMap, Form(c): Form<ChangementRegle>) -> Response {
    garde!(state, entetes, post);
    if let Some(date) = c.early_before {
        let date = date.trim().to_string();
        if date.is_empty() {
            state.registre.modifier("Early players date: the EARLY_BEFORE one", |b| b.early_before = None);
            return retour("/admin/cosmetics", true, "The date is back to the EARLY_BEFORE one.");
        }
        if registre::date_unix(&date).is_none() {
            return retour("/admin/cosmetics", false, "Invalid date (YYYY-MM-DD).");
        }
        state.registre.modifier(&format!("Early players date: {date}"), |b| b.early_before = Some(date));
        return retour("/admin/cosmetics", true, "Early players date saved.");
    }
    let (Some(cos), Some(regle)) = (c.cosmetic.as_deref().and_then(registre::cosmetique), c.rule.as_deref().and_then(Regle::depuis)) else {
        return retour("/admin/cosmetics", false, "Invalid choice.");
    };
    state.registre.modifier(&format!("{} : {}", cos.nom, regle.libelle()), |b| {
        if regle == cos.regle_par_defaut {
            b.cosmetics.remove(cos.id);
        } else {
            b.cosmetics.insert(cos.id.to_string(), regle);
        }
    });
    for j in state.client.liste_en_ligne() {
        state.client.reverifier(&state.registre, &j.uuid);
    }
    retour("/admin/cosmetics", true, &format!("{} : {}.", cos.nom, regle.libelle()))
}

// ---------- Boutique ----------

pub async fn boutique(State(state): State<Arc<AppState>>, entetes: HeaderMap, Query(m): Query<Message>) -> Response {
    garde!(state, entetes);
    page(vue::boutique(&m.vue(), &crate::views::shop::pour_admin()))
}

/// Enregistre les réglages de toute la boutique (`<article>.prix`, `.statut`, `.nouveau`…).
pub async fn enregistrer_boutique(State(state): State<Arc<AppState>>, entetes: HeaderMap, Form(champs): Form<Vec<(String, String)>>) -> Response {
    garde!(state, entetes, post);
    use crate::boutique::{Reglage, Statut};
    let mut reglages: std::collections::BTreeMap<String, Reglage> = std::collections::BTreeMap::new();
    for l in crate::views::shop::pour_admin() {
        reglages.insert(l.cle.to_string(), Reglage::default());
    }
    for (cle, valeur) in champs {
        let Some((article, champ)) = cle.split_once('.') else { continue };
        let Some(r) = reglages.get_mut(article) else { continue };
        match champ {
            "prix" => match crate::boutique::lire_prix(&valeur) {
                Ok(p) => r.prix = p,
                Err(()) => return retour("/admin/shop", false, &format!("Invalid price for {article}: {valeur}")),
            },
            "statut" => r.statut = Statut::depuis(&valeur),
            "nouveau" => r.nouveau = true,
            "populaire" => r.populaire = true,
            "cache" => r.cache = true,
            _ => {}
        }
    }
    if let Err(e) = crate::boutique::remplacer(reglages) {
        return retour("/admin/shop", false, &format!("Could not save the shop: {e}"));
    }
    state.registre.modifier("Shop settings saved", |_| ());
    retour("/admin/shop", true, "Shop saved.")
}

// ---------- Rapports de logs ----------

#[derive(Deserialize, Default)]
pub struct Code {
    code: Option<String>,
}

pub async fn rapports(State(state): State<Arc<AppState>>, entetes: HeaderMap, Query(q): Query<Code>) -> Response {
    garde!(state, entetes);
    let dossier = state.telemetrie.dossier().join("reports");
    if let Some(code) = q.code.as_deref().map(|c| c.trim().to_ascii_uppercase()).filter(|c| !c.is_empty()) {
        let code = if code.starts_with("MR-") { code } else { format!("MR-{code}") };
        if !crate::telemetrie::code_valide(&code) {
            return page(vue::rapports(Some("Invalid code (MR- followed by 7 characters)."), &[], None));
        }
        return match tokio::fs::read(dossier.join(format!("{code}.json"))).await {
            Ok(o) => {
                let v: serde_json::Value = serde_json::from_slice(&o).unwrap_or_default();
                page(vue::rapports(None, &[], Some(&v)))
            }
            Err(_) => page(vue::rapports(Some(&format!("No report {code}.")), &[], None)),
        };
    }
    let recents = tokio::task::spawn_blocking(move || {
        let mut fichiers: Vec<(std::time::SystemTime, std::path::PathBuf)> = std::fs::read_dir(&dossier)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
            .collect();
        fichiers.sort_by(|a, b| b.0.cmp(&a.0));
        fichiers.truncate(60);
        let l: Vec<(std::time::SystemTime, serde_json::Value)> = fichiers
            .into_iter()
            .map(|(t, p)| (t, std::fs::read(p).ok().and_then(|o| serde_json::from_slice(&o).ok()).unwrap_or_default()))
            .collect();
        l.into_iter()
            .map(|(_, mut v)| {
                // la liste n'a pas besoin du journal, seulement de sa dernière ligne utile
                let fin = v["log"].as_str().and_then(|l| l.lines().rev().find(|x| !x.trim().is_empty())).unwrap_or("").chars().take(160).collect::<String>();
                if let Some(o) = v.as_object_mut() {
                    o.remove("log");
                    o.insert("last".into(), fin.into());
                }
                v
            })
            .collect::<Vec<_>>()
    })
    .await
    .unwrap_or_default();
    page(vue::rapports(None, &recents, None))
}

// ---------- Journal et base ----------

pub async fn journal(State(state): State<Arc<AppState>>, entetes: HeaderMap) -> Response {
    garde!(state, entetes);
    let base = state.registre.copie();
    page(vue::journal(&base.log))
}

pub async fn base(State(state): State<Arc<AppState>>, entetes: HeaderMap, Query(m): Query<Message>) -> Response {
    garde!(state, entetes);
    let mut b = state.registre.copie();
    let lignes = b.log.len();
    // le journal ne s'édite pas : il reste en dehors de l'éditeur
    b.log.clear();
    let texte = serde_json::to_string_pretty(&b).unwrap_or_default();
    page(vue::base(&m.vue(), &texte, b.players.len(), lignes))
}

pub async fn base_json(State(state): State<Arc<AppState>>, entetes: HeaderMap) -> Response {
    garde!(state, entetes);
    let texte = serde_json::to_vec_pretty(&state.registre.copie()).unwrap_or_default();
    let date = time::OffsetDateTime::now_utc().date();
    let mut res = ([(header::CONTENT_TYPE, "application/json")], texte).into_response();
    if let Ok(v) = HeaderValue::from_str(&format!("attachment; filename=\"mistly-joueurs-{date}.json\"")) {
        res.headers_mut().insert(header::CONTENT_DISPOSITION, v);
    }
    entetes_prives(res.headers_mut());
    res
}

#[derive(Deserialize)]
pub struct Edition {
    json: String,
}

pub async fn enregistrer_base(State(state): State<Arc<AppState>>, entetes: HeaderMap, Form(e): Form<Edition>) -> Response {
    garde!(state, entetes, post);
    let nouvelle: registre::Base = match serde_json::from_str(&e.json) {
        Ok(b) => b,
        Err(err) => return retour("/admin/database", false, &format!("Invalid JSON: {err}")),
    };
    match state.registre.remplacer(nouvelle) {
        Ok(()) => {
            for j in state.client.liste_en_ligne() {
                if state.registre.banni(&j.uuid) {
                    state.client.expulser(&j.uuid);
                } else {
                    state.client.reverifier(&state.registre, &j.uuid);
                }
            }
            retour("/admin/database", true, "Database saved.")
        }
        Err(err) => retour("/admin/database", false, &err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sessions_signees() {
        let a = Admin::avec(Some("un-mot-de-passe-long".into()));
        let s = a.nouvelle_session();
        assert!(a.session_valide(&s));
        // signature modifiée, date repoussée ou session d'un autre mot de passe : refusées
        let (fin, sig) = s.split_once('.').unwrap();
        let autre = if sig.ends_with('0') { '1' } else { '0' };
        assert!(!a.session_valide(&format!("{fin}.{}{autre}", &sig[..sig.len() - 1])));
        assert!(!a.session_valide(&format!("{}.{sig}", fin.parse::<u64>().unwrap() + 100)));
        assert!(!Admin::avec(Some("un-autre-mot-de-passe".into())).session_valide(&s));
        assert!(!a.session_valide("1.abc"));
        // expirée
        let vieille = format!("10.{}", a.signer("10"));
        assert!(!a.session_valide(&vieille));
    }

    #[test]
    fn mot_de_passe_et_essais() {
        let a = Admin::avec(Some("un-mot-de-passe-long".into()));
        assert!(a.mot_de_passe_juste("un-mot-de-passe-long"));
        assert!(!a.mot_de_passe_juste("un-mot-de-passe-lon"));
        assert!(!Admin::avec(None).mot_de_passe_juste(""));
        for _ in 0..MAX_ESSAIS {
            assert!(a.peut_essayer("1.2.3.4"));
            a.rate("1.2.3.4");
        }
        assert!(!a.peut_essayer("1.2.3.4"));
        assert!(a.peut_essayer("5.6.7.8"));
    }

    #[test]
    fn origine_et_cookie() {
        let mut h = HeaderMap::new();
        h.insert(header::HOST, "mistly.cc".parse().unwrap());
        assert!(meme_origine(&h));
        h.insert(header::ORIGIN, "https://mistly.cc".parse().unwrap());
        assert!(meme_origine(&h));
        h.insert(header::ORIGIN, "https://evil.example".parse().unwrap());
        assert!(!meme_origine(&h));
        h.insert(header::COOKIE, "lang=fr; mistly_admin=123.abc".parse().unwrap());
        assert_eq!(cookie(&h, COOKIE).as_deref(), Some("123.abc"));
        assert_eq!(cookie(&h, "absent"), None);
        assert_eq!(encoder("Cape donnée !"), "Cape%20donn%C3%A9e%20%21");
    }
}
