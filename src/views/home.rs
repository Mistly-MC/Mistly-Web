//! Page d'accueil.

use super::icons;
use super::layout::{self, Head, Page};
use crate::format;
use crate::github::{Kind, Release};
use maud::{Markup, html};

/// Mod loaders proposés par le launcher.
const LOADERS: [&str; 6] = ["Forge", "NeoForge", "Fabric", "Quilt", "OptiFine", "LiteLoader"];

/// `linux` : le visiteur est sous Linux, la version Linux passe devant (bouton blanc, carte
/// recommandée) ; sinon Windows.
pub fn render(p: &Page, latest: Option<&Release>, downloads: u64, linux: bool) -> Markup {
    let tr = p.tr;
    let main = html! {
        (hero(p, linux))
        (chiffres(p, latest, downloads))
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
        (download(p, latest, linux))
        (rayures())
        (faq(p))
        (appel(p, linux))
    };
    layout::render(
        p,
        Head {
            title: &tr.title,
            description: &tr.description,
            jsonld: Some(crate::seo::home_jsonld(p.site_url, tr, p.repo, latest)),
            css: &["home"],
            scripts: &["/js/sim.js"],
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

/// Le bouton principal (blanc) pour le système du visiteur.
fn bouton_principal(p: &Page, linux: bool) -> Markup {
    let tr = p.tr;
    html! {
        @if linux {
            a.btn.play href="/download/linux" { (icons::linux()) (tr.hero_download_linux) }
        } @else {
            a.btn.play href="/download/setup" { (icons::windows()) (tr.hero_download_win) }
        }
    }
}

fn hero(p: &Page, linux: bool) -> Markup {
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
                    // le système du visiteur en blanc, l'autre au milieu, macOS toujours à droite
                    (bouton_principal(p, linux))
                    @if linux {
                        a.btn href="/download/setup" { (icons::windows()) "Windows" }
                    } @else {
                        a.btn href="/download/linux" title=(tr.os_linux_p) { (icons::linux()) "Linux" }
                    }
                    span.btn.os-soon aria-disabled="true" title=(tr.os_macos_p) { (icons::apple()) "macOS" span.badge { (tr.os_soon) } }
                }
            }
            // Le launcher, dans un cadre posé sur le paysage : on peut cliquer ses onglets
            (apercu(p))
        }
    }
}

/// Dernier appel à télécharger, sur un fond de caractères ASCII qui ondulent.
fn appel(p: &Page, linux: bool) -> Markup {
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
                            (bouton_principal(p, linux))
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
        "en" | "zh" | "ja" | "ko" => ",",
        "es" | "it" | "pt" | "de" | "nl" | "tr" | "id" => ".",
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

/// Une capture du launcher, en thème sombre et en thème clair : le CSS montre celle du thème du
/// site (`.th-dark` / `.th-light`), et le navigateur ne charge que celle qui s'affiche.
fn capture(p: &Page, nom: &str, alt: &str, chargement: &str) -> Markup {
    let dir = p.lang.screens_dir();
    let v = layout::ASSET_VERSION.as_str();
    html! {
        img.th-dark src=(format!("/assets/screens/{dir}/{nom}.png?v={v}")) alt=(alt) width="1180" height="720" loading=(chargement) fetchpriority=[(chargement == "eager").then_some("high")];
        img.th-light src=(format!("/assets/screens/{dir}/light/{nom}.png?v={v}")) alt=(alt) width="1180" height="720" loading="lazy";
    }
}

/// Zone cliquable posée sur la capture, en pixels de la capture (1180 × 720) : `x0, y0, x1, y1`,
/// l'écran qu'elle ouvre, et les écrans où elle existe (vide : tous).
struct Zone {
    r: (u32, u32, u32, u32),
    va: &'static str,
    sur: &'static [&'static str],
}

