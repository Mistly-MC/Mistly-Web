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
//! Mise en forme des dates et des tailles selon la langue.

use crate::i18n::Tr;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

/// « 20 octobre 2026 » ou « October 20, 2026 » selon `date_order`.
pub fn date(tr: &Tr, iso: Option<&str>) -> String {
    let Some(d) = iso.and_then(|s| OffsetDateTime::parse(s, &Rfc3339).ok()) else {
        return String::new();
    };
    let month = &tr.months[usize::from(u8::from(d.month())) - 1];
    if tr.date_order == "ymd" {
        // chinois et japonais : « 2026年11月16日 » (le mois porte déjà son 月)
        format!("{}年{month}{}日", d.year(), d.day())
    } else if tr.date_order == "ko" {
        // coréen : « 2026년 11월 16일 »
        format!("{}년 {month} {}일", d.year(), d.day())
    } else if tr.date_order == "dmy." {
        // allemand : « 16. November 2026 »
        format!("{}. {month} {}", d.day(), d.year())
    } else if tr.date_order == "mdy" {
        format!("{month} {}, {}", d.day(), d.year())
    } else {
        format!("{} {month} {}", d.day(), d.year())
    }
}

/// Un jour : « 8 novembre » ou « November 8 » (avec l'année si `annee`).
pub fn jour(tr: &Tr, d: time::Date, annee: bool) -> String {
    let month = &tr.months[usize::from(u8::from(d.month())) - 1];
    if tr.date_order == "ymd" {
        return if annee { format!("{}年{month}{}日", d.year(), d.day()) } else { format!("{month}{}日", d.day()) };
    }
    if tr.date_order == "ko" {
        return if annee { format!("{}년 {month} {}일", d.year(), d.day()) } else { format!("{month} {}일", d.day()) };
    }
    let base = match tr.date_order.as_str() {
        "mdy" => format!("{month} {}", d.day()),
        "dmy." => format!("{}. {month}", d.day()),
        _ => format!("{} {month}", d.day()),
    };
    if annee {
        if tr.date_order == "mdy" { format!("{base}, {}", d.year()) } else { format!("{base} {}", d.year()) }
    } else {
        base
    }
}

/// Un instant (secondes Unix, UTC) : « 8 novembre, 14:03 UTC ».
pub fn moment(tr: &Tr, unix: i64) -> String {
    let Ok(t) = OffsetDateTime::from_unix_timestamp(unix) else {
        return String::new();
    };
    format!("{}, {:02}:{:02} UTC", jour(tr, t.date(), false), t.hour(), t.minute())
}

/// « 9,0 Mo » / « 9.0 MB ».
pub fn size(tr: &Tr, bytes: u64) -> String {
    let mb = format!("{:.1}", bytes as f64 / 1_048_576.0);
    format!("{} {}", mb.replace('.', &tr.decimal_sep), tr.mb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn dates() {
        let iso = Some("2026-10-20T10:00:00Z");
        assert_eq!(date(Lang::Fr.tr(), iso), "20 octobre 2026");
        assert_eq!(date(Lang::En.tr(), iso), "October 20, 2026");
        assert_eq!(date(Lang::Fr.tr(), Some("pas une date")), "");
    }

    #[test]
    fn sizes() {
        assert_eq!(size(Lang::Fr.tr(), 9_437_184), "9,0 Mo");
        assert_eq!(size(Lang::En.tr(), 9_437_184), "9.0 MB");
    }
}
