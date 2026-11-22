//! Pages du tableau de bord de l'équipe (`/admin`), en français seulement.

use super::icons;
use super::layout::ASSET_VERSION;
use crate::client::avec_tirets;
use crate::client::EnLigne;
use crate::registre::{Cosmetique, Entree, Fiche as FicheBase, Place, Regle};
use maud::{DOCTYPE, Markup, PreEscaped, html};
use serde_json::Value;

pub struct Flash<'a> {
    pub ok: Option<&'a str>,
    pub err: Option<&'a str>,
}

pub struct Chiffres {
    pub launchers: usize,
    pub en_partie: usize,
    pub record: usize,
    pub client: usize,
    pub joueurs: usize,
    pub nouveaux_24h: usize,
    pub nouveaux_7j: usize,
    pub actifs_24h: usize,
    pub bannis: usize,
    pub version: Option<String>,
    pub dispo: Option<f32>,
}

/// Un joueur en ligne avec Mistly Client.
pub struct Ligne {
    pub uuid: String,
    pub nom: Option<String>,
    pub cosmetique: &'static str,
    pub chapeau: &'static str,
    pub depuis: u64,
    pub note: bool,
}

/// Une ligne de la liste des joueurs.
pub struct Fiche {
    pub uuid: String,
    pub fiche: FicheBase,
    pub en_ligne: bool,
}

pub struct EtatCosmetique {
    pub c: &'static Cosmetique,
    pub regle: Regle,
    pub donne: bool,
    pub retire: bool,
    pub permis: bool,
    pub porte: bool,
}

pub struct RegleCosmetique {
    pub c: &'static Cosmetique,
    pub regle: Regle,
    pub donnes: usize,
    pub retires: usize,
    pub portes: usize,
}

pub struct Early {
    pub date: Option<String>,
    pub depuis_la_base: bool,
    pub eligibles: Option<usize>,
}

#[derive(Clone, Copy, PartialEq)]
enum Onglet {
    Accueil,
    Joueurs,
    Cosmetiques,
    Boutique,
    Docs,
    Todo,
    Rapports,
    Journal,
    Base,
}

// ---------- Mise en forme ----------

const MOIS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

/// « 16 nov. 2026, 14:03 » (UTC).
fn date(secs: u64) -> String {
    let Ok(d) = time::OffsetDateTime::from_unix_timestamp(secs as i64) else { return "?".into() };
    format!("{} {} {}, {:02}:{:02}", MOIS[d.month() as usize - 1], d.day(), d.year(), d.hour(), d.minute())
}

/// « il y a 3 h », « il y a 2 j ».
fn il_y_a(secs: u64) -> String {
    let ecart = crate::registre::maintenant().saturating_sub(secs);
    match ecart {
        0..60 => "just now".into(),
        60..3600 => format!("{} min ago", ecart / 60),
        3600..86_400 => format!("{} h ago", ecart / 3600),
        86_400..2_592_000 => format!("{} d ago", ecart / 86_400),
        _ => format!("{} mo ago", ecart / 2_592_000),
    }
}

/// « 1 h 05 », « 12 min ».
fn duree(secs: u64) -> String {
    if secs < 60 {
        "< 1 min".into()
    } else if secs < 3600 {
        format!("{} min", secs / 60)
    } else {
        format!("{} h {:02}", secs / 3600, secs % 3600 / 60)
    }
}

fn nom_cosmetique(id: &str) -> &str {
    crate::registre::cosmetique(id).map_or(id, |c| c.nom)
}

/// L'adresse du site principal (le tableau de bord est sur son propre sous-domaine, où `/` mène au
/// tableau de bord).
fn site(chemin: &str) -> String {
    let base = std::env::var("SITE_URL").ok().filter(|v| !v.is_empty()).unwrap_or_else(|| "https://mistly.cc".into());
    format!("{}{chemin}", base.trim_end_matches('/'))
}

/// Tête du joueur, dessinée par admin.js depuis son skin (`/skin/<pseudo>`).
fn tete(nom: Option<&str>, taille: u32) -> Markup {
    html! {
        canvas.head width="8" height="8" data-name=[nom] style=(format!("width:{taille}px;height:{taille}px")) aria-hidden="true" {}
    }
}

fn nom_ou_uuid(nom: Option<&str>, uuid: &str) -> String {
    nom.map_or_else(|| avec_tirets(uuid)[..8].to_string() + "…", str::to_string)
}

// ---------- Squelette ----------

impl Onglet {
    /// Le titre de la page, son icône et son groupe dans le menu.
    fn titre(self) -> &'static str {
        match self {
            Onglet::Accueil => "Overview",
            Onglet::Joueurs => "Players",
            Onglet::Cosmetiques => "Cosmetics",
            Onglet::Boutique => "Shop",
            Onglet::Docs => "Docs",
            Onglet::Todo => "To-do",
            Onglet::Rapports => "Reports",
            Onglet::Journal => "Log",
            Onglet::Base => "Database",
        }
    }
    fn icone(self) -> Markup {
        match self {
            Onglet::Accueil => icons::grid(),
            Onglet::Joueurs => icons::user(),
            Onglet::Cosmetiques => icons::sparkles(),
            Onglet::Boutique => icons::bag(),
            Onglet::Docs => icons::bolt(),
            Onglet::Todo => icons::check(),
            Onglet::Rapports => icons::alert(),
            Onglet::Journal => icons::notes(),
            Onglet::Base => icons::archive(),
        }
    }
    fn lien(self) -> &'static str {
        match self {
            Onglet::Accueil => "/admin",
            Onglet::Joueurs => "/admin/players",
            Onglet::Cosmetiques => "/admin/cosmetics",
            Onglet::Boutique => "/admin/shop",
            Onglet::Docs => "/admin/docs",
            Onglet::Todo => "/admin/todo",
            Onglet::Rapports => "/admin/reports",
            Onglet::Journal => "/admin/log",
            Onglet::Base => "/admin/database",
        }
    }
}

