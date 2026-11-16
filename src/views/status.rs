//! Page de statut : l'état des services dont Mistly a besoin, vérifiés par le site lui-même.
//!
//! Tout est dessiné en caractères, comme une sortie de terminal : un résumé encadré en ASCII, puis
//! une ligne par service avec la dernière heure (un caractère par minute) et les 30 derniers jours
//! (un caractère par jour) : `#` en ligne, `~` lent, `x` hors ligne, `.` sans mesure.

use super::layout::{self, Head, Page};
use crate::format;
use crate::status::{Etat, Groupe, Incident, Jour, Ligne, Role, SERVICES};
use maud::{Markup, html};

/// « 99,98 % » selon la langue.
fn pourcent(p: &Page, v: f32) -> String {
    format!("{v:.2}").replace('.', &p.tr.decimal_sep) + " %"
}

/// Le caractère d'une mesure : `#` en ligne, `~` lent, `x` hors ligne.
fn car(e: Etat) -> char {
    match e {
        Etat::EnLigne => '#',
        Etat::Lent => '~',
        Etat::HorsLigne => 'x',
    }
}

/// L'état d'un jour d'après ses compteurs : une panne qui a duré compte comme hors ligne.
fn etat_jour(j: &Jour) -> Option<Etat> {
    let total = j.up + j.slow + j.down;
    if total == 0 {
        None
    } else if j.down * 20 > total {
        Some(Etat::HorsLigne)
    } else if (j.slow + j.down) * 10 > total {
        Some(Etat::Lent)
    } else {
        Some(Etat::EnLigne)
    }
}

/// Une barre de caractères : les suites d'un même état sont regroupées dans un `<span>` coloré.
/// Une barre de caractères, un caractère coloré par minute ou par jour (`data-t` : l'heure, que le
/// navigateur remet à l'heure locale dans la bulle).
fn barre(cases: impl Iterator<Item = (Option<Etat>, String, Option<i64>)>) -> Markup {
    let cases: Vec<_> = cases.collect();
    html! {
        @for (e, titre, t) in &cases {
            @let classe = e.map_or("none", |e| e.slug());
            span.(classe) title=(titre) data-t=[t] { (e.map_or('.', car)) }
        }
    }
}

pub fn render(p: &Page, lignes: &[Ligne], incidents: &[Incident], dispo: Option<f32>, age: Option<u64>) -> Markup {
    let tr = p.tr;
    let mesurees: Vec<&Ligne> = lignes.iter().filter(|l| l.actuel.is_some()).collect();
    let hors_ligne = mesurees.iter().filter(|l| l.actuel.map(|m| m.etat) == Some(Etat::HorsLigne)).count();
    let lents = mesurees.iter().filter(|l| l.actuel.map(|m| m.etat) == Some(Etat::Lent)).count();
    let en_ligne = mesurees.len() - hors_ligne - lents;
    let (classe, etiquette, titre) = if mesurees.is_empty() {
        ("wait", " .. ", &tr.st_waiting)
    } else if hors_ligne > 0 {
        ("down", "DOWN", &tr.st_some_down)
    } else if lents > 0 {
        ("slow", "SLOW", &tr.st_some_slow)
    } else {
        ("up", " OK ", &tr.st_all_ok)
    };
    let groupes = [
        (Groupe::Misty, &tr.st_group_misty, &tr.st_group_misty_desc),
        (Groupe::Minecraft, &tr.st_group_minecraft, &tr.st_group_minecraft_desc),
        (Groupe::Mods, &tr.st_group_mods, &tr.st_group_mods_desc),
    ];
    let main = html! {
        section.hero.st-hero {
            div.col.st-head {
                span.label { (tr.st_label) }
                h1 { (tr.st_h1_1) " " span.soft { (tr.st_h1_2) } }
                p.sub { (tr.st_sub) }
                // Le résumé, comme la sortie d'une commande
                div.term.(classe) {
                    div.term-line.cmd { span.prompt { "$" } " mistly status --all" }
                    div.term-line.big {
                        span.tag.(classe) { "[" (etiquette) "]" }
                        strong { (titre) }
                    }
                    div.term-line.kv {
                        span { span.k { "services" } " " (en_ligne) "/" (lignes.len()) }
                        span { span.k { "uptime(30d)" } " " @if let Some(d) = dispo { (pourcent(p, d)) } @else { "—" } }
                        @if let Some(s) = age { span.k { (tr.st_checked.replace("{0}", &s.to_string())) } }
                    }
                    div.term-line.legend {
                        span.up { "#" } " " (tr.st_up) "   "
                        span.slow { "~" } " " (tr.st_slow) "   "
                        span.down { "x" } " " (tr.st_down) "   "
                        span.none { "." } " " (tr.st_no_data)
                    }
                }
            }
        }
        @for (groupe, nom, desc) in groupes {
            @let services: Vec<&Ligne> = lignes.iter().filter(|l| SERVICES[l.index].groupe == groupe).collect();
            section.blk.st-group {
                div.col {
                    h2.blk-title { (nom) sup { "(" (services.len()) ")" } }
                    p.blk-lead { (desc) }
                    div.st-list {
                        @for l in services { (ligne(p, l)) }
                    }
                }
            }
        }
        section.blk.st-group {
            div.col {
                h2.blk-title { (tr.st_incidents) sup { "(" (incidents.len()) ")" } }
                div.st-log {
                    @if incidents.is_empty() {
                        p.log-line { span.tag.up { "[ OK ]" } " " (tr.st_no_incidents) }
                    } @else {
                        @for i in incidents { (incident(p, i)) }
                    }
                }
                p.st-how { (tr.st_how) " " a href="/status.json" { "status.json" } }
            }
        }
    };
    layout::render(
        p,
        Head {
            title: &tr.st_title,
            description: &tr.st_sub,
            jsonld: None,
            css: &["home", "patch-notes", "status"],
            scripts: &[],
            body_class: "status-page",
        },
        main,
    )
}

