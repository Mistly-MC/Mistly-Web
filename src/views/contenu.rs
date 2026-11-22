//! Pages de la roadmap (`/roadmap`) et des articles (`/articles`, `/articles/<slug>`).
//! Les textes de ces pages sont en français et en anglais (les autres langues voient l'anglais).

use super::icons;
use super::layout::{self, DISCORD_URL, Head, Page};
use crate::contenu::{self, Article, Avancement};
use crate::format;
use crate::i18n::Lang;
use maud::{Markup, PreEscaped, html};

/// Dépôt du site, pour le lien « Modifier cette page ».
const WEB_REPO: &str = "Mistly-MC/Mistly-Web";

/// Un texte en français ou en anglais.
fn t<'a>(p: &Page, fr: &'a str, en: &'a str) -> &'a str {
    if p.lang == Lang::Fr { fr } else { en }
}

// ---------- Roadmap ----------

pub fn roadmap(p: &Page) -> Markup {
    let colonnes = [
        (Avancement::Progress, t(p, "En cours", "In progress"), t(p, "Ce sur quoi on travaille en ce moment.", "What we're working on right now.")),
        (Avancement::Planned, t(p, "Prévu", "Planned"), t(p, "La suite, sans date promise.", "What comes next, with no promised date.")),
        (Avancement::Done, t(p, "Terminé", "Shipped"), t(p, "Déjà entre vos mains.", "Already in your hands.")),
    ];
    let chantiers = contenu::roadmap();
    let main = html! {
        section.hero.ct-hero {
            div.col.ct-head {
                span.label { (p.tr.nav_roadmap) }
                h1 { (t(p, "Où va", "Where Mistly")) " " span.soft { (t(p, "Mistly.", "is heading.")) } }
                p.sub { (t(p, "Ce qui est en cours, ce qui arrive et ce qui est déjà sorti, pour le launcher, Mistly Client et le site.", "What's in progress, what's coming and what has shipped, for the launcher, Mistly Client and the site.")) }
                div.ct-ctas {
                    a.btn.play href="/patch-notes" { (icons::notes()) span { (p.tr.nav_patch_notes) } }
                    a.btn href=(DISCORD_URL) target="_blank" rel="noopener" { (icons::discord()) span { (t(p, "Proposer une idée", "Suggest an idea")) } }
                }
            }
        }
        section.blk {
            div.col {
                div.cells.c3.rm-board {
                    @for (statut, titre, sous) in colonnes {
                        @let items: Vec<_> = chantiers.iter().filter(|c| c.status == statut).collect();
                        div.rm-col.(statut.slug()) {
                            header {
                                span.rm-dot {}
                                h2 { (titre) }
                                span.rm-n { (items.len()) }
                            }
                            p.rm-sub { (sous) }
                            ul {
                                @for c in items {
                                    li.rm-card {
                                        div.rm-tags {
                                            span.rm-proj.(c.project.slug()) { (c.project.nom()) }
                                            @if let Some(v) = &c.version { span.rm-ver { "v" (v) } }
                                        }
                                        h3 { (c.titre(p.lang)) }
                                        p { (c.description(p.lang)) }
                                    }
                                }
                            }
                        }
                    }
                }
                p.ct-note {
                    (t(p, "La roadmap donne une direction, pas des dates : l'ordre peut changer. Ce qui sort est détaillé dans les ", "The roadmap gives a direction, not dates: the order can change. What ships is detailed in the "))
                    a href="/patch-notes" { (p.tr.nav_patch_notes) } "."
                }
            }
        }
    };
    layout::render(
        p,
        Head {
            title: &format!("{} – Mistly MC", p.tr.nav_roadmap),
            description: t(p, "La roadmap de Mistly : ce qui est en cours, prévu et terminé pour le launcher, Mistly Client et le site.", "Mistly's roadmap: what's in progress, planned and shipped for the launcher, Mistly Client and the site."),
            jsonld: None,
            css: &["home", "contenu"],
            scripts: &[],
            body_class: "roadmap-page",
        },
        main,
    )
}

// ---------- Articles ----------

fn meta(p: &Page, a: &Article) -> Markup {
    html! {
        span.ar-meta {
            @if let Some(tag) = &a.tag { span.ar-tag { (tag) } }
            time datetime=(a.date.to_string()) { (format::jour(p.tr, a.date, true)) }
            span { "·" }
            span { (a.minutes) " min" }
        }
    }
}

