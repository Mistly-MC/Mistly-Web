//! Patch notes : toutes les versions publiées en cartes, filtrables (`/patch-notes`), et une page
//! par version avec ses notes complètes (`/patch-notes/<tag>`).

use super::icons;
use super::layout::{self, Head, Page};
use crate::format;
use crate::github::{self, Release};
use crate::markdown;
use maud::{Markup, PreEscaped, html};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Stable,
    Pre,
}

impl Filter {
    pub fn from_query(q: Option<&str>) -> Filter {
        match q {
            Some("stable") => Filter::Stable,
            Some("pre") => Filter::Pre,
            _ => Filter::All,
        }
    }
    fn slug(self) -> &'static str {
        match self {
            Filter::All => "all",
            Filter::Stable => "stable",
            Filter::Pre => "pre",
        }
    }
    fn keeps(self, r: &Release) -> bool {
        match self {
            Filter::All => true,
            Filter::Stable => !r.prerelease,
            Filter::Pre => r.prerelease,
        }
    }
}

/// Le projet affiché : le launcher, ou Mistly Client (le mod en jeu, publié dans son propre dépôt).
pub struct Projet<'a> {
    pub client: bool,
    /// `owner/repo` du projet affiché
    pub repo: &'a str,
    /// Nombre de versions de chaque projet (onglets)
    pub launcher: Option<usize>,
    pub mod_client: Option<usize>,
}

impl Projet<'_> {
    /// Paramètre à garder dans les liens des filtres.
    fn param(&self) -> &'static str {
        if self.client { "&project=client" } else { "" }
    }
}

pub fn render(p: &Page, releases: Option<&[Release]>, filter: Filter, projet: &Projet) -> Markup {
    let tr = p.tr;
    let releases_url = format!("https://github.com/{}/releases", projet.repo);
    let main = html! {
        section.hero.pn-hero {
            div.wrap { div.pn-copy {
                div.tags {
                    span { (tr.pn_label) }
                    span.sep { "·" }
                    a href=(format!("https://github.com/{}", projet.repo)) target="_blank" rel="noopener" { (icons::github()) (projet.repo) }
                }
                h1 { (tr.pn_h1_1) br; span.accent { (tr.pn_h1_2) } }
                p.sub { (tr.pn_sub) }
                div.actions {
                    a.btn.play href="#versions" { (icons::notes()) (tr.pn_browse) }
                    a.btn.ghost href=(releases_url) target="_blank" rel="noopener" { (icons::github()) (tr.pn_all_github) }
                }
            } }
        }
        section.sec #versions {
            div.wrap.versions {
                // Une barre : le projet à gauche (le launcher, ou Mistly Client, le mod en jeu,
                // publiés séparément), les filtres à droite, sur un fond de caractères ASCII
                div.pn-barre {
                    pre.ascii-noise aria-hidden="true" {}
                    nav.projects aria-label=(tr.pn_project) {
                        a href="/patch-notes#versions" aria-current=[(!projet.client).then_some("page")] {
                            "Launcher" @if let Some(n) = projet.launcher { span { (n) } }
                        }
                        a href="/patch-notes?project=client#versions" aria-current=[projet.client.then_some("page")] {
                            "Mistly Client" @if let Some(n) = projet.mod_client { span { (n) } }
                        }
                    }
                    @if let Some(list) = releases { (filtres(p, list, filter, projet)) }
                }
                @match releases {
                    None => { p.empty { (tr.pn_unavailable) } }
                    Some(list) => { (list_view(p, list, filter, projet)) }
                }
            }
        }
    };
    layout::render(
        p,
        Head {
            title: &tr.pn_title,
            description: &tr.pn_sub,
            jsonld: None,
            css: &["home", "patch-notes"],
            scripts: &[],
            body_class: "notes",
        },
        main,
    )
}

/// Adresse de la page d'une version (`/patch-notes/<tag>`, `?project=client` pour Mistly Client).
fn url_version(tag: &str, client: bool) -> String {
    format!("/patch-notes/{tag}{}", if client { "?project=client" } else { "" })
}

