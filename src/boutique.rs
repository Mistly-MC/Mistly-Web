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
//! Réglages de la boutique, modifiés depuis le tableau de bord (`/admin/shop`) : pour chaque
//! article, son prix, son statut (sinon celui du code), s'il est dans les nouveautés ou les
//! populaires, et s'il est caché.
//!
//! Gardés dans `boutique.json` (dans `MISTY_CACHE_DIR`), écrit à chaque enregistrement. Sans ce
//! dossier (tests), rien n'est écrit. Les pages lisent la copie en mémoire.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

/// Le statut imposé à un article.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Statut {
    Available,
    Early,
    New,
    Soon,
}

impl Statut {
    pub const TOUS: [Statut; 4] = [Statut::Available, Statut::Early, Statut::New, Statut::Soon];

    pub fn code(self) -> &'static str {
        match self {
            Statut::Available => "available",
            Statut::Early => "early",
            Statut::New => "new",
            Statut::Soon => "soon",
        }
    }

    pub fn depuis(code: &str) -> Option<Self> {
        Self::TOUS.into_iter().find(|s| s.code() == code)
    }

    pub fn libelle(self) -> &'static str {
        match self {
            Statut::Available => "Available",
            Statut::Early => "Early players",
            Statut::New => "New",
            Statut::Soon => "Soon",
        }
    }
}

/// Les réglages d'un article (tout est facultatif : sans réglage, l'article garde ce que dit le code).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Reglage {
    /// Prix en centimes d'euro (aucun : pas de prix affiché)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prix: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statut: Option<Statut>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub nouveau: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub populaire: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub cache: bool,
}

fn fichier() -> Option<PathBuf> {
    if cfg!(test) {
        return None;
    }
    std::env::var_os("MISTY_CACHE_DIR").map(|d| PathBuf::from(d).join("boutique.json"))
}

static REGLAGES: LazyLock<RwLock<BTreeMap<String, Reglage>>> = LazyLock::new(|| {
    let lus = fichier()
        .and_then(|f| std::fs::read(f).ok())
        .and_then(|o| serde_json::from_slice(&o).ok())
        .unwrap_or_default();
    RwLock::new(lus)
});

/// Charge les réglages (au démarrage).
pub fn init() {
    LazyLock::force(&REGLAGES);
}

/// Les réglages d'un article.
pub fn de(cle: &str) -> Reglage {
    REGLAGES.read().unwrap().get(cle).cloned().unwrap_or_default()
}

/// Les articles réglés « nouveautés » ou « populaires » (dans l'ordre de `cles`), s'il y en a.
pub fn marques(cles: &[&'static str], populaire: bool) -> Vec<&'static str> {
    let r = REGLAGES.read().unwrap();
    cles.iter().copied().filter(|c| r.get(*c).is_some_and(|x| if populaire { x.populaire } else { x.nouveau })).collect()
}

/// Remplace tous les réglages et les écrit. Les réglages vides ne sont pas gardés.
pub fn remplacer(nouveaux: BTreeMap<String, Reglage>) -> Result<(), String> {
    let nouveaux: BTreeMap<String, Reglage> = nouveaux.into_iter().filter(|(_, r)| *r != Reglage::default()).collect();
    if let Some(f) = fichier() {
        let texte = serde_json::to_vec_pretty(&nouveaux).map_err(|e| e.to_string())?;
        if let Some(d) = f.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let tmp = f.with_extension("tmp");
        std::fs::write(&tmp, texte).and_then(|_| std::fs::rename(&tmp, &f)).map_err(|e| e.to_string())?;
    }
    *REGLAGES.write().unwrap() = nouveaux;
    Ok(())
}

/// « 4,99 € » : un prix en centimes, avec le séparateur décimal de la langue.
pub fn prix(centimes: u32, sep: &str) -> String {
    if centimes == 0 {
        return String::new();
    }
    format!("{}{sep}{:02} €", centimes / 100, centimes % 100)
}

/// Lit un prix saisi (« 4.99 », « 4,99 », « 5 », « 4,99 € ») en centimes ; vide : pas de prix.
pub fn lire_prix(saisie: &str) -> Result<Option<u32>, ()> {
    let t: String = saisie.trim().trim_end_matches('€').trim().replace(',', ".");
    if t.is_empty() {
        return Ok(None);
    }
    let v: f64 = t.parse().map_err(|_| ())?;
    if !(0.0..=1000.0).contains(&v) {
        return Err(());
    }
    Ok(Some((v * 100.0).round() as u32).filter(|c| *c > 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prix_saisis_et_affiches() {
        assert_eq!(lire_prix("4,99"), Ok(Some(499)));
        assert_eq!(lire_prix(" 5 € "), Ok(Some(500)));
        assert_eq!(lire_prix(""), Ok(None));
        assert_eq!(lire_prix("0"), Ok(None));
        assert!(lire_prix("abc").is_err());
        assert!(lire_prix("-3").is_err());
        assert_eq!(prix(499, ","), "4,99 €");
        assert_eq!(prix(1250, "."), "12.50 €");
    }
}
