//! Référencement : données structurées (JSON-LD), robots.txt et sitemap.xml.

use crate::docs;
use crate::github::Release;
use crate::i18n::{Lang, Tr};
use serde_json::json;

/// Noms sous lesquels on cherche le launcher (y compris l'ancien nom, Misty).
pub const ALTERNATE_NAMES: [&str; 9] = [
    "Mistly Launcher",
    "Mistly MC",
    "MistlyMC",
    "Mistly Minecraft",
    "Mistly Minecraft Launcher",
    "Mist Launcher",
    "Misty Launcher",
    "Misty MC",
    "Mistly Client",
];

/// Mots-clés de la balise `keywords` (Bing, Yandex, Baidu, Naver la lisent encore ; Google l'ignore).
pub const KEYWORDS: &str = "Mistly, Mistly Launcher, Mistly Minecraft, Mistly MC, Mist Launcher, Misty Launcher, \
    Minecraft launcher, launcher Minecraft, Minecraft Java launcher, free Minecraft launcher, Minecraft launcher Windows, \
    Fabric launcher, Forge launcher, NeoForge, Quilt, OptiFine, Modrinth, CurseForge, modpacks, Minecraft mods, Mistly Client";

/// Sérialise du JSON-LD pour l'insérer dans une balise <script> sans risque de la refermer.
fn to_script(value: serde_json::Value) -> String {
    value.to_string().replace("</", "<\\/")
}

/// JSON-LD de la page d'accueil : le site, l'organisation et le logiciel.
pub fn home_jsonld(site: &str, tr: &Tr, repo: &str, latest: Option<&Release>) -> String {
    let org = json!({
        "@type": "Organization",
        "@id": format!("{site}/#organization"),
        "name": "Mistly",
        "alternateName": ["Mistly MC", "Misty MC"],
        "url": format!("{site}/"),
        "logo": format!("{site}/assets/icone.png"),
        "sameAs": ["https://github.com/Mistly-MC", format!("https://github.com/{repo}")],
    });
    let website = json!({
        "@type": "WebSite",
        "@id": format!("{site}/#website"),
        "name": "Mistly Launcher",
        "alternateName": ["Mistly", "Mistly MC", "Mistly Minecraft Launcher", "Mist Launcher"],
        "url": format!("{site}/"),
        "inLanguage": Lang::ALL.iter().map(|l| l.code()).collect::<Vec<_>>(),
        "publisher": { "@id": format!("{site}/#organization") },
        // Recherche dans la doc depuis les résultats de recherche
        "potentialAction": {
            "@type": "SearchAction",
            "target": { "@type": "EntryPoint", "urlTemplate": format!("{site}/docs?q={{search_term_string}}") },
            "query-input": "required name=search_term_string",
        },
    });
    let mut app = json!({
        "@type": "SoftwareApplication",
        "@id": format!("{site}/#software"),
        "name": "Mistly Launcher",
        "alternateName": ALTERNATE_NAMES,
        "description": tr.description,
        "url": format!("{site}/"),
        "image": format!("{site}/assets/og/en.png"),
        "screenshot": [format!("{site}/assets/screens/en/home.png"), format!("{site}/assets/screens/en/versions.png"), format!("{site}/assets/screens/en/skin.png")],
        "applicationCategory": "GameApplication",
        "applicationSubCategory": "Minecraft launcher",
        "genre": "Minecraft launcher",
        "keywords": KEYWORDS,
        "operatingSystem": "Windows 10, Windows 11, Linux",
        "downloadUrl": format!("{site}/download/setup"),
        "installUrl": format!("{site}/download/setup"),
        "featureList": [tr.c1_t.clone(), tr.c2_t.clone(), tr.c3_t.clone(), tr.c4_t.clone(), tr.c5_t.clone(), tr.c6_t.clone(), tr.f2_t.clone(), tr.f3_t.clone()],
        "license": format!("https://github.com/{repo}/blob/main/LICENSE.txt"),
        "isAccessibleForFree": true,
        "offers": { "@type": "Offer", "price": "0", "priceCurrency": "EUR" },
        "publisher": { "@id": format!("{site}/#organization") },
        "codeRepository": format!("https://github.com/{repo}"),
        "programmingLanguage": "Rust",
    });
    if let Some(r) = latest {
        app["softwareVersion"] = json!(r.tag_name.trim_start_matches('v'));
        if let Some(date) = &r.published_at {
            app["dateModified"] = json!(date);
        }
    }
    // FAQ de la page : peut s'afficher en questions dépliables dans les résultats
    let questions: Vec<serde_json::Value> = [(&tr.q1, &tr.a1), (&tr.q2, &tr.a2), (&tr.q3, &tr.a3), (&tr.q4, &tr.a4), (&tr.q5, &tr.a5)]
        .iter()
        .map(|(q, a)| json!({ "@type": "Question", "name": q, "acceptedAnswer": { "@type": "Answer", "text": a } }))
        .collect();
    let faq = json!({ "@type": "FAQPage", "@id": format!("{site}/#faq"), "mainEntity": questions });
    to_script(json!({ "@context": "https://schema.org", "@graph": [org, website, app, faq] }))
}