/// Les changements d'une version, en texte brut : les points des listes de ses notes (sans le
/// Markdown : gras, code, liens), pour les aperçus des cartes.
fn points(notes: &str) -> Vec<String> {
    notes
        .lines()
        .filter_map(|l| {
            let l = l.trim_start();
            l.strip_prefix("- ").or_else(|| l.strip_prefix("* "))
        })
        .map(|t| {
            let mut s = String::new();
            let mut reste = t;
            // [texte](lien) → texte
            while let Some(debut) = reste.find('[') {
                s.push_str(&reste[..debut]);
                let apres = &reste[debut + 1..];
                match apres.find("](").and_then(|f| apres[f..].find(')').map(|g| (f, f + g))) {
                    Some((f, g)) => {
                        s.push_str(&apres[..f]);
                        reste = &apres[g + 1..];
                    }
                    None => {
                        s.push('[');
                        reste = apres;
                    }
                }
            }
            s.push_str(reste);
            s.replace("**", "").replace('`', "").trim().to_string()
        })
        .filter(|t| !t.is_empty())
        .collect()
}

/// Le badge d'une version : la dernière, une pré-version ou une version stable.
fn badge(p: &Page, r: &Release, is_latest: bool) -> Markup {
    let tr = p.tr;
    html! {
        span.badge.latest[is_latest].pre[r.prerelease] {
            @if is_latest { (tr.pn_badge_latest) }
            @else if r.prerelease { (tr.pn_badge_pre) }
            @else { (tr.pn_badge_stable) }
        }
    }
}

/// Les filtres (toutes, stables, pré-versions), avec le nombre de versions de chacun.
fn filtres(p: &Page, list: &[Release], filter: Filter, projet: &Projet) -> Markup {
    let tr = p.tr;
    let stable = list.iter().filter(|r| !r.prerelease).count();
    let chips = [
        (Filter::All, &tr.pn_filter_all, list.len()),
        (Filter::Stable, &tr.pn_filter_stable, stable),
        (Filter::Pre, &tr.pn_filter_pre, list.len() - stable),
    ];
    html! {
        nav.chips aria-label=(tr.pn_label) {
            @for (f, label, n) in chips {
                a href=(format!("/patch-notes?filter={}{}#versions", f.slug(), projet.param())) aria-current=[(f == filter).then_some("true")] {
                    (label) span { (n) }
                }
            }
        }
    }
}

fn list_view(p: &Page, list: &[Release], filter: Filter, projet: &Projet) -> Markup {
    let tr = p.tr;
    let latest_tag = github::latest(list).map(|r| r.tag_name.as_str());
    let shown: Vec<&Release> = list.iter().filter(|r| filter.keeps(r)).collect();
    html! {
        @if shown.is_empty() {
            p.empty { (tr.pn_empty) }
        }
        // Une carte par version, la plus récente en grand ; un clic ouvre sa page
        div.pn-grid {
            @for (i, r) in shown.iter().enumerate() {
                @let is_latest = Some(r.tag_name.as_str()) == latest_tag;
                @let pts = points(r.notes());
                @let vedette = i == 0;
                a.pn-card.vedette[vedette] id=(r.tag_name) href=(url_version(&r.tag_name, projet.client)) {
                    pre.ascii-noise aria-hidden="true" {}
                    div.pn-card-top {
                        (badge(p, r, is_latest))
                        span.pn-date { (format::date(tr, r.published_at.as_deref())) }
                    }
                    h3 { (r.title()) }
                    @if r.title() != r.tag_name { code.pn-tag { (r.tag_name) } }
                    @if pts.is_empty() {
                        p.pn-vide { (tr.card_no_notes) }
                    } @else {
                        ul.pn-points {
                            @for pt in pts.iter().take(if vedette { 5 } else { 3 }) { li { (pt) } }
                        }
                    }
                    div.pn-card-bas {
                        @if !pts.is_empty() {
                            span.pn-count {
                                @if pts.len() == 1 { (tr.pn_one_change) }
                                @else { (tr.pn_changes.replace("{0}", &pts.len().to_string())) }
                            }
                        }
                        span.pn-more { (tr.pn_details) (icons::arrow_right()) }
                    }
                }
            }
        }
    }
}

