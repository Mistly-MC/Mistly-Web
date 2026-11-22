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
    Rapports,
    Journal,
    Base,
}

// ---------- Mise en forme ----------

const MOIS: [&str; 12] = ["janv.", "févr.", "mars", "avr.", "mai", "juin", "juil.", "août", "sept.", "oct.", "nov.", "déc."];

/// « 16 nov. 2026, 14:03 » (UTC).
fn date(secs: u64) -> String {
    let Ok(d) = time::OffsetDateTime::from_unix_timestamp(secs as i64) else { return "?".into() };
    format!("{} {} {}, {:02}:{:02}", d.day(), MOIS[d.month() as usize - 1], d.year(), d.hour(), d.minute())
}

/// « il y a 3 h », « il y a 2 j ».
fn il_y_a(secs: u64) -> String {
    let ecart = crate::registre::maintenant().saturating_sub(secs);
    match ecart {
        0..60 => "à l'instant".into(),
        60..3600 => format!("il y a {} min", ecart / 60),
        3600..86_400 => format!("il y a {} h", ecart / 3600),
        86_400..2_592_000 => format!("il y a {} j", ecart / 86_400),
        _ => format!("il y a {} mois", ecart / 2_592_000),
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
            Onglet::Accueil => "Vue d'ensemble",
            Onglet::Joueurs => "Joueurs",
            Onglet::Cosmetiques => "Cosmétiques",
            Onglet::Rapports => "Rapports",
            Onglet::Journal => "Journal",
            Onglet::Base => "Base de données",
        }
    }
    fn icone(self) -> Markup {
        match self {
            Onglet::Accueil => icons::grid(),
            Onglet::Joueurs => icons::user(),
            Onglet::Cosmetiques => icons::sparkles(),
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
            Onglet::Rapports => "/admin/reports",
            Onglet::Journal => "/admin/log",
            Onglet::Base => "/admin/database",
        }
    }
}

