//! Contributions des auteurs de Mistly sur les trois dépôts (launcher, client, site), affichées
//! dans la bulle qui s'ouvre au survol de leur nom en bas de chaque page.
//!
//! Lues dans les statistiques de GitHub (`/repos/<repo>/stats/contributors`) toutes les 6 heures.
//! GitHub calcule ces statistiques à la demande : la première réponse est souvent un 202 (« en
//! cours »), on redemande donc quelques secondes plus tard. Les pages lisent la dernière
//! version en mémoire (rien tant que la première lecture n'a pas abouti).

use serde::Deserialize;
use std::sync::{LazyLock, RwLock};
use std::time::Duration;

const INTERVALLE: Duration = Duration::from_secs(6 * 3600);
/// Semaines montrées dans le petit graphique.
pub const SEMAINES: usize = 8;

/// Les auteurs affichés, avec leurs comptes GitHub (un auteur peut en avoir plusieurs).
pub const AUTEURS: [(&str, &[&str]); 3] = [
    ("EnderMythex", &["EnderMythex"]),
    ("xtheox_x", &["xTheoxreborn", "xTheoxo"]),
    ("Claude", &["claude"]),
];

#[derive(Clone, Debug, Default)]
pub struct Contribution {
    pub commits: u64,
    pub ajouts: u64,
    pub retraits: u64,
    /// Commits par semaine, des plus anciennes aux plus récentes (`SEMAINES` valeurs)
    pub semaines: Vec<u64>,
    /// Rang parmi les auteurs (1 = le plus de commits)
    pub rang: usize,
}

#[derive(Deserialize)]
struct Semaine {
    w: i64,
    a: u64,
    d: u64,
    c: u64,
}

#[derive(Deserialize)]
struct Compte {
    login: String,
}

#[derive(Deserialize)]
struct Contributeur {
    author: Option<Compte>,
    weeks: Vec<Semaine>,
}

static STATS: LazyLock<RwLock<Option<Vec<Contribution>>>> = LazyLock::new(|| RwLock::new(None));

/// Les contributions de l'auteur `nom` (un des `AUTEURS`), si elles sont connues.
pub fn de(nom: &str) -> Option<Contribution> {
    let i = AUTEURS.iter().position(|(n, _)| *n == nom)?;
    STATS.read().unwrap().as_ref().map(|s| s[i].clone())
}

/// Le plus grand nombre de commits en une semaine, tous auteurs confondus (échelle commune des
/// graphiques).
pub fn maximum() -> u64 {
    STATS.read().unwrap().as_ref().map_or(1, |s| s.iter().flat_map(|c| c.semaines.iter().copied()).max().unwrap_or(1).max(1))
}

/// Regroupe les statistiques des dépôts par auteur (les semaines sont des horodatages UNIX du
/// dimanche, communs à tous les dépôts).
fn regrouper(depots: &[Vec<Contributeur>], maintenant: i64) -> Vec<Contribution> {
    let semaine = 7 * 86_400;
    // le début de la semaine en cours, puis les SEMAINES dernières
    let derniere = depots
        .iter()
        .flatten()
        .flat_map(|c| c.weeks.iter().map(|w| w.w))
        .max()
        .unwrap_or(maintenant - maintenant.rem_euclid(semaine));
    let premiere = derniere - (SEMAINES as i64 - 1) * semaine;
    let mut liste: Vec<Contribution> = AUTEURS
        .iter()
        .map(|(_, comptes)| {
            let mut c = Contribution { semaines: vec![0; SEMAINES], ..Default::default() };
            for x in depots.iter().flatten() {
                let Some(login) = x.author.as_ref().map(|a| a.login.as_str()) else { continue };
                if !comptes.iter().any(|k| k.eq_ignore_ascii_case(login)) {
                    continue;
                }
                for w in &x.weeks {
                    c.commits += w.c;
                    c.ajouts += w.a;
                    c.retraits += w.d;
                    if w.w >= premiere && w.w <= derniere {
                        c.semaines[((w.w - premiere) / semaine) as usize] += w.c;
                    }
                }
            }
            c
        })
        .collect();
    let mut ordre: Vec<usize> = (0..liste.len()).collect();
    ordre.sort_by(|a, b| liste[*b].commits.cmp(&liste[*a].commits));
    for (rang, i) in ordre.into_iter().enumerate() {
        liste[i].rang = rang + 1;
    }
    liste
}

pub async fn rafraichir_en_boucle(api_base: String, token: Option<String>, depots: [String; 3]) {
    let http = crate::http()
        .user_agent("misty-site")
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .build()
        .expect("client HTTP");
    loop {
        let mut lus = Vec::new();
        for repo in &depots {
            // 202 : GitHub calcule encore, on redemande un peu plus tard
            for _ in 0..6 {
                let mut req = http.get(format!("{api_base}/repos/{repo}/stats/contributors")).header("Accept", "application/vnd.github+json");
                if let Some(t) = &token {
                    req = req.bearer_auth(t);
                }
                match req.send().await {
                    Ok(r) if r.status() == reqwest::StatusCode::OK => {
                        if let Ok(liste) = r.json::<Vec<Contributeur>>().await {
                            lus.push(liste);
                        }
                        break;
                    }
                    Ok(r) if r.status() == reqwest::StatusCode::ACCEPTED => tokio::time::sleep(Duration::from_secs(10)).await,
                    _ => break,
                }
            }
        }
        if lus.len() == depots.len() {
            let maintenant = time::OffsetDateTime::now_utc().unix_timestamp();
            *STATS.write().unwrap() = Some(regrouper(&lus, maintenant));
            tokio::time::sleep(INTERVALLE).await;
        } else {
            // un dépôt n'a pas répondu : on réessaie bientôt
            tokio::time::sleep(Duration::from_secs(600)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regroupe_les_comptes_et_les_depots() {
        let s = 7 * 86_400;
        let json = |login: &str, c: u64| format!(r#"{{"author":{{"login":"{login}"}},"weeks":[{{"w":{},"a":10,"d":2,"c":{c}}},{{"w":{},"a":5,"d":1,"c":1}}]}}"#, 10 * s, 11 * s);
        let a: Vec<Contributeur> = serde_json::from_str(&format!("[{},{}]", json("EnderMythex", 3), json("xTheoxo", 1))).unwrap();
        let b: Vec<Contributeur> = serde_json::from_str(&format!("[{},{}]", json("xTheoxreborn", 4), json("claude", 9))).unwrap();
        let r = regrouper(&[a, b], 11 * s);
        // xtheox_x : ses deux comptes additionnés
        assert_eq!(r[1].commits, (1 + 1) + (4 + 1));
        assert_eq!(r[1].ajouts, 30);
        assert_eq!(r[1].semaines[SEMAINES - 1], 2);
        assert_eq!(r[1].semaines[SEMAINES - 2], 5);
        // rangs : Claude, xtheox_x, EnderMythex
        assert_eq!((r[2].rang, r[1].rang, r[0].rang), (1, 2, 3));
    }
}
