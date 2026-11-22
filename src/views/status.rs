//! Page de statut : l'état des services dont Mistly a besoin, vérifiés par le site lui-même.
//!
//! Une page à part (sans la navigation du site), servie sur `/status` et à la racine du
//! sous-domaine `status.<domaine>` : les liens vers le reste du site sont donc absolus.
//! En haut l'état général sur le paysage du launcher, puis les chiffres clés, une carte par groupe
//! de services (une barre par jour sur 30 jours, ou par minute sur la dernière heure) et le
//! journal des incidents. status.js met la page à jour toutes les minutes.

use super::icons;
use super::layout::{ASSET_VERSION, DISCORD_URL, Page};
use crate::format;
use crate::i18n::Lang;
use crate::status::{Etat, Groupe, Incident, Jour, Ligne, Role, SERVICES};
use maud::{DOCTYPE, Markup, PreEscaped, html};

/// « 99,98 % » selon la langue.
fn pourcent(p: &Page, v: f32) -> String {
    format!("{v:.2}").replace('.', &p.tr.decimal_sep) + " %"
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

/// Le pire état d'une liste (sans mesure : `None`).
fn pire(etats: impl Iterator<Item = Option<Etat>>) -> Option<Etat> {
    let rang = |e: Etat| match e {
        Etat::EnLigne => 0,
        Etat::Lent => 1,
        Etat::HorsLigne => 2,
    };
    etats.flatten().max_by_key(|e| rang(*e))
}

fn classe(e: Option<Etat>) -> &'static str {
    e.map_or("none", |e| e.slug())
}

fn nom_etat<'a>(p: &'a Page, e: Option<Etat>) -> &'a str {
    match e {
        Some(Etat::EnLigne) => &p.tr.st_up,
        Some(Etat::Lent) => &p.tr.st_slow,
        Some(Etat::HorsLigne) => &p.tr.st_down,
        None => &p.tr.st_waiting_short,
    }
}

/// Une rangée de barres : une par jour ou par minute, avec sa bulle (`data-tip`, `data-t` : l'heure,
/// que le navigateur remet à l'heure locale).
fn barres(cases: impl Iterator<Item = (Option<Etat>, String, Option<i64>)>) -> Markup {
    html! {
        @for (e, tip, t) in cases {
            i.(classe(e)) data-tip=(tip) data-t=[t] {}
        }
    }
}

