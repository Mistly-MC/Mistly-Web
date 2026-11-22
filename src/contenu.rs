//! Roadmap (`/roadmap`) et articles (`/articles`), écrits dans `content/` et intégrés au binaire.
//!
//! - `content/roadmap.toml` : une entrée `[[item]]` par chantier (`status` = `done`, `progress` ou
//!   `planned`, `project`, `title_fr`, `title_en`, `desc_fr`, `desc_en`, `version` facultative).
//! - `content/articles/<fr|en>/<slug>.md` : un article, avec un en-tête TOML entre `+++`
//!   (`title`, `description`, `date` au format AAAA-MM-JJ, `tag` facultatif). Le même nom de
//!   fichier dans les deux dossiers, c'est le même article traduit ; s'il manque dans une langue,
//!   l'autre version est affichée.
//!
//! Tout est lu au démarrage (`init`) : une erreur dans un fichier empêche le site de démarrer et
//! fait échouer `cargo test`.

use crate::i18n::Lang;
use crate::markdown::{self, TocEntry};
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use std::sync::LazyLock;

static ARTICLES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/articles");

// ---------- Roadmap ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Avancement {
    Done,
    Progress,
    Planned,
}

impl Avancement {
    pub fn slug(self) -> &'static str {
        match self {
            Avancement::Done => "done",
            Avancement::Progress => "progress",
            Avancement::Planned => "planned",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Projet {
    Launcher,
    Client,
    Site,
    Horizon,
}

impl Projet {
    pub fn nom(self) -> &'static str {
        match self {
            Projet::Launcher => "Launcher",
            Projet::Client => "Mistly Client",
            Projet::Site => "mistly.cc",
            Projet::Horizon => "Horizon",
        }
    }
    pub fn slug(self) -> &'static str {
        match self {
            Projet::Launcher => "launcher",
            Projet::Client => "client",
            Projet::Site => "site",
            Projet::Horizon => "horizon",
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chantier {
    pub status: Avancement,
    pub project: Projet,
    title_fr: String,
    title_en: String,
    desc_fr: String,
    desc_en: String,
    pub version: Option<String>,
}

impl Chantier {
    pub fn titre(&self, lang: Lang) -> &str {
        if lang == Lang::Fr { &self.title_fr } else { &self.title_en }
    }
    pub fn description(&self, lang: Lang) -> &str {
        if lang == Lang::Fr { &self.desc_fr } else { &self.desc_en }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FichierRoadmap {
    item: Vec<Chantier>,
}

static ROADMAP: LazyLock<Vec<Chantier>> = LazyLock::new(|| {
    toml::from_str::<FichierRoadmap>(include_str!("../content/roadmap.toml"))
        .unwrap_or_else(|e| panic!("content/roadmap.toml : {e}"))
        .item
});

pub fn roadmap() -> &'static [Chantier] {
    &ROADMAP
}

// ---------- Articles ----------

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EnTete {
    title: String,
    description: String,
    date: String,
    tag: Option<String>,
}

pub struct Article {
    pub slug: String,
    /// `fr` ou `en` : la langue dans laquelle l'article est écrit
    pub lang_dir: &'static str,
    /// Chemin dans le dépôt, pour le lien « Modifier sur GitHub »
    pub path: String,
    pub title: String,
    pub description: String,
    pub date: time::Date,
    pub tag: Option<String>,
    pub html: String,
    pub toc: Vec<TocEntry>,
    /// Temps de lecture estimé, en minutes
    pub minutes: usize,
}

fn lire(lang_dir: &'static str, nom: &str, src: &str) -> Article {
    let path = format!("content/articles/{lang_dir}/{nom}");
    let slug = nom.trim_end_matches(".md").to_string();
    let rest = src.strip_prefix("+++\n").unwrap_or_else(|| panic!("{path} : l'en-tête +++ manque"));
    let (front, body) = rest.split_once("\n+++\n").unwrap_or_else(|| panic!("{path} : en-tête +++ non refermé"));
    let e: EnTete = toml::from_str(front).unwrap_or_else(|err| panic!("{path} : en-tête invalide : {err}"));
    let date = time::Date::parse(&e.date, time::macros::format_description!("[year]-[month]-[day]"))
        .unwrap_or_else(|err| panic!("{path} : date invalide : {err}"));
    let (html, toc) = markdown::render_doc(body);
    let minutes = body.split_whitespace().count().div_ceil(220).max(1);
    Article { slug, lang_dir, path, title: e.title, description: e.description, date, tag: e.tag, html, toc, minutes }
}

static ARTICLES: LazyLock<Vec<Article>> = LazyLock::new(|| {
    let mut tous: Vec<Article> = ["fr", "en"]
        .into_iter()
        .flat_map(|l| {
            ARTICLES_DIR.get_dir(l).into_iter().flat_map(move |d| {
                d.files().filter_map(move |f| {
                    let nom = f.path().file_name()?.to_str()?;
                    nom.ends_with(".md").then(|| {
                        let src = f.contents_utf8().unwrap_or_else(|| panic!("{nom} : UTF-8 attendu"));
                        lire(l, nom, &src.replace("\r\n", "\n"))
                    })
                })
            })
        })
        .collect();
    // les plus récents d'abord
    tous.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.slug.cmp(&b.slug)));
    tous
});

/// Charge la roadmap et les articles (appelé au démarrage pour échouer tôt en cas d'erreur).
pub fn init() {
    LazyLock::force(&ROADMAP);
    LazyLock::force(&ARTICLES);
}

/// Le dossier préféré pour une langue : le français pour le français, l'anglais pour les autres.
fn dossier(lang: Lang) -> &'static str {
    if lang == Lang::Fr { "fr" } else { "en" }
}

/// Un article dans la langue demandée, sinon dans l'autre.
pub fn article(lang: Lang, slug: &str) -> Option<&'static Article> {
    let pref = dossier(lang);
    let candidats = || ARTICLES.iter().filter(move |a| a.slug == slug);
    candidats().find(|a| a.lang_dir == pref).or_else(|| candidats().next())
}

/// Tous les articles, une fois chacun (dans la langue demandée quand il existe), du plus récent
/// au plus ancien.
pub fn articles(lang: Lang) -> Vec<&'static Article> {
    let mut vus: Vec<&str> = Vec::new();
    let mut liste = Vec::new();
    for a in ARTICLES.iter() {
        if vus.contains(&a.slug.as_str()) {
            continue;
        }
        vus.push(&a.slug);
        liste.push(article(lang, &a.slug).unwrap_or(a));
    }
    liste
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_roadmap_et_les_articles_se_chargent() {
        init();
        assert!(!roadmap().is_empty());
        for s in [Avancement::Done, Avancement::Progress, Avancement::Planned] {
            assert!(roadmap().iter().any(|c| c.status == s), "aucun chantier {s:?}");
        }
        let fr = articles(Lang::Fr);
        assert!(!fr.is_empty());
        // un article traduit est servi dans la langue demandée
        let slug = &fr[0].slug;
        assert_eq!(article(Lang::Fr, slug).unwrap().lang_dir, "fr");
        assert_eq!(article(Lang::Ja, slug).unwrap().lang_dir, "en");
        assert!(article(Lang::En, "n-existe-pas").is_none());
    }
}
