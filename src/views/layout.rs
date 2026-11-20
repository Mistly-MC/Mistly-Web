//! Squelette commun : <head>, barre de navigation et pied de page.

use super::icons;
use crate::i18n::{Lang, Tr};
use maud::{DOCTYPE, Markup, PreEscaped, html};
use std::sync::LazyLock;

/// « MISTLY » en lettres de blocs ASCII, en bas de chaque page (site.js y fait passer une vague de
/// lumière). Affiché en JetBrains Mono, qui a tous ces caractères (█, ╗, ═…) avec la même largeur :
/// sans elle, le navigateur en prend une autre et les lettres se décalent.
pub const BANNER: &str = "\
███╗   ███╗ ██╗ ███████╗ ████████╗ ██╗      ██╗   ██╗
████╗ ████║ ██║ ██╔════╝ ╚══██╔══╝ ██║      ╚██╗ ██╔╝
██╔████╔██║ ██║ ███████╗    ██║    ██║       ╚████╔╝
██║╚██╔╝██║ ██║ ╚════██║    ██║    ██║        ╚██╔╝
██║ ╚═╝ ██║ ██║ ███████║    ██║    ███████╗    ██║
╚═╝     ╚═╝ ╚═╝ ╚══════╝    ╚═╝    ╚══════╝    ╚═╝";

/// Codes de vérification des outils pour webmasters : Google, Bing, Yandex.
static VERIFS: LazyLock<(Option<String>, Option<String>, Option<String>)> = LazyLock::new(|| {
    let lire = |k: &str| std::env::var(k).ok().map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
    (lire("GOOGLE_SITE_VERIFICATION"), lire("BING_SITE_VERIFICATION"), lire("YANDEX_VERIFICATION"))
});

/// Serveur Discord de Mistly.
pub const DISCORD_URL: &str = "https://discord.gg/rdQJUgvUh3";

/// Ajouté aux adresses des CSS et du JS (`?v=`) : change à chaque démarrage du serveur,
/// donc à chaque déploiement, pour que les navigateurs ne gardent pas d'anciennes versions en cache.
pub static ASSET_VERSION: LazyLock<String> = LazyLock::new(|| {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default()
});

/// Ce dont toutes les pages ont besoin.
pub struct Page<'a> {
    pub lang: Lang,
    pub tr: &'static Tr,
    /// Chemin courant (sans paramètres), pour les liens de changement de langue.
    pub path: &'a str,
    /// Nombre de versions publiées, affiché à côté de « Patch notes ».
    pub release_count: Option<usize>,
    /// `owner/repo` du launcher sur GitHub.
    pub repo: &'a str,
    /// Paysage de fond (SVG en pixel art, le même que dans le launcher).
    pub scene: &'a str,
    /// Adresse publique du site, sans `/` final (balises canoniques, Open Graph, sitemap).
    pub site_url: &'a str,
}

impl Page<'_> {
    pub fn github_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }
    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }
}

pub struct Head<'a> {
    pub title: &'a str,
    /// Description de la page pour les moteurs de recherche et les aperçus de liens.
    pub description: &'a str,
    /// Données structurées JSON-LD (déjà sérialisées), pour les moteurs de recherche.
    pub jsonld: Option<String>,
    /// Feuilles de style propres à la page, en plus de base.css et layout.css.
    pub css: &'a [&'a str],
    /// Scripts propres à la page, chargés avant site.js.
    pub scripts: &'a [&'a str],
    pub body_class: &'a str,
}