fn shell(titre: &str, onglet: Option<Onglet>, flash: Option<&Flash>, corps: Markup) -> Markup {
    let v = ASSET_VERSION.as_str();
    let onglets = [Onglet::Accueil, Onglet::Todo, Onglet::Joueurs, Onglet::Cosmetiques, Onglet::Boutique, Onglet::Rapports, Onglet::Journal, Onglet::Base, Onglet::Docs];
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="robots" content="noindex, nofollow";
                meta name="color-scheme" content="dark";
                title { (titre) " · Mistly Admin" }
                link rel="icon" href="/assets/icone.png";
                link rel="preload" href="/assets/Geist-latin.woff2" as="font" type="font/woff2" crossorigin;
                link rel="stylesheet" href=(format!("/css/base.css?v={v}"));
                link rel="stylesheet" href=(format!("/css/admin.css?v={v}"));
                script src=(format!("/js/admin.js?v={v}")) defer {}
            }
            body.admin {
                div.frame {
                    // La barre latérale : des icônes, leur nom en bulle au survol
                    aside.rail {
                        a.rail-logo href="/admin" title="Mistly Admin" {
                            img src="/assets/icone.png" alt="" width="34" height="34";
                            span { "MISTLY" }
                        }
                        nav.rail-nav aria-label="Dashboard" {
                            // le rond rose : un seul, qui glisse d'une icône à l'autre (admin.js)
                            span.rail-pill aria-hidden="true" {}
                            @for o in onglets {
                                a href=(o.lien()) aria-current=[(onglet == Some(o)).then_some("page")] aria-label=(o.titre()) {
                                    (o.icone())
                                    span.tip { (o.titre()) }
                                }
                            }
                        }
                        div.rail-foot {
                            a href=(site("/")) target="_blank" rel="noopener" aria-label="View site" { (icons::globe()) span.tip { "View site" } }
                        }
                    }
                    main.content {
                        header.topbar {
                            h1.tb-title { @if let Some(o) = onglet { (o.titre()) } @else { (titre) } }
                            form.tb-search method="get" action="/admin/players" role="search" {
                                input #tb-q type="search" name="q" placeholder="Search for a player…" aria-label="Search for a player" autocomplete="off";
                                (icons::search())
                            }
                            div.tb-right {
                                span.tb-live title="Session" { i {} code #uptime { "00:00:00" } }
                                span.tb-av { img src="/assets/icone.png" alt="" width="22" height="22"; }
                                form method="post" action="/admin/logout" {
                                    button.tb-out type="submit" aria-label="Sign out" title="Sign out" { (icons::arrow_right()) }
                                }
                            }
                        }
                        div.scroll {
                            @if let Some(f) = flash {
                                @if let Some(m) = f.ok { div.flash.ok role="status" { (icons::check()) span { (m) } } }
                                @if let Some(m) = f.err { div.flash.err role="alert" { (icons::alert()) span { (m) } } }
                            }
                            div.page { (corps) }
                        }
                    }
                }
            }
        }
    }
}

/// Sous la barre du haut (qui porte déjà le titre) : la phrase qui présente la page.
fn entete(_titre: &str, sous: Option<&str>) -> Markup {
    html! {
        @if let Some(s) = sous { p.page-sub { (s) } }
    }
}

// ---------- Connexion ----------

/// La connexion, comme une session SSH : les lignes s'écrivent l'une après l'autre (admin.js),
/// puis l'invite du mot de passe. Sans JavaScript, tout est affiché d'un coup.
/// Connexion : à gauche le nom et le mot de passe, à droite le paysage du launcher (pixel art) sous
/// une brume de caractères.
pub fn connexion(erreur: Option<&str>, scene: &str) -> Markup {
    let v = ASSET_VERSION.as_str();
    let lien = |href: &str, titre: &str, icone: Markup| {
        html! { a href=(href) target="_blank" rel="noopener" aria-label=(titre) title=(titre) { (icone) } }
    };
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="robots" content="noindex, nofollow";
                meta name="color-scheme" content="dark";
                title { "Sign in · Mistly Admin" }
                link rel="icon" href="/assets/icone.png";
                link rel="preload" href="/assets/Geist-latin.woff2" as="font" type="font/woff2" crossorigin;
                link rel="stylesheet" href=(format!("/css/base.css?v={v}"));
                link rel="stylesheet" href=(format!("/css/admin.css?v={v}"));
                script src=(format!("/js/admin.js?v={v}")) defer {}
            }
            body.admin.login-page {
                div.lg {
                    section.lg-side {
                        span.lg-host { i {} "mistly.cc / admin" }
                        div.lg-main {
                            img.lg-logo src="/assets/icone.png" alt="" width="56" height="56";
                            h1 { "Mistly " strong { "Admin" } }
                            p.lg-sub { "The team dashboard." }
                            form.lg-form #login method="post" action="/admin/login" data-error=[erreur.map(|_| "1")] {
                                label.sr for="pw" { "Password" }
                                div.lg-field {
                                    (icons::lock())
                                    input #pw type="password" name="password" placeholder="Password" autocomplete="current-password" required autofocus spellcheck="false";
                                    button.lg-eye type="button" aria-label="Show the password" aria-pressed="false" { "show" }
                                }
                                @if let Some(e) = erreur { p.lg-err role="alert" { (icons::alert()) (e) } }
                                button.lg-btn.solid type="submit" {
                                    span.t { "Sign in" }
                                    (icons::arrow_right())
                                }
                            }
                            a.lg-btn href="https://github.com/Mistly-MC/Mistly-Web" target="_blank" rel="noopener" {
                                span.t { "View the code" }
                                span.lg-gh { (icons::github()) "GitHub" }
                            }
                            div.lg-social {
                                (lien(super::layout::DISCORD_URL, "Discord", icons::discord()))
                                (lien(super::layout::GITHUB_ORG_URL, "GitHub", icons::github()))
                                (lien(&site("/status"), "Status", icons::globe()))
                                (lien(&site("/"), "Website", icons::external()))
                            }
                        }
                        nav.lg-foot {
                            a href=(site("/privacy?lang=en")) { "Privacy" }
                            a href=(site("/terms?lang=en")) { "Terms" }
                            a href=(site("/status?lang=en")) { "Status" }
                        }
                    }
                    div.lg-art aria-hidden="true" {
                        div.lg-sky { (PreEscaped(scene)) }
                        canvas.adm-bg {}
                        div.lg-badge { img src="/assets/icone.png" alt="" width="18" height="18"; span { "Team access only" } }
                        p.lg-quote { "Light as the mist." }
                    }
                }
            }
        }
    }
}

// ---------- Vue d'ensemble ----------

pub fn accueil(flash: &Flash, c: &Chiffres, en_ligne: &[Ligne], telemetrie: Option<&Value>, journal: &[Entree]) -> Markup {
    // Une grande carte pastel : l'icône dans un rond, le titre, le chiffre en direct, des détails
    let carte = |teinte: &str, icone: Markup, titre: &str, valeur: String, id: Option<&str>, details: Vec<(Markup, String)>| {
        html! {
            div.ov-card.(teinte) {
                div.ov-card-head {
                    span.ov-ic { (icone) }
                    span { (titre) }
                }
                strong.ov-num id=[id] { (valeur) }
                @if let Some(i) = id { svg.mini data-spark=(i) viewBox="0 0 120 32" preserveAspectRatio="none" aria-hidden="true" {} }
                ul {
                    @for (ic, texte) in details { li { (ic) span { (texte) } } }
                }
            }
        }
    };
    shell(
        "Overview",
        Some(Onglet::Accueil),
        Some(flash),
        html! {
            div.ov {
                div.ov-main {
                    div.ov-cards {
                        (carte("p-lilac", icons::download(), "Open launchers", c.launchers.to_string(), Some("n-launchers"), vec![
                            (icons::check(), format!("{} in game right now", c.en_partie)),
                            (icons::check(), format!("Record: {} at once", c.record)),
                            (icons::check(), format!("Version {}", c.version.as_deref().unwrap_or("?"))),
                        ]))
                        (carte("p-pink", icons::user(), "Mistly Client", c.client.to_string(), Some("n-client"), vec![
                            (icons::check(), format!("{} players in the database", c.joueurs)),
                            (icons::check(), format!("{} active in 24 h", c.actifs_24h)),
                            (icons::check(), format!("{} banned", c.bannis)),
                        ]))
                        (carte("p-peach", icons::sparkles(), "Growth", format!("+{}", c.nouveaux_24h), None, vec![
                            (icons::check(), "new players in 24 h".into()),
                            (icons::check(), format!("+{} in 7 days", c.nouveaux_7j)),
                            (icons::check(), format!("Uptime: {}", c.dispo.map_or_else(|| "?".into(), |d| format!("{d:.1} %")))),
                        ]))
                    }
                    @if let Some(t) = telemetrie { (telemetrie_bloc(t)) } @else {
                        section.ov-panel { p.empty { "Telemetry is not available." } }
                    }
                }
                aside.ov-side {
                    div.side-sec {
                        div.side-head { h2 { "Playing now" } span.live { i {} "live" } }
                        (liste_en_ligne(en_ligne))
                    }
                    div.side-sec {
                        div.side-head { h2 { "Latest changes" } a.more href="/admin/log" { "View all" } }
                        @if journal.is_empty() {
                            p.empty { "No changes yet." }
                        } @else {
                            ul.msgs {
                                @for e in journal.iter().take(6) {
                                    li {
                                        span.msg-ic { (icons::notes()) }
                                        div { strong { (e.text) } time { (il_y_a(e.at)) @if let Some(ip) = &e.ip { " · " code.ip { (ip) } } } }
                                    }
                                }
                            }
                        }
                    }
                    @if let Some(t) = telemetrie { (repartition(t)) }
                    div.side-sec {
                        div.side-head { h2 { "Open a report" } }
                        form.quick method="get" action="/admin/reports" {
                            input type="text" name="code" placeholder="MR-ABCD234" aria-label="Report code" autocomplete="off" spellcheck="false";
                            button type="submit" aria-label="Open" { (icons::arrow_right()) }
                        }
                    }
                }
            }
        },
    )
}

