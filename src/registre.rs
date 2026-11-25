//  ███╗   ███╗██╗███████╗████████╗██╗     ██╗   ██╗
//  ████╗ ████║██║██╔════╝╚══██╔══╝██║     ╚██╗ ██╔╝
//  ██╔████╔██║██║███████╗   ██║   ██║      ╚████╔╝
//  ██║╚██╔╝██║██║╚════██║   ██║   ██║       ╚██╔╝
//  ██║ ╚═╝ ██║██║███████║   ██║   ███████╗   ██║
//  ╚═╝     ╚═╝╚═╝╚══════╝   ╚═╝   ╚══════╝   ╚═╝
//
//  Mistly Web · https://mistly.cc
//  Code : https://github.com/Mistly-MC/Mistly-Web
//  Discord : https://discord.gg/rdQJUgvUh3
//  Misty License 1.0 : free to use, modify and share, not to sell (LICENSE.txt)
//
//! Registre des joueurs de Mistly Client : la petite base de données du site, éditée depuis le
//! tableau de bord (`/admin`).
//!
//! Pour chaque compte vu par Mistly Client (ou ajouté à la main) : dernier pseudo, première et
//! dernière connexion, nombre de connexions, cosmétiques donnés ou retirés, bannissement et note
//! de l'équipe. Plus une règle par cosmétique (pour tout le monde, premiers joueurs, sur
//! attribution, désactivé) et le journal des changements faits depuis le tableau de bord.
//!
//! Tout est gardé dans `joueurs.json` (dans `MISTY_CACHE_DIR`). L'ancien fichier
//! `premiers-joueurs.json` (UUID → première connexion) est repris au premier démarrage.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Au-delà, les nouveaux comptes ne sont plus notés (protection mémoire et disque).
const MAX_FICHES: usize = 2_000_000;
/// Entrées gardées dans le journal.
const MAX_JOURNAL: usize = 2_000;
/// Longueur maximale d'une note.
pub const MAX_NOTE: usize = 2_000;

/// Où un cosmétique se porte.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Place {
    /// Cape ou élytres
    Dos,
    Tete,
}

pub struct Cosmetique {
    /// Identifiant partagé avec le launcher et le mod
    pub id: &'static str,
    pub nom: &'static str,
    pub place: Place,
    pub regle_par_defaut: Regle,
}

/// Les cosmétiques Mistly que le mod sait afficher.
pub const CATALOGUE: &[Cosmetique] = &[
    Cosmetique { id: "misty_cape", nom: "Mistly Cape", place: Place::Dos, regle_par_defaut: Regle::Everyone },
    Cosmetique { id: "misty_elytra", nom: "Mistly Elytra", place: Place::Dos, regle_par_defaut: Regle::Everyone },
    Cosmetique { id: "early_cape", nom: "Early player cape", place: Place::Dos, regle_par_defaut: Regle::Early },
    Cosmetique { id: "cloudly", nom: "Cloudly (hat)", place: Place::Tete, regle_par_defaut: Regle::Everyone },
];

pub fn cosmetique(id: &str) -> Option<&'static Cosmetique> {
    CATALOGUE.iter().find(|c| c.id == id)
}

/// Qui peut porter un cosmétique.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Regle {
    /// Tout le monde
    Everyone,
    /// Les comptes vus avant la date des premiers joueurs
    Early,
    /// Seulement ceux à qui il a été donné
    Granted,
    /// Personne, même ceux à qui il a été donné
    Off,
}

impl Regle {
    pub const TOUTES: [Regle; 4] = [Regle::Everyone, Regle::Early, Regle::Granted, Regle::Off];

    pub fn code(self) -> &'static str {
        match self {
            Regle::Everyone => "everyone",
            Regle::Early => "early",
            Regle::Granted => "granted",
            Regle::Off => "off",
        }
    }

    pub fn libelle(self) -> &'static str {
        match self {
            Regle::Everyone => "Everyone",
            Regle::Early => "Early players",
            Regle::Granted => "When given",
            Regle::Off => "Disabled",
        }
    }

    pub fn depuis(code: &str) -> Option<Regle> {
        Regle::TOUTES.into_iter().find(|r| r.code() == code)
    }
}

