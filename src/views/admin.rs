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
    let groupes: [(&str, &[Onglet]); 2] = [
        ("Pilotage", &[Onglet::Accueil, Onglet::Joueurs, Onglet::Cosmetiques]),
        ("Données", &[Onglet::Rapports, Onglet::Journal, Onglet::Base]),
    ];
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
                div.adm-glow aria-hidden="true" {}
                aside.side {
                    a.brand href="/admin" {
                        img src="/assets/icone.png" alt="" width="30" height="30";
                        span { strong { "Mistly" } em { "Admin" } }
                    }
                    nav.menu aria-label="Tableau de bord" {
                        @for (groupe, onglets) in groupes {
                            span.menu-t { (groupe) }
                            @for o in onglets.iter() {
                                a href=(o.lien()) aria-current=[(onglet == Some(*o)).then_some("page")] {
                                    span.mi { (o.icone()) }
                                    span { (o.titre()) }
                                }
                            }
                        }
                    }
                    div.side-foot {
                        a.side-link href="/" target="_blank" rel="noopener" { (icons::globe()) span { "Voir le site" } (icons::external()) }
                        div.me {
                            span.me-av { img src="/assets/icone.png" alt="" width="20" height="20"; }
                            div {
                                strong { "Équipe Mistly" }
                                span { i.dot {} "session " code #uptime { "00:00:00" } }
                            }
                            form method="post" action="/admin/logout" {
                                button.me-out type="submit" aria-label="Se déconnecter" title="Se déconnecter" { (icons::arrow_right()) }
                            }
                        }
                    }
                }
                main.content {
                    header.topbar {
                        nav.tb-crumbs aria-label="Fil d'Ariane" {
                            span { "Admin" }
                            (icons::chevron())
                            strong { @if let Some(o) = onglet { (o.titre()) } @else { (titre) } }
                        }
                        form.tb-search method="get" action="/admin/players" role="search" {
                            (icons::search())
                            input #tb-q type="search" name="q" placeholder="Rechercher un joueur…" aria-label="Rechercher un joueur" autocomplete="off";
                            kbd { "/" }
                        }
                        span.tb-live { i {} "En ligne" }
                    }
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

fn entete(titre: &str, sous: Option<&str>) -> Markup {
    html! {
        header.page-head {
            h1 { (titre) }
            @if let Some(s) = sous { p { (s) } }
        }
    }
}

/// Une courbe pleine (SVG) : les valeurs de gauche à droite, dans un cadre de 600 × 140.
fn aire(valeurs: &[u64]) -> Markup {
    let (l, h) = (600.0_f64, 140.0_f64);
    let max = valeurs.iter().copied().max().unwrap_or(0).max(1) as f64;
    let n = valeurs.len().max(2) as f64 - 1.0;
    let points: Vec<(f64, f64)> = valeurs.iter().enumerate().map(|(i, v)| (i as f64 / n * l, h - 8.0 - (*v as f64 / max) * (h - 22.0))).collect();
    let ligne: String = points.iter().enumerate().map(|(i, (x, y))| format!("{}{x:.1},{y:.1}", if i == 0 { "M" } else { " L" })).collect();
    let plein = format!("{ligne} L{l},{h} L0,{h} Z");
    html! {
        svg.area viewBox=(format!("0 0 {l} {h}")) preserveAspectRatio="none" aria-hidden="true" {
            defs {
                linearGradient #aire-g x1="0" y1="0" x2="0" y2="1" {
                    stop offset="0" stop-color="#8eaaf0" stop-opacity=".45" {}
                    stop offset="1" stop-color="#8eaaf0" stop-opacity="0" {}
                }
            }
            @for k in 1..4 { line x1="0" x2=(l) y1=(k as f64 * h / 4.0) y2=(k as f64 * h / 4.0) {} }
            path.fill d=(plein) {}
            path.stroke d=(ligne) {}
            @if let Some((x, y)) = points.last() { circle cx=(format!("{x:.1}")) cy=(format!("{y:.1}")) r="4" {} }
        }
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
        html lang="fr" data-theme="dark" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="robots" content="noindex, nofollow";
                meta name="color-scheme" content="dark";
                title { "Connexion · Mistly Admin" }
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
                            p.lg-sub { "Tableau de bord de l'équipe." }
                            form.lg-form #login method="post" action="/admin/login" data-error=[erreur.map(|_| "1")] {
                                label.sr for="pw" { "Mot de passe" }
                                div.lg-field {
                                    (icons::lock())
                                    input #pw type="password" name="password" placeholder="Mot de passe" autocomplete="current-password" required autofocus spellcheck="false";
                                    button.lg-eye type="button" aria-label="Afficher le mot de passe" aria-pressed="false" { "voir" }
                                }
                                @if let Some(e) = erreur { p.lg-err role="alert" { (icons::alert()) (e) } }
                                button.lg-btn.solid type="submit" {
                                    span.t { "Se connecter" }
                                    (icons::arrow_right())
                                }
                            }
                            a.lg-btn href="https://github.com/Mistly-MC/Mistly-Web" target="_blank" rel="noopener" {
                                span.t { "Voir le code" }
                                span.lg-gh { (icons::github()) "GitHub" }
                            }
                            div.lg-social {
                                (lien(super::layout::DISCORD_URL, "Discord", icons::discord()))
                                (lien(super::layout::GITHUB_ORG_URL, "GitHub", icons::github()))
                                (lien("/status", "Statut", icons::globe()))
                                (lien("/", "Site", icons::external()))
                            }
                        }
                        nav.lg-foot {
                            a href="/privacy" { "Confidentialité" }
                            a href="/terms" { "Conditions" }
                            a href="/status" { "Statut" }
                        }
                    }
                    div.lg-art aria-hidden="true" {
                        div.lg-sky { (PreEscaped(scene)) }
                        canvas.adm-bg {}
                        div.lg-badge { img src="/assets/icone.png" alt="" width="18" height="18"; span { "Accès réservé à l'équipe" } }
                        p.lg-quote { "Léger comme la brume." }
                    }
                }
            }
        }
    }
}