/// Les joueurs en jeu : une ligne par joueur (tête, pseudo, ce qu'il porte, depuis quand).
/// admin.js la remplace toutes les 15 secondes avec le même gabarit.
fn liste_en_ligne(en_ligne: &[Ligne]) -> Markup {
    html! {
        div #online {
            @if en_ligne.is_empty() {
                p.empty { "Nobody is playing with Mistly Client." }
            } @else {
                ul.msgs {
                    @for l in en_ligne {
                        li {
                            (tete(l.nom.as_deref(), 34))
                            a href=(format!("/admin/players/{}", l.uuid)) {
                                strong { (nom_ou_uuid(l.nom.as_deref(), &l.uuid)) @if l.note { " " span.tag title="Has a note" { "note" } } }
                                span {
                                    @let porte: Vec<&str> = [l.cosmetique, l.chapeau].into_iter().filter(|c| !c.is_empty()).map(nom_cosmetique).collect();
                                    @if porte.is_empty() { "—" } @else { (porte.join(" + ")) }
                                }
                            }
                            span.since { (duree(l.depuis)) }
                        }
                    }
                }
            }
        }
    }
}

/// La répartition (versions du launcher, parties par loader), dans la colonne de droite.
fn repartition(t: &Value) -> Markup {
    let paires = |v: &Value| -> Vec<(String, u64)> {
        let mut l: Vec<(String, u64)> = v.as_object().map(|o| o.iter().map(|(k, v)| (k.clone(), v.as_u64().unwrap_or(0))).collect()).unwrap_or_default();
        l.sort_by_key(|x| std::cmp::Reverse(x.1));
        l
    };
    let versions = paires(&t["launcher_versions"]);
    let loaders = paires(&t["games_by_loader"]);
    let crashs = paires(&t["crashes_by_loader"]);
    let ligne = |teinte: &str, ic: Markup, nom: &str, detail: String, pastille: String| {
        html! {
            li.rep {
                span.rep-ic.(teinte) { (ic) }
                div { strong { (nom) } span { (detail) } }
                span.pill.(teinte) { (pastille) }
            }
        }
    };
    html! {
        div.side-sec {
            div.side-head { h2 { "Launcher versions" } }
            @if versions.is_empty() { p.empty { "—" } }
            ul.reps { @for (k, n) in versions.iter().take(3) { (ligne("p-lilac", icons::tag(), &format!("Launcher {k}"), format!("{n} install(s)"), format!("{n}"))) } }
        }
        div.side-sec {
            div.side-head { h2 { "Games by loader" } }
            @if loaders.is_empty() { p.empty { "—" } }
            ul.reps {
                @for (k, n) in loaders.iter().take(3) {
                    @let c = crashs.iter().find(|x| x.0 == *k).map_or(0, |x| x.1);
                    (ligne(if c > 0 { "p-pink" } else { "p-mint" }, icons::package(), k, format!("{c} crash(es)"), format!("{n}")))
                }
            }
        }
    }
}

fn telemetrie_bloc(t: &Value) -> Markup {
    let paires = |v: &Value| -> Vec<(String, u64)> {
        let mut l: Vec<(String, u64)> = v.as_object().map(|o| o.iter().map(|(k, v)| (k.clone(), v.as_u64().unwrap_or(0))).collect()).unwrap_or_default();
        l.sort_by_key(|x| std::cmp::Reverse(x.1));
        l
    };
    // Les deux courbes (installations et événements par jour), dessinées par admin.js
    let jours: Vec<(String, u64, u64)> = t["per_day"]
        .as_object()
        .map(|o| o.iter().map(|(k, v)| (k.clone(), v["installs"].as_u64().unwrap_or(0), v["events"].as_u64().unwrap_or(0))).collect())
        .unwrap_or_default();
    let series = serde_json::json!({
        "labels": jours.iter().map(|j| j.0.clone()).collect::<Vec<_>>(),
        "a": jours.iter().map(|j| j.1).collect::<Vec<_>>(),
        "b": jours.iter().map(|j| j.2).collect::<Vec<_>>(),
    });
    let plantages: u64 = paires(&t["crashes_by_loader"]).iter().map(|x| x.1).sum();
    html! {
        section.ov-panel.ov-chart {
            div.chart-head {
                div {
                    h2 { "Activity" }
                    div.legend-line {
                        span.k-a { i {} "Active installs" }
                        span.k-b { i {} "Events" }
                    }
                }
                div.tabs role="group" aria-label="Range" {
                    button type="button" data-jours="7" { "7 days" }
                    button type="button" data-jours="14" { "14 days" }
                    button type="button" data-jours="30" aria-pressed="true" { "30 days" }
                }
            }
            div.chart-box #chart data-series=(series.to_string()) {
                @if jours.len() < 2 { p.empty { "Not enough days yet to draw a curve." } }
            }
            div.ov-mini {
                div { span { "Installs" } strong { (t["installs"].as_u64().unwrap_or(0)) } }
                div { span { "Games started" } strong { (t["events"]["game_start"].as_u64().unwrap_or(0)) } }
                div { span { "Crashes" } strong { (plantages) } }
                div { span { "Reports" } strong { (t["reports"].as_u64().unwrap_or(0)) } }
            }
        }

    }
}

// ---------- Joueurs ----------