/// La fiche d'un compte.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Fiche {
    /// Dernier pseudo connu
    pub name: Option<String>,
    /// Première connexion à Mistly Client (secondes Unix) ; absente pour un compte ajouté à la main
    pub first_seen: Option<u64>,
    /// Dernier signe de vie
    pub last_seen: Option<u64>,
    pub logins: u64,
    /// Cosmétiques donnés à ce compte
    pub granted: BTreeSet<String>,
    /// Cosmétiques retirés à ce compte (même si la règle les donne à tout le monde)
    pub revoked: BTreeSet<String>,
    /// Banni du réseau Mistly : connexion refusée, plus aucun cosmétique
    pub banned: bool,
    pub note: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entree {
    /// Secondes Unix
    pub at: u64,
    pub text: String,
    /// L'adresse IP de qui a fait le changement depuis le tableau de bord (les entrées plus
    /// anciennes et les changements automatiques n'en ont pas)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// Le contenu de `joueurs.json`.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Base {
    /// UUID (32 chiffres hexadécimaux en minuscules) → fiche
    pub players: BTreeMap<String, Fiche>,
    /// Règles changées depuis le tableau de bord (les autres gardent leur règle par défaut)
    pub cosmetics: BTreeMap<String, Regle>,
    /// Date des premiers joueurs (AAAA-MM-JJ) ; prend le pas sur `EARLY_BEFORE`
    pub early_before: Option<String>,
    /// Journal des changements, du plus ancien au plus récent
    pub log: Vec<Entree>,
}

impl Base {
    /// Vérifie une base envoyée depuis le tableau de bord avant de la prendre.
    pub fn valider(&self) -> Result<(), String> {
        for (uuid, f) in &self.players {
            if crate::client::uuid_normalise(uuid).as_deref() != Some(uuid.as_str()) {
                return Err(format!("Invalid UUID: {uuid} (32 lowercase hex digits, no dashes)"));
            }
            for id in f.granted.iter().chain(&f.revoked) {
                if cosmetique(id).is_none() {
                    return Err(format!("Unknown cosmetic for {uuid}: {id}"));
                }
            }
            if f.note.len() > MAX_NOTE {
                return Err(format!("Note too long for {uuid}"));
            }
        }
        for id in self.cosmetics.keys() {
            if cosmetique(id).is_none() {
                return Err(format!("Unknown cosmetic in the rules: {id}"));
            }
        }
        if let Some(d) = &self.early_before {
            if date_unix(d).is_none() {
                return Err(format!("Invalid early players date: {d} (YYYY-MM-DD)"));
            }
        }
        Ok(())
    }
}

pub struct Registre {
    base: Mutex<Base>,
    fichier: Option<PathBuf>,
    /// Des connexions ont changé la base depuis la dernière écriture
    sale: AtomicBool,
    /// `EARLY_BEFORE` (secondes Unix), si la base n'a pas sa propre date
    early_env: Option<u64>,
}

impl Registre {
    pub fn new() -> Self {
        let fichier = dossier().map(|d| d.join("joueurs.json"));
        let base = fichier
            .as_ref()
            .and_then(|f| std::fs::read(f).ok())
            .and_then(|o| serde_json::from_slice::<Base>(&o).ok())
            .unwrap_or_else(|| {
                // premier démarrage avec le registre : on reprend les premières connexions
                let mut base = Base::default();
                let anciens: BTreeMap<String, u64> = dossier()
                    .and_then(|d| std::fs::read(d.join("premiers-joueurs.json")).ok())
                    .and_then(|o| serde_json::from_slice(&o).ok())
                    .unwrap_or_default();
                for (uuid, vu) in anciens {
                    base.players.insert(uuid, Fiche { first_seen: Some(vu), last_seen: Some(vu), ..Fiche::default() });
                }
                base
            });
        Self::avec(base, fichier, std::env::var("EARLY_BEFORE").ok().and_then(|d| date_unix(&d)))
    }

