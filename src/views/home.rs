//! Page d'accueil.

use super::icons;
use super::layout::{self, Head, Page};
use crate::format;
use crate::github::{Kind, Release};
use maud::{Markup, html};

/// Mod loaders proposés par le launcher.
const LOADERS: [&str; 6] = ["Forge", "NeoForge", "Fabric", "Quilt", "OptiFine", "LiteLoader"];

pub fn render(p: &Page, latest: Option<&Release>, downloads: u64) -> Markup {
    let tr = p.tr;
    let main = html! {
        (hero(p))
        (chiffres(p, latest, downloads))
        (rayures())
        (galerie(p))
        (rayures())
        section.blk #features {
            div.col {
                (titre(&tr.feat_label, Some(6)))
                div.cells.c3 {
                    (cellule(icons::bolt(), &tr.c1_t, &tr.c1_p))
                    (cellule(icons::grid(), &tr.c2_t, &tr.c2_p))
                    (cellule(icons::user(), &tr.c3_t, &tr.c3_p))
                    (cellule(icons::refresh(), &tr.c4_t, &tr.c4_p))
                    (cellule(icons::globe(), &tr.c5_t, &tr.c5_p))
                    (cellule(icons::layers(), &tr.c6_t, &tr.c6_p))
                }
            }
        }
        (rayures())
        (pile(p))
        (rayures())
        (horizon(p))
        (rayures())
        (download(p, latest))
        (rayures())
        (faq(p))
        (appel(p))
    };
    layout::render(
        p,
        Head {
            title: &tr.title,
            description: &tr.description,
            jsonld: Some(crate::seo::home_jsonld(p.site_url, tr, p.repo, latest)),
            css: &["home"],
            scripts: &[],
            body_class: "home",
        },
        main,
    )
}

/// Titre de section, avec un compteur en exposant (« Fonctionnalités ⁶ »).
fn titre(texte: &str, compte: Option<usize>) -> Markup {
    html! {
        h2.blk-title { (texte) @if let Some(n) = compte { sup { "(" (n) ")" } } }
    }
}

/// Bande hachurée entre deux sections.
fn rayures() -> Markup {
    html! { div.col.hatch aria-hidden="true" {} }
}

fn hero(p: &Page) -> Markup {
    let tr = p.tr;
    html! {
        section.hero {
            div.col.hero-in {
                div.hero-logo {
                    img src="/assets/icone.png" alt="Mistly Launcher" width="84" height="84";
                    span aria-hidden="true" { "MISTLY" }
                }
                h1 { (tr.hero_title_1) " " span.soft { (tr.hero_title_2) } }
                div.actions {
                    a.btn.play href="/download/setup" { (icons::windows()) (tr.hero_download_win) }
                    span.btn.os-soon aria-disabled="true" title=(tr.os_macos_p) { (icons::apple()) "macOS" span.badge { (tr.os_soon) } }
                    span.btn.os-soon aria-disabled="true" title=(tr.os_linux_p) { (icons::linux()) "Linux" span.badge { (tr.os_soon) } }
                }
            }
            // Le launcher, dans un cadre posé sur le paysage
            div.col.hero-shot-wrap {
                figure.hero-shot {
                    img src=(format!("/assets/screens/{}/home.png", p.lang.screens_dir())) alt=(tr.alt_home) width="1180" height="720" fetchpriority="high";
                }
            }
        }
    }
}

/// Dernier appel à télécharger, sur un fond de caractères ASCII qui ondulent.
fn appel(p: &Page) -> Markup {
    let tr = p.tr;
    html! {
        section.blk.final-cta {
            div.col {
                div.cta-box {
                    pre.ascii-noise aria-hidden="true" {}
                    div.cta-body {
                        img src="/assets/icone.png" alt="" width="56" height="56";
                        h2 { (tr.cta_title) }
                        p { (tr.cta_text) }
                        div.actions {
                            a.btn.play href="/download/setup" { (icons::windows()) (tr.hero_download_win) }
                            a.btn href="/docs" { (tr.nav_docs) (icons::arrow_right()) }
                        }
                    }
                }
            }
        }
    }
}