pub fn joueurs(flash: &Flash, q: &str, filtre: &str, lignes: &[Fiche], total: usize, page: usize, pages: usize) -> Markup {
    let filtres = [("all", "All"), ("online", "Online"), ("granted", "Changed cosmetics"), ("banned", "Banned"), ("notes", "With a note"), ("manual", "Added by hand")];
    let lien_page = |p: usize| format!("/admin/players?q={}&filter={filtre}&page={p}", encoder(q));
    shell(
        "Players",
        Some(Onglet::Joueurs),
        Some(flash),
        html! {
            (entete("Players", Some("Every account seen by Mistly Client, and those added by hand.")))
            div.toolbar {
                form.search method="get" action="/admin/players" {
                    (icons::search())
                    input type="search" name="q" value=(q) placeholder="Username or UUID…" aria-label="Search for a player";
                    input type="hidden" name="filter" value=(filtre);
                }
                form.add method="post" action="/admin/players/add" {
                    input type="text" name="name" placeholder="Username or UUID to add" aria-label="Add a player" required;
                    button.btn.sm type="submit" { "Add" }
                }
            }
            div.chips {
                @for (code, nom) in filtres {
                    a href=(format!("/admin/players?q={}&filter={code}", encoder(q))) aria-current=[(code == filtre).then_some("true")] { (nom) }
                }
            }
            section.panel.flush {
                @if lignes.is_empty() {
                    p.empty { "No player matches." }
                } @else {
                    table.tbl {
                        thead { tr { th { "Player" } th { "Cosmetics" } th { "First seen" } th { "Last seen" } th.num { "Sign-ins" } } }
                        tbody {
                            @for l in lignes {
                                @let f = &l.fiche;
                                tr {
                                    td {
                                        a.who href=(format!("/admin/players/{}", l.uuid)) {
                                            (tete(f.name.as_deref(), 28))
                                            span {
                                                strong { (nom_ou_uuid(f.name.as_deref(), &l.uuid)) }
                                                small.mono { (avec_tirets(&l.uuid)) }
                                            }
                                        }
                                        @if l.en_ligne { span.tag.on { "online" } }
                                        @if f.banned { span.tag.bad { "banni" } }
                                        @if !f.note.is_empty() { span.tag title=(f.note) { "note" } }
                                    }
                                    td.muted {
                                        @for c in &f.granted { span.tag.give { "+ " (nom_cosmetique(c)) } }
                                        @for c in &f.revoked { span.tag.take { "− " (nom_cosmetique(c)) } }
                                        @if f.granted.is_empty() && f.revoked.is_empty() { "—" }
                                    }
                                    td.muted { @match f.first_seen { Some(t) => (date(t)), None => "added by hand" } }
                                    td.muted { @match f.last_seen { Some(t) => (il_y_a(t)), None => "jamais" } }
                                    td.num.muted { (f.logins) }
                                }
                            }
                        }
                    }
                }
            }
            div.pager {
                span.muted { (total) " player(s)" }
                @if pages > 1 {
                    div {
                        @if page > 1 { a.btn.sm href=(lien_page(page - 1)) { "Previous" } }
                        span.muted { "page " (page) " / " (pages) }
                        @if page < pages { a.btn.sm href=(lien_page(page + 1)) { "Next" } }
                    }
                }
            }
        },
    )
}

