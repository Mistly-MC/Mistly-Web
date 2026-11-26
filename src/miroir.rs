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
//! Copie locale des fichiers de la dernière version (installeur et portable).
//!
//! `/download/…` sert d'abord ces copies : un téléchargement ne dépend ni du quota de l'API
//! GitHub, ni d'une lenteur de GitHub. Les fichiers sont téléchargés en arrière-plan dans un
//! fichier temporaire, vérifiés (taille annoncée) puis renommés : une copie présente est
//! toujours complète. Seule la dernière version est gardée.
//!
//! GitHub compte chaque copie comme un téléchargement : le miroir note les siennes
//! (`misty-site-downloads-copies.json`, à côté du dossier des copies) et l'accueil les retire du
//! total. Sans ça, chaque redémarrage du site sans sa copie (nouveau conteneur, volume non
//! persistant) ajoutait 3 téléchargements.

use crate::github::{Kind, Release};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

pub struct Miroir {
    dossier: PathBuf,
    /// Client sans délai total (gros fichiers) mais avec délai de connexion et de lecture
    http: reqwest::Client,
    /// Fichier des copies faites par le site : nom du fichier → nombre de copies
    fichier_copies: PathBuf,
    copies: Mutex<HashMap<String, u64>>,
}

impl Miroir {
    pub fn new(dossier: PathBuf) -> Self {
        let http = crate::http()
            .user_agent("misty-site")
            .connect_timeout(Duration::from_secs(10))
            .read_timeout(Duration::from_secs(30))
            .build()
            .expect("client HTTP");
        let fichier_copies = dossier.with_file_name("misty-site-downloads-copies.json");
        let copies = std::fs::read(&fichier_copies)
            .ok()
            .and_then(|o| serde_json::from_slice(&o).ok())
            .unwrap_or_default();
        Self { dossier, http, fichier_copies, copies: Mutex::new(copies) }
    }

    /// Téléchargements de ce fichier faits par le site lui-même (comptés par GitHub, pas par un joueur).
    pub fn copies(&self, nom: &str) -> u64 {
        self.copies.lock().unwrap().get(nom).copied().unwrap_or(0)
    }

    fn noter_copie(&self, nom: &str) {
        let mut copies = self.copies.lock().unwrap();
        *copies.entry(nom.to_string()).or_default() += 1;
        if let Ok(json) = serde_json::to_vec(&*copies) {
            let _ = std::fs::write(&self.fichier_copies, json);
        }
    }

    /// Client pour relayer un fichier depuis GitHub (pas de délai total).
    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }

    /// La copie locale d'un fichier, si elle est complète.
    pub fn fichier(&self, nom: &str, taille: u64) -> Option<PathBuf> {
        let chemin = self.dossier.join(nom_sur(nom)?);
        let meta = std::fs::metadata(&chemin).ok()?;
        (meta.is_file() && (taille == 0 || meta.len() == taille)).then_some(chemin)
    }

    /// Télécharge les fichiers de cette version qui manquent, puis supprime les anciens.
    pub async fn synchroniser(&self, release: &Release) {
        if tokio::fs::create_dir_all(&self.dossier).await.is_err() {
            return;
        }
        let mut gardes = Vec::new();
        for kind in [Kind::Setup, Kind::Portable, Kind::Linux] {
            let Some(asset) = release.asset(kind) else { continue };
            let Some(nom) = nom_sur(&asset.name) else { continue };
            gardes.push(nom.to_string());
            if self.fichier(&asset.name, asset.size).is_some() {
                continue;
            }
            match self.telecharger(&asset.browser_download_url, nom, asset.size).await {
                Ok(n) => eprintln!("Miroir : {nom} copié ({} Mo)", n / 1_048_576),
                Err(e) => eprintln!("Miroir : {nom} non copié ({e}), les téléchargements passent par GitHub"),
            }
        }
        // ménage : anciennes versions et fichiers temporaires abandonnés
        if gardes.is_empty() {
            return;
        }
        if let Ok(mut dir) = tokio::fs::read_dir(&self.dossier).await {
            while let Ok(Some(e)) = dir.next_entry().await {
                let nom = e.file_name().to_string_lossy().to_string();
                if !gardes.contains(&nom) {
                    let _ = tokio::fs::remove_file(e.path()).await;
                }
            }
        }
    }

    async fn telecharger(&self, url: &str, nom: &str, taille: u64) -> Result<u64, String> {
        let tmp = self.dossier.join(format!("{nom}.part"));
        let mut rep = self
            .http
            .get(url)
            .send()
            .await
            .and_then(|r| r.error_for_status())
            .map_err(|e| e.to_string())?;
        // GitHub a déjà compté ce téléchargement, même s'il est interrompu ensuite
        self.noter_copie(nom);
        let mut f = tokio::fs::File::create(&tmp).await.map_err(|e| e.to_string())?;
        let mut n = 0u64;
        let resultat = async {
            while let Some(morceau) = rep.chunk().await.map_err(|e| e.to_string())? {
                n += morceau.len() as u64;
                f.write_all(&morceau).await.map_err(|e| e.to_string())?;
            }
            f.flush().await.map_err(|e| e.to_string())?;
            if taille != 0 && n != taille {
                return Err(format!("taille {n} au lieu de {taille}"));
            }
            Ok(())
        }
        .await;
        drop(f);
        match resultat {
            Ok(()) => {
                tokio::fs::rename(&tmp, self.dossier.join(nom)).await.map_err(|e| e.to_string())?;
                Ok(n)
            }
            Err(e) => {
                let _ = tokio::fs::remove_file(&tmp).await;
                Err(e)
            }
        }
    }
}

/// Nom de fichier sans chemin (un nom venant de GitHub ne doit jamais sortir du dossier).
fn nom_sur(nom: &str) -> Option<&str> {
    let ok = !nom.is_empty()
        && !nom.starts_with('.')
        && !nom.contains(['/', '\\'])
        && Path::new(nom).file_name().is_some_and(|f| f == nom);
    ok.then_some(nom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noms_refuses() {
        assert_eq!(nom_sur("Misty-Setup-0.3.4.exe"), Some("Misty-Setup-0.3.4.exe"));
        assert_eq!(nom_sur("../etc/passwd"), None);
        assert_eq!(nom_sur("a/b.zip"), None);
        assert_eq!(nom_sur(".."), None);
        assert_eq!(nom_sur(""), None);
    }

    #[test]
    fn copies_gardees_sur_le_disque() {
        let base = std::env::temp_dir().join("misty-site-test-copies");
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let m = Miroir::new(base.join("misty-site-downloads"));
        assert_eq!(m.copies("Misty-0.4.0.zip"), 0);
        m.noter_copie("Misty-0.4.0.zip");
        m.noter_copie("Misty-0.4.0.zip");
        // redémarrage : les copies déjà faites sont relues
        let m = Miroir::new(base.join("misty-site-downloads"));
        assert_eq!(m.copies("Misty-0.4.0.zip"), 2);
        assert_eq!(m.copies("Misty-Setup-0.4.0.exe"), 0);
    }
}
