//! La liste des choses à faire de l'équipe (page « To-do » du tableau de bord), en quatre parties :
//! Mistly Launcher, Mistly Client, Mistly Web et les cosmétiques.
//!
//! Gardée dans `todo.json` (dans `MISTY_CACHE_DIR`), écrit à chaque changement. Sans ce dossier
//! (tests), rien n'est écrit. La première fois, la liste part de quelques tâches du projet.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

/// La partie du projet d'une tâche.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Projet {
    Launcher,
    Client,
    Web,
    Cosmetics,
}

impl Projet {
    pub const TOUS: [Projet; 4] = [Projet::Launcher, Projet::Client, Projet::Web, Projet::Cosmetics];

    pub fn code(self) -> &'static str {
        match self {
            Projet::Launcher => "launcher",
            Projet::Client => "client",
            Projet::Web => "web",
            Projet::Cosmetics => "cosmetics",
        }
    }

    pub fn depuis(code: &str) -> Option<Self> {
        Self::TOUS.into_iter().find(|p| p.code() == code)
    }

    pub fn nom(self) -> &'static str {
        match self {
            Projet::Launcher => "Mistly Launcher",
            Projet::Client => "Mistly Client",
            Projet::Web => "Mistly Web",
            Projet::Cosmetics => "Cosmetics",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tache {
    pub id: u64,
    pub projet: Projet,
    pub texte: String,
    /// Secondes Unix
    pub cree: u64,
    /// Terminée le (secondes Unix)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fait: Option<u64>,
}

/// Longueur maximale d'une tâche.
pub const MAX_TEXTE: usize = 200;

fn fichier() -> Option<PathBuf> {
    if cfg!(test) {
        return None;
    }
    std::env::var_os("MISTY_CACHE_DIR").map(|d| PathBuf::from(d).join("todo.json"))
}

/// La liste de départ : des tâches réelles du projet (à supprimer ou garder).
fn depart() -> Vec<Tache> {
    let t = crate::registre::maintenant();
    [
        (Projet::Launcher, "Publish 0.4.1 (Discord tile, Mistly Client version in About)"),
        (Projet::Launcher, "Finish and publish 0.5.0 (new design, Discover, Servers, Friends)"),
        (Projet::Launcher, "Mistly on macOS"),
        (Projet::Client, "Mist Wings in game, with their colours"),
        (Projet::Client, "Mistly Client for Bedrock Edition"),
        (Projet::Web, "Coolify: Persistent Storage on /app/data"),
        (Projet::Web, "DNS + Coolify domains for a0, status, shop and docs"),
        (Projet::Web, "Set GITHUB_TOKEN in Coolify"),
        (Projet::Web, "Payments to open the shop"),
        (Projet::Cosmetics, "Bunny ears in Mistly Client"),
        (Projet::Cosmetics, "Minime (pet on the shoulder) in Mistly Client"),
        (Projet::Cosmetics, "Emotes in game (Hug, Wave, Piggyback…)"),
        (Projet::Cosmetics, "Effects (particles and trails)"),
        (Projet::Cosmetics, "Name tags"),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, (projet, texte))| Tache { id: i as u64 + 1, projet, texte: texte.into(), cree: t, fait: None })
    .collect()
}

static TACHES: LazyLock<RwLock<Vec<Tache>>> = LazyLock::new(|| {
    let mut lues: Vec<Tache> = match fichier() {
        Some(f) if f.exists() => std::fs::read(&f).ok().and_then(|o| serde_json::from_slice(&o).ok()).unwrap_or_default(),
        _ => depart(),
    };
    // Une liste créée avant la colonne des cosmétiques la reçoit une fois (un fichier témoin évite
    // de la remettre si l'équipe a supprimé ces tâches)
    if let Some(f) = fichier() {
        let temoin = f.with_file_name("todo-cosmetics.done");
        if !temoin.exists() {
            if !lues.iter().any(|t| t.projet == Projet::Cosmetics) {
                let premier = lues.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                let nouvelles: Vec<Tache> = depart().into_iter().filter(|t| t.projet == Projet::Cosmetics).collect();
                for (id, t) in (premier..).zip(nouvelles) {
                    lues.push(Tache { id, ..t });
                }
            }
            ecrire(&lues);
            let _ = std::fs::write(&temoin, b"");
        }
    }
    RwLock::new(lues)
});

pub fn init() {
    LazyLock::force(&TACHES);
}

/// Toutes les tâches.
pub fn liste() -> Vec<Tache> {
    TACHES.read().unwrap().clone()
}

fn ecrire(taches: &[Tache]) {
    let Some(f) = fichier() else { return };
    let Ok(texte) = serde_json::to_vec_pretty(taches) else { return };
    if let Some(d) = f.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let tmp = f.with_extension("tmp");
    if std::fs::write(&tmp, texte).and_then(|_| std::fs::rename(&tmp, &f)).is_err() {
        eprintln!("To-do : impossible d'écrire {}", f.display());
    }
}

/// Ajoute une tâche ; rend son texte (raccourci) s'il n'est pas vide.
pub fn ajouter(projet: Projet, texte: &str) -> Option<String> {
    let texte: String = texte.trim().chars().take(MAX_TEXTE).collect();
    if texte.is_empty() {
        return None;
    }
    let mut l = TACHES.write().unwrap();
    let id = l.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    l.push(Tache { id, projet, texte: texte.clone(), cree: crate::registre::maintenant(), fait: None });
    ecrire(&l);
    Some(texte)
}

/// Termine une tâche (ou la rouvre) ; rend son texte.
pub fn marquer(id: u64, fait: bool) -> Option<String> {
    let mut l = TACHES.write().unwrap();
    let t = l.iter_mut().find(|t| t.id == id)?;
    t.fait = fait.then(crate::registre::maintenant);
    let texte = t.texte.clone();
    ecrire(&l);
    Some(texte)
}

/// Supprime une tâche ; rend son texte.
pub fn supprimer(id: u64) -> Option<String> {
    let mut l = TACHES.write().unwrap();
    let i = l.iter().position(|t| t.id == id)?;
    let t = l.remove(i);
    ecrire(&l);
    Some(t.texte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajouter_terminer_supprimer() {
        let avant = liste().len();
        assert!(ajouter(Projet::Web, "   ").is_none());
        let texte = ajouter(Projet::Web, "Tester la liste").unwrap();
        assert_eq!(texte, "Tester la liste");
        let id = liste().iter().find(|t| t.texte == "Tester la liste").unwrap().id;
        assert!(marquer(id, true).is_some());
        assert!(liste().iter().find(|t| t.id == id).unwrap().fait.is_some());
        assert!(marquer(id, false).is_some());
        assert!(supprimer(id).is_some());
        assert_eq!(liste().len(), avant);
        assert!(supprimer(id).is_none());
    }
}