fn encoder(t: &str) -> String {
    t.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

// ---------- Fiche d'un joueur ----------

pub fn fiche(flash: &Flash, uuid: &str, f: &FicheBase, en_ligne: Option<&EnLigne>, cosmetiques: &[EtatCosmetique], early_avant: Option<u64>, journal: &[Entree]) -> Markup {
    let nom = f.name.as_deref();
    let action = format!("/admin/players/{uuid}");
    let bouton = |act: &str, cos: &str, texte: &str, classe: &str| {
        html! {
            form method="post" action=(action) {
                input type="hidden" name="action" value=(act);
                input type="hidden" name="cosmetic" value=(cos);
                button class=(format!("btn sm {classe}")) type="submit" { (texte) }
            }
        }
    };
    let early_ok = match (early_avant, f.first_seen) {
        (None, _) => true,
        (Some(l), Some(v)) => v < l,
        _ => false,
    };
    shell(
        nom.unwrap_or("Player"),
        Some(Onglet::Joueurs),
        Some(flash),
        html! {
            nav.crumbs { a href="/admin/players" { "Players" } span { "/" } span { (nom_ou_uuid(nom, uuid)) } }
            section.profile.panel {
                div.avatar { (tete(nom, 96)) }
                div.who-big {
                    h1 {
                        (nom.unwrap_or("Unknown username"))
                        @if en_ligne.is_some() { span.tag.on { "online" } }
                        @if f.banned { span.tag.bad { "banni" } }
                    }
                    button.uuid type="button" data-copy=(avec_tirets(uuid)) title="Copy the UUID" { code { (avec_tirets(uuid)) } (icons::copy()) }
                    div.facts {
                        div { span { "First seen" } strong { @match f.first_seen { Some(t) => (date(t)), None => "added by hand" } } }
                        div { span { "Last seen" } strong { @match f.last_seen { Some(t) => (il_y_a(t)), None => "jamais" } } }
                        div { span { "Sign-ins" } strong { (f.logins) } }
                        div { span { "Early player" } strong { @if early_ok { "oui" } @else { "non" } } }
                        @if let Some(j) = en_ligne { div { span { "Playing for" } strong { (duree(j.depuis)) } } }
                    }
                    @if let Some(n) = nom {
                        div.links {
                            a href=(site(&format!("/shop?player={n}"))) target="_blank" rel="noopener" { "3D preview" (icons::external()) }
                            a href=(format!("https://namemc.com/profile/{n}")) target="_blank" rel="noopener" { "NameMC" (icons::external()) }
                        }
                    }
                }
                div.actions {
                    @if en_ligne.is_some() {
                        form method="post" action=(action) { input type="hidden" name="action" value="kick"; button.btn.sm type="submit" { "Kick from the network" } }
                    }
                    @if f.banned {
                        form method="post" action=(action) { input type="hidden" name="action" value="unban"; button.btn.sm type="submit" { "Lift the ban" } }
                    } @else {
                        form method="post" action=(action) data-confirm="Ban this player from the Mistly network? They lose their cosmetics and can no longer connect." {
                            input type="hidden" name="action" value="ban"; button.btn.sm.danger type="submit" { "Ban" }
                        }
                    }
                }
            }

            div.grid2 {
                section.panel {
                    div.panel-head { h2 { (icons::sparkles()) "Cosmetics" } a.more href="/admin/cosmetics" { "Rules" (icons::arrow_right()) } }
                    @if f.banned { p.note-bad { "Banned: no cosmetic is relayed while the ban lasts." } }
                    table.tbl.cos {
                        thead { tr { th { "Cosmetic" } th { "Status" } th {} } }
                        tbody {
                            @for e in cosmetiques {
                                tr {
                                    td {
                                        strong { (e.c.nom) }
                                        small.muted { (if e.c.place == Place::Tete { "head" } else { "dos" }) " · rule: " (e.regle.libelle()) }
                                    }
                                    td {
                                        @if e.permis { span.state.yes { "can wear it" } } @else { span.state.no { "cannot" } }
                                        @if e.donne { span.tag.give { "given" } }
                                        @if e.retire { span.tag.take { "taken" } }
                                        @if e.porte { span.tag.on { "wears it" } }
                                    }
                                    td.btns {
                                        @if !e.donne { (bouton("grant", e.c.id, "Give", "")) }
                                        @if !e.retire { (bouton("revoke", e.c.id, "Take", "danger")) }
                                        @if e.donne || e.retire { (bouton("default", e.c.id, "Default", "ghost")) }
                                    }
                                }
                            }
                        }
                    }
                }
                section.panel {
                    div.panel-head { h2 { (icons::notes()) "Team note" } }
                    form.note method="post" action=(action) {
                        input type="hidden" name="action" value="note";
                        textarea name="note" rows="6" maxlength=(crate::registre::MAX_NOTE) placeholder="Only visible here." { (f.note) }
                        button.btn.sm type="submit" { "Save the note" }
                    }
                    div.panel-head.sub { h2 { (icons::refresh()) "History" } }
                    @if journal.is_empty() {
                        p.empty { "No changes on this player." }
                    } @else {
                        ul.log { @for e in journal { li { time title=(date(e.at)) { (il_y_a(e.at)) } span { (e.text) @if let Some(ip) = &e.ip { " " code.ip title="IP address" { (ip) } } } } } }
                    }
                }
            }

            section.panel.danger-zone {
                div { strong { "Delete the record" } p.muted { "Forgets this account: dates, given or taken cosmetics, note. If they play again, a new record is created (without the early player cape if the date has passed)." } }
                form method="post" action=(action) data-confirm="Permanently delete this player's record?" {
                    input type="hidden" name="action" value="forget"; button.btn.sm.danger type="submit" { "Delete" }
                }
            }
        },
    )
}

// ---------- Cosmétiques ----------

pub fn cosmetiques(flash: &Flash, lignes: &[RegleCosmetique], early: &Early) -> Markup {
    shell(
        "Cosmetics",
        Some(Onglet::Cosmetiques),
        Some(flash),
        html! {
            (entete("Cosmetics", Some("Who can wear each cosmetic. A cosmetic given to or taken from a player on their page overrides the rule (except “Disabled”).")))
            section.panel.flush {
                table.tbl {
                    thead { tr { th { "Cosmetic" } th { "Rule" } th.num { "Given" } th.num { "Taken" } th.num { "Worn now" } } }
                    tbody {
                        @for l in lignes {
                            tr {
                                td { strong { (l.c.nom) } small.muted.mono { (l.c.id) } }
                                td {
                                    form.rule method="post" action="/admin/cosmetics" {
                                        input type="hidden" name="cosmetic" value=(l.c.id);
                                        select name="rule" aria-label=(format!("Rule for {}", l.c.nom)) data-autosubmit {
                                            @for r in Regle::TOUTES {
                                                option value=(r.code()) selected[r == l.regle] { (r.libelle()) @if r == l.c.regle_par_defaut { " (default)" } }
                                            }
                                        }
                                        noscript { button.btn.sm type="submit" { "OK" } }
                                    }
                                }
                                td.num { (l.donnes) }
                                td.num { (l.retires) }
                                td.num { (l.portes) }
                            }
                        }
                    }
                }
            }
            dl.legend {
                @for r in Regle::TOUTES {
                    div {
                        dt { (r.libelle()) }
                        dd { (match r {
                            Regle::Everyone => "Every Mistly Client player.",
                            Regle::Early => "Accounts seen before the early players date.",
                            Regle::Granted => "Only players it was given to.",
                            Regle::Off => "Nobody, not even those it was given to.",
                        }) }
                    }
                }
            }
            section.panel {
                div.panel-head { h2 { (icons::crown()) "Early players date" } }
                p.muted {
                    "Accounts first seen before this date get the cosmetics set to “Early players” (the early player cape). "
                    @match &early.date {
                        Some(d) => { "Current date: " strong { (d) } @if !early.depuis_la_base { " (EARLY_BEFORE variable)" } "." }
                        None => { "No date: everyone gets it." }
                    }
                    @if let Some(n) = early.eligibles { " " (n) " player(s) y ont droit." }
                }
                form.inline method="post" action="/admin/cosmetics" {
                    input type="date" name="early_before" value=[early.date.as_deref()] aria-label="Early players date";
                    button.btn.sm type="submit" { "Save" }
                    @if early.depuis_la_base {
                        button.btn.sm.ghost type="submit" name="early_before" value="" { "Back to EARLY_BEFORE" }
                    }
                }
            }
        },
    )
}

// ---------- Boutique ----------

/// Les réglages de la boutique : une ligne par article (prix, statut, nouveautés, populaires,
/// caché), enregistrées d'un coup.
pub fn boutique(flash: &Flash, lignes: &[crate::views::shop::LigneAdmin]) -> Markup {
    use crate::boutique::{self as b, Statut};
    shell(
        "Shop",
        Some(Onglet::Boutique),
        Some(flash),
        html! {
            (entete("Shop", Some("Prices, status and where each item shows up in the shop. Leave a price empty to show none; “Auto” keeps the status set in the code.")))
            form.shopcfg method="post" action="/admin/shop" {
                section.panel.flush {
                    table.tbl {
                        thead { tr { th { "Item" } th { "Status" } th { "Price (€)" } th.c { "New" } th.c { "Popular" } th.c { "Hidden" } th {} } }
                        tbody {
                            @for l in lignes {
                                @let r = b::de(l.cle);
                                tr.hidden-row[r.cache] {
                                    td {
                                        strong { (l.nom) }
                                        small.muted { (l.categorie) " · " code { (l.cle) } }
                                    }
                                    td {
                                        select name=(format!("{}.statut", l.cle)) aria-label=(format!("Status of {}", l.nom)) {
                                            option value="" selected[r.statut.is_none()] { "Auto (" (l.statut_defaut) ")" }
                                            @for s in Statut::TOUS {
                                                option value=(s.code()) selected[r.statut == Some(s)] { (s.libelle()) }
                                            }
                                        }
                                    }
                                    td {
                                        input.price type="text" inputmode="decimal" name=(format!("{}.prix", l.cle)) placeholder="—"
                                            value=(r.prix.map(|c| b::prix(c, ".").trim_end_matches(" €").to_string()).unwrap_or_default())
                                            aria-label=(format!("Price of {}", l.nom));
                                    }
                                    td.c { label.switch { input type="checkbox" name=(format!("{}.nouveau", l.cle)) value="1" checked[r.nouveau]; span {} } }
                                    td.c { label.switch { input type="checkbox" name=(format!("{}.populaire", l.cle)) value="1" checked[r.populaire]; span {} } }
                                    td.c { label.switch.danger { input type="checkbox" name=(format!("{}.cache", l.cle)) value="1" checked[r.cache]; span {} } }
                                    td.num { a.more href=(site(&format!("/shop/item/{}", l.cle))) target="_blank" rel="noopener" { "View" (icons::external()) } }
                                }
                            }
                        }
                    }
                }
                div.savebar {
                    p.muted { "If no item is ticked in “New” or “Popular”, the shop uses its default selection." }
                    button.btn.play type="submit" { (icons::check()) "Save the shop" }
                }
            }
        },
    )
}

// ---------- Docs de l'équipe ----------

/// Ce que la page de diagnostic montre en direct.
pub struct Diag {
    pub launchers: usize,
    pub en_partie: usize,
    pub client: usize,
    pub fiches: usize,
    pub dossier: Option<String>,
    /// (fichier ou dossier dans le dossier de données, taille en octets s'il existe)
    pub fichiers: Vec<(&'static str, &'static str, Option<u64>)>,
    /// (nom, dépôt, état)
    pub github: Vec<(&'static str, String, crate::github::Sante)>,
    /// (projet, dépôt, exécutions lues)
    pub deploiements: Vec<(&'static str, String, Option<usize>)>,
    pub contributeurs: bool,
    pub services: (usize, usize),
    pub age_statut: Option<u64>,
    pub dispo: Option<f32>,
    /// (jours de télémétrie sur le disque, rapports gardés, lecture par jeton possible)
    pub telemetrie: (usize, usize, bool),
    /// (variable, réglée)
    pub reglages: Vec<(&'static str, bool, &'static str)>,
}

/// Une route de l'API : méthode, chemin, qui peut l'appeler, ce qu'elle fait.
type Route = (&'static str, &'static str, &'static str, &'static str);
/// Les routes de l'API, par groupe, pour la page Docs.
const API: [(&str, &[Route]); 6] = [
    ("Website & launcher", &[
        ("GET", "/api/latest", "public", "Latest launcher release: version, date, notes and file sizes. Used by the launcher's updater and the website. 503 while GitHub was never reached."),
        ("GET", "/download/{setup|portable|linux}", "public", "Downloads the latest launcher file: server copy first, then relayed from GitHub, then a redirect to GitHub."),
        ("GET", "/status.json", "public", "Every checked service with its state, latency and 30-day uptime."),
        ("GET", "/api/online", "public", "Launchers open right now (seen in the last 3 minutes) and how many are in a game."),
        ("POST", "/api/presence", "launcher", "Heartbeat sent every 60 s by each open launcher: a random per-launch session id and whether a game is running. Kept in memory only."),
        ("GET", "/skin/{name} · /cape/{name} · /player/{name}", "public", "Minecraft skin, cape and profile relayed from Mojang (fallback mc-heads.net), for the shop's 3D preview and player heads."),
    ]),
    ("Mistly Client (in game)", &[
        ("POST", "/api/client/login", "game", "The game proves its account like on a Minecraft server (joinServer / hasJoined at Mojang) and gets a token. The Minecraft token never leaves the game."),
        ("POST", "/api/client/presence", "token", "Heartbeat every minute with what the player wears (back: cape or elytra; head: hat)."),
        ("POST", "/api/client/players", "token", "Among the given players (those on the same server), who plays with Mistly and what they wear."),
        ("GET", "/api/client/early/{uuid}", "public", "Whether this account gets the early player cape."),
        ("GET", "/api/client/cosmetics/{uuid}", "public", "The cosmetics this account may wear (rules, given or taken cosmetics, bans)."),
    ]),
    ("Telemetry", &[
        ("POST", "/api/telemetry", "launcher", "One event (game start, game end or crash, update…) from players who accepted telemetry."),
        ("POST", "/api/telemetry/report", "launcher", "The last console lines of a game (sent by the player, or after a crash). Answers the report code (MR-XXXXXXX)."),
        ("GET", "/api/telemetry/summary", "TELEMETRY_TOKEN", "30-day summary: installs, events, versions, games and crashes by loader, reports. Bearer token."),
        ("GET", "/api/telemetry/report/{code}", "TELEMETRY_TOKEN", "One log report. Bearer token. Also readable here in Reports."),
    ]),
    ("Discord bot (captcha)", &[
        ("GET", "/verify", "public", "Captcha page a new member opens from Mistly Bot."),
        ("POST", "/verify/new · /verify/answer", "public", "New puzzle, and the answer check (then the bot gives the role)."),
        ("GET", "/verify/captcha/{id}/{file}", "public", "Puzzle images."),
        ("POST", "/api/verify/config", "VERIFY_SECRET", "The bot sends each server's settings (role to give)."),
        ("GET/POST", "/activity · /activity/start · /activity/log", "Discord", "The same captcha inside Discord (Activity)."),
    ]),
    ("Pages & search engines", &[
        ("GET", "/docs/search.json", "public", "Full-text index of the documentation, for the docs search box."),
        ("GET", "/sitemap.xml · /robots.txt · /indexnow.txt", "public", "Search engines. IndexNow announces new pages to Bing and Yandex when INDEXNOW_KEY is set."),
        ("GET", "/site.webmanifest", "public", "Web app manifest (icon, name, colors)."),
    ]),
    ("Dashboard (this site, a0 only)", &[
        ("GET/POST", "/admin/login · /admin/logout", "password", "Signed session cookie (HMAC, 12 h, HttpOnly, Secure, SameSite=Strict). 8 wrong passwords per address lock it for 15 minutes."),
        ("GET", "/admin/online.json", "session", "Live numbers of the overview, refreshed every 15 s."),
        ("GET/POST", "/admin/players · /admin/players/{uuid}", "session", "Player list and record: give or take cosmetics, ban, kick, note, delete."),
        ("GET/POST", "/admin/cosmetics · /admin/shop", "session", "Cosmetic rules, early players date, shop prices and status."),
        ("GET/POST", "/admin/database · /admin/database.json", "session", "Edit or download joueurs.json."),
    ]),
];

pub fn docs(d: &Diag) -> Markup {
    let pastille = |ok: bool, oui: &str, non: &str| html! { @if ok { span.tag.on { (oui) } } @else { span.tag.bad { (non) } } };
    let taille = |o: u64| if o < 1024 { format!("{o} B") } else if o < 1024 * 1024 { format!("{:.1} KB", o as f64 / 1024.0) } else { format!("{:.1} MB", o as f64 / 1048576.0) };
    shell(
        "Docs",
        Some(Onglet::Docs),
        None,
        html! {
            (entete("Docs", Some("For the team: the live state of the site, every API endpoint, how telemetry works and where data is kept. Secrets are never shown here, only whether they are set.")))
            nav.doc-toc {
                a href="#diag" { "Live diagnostics" } a href="#api" { "API endpoints" } a href="#telemetry" { "Telemetry" }
                a href="#data" { "Data files" } a href="#settings" { "Settings" } a href="#domains" { "Domains" }
            }

            section.panel #diag {
                div.panel-head { h2 { (icons::bolt()) "Live diagnostics" } span.live { i {} "live" } }
                div.diag {
                    div.diag-card {
                        h3 { "Mistly network" }
                        dl {
                            div { dt { "Open launchers" } dd { (d.launchers) " · " (d.en_partie) " in game" } }
                            div { dt { "Mistly Client players online" } dd { (d.client) } }
                            div { dt { "Player records" } dd { (d.fiches) } }
                            div { dt { "Status checks" } dd {
                                (d.services.0) "/" (d.services.1) " up · checked " (d.age_statut.map_or_else(|| "not yet".into(), |s| format!("{s} s ago")))
                            } }
                            div { dt { "30-day uptime" } dd { (d.dispo.map_or_else(|| "—".into(), |v| format!("{v:.2} %"))) } }
                        }
                    }
                    div.diag-card {
                        h3 { "GitHub data" }
                        @for (nom, repo, s) in &d.github {
                            div.gh-row {
                                div {
                                    strong { (nom) }
                                    small.mono { (repo) }
                                }
                                @if s.versions > 0 && s.echecs == 0 { span.tag.on { "OK" } } @else if s.versions > 0 { span.tag.take { "stale" } } @else { span.tag.bad { "no data" } }
                            }
                            p.gh-detail {
                                (s.versions) " release(s) · latest " code { (s.derniere.as_deref().unwrap_or("—")) }
                                " · last fetched " (s.depuis.map_or_else(|| "never".into(), |x| if x < 60 { "just now".into() } else { format!("{} min ago", x / 60) }))
                                @if s.echecs > 0 { " · " span.bad-txt { (s.echecs) " failure(s) in a row" } }
                                @if let Some(e) = &s.erreur { br; span.bad-txt.mono { (e) } }
                            }
                        }
                        @for (nom, repo, n) in &d.deploiements {
                            div.gh-row {
                                div { strong { (nom) " deployments" } small.mono { (repo) } }
                                @match n { Some(n) => span.tag.on { (n) " runs" }, None => span.tag.take { "not read yet" } }
                            }
                        }
                        div.gh-row {
                            div { strong { "Contributor stats (footer)" } small.mono { "stats/contributors" } }
                            (pastille(d.contributeurs, "loaded", "not yet"))
                        }
                        div.gh-row {
                            div { strong { "GITHUB_TOKEN" } small { "5,000 requests/h instead of 60" } }
                            (pastille(d.github.first().is_some_and(|g| g.2.jeton), "set", "not set"))
                        }
                    }
                    div.diag-card {
                        h3 { "Data folder" }
                        p.gh-detail {
                            @match &d.dossier {
                                Some(dir) => { code { (dir) } " · must be a persistent volume (Coolify: Persistent Storage), or everything below is lost on each deploy." }
                                None => { span.bad-txt { "MISTY_CACHE_DIR is not set: nothing is saved to disk." } }
                            }
                        }
                        ul.files {
                            @for (nom, role, t) in &d.fichiers {
                                li {
                                    code { (nom) }
                                    span { (role) }
                                    @match t { Some(o) => span.tag.on { (taille(*o)) }, None => span.tag { "missing" } }
                                }
                            }
                        }
                    }
                }
            }

            section.panel #api {
                div.panel-head { h2 { (icons::layers()) "API endpoints" } span.muted { "Base: https://mistly.cc (the old misty-launcher.online still serves the API and downloads)" } }
                @for (groupe, routes) in API {
                    h3.api-g { (groupe) }
                    table.tbl.api {
                        tbody {
                            @for (methode, chemin, qui, quoi) in routes.iter() {
                                tr {
                                    td.m { span.verb.(methode.split('/').next().unwrap_or("").to_lowercase()) { (methode) } }
                                    td { code { (chemin) } p { (quoi) } }
                                    td.who { span.tag.give { (qui) } }
                                }
                            }
                        }
                    }
                }
                p.muted.note-small { "Rate limits (per address, in memory): telemetry 240 events and 12 reports per hour, Mistly Client a few calls per minute. No IP address is stored by the public API; only dashboard actions record the IP of the team member who made them, in the log." }
            }

            section.panel #telemetry {
                div.panel-head { h2 { (icons::bolt()) "Telemetry" } }
                div.cols2 {
                    div {
                        h3 { "What is sent" }
                        ul.dots {
                            li { "Only from players who said yes on the launcher's Privacy screen (Settings → Privacy shows exactly what is sent)." }
                            li { "Only these fields are kept, anything else is dropped: a random installation id (players can draw a new one), the launcher version, the system, the RAM and the language." }
                            li { "Events: a short name and a few flat fields. Counted here: " code { "game_start" } " (with the loader), " code { "game_end" } " (crashed or not) and " code { "update" } "." }
                            li { "Log reports: the last console lines, when the player clicks “Send logs” or after a crash. Each gets a code like " code { "MR-ABCD234" } " the player gives on Discord." }
                            li { "Never kept: IP addresses (only used in memory to limit the rate)." }
                        }
                    }
                    div {
                        h3 { "Where it goes" }
                        ul.dots {
                            li { code { "telemetry/events-YYYY-MM-DD.jsonl" } " : one line per event, one file per day." }
                            li { code { "telemetry/reports/MR-XXXXXXX.json" } " : one file per report (log capped at 700 KB)." }
                            li { "Everything older than 90 days is deleted automatically." }
                            li { "On disk now: " strong { (d.telemetrie.0) } " day(s) of events, " strong { (d.telemetrie.1) } " report(s)." }
                            li { "Read with " code { "TELEMETRY_TOKEN" } " (" (if d.telemetrie.2 { "set" } else { "not set: the read endpoints answer 404" }) "), or here: Overview and Reports." }
                            li { "With " code { "DISCORD_WEBHOOK" } ", each report and a daily summary (8 am, Paris) are posted to the team's Discord channel." }
                        }
                    }
                }
            }

            section.panel #data {
                div.panel-head { h2 { (icons::archive()) "Data files" } }
                p.muted { "Everything lives in the data folder above. joueurs.json and boutique.json are written on every change; new players are saved every 20 s and when the container stops." }
            }

            section.panel #settings {
                div.panel-head { h2 { (icons::tag()) "Settings (environment variables)" } span.muted { "set in Coolify; values are never shown" } }
                table.tbl {
                    tbody {
                        @for (nom, ok, role) in &d.reglages {
                            tr {
                                td { code { (nom) } }
                                td.muted { (role) }
                                td.num { @if *ok { span.tag.on { "set" } } @else { span.tag { "not set" } } }
                            }
                        }
                    }
                }
            }

            section.panel #domains {
                div.panel-head { h2 { (icons::globe()) "Domains" } }
                table.tbl {
                    tbody {
                        tr { td { code { "mistly.cc" } } td.muted { "The website. /admin answers 404 here." } }
                        tr { td { code { "a0.mistly.cc" } } td.muted { "This dashboard." } }
                        tr { td { code { "status.mistly.cc" } } td.muted { "Status page at the root (STATUS_URL makes /status redirect there)." } }
                        tr { td { code { "shop.mistly.cc" } } td.muted { "The shop at the root, /capes, /item/<id>… (SHOP_URL)." } }
                        tr { td { code { "docs.mistly.cc" } } td.muted { "The documentation at the root (DOCS_URL)." } }
                        tr { td { code { "misty-launcher.online" } } td.muted { "Old domain: pages redirect to mistly.cc, the API and downloads still answer (old launchers)." } }
                    }
                }
            }
        },
    )
}