const fn z(x0: u32, y0: u32, x1: u32, y1: u32, va: &'static str, sur: &'static [&'static str]) -> Zone {
    Zone { r: (x0, y0, x1, y1), va, sur }
}

/// Zones communes à toutes les captures (logo, avatar, options, choix de version) et propres à
/// certaines (croix de fermeture, onglet Horizon). Les onglets du haut dépendent de la langue.
fn zones(dir: &str) -> Vec<Zone> {
    let onglets: [(u32, u32, &'static str); 6] = if dir == "fr" {
        [(252, 310, "home"), (318, 366, "skin"), (374, 436, "profiles"), (444, 576, "notes"), (583, 721, "textures"), (729, 782, "friends")]
    } else {
        [(263, 311, "home"), (319, 367, "skin"), (375, 445, "profiles"), (453, 566, "notes"), (574, 686, "textures"), (694, 763, "friends")]
    };
    let horizon = if dir == "fr" { (623, 713) } else { (625, 715) };
    let mut v = vec![
        z(14, 14, 78, 78, "home", &[]),
        z(105, 27, 217, 66, "home", &[]),
        z(14, 642, 78, 706, "settings", &[]),
        z(90, 642, 472, 706, "versions", &[]),
        z(1102, 642, 1166, 706, "skin", &[]),
        z(1124, 88, 1168, 132, "home", &["versions", "horizon"]),
        z(horizon.0, 144, horizon.1, 181, "horizon", &["versions"]),
        z(90, 144, 188, 181, "versions", &["horizon"]),
        z(1113, 99, 1157, 143, "home", &["settings"]),
    ];
    v.extend(onglets.iter().map(|&(x0, x1, va)| z(x0, 24, x1, 66, va, &[])));
    v
}

/// Le launcher en haut de la page, dont on peut cliquer les onglets (une capture par écran, rien
/// ne marche vraiment). Les pastilles en dessous font la même chose, et suffisent sur téléphone.
fn apercu(p: &Page) -> Markup {
    let tr = p.tr;
    let ecrans: [(&str, &str, &str); 9] = [
        ("home", &tr.gal_home_t, &tr.gal_home_p),
        ("skin", &tr.gal_skin_t, &tr.gal_skin_p),
        ("profiles", &tr.gal_profiles_t, &tr.gal_profiles_p),
        ("notes", &tr.gal_notes_t, &tr.gal_notes_p),
        ("textures", &tr.gal_textures_t, &tr.gal_textures_p),
        ("friends", &tr.gal_friends_t, &tr.gal_friends_p),
        ("versions", &tr.gal_versions_t, &tr.gal_versions_p),
        ("settings", &tr.gal_settings_t, &tr.gal_settings_p),
        ("horizon", &tr.hz_label, &tr.alt_horizon),
    ];
    let pct = |v: u32, total: u32| format!("{:.3}%", v as f64 * 100.0 / total as f64);
    html! {
        div.col.hero-shot-wrap #preview data-sim {
            figure.hero-shot {
                // les zones se placent sur la capture elle-même, pas sur le cadre (qui a une marge)
                div.sim-screen role="group" aria-label=(tr.prev_hint) {
                    @for (i, (nom, t, d)) in ecrans.iter().enumerate() {
                        div.sim-view data-view=(nom) data-t=(t) data-p=(d) hidden[i > 0] {
                            (capture(p, nom, t, if i == 0 { "eager" } else { "lazy" }))
                        }
                    }
                    // transition : un nuage en pixels traverse l'écran (dessiné par sim.js)
                    canvas.sim-nuage aria-hidden="true" {}
                    @for zone in zones(p.lang.screens_dir()) {
                        @let (x0, y0, x1, y1) = zone.r;
                        @let t = ecrans.iter().find(|e| e.0 == zone.va).map(|e| e.1).unwrap_or_default();
                        button.sim-hot type="button" data-go=(zone.va) data-on=(zone.sur.join(" ")) aria-label=(t) title=(t)
                            hidden[!zone.sur.is_empty() && !zone.sur.contains(&ecrans[0].0)]
                            style=(format!("left:{};top:{};width:{};height:{}", pct(x0, 1180), pct(y0, 720), pct(x1 - x0, 1180), pct(y1 - y0, 720))) {}
                    }
                }
            }
            div.sim-bar {
                div.sim-tabs role="tablist" {
                    @for (i, (nom, t, _)) in ecrans.iter().take(8).enumerate() {
                        button.chip type="button" role="tab" data-go=(nom) aria-selected=(if i == 0 { "true" } else { "false" }) { (t) }
                    }
                }
                p.sim-cap aria-live="polite" { strong { (ecrans[0].1) } " " span { (ecrans[0].2) } }
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
        (&tr.stack_system, vec![("Windows 10", None), ("Windows 11", None), ("Linux x86_64", None), ("Java auto", None)]),
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
    html! {
        section.blk #horizon {
            div.col {
                (titre(&tr.hz_label, None))
                div.cells.hz {
                    figure.shot.reveal { div.frame { (capture(p, "horizon", &tr.alt_horizon, "lazy")) } }
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

fn download(p: &Page, latest: Option<&Release>, linux: bool) -> Markup {
    let tr = p.tr;
    let meta = |kind: Kind| -> Option<String> {
        let r = latest?;
        let a = r.asset(kind)?;
        Some(format!("{} · {}", r.tag_name, format::size(tr, a.size)))
    };
    // la carte Linux : en tête et recommandée pour un visiteur sous Linux, après macOS sinon
    let carte_linux = html! {
        a.dl-card.main[linux] href="/download/linux" {
            span.top { (icons::linux()) "Linux" @if linux { span.pill { (tr.recommended) } } }
            span.txt { (tr.os_linux_p) }
            @if let Some(m) = meta(Kind::Linux) { span.m { (m) " · .tar.gz" } }
        }
    };
    html! {
        section.blk #download {
            div.col {
                (titre(&tr.dl_label, None))
                p.blk-lead { (tr.dl_text) }
                div.cells.c3.dl {
                    @if linux { (carte_linux) }
                    a.dl-card.main[!linux] href="/download/setup" {
                        span.top { (icons::download()) (tr.inst_t) @if !linux { span.pill { (tr.recommended) } } }
                        span.txt { (tr.inst_p) }
                        @if let Some(m) = meta(Kind::Setup) { span.m { (m) " · .exe" } }
                    }
                    a.dl-card href="/download/portable" {
                        span.top { (icons::archive()) (tr.port_t) }
                        span.txt { (tr.port_p) }
                        @if let Some(m) = meta(Kind::Portable) { span.m { (m) " · .zip" } }
                    }
                    @if !linux { (carte_linux) }
                    div.dl-card.soon aria-disabled="true" {
                        span.top { (icons::apple()) "macOS" span.badge { (tr.os_soon) } }
                        span.txt { (tr.os_macos_p) }
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
                        // name : une seule question ouverte à la fois (ouvrir l'une ferme l'autre)
                        details name="faq" { summary { span { (q) } (icons::chevron()) } p { (a) } }
                    }
                }
            }
        }
    }
}