fn ligne(p: &Page, l: &Ligne) -> Markup {
    let tr = p.tr;
    let s = &SERVICES[l.index];
    let role = match s.role {
        Role::Telechargement => &tr.st_role_download,
        Role::Notes => &tr.st_role_notes,
        Role::Horizon => &tr.st_role_horizon,
        Role::Comptes => &tr.st_role_accounts,
        Role::Connexion => &tr.st_role_login,
        Role::Skins => &tr.st_role_skins,
        Role::Versions => &tr.st_role_versions,
        Role::Ressources => &tr.st_role_assets,
        Role::Bibliotheques => &tr.st_role_libraries,
        Role::Mods => &tr.st_role_mods,
        Role::Loaders => &tr.st_role_loaders,
        Role::Client => &tr.st_role_client,
    };
    let (classe, etiquette, etat) = match l.actuel.map(|m| m.etat) {
        Some(Etat::EnLigne) => ("up", " OK ", &tr.st_up),
        Some(Etat::Lent) => ("slow", "SLOW", &tr.st_slow),
        Some(Etat::HorsLigne) => ("down", "DOWN", &tr.st_down),
        None => ("wait", " .. ", &tr.st_waiting_short),
    };
    let minutes = l.minutes.iter().map(|(debut, m)| {
        let quand = heure(*debut);
        match m {
            Some(m) => {
                let ms = m.ms.map(|ms| format!(" · {ms} ms")).unwrap_or_default();
                (Some(m.etat), format!("{quand} · {}{ms}", nom_etat(p, m.etat)), Some(*debut))
            }
            None => (None, format!("{quand} · {}", tr.st_no_data), Some(*debut)),
        }
    });
    let jours = l.jours.iter().map(|(d, j)| {
        let e = j.as_ref().and_then(etat_jour);
        let texte = e.map_or(tr.st_no_data.as_str(), |e| nom_etat(p, e));
        (e, format!("{} · {texte}", format::jour(tr, *d, true)), None)
    });
    html! {
        div.st-row {
            div.st-name {
                strong { (s.nom) }
                span { (role) }
            }
            div.st-bars {
                div.bar-line { span.k { "60m" } code.bar { (barre(minutes)) } }
                div.bar-line { span.k { "30d" } code.bar.days { (barre(jours)) } }
            }
            div.st-right {
                span.tag.(classe) title=(etat) { "[" (etiquette) "]" }
                span.st-num { @if let Some(d) = l.dispo { (pourcent(p, d)) } @else { "—" } }
                span.st-ms { @if let Some(ms) = l.actuel.and_then(|m| m.latence) { (ms.as_millis()) " ms" } @else { "—" } }
            }
        }
    }
}

fn nom_etat<'a>(p: &'a Page, e: Etat) -> &'a str {
    match e {
        Etat::EnLigne => &p.tr.st_up,
        Etat::Lent => &p.tr.st_slow,
        Etat::HorsLigne => &p.tr.st_down,
    }
}

fn incident(p: &Page, i: &Incident) -> Markup {
    let tr = p.tr;
    let (classe, etiquette, titre) = if i.panne { ("down", "DOWN", &tr.st_inc_down) } else { ("slow", "SLOW", &tr.st_inc_slow) };
    html! {
        p.log-line {
            span.when { (format::moment(tr, i.debut)) }
            " " span.tag.(classe) { "[" (etiquette) "]" }
            " " strong { (i.service) } " " span.muted { (titre) }
            " " @match i.fin {
                Some(f) => span.tag.up { "→ " (tr.st_resolved) " · " (duree(f - i.debut)) },
                None => span.tag.down { "→ " (tr.st_ongoing) },
            }
        }
    }
}

/// « 14:03 UTC » (le navigateur la remplace par l'heure locale).
fn heure(unix: i64) -> String {
    let t = time::OffsetDateTime::from_unix_timestamp(unix).unwrap_or(time::OffsetDateTime::UNIX_EPOCH);
    format!("{:02}:{:02} UTC", t.hour(), t.minute())
}

/// « 3 min », « 2 h 05 ».
fn duree(secondes: i64) -> String {
    let min = (secondes.max(60) + 30) / 60;
    if min < 60 { format!("{min} min") } else { format!("{} h {:02}", min / 60, min % 60) }
}