// ---------- To-do ----------

/// La liste des choses à faire : une colonne par projet. Dans chacune, un champ pour ajouter, les
/// tâches à faire (bouton « Done ») puis celles terminées (on peut les rouvrir).
pub fn todo(flash: &Flash, taches: &[crate::todo::Tache]) -> Markup {
    use crate::todo::Projet;
    let bouton = |action: &str, id: u64, classe: &str, titre: &str, contenu: Markup| {
        html! {
            form method="post" action="/admin/todo" {
                input type="hidden" name="action" value=(action);
                input type="hidden" name="id" value=(id);
                button class=(classe) type="submit" title=(titre) aria-label=(titre) { (contenu) }
            }
        }
    };
    shell(
        "To-do",
        Some(Onglet::Todo),
        Some(flash),
        html! {
            (entete("To-do", Some("What's left to do on each project. Every change goes to the log with who made it.")))
            div.todo {
                @for (p, teinte, ic) in [(Projet::Launcher, "p-lilac", icons::download()), (Projet::Client, "p-pink", icons::sparkles()), (Projet::Web, "p-peach", icons::globe())] {
                    @let a_faire: Vec<&crate::todo::Tache> = taches.iter().filter(|t| t.projet == p && t.fait.is_none()).collect();
                    @let faites: Vec<&crate::todo::Tache> = { let mut f: Vec<_> = taches.iter().filter(|t| t.projet == p && t.fait.is_some()).collect(); f.sort_by_key(|t| std::cmp::Reverse(t.fait)); f };
                    section.todo-col.(teinte) {
                        header {
                            span.ov-ic { (ic) }
                            h2 { (p.nom()) }
                            span.todo-n { (a_faire.len()) " to do" }
                        }
                        @let total = a_faire.len() + faites.len();
                        div.todo-bar title=(format!("{} / {} done", faites.len(), total)) {
                            i style=(format!("width:{}%", (faites.len() * 100).checked_div(total).unwrap_or(0))) {}
                        }
                        form.todo-add method="post" action="/admin/todo" {
                            input type="hidden" name="action" value="add";
                            input type="hidden" name="projet" value=(p.code());
                            input type="text" name="texte" placeholder="Add a task…" maxlength=(crate::todo::MAX_TEXTE) required aria-label=(format!("New task for {}", p.nom()));
                            button type="submit" aria-label="Add" { (icons::arrow_right()) }
                        }
                        @if a_faire.is_empty() { p.empty { "Nothing left to do." } }
                        ul.todo-list {
                            @for t in &a_faire {
                                li {
                                    (bouton("done", t.id, "todo-check", "Done", html! { (icons::check()) }))
                                    div { span { (t.texte) } small { "added " (il_y_a(t.cree)) } }
                                    (bouton("delete", t.id, "todo-del", "Delete", html! { "✕" }))
                                }
                            }
                        }
                        @if !faites.is_empty() {
                            details.todo-done {
                                summary { "Done" span { (faites.len()) } }
                                ul.todo-list {
                                    @for t in &faites {
                                        li.fait {
                                            (bouton("undo", t.id, "todo-check on", "Reopen", html! { (icons::check()) }))
                                            div { span { (t.texte) } small { "done " (il_y_a(t.fait.unwrap_or(t.cree))) } }
                                            (bouton("delete", t.id, "todo-del", "Delete", html! { "✕" }))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}

// ---------- Rapports ----------

pub fn rapports(erreur: Option<&str>, recents: &[Value], detail: Option<&Value>) -> Markup {
    let flash = Flash { ok: None, err: erreur };
    shell(
        "Log reports",
        Some(Onglet::Rapports),
        Some(&flash),
        html! {
            (entete("Log reports", Some("Sent by players with “Send logs” in the console, or automatically after a crash.")))
            form.search.wide method="get" action="/admin/reports" {
                (icons::search())
                input type="search" name="code" placeholder="Report code, for example MR-ABCD234" aria-label="Report code" autofocus[detail.is_none()];
                button.btn.sm type="submit" { "Open" }
            }
            @if let Some(r) = detail {
                section.panel {
                    div.panel-head {
                        h2.mono { (r["code"].as_str().unwrap_or("?")) }
                        a.more href="/admin/reports" { "All reports" (icons::arrow_right()) }
                    }
                    div.facts {
                        @for (cle, libelle) in [("reason", "Reason"), ("version", "Launcher"), ("game", "Game"), ("os", "Windows"), ("ram_mo", "RAM (MB)"), ("langue", "Language"), ("id", "Installation")] {
                            @if let Some(v) = r.get(cle).filter(|v| !v.is_null()) {
                                div { span { (libelle) } strong.mono { (v.as_str().map_or_else(|| v.to_string(), str::to_string)) } }
                            }
                        }
                        @if let Some(t) = r["received"].as_u64() { div { span { "Received" } strong { (date(t)) } } }
                    }
                    div.logbar {
                        span.muted { "Log" }
                        div {
                            label.check { input type="checkbox" data-filter-errors; " Errors only" }
                            button.btn.sm type="button" data-copy-log { "Copy" }
                        }
                    }
                    pre.logview { (r["log"].as_str().unwrap_or("")) }
                }
            } @else {
                section.panel.flush {
                    @if recents.is_empty() {
                        p.empty { "No reports." }
                    } @else {
                        table.tbl {
                            thead { tr { th { "Code" } th { "Reason" } th { "Game" } th { "Last line" } th.num { "Received" } } }
                            tbody {
                                @for r in recents {
                                    @let code = r["code"].as_str().unwrap_or("?");
                                    tr {
                                        td { a.mono href=(format!("/admin/reports?code={code}")) { (code) } }
                                        td { span.tag.(if r["reason"].as_str() == Some("crash") { "bad" } else { "" }) { (r["reason"].as_str().unwrap_or("—")) } }
                                        td.muted { (r["game"].as_str().unwrap_or("—")) }
                                        td.muted.clip.mono { (r["last"].as_str().unwrap_or("")) }
                                        td.num.muted { @if let Some(t) = r["received"].as_u64() { (il_y_a(t)) } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}

// ---------- Journal ----------

pub fn journal(log: &[Entree]) -> Markup {
    shell(
        "Log",
        Some(Onglet::Journal),
        None,
        html! {
            (entete("Log", Some("Everything changed from the dashboard, newest first.")))
            section.panel {
                @if log.is_empty() {
                    p.empty { "The log is empty." }
                } @else {
                    ul.log.full { @for e in log.iter().rev() { li { time { (date(e.at)) } span { (e.text) @if let Some(ip) = &e.ip { " " code.ip title="IP address" { (ip) } } } } } }
                }
            }
        },
    )
}

// ---------- Base de données ----------

pub fn base(flash: &Flash, json: &str, joueurs: usize, journal: usize) -> Markup {
    shell(
        "Database",
        Some(Onglet::Base),
        Some(flash),
        html! {
            (entete("Database", Some("The content of joueurs.json: player records, cosmetic rules and the early players date.")))
            div.toolbar {
                span.muted {
                    (joueurs) " record(s) · " (journal) " log entries (not editable here) · data folder: "
                    @match std::env::var("MISTY_CACHE_DIR") { Ok(d) => code { (d) }, Err(_) => strong.bad-txt { "none, nothing is saved to disk" } }
                }
                a.btn.sm href="/admin/database.json" download { (icons::download()) "Download a backup" }
            }
            section.panel {
                form.dbedit method="post" action="/admin/database" data-confirm="Replace the whole database with this content? Download a backup first." {
                    textarea name="json" spellcheck="false" autocomplete="off" aria-label="Database JSON content" { (json) }
                    div.dbbar {
                        p.muted {
                            "UUID as key (32 hex digits, no dashes). "
                            "Cosmetics: " @for (i, c) in crate::registre::CATALOGUE.iter().enumerate() { @if i > 0 { ", " } code { (c.id) } } ". "
                            "Rules: " code { "everyone" } ", " code { "early" } ", " code { "granted" } ", " code { "off" } "."
                        }
                        span #json-state.muted {}
                        button.btn.play.sm type="submit" { "Save the database" }
                    }
                }
            }
        },
    )
}
