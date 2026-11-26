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
//! Déploiements : les dernières exécutions GitHub Actions du launcher, de Mistly Client et du site,
//! affichées sur la page de statut.
//!
//! Relues toutes les 2 minutes en tâche de fond, avec l'ETag de la réponse précédente : une
//! réponse 304 (rien de nouveau) ne compte pas dans la limite de l'API GitHub. Les pages ne lisent
//! que ce qui est en mémoire.

use serde::Deserialize;
use std::sync::Mutex;
use std::time::Duration;

const INTERVALLE: Duration = Duration::from_secs(120);
/// Exécutions gardées par projet.
pub const PAR_PROJET: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Projet {
    Launcher,
    Client,
    Site,
}

/// Une exécution d'un workflow.
#[derive(Clone, Debug, Deserialize)]
pub struct Execution {
    /// Nom du workflow (« Release », « CI », « Deploy »…)
    pub name: Option<String>,
    /// Titre affiché : le message du commit, ou le tag
    pub display_title: Option<String>,
    /// queued, in_progress, completed…
    pub status: Option<String>,
    /// success, failure, cancelled… (quand c'est fini)
    pub conclusion: Option<String>,
    pub head_branch: Option<String>,
    pub head_sha: Option<String>,
    pub html_url: String,
    pub run_number: Option<u64>,
    #[serde(with = "time::serde::rfc3339::option", default)]
    pub run_started_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

/// L'état d'une exécution, simplifié pour l'affichage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Issue {
    Reussi,
    Echoue,
    EnCours,
    Annule,
    EnAttente,
}

impl Issue {
    pub fn slug(self) -> &'static str {
        match self {
            Issue::Reussi => "ok",
            Issue::Echoue => "fail",
            Issue::EnCours => "running",
            Issue::Annule => "cancel",
            Issue::EnAttente => "queued",
        }
    }
}

impl Execution {
    pub fn issue(&self) -> Issue {
        match (self.status.as_deref(), self.conclusion.as_deref()) {
            (Some("completed"), Some("success")) => Issue::Reussi,
            (Some("completed"), Some("cancelled" | "skipped" | "neutral" | "stale")) => Issue::Annule,
            (Some("completed"), _) => Issue::Echoue,
            (Some("in_progress"), _) => Issue::EnCours,
            _ => Issue::EnAttente,
        }
    }

    /// Durée en secondes (jusqu'à maintenant si l'exécution tourne encore).
    pub fn duree(&self) -> Option<i64> {
        let debut = self.run_started_at?;
        let fin = if matches!(self.issue(), Issue::EnCours | Issue::EnAttente) {
            time::OffsetDateTime::now_utc()
        } else {
            self.updated_at?
        };
        Some((fin - debut).whole_seconds().max(0))
    }

    /// La première ligne du titre.
    pub fn titre(&self) -> &str {
        self.display_title.as_deref().and_then(|t| t.lines().next()).unwrap_or("")
    }
}

#[derive(Deserialize)]
struct Reponse {
    workflow_runs: Vec<Execution>,
}

struct Suivi {
    projet: Projet,
    repo: String,
    etag: Option<String>,
    executions: Option<Vec<Execution>>,
}

pub struct Deploiements {
    http: reqwest::Client,
    api_base: String,
    token: Option<String>,
    suivis: Mutex<Vec<Suivi>>,
}

impl Deploiements {
    pub fn new(api_base: String, token: Option<String>, repos: [(Projet, String); 3]) -> Self {
        let http = crate::http()
            .user_agent("misty-site")
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(15))
            .build()
            .expect("client HTTP");
        let suivis = repos.into_iter().map(|(projet, repo)| Suivi { projet, repo, etag: None, executions: None }).collect();
        Self { http, api_base, token, suivis: Mutex::new(suivis) }
    }

    /// Les projets suivis, dans l'ordre, avec leur dépôt et leurs dernières exécutions (`None` :
    /// pas encore lues).
    pub fn liste(&self) -> Vec<(Projet, String, Option<Vec<Execution>>)> {
        self.suivis.lock().unwrap().iter().map(|s| (s.projet, s.repo.clone(), s.executions.clone())).collect()
    }

    pub async fn rafraichir_en_boucle(self: std::sync::Arc<Self>) {
        loop {
            self.rafraichir().await;
            tokio::time::sleep(INTERVALLE).await;
        }
    }

    async fn rafraichir(&self) {
        let cibles: Vec<(String, Option<String>)> = self.suivis.lock().unwrap().iter().map(|s| (s.repo.clone(), s.etag.clone())).collect();
        for (i, (repo, etag)) in cibles.into_iter().enumerate() {
            let url = format!("{}/repos/{repo}/actions/runs?per_page={PAR_PROJET}&exclude_pull_requests=true", self.api_base);
            let mut req = self.http.get(url).header("Accept", "application/vnd.github+json");
            if let Some(t) = &self.token {
                req = req.bearer_auth(t);
            }
            if let Some(e) = &etag {
                req = req.header("If-None-Match", e);
            }
            let Ok(rep) = req.send().await else { continue };
            if rep.status() == reqwest::StatusCode::NOT_MODIFIED || !rep.status().is_success() {
                continue;
            }
            let etag = rep.headers().get("etag").and_then(|v| v.to_str().ok()).map(str::to_string);
            if let Ok(r) = rep.json::<Reponse>().await {
                let mut suivis = self.suivis.lock().unwrap();
                suivis[i].etag = etag;
                suivis[i].executions = Some(r.workflow_runs);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lit_une_execution_github() {
        let json = r#"{"workflow_runs":[{"name":"Release","display_title":"0.3.9\nbody","status":"completed","conclusion":"success",
            "head_branch":"0.3.9","head_sha":"abc","html_url":"https://github.com/x/y/actions/runs/1","run_number":12,
            "run_started_at":"2026-11-20T10:00:00Z","updated_at":"2026-11-20T10:07:30Z"},
            {"name":"CI","status":"in_progress","conclusion":null,"html_url":"u","run_started_at":null,"updated_at":null}]}"#;
        let r: Reponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.workflow_runs[0].issue(), Issue::Reussi);
        assert_eq!(r.workflow_runs[0].duree(), Some(450));
        assert_eq!(r.workflow_runs[0].titre(), "0.3.9");
        assert_eq!(r.workflow_runs[1].issue(), Issue::EnCours);
        assert_eq!(r.workflow_runs[1].duree(), None);
    }
}