// ---------- Vue d'ensemble ----------

pub fn accueil(flash: &Flash, c: &Chiffres, en_ligne: &[Ligne], telemetrie: Option<&Value>, journal: &[Entree]) -> Markup {
    let carte = |valeur: String, libelle: &str, detail: Option<String>, id: Option<&str>, icone: Markup, teinte: &str| {
        html! {
            div.stat.(teinte) {
                div.stat-top {
                    span.stat-ic { (icone) }
                    span.stat-l { (libelle) }
                }
                strong id=[id] { (valeur) }
                @if let Some(d) = detail { small { (d) } }
                // la courbe des dernières minutes, dessinée par admin.js
                @if let Some(i) = id { svg.mini data-spark=(i) viewBox="0 0 120 32" preserveAspectRatio="none" aria-hidden="true" {} }
            }
        }
    };
    shell(
        "Vue d'ensemble",
        Some(Onglet::Accueil),
        Some(flash),
        html! {
            (entete("Vue d'ensemble", Some("Les chiffres en direct se mettent à jour toutes les 15 secondes.")))
            div.stats {
                (carte(c.launchers.to_string(), "Launchers ouverts", Some(format!("{} en partie · record {}", c.en_partie, c.record)), Some("n-launchers"), icons::download(), "t-green"))
                (carte(c.client.to_string(), "En jeu avec Mistly Client", Some("relayés par le site".into()), Some("n-client"), icons::user(), "t-blue"))
                (carte(c.joueurs.to_string(), "Joueurs dans la base", Some(format!("{} actifs sur 24 h", c.actifs_24h)), None, icons::layers(), "t-violet"))
                (carte(format!("+{}", c.nouveaux_24h), "Nouveaux sur 24 h", Some(format!("+{} sur 7 jours", c.nouveaux_7j)), None, icons::sparkles(), "t-pink"))
                (carte(c.version.clone().unwrap_or_else(|| "?".into()), "Dernière version", Some("du launcher".into()), None, icons::tag(), "t-amber"))
                (carte(c.dispo.map_or_else(|| "?".into(), |d| format!("{d:.1} %")), "Disponibilité (30 j)", Some(format!("{} banni(s)", c.bannis)), None, icons::globe(), "t-cyan"))
            }

            div.grid2 {
                section.panel {
                    div.panel-head { h2 { (icons::user()) "En jeu maintenant" } span.live { i {} "direct" } }
                    (table_en_ligne(en_ligne))
                }
                section.panel {
                    div.panel-head { h2 { (icons::notes()) "Derniers changements" } a.more href="/admin/log" { "Tout le journal" (icons::arrow_right()) } }
                    @if journal.is_empty() {
                        p.empty { "Aucun changement pour l'instant." }
                    } @else {
                        ul.log { @for e in journal { li { time { (il_y_a(e.at)) } span { (e.text) } } } }
                    }
                }
            }

            @if let Some(t) = telemetrie {
                section.panel {
                    div.panel-head { h2 { (icons::bolt()) "Télémétrie (30 jours)" } span.muted { "seulement les joueurs qui l'ont acceptée" } }
                    (telemetrie_bloc(t))
                }
            }
        },
    )
}