pub fn render(p: &Page, head: Head, main: Markup) -> Markup {
    let tr = p.tr;
    html! {
        (DOCTYPE)
        html lang=(p.lang.code()) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (head.title) }
                meta name="description" content=(head.description);
                meta name="keywords" content=(crate::seo::KEYWORDS);
                meta name="author" content="Mistly";
                meta name="theme-color" content="#09090b" media="(prefers-color-scheme: dark)";
                meta name="theme-color" content="#f6f7fb" media="(prefers-color-scheme: light)";
                meta name="color-scheme" content="dark light";
                meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1";
                meta name="application-name" content="Mistly Launcher";
                meta name="apple-mobile-web-app-title" content="Mistly";
                // Codes de vérification des outils pour webmasters (Google, Bing, Yandex), s'ils sont réglés
                @if let Some(c) = &VERIFS.0 { meta name="google-site-verification" content=(c); }
                @if let Some(c) = &VERIFS.1 { meta name="msvalidate.01" content=(c); }
                @if let Some(c) = &VERIFS.2 { meta name="yandex-verification" content=(c); }
                link rel="manifest" href=(format!("/site.webmanifest?lang={}", p.lang.code()));
                // Une URL par langue (?lang=xx) : la canonique pointe sur la langue affichée,
                // et les variantes hreflang annoncent les autres à Google.
                @let base = format!("{}{}", p.site_url, p.path);
                @let canonical = format!("{base}?lang={}", p.lang.code());
                link rel="canonical" href=(canonical);
                @for l in Lang::ALL {
                    link rel="alternate" hreflang=(l.code()) href=(format!("{base}?lang={}", l.code()));
                }
                link rel="alternate" hreflang="x-default" href=(base);
                // Aperçus de liens (Discord, réseaux sociaux…)
                @let image = format!("{}/assets/og/{}.png?v={}", p.site_url, p.lang.code(), ASSET_VERSION.as_str());
                meta property="og:type" content="website";
                meta property="og:site_name" content="Mistly Launcher";
                meta property="og:title" content=(head.title);
                meta property="og:description" content=(head.description);
                meta property="og:url" content=(canonical);
                meta property="og:image" content=(image);
                meta property="og:image:width" content="1200";
                meta property="og:image:height" content="630";
                meta property="og:image:type" content="image/png";
                meta property="og:image:alt" content=(format!("Mistly Launcher – {}{}", tr.hero_title_1, tr.hero_title_2));
                meta property="og:locale" content=(tr.og_locale);
                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:title" content=(head.title);
                meta name="twitter:description" content=(head.description);
                meta name="twitter:image" content=(image);
                link rel="icon" href="/assets/icone.png";
                link rel="apple-touch-icon" href="/assets/icone.png";
                @if let Some(json) = &head.jsonld {
                    script type="application/ld+json" { (PreEscaped(json)) }
                }
                link rel="preload" href="/assets/Geist-latin.woff2" as="font" type="font/woff2" crossorigin;
                @let v = ASSET_VERSION.as_str();
                link rel="stylesheet" href=(format!("/css/base.css?v={v}"));
                link rel="stylesheet" href=(format!("/css/layout.css?v={v}"));
                @for css in head.css { link rel="stylesheet" href=(format!("/css/{css}.css?v={v}")); }
                // Le thème choisi (clair ou sombre) est posé avant l'affichage, sans clignotement
                script { (PreEscaped("document.documentElement.classList.add('js');try{var t=localStorage.getItem('theme');if(t==='light'||t==='dark')document.documentElement.dataset.theme=t}catch(e){}")) }
                @for js in head.scripts { script src=(format!("{js}?v={v}")) defer {} }
                script src=(format!("/js/site.js?v={v}")) defer {}
            }
            body class=(head.body_class) {
                // Le paysage du launcher (pixel art), et par-dessus une brume en caractères ASCII qui
                // dérive (dessinée par site.js).
                div.page-sky aria-hidden="true" { (PreEscaped(p.scene)) canvas.ascii-bg {} }
                // Les deux traits verticaux qui bordent la colonne du site
                div.rails aria-hidden="true" {}
                (telechargement(p))
                (nav(p))
                main #main { (main) }
                (footer(p))
            }
        }
    }
}

