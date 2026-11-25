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
//! Kit média et charte graphique (`/brand`) : logo, nom, couleurs, polices, captures et textes de
//! présentation, à télécharger ou copier. En français et en anglais (les autres langues voient
//! l'anglais, comme les pages légales).

use super::icons;
use super::layout::{self, BANNER, DISCORD_URL, Head, Page};
use crate::i18n::Lang;
use maud::{Markup, html};

/// Les couleurs de la charte : nom, valeur, rôle (fr, en).
const COULEURS: [(&str, &str, &str, &str); 8] = [
    ("Night", "#09090B", "Fond principal", "Main background"),
    ("Panel", "#111114", "Cartes et panneaux", "Cards and panels"),
    ("Sky", "#0D1834", "Ciel de nuit du paysage", "Night sky of the landscape"),
    ("Mist", "#B8CCF4", "Accent : liens, puces, états", "Accent: links, bullets, states"),
    ("Mist strong", "#8EAAF0", "Accent appuyé, survols", "Strong accent, hovers"),
    ("Snow", "#FAFAFA", "Texte et bouton plein", "Text and solid button"),
    ("Fog", "#A1A1AA", "Texte secondaire", "Secondary text"),
    ("Daylight", "#F6F7FB", "Fond du thème clair", "Light theme background"),
];

/// Captures du launcher montrées (et téléchargeables).
const CAPTURES: [&str; 6] = ["home", "skin", "profiles", "friends", "versions", "settings"];