    pub fn avec(base: Base, fichier: Option<PathBuf>, early_env: Option<u64>) -> Self {
        Self { base: Mutex::new(base), fichier, sale: AtomicBool::new(false), early_env }
    }

    /// Écrit la base si des connexions l'ont changée, toutes les 20 secondes.
    pub async fn ecrire_en_boucle(self: std::sync::Arc<Self>) {
        loop {
            tokio::time::sleep(Duration::from_secs(20)).await;
            if self.sale.swap(false, Ordering::Relaxed) {
                self.ecrire();
            }
        }
    }

    /// Écrit tout de suite ce qui ne l'est pas encore (à l'arrêt du serveur).
    pub fn sauver(&self) {
        if self.sale.swap(false, Ordering::Relaxed) {
            self.ecrire();
        }
    }

    fn ecrire(&self) {
        let Some(f) = &self.fichier else { return };
        let texte = match serde_json::to_vec(&*self.base.lock().unwrap()) {
            Ok(t) => t,
            Err(_) => return,
        };
        let temporaire = f.with_extension("tmp");
        if let Some(d) = f.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        if std::fs::write(&temporaire, texte).is_ok() {
            let _ = std::fs::rename(&temporaire, f);
        } else {
            eprintln!("Registre : impossible d'écrire {}", f.display());
        }
    }

    /// Date des premiers joueurs en vigueur (secondes Unix ; None : tout le monde y a droit).
    pub fn early_avant(&self) -> Option<u64> {
        let base = self.base.lock().unwrap();
        base.early_before.as_deref().and_then(date_unix).or(self.early_env)
    }

    /// La date vient-elle de la base (true) ou de `EARLY_BEFORE` (false) ?
    pub fn early_dans_la_base(&self) -> bool {
        self.base.lock().unwrap().early_before.is_some()
    }

    /// Une connexion de Mistly Client : note le pseudo et les dates. Rend false si le compte est banni.
    pub fn connexion(&self, uuid: &str, nom: &str) -> bool {
        let mut base = self.base.lock().unwrap();
        if !base.players.contains_key(uuid) && base.players.len() >= MAX_FICHES {
            return true;
        }
        let f = base.players.entry(uuid.to_string()).or_default();
        let t = maintenant();
        f.name = Some(nom.to_string());
        f.first_seen.get_or_insert(t);
        f.last_seen = Some(t);
        f.logins += 1;
        let banni = f.banned;
        drop(base);
        self.sale.store(true, Ordering::Relaxed);
        !banni
    }

    /// Signe de vie : seulement la date (écrite plus tard avec le reste).
    pub fn vu(&self, uuid: &str) {
        if let Some(f) = self.base.lock().unwrap().players.get_mut(uuid) {
            f.last_seen = Some(maintenant());
            self.sale.store(true, Ordering::Relaxed);
        }
    }

    pub fn banni(&self, uuid: &str) -> bool {
        self.base.lock().unwrap().players.get(uuid).is_some_and(|f| f.banned)
    }

    pub fn regle(&self, id: &str) -> Regle {
        let base = self.base.lock().unwrap();
        regle_de(&base, id)
    }

    /// Ce compte peut-il porter ce cosmétique ?
    pub fn peut_porter(&self, uuid: &str, id: &str) -> bool {
        let early = self.early_avant();
        let base = self.base.lock().unwrap();
        peut_porter(&base, uuid, id, early)
    }