pub fn articles(p: &Page) -> Markup {
    let liste = contenu::articles(p.lang);
    let main = html! {
        section.hero.ct-hero {
            div.col.ct-head {
                span.label { (p.tr.nav_blog) }
                h1 { (t(p, "Les nouvelles", "News from")) " " span.soft { (t(p, "de Mistly.", "Mistly.")) } }
                p.sub { (t(p, "Annonces des nouvelles versions, coulisses du développement et guides.", "Release announcements, behind the scenes and guides.")) }
                div.ct-ctas {
                    a.btn.play href="/roadmap" { (icons::layers()) span { (p.tr.nav_roadmap) } }
                    a.btn href=(DISCORD_URL) target="_blank" rel="noopener" { (icons::discord()) span { "Discord" } }
                }
            }
        }
        section.blk {
            div.col {
                h2.blk-title { (p.tr.nav_blog) sup { "(" (liste.len()) ")" } }
                div.cells.c2.ar-grid {
                    @for (i, a) in liste.iter().enumerate() {
                        a.ar-card.une[i == 0] href=(format!("/articles/{}", a.slug)) {
                            (meta(p, a))
                            h3 { (a.title) }
                            p { (a.description) }
                            span.ar-more { (t(p, "Lire l'article", "Read the article")) (icons::arrow_right()) }
                        }
                    }
                }
            }
        }
    };
    layout::render(
        p,
        Head {
            title: &format!("{} – Mistly MC", p.tr.nav_blog),
            description: t(p, "Les articles de Mistly : nouvelles versions, coulisses et guides.", "Mistly articles: new releases, behind the scenes and guides."),
            jsonld: None,
            css: &["home", "contenu"],
            scripts: &[],
            body_class: "articles-page",
        },
        main,
    )
}

pub fn article(p: &Page, a: &Article) -> Markup {
    let tr = p.tr;
    let autres: Vec<&Article> = contenu::articles(p.lang).into_iter().filter(|x| x.slug != a.slug).take(3).collect();
    let main = html! {
        div.col.docs-grid {
            aside.docs-side aria-label=(tr.nav_blog) {
                nav.docs-nav {
                    div.nav-sec {
                        span.small { (tr.nav_blog) }
                        ul {
                            @for x in contenu::articles(p.lang) {
                                li { a href=(format!("/articles/{}", x.slug)) aria-current=[(x.slug == a.slug).then_some("page")] { (x.title) } }
                            }
                        }
                    }
                }
            }
            article.doc {
                nav.crumbs aria-label="Breadcrumb" {
                    a href="/" { "Mistly" }
                    span { "/" }
                    a href="/articles" { (tr.nav_blog) }
                }
                @if a.lang_dir != p.lang.code() && !tr.legal_untranslated.is_empty() {
                    p.untranslated { (icons::globe()) (tr.legal_untranslated) }
                }
                (meta(p, a))
                h1 { (a.title) }
                p.lead { (a.description) }
                div.md-doc { (PreEscaped(&a.html)) }
                @if !autres.is_empty() {
                    div.ar-next {
                        span.small { (t(p, "À lire aussi", "Read next")) }
                        @for x in autres {
                            a href=(format!("/articles/{}", x.slug)) { strong { (x.title) } (icons::arrow_right()) }
                        }
                    }
                }
                div.doc-foot {
                    a href=(format!("https://github.com/{WEB_REPO}/blob/main/{}", a.path)) target="_blank" rel="noopener" { (icons::github()) (tr.docs_edit) }
                }
            }
            @if !a.toc.is_empty() {
                aside.docs-toc {
                    span.small { (tr.docs_on_page) }
                    ul {
                        @for e in &a.toc {
                            li.lvl3[e.level == 3] { a href=(format!("#{}", e.id)) { (e.text) } }
                        }
                    }
                }
            }
        }
    };
    layout::render(
        p,
        Head {
            title: &format!("{} – Mistly MC", a.title),
            description: &a.description,
            jsonld: None,
            css: &["docs", "contenu"],
            scripts: &[],
            body_class: "docs",
        },
        main,
    )
}