fn table_en_ligne(en_ligne: &[Ligne]) -> Markup {
    html! {
        div #online {
            @if en_ligne.is_empty() {
                p.empty { "Personne n'est en jeu avec Mistly Client." }
            } @else {
                table.tbl {
                    thead { tr { th { "Joueur" } th { "Porte" } th.num { "Depuis" } } }
                    tbody {
                        @for l in en_ligne {
                            tr {
                                td {
                                    a.who href=(format!("/admin/players/{}", l.uuid)) {
                                        (tete(l.nom.as_deref(), 24))
                                        span { (nom_ou_uuid(l.nom.as_deref(), &l.uuid)) }
                                        @if l.note { span.tag title="A une note" { "note" } }
                                    }
                                }
                                td.muted {
                                    @let porte: Vec<&str> = [l.cosmetique, l.chapeau].into_iter().filter(|c| !c.is_empty()).map(nom_cosmetique).collect();
                                    @if porte.is_empty() { "—" } @else { (porte.join(" + ")) }
                                }
                                td.num.muted { (duree(l.depuis)) }
                            }
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
        l.sort_by(|a, b| b.1.cmp(&a.1));
        l
    };
    let jours: Vec<(String, u64)> = t["per_day"].as_object().map(|o| o.iter().map(|(k, v)| (k.clone(), v["installs"].as_u64().unwrap_or(0))).collect()).unwrap_or_default();
    let max = jours.iter().map(|j| j.1).max().unwrap_or(0).max(1);
    let barres = |titre: &str, l: Vec<(String, u64)>| {
        let max = l.first().map(|x| x.1).unwrap_or(0).max(1);
        html! {
            div.bars {
                h3 { (titre) }
                @if l.is_empty() { p.empty { "—" } }
                @for (k, n) in l.iter().take(8) {
                    div.bar { span.k title=(k) { (k) } span.track { i style=(format!("width:{:.1}%", (*n as f64 / max as f64 * 100.0).max(2.0))) {} } span.n { (n) } }
                }
            }
        }
    };
    html! {
        div.tele-top {
            div { span.tt-ic.t-blue { (icons::download()) } strong { (t["installs"].as_u64().unwrap_or(0)) } span { "installations actives" } }
            div { span.tt-ic.t-green { (icons::bolt()) } strong { (t["events"]["game_start"].as_u64().unwrap_or(0)) } span { "parties lancées" } }
            div { span.tt-ic.t-red { (icons::alert()) } strong { (paires(&t["crashes_by_loader"]).iter().map(|x| x.1).sum::<u64>()) } span { "plantages" } }
            div { span.tt-ic.t-amber { (icons::notes()) } strong { (t["reports"].as_u64().unwrap_or(0)) } span { "rapports de logs" } }
        }
        @if jours.len() >= 2 {
            div.chart {
                div.chart-head {
                    span { "Installations actives par jour" }
                    span.muted { "max " (max) }
                }
                (aire(&jours.iter().map(|j| j.1).collect::<Vec<_>>()))
                div.chart-axis { span { (jours.first().map_or("", |j| j.0.as_str())) } span { (jours.last().map_or("", |j| j.0.as_str())) } }
            }
        }
        div.grid3 {
            (barres("Versions du launcher", paires(&t["launcher_versions"])))
            (barres("Parties par loader", paires(&t["games_by_loader"])))
            (barres("Plantages par loader", paires(&t["crashes_by_loader"])))
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