/// L'écran plein qui s'ouvre quand on télécharge le launcher : un nuage qui se forme en caractères
/// ASCII, une barre de progression, puis les trois étapes pour installer (site.js l'anime).
fn telechargement(p: &Page) -> Markup {
    let tr = p.tr;
    html! {
        div.dlx #dlx role="dialog" aria-modal="true" aria-labelledby="dlx-title" hidden {
            canvas.dlx-art aria-hidden="true" {}
            div.dlx-panel {
                h2 #dlx-title { (tr.dlx_title) }
                p.dlx-sub { (tr.dlx_sub) }
                div.dlx-bar aria-hidden="true" {
                    span.dlx-label data-preparing=(tr.dlx_preparing) data-started=(tr.dlx_started) { (tr.dlx_preparing) }
                    code.dlx-track {}
                    span.dlx-pct { "0%" }
                }
                ol.dlx-steps {
                    li { span.n { "01" } (tr.dlx_s1) }
                    li { span.n { "02" } span.s2 data-setup=(tr.dlx_s2_setup) data-portable=(tr.dlx_s2_port) data-linux=(tr.dlx_s2_linux) { (tr.dlx_s2_setup) } }
                    li { span.n { "03" } (tr.dlx_s3) }
                }
                div.dlx-actions {
                    span.dlx-retry { (tr.dlx_retry) " " a #dlx-again href="/download/setup" { (tr.dlx_retry_link) } }
                    a.dlx-guide href="/docs/installation" { (tr.dlx_guide) (icons::arrow_right()) }
                }
                button.btn.dlx-close type="button" data-dlx-close { (tr.dlx_close) " " kbd { "Esc" } }
            }
        }
    }
}

/// Le menu de la documentation, ouvert au survol du lien « Docs » (ou au clavier) : deux pages à
/// la une en cartes, les pages les plus lues, puis les liens vers le reste du site.
fn docs_menu(p: &Page) -> Markup {
    let tr = p.tr;
    let docs = crate::docs::get(p.lang);
    let titre = |slug: &str| docs.find(slug).map_or(slug.to_string(), |i| docs.pages[i].title.clone());
    let pages: [(&str, Markup); 7] = [
        ("installation", icons::download()),
        ("first-launch", icons::bolt()),
        ("profiles", icons::layers()),
        ("mods-and-content", icons::package()),
        ("modpacks", icons::archive()),
        ("skins", icons::shirt()),
        ("troubleshooting", icons::alert()),
    ];
    html! {
        div.navdd {
            a.navdd-btn href="/docs" aria-current=[p.path.starts_with("/docs").then_some("page")] aria-haspopup="true" {
                (tr.nav_docs) (icons::chevron())
            }
            div.mega {
                div.mega-feat {
                    a.feat.f-client href="/docs/mistly-client" {
                        span.k { (tr.mega_new) }
                        strong { "Mistly Client" }
                        span.d { (tr.mega_client_p) }
                        i aria-hidden="true" {}
                    }
                    a.feat.f-horizon href="/docs/horizon" {
                        span.k { (tr.mega_new) }
                        strong { "Horizon" }
                        span.d { (tr.mega_horizon_p) }
                        i aria-hidden="true" {}
                    }
                }
                div.mega-col {
                    span.mega-t { (tr.docs_label) }
                    @for (slug, ic) in pages {
                        a href=(format!("/docs/{slug}")) { (ic) span { (titre(slug)) } }
                    }
                    a.mega-all href="/docs" { (tr.mega_all) (icons::arrow_right()) }
                }
                div.mega-col {
                    span.mega-t { "Mistly" }
                    a href="/patch-notes" { (icons::notes()) span { (tr.nav_patch_notes) } }
                    a href="/status" { (icons::globe()) span { (tr.nav_status) } }
                    a href="/shop" { (icons::bag()) span { (tr.nav_shop) } }
                    a href=(DISCORD_URL) target="_blank" rel="noopener" { (icons::discord()) span { "Discord" } }
                    a href=(p.github_url()) target="_blank" rel="noopener" { (icons::github()) span { "GitHub" } }
                }
            }
        }
    }
}