    /// Les cosmétiques que ce compte peut porter.
    pub fn permis(&self, uuid: &str) -> Vec<&'static str> {
        let early = self.early_avant();
        let base = self.base.lock().unwrap();
        CATALOGUE.iter().filter(|c| peut_porter(&base, uuid, c.id, early)).map(|c| c.id).collect()
    }

    /// Copie de la base (pour l'afficher).
    pub fn copie(&self) -> Base {
        self.base.lock().unwrap().clone()
    }

    pub fn fiche(&self, uuid: &str) -> Option<Fiche> {
        self.base.lock().unwrap().players.get(uuid).cloned()
    }

    /// Change la base depuis le tableau de bord, note le changement au journal et l'écrit tout de suite.
    pub fn modifier<R>(&self, journal: &str, f: impl FnOnce(&mut Base) -> R) -> R {
        self.modifier_par(None, journal, f)
    }

    /// Comme `modifier`, en notant au journal l'adresse IP de qui fait le changement.
    pub fn modifier_par<R>(&self, ip: Option<&str>, journal: &str, f: impl FnOnce(&mut Base) -> R) -> R {
        let mut base = self.base.lock().unwrap();
        let r = f(&mut base);
        base.log.push(Entree { at: maintenant(), text: journal.to_string(), ip: ip.map(str::to_string) });
        let trop = base.log.len().saturating_sub(MAX_JOURNAL);
        base.log.drain(..trop);
        drop(base);
        self.ecrire();
        r
    }

    /// Remplace toute la base (éditeur JSON du tableau de bord). Le journal envoyé est ignoré : on garde le vrai.
    pub fn remplacer(&self, mut nouvelle: Base, ip: Option<&str>) -> Result<(), String> {
        nouvelle.valider()?;
        self.modifier_par(ip, "Database replaced from the JSON editor", |base| {
            nouvelle.log = std::mem::take(&mut base.log);
            *base = nouvelle;
        });
        Ok(())
    }
}

fn regle_de(base: &Base, id: &str) -> Regle {
    base.cosmetics
        .get(id)
        .copied()
        .or_else(|| cosmetique(id).map(|c| c.regle_par_defaut))
        .unwrap_or(Regle::Off)
}

fn peut_porter(base: &Base, uuid: &str, id: &str, early_avant: Option<u64>) -> bool {
    if cosmetique(id).is_none() {
        return false;
    }
    let fiche = base.players.get(uuid);
    if fiche.is_some_and(|f| f.banned) {
        return false;
    }
    let regle = regle_de(base, id);
    if regle == Regle::Off || fiche.is_some_and(|f| f.revoked.contains(id)) {
        return false;
    }
    if fiche.is_some_and(|f| f.granted.contains(id)) {
        return true;
    }
    match regle {
        Regle::Everyone => true,
        Regle::Early => match early_avant {
            None => true,
            Some(limite) => fiche.and_then(|f| f.first_seen).is_some_and(|vu| vu < limite),
        },
        Regle::Granted | Regle::Off => false,
    }
}

pub fn maintenant() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// "2027-01-01" → secondes Unix à minuit UTC.
pub fn date_unix(texte: &str) -> Option<u64> {
    let format = time::macros::format_description!("[year]-[month]-[day]");
    let date = time::Date::parse(texte.trim(), &format).ok()?;
    u64::try_from(date.midnight().assume_utc().unix_timestamp()).ok()
}