/// Chiffres clés, en grandes cellules (téléchargements, versions, mod loaders, langues).
fn chiffres(p: &Page, latest: Option<&Release>, downloads: u64) -> Markup {
    let tr = p.tr;
    let mut cases: Vec<(String, &str)> = Vec::new();
    if downloads > 0 {
        cases.push((milliers(p, downloads), &tr.stat_downloads));
    }
    if let Some(n) = p.release_count {
        cases.push((n.to_string(), &tr.stat_releases));
    }
    cases.push((LOADERS.len().to_string(), &tr.stat_loaders));
    cases.push((crate::i18n::Lang::ALL.len().to_string(), &tr.stat_langs));
    if let Some(t) = latest.and_then(|r| r.asset(Kind::Setup)).map(|a| format::size(tr, a.size)) {
        cases.push((t, &tr.stat_size));
    }
    cases.truncate(4);
    html! {
        section.blk.numbers {
            div.col {
                p.caption { (tr.stat_title) }
                dl.cells.big-stats style=(format!("--n:{}", cases.len())) {
                    @for (v, k) in &cases {
                        div { dd { (v) } dt { (majuscule(k)) } }
                    }
                }
            }
        }
    }
}

fn majuscule(t: &str) -> String {
    let mut c = t.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}

/// 2267 → « 2 267 » en français, « 2,267 » en anglais.
fn milliers(p: &Page, n: u64) -> String {
    let sep = match p.lang.code() {
        "en" => ",",
        "es" | "it" | "pt" => ".",
        "zh" | "ja" => ",",
        _ => "\u{202f}",
    };
    let chiffres = n.to_string();
    let mut out = String::new();
    for (i, c) in chiffres.chars().enumerate() {
        if i > 0 && (chiffres.len() - i) % 3 == 0 {
            out.push_str(sep);
        }
        out.push(c);
    }
    out
}

/// Captures du launcher, chacune dans un cadre avec sa légende.
fn galerie(p: &Page) -> Markup {
    let tr = p.tr;
    let images = [
        ("versions", &tr.gal_versions_t, &tr.gal_versions_p),
        ("profiles", &tr.gal_profiles_t, &tr.gal_profiles_p),
        ("skin", &tr.gal_skin_t, &tr.gal_skin_p),
        ("textures", &tr.gal_textures_t, &tr.gal_textures_p),
    ];
    html! {
        section.blk #preview {
            div.col {
                (titre(&tr.prev_label, Some(images.len())))
                div.cells.c2 {
                    @for (i, (img, t, d)) in images.iter().enumerate() {
                        figure.shot.reveal {
                            div.frame {
                                img src=(format!("/assets/screens/{}/{img}.png", p.lang.screens_dir())) alt=(t) width="1180" height="720"
                                    loading=(if i < 2 { "eager" } else { "lazy" });
                            }
                            figcaption { strong { (t) } span { (d) } }
                        }
                    }
                }
            }
        }
    }
}

fn cellule(icon: Markup, title: &str, text: &str) -> Markup {
    html! {
        div.cell.reveal {
            span.ico { (icon) }
            h3 { (title) }
            p { (text) }
        }
    }
}