fn nav(p: &Page) -> Markup {
    let tr = p.tr;
    let on_notes = p.path.starts_with("/patch-notes");
    html! {
        header.nav #top {
            div.col.nav-in {
                a.brand href="/" { img src="/assets/icone.png" alt="" width="30" height="30"; span { "Mistly" } }
                nav.links aria-label="Navigation" {
                    a href="/#features" { (tr.nav_features) }
                    a href="/#loaders" { (tr.nav_loaders) }
                    a href="/#faq" { (tr.nav_faq) }
                    (docs_menu(p))
                    a href="/shop" aria-current=[p.path.starts_with("/shop").then_some("page")] { (tr.nav_shop) }
                    a href="/patch-notes" aria-current=[on_notes.then_some("page")] {
                        (tr.nav_patch_notes)
                        @if let Some(n) = p.release_count { span.count { (n) } }
                    }
                }
                div.nav-right {
                    a.gh.dc href=(DISCORD_URL) target="_blank" rel="noopener" aria-label="Discord" title="Discord" { (icons::discord()) }
                    a.gh href=(p.github_url()) target="_blank" rel="noopener" aria-label="GitHub" { (icons::github()) span { "GitHub" } }
                    button.theme-toggle type="button" data-theme-toggle aria-label=(tr.nav_theme) title=(tr.nav_theme) {
                        span.moon { (icons::moon()) } span.sun { (icons::sun()) }
                    }
                    (lang_menu(p))
                    a.btn.play.sm href="/#download" { (icons::download()) span { (tr.nav_download) } }
                    (mobile_menu(p))
                }
            }
        }
    }
}

/// Menu de langue sans JavaScript : un <details> dont les liens rechargent la page avec `?lang=`.
/// Menu des pages pour les petits écrans (la barre de liens y est masquée).
fn mobile_menu(p: &Page) -> Markup {
    let tr = p.tr;
    let here = |path: &str| {
        (p.path == path || (path != "/" && p.path.starts_with(path))).then_some("page")
    };
    html! {
        details.burger {
            summary aria-label="Menu" { (icons::menu()) }
            nav.menu aria-label="Navigation" {
                a href="/#features" { (tr.nav_features) }
                a href="/#loaders" { (tr.nav_loaders) }
                a href="/#faq" { (tr.nav_faq) }
                a href="/docs" aria-current=[here("/docs")] { (tr.nav_docs) }
                a href="/shop" aria-current=[here("/shop")] { (tr.nav_shop) }
                a href="/patch-notes" aria-current=[here("/patch-notes")] {
                    (tr.nav_patch_notes)
                    @if let Some(n) = p.release_count { span.count { (n) } }
                }
                a href="/status" aria-current=[here("/status")] { (tr.nav_status) }
                a href=(DISCORD_URL) target="_blank" rel="noopener" { "Discord" (icons::external()) }
                a href=(p.github_url()) target="_blank" rel="noopener" { "GitHub" (icons::external()) }
            }
        }
    }
}

fn lang_menu(p: &Page) -> Markup {
    html! {
        details.langdd {
            summary aria-label=(format!("{} : {}", p.tr.nav_language, p.tr.lang_name)) title=(p.tr.lang_name) { (icons::flag(p.lang.code())) }
            div.menu {
                @for l in Lang::ALL {
                    a href=(format!("{}?lang={}", p.path, l.code())) hreflang=(l.code()) aria-current=[(l == p.lang).then_some("true")] {
                        (icons::flag(l.code())) span { (l.tr().lang_name) }
                    }
                }
            }
        }
    }
}

/// Les auteurs du site, avec la tête de leur skin Minecraft (découpée par le CSS dans le skin
/// servi par `/skin/<pseudo>`, donc toujours à jour).
const AUTEURS: [&str; 2] = ["EnderMythex", "xtheox_x"];