fn shell(titre: &str, onglet: Option<Onglet>, flash: Option<&Flash>, corps: Markup) -> Markup {
    let v = ASSET_VERSION.as_str();
    let onglets = [Onglet::Accueil, Onglet::Joueurs, Onglet::Cosmetiques, Onglet::Rapports, Onglet::Journal, Onglet::Base];
    html! {
        (DOCTYPE)
        html lang="fr" data-theme="dark" {
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
                        nav.rail-nav aria-label="Tableau de bord" {
                            @for o in onglets {
                                a href=(o.lien()) aria-current=[(onglet == Some(o)).then_some("page")] aria-label=(o.titre()) {
                                    (o.icone())
                                    span.tip { (o.titre()) }
                                }
                            }
                        }
                        div.rail-foot {
                            a href="/" target="_blank" rel="noopener" aria-label="Voir le site" { (icons::globe()) span.tip { "Voir le site" } }
                        }
                    }
                    main.content {
                        header.topbar {
                            h1.tb-title { @if let Some(o) = onglet { (o.titre()) } @else { (titre) } }
                            form.tb-search method="get" action="/admin/players" role="search" {
                                input #tb-q type="search" name="q" placeholder="Rechercher un joueur…" aria-label="Rechercher un joueur" autocomplete="off";
                                (icons::search())
                            }
                            div.tb-right {
                                span.tb-live title="Session" { i {} code #uptime { "00:00:00" } }
                                span.tb-av { img src="/assets/icone.png" alt="" width="22" height="22"; }
                                form method="post" action="/admin/logout" {
                                    button.tb-out type="submit" aria-label="Se déconnecter" title="Se déconnecter" { (icons::arrow_right()) }
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
                                (lien("/status", "Status", icons::globe()))
                                (lien("/", "Website", icons::external()))
                            }
                        }
                        nav.lg-foot {
                            a href="/privacy?lang=en" { "Privacy" }
                            a href="/terms?lang=en" { "Terms" }
                            a href="/status?lang=en" { "Status" }
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
        "Vue d'ensemble",
        Some(Onglet::Accueil),
        Some(flash),
        html! {
            div.ov {
                div.ov-main {
                    div.ov-cards {
                        (carte("p-lilac", icons::download(), "Launchers ouverts", c.launchers.to_string(), Some("n-launchers"), vec![
                            (icons::check(), format!("{} en partie maintenant", c.en_partie)),
                            (icons::check(), format!("Record : {} en même temps", c.record)),
                            (icons::check(), format!("Version {}", c.version.as_deref().unwrap_or("?"))),
                        ]))
                        (carte("p-pink", icons::user(), "Mistly Client", c.client.to_string(), Some("n-client"), vec![
                            (icons::check(), format!("{} joueurs dans la base", c.joueurs)),
                            (icons::check(), format!("{} actifs sur 24 h", c.actifs_24h)),
                            (icons::check(), format!("{} banni(s)", c.bannis)),
                        ]))
                        (carte("p-peach", icons::sparkles(), "Croissance", format!("+{}", c.nouveaux_24h), None, vec![
                            (icons::check(), "nouveaux joueurs sur 24 h".into()),
                            (icons::check(), format!("+{} sur 7 jours", c.nouveaux_7j)),
                            (icons::check(), format!("Disponibilité : {}", c.dispo.map_or_else(|| "?".into(), |d| format!("{d:.1} %")))),
                        ]))
                    }
                    @if let Some(t) = telemetrie { (telemetrie_bloc(t)) } @else {
                        section.ov-panel { p.empty { "La télémétrie n'est pas disponible." } }
                    }
                }
                aside.ov-side {
                    div.side-sec {
                        div.side-head { h2 { "En jeu maintenant" } span.live { i {} "direct" } }
                        (liste_en_ligne(en_ligne))
                    }
                    div.side-sec {
                        div.side-head { h2 { "Derniers changements" } a.more href="/admin/log" { "Tout voir" } }
                        @if journal.is_empty() {
                            p.empty { "Aucun changement pour l'instant." }
                        } @else {
                            ul.msgs {
                                @for e in journal.iter().take(6) {
                                    li {
                                        span.msg-ic { (icons::notes()) }
                                        div { strong { (e.text) } time { (il_y_a(e.at)) } }
                                    }
                                }
                            }
                        }
                    }
                    div.side-sec {
                        div.side-head { h2 { "Ouvrir un rapport" } }
                        form.quick method="get" action="/admin/reports" {
                            input type="text" name="code" placeholder="MR-ABCD234" aria-label="Code du rapport" autocomplete="off" spellcheck="false";
                            button type="submit" aria-label="Ouvrir" { (icons::arrow_right()) }
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
                p.empty { "Personne n'est en jeu avec Mistly Client." }
            } @else {
                ul.msgs {
                    @for l in en_ligne {
                        li {
                            (tete(l.nom.as_deref(), 34))
                            a href=(format!("/admin/players/{}", l.uuid)) {
                                strong { (nom_ou_uuid(l.nom.as_deref(), &l.uuid)) @if l.note { " " span.tag title="A une note" { "note" } } }
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
    let versions = paires(&t["launcher_versions"]);
    let loaders = paires(&t["games_by_loader"]);
    let crashs = paires(&t["crashes_by_loader"]);
    // Une ligne de la répartition : l'icône dans un rond, le nom, le nombre, une pastille
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
        section.ov-panel.ov-chart {
            div.chart-head {
                div {
                    h2 { "Activité" }
                    div.legend-line {
                        span.k-a { i {} "Installations actives" }
                        span.k-b { i {} "Événements" }
                    }
                }
                div.tabs role="group" aria-label="Période" {
                    button type="button" data-jours="7" { "7 jours" }
                    button type="button" data-jours="14" { "14 jours" }
                    button type="button" data-jours="30" aria-pressed="true" { "30 jours" }
                }
            }
            div.chart-box #chart data-series=(series.to_string()) {
                @if jours.len() < 2 { p.empty { "Pas encore assez de jours pour tracer une courbe." } }
            }
            div.ov-mini {
                div { span { "Installations" } strong { (t["installs"].as_u64().unwrap_or(0)) } }
                div { span { "Parties lancées" } strong { (t["events"]["game_start"].as_u64().unwrap_or(0)) } }
                div { span { "Plantages" } strong { (plantages) } }
                div { span { "Rapports" } strong { (t["reports"].as_u64().unwrap_or(0)) } }
            }
        }
        section.ov-panel {
            div.panel-head { h2 { "Répartition" } span.muted { "30 derniers jours, joueurs qui ont accepté la télémétrie" } }
            div.rep-cols {
                div {
                    h3 { "Versions du launcher" }
                    @if versions.is_empty() { p.empty { "—" } }
                    ul { @for (k, n) in versions.iter().take(4) { (ligne("p-lilac", icons::tag(), &format!("Launcher {k}"), format!("{n} installation(s)"), format!("{n}"))) } }
                }
                div {
                    h3 { "Parties par loader" }
                    @if loaders.is_empty() { p.empty { "—" } }
                    ul {
                        @for (k, n) in loaders.iter().take(4) {
                            @let c = crashs.iter().find(|x| x.0 == *k).map_or(0, |x| x.1);
                            (ligne(if c > 0 { "p-pink" } else { "p-mint" }, icons::package(), k, format!("{n} partie(s) · {c} plantage(s)"), format!("{n}")))
                        }
                    }
                }
            }
        }
    }
}

// ---------- Joueurs ----------

pub fn joueurs(flash: &Flash, q: &str, filtre: &str, lignes: &[Fiche], total: usize, page: usize, pages: usize) -> Markup {
    let filtres = [("all", "Tous"), ("online", "En ligne"), ("granted", "Cosmétiques modifiés"), ("banned", "Bannis"), ("notes", "Avec une note"), ("manual", "Ajoutés à la main")];
    let lien_page = |p: usize| format!("/admin/players?q={}&filter={filtre}&page={p}", encoder(q));
    shell(
        "Joueurs",
        Some(Onglet::Joueurs),
        Some(flash),
        html! {
            (entete("Joueurs", Some("Tous les comptes vus par Mistly Client, et ceux ajoutés à la main.")))
            div.toolbar {
                form.search method="get" action="/admin/players" {
                    (icons::search())
                    input type="search" name="q" value=(q) placeholder="Pseudo ou UUID…" aria-label="Rechercher un joueur";
                    input type="hidden" name="filter" value=(filtre);
                }
                form.add method="post" action="/admin/players/add" {
                    input type="text" name="name" placeholder="Pseudo ou UUID à ajouter" aria-label="Ajouter un joueur" required;
                    button.btn.sm type="submit" { "Ajouter" }
                }
            }
            div.chips {
                @for (code, nom) in filtres {
                    a href=(format!("/admin/players?q={}&filter={code}", encoder(q))) aria-current=[(code == filtre).then_some("true")] { (nom) }
                }
            }
            section.panel.flush {
                @if lignes.is_empty() {
                    p.empty { "Aucun joueur ne correspond." }
                } @else {
                    table.tbl {
                        thead { tr { th { "Joueur" } th { "Cosmétiques" } th { "Première connexion" } th { "Dernier signe" } th.num { "Connexions" } } }
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
                                        @if l.en_ligne { span.tag.on { "en ligne" } }
                                        @if f.banned { span.tag.bad { "banni" } }
                                        @if !f.note.is_empty() { span.tag title=(f.note) { "note" } }
                                    }
                                    td.muted {
                                        @for c in &f.granted { span.tag.give { "+ " (nom_cosmetique(c)) } }
                                        @for c in &f.revoked { span.tag.take { "− " (nom_cosmetique(c)) } }
                                        @if f.granted.is_empty() && f.revoked.is_empty() { "—" }
                                    }
                                    td.muted { @match f.first_seen { Some(t) => (date(t)), None => "ajouté à la main" } }
                                    td.muted { @match f.last_seen { Some(t) => (il_y_a(t)), None => "jamais" } }
                                    td.num.muted { (f.logins) }
                                }
                            }
                        }
                    }
                }
            }
            div.pager {
                span.muted { (total) " joueur(s)" }
                @if pages > 1 {
                    div {
                        @if page > 1 { a.btn.sm href=(lien_page(page - 1)) { "Précédents" } }
                        span.muted { "page " (page) " / " (pages) }
                        @if page < pages { a.btn.sm href=(lien_page(page + 1)) { "Suivants" } }
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
        nom.unwrap_or("Joueur"),
        Some(Onglet::Joueurs),
        Some(flash),
        html! {
            nav.crumbs { a href="/admin/players" { "Joueurs" } span { "/" } span { (nom_ou_uuid(nom, uuid)) } }
            section.profile.panel {
                div.avatar { (tete(nom, 96)) }
                div.who-big {
                    h1 {
                        (nom.unwrap_or("Pseudo inconnu"))
                        @if en_ligne.is_some() { span.tag.on { "en ligne" } }
                        @if f.banned { span.tag.bad { "banni" } }
                    }
                    button.uuid type="button" data-copy=(avec_tirets(uuid)) title="Copier l'UUID" { code { (avec_tirets(uuid)) } (icons::copy()) }
                    div.facts {
                        div { span { "Première connexion" } strong { @match f.first_seen { Some(t) => (date(t)), None => "ajouté à la main" } } }
                        div { span { "Dernier signe" } strong { @match f.last_seen { Some(t) => (il_y_a(t)), None => "jamais" } } }
                        div { span { "Connexions" } strong { (f.logins) } }
                        div { span { "Premier joueur" } strong { @if early_ok { "oui" } @else { "non" } } }
                        @if let Some(j) = en_ligne { div { span { "En jeu depuis" } strong { (duree(j.depuis)) } } }
                    }
                    @if let Some(n) = nom {
                        div.links {
                            a href=(format!("/shop?player={n}")) target="_blank" rel="noopener" { "Aperçu 3D" (icons::external()) }
                            a href=(format!("https://namemc.com/profile/{n}")) target="_blank" rel="noopener" { "NameMC" (icons::external()) }
                        }
                    }
                }
                div.actions {
                    @if en_ligne.is_some() {
                        form method="post" action=(action) { input type="hidden" name="action" value="kick"; button.btn.sm type="submit" { "Expulser du réseau" } }
                    }
                    @if f.banned {
                        form method="post" action=(action) { input type="hidden" name="action" value="unban"; button.btn.sm type="submit" { "Lever le bannissement" } }
                    } @else {
                        form method="post" action=(action) data-confirm="Bannir ce joueur du réseau Mistly ? Il perd ses cosmétiques et ne peut plus se connecter." {
                            input type="hidden" name="action" value="ban"; button.btn.sm.danger type="submit" { "Bannir" }
                        }
                    }
                }
            }

            div.grid2 {
                section.panel {
                    div.panel-head { h2 { (icons::sparkles()) "Cosmétiques" } a.more href="/admin/cosmetics" { "Règles" (icons::arrow_right()) } }
                    @if f.banned { p.note-bad { "Banni : aucun cosmétique n'est relayé tant que le bannissement dure." } }
                    table.tbl.cos {
                        thead { tr { th { "Cosmétique" } th { "État" } th {} } }
                        tbody {
                            @for e in cosmetiques {
                                tr {
                                    td {
                                        strong { (e.c.nom) }
                                        small.muted { (if e.c.place == Place::Tete { "tête" } else { "dos" }) " · règle : " (e.regle.libelle()) }
                                    }
                                    td {
                                        @if e.permis { span.state.yes { "peut le porter" } } @else { span.state.no { "ne peut pas" } }
                                        @if e.donne { span.tag.give { "donné" } }
                                        @if e.retire { span.tag.take { "retiré" } }
                                        @if e.porte { span.tag.on { "le porte" } }
                                    }
                                    td.btns {
                                        @if !e.donne { (bouton("grant", e.c.id, "Donner", "")) }
                                        @if !e.retire { (bouton("revoke", e.c.id, "Retirer", "danger")) }
                                        @if e.donne || e.retire { (bouton("default", e.c.id, "Par défaut", "ghost")) }
                                    }
                                }
                            }
                        }
                    }
                }
                section.panel {
                    div.panel-head { h2 { (icons::notes()) "Note de l'équipe" } }
                    form.note method="post" action=(action) {
                        input type="hidden" name="action" value="note";
                        textarea name="note" rows="6" maxlength=(crate::registre::MAX_NOTE) placeholder="Visible seulement ici." { (f.note) }
                        button.btn.sm type="submit" { "Enregistrer la note" }
                    }
                    div.panel-head.sub { h2 { (icons::refresh()) "Historique" } }
                    @if journal.is_empty() {
                        p.empty { "Aucun changement sur ce joueur." }
                    } @else {
                        ul.log { @for e in journal { li { time title=(date(e.at)) { (il_y_a(e.at)) } span { (e.text) } } } }
                    }
                }
            }

            section.panel.danger-zone {
                div { strong { "Supprimer la fiche" } p.muted { "Oublie ce compte : dates, cosmétiques donnés ou retirés, note. S'il rejoue, une nouvelle fiche est créée (sans la cape early player si la date est passée)." } }
                form method="post" action=(action) data-confirm="Supprimer définitivement la fiche de ce joueur ?" {
                    input type="hidden" name="action" value="forget"; button.btn.sm.danger type="submit" { "Supprimer" }
                }
            }
        },
    )
}

// ---------- Cosmétiques ----------

pub fn cosmetiques(flash: &Flash, lignes: &[RegleCosmetique], early: &Early) -> Markup {
    shell(
        "Cosmétiques",
        Some(Onglet::Cosmetiques),
        Some(flash),
        html! {
            (entete("Cosmétiques", Some("Qui peut porter chaque cosmétique. Un cosmétique donné ou retiré à un joueur, depuis sa fiche, passe avant la règle (sauf « Désactivé »).")))
            section.panel.flush {
                table.tbl {
                    thead { tr { th { "Cosmétique" } th { "Règle" } th.num { "Donnés" } th.num { "Retirés" } th.num { "Portés maintenant" } } }
                    tbody {
                        @for l in lignes {
                            tr {
                                td { strong { (l.c.nom) } small.muted.mono { (l.c.id) } }
                                td {
                                    form.rule method="post" action="/admin/cosmetics" {
                                        input type="hidden" name="cosmetic" value=(l.c.id);
                                        select name="rule" aria-label=(format!("Règle de {}", l.c.nom)) data-autosubmit {
                                            @for r in Regle::TOUTES {
                                                option value=(r.code()) selected[r == l.regle] { (r.libelle()) @if r == l.c.regle_par_defaut { " (par défaut)" } }
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
                            Regle::Everyone => "Tous les joueurs de Mistly Client.",
                            Regle::Early => "Les comptes vus avant la date des premiers joueurs.",
                            Regle::Granted => "Seulement les joueurs à qui il a été donné.",
                            Regle::Off => "Personne, même ceux à qui il a été donné.",
                        }) }
                    }
                }
            }
            section.panel {
                div.panel-head { h2 { (icons::crown()) "Date des premiers joueurs" } }
                p.muted {
                    "Les comptes vus pour la première fois avant cette date ont droit aux cosmétiques réglés sur « Premiers joueurs » (la cape early player). "
                    @match &early.date {
                        Some(d) => { "Date actuelle : " strong { (d) } @if !early.depuis_la_base { " (variable EARLY_BEFORE)" } "." }
                        None => { "Aucune date : tout le monde y a droit." }
                    }
                    @if let Some(n) = early.eligibles { " " (n) " joueur(s) y ont droit." }
                }
                form.inline method="post" action="/admin/cosmetics" {
                    input type="date" name="early_before" value=[early.date.as_deref()] aria-label="Date des premiers joueurs";
                    button.btn.sm type="submit" { "Enregistrer" }
                    @if early.depuis_la_base {
                        button.btn.sm.ghost type="submit" name="early_before" value="" { "Revenir à EARLY_BEFORE" }
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
        "Rapports de logs",
        Some(Onglet::Rapports),
        Some(&flash),
        html! {
            (entete("Rapports de logs", Some("Envoyés par les joueurs avec « Envoyer les logs » dans la console, ou automatiquement après un plantage.")))
            form.search.wide method="get" action="/admin/reports" {
                (icons::search())
                input type="search" name="code" placeholder="Code du rapport, par exemple MR-ABCD234" aria-label="Code du rapport" autofocus[detail.is_none()];
                button.btn.sm type="submit" { "Ouvrir" }
            }
            @if let Some(r) = detail {
                section.panel {
                    div.panel-head {
                        h2.mono { (r["code"].as_str().unwrap_or("?")) }
                        a.more href="/admin/reports" { "Tous les rapports" (icons::arrow_right()) }
                    }
                    div.facts {
                        @for (cle, libelle) in [("reason", "Raison"), ("version", "Launcher"), ("game", "Jeu"), ("os", "Windows"), ("ram_mo", "RAM (Mo)"), ("langue", "Langue"), ("id", "Installation")] {
                            @if let Some(v) = r.get(cle).filter(|v| !v.is_null()) {
                                div { span { (libelle) } strong.mono { (v.as_str().map_or_else(|| v.to_string(), str::to_string)) } }
                            }
                        }
                        @if let Some(t) = r["received"].as_u64() { div { span { "Reçu" } strong { (date(t)) } } }
                    }
                    div.logbar {
                        span.muted { "Journal" }
                        div {
                            label.check { input type="checkbox" data-filter-errors; " Erreurs seulement" }
                            button.btn.sm type="button" data-copy-log { "Copier" }
                        }
                    }
                    pre.logview { (r["log"].as_str().unwrap_or("")) }
                }
            } @else {
                section.panel.flush {
                    @if recents.is_empty() {
                        p.empty { "Aucun rapport." }
                    } @else {
                        table.tbl {
                            thead { tr { th { "Code" } th { "Raison" } th { "Jeu" } th { "Dernière ligne" } th.num { "Reçu" } } }
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
        "Journal",
        Some(Onglet::Journal),
        None,
        html! {
            (entete("Journal", Some("Tout ce qui a été changé depuis le tableau de bord, du plus récent au plus ancien.")))
            section.panel {
                @if log.is_empty() {
                    p.empty { "Le journal est vide." }
                } @else {
                    ul.log.full { @for e in log.iter().rev() { li { time { (date(e.at)) } span { (e.text) } } } }
                }
            }
        },
    )
}

// ---------- Base de données ----------

pub fn base(flash: &Flash, json: &str, joueurs: usize, journal: usize) -> Markup {
    shell(
        "Base de données",
        Some(Onglet::Base),
        Some(flash),
        html! {
            (entete("Base de données", Some("Le contenu de joueurs.json : fiches des joueurs, règles des cosmétiques et date des premiers joueurs.")))
            div.toolbar {
                span.muted { (joueurs) " fiche(s) · " (journal) " entrée(s) au journal (non éditable ici)" }
                a.btn.sm href="/admin/database.json" download { (icons::download()) "Télécharger une sauvegarde" }
            }
            section.panel {
                form.dbedit method="post" action="/admin/database" data-confirm="Remplacer toute la base par ce contenu ? Pense à télécharger une sauvegarde avant." {
                    textarea name="json" spellcheck="false" autocomplete="off" aria-label="Contenu JSON de la base" { (json) }
                    div.dbbar {
                        p.muted {
                            "UUID en clé (32 chiffres hexadécimaux, sans tirets). "
                            "Cosmétiques : " @for (i, c) in crate::registre::CATALOGUE.iter().enumerate() { @if i > 0 { ", " } code { (c.id) } } ". "
                            "Règles : " code { "everyone" } ", " code { "early" } ", " code { "granted" } ", " code { "off" } "."
                        }
                        span #json-state.muted {}
                        button.btn.play.sm type="submit" { "Enregistrer la base" }
                    }
                }
            }
        },
    )
}