/// Compatibilité, présentée comme une liste de technologies : une ligne par sujet, des pastilles.
fn pile(p: &Page) -> Markup {
    let tr = p.tr;
    let langues: Vec<&str> = crate::i18n::Lang::ALL.iter().map(|l| l.tr().lang_name.as_str()).collect();
    let lignes: [(&str, Vec<(&str, Option<&str>)>); 4] = [
        (&tr.stack_loaders, LOADERS.iter().map(|l| (*l, None)).collect()),
        (&tr.load_sources, vec![("Modrinth", Some("#1bd96a")), ("CurseForge", Some("#f16436"))]),
        (&tr.stack_system, vec![("Windows 10", None), ("Windows 11", None), ("Java auto", None)]),
        (&tr.stack_langs, langues.iter().map(|l| (*l, None)).collect()),
    ];
    html! {
        section.blk #loaders {
            div.col {
                (titre(&tr.load_label, None))
                p.blk-lead { (tr.load_title) }
                div.stack {
                    @for (i, (nom, puces)) in lignes.iter().enumerate() {
                        div.row {
                            span.k { span.n { (format!("{:02}", i + 1)) } (nom) }
                            div.chips {
                                @for (t, c) in puces {
                                    span.chip { @if let Some(c) = c { i.sw style=(format!("--c:{c}")) {} } (t) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Horizon : Minecraft réécrit en Rust, jouable depuis le launcher.
fn horizon(p: &Page) -> Markup {
    let tr = p.tr;
    let shot = format!("/assets/screens/{}/horizon.png", p.lang.screens_dir());
    html! {
        section.blk #horizon {
            div.col {
                (titre(&tr.hz_label, None))
                div.cells.hz {
                    figure.shot.reveal { div.frame { img src=(shot) alt=(tr.alt_horizon) width="1180" height="720" loading="lazy"; } }
                    div.cell.hz-copy {
                        h3 { (tr.hz_title) }
                        ul.hz-points {
                            li { (icons::check()) (tr.hz_p1) }
                            li { (icons::check()) (tr.hz_p2) }
                            li { (icons::check()) (tr.hz_p3) }
                        }
                        a.more href="https://playhorizon.dev" target="_blank" rel="noopener" { (tr.hz_site) (icons::external()) }
                        p.hz-note { (tr.hz_note) }
                    }
                }
            }
        }
    }
}

fn download(p: &Page, latest: Option<&Release>) -> Markup {
    let tr = p.tr;
    let meta = |kind: Kind| -> Option<String> {
        let r = latest?;
        let a = r.asset(kind)?;
        Some(format!("{} · {}", r.tag_name, format::size(tr, a.size)))
    };
    html! {
        section.blk #download {
            div.col {
                (titre(&tr.dl_label, None))
                p.blk-lead { (tr.dl_text) }
                div.cells.c3.dl {
                    a.dl-card.main href="/download/setup" {
                        span.top { (icons::download()) (tr.inst_t) span.pill { (tr.recommended) } }
                        span.txt { (tr.inst_p) }
                        @if let Some(m) = meta(Kind::Setup) { span.m { (m) " · .exe" } }
                    }
                    a.dl-card href="/download/portable" {
                        span.top { (icons::archive()) (tr.port_t) }
                        span.txt { (tr.port_p) }
                        @if let Some(m) = meta(Kind::Portable) { span.m { (m) " · .zip" } }
                    }
                    div.dl-card.soon aria-disabled="true" {
                        span.top { (icons::apple()) "macOS" span.badge { (tr.os_soon) } }
                        span.txt { (tr.os_macos_p) }
                    }
                    div.dl-card.soon aria-disabled="true" {
                        span.top { (icons::linux()) "Linux" span.badge { (tr.os_soon) } }
                        span.txt { (tr.os_linux_p) }
                    }
                    a.dl-card href="/patch-notes" {
                        span.top { (icons::notes()) (tr.notes_t) }
                        span.txt { (tr.notes_p) }
                    }
                    a.dl-card href=(p.releases_url()) target="_blank" rel="noopener" {
                        span.top { (icons::github()) (tr.github_t) (icons::external()) }
                        span.txt { (tr.github_p) }
                    }
                }
            }
        }
    }
}

fn faq(p: &Page) -> Markup {
    let tr = p.tr;
    let qa = [
        (&tr.q1, &tr.a1),
        (&tr.q2, &tr.a2),
        (&tr.q3, &tr.a3),
        (&tr.q4, &tr.a4),
        (&tr.q5, &tr.a5),
    ];
    html! {
        section.blk #faq {
            div.col {
                (titre(&tr.faq_title, Some(qa.len())))
                div.qa {
                    @for (q, a) in qa {
                        details { summary { span { (q) } (icons::chevron()) } p { (a) } }
                    }
                }
            }
        }
    }
}