/// `MISTY_CACHE_DIR` (sans lui, rien n'est gardé sur disque ; jamais pendant les tests).
fn dossier() -> Option<PathBuf> {
    if cfg!(test) {
        return None;
    }
    std::env::var_os("MISTY_CACHE_DIR").map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALEX: &str = "ec561538f3fd461daff5086b22154bce";

    fn registre(early: Option<u64>) -> Registre {
        Registre::avec(Base::default(), None, early)
    }

    #[test]
    fn regles_par_defaut() {
        let r = registre(None);
        assert!(r.peut_porter(ALEX, "misty_cape"));
        assert!(r.peut_porter(ALEX, "cloudly"));
        // sans date : la cape early est pour tout le monde
        assert!(r.peut_porter(ALEX, "early_cape"));
        assert!(!r.peut_porter(ALEX, "inconnu"));
    }

    #[test]
    fn cape_early_selon_la_premiere_connexion() {
        let r = registre(date_unix("2026-01-01"));
        assert!(!r.peut_porter(ALEX, "early_cape"));
        r.connexion(ALEX, "Alex");
        // vu maintenant, donc après la date
        assert!(!r.peut_porter(ALEX, "early_cape"));
        r.modifier("test", |b| b.players.get_mut(ALEX).unwrap().first_seen = Some(1_000));
        assert!(r.peut_porter(ALEX, "early_cape"));
        // une date dans la base prend le pas sur EARLY_BEFORE
        r.modifier("test", |b| b.early_before = Some("1970-01-01".into()));
        assert!(!r.peut_porter(ALEX, "early_cape"));
    }

    #[test]
    fn donner_retirer_bannir() {
        let r = registre(Some(0));
        r.modifier("donner", |b| {
            b.players.entry(ALEX.into()).or_default().granted.insert("early_cape".into());
        });
        assert!(r.peut_porter(ALEX, "early_cape"));
        r.modifier("retirer", |b| {
            b.players.get_mut(ALEX).unwrap().revoked.insert("misty_cape".into());
        });
        assert!(!r.peut_porter(ALEX, "misty_cape"));
        assert_eq!(r.permis(ALEX), ["misty_elytra", "early_cape", "cloudly"]);
        // sur attribution : seulement ceux à qui on l'a donné ; désactivé : personne
        r.modifier("règle", |b| {
            b.cosmetics.insert("cloudly".into(), Regle::Granted);
        });
        assert!(!r.peut_porter(ALEX, "cloudly"));
        r.modifier("règle", |b| {
            b.cosmetics.insert("early_cape".into(), Regle::Off);
        });
        assert!(!r.peut_porter(ALEX, "early_cape"));
        // banni : plus rien, et la connexion est refusée
        r.modifier("bannir", |b| b.players.get_mut(ALEX).unwrap().banned = true);
        assert!(r.permis(ALEX).is_empty());
        assert!(!r.connexion(ALEX, "Alex"));
        assert_eq!(r.copie().log.len(), 5);
    }

    #[test]
    fn connexion_note_le_pseudo() {
        let r = registre(None);
        assert!(r.connexion(ALEX, "Alex"));
        assert!(r.connexion(ALEX, "Alex2"));
        let f = r.fiche(ALEX).unwrap();
        assert_eq!(f.name.as_deref(), Some("Alex2"));
        assert_eq!(f.logins, 2);
        assert!(f.first_seen.is_some());
    }

    #[test]
    fn validation_de_la_base() {
        let mut b = Base::default();
        assert!(b.valider().is_ok());
        b.players.insert("pas-un-uuid".into(), Fiche::default());
        assert!(b.valider().is_err());
        let mut b = Base::default();
        b.players.insert(ALEX.into(), Fiche { granted: ["chapeau_inconnu".to_string()].into(), ..Fiche::default() });
        assert!(b.valider().is_err());
        let mut b = Base::default();
        b.early_before = Some("demain".into());
        assert!(b.valider().is_err());
        // le JSON de la base se relit tel quel, avec des champs en camelCase
        let mut b = Base::default();
        b.players.insert(ALEX.into(), Fiche { first_seen: Some(5), ..Fiche::default() });
        b.cosmetics.insert("cloudly".into(), Regle::Granted);
        let texte = serde_json::to_string(&b).unwrap();
        assert!(texte.contains("\"firstSeen\":5") && texte.contains("\"cloudly\":\"granted\""));
        assert_eq!(serde_json::from_str::<Base>(&texte).unwrap(), b);
    }

    #[test]
    fn remplacer_garde_le_journal() {
        let r = registre(None);
        r.modifier("avant", |_| ());
        let mut nouvelle = Base::default();
        nouvelle.log.push(Entree { at: 1, text: "faux".into(), ip: None });
        nouvelle.players.insert(ALEX.into(), Fiche::default());
        r.remplacer(nouvelle, None).unwrap();
        let b = r.copie();
        assert_eq!(b.players.len(), 1);
        assert_eq!(b.log.iter().map(|e| e.text.as_str()).collect::<Vec<_>>(), ["avant", "Database replaced from the JSON editor"]);
    }
}