struct Textes {
    titre: &'static str,
    label: &'static str,
    h1_1: &'static str,
    h1_2: &'static str,
    sub: &'static str,
    cta_logo: &'static str,
    cta_contact: &'static str,
    logo_t: &'static str,
    logo_p: &'static str,
    logo_sombre: &'static str,
    logo_clair: &'static str,
    wordmark: &'static str,
    ascii: &'static str,
    copier: &'static str,
    copie: &'static str,
    nom_t: &'static str,
    nom_p: &'static str,
    oui: [&'static str; 3],
    non: [&'static str; 3],
    couleurs_t: &'static str,
    couleurs_p: &'static str,
    polices_t: &'static str,
    polices_p: &'static str,
    police_logo: &'static str,
    police_texte: &'static str,
    police_code: &'static str,
    captures_t: &'static str,
    captures_p: &'static str,
    regles_t: &'static str,
    a_faire: &'static str,
    a_eviter: &'static str,
    faire: [&'static str; 4],
    eviter: [&'static str; 4],
    textes_t: &'static str,
    textes_p: &'static str,
    court: &'static str,
    court_txt: &'static str,
    long: &'static str,
    long_txt: &'static str,
    contact_t: &'static str,
    contact_p: &'static str,
    marque: &'static str,
}

const FR: Textes = Textes {
    titre: "Kit média et charte graphique – Mistly MC",
    label: "Kit média",
    h1_1: "Parler de Mistly,",
    h1_2: "avec les bons mots.",
    sub: "Logo, couleurs, polices, captures et textes prêts à l'emploi pour les vidéos, articles et serveurs qui présentent Mistly.",
    cta_logo: "Télécharger le logo",
    cta_contact: "Contacter l'équipe",
    logo_t: "Logo",
    logo_p: "Le nuage pixel de Mistly. Gardez-le net (rendu « pixelated »), sans le déformer ni le recolorer.",
    logo_sombre: "Sur fond sombre",
    logo_clair: "Sur fond clair",
    wordmark: "Nom en Silkscreen",
    ascii: "Bannière ASCII",
    copier: "Copier",
    copie: "Copié",
    nom_t: "Le nom",
    nom_p: "On écrit Mistly, avec une majuscule. Le launcher s'appelle Mistly Launcher, le mod en jeu Mistly Client.",
    oui: ["Mistly", "Mistly Launcher", "Mistly Client"],
    non: ["Misty", "MistlyMC", "mistly launcher"],
    couleurs_t: "Couleurs",
    couleurs_p: "Une nuit calme et une brume bleutée. Cliquez sur une couleur pour copier sa valeur.",
    polices_t: "Polices",
    polices_p: "Trois polices libres (SIL Open Font License), toutes incluses dans le site.",
    police_logo: "Le nom MISTLY et les titres pixel",
    police_texte: "Les titres et le texte",
    police_code: "Le code, les chiffres et les terminaux",
    captures_t: "Captures",
    captures_p: "Le launcher tel qu'il est aujourd'hui (1180 × 720, thème nuit). Cliquez pour télécharger en taille réelle.",
    regles_t: "Utilisation",
    a_faire: "À faire",
    a_eviter: "À éviter",
    faire: [
        "Laisser de l'espace autour du logo (au moins la moitié de sa largeur)",
        "Mettre un lien vers mistly.cc quand c'est possible",
        "Utiliser les captures telles quelles, ou recadrées",
        "Préciser que Mistly n'est pas affilié à Mojang ni à Microsoft",
    ],
    eviter: [
        "Étirer, pencher, flouter ou recolorer le logo",
        "Ajouter des effets (ombres, contours, dégradés) au logo",
        "Présenter Mistly comme un produit officiel de Minecraft",
        "Vendre quoi que ce soit avec le nom ou le logo de Mistly",
    ],
    textes_t: "Textes de présentation",
    textes_p: "À copier tels quels dans une description de vidéo, un article ou un salon Discord.",
    court: "Version courte",
    court_txt: "Mistly est un launcher Minecraft gratuit et léger, écrit en Rust, pour Windows et Linux.",
    long: "Version longue",
    long_txt: "Mistly est un launcher Minecraft: Java Edition gratuit et open source, écrit en Rust. Il installe toutes les versions et tous les mod loaders (Forge, NeoForge, Fabric, Quilt, OptiFine), les mods et modpacks de Modrinth et CurseForge, et gère skins, profils et amis. Avec Mistly Client, les joueurs portent des capes, élytres et chapeaux visibles par tous. Téléchargement sur https://mistly.cc.",
    contact_t: "Une question ?",
    contact_p: "Pour une vidéo, un partenariat ou un usage qui n'est pas prévu ici, écrivez-nous sur Discord.",
    marque: "Minecraft est une marque de Mojang AB. Mistly n'est ni affilié à Mojang Studios ni à Microsoft, ni approuvé par eux.",
};

const EN: Textes = Textes {
    titre: "Media kit and brand guidelines – Mistly MC",
    label: "Media kit",
    h1_1: "Talk about Mistly,",
    h1_2: "with the right words.",
    sub: "Logo, colors, fonts, screenshots and ready-made texts for the videos, articles and servers that feature Mistly.",
    cta_logo: "Download the logo",
    cta_contact: "Contact the team",
    logo_t: "Logo",
    logo_p: "Mistly's pixel cloud. Keep it sharp (pixelated rendering), never stretched or recolored.",
    logo_sombre: "On dark backgrounds",
    logo_clair: "On light backgrounds",
    wordmark: "Name in Silkscreen",
    ascii: "ASCII banner",
    copier: "Copy",
    copie: "Copied",
    nom_t: "The name",
    nom_p: "It's written Mistly, with a capital M. The launcher is Mistly Launcher, the in-game mod is Mistly Client.",
    oui: ["Mistly", "Mistly Launcher", "Mistly Client"],
    non: ["Misty", "MistlyMC", "mistly launcher"],
    couleurs_t: "Colors",
    couleurs_p: "A calm night and a bluish mist. Click a color to copy its value.",
    polices_t: "Fonts",
    polices_p: "Three free fonts (SIL Open Font License), all bundled with the site.",
    police_logo: "The MISTLY name and pixel headings",
    police_texte: "Headings and body text",
    police_code: "Code, numbers and terminals",
    captures_t: "Screenshots",
    captures_p: "The launcher as it is today (1180 × 720, night theme). Click to download at full size.",
    regles_t: "Usage",
    a_faire: "Do",
    a_eviter: "Don't",
    faire: [
        "Leave room around the logo (at least half its width)",
        "Link to mistly.cc whenever you can",
        "Use the screenshots as they are, or cropped",
        "Mention that Mistly is not affiliated with Mojang or Microsoft",
    ],
    eviter: [
        "Stretch, tilt, blur or recolor the logo",
        "Add effects (shadows, outlines, gradients) to the logo",
        "Present Mistly as an official Minecraft product",
        "Sell anything with the Mistly name or logo",
    ],
    textes_t: "Boilerplate",
    textes_p: "Copy them as they are into a video description, an article or a Discord channel.",
    court: "Short version",
    court_txt: "Mistly is a free, lightweight Minecraft launcher written in Rust, for Windows and Linux.",
    long: "Long version",
    long_txt: "Mistly is a free and open-source Minecraft: Java Edition launcher written in Rust. It installs every version and every mod loader (Forge, NeoForge, Fabric, Quilt, OptiFine), mods and modpacks from Modrinth and CurseForge, and manages skins, profiles and friends. With Mistly Client, players wear capes, elytra and hats that everyone can see. Download it at https://mistly.cc.",
    contact_t: "A question?",
    contact_p: "For a video, a partnership or a use not covered here, reach us on Discord.",
    marque: "Minecraft is a trademark of Mojang AB. Mistly is not affiliated with, nor endorsed by, Mojang Studios or Microsoft.",
};

/// Bouton qui copie `texte` (brand.js).
fn copier(t: &Textes, texte: &str) -> Markup {
    html! {
        button.bk-copy type="button" data-copy=(texte) data-done=(t.copie) { (icons::copy()) span { (t.copier) } }
    }
}

pub fn render(p: &Page) -> Markup {
    let t = if p.lang == Lang::Fr { &FR } else { &EN };
    let ecrans = p.lang.screens_dir();
    let section = |id: &str, n: &str, titre: &str, lead: &str, corps: Markup| {
        html! {
            section.blk.bk-sec id=(id) {
                div.col {
                    h2.blk-title { (titre) sup { "(" (n) ")" } }
                    p.blk-lead { (lead) }
                    (corps)
                }
            }
        }
    };
    let main = html! {
        section.hero.bk-hero {
            div.col.bk-head {
                span.label { (t.label) }
                h1 { (t.h1_1) " " span.soft { (t.h1_2) } }
                p.sub { (t.sub) }
                div.bk-ctas {
                    a.btn.play href="/assets/icone.png" download="mistly-logo.png" { (icons::download()) span { (t.cta_logo) } }
                    a.btn href=(DISCORD_URL) target="_blank" rel="noopener" { (icons::discord()) span { (t.cta_contact) } }
                }
                nav.bk-toc aria-label=(t.label) {
                    a href="#logo" { "01 " (t.logo_t) }
                    a href="#name" { "02 " (t.nom_t) }
                    a href="#colors" { "03 " (t.couleurs_t) }
                    a href="#fonts" { "04 " (t.polices_t) }
                    a href="#screens" { "05 " (t.captures_t) }
                    a href="#usage" { "06 " (t.regles_t) }
                    a href="#boilerplate" { "07 " (t.textes_t) }
                }
            }
        }

        (section("logo", "01", t.logo_t, t.logo_p, html! {
            div.cells.c2.bk-logos {
                figure.bk-logo.dark {
                    img src="/assets/icone.png" alt="Mistly" width="120" height="120";
                    figcaption {
                        span { (t.logo_sombre) }
                        span.dl {
                            a href="/assets/icone.png" download="mistly-logo.png" { "PNG" }
                            a href="/assets/icone.ico" download="mistly-logo.ico" { "ICO" }
                        }
                    }
                }
                figure.bk-logo.light {
                    img src="/assets/icone.png" alt="Mistly" width="120" height="120";
                    figcaption {
                        span { (t.logo_clair) }
                        span.dl { a href="/assets/icone.png" download="mistly-logo.png" { "PNG" } }
                    }
                }
                figure.bk-logo.word {
                    div.bk-wordmark { img src="/assets/icone.png" alt="" width="44" height="44"; span { "MISTLY" } }
                    figcaption { span { (t.wordmark) } }
                }
                figure.bk-logo.ascii {
                    pre.bk-banner aria-label="MISTLY" { (BANNER) }
                    figcaption { span { (t.ascii) } (copier(t, BANNER)) }
                }
            }
        }))

        (section("name", "02", t.nom_t, t.nom_p, html! {
            div.cells.c2.bk-names {
                div.cell.ok {
                    span.bk-tag { (icons::check()) (t.a_faire) }
                    @for n in t.oui { strong { (n) } }
                }
                div.cell.no {
                    span.bk-tag { "✕ " (t.a_eviter) }
                    @for n in t.non { s { (n) } }
                }
            }
        }))

        (section("colors", "03", t.couleurs_t, t.couleurs_p, html! {
            div.cells.c4.bk-colors {
                @for (nom, hex, fr, en) in COULEURS {
                    button.bk-swatch type="button" data-copy=(hex) data-done=(t.copie) style=(format!("--c:{hex}")) {
                        span.chip aria-hidden="true" {}
                        strong { (nom) }
                        code { (hex) }
                        small { (if p.lang == Lang::Fr { fr } else { en }) }
                    }
                }
            }
        }))

        (section("fonts", "04", t.polices_t, t.polices_p, html! {
            div.cells.c3.bk-fonts {
                div.cell.f-pixel {
                    span.sample { "Aa" }
                    strong { "Silkscreen" }
                    small { (t.police_logo) }
                    span.abc { "MISTLY 0123" }
                }
                div.cell.f-sans {
                    span.sample { "Aa" }
                    strong { "Geist" }
                    small { (t.police_texte) }
                    span.abc { "Léger comme la brume" }
                }
                div.cell.f-mono {
                    span.sample { "Aa" }
                    strong { "Geist Mono" }
                    small { (t.police_code) }
                    span.abc { "$ mistly --play" }
                }
            }
        }))

        (section("screens", "05", t.captures_t, t.captures_p, html! {
            div.bk-shots {
                @for nom in CAPTURES {
                    @let src = format!("/assets/screens/{ecrans}/{nom}.png");
                    a.bk-shot href=(src) download=(format!("mistly-{nom}.png")) {
                        img src=(src) alt=(format!("Mistly Launcher – {nom}")) width="1180" height="720" loading="lazy";
                        span { (nom) (icons::download()) }
                    }
                }
            }
        }))

        (section("usage", "06", t.regles_t, "", html! {
            div.cells.c2.bk-rules {
                div.cell.ok {
                    span.bk-tag { (icons::check()) (t.a_faire) }
                    ul { @for r in t.faire { li { (r) } } }
                }
                div.cell.no {
                    span.bk-tag { "✕ " (t.a_eviter) }
                    ul { @for r in t.eviter { li { (r) } } }
                }
            }
        }))

        (section("boilerplate", "07", t.textes_t, t.textes_p, html! {
            div.cells.c2.bk-texts {
                div.cell {
                    div.bk-text-head { strong { (t.court) } (copier(t, t.court_txt)) }
                    p { (t.court_txt) }
                }
                div.cell {
                    div.bk-text-head { strong { (t.long) } (copier(t, t.long_txt)) }
                    p { (t.long_txt) }
                }
            }
        }))

        section.blk.bk-sec {
            div.col {
                div.bk-contact {
                    div {
                        h2 { (t.contact_t) }
                        p { (t.contact_p) }
                    }
                    a.btn.play href=(DISCORD_URL) target="_blank" rel="noopener" { (icons::discord()) span { "Discord" } }
                }
                p.bk-legal { (t.marque) }
            }
        }
    };
    layout::render(
        p,
        Head {
            title: t.titre,
            description: t.sub,
            jsonld: None,
            css: &["home", "brand"],
            scripts: &["/js/brand.js"],
            body_class: "brand-page",
        },
        main,
    )
}
