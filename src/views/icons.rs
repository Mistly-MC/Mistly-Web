//! Icônes SVG en ligne (traits 24×24, couleur héritée du texte).

use maud::{Markup, PreEscaped};

fn icon(paths: &str) -> Markup {
    PreEscaped(format!(
        r#"<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">{paths}</svg>"#
    ))
}

pub fn download() -> Markup {
    icon(r#"<path d="M12 3v12m0 0l-5-5m5 5l5-5M4 20h16"/>"#)
}
pub fn archive() -> Markup {
    icon(r#"<path d="M21 8v13H3V8M1 3h22v5H1zM10 12h4"/>"#)
}
pub fn notes() -> Markup {
    icon(
        r#"<path d="M14 3H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><path d="M14 3v6h6M8 13h8M8 17h5"/>"#,
    )
}
pub fn arrow_right() -> Markup {
    icon(r#"<path d="M5 12h14M13 6l6 6-6 6"/>"#)
}
pub fn arrow_left() -> Markup {
    icon(r#"<path d="M19 12H5M11 6l-6 6 6 6"/>"#)
}
pub fn tag() -> Markup {
    icon(r#"<path d="M3 12V4a1 1 0 0 1 1-1h8l9 9-9 9z"/><circle cx="8" cy="8" r="1.5"/>"#)
}
pub fn apple() -> Markup {
    PreEscaped(r#"<svg class="ic" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M16.4 12.6c0-2.4 2-3.6 2.1-3.7-1.1-1.7-2.9-1.9-3.5-1.9-1.5-.2-2.9.9-3.7.9-.8 0-1.9-.9-3.2-.8-1.6 0-3.2 1-4 2.5-1.7 3-.4 7.4 1.2 9.8.8 1.2 1.8 2.5 3 2.4 1.2 0 1.7-.8 3.1-.8 1.5 0 1.9.8 3.2.8 1.3 0 2.2-1.2 3-2.4.9-1.4 1.3-2.7 1.3-2.8 0 0-2.5-1-2.5-4zM14 5.4c.7-.8 1.1-1.9 1-3-1 0-2.1.7-2.8 1.5-.6.7-1.2 1.8-1 2.9 1.1.1 2.1-.6 2.8-1.4z"/></svg>"#.to_string())
}
pub fn linux() -> Markup {
    // Tux, en silhouette
    PreEscaped(r##"<svg class="ic" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 2c-2.3 0-3.6 1.9-3.6 4.4 0 1.5.3 2.4-.3 3.6-.8 1.5-2.6 3.3-2.6 6 0 1.4.5 2.4 1.3 3-.9.2-1.8.7-1.6 1.5.3 1 2.4.9 4 .3.6.2 1.8.3 2.8.3s2.2-.1 2.8-.3c1.6.6 3.7.7 4-.3.2-.8-.7-1.3-1.6-1.5.8-.6 1.3-1.6 1.3-3 0-2.7-1.8-4.5-2.6-6-.6-1.2-.3-2.1-.3-3.6C15.6 3.9 14.3 2 12 2z"/><ellipse cx="12" cy="15.2" rx="3" ry="4" fill="#09090b" fill-opacity=".45"/><circle cx="10.7" cy="6.4" r=".7" fill="#09090b"/><circle cx="13.3" cy="6.4" r=".7" fill="#09090b"/><path d="M10.9 8.2h2.2L12 9.4z" fill="#f2b33d"/></svg>"##.to_string())
}
pub fn sun() -> Markup {
    icon(r#"<circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/>"#)
}
pub fn moon() -> Markup {
    icon(r#"<path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z"/>"#)
}
pub fn external() -> Markup {
    icon(r#"<path d="M7 17L17 7M8 7h9v9"/>"#)
}
pub fn arrow_up() -> Markup {
    icon(r#"<path d="M12 19V5M5 12l7-7 7 7"/>"#)
}
pub fn bag() -> Markup {
    icon(r#"<path d="M5 8h14l-1 12H6z"/><path d="M9 8V6a3 3 0 0 1 6 0v2"/>"#)
}
pub fn cape() -> Markup {
    icon(r#"<path d="M8 3h8l1 3-1 15-4-2-4 2-1-15z"/><path d="M8 3l4 4 4-4"/>"#)
}
pub fn crown() -> Markup {
    icon(r#"<path d="M3 8l4 4 5-7 5 7 4-4-2 11H5z"/>"#)
}
pub fn wings() -> Markup {
    icon(
        r#"<path d="M12 8c-2-4-7-5-10-3 1 5 4 8 10 9"/><path d="M12 8c2-4 7-5 10-3-1 5-4 8-10 9"/><path d="M12 8v12"/>"#,
    )
}
pub fn sparkles() -> Markup {
    icon(
        r#"<path d="M12 3l1.8 5.2L19 10l-5.2 1.8L12 17l-1.8-5.2L5 10l5.2-1.8z"/><path d="M19 15l.8 2.2L22 18l-2.2.8L19 21l-.8-2.2L16 18l2.2-.8z"/>"#,
    )
}
pub fn shirt() -> Markup {
    icon(r#"<path d="M8 3l4 2 4-2 5 4-3 3-2-1v12H8V9l-2 1-3-3z"/>"#)
}
pub fn paw() -> Markup {
    icon(
        r#"<circle cx="6" cy="10" r="2"/><circle cx="10" cy="5.5" r="2"/><circle cx="14" cy="5.5" r="2"/><circle cx="18" cy="10" r="2"/><path d="M8 17c0-3 2-5 4-5s4 2 4 5a2.5 2.5 0 0 1-4 2 2.5 2.5 0 0 1-4-2z"/>"#,
    )
}
pub fn smile() -> Markup {
    icon(r#"<circle cx="12" cy="12" r="9"/><path d="M8 14s1.5 2 4 2 4-2 4-2M9 9h.01M15 9h.01"/>"#)
}
pub fn copy() -> Markup {
    icon(r#"<rect x="9" y="9" width="12" height="12" rx="2"/><path d="M5 15V5a2 2 0 0 1 2-2h10"/>"#)
}
pub fn alert() -> Markup {
    icon(r#"<path d="M12 3l10 18H2z"/><path d="M12 10v5M12 18h.01"/>"#)
}
/// Rotation automatique : orbite autour du joueur.
pub fn orbit() -> Markup {
    icon(r#"<ellipse cx="12" cy="12" rx="10" ry="4"/><path d="M12 8v8"/><path d="M19 7l2 1-1 2"/>"#)
}
pub fn menu() -> Markup {
    icon(r#"<path d="M4 6h16M4 12h16M4 18h16"/>"#)
}
pub fn search() -> Markup {
    icon(r#"<circle cx="11" cy="11" r="7"/><path d="M20 20l-3.5-3.5"/>"#)
}
pub fn chevron() -> Markup {
    icon(r#"<path d="M6 9l6 6 6-6"/>"#)
}
pub fn check() -> Markup {
    icon(r#"<path d="M5 12l5 5 9-10"/>"#)
}
pub fn globe() -> Markup {
    icon(
        r#"<circle cx="12" cy="12" r="9"/><path d="M3 12h18M12 3a14 14 0 0 1 0 18M12 3a14 14 0 0 0 0 18"/>"#,
    )
}
pub fn bolt() -> Markup {
    icon(r#"<path d="M13 2L4 14h7l-1 8 9-12h-7z"/>"#)
}
pub fn grid() -> Markup {
    icon(
        r#"<rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/>"#,
    )
}
pub fn user() -> Markup {
    icon(r#"<circle cx="12" cy="8" r="4"/><path d="M4 21a8 8 0 0 1 16 0"/>"#)
}
pub fn refresh() -> Markup {
    icon(r#"<path d="M21 12a9 9 0 1 1-3-6.7L21 8"/><path d="M21 3v5h-5"/>"#)
}
pub fn layers() -> Markup {
    icon(r#"<path d="M12 2l10 5-10 5L2 7z"/><path d="M2 12l10 5 10-5M2 17l10 5 10-5"/>"#)
}
pub fn package() -> Markup {
    icon(
        r#"<path d="M16.5 9.4L7.5 4.2M21 16V8l-9-5-9 5v8l9 5z"/><path d="M3.3 7L12 12l8.7-5M12 22V12"/>"#,
    )
}
pub fn windows() -> Markup {
    PreEscaped(
        r#"<svg class="ic" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M3 5.5l7.5-1v7H3zM11.5 4.4L21 3v8.5h-9.5zM3 12.5h7.5v7L3 18.5zM11.5 12.5H21V21l-9.5-1.4z"/></svg>"#.into(),
    )
}
pub fn github() -> Markup {
    PreEscaped(
        r#"<svg class="ic" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 .5a11.5 11.5 0 0 0-3.6 22.4c.6.1.8-.3.8-.6v-2c-3.2.7-3.9-1.5-3.9-1.5-.5-1.3-1.3-1.7-1.3-1.7-1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1 1.8 2.8 1.3 3.5 1 .1-.8.4-1.3.7-1.6-2.6-.3-5.3-1.3-5.3-5.7 0-1.3.5-2.3 1.2-3.1-.1-.3-.5-1.5.1-3.1 0 0 1-.3 3.2 1.2a11 11 0 0 1 5.8 0c2.2-1.5 3.2-1.2 3.2-1.2.6 1.6.2 2.8.1 3.1.8.8 1.2 1.8 1.2 3.1 0 4.4-2.7 5.4-5.3 5.7.4.4.8 1.1.8 2.2v3.3c0 .3.2.7.8.6A11.5 11.5 0 0 0 12 .5z"/></svg>"#.into(),
    )
}
pub fn discord() -> Markup {
    PreEscaped(
        r#"<svg class="ic" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M20.32 4.37a19.8 19.8 0 0 0-4.89-1.52.07.07 0 0 0-.08.04c-.21.38-.44.86-.6 1.25a18.3 18.3 0 0 0-5.5 0 12.6 12.6 0 0 0-.61-1.25.08.08 0 0 0-.08-.04 19.7 19.7 0 0 0-4.88 1.52.07.07 0 0 0-.04.03C.53 9.05-.32 13.58.1 18.06a.08.08 0 0 0 .03.05 19.9 19.9 0 0 0 6 3.03.08.08 0 0 0 .08-.03c.46-.63.87-1.3 1.23-2a.08.08 0 0 0-.04-.1 13.1 13.1 0 0 1-1.88-.9.08.08 0 0 1 0-.12l.38-.3a.07.07 0 0 1 .07-.01c3.93 1.8 8.18 1.8 12.06 0a.07.07 0 0 1 .08.01l.37.3a.08.08 0 0 1 0 .12c-.6.35-1.22.65-1.88.9a.08.08 0 0 0-.04.1c.36.7.78 1.37 1.23 2a.08.08 0 0 0 .08.03 19.8 19.8 0 0 0 6-3.03.08.08 0 0 0 .04-.05c.5-5.18-.84-9.67-3.55-13.66a.06.06 0 0 0-.03-.03zM8.02 15.33c-1.18 0-2.16-1.08-2.16-2.42s.96-2.42 2.16-2.42c1.21 0 2.18 1.1 2.16 2.42 0 1.34-.96 2.42-2.16 2.42zm7.97 0c-1.18 0-2.15-1.08-2.15-2.42s.95-2.42 2.15-2.42c1.21 0 2.18 1.1 2.16 2.42 0 1.34-.95 2.42-2.16 2.42z"/></svg>"#.into(),
    )
}
pub fn rust() -> Markup {
    icon(
        r#"<circle cx="12" cy="12" r="8"/><circle cx="12" cy="12" r="3"/><path d="M12 1v3M12 20v3M1 12h3M20 12h3M4.2 4.2l2.1 2.1M17.7 17.7l2.1 2.1M4.2 19.8l2.1-2.1M17.7 6.3l2.1-2.1"/>"#,
    )
}

/// Drapeau d'une langue (SVG 3:2). Pas d'emoji : Windows n'affiche pas les drapeaux emoji.
pub fn flag(code: &str) -> Markup {
    let (view, shapes) = match code {
        "fr" => ("0 0 3 2", r##"<path fill="#002654" d="M0 0h1v2H0z"/><path fill="#fff" d="M1 0h1v2H1z"/><path fill="#ce1126" d="M2 0h1v2H2z"/>"##),
        "it" => ("0 0 3 2", r##"<path fill="#009246" d="M0 0h1v2H0z"/><path fill="#fff" d="M1 0h1v2H1z"/><path fill="#ce2b37" d="M2 0h1v2H2z"/>"##),
        "ru" => ("0 0 3 2", r##"<path fill="#fff" d="M0 0h3v.667H0z"/><path fill="#0039a6" d="M0 .667h3v.667H0z"/><path fill="#d52b1e" d="M0 1.333h3V2H0z"/>"##),
        "es" => ("0 0 3 2", r##"<path fill="#aa151b" d="M0 0h3v2H0z"/><path fill="#f1bf00" d="M0 .5h3v1H0z"/>"##),
        // Portugal : vert et rouge, avec la sphère armillaire simplifiée
        "pt" => ("0 0 30 20", r##"<path fill="#da291c" d="M0 0h30v20H0z"/><path fill="#046a38" d="M0 0h12v20H0z"/><circle cx="12" cy="10" r="4" fill="#ffe900"/><circle cx="12" cy="10" r="2.4" fill="#da291c"/><path fill="#fff" d="M11 8.6h2v2.6a1 1 0 0 1-2 0z"/>"##),
        // Chine : fond rouge, grande étoile et quatre petites
        "zh" => ("0 0 30 20", r##"<path fill="#ee1c25" d="M0 0h30v20H0z"/><path fill="#ffde00" d="M5 2l1.18 3.62H10L6.91 7.86 8.09 11.5 5 9.26 1.91 11.5 3.09 7.86 0 5.62h3.82z"/><circle cx="10" cy="2" r=".9" fill="#ffde00"/><circle cx="12" cy="4" r=".9" fill="#ffde00"/><circle cx="12" cy="7" r=".9" fill="#ffde00"/><circle cx="10" cy="9" r=".9" fill="#ffde00"/>"##),
        // Japon : disque rouge sur fond blanc
        "ja" => ("0 0 30 20", r##"<path fill="#fff" d="M0 0h30v20H0z"/><circle cx="15" cy="10" r="6" fill="#bc002d"/>"##),
        // Allemagne
        "de" => ("0 0 5 3", r##"<path fill="#000" d="M0 0h5v1H0z"/><path fill="#d00" d="M0 1h5v1H0z"/><path fill="#ffce00" d="M0 2h5v1H0z"/>"##),
        // Pays-Bas
        "nl" => ("0 0 3 2", r##"<path fill="#ae1c28" d="M0 0h3v.667H0z"/><path fill="#fff" d="M0 .667h3v.667H0z"/><path fill="#21468b" d="M0 1.333h3V2H0z"/>"##),
        // Pologne
        "pl" => ("0 0 3 2", r##"<path fill="#fff" d="M0 0h3v1H0z"/><path fill="#dc143c" d="M0 1h3v1H0z"/>"##),
        // Ukraine
        "uk" => ("0 0 3 2", r##"<path fill="#0057b7" d="M0 0h3v1H0z"/><path fill="#ffd700" d="M0 1h3v1H0z"/>"##),
        // Indonésie
        "id" => ("0 0 3 2", r##"<path fill="#ce1126" d="M0 0h3v1H0z"/><path fill="#fff" d="M0 1h3v1H0z"/>"##),
        // Turquie : croissant et étoile
        "tr" => ("0 0 30 20", r##"<path fill="#e30a17" d="M0 0h30v20H0z"/><circle cx="11" cy="10" r="5" fill="#fff"/><circle cx="12.25" cy="10" r="4" fill="#e30a17"/><path fill="#fff" d="M15.30,10.00 L16.71,9.50 L16.75,8.00 L17.66,9.19 L19.10,8.77 L18.25,10.00 L19.10,11.23 L17.66,10.81 L16.75,12.00 L16.71,10.50Z"/>"##),
        // Corée du Sud : taegeuk et trigrammes
        "ko" => ("0 0 30 20", r##"<path fill="#fff" d="M0 0h30v20H0z"/><path fill="#cd2e3a" d="M10 10a5 5 0 0 1 10 0a2.5 2.5 0 0 1-5 0a2.5 2.5 0 0 0-5 0z" transform="rotate(33.7 15 10)"/><path fill="#0047a0" d="M20 10a5 5 0 0 1-10 0a2.5 2.5 0 0 1 5 0a2.5 2.5 0 0 0 5 0z" transform="rotate(33.7 15 10)"/><g transform="translate(8.18 5.45) rotate(123.7)" stroke="#000" stroke-width=".7"><path d="M-1.60 -1.05H1.60" /><path d="M-1.60 0.00H1.60" /><path d="M-1.60 1.05H1.60" /></g><g transform="translate(21.82 14.55) rotate(123.7)" stroke="#000" stroke-width=".7"><path d="M-1.60 -1.05H-0.20" /><path d="M0.20 -1.05H1.60" /><path d="M-1.60 0.00H-0.20" /><path d="M0.20 0.00H1.60" /><path d="M-1.60 1.05H-0.20" /><path d="M0.20 1.05H1.60" /></g><g transform="translate(21.82 5.45) rotate(56.3)" stroke="#000" stroke-width=".7"><path d="M-1.60 -1.05H1.60" /><path d="M-1.60 0.00H-0.20" /><path d="M0.20 0.00H1.60" /><path d="M-1.60 1.05H1.60" /></g><g transform="translate(8.18 14.55) rotate(56.3)" stroke="#000" stroke-width=".7"><path d="M-1.60 -1.05H-0.20" /><path d="M0.20 -1.05H1.60" /><path d="M-1.60 0.00H1.60" /><path d="M-1.60 1.05H-0.20" /><path d="M0.20 1.05H1.60" /></g>"##),
        // États-Unis pour l'anglais : 13 bandes, canton bleu et étoiles simplifiées
        _ => ("0 0 38 20", r##"<path fill="#b22234" d="M0 0h38v20H0z"/><path stroke="#fff" stroke-width="1.54" d="M0 2.31h38M0 5.38h38M0 8.46h38M0 11.54h38M0 14.62h38M0 17.69h38"/><path fill="#3c3b6e" d="M0 0h15.2v10.77H0z"/><g fill="#fff"><circle cx="1.9" cy="1.6" r=".55"/><circle cx="5.07" cy="1.6" r=".55"/><circle cx="8.23" cy="1.6" r=".55"/><circle cx="11.4" cy="1.6" r=".55"/><circle cx="3.48" cy="3.7" r=".55"/><circle cx="6.65" cy="3.7" r=".55"/><circle cx="9.82" cy="3.7" r=".55"/><circle cx="12.98" cy="3.7" r=".55"/><circle cx="1.9" cy="5.8" r=".55"/><circle cx="5.07" cy="5.8" r=".55"/><circle cx="8.23" cy="5.8" r=".55"/><circle cx="11.4" cy="5.8" r=".55"/><circle cx="3.48" cy="7.9" r=".55"/><circle cx="6.65" cy="7.9" r=".55"/><circle cx="9.82" cy="7.9" r=".55"/><circle cx="12.98" cy="7.9" r=".55"/><circle cx="1.9" cy="9.6" r=".55"/><circle cx="5.07" cy="9.6" r=".55"/><circle cx="8.23" cy="9.6" r=".55"/><circle cx="11.4" cy="9.6" r=".55"/></g>"##),
    };
    PreEscaped(format!(
        r#"<svg class="flag" viewBox="{view}" preserveAspectRatio="xMidYMid slice" aria-hidden="true">{shapes}</svg>"#
    ))
}