/// JSON-LD d'une page de documentation : article technique + fil d'Ariane.
pub fn doc_jsonld(site: &str, tr: &Tr, page: &docs::DocPage, lang: Lang) -> String {
    let url = format!("{site}/docs/{}?lang={}", page.slug, lang.code());
    to_script(json!({
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "TechArticle",
                "headline": page.title,
                "description": page.description,
                "url": url,
                "inLanguage": lang.code(),
                "about": { "@type": "SoftwareApplication", "name": "Mistly", "alternateName": ALTERNATE_NAMES },
                "publisher": { "@id": format!("{site}/#organization") },
            },
            {
                "@type": "BreadcrumbList",
                "itemListElement": [
                    { "@type": "ListItem", "position": 1, "name": "Mistly MC", "item": format!("{site}/") },
                    { "@type": "ListItem", "position": 2, "name": tr.docs_label, "item": format!("{site}/docs") },
                    { "@type": "ListItem", "position": 3, "name": page.title, "item": url },
                ],
            },
        ],
    }))
}

pub fn robots_txt(site: &str) -> String {
    format!(
        "User-agent: *\nAllow: /\nDisallow: /download/\nDisallow: /api/\nDisallow: /verify\nDisallow: /activity\nDisallow: /admin\n\nSitemap: {site}/sitemap.xml\n"
    )
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Sitemap : chaque page, dans chaque langue, avec ses variantes hreflang.
pub fn sitemap_xml(site: &str) -> String {
    let mut paths: Vec<String> = vec![
        "/".into(),
        "/docs".into(),
        "/patch-notes".into(),
        "/shop".into(),
        "/shop/emotes".into(),
        "/shop/capes".into(),
        "/shop/hats".into(),
        "/shop/pets".into(),
        "/status".into(),
        "/brand".into(),
        "/roadmap".into(),
        "/articles".into(),
    ];
    paths.extend(crate::legal::Kind::ALL.iter().map(|k| k.path().to_string()));
    paths.extend(crate::contenu::articles(Lang::En).iter().map(|a| format!("/articles/{}", a.slug)));
    paths.extend(
        docs::get(Lang::En)
            .pages
            .iter()
            .map(|p| format!("/docs/{}", p.slug)),
    );

    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n",
    );
    for path in &paths {
        let base = format!("{site}{path}");
        let alternates: String = Lang::ALL
            .iter()
            .map(|l| {
                format!(
                    "    <xhtml:link rel=\"alternate\" hreflang=\"{}\" href=\"{}\"/>\n",
                    l.code(),
                    escape_xml(&format!("{base}?lang={}", l.code()))
                )
            })
            .chain(std::iter::once(format!(
                "    <xhtml:link rel=\"alternate\" hreflang=\"x-default\" href=\"{}\"/>\n",
                escape_xml(&base)
            )))
            .collect();
        for l in Lang::ALL {
            let priority = match path.as_str() {
                "/" => "1.0",
                "/docs" | "/patch-notes" => "0.8",
                _ => "0.6",
            };
            out.push_str(&format!(
                "  <url>\n    <loc>{}</loc>\n{alternates}    <priority>{priority}</priority>\n  </url>\n",
                escape_xml(&format!("{base}?lang={}", l.code()))
            ));
        }
    }
    out.push_str("</urlset>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sitemap_lists_every_page_in_every_language() {
        let xml = sitemap_xml("https://example.com");
        let pages = 12 + crate::contenu::articles(Lang::En).len() + crate::legal::Kind::ALL.len() + docs::get(Lang::En).pages.len();
        assert_eq!(xml.matches("<url>").count(), pages * Lang::ALL.len());
        assert!(xml.contains("<loc>https://example.com/docs/installation?lang=fr</loc>"));
        assert!(xml.contains("<loc>https://example.com/privacy?lang=fr</loc>"));
        assert!(xml.contains("hreflang=\"x-default\" href=\"https://example.com/shop\""));
    }

    #[test]
    fn jsonld_is_valid_and_names_misty_mc() {
        let s = home_jsonld("https://example.com", Lang::En.tr(), "Mistly-MC/Mistly-Launcher", None);
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert!(v.to_string().contains("Mistly MC"));
        assert!(!s.contains("</"));
    }
}

/// Manifeste web : nom, icône et couleurs quand le site est ajouté à l'écran d'accueil.
pub fn manifest(tr: &Tr) -> String {
    json!({
        "name": "Mistly Launcher",
        "short_name": "Mistly",
        "description": tr.description,
        "start_url": "/",
        "display": "standalone",
        "background_color": "#09090b",
        "theme_color": "#09090b",
        "icons": [{ "src": "/assets/icone.png", "sizes": "256x256", "type": "image/png" }],
    })
    .to_string()
}

/// IndexNow (Bing, Yandex, Seznam, Naver…) : annonce toutes les pages du sitemap au démarrage, pour
/// qu'elles soient explorées sans attendre. Seulement avec `INDEXNOW_KEY` (la clé est servie sur
/// `/indexnow.txt`).
pub async fn indexnow(site: String, cle: String, http: reqwest::Client) {
    if site.contains("localhost") || site.contains("127.0.0.1") {
        return;
    }
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    let xml = sitemap_xml(&site);
    let urls: Vec<String> = xml
        .split("<loc>")
        .skip(1)
        .filter_map(|p| p.split("</loc>").next())
        .map(|u| u.replace("&amp;", "&"))
        .collect();
    let hote = site.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/').to_string();
    let corps = json!({ "host": hote, "key": cle, "keyLocation": format!("{site}/indexnow.txt"), "urlList": urls });
    match http.post("https://api.indexnow.org/indexnow").json(&corps).send().await {
        Ok(r) => eprintln!("IndexNow : {} page(s) annoncée(s), réponse {}", urls.len(), r.status()),
        Err(e) => eprintln!("IndexNow injoignable : {e}"),
    }
}