fn credits(p: &Page) -> Markup {
    let auteur = |nom: &str| {
        html! {
            span.author {
                span.mc-head style=(format!("--skin:url(/skin/{nom})")) aria-hidden="true" {}
                span { (nom) }
            }
        }
    };
    // « Made in France by {0} and {1} » : l'ordre des mots dépend de la langue
    let texte = &p.tr.ft_credit;
    let (avant, reste) = texte.split_once("{0}").unwrap_or((texte, ""));
    let (milieu, apres) = reste.split_once("{1}").unwrap_or((reste, ""));
    html! {
        div.fcredit {
            (icons::flag("fr"))
            span { (avant) } (auteur(AUTEURS[0])) span { (milieu) } (auteur(AUTEURS[1])) span { (apres) }
        }
    }
}

fn footer(p: &Page) -> Markup {
    let tr = p.tr;
    html! {
        footer.foot {
            div.col {
                div.ftop {
                    div {
                        a.brand href="/" { img src="/assets/icone.png" alt="" width="30" height="30"; span { "Mistly" } }
                        p.tag { (tr.ft_tag) }
                    }
                    p.disclaimer { (tr.ft_disclaimer) }
                }
                div.fcols {
                    div {
                        h2 { (tr.ft_nav) }
                        ul {
                            li { a href="/#features" { (tr.nav_features) } }
                            li { a href="/#loaders" { (tr.nav_loaders) } }
                            li { a href="/#faq" { (tr.nav_faq) } }
                            li { a href="/docs" { (tr.docs_label) } }
                            li { a href="/shop" { (tr.nav_shop) } }
                            li { a href="/patch-notes" { (tr.nav_patch_notes) } }
                            li { a href="/status" { (tr.nav_status) } }
                        }
                    }
                    div {
                        h2 { (tr.nav_download) }
                        ul {
                            li { a href="/download/setup" { (tr.inst_t) } }
                            li { a href="/download/portable" { (tr.port_t) } }
                            li { a href="/patch-notes?project=client" { "Mistly Client" } }
                            li { a href=(p.releases_url()) target="_blank" rel="noopener" { (tr.github_t) (icons::external()) } }
                            li { a href=(p.github_url()) target="_blank" rel="noopener" { "GitHub" (icons::external()) } }
                        }
                    }
                    div {
                        h2 { (tr.nav_language) }
                        div.flangs {
                            @for l in Lang::ALL {
                                a href=(format!("{}?lang={}", p.path, l.code())) hreflang=(l.code()) aria-current=[(l == p.lang).then_some("true")] aria-label=(l.tr().lang_name) title=(l.tr().lang_name) { (icons::flag(l.code())) }
                            }
                        }
                    }
                    div {
                        h2 { (tr.ft_legal_t) }
                        ul {
                            @for k in crate::legal::Kind::ALL {
                                li { a href=(k.path()) { (k.label(tr)) } }
                            }
                        }
                    }
                    div.about {
                        h2 { (tr.ft_about_t) }
                        p { (tr.ft_about_p) }
                        a.discord-cta href=(DISCORD_URL) target="_blank" rel="noopener" {
                            (icons::discord())
                            span { strong { (tr.discord_join) } small { (tr.discord_sub) } }
                            (icons::external())
                        }
                    }
                }
                (credits(p))
                div.fbottom {
                    span.made { (icons::rust()) (tr.ft_made) }
                    span.fb-right {
                        a href=(DISCORD_URL) target="_blank" rel="noopener" { (icons::discord()) "Discord" }
                        a href=(p.github_url()) target="_blank" rel="noopener" { (icons::github()) (p.repo) }
                        a.totop href="#top" { (tr.ft_top) (icons::arrow_up()) }
                    }
                }
            }
            pre.ascii-name aria-hidden="true" { (BANNER) }
        }
    }
}