/// Page d'une version : ses notes complètes, ses fichiers, et les versions voisines.
pub fn render_version(p: &Page, list: &[Release], index: usize, projet: &Projet) -> Markup {
    let tr = p.tr;
    let r = &list[index];
    let is_latest = github::latest(list).map(|l| l.tag_name.as_str()) == Some(r.tag_name.as_str());
    let liste = format!("/patch-notes{}#versions", if projet.client { "?project=client" } else { "" });
    let nom_projet = if projet.client { "Mistly Client" } else { "Mistly Launcher" };
    // la liste va de la plus récente à la plus ancienne
    let plus_recente = index.checked_sub(1).map(|i| &list[i]);
    let plus_ancienne = list.get(index + 1);
    let titre = format!("{} – {}", r.title(), tr.pn_title);
    let description = points(r.notes()).into_iter().take(3).collect::<Vec<_>>().join(" · ");
    let main = html! {
        section.hero.pn-hero.pn-version-hero {
            div.wrap { div.pn-copy {
                a.pn-retour href=(liste) { (icons::arrow_left()) (tr.pn_back) }
                div.tags {
                    (badge(p, r, is_latest))
                    span { (nom_projet) }
                    span.sep { "·" }
                    span { (format::date(tr, r.published_at.as_deref())) }
                }
                h1 { (r.title()) }
                @if r.title() != r.tag_name { p.sub { code { (r.tag_name) } } }
                div.actions {
                    a.btn.ghost href=(r.html_url) target="_blank" rel="noopener" { (icons::github()) (tr.pn_view_github) }
                }
            } }
        }
        section.sec {
            div.wrap.pn-version {
                // les notes et les fichiers ensemble : le panneau des fichiers, collant, s'arrête
                // avec les notes au lieu de passer sur la navigation
                div.pn-corps {
                    article.pn-notes {
                        div.md {
                            @if r.notes().is_empty() { p { (tr.card_no_notes) } }
                            @else { (PreEscaped(markdown::render(r.notes()))) }
                        }
                    }
                    @if !r.assets.is_empty() {
                        aside.pn-fichiers {
                            h2 { (tr.pn_files) }
                            div.files {
                                @for a in &r.assets {
                                    a.file href=(a.browser_download_url) {
                                        (icons::download())
                                        span { (a.name) }
                                        span.sz { (format::size(tr, a.size)) }
                                    }
                                }
                            }
                        }
                    }
                }
                nav.pn-voisines aria-label=(tr.pn_label) {
                    @if let Some(v) = plus_ancienne {
                        a.pn-voisine href=(url_version(&v.tag_name, projet.client)) {
                            pre.ascii-noise aria-hidden="true" {}
                            span.pn-sens { (icons::arrow_left()) (tr.pn_older) }
                            strong { (v.title()) }
                        }
                    } @else { span {} }
                    @if let Some(v) = plus_recente {
                        a.pn-voisine.suivante href=(url_version(&v.tag_name, projet.client)) {
                            pre.ascii-noise aria-hidden="true" {}
                            span.pn-sens { (tr.pn_newer) (icons::arrow_right()) }
                            strong { (v.title()) }
                        }
                    }
                }
            }
        }
    };
    layout::render(
        p,
        Head {
            title: &titre,
            description: if description.is_empty() { &tr.pn_sub } else { &description },
            jsonld: None,
            css: &["home", "patch-notes"],
            scripts: &[],
            body_class: "notes",
        },
        main,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_en_texte_brut() {
        let notes = "### Launcher\n- **New:** a [Discord](https://x.y) button\n* Fixed `data.txt`\nSome text\n  - nested point\n";
        assert_eq!(points(notes), ["New: a Discord button", "Fixed data.txt", "nested point"]);
        assert!(points("No list here").is_empty());
    }
}