pub fn render(p: &Page, lignes: &[Ligne], incidents: &[Incident], dispo: Option<f32>, age: Option<u64>) -> Markup {
    let tr = p.tr;
    let site = p.site_url;
    let v = ASSET_VERSION.as_str();
    let mesurees: Vec<&Ligne> = lignes.iter().filter(|l| l.actuel.is_some()).collect();
    let hors_ligne = mesurees.iter().filter(|l| l.actuel.map(|m| m.etat) == Some(Etat::HorsLigne)).count();
    let lents = mesurees.iter().filter(|l| l.actuel.map(|m| m.etat) == Some(Etat::Lent)).count();
    let en_ligne = mesurees.len() - hors_ligne - lents;
    let (etat, titre) = if mesurees.is_empty() {
        ("wait", &tr.st_waiting)
    } else if hors_ligne > 0 {
        ("down", &tr.st_some_down)
    } else if lents > 0 {
        ("slow", &tr.st_some_slow)
    } else {
        ("up", &tr.st_all_ok)
    };
    let latences: Vec<u128> = mesurees.iter().filter_map(|l| l.actuel.and_then(|m| m.latence)).map(|d| d.as_millis()).collect();
    let latence = (!latences.is_empty()).then(|| latences.iter().sum::<u128>() / latences.len() as u128);
    let groupes = [
        (Groupe::Misty, &tr.st_group_misty, &tr.st_group_misty_desc),
        (Groupe::Minecraft, &tr.st_group_minecraft, &tr.st_group_minecraft_desc),
        (Groupe::Mods, &tr.st_group_mods, &tr.st_group_mods_desc),
    ];
    let premier_jour = lignes.first().and_then(|l| l.jours.first()).map(|(d, _)| format::jour(tr, *d, false));
    let canonical = super::layout::STATUS_URL.clone().map_or_else(|| format!("{site}/status"), |u| format!("{u}/"));
    let stat = |valeur: String, libelle: &str, id: &str| {
        html! { div.sp-stat { strong id=(id) { (valeur) } span { (libelle) } } }
    };

    html! {
        (DOCTYPE)
        html lang=(p.lang.code()) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (tr.st_title) }
                meta name="description" content=(tr.st_sub);
                meta name="theme-color" content="#09090b";
                meta name="color-scheme" content="dark light";
                link rel="canonical" href=(format!("{canonical}?lang={}", p.lang.code()));
                meta property="og:type" content="website";
                meta property="og:site_name" content="Mistly Status";
                meta property="og:title" content=(tr.st_title);
                meta property="og:description" content=(tr.st_sub);
                meta property="og:image" content=(format!("{site}/assets/og/{}.png?v={v}", p.lang.code()));
                meta name="twitter:card" content="summary_large_image";
                link rel="icon" href="/assets/icone.png";
                link rel="preload" href="/assets/Geist-latin.woff2" as="font" type="font/woff2" crossorigin;
                link rel="stylesheet" href=(format!("/css/base.css?v={v}"));
                link rel="stylesheet" href=(format!("/css/status.css?v={v}"));
                script { (PreEscaped("document.documentElement.classList.add('js');try{var t=localStorage.getItem('theme');if(t==='light'||t==='dark')document.documentElement.dataset.theme=t;if(localStorage.getItem('st-view')==='min')document.documentElement.classList.add('st-min')}catch(e){}")) }
                script src=(format!("/js/status.js?v={v}")) defer {}
            }
            body.sp data-state=(etat) {
                // Le paysage du launcher, en fond du haut de page
                div.sp-sky aria-hidden="true" { (PreEscaped(p.scene)) }

                header.sp-top {
                    div.sp-wrap.sp-top-in {
                        a.sp-brand href=(site) {
                            img src="/assets/icone.png" alt="" width="26" height="26";
                            span { "Mistly" }
                            em { "Status" }
                        }
                        div.sp-actions {
                            details.sp-lang {
                                summary aria-label=(tr.nav_language) title=(tr.lang_name) { (icons::flag(p.lang.code())) }
                                div.sp-lang-menu {
                                    @for l in Lang::ALL {
                                        a href=(format!("?lang={}", l.code())) hreflang=(l.code()) aria-current=[(l == p.lang).then_some("true")] title=(l.tr().lang_name) aria-label=(l.tr().lang_name) { (icons::flag(l.code())) }
                                    }
                                }
                            }
                            button.sp-icon type="button" data-theme-toggle aria-label=(tr.nav_theme) title=(tr.nav_theme) {
                                span.moon { (icons::moon()) } span.sun { (icons::sun()) }
                            }
                            a.sp-site href=(site) { "mistly.cc" (icons::arrow_right()) }
                        }
                    }
                }

                main.sp-wrap {
                    // L'état général
                    section.sp-hero #sp-hero {
                        div.sp-orb.(etat) aria-hidden="true" {
                            i {}
                            @match etat {
                                "up" => (icons::check()),
                                "wait" => (icons::refresh()),
                                _ => (icons::alert()),
                            }
                        }
                        h1 { (titre) }
                        p.sp-meta {
                            span.sp-live { i {} (tr.st_live) }
                            @if let Some(s) = age { span #sp-age data-age=(s) { (tr.st_checked.replace("{0}", &s.to_string())) } }
                        }
                        div.sp-stats {
                            (stat(dispo.map_or_else(|| "—".into(), |d| pourcent(p, d)), &tr.st_uptime_30, "sp-uptime"))
                            (stat(format!("{en_ligne}/{}", lignes.len()), &tr.st_online, "sp-online"))
                            (stat(latence.map_or_else(|| "—".into(), |ms| format!("{ms} ms")), &tr.st_latency, "sp-latency"))
                            (stat(incidents.len().to_string(), &tr.st_incidents, "sp-incidents"))
                        }
                    }

                    // Les services
                    div.sp-bar {
                        h2 { "Services" }
                        div.sp-seg role="group" {
                            button type="button" data-view="days" { (tr.st_view_days) }
                            button type="button" data-view="min" { (tr.st_view_minutes) }
                        }
                    }
                    div #sp-groups {
                        @for (groupe, nom, desc) in groupes {
                            @let services: Vec<&Ligne> = lignes.iter().filter(|l| SERVICES[l.index].groupe == groupe).collect();
                            @let e = pire(services.iter().map(|l| l.actuel.map(|m| m.etat)));
                            section.sp-group {
                                header.sp-group-head {
                                    div {
                                        h3 { (nom) span.n { (services.len()) } }
                                        p { (desc) }
                                    }
                                    span.sp-pill.(classe(e)) { i {} (nom_etat(p, e)) }
                                }
                                @for l in services { (ligne(p, l)) }
                                div.sp-axis aria-hidden="true" {
                                    span {}
                                    div { span.d { (premier_jour.as_deref().unwrap_or("")) } span.m { (tr.st_hour_ago) } span { (tr.st_now) } }
                                    span {}
                                }
                            }
                        }
                    }

                    // Le journal des incidents
                    section.sp-incidents #sp-log {
                        div.sp-bar { h2 { (tr.st_incidents_t) } span.sp-sub { (tr.st_incidents) } }
                        @if incidents.is_empty() {
                            div.sp-calm { (icons::check()) span { (tr.st_no_incidents) } }
                        } @else {
                            ol.sp-timeline {
                                @for i in incidents { (incident(p, i)) }
                            }
                        }
                    }

                    footer.sp-foot {
                        div.sp-legend {
                            span { i.up {} (tr.st_up) }
                            span { i.slow {} (tr.st_slow) }
                            span { i.down {} (tr.st_down) }
                            span { i.none {} (tr.st_no_data) }
                        }
                        p { (tr.st_how) " " a href="/status.json" { "status.json" } }
                        div.sp-links {
                            a href=(site) { img src="/assets/icone.png" alt="" width="16" height="16"; "mistly.cc" }
                            a href=(format!("{site}/docs")) { (tr.nav_docs) }
                            a href=(DISCORD_URL) target="_blank" rel="noopener" { (icons::discord()) "Discord" }
                        }
                    }
                }
                div.sp-tip #sp-tip role="tooltip" hidden {}
            }
        }
    }
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
    let e = l.actuel.map(|m| m.etat);
    let minutes = l.minutes.iter().map(|(debut, m)| {
        let quand = heure(*debut);
        match m {
            Some(m) => {
                let ms = m.ms.map(|ms| format!(" · {ms} ms")).unwrap_or_default();
                (Some(m.etat), format!("{quand} · {}{ms}", nom_etat(p, Some(m.etat))), Some(*debut))
            }
            None => (None, format!("{quand} · {}", tr.st_no_data), Some(*debut)),
        }
    });
    let jours = l.jours.iter().map(|(d, j)| {
        let e = j.as_ref().and_then(etat_jour);
        let texte = if e.is_some() { nom_etat(p, e) } else { tr.st_no_data.as_str() };
        let dispo = j.as_ref().filter(|j| j.total() > 0).map(|j| format!(" · {}", pourcent(p, j.dispo()))).unwrap_or_default();
        (e, format!("{} · {texte}{dispo}", format::jour(tr, *d, true)), None)
    });
    html! {
        div.sp-row {
            div.sp-name {
                span.sp-dot.(classe(e)) title=(nom_etat(p, e)) {}
                div {
                    strong { (s.nom) }
                    span { (role) }
                }
            }
            div.sp-bars.days { (barres(jours)) }
            div.sp-bars.min { (barres(minutes)) }
            div.sp-nums {
                strong { @if let Some(d) = l.dispo { (pourcent(p, d)) } @else { "—" } }
                span { @if let Some(ms) = l.actuel.and_then(|m| m.latence) { (ms.as_millis()) " ms" } @else { "—" } }
            }
        }
    }
}

fn incident(p: &Page, i: &Incident) -> Markup {
    let tr = p.tr;
    let (classe, titre) = if i.panne { ("down", &tr.st_inc_down) } else { ("slow", &tr.st_inc_slow) };
    html! {
        li.(classe) {
            span.sp-when { (format::moment(tr, i.debut)) }
            div.sp-inc {
                strong { (i.service) " · " span { (titre) } }
                @match i.fin {
                    Some(f) => span.sp-pill.up { i {} (tr.st_resolved) " · " (duree(f - i.debut)) },
                    None => span.sp-pill.down { i {} (tr.st_ongoing) },
                }
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
