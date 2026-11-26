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
//! Routes HTTP : pages, API JSON et téléchargements.

use crate::AppState;
use crate::docs;
use crate::github::{self, Kind};
use crate::i18n::{self, Lang};
use crate::skins;
use crate::views::docs as docs_view;
use crate::views::shop;
use crate::views::status as status_view;
use crate::views::{home, layout::Page, not_found, patch_notes};
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, Uri, header};
use axum::response::{IntoResponse, Json, Response};
use maud::Markup;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct PageQuery {
    lang: Option<String>,
    filter: Option<String>,
    /// Boutique : pseudo Minecraft saisi.
    player: Option<String>,
    /// Patch notes : `client` pour les versions de Mistly Client.
    project: Option<String>,
}

/// Réponse HTML : pose le cookie de langue si elle vient d'être choisie avec `?lang=`.
fn html_response(status: StatusCode, markup: Markup, lang: Lang, remember: bool) -> Response {
    let mut res = (status, markup).into_response();
    let h = res.headers_mut();
    h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    h.insert(
        header::VARY,
        HeaderValue::from_static("Cookie, Accept-Language"),
    );
    if remember {
        // partagé avec shop., docs. et status. (même domaine)
        let cookie = format!(
            "{}={}; Path=/; Max-Age=31536000; SameSite=Lax{}",
            i18n::COOKIE,
            lang.code(),
            i18n::domaine_cookie()
        );
        if let Ok(v) = HeaderValue::from_str(&cookie) {
            h.insert(header::SET_COOKIE, v);
        }
    }
    res
}

fn page<'a>(state: &'a AppState, lang: Lang, path: &'a str, count: Option<usize>) -> Page<'a> {
    Page {
        lang,
        tr: lang.tr(),
        path,
        release_count: count,
        repo: state.github.repo(),
        scene: &state.scene,
        site_url: &state.site_url,
        base: "",
        boutique: false,
        docs: false,
    }
}

pub async fn index(
    State(state): State<Arc<AppState>>,
    Query(q): Query<PageQuery>,
    headers: HeaderMap,
) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let releases = state.github.releases().await;
    let latest = releases.as_deref().and_then(|l| github::latest(l));
    let p = page(&state, lang, "/", releases.as_ref().map(|l| l.len()));
    let downloads = releases.as_deref().map_or(0, |l| github::total_downloads(l, |nom| state.github.miroir.copies(nom)));
    let linux = visiteur_linux(&headers);
    let mut res = html_response(StatusCode::OK, home::render(&p, latest, downloads, linux), lang, remember);
    // la page change selon le système du visiteur (bouton Linux mis en avant)
    res.headers_mut().insert(
        header::VARY,
        HeaderValue::from_static("Cookie, Accept-Language, User-Agent, Sec-CH-UA-Platform"),
    );
    res
}

/// Le visiteur est-il sur un Linux de bureau ? D'après `Sec-CH-UA-Platform` quand le navigateur
/// l'envoie (Chrome, Edge), sinon d'après le User-Agent. Android et ChromeOS n'en font pas partie :
/// la version Linux ne s'y installe pas.
fn visiteur_linux(headers: &HeaderMap) -> bool {
    if let Some(p) = headers.get("sec-ch-ua-platform").and_then(|v| v.to_str().ok()) {
        return p.trim().trim_matches('"').eq_ignore_ascii_case("linux");
    }
    let ua = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok()).unwrap_or("");
    ua.contains("Linux") && !ua.contains("Android") && !ua.contains("CrOS")
}


pub async fn patch_notes(
    State(state): State<Arc<AppState>>,
    Query(q): Query<PageQuery>,
    headers: HeaderMap,
) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let launcher = state.github.releases().await;
    let client = state.client_releases.releases().await;
    let p = page(&state, lang, "/patch-notes", launcher.as_ref().map(|l| l.len()));
    let filter = patch_notes::Filter::from_query(q.filter.as_deref());
    let est_client = q.project.as_deref() == Some("client");
    let projet = patch_notes::Projet {
        client: est_client,
        repo: if est_client { state.client_releases.repo() } else { state.github.repo() },
        launcher: launcher.as_ref().map(|l| l.len()),
        mod_client: client.as_ref().map(|l| l.len()),
    };
    let releases = if est_client { client } else { launcher };
    let markup = patch_notes::render(&p, releases.as_deref().map(|v| v.as_slice()), filter, &projet);
    html_response(StatusCode::OK, markup, lang, remember)
}

/// `/patch-notes/<tag>` : la page d'une version (`?project=client` pour Mistly Client).
pub async fn patch_notes_version(
    State(state): State<Arc<AppState>>,
    Path(tag): Path<String>,
    Query(q): Query<PageQuery>,
    headers: HeaderMap,
    uri: Uri,
) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let launcher = state.github.releases().await;
    let client = state.client_releases.releases().await;
    let est_client = q.project.as_deref() == Some("client");
    let projet = patch_notes::Projet {
        client: est_client,
        repo: if est_client { state.client_releases.repo() } else { state.github.repo() },
        launcher: launcher.as_ref().map(|l| l.len()),
        mod_client: client.as_ref().map(|l| l.len()),
    };
    let releases = if est_client { client } else { launcher.clone() };
    let chemin = uri.path().to_string();
    let p = page(&state, lang, &chemin, launcher.as_ref().map(|l| l.len()));
    let Some(list) = releases.as_deref() else {
        return redirection("/patch-notes");
    };
    match list.iter().position(|r| r.tag_name == tag) {
        Some(i) => html_response(StatusCode::OK, patch_notes::render_version(&p, list, i, &projet), lang, remember),
        None => html_response(StatusCode::NOT_FOUND, not_found::render(&p), lang, false),
    }
}

pub async fn docs_index(
    State(state): State<Arc<AppState>>,
    Query(q): Query<PageQuery>,
    headers: HeaderMap,
) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let count = state.github.releases().await.map(|l| l.len());
    let mut p = page(&state, lang, "/docs", count);
    if sous_domaine_docs(&headers) {
        p.base = &state.site_url;
        p.docs = true;
    }
    html_response(
        StatusCode::OK,
        docs_view::index(&p, docs::get(lang)),
        lang,
        remember,
    )
}

pub async fn docs_page(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    Query(q): Query<PageQuery>,
    uri: Uri,
    headers: HeaderMap,
) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let set = docs::get(lang);
    let Some(idx) = set.find(&slug) else {
        return not_found(State(state), uri, headers).await;
    };
    let count = state.github.releases().await.map(|l| l.len());
    let path = format!("/docs/{slug}");
    let mut p = page(&state, lang, &path, count);
    if sous_domaine_docs(&headers) {
        p.base = &state.site_url;
        p.docs = true;
    }
    html_response(
        StatusCode::OK,
        docs_view::page(&p, set, idx),
        lang,
        remember,
    )
}

/// Page légale (`/legal`, `/privacy`, `/terms`) : en français pour le français, sinon en anglais.
async fn legal_page(state: &AppState, q: PageQuery, headers: &HeaderMap, kind: crate::legal::Kind) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), headers);
    let count = state.github.releases().await.map(|l| l.len());
    let p = page(state, lang, kind.path(), count);
    let markup = crate::views::legal::render(&p, crate::legal::get(lang, kind));
    html_response(StatusCode::OK, markup, lang, remember)
}

pub async fn legal_notice(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    legal_page(&state, q, &headers, crate::legal::Kind::Notice).await
}

pub async fn privacy(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    legal_page(&state, q, &headers, crate::legal::Kind::Privacy).await
}

pub async fn terms(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    legal_page(&state, q, &headers, crate::legal::Kind::Terms).await
}

pub async fn license(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    legal_page(&state, q, &headers, crate::legal::Kind::License).await
}

/// `/brand` : kit média et charte graphique.
pub async fn brand(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let count = state.github.releases().await.map(|l| l.len());
    let p = page(&state, lang, "/brand", count);
    html_response(StatusCode::OK, crate::views::brand::render(&p), lang, remember)
}

/// `/roadmap` : ce qui est en cours, prévu et terminé.
pub async fn roadmap(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let count = state.github.releases().await.map(|l| l.len());
    let p = page(&state, lang, "/roadmap", count);
    html_response(StatusCode::OK, crate::views::contenu::roadmap(&p), lang, remember)
}

/// `/articles` : la liste des articles.
pub async fn articles(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let count = state.github.releases().await.map(|l| l.len());
    let p = page(&state, lang, "/articles", count);
    html_response(StatusCode::OK, crate::views::contenu::articles(&p), lang, remember)
}

/// `/articles/<slug>` : un article (404 s'il n'existe pas).
pub async fn article(State(state): State<Arc<AppState>>, Path(slug): Path<String>, Query(q): Query<PageQuery>, uri: Uri, headers: HeaderMap) -> Response {
    let Some(a) = crate::contenu::article(i18n::negotiate(q.lang.as_deref(), &headers).0, &slug) else {
        return not_found(State(state), uri, headers).await;
    };
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let count = state.github.releases().await.map(|l| l.len());
    let chemin = format!("/articles/{}", a.slug);
    let p = page(&state, lang, &chemin, count);
    html_response(StatusCode::OK, crate::views::contenu::article(&p, a), lang, remember)
}

/// `/media-kit` : l'autre nom courant de la page, redirigé vers `/brand`.
pub async fn brand_alias() -> Response {
    let mut res = StatusCode::MOVED_PERMANENTLY.into_response();
    res.headers_mut().insert(header::LOCATION, HeaderValue::from_static("/brand"));
    res
}

/// `/tools` et `/tools/…` : les outils ont été retirés, redirection permanente vers l'accueil.
pub async fn outils_retires() -> Response {
    let mut res = StatusCode::MOVED_PERMANENTLY.into_response();
    res.headers_mut().insert(header::LOCATION, HeaderValue::from_static("/"));
    res
}

const PLAYER_COOKIE: &str = "misty-player";

fn read_cookie<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|c| c.trim().split_once('='))
        .find(|(k, _)| *k == name)
        .map(|(_, v)| v)
}

/// Boutique : formulaire du pseudo, puis « bientôt disponible ».
/// Le pseudo saisi (`?player=`), le dernier retenu (cookie) et ce qu'il faut afficher :
/// `(joueur de l'aperçu, texte de la case, saisie refusée, pseudo à retenir)`.
fn joueur_boutique<'a>(headers: &'a HeaderMap, q: &'a PageQuery) -> (Option<&'a str>, &'a str, bool, Option<&'a str>) {
    let saved = read_cookie(headers, PLAYER_COOKIE).filter(|n| shop::valid_username(n));
    match q.player.as_deref().map(str::trim) {
        Some(name) if shop::valid_username(name) => (Some(name), name, false, Some(name)),
        Some(name) => (saved, name, !name.is_empty(), None),
        None => (saved, saved.unwrap_or(""), false, None),
    }
}

/// Une page de la boutique, avec le pseudo à retenir s'il vient d'être saisi.
fn reponse_boutique(markup: Markup, lang: Lang, remember: bool, nouveau: Option<&str>) -> Response {
    let mut res = html_response(StatusCode::OK, markup, lang, remember);
    if let Some(name) = nouveau {
        let cookie = format!("{PLAYER_COOKIE}={name}; Path=/; Max-Age=31536000; SameSite=Lax{}", i18n::domaine_cookie());
        if let Ok(v) = HeaderValue::from_str(&cookie) {
            res.headers_mut().append(header::SET_COOKIE, v);
        }
    }
    res
}

/// La page de la boutique (sur `shop.<domaine>`, ses liens et sa navigation s'y adaptent).
async fn page_boutique<'a>(state: &'a AppState, lang: Lang, chemin: &'a str, headers: &HeaderMap) -> Page<'a> {
    let count = state.github.releases().await.map(|l| l.len());
    let mut p = page(state, lang, chemin, count);
    if sous_domaine_boutique(headers) {
        p.base = &state.site_url;
        p.boutique = true;
    }
    p
}

/// `/shop` : l'accueil de la boutique.
pub async fn shop(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let p = page_boutique(&state, lang, "/shop", &headers).await;
    let (_, _, _, nouveau) = joueur_boutique(&headers, &q);
    reponse_boutique(shop::render(&p), lang, remember, nouveau)
}

/// `/shop/<catégorie>` : une catégorie de la boutique.
pub async fn shop_catalogue(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    Query(q): Query<PageQuery>,
    uri: Uri,
    headers: HeaderMap,
) -> Response {
    let Some(cat) = shop::Catalogue::from_slug(&slug) else {
        return not_found(State(state), uri, headers).await;
    };
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let chemin = format!("/shop/{}", cat.slug());
    let p = page_boutique(&state, lang, &chemin, &headers).await;
    let (_, _, _, nouveau) = joueur_boutique(&headers, &q);
    reponse_boutique(shop::render_catalogue(&p, cat), lang, remember, nouveau)
}

/// `/shop/wishlist` : les articles gardés (la liste est dans le navigateur).
pub async fn shop_wishlist(State(state): State<Arc<AppState>>, Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let p = page_boutique(&state, lang, "/shop/wishlist", &headers).await;
    reponse_boutique(shop::render_wishlist(&p), lang, remember, None)
}

/// `/shop/item/<article>` : la page d'un article, essayé en 3D sur le pseudo saisi ou retenu.
pub async fn shop_item(
    State(state): State<Arc<AppState>>,
    Path(cle): Path<String>,
    Query(q): Query<PageQuery>,
    uri: Uri,
    headers: HeaderMap,
) -> Response {
    let Some(o) = shop::objet(&cle) else {
        return not_found(State(state), uri, headers).await;
    };
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let chemin = format!("/shop/item/{}", o.cle);
    let p = page_boutique(&state, lang, &chemin, &headers).await;
    let (joueur, valeur, refuse, nouveau) = joueur_boutique(&headers, &q);
    reponse_boutique(shop::render_objet(&p, &o, joueur, valeur, refuse), lang, remember, nouveau)
}

fn texture_response(png: axum::body::Bytes) -> Response {
    let mut res = Body::from(png).into_response();
    let h = res.headers_mut();
    h.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/png"));
    h.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=600"),
    );
    res
}

async fn lookup_player(
    state: &AppState,
    name: &str,
    fresh: bool,
) -> Result<Arc<skins::Profile>, Response> {
    let name = name.strip_suffix(".png").unwrap_or(name);
    if !shop::valid_username(name) {
        return Err((StatusCode::NOT_FOUND, "Pseudo invalide").into_response());
    }
    match state.skins.lookup(state.github.http(), name, fresh).await {
        skins::Lookup::Found(p) => Ok(p),
        skins::Lookup::NotFound => {
            Err((StatusCode::NOT_FOUND, "Joueur introuvable").into_response())
        }
        skins::Lookup::Unavailable => {
            Err((StatusCode::BAD_GATEWAY, "Skin indisponible").into_response())
        }
    }
}

/// `/skin/<pseudo>` (ou `/skin/<pseudo>.png`) : le skin du joueur, relayé depuis le site
/// pour que l'aperçu 3D puisse le lire.
pub async fn skin(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Response {
    match lookup_player(&state, &name, false).await {
        Ok(p) => texture_response(p.skin.clone()),
        Err(res) => res,
    }
}

/// `/cape/<pseudo>` : la cape Mojang du joueur, s'il en a une.
pub async fn cape(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Response {
    match lookup_player(&state, &name, false).await {
        Ok(p) => match &p.cape {
            Some(cape) => texture_response(cape.clone()),
            None => (StatusCode::NOT_FOUND, "Pas de cape").into_response(),
        },
        Err(res) => res,
    }
}

#[derive(Deserialize)]
pub struct PlayerQuery {
    /// `?fresh=1` : recharge le profil chez Mojang au lieu d'utiliser le cache.
    fresh: Option<String>,
}

/// `/player/<pseudo>` : pseudo exact, UUID, modèle de bras et présence d'une cape (JSON).
pub async fn player(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(q): Query<PlayerQuery>,
) -> Response {
    let fresh = q.fresh.is_some();
    match lookup_player(&state, &name, fresh).await {
        Ok(p) => {
            let mut res = Json(serde_json::json!({
                "name": p.name,
                "uuid": p.uuid.as_deref().map(skins::dashed_uuid),
                "slim": p.slim,
                "cape": p.cape.is_some(),
            }))
            .into_response();
            res.headers_mut().insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=600"),
            );
            res
        }
        Err(res) => res,
    }
}

pub async fn robots(State(state): State<Arc<AppState>>) -> Response {
    let mut res = crate::seo::robots_txt(&state.site_url).into_response();
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    res
}

pub async fn sitemap(State(state): State<Arc<AppState>>) -> Response {
    let mut res = crate::seo::sitemap_xml(&state.site_url).into_response();
    let h = res.headers_mut();
    h.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/xml; charset=utf-8"),
    );
    h.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600"),
    );
    res
}

/// Index de recherche plein texte de la doc, chargé par site.js à la première recherche.
pub async fn docs_search_index(Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    let (lang, _) = i18n::negotiate(q.lang.as_deref(), &headers);
    let entries: Vec<_> = docs::get(lang)
        .pages
        .iter()
        .map(|p| serde_json::json!({ "slug": p.slug, "text": p.text }))
        .collect();
    let mut res = Json(entries).into_response();
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=300"),
    );
    res
}

pub async fn not_found(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    headers: HeaderMap,
) -> Response {
    let (lang, _) = i18n::negotiate(None, &headers);
    let path = uri.path().to_string();
    let p = page(&state, lang, &path, None);
    html_response(StatusCode::NOT_FOUND, not_found::render(&p), lang, false)
}

/// Infos de la dernière version (même format que l'ancienne fonction Cloudflare).
pub async fn api_latest(State(state): State<Arc<AppState>>) -> Response {
    let releases = state.github.releases().await;
    let Some(r) = releases.as_deref().and_then(|l| github::latest(l)) else {
        let mut res = (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "unavailable" })),
        )
            .into_response();
        res.headers_mut().insert(header::RETRY_AFTER, HeaderValue::from_static("60"));
        return res;
    };
    let mut res = Json(serde_json::json!({
        "version": r.tag_name,
        "date": r.published_at,
        "notes": r.notes(),
        "setupSize": r.asset(Kind::Setup).map(|a| a.size),
        "portableSize": r.asset(Kind::Portable).map(|a| a.size),
        "linuxSize": r.asset(Kind::Linux).map(|a| a.size),
    }))
    .into_response();
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=120"),
    );
    res
}

/// `/download/setup`, `/download/portable` et `/download/linux` : le fichier de la dernière version, dans l'ordre
/// 1. la copie sur le serveur (voir `miroir`) ;
/// 2. sinon, le fichier relayé depuis GitHub (l'utilisateur reste sur le domaine du site) ;
/// 3. sinon, une redirection vers GitHub : un téléchargement ne finit jamais sur une erreur.
pub async fn download(
    State(state): State<Arc<AppState>>,
    Path(kind): Path<String>,
    req: axum::extract::Request,
) -> Response {
    let Some(kind) = Kind::from_slug(&kind) else {
        return (StatusCode::NOT_FOUND, "Type inconnu").into_response();
    };
    let page_github = format!("https://github.com/{}/releases/latest", state.github.repo());
    let releases = state.github.releases().await;
    let Some(asset) = releases
        .as_deref()
        .and_then(|l| github::latest(l))
        .and_then(|r| r.asset(kind))
        .cloned()
    else {
        return redirection(&page_github);
    };
    let disposition = format!("attachment; filename=\"{}\"", asset.name.replace('"', ""));
    let content_type = asset
        .content_type
        .clone()
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| "application/octet-stream".into());

    // 1. copie locale (reprise des téléchargements interrompus gérée par ServeFile)
    if let Some(chemin) = state.github.miroir.fichier(&asset.name, asset.size) {
        use tower::ServiceExt;
        let Ok(res) = tower_http::services::ServeFile::new(chemin).oneshot(req).await;
        let mut res = res.map(Body::new);
        for (name, value) in [(header::CONTENT_DISPOSITION, &disposition), (header::CONTENT_TYPE, &content_type)] {
            if let Ok(v) = HeaderValue::from_str(value) {
                res.headers_mut().insert(name, v);
            }
        }
        res.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("public, max-age=300"));
        return res;
    }

    // 2. relais depuis GitHub
    let upstream = match state.github.miroir.http().get(&asset.browser_download_url).send().await {
        Ok(r) if r.status().is_success() => r,
        // 3. dernier recours : GitHub directement
        _ => return redirection(&asset.browser_download_url),
    };
    let longueur = upstream.content_length().unwrap_or(asset.size);
    let mut res = Response::new(Body::from_stream(upstream.bytes_stream()));
    let h = res.headers_mut();
    for (name, value) in [
        (header::CONTENT_TYPE, content_type),
        (header::CONTENT_LENGTH, longueur.to_string()),
        (header::CONTENT_DISPOSITION, disposition),
        (header::CACHE_CONTROL, "public, max-age=300".to_string()),
    ] {
        if let Ok(v) = HeaderValue::from_str(&value) {
            h.insert(name, v);
        }
    }
    res
}

fn redirection(url: &str) -> Response {
    let mut res = StatusCode::FOUND.into_response();
    if let Ok(v) = HeaderValue::from_str(url) {
        res.headers_mut().insert(header::LOCATION, v);
    }
    res.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    res
}

/// `/status` : état des services dont Mistly a besoin (vérifiés par le site toutes les minutes).
pub async fn status_page(
    State(state): State<Arc<AppState>>,
    Query(q): Query<PageQuery>,
    headers: HeaderMap,
) -> Response {
    let (lang, remember) = i18n::negotiate(q.lang.as_deref(), &headers);
    let count = state.github.releases().await.map(|l| l.len());
    let mut p = page(&state, lang, "/status", count);
    // Sur status.<domaine>, les liens de la navigation et du pied de page mènent au site principal
    if sous_domaine_statut(&headers) {
        p.base = &state.site_url;
    }
    let markup = status_view::render(&p, &state.status.lignes(), &state.status.incidents(), state.status.dispo_globale(), state.status.age(), &state.deploiements.liste());
    let mut res = html_response(StatusCode::OK, markup, lang, remember);
    res.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    res
}

/// `/status.json` : la même chose pour les robots (et les curieux).
pub async fn status_json(State(state): State<Arc<AppState>>) -> Response {
    let services: Vec<_> = state
        .status
        .lignes()
        .iter()
        .map(|l| {
            let s = &crate::status::SERVICES[l.index];
            serde_json::json!({
                "name": s.nom,
                "url": s.url,
                "state": l.actuel.map(|m| m.etat.slug()),
                "latency_ms": l.actuel.and_then(|m| m.latence).map(|d| d.as_millis() as u64),
                "uptime_30d_percent": l.dispo,
                "days": l.jours.iter().map(|(d, j)| serde_json::json!({
                    "date": d.to_string(),
                    "state": j.map(|j| j.etat().slug()),
                    "uptime_percent": j.map(|j| j.dispo()),
                    "checks": j.map(|j| j.total()).unwrap_or(0),
                })).collect::<Vec<_>>(),
                "minutes": l.minutes.iter().map(|(t, m)| serde_json::json!({
                    "start": t,
                    "state": m.map(|m| m.etat.slug()),
                    "latency_ms": m.and_then(|m| m.ms),
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let mut res = Json(serde_json::json!({
        "checked_seconds_ago": state.status.age(),
        "uptime_30d_percent": state.status.dispo_globale(),
        "incidents": state.status.incidents().iter().map(|i| serde_json::json!({
            "service": i.service,
            "kind": if i.panne { "outage" } else { "slowdown" },
            "start": i.debut,
            "end": i.fin,
        })).collect::<Vec<_>>(),
        "services": services,
    }))
    .into_response();
    res.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    res
}

/// `/site.webmanifest`
pub async fn manifest(Query(q): Query<PageQuery>, headers: HeaderMap) -> Response {
    let (lang, _) = i18n::negotiate(q.lang.as_deref(), &headers);
    let mut res = ([(header::CONTENT_TYPE, "application/manifest+json")], crate::seo::manifest(lang.tr())).into_response();
    res.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("public, max-age=86400"));
    res
}

/// `/indexnow.txt` : la clé IndexNow (`INDEXNOW_KEY`), pour prouver que les annonces viennent du site.
pub async fn indexnow_cle() -> Response {
    match std::env::var("INDEXNOW_KEY").ok().filter(|k| (8..=128).contains(&k.len())) {
        Some(k) => ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], k).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// Redirection permanente (301).
fn redirection_permanente(cible: &str) -> Response {
    let mut res = StatusCode::MOVED_PERMANENTLY.into_response();
    if let Ok(v) = HeaderValue::from_str(cible) {
        res.headers_mut().insert(header::LOCATION, v);
    }
    res
}

/// Le tableau de bord sur son sous-domaine (`a0.<domaine>`), avant le routage : `/` devient
/// `/admin`, et les chemins hors de `/admin` y sont rangés (`/players` → `/admin/players`). Les
/// fichiers statiques et les têtes des joueurs (`/skin/…`) restent servis tels quels.
pub fn sous_domaine_admin(mut req: axum::extract::Request) -> axum::extract::Request {
    let sur_a0 = req
        .headers()
        .get("x-forwarded-host")
        .or_else(|| req.headers().get(header::HOST))
        .and_then(|v| v.to_str().ok())
        .is_some_and(|h| h.to_ascii_lowercase().starts_with("a0."));
    if !sur_a0 {
        return req;
    }
    let chemin = req.uri().path();
    let garde = ["/admin", "/css/", "/js/", "/assets/", "/vendor/", "/skin/", "/robots.txt", "/site.webmanifest"]
        .iter()
        .any(|p| chemin.starts_with(p));
    if !garde {
        let nouveau = format!(
            "/admin{}{}",
            if chemin == "/" { "" } else { chemin },
            req.uri().query().map(|q| format!("?{q}")).unwrap_or_default()
        );
        if let Ok(uri) = nouveau.parse() {
            *req.uri_mut() = uri;
        }
    }
    req
}

/// La requête arrive-t-elle sur le sous-domaine de la documentation (`docs.<domaine>`) ?
fn sous_domaine_docs(entetes: &HeaderMap) -> bool {
    entetes
        .get("x-forwarded-host")
        .or_else(|| entetes.get(header::HOST))
        .and_then(|v| v.to_str().ok())
        .is_some_and(|h| h.to_ascii_lowercase().starts_with("docs."))
}

/// La requête arrive-t-elle sur le sous-domaine de la boutique (`shop.<domaine>`) ?
fn sous_domaine_boutique(entetes: &HeaderMap) -> bool {
    entetes
        .get("x-forwarded-host")
        .or_else(|| entetes.get(header::HOST))
        .and_then(|v| v.to_str().ok())
        .is_some_and(|h| h.to_ascii_lowercase().starts_with("shop."))
}

/// La requête arrive-t-elle sur le sous-domaine de statut (`status.<domaine>`) ?
fn sous_domaine_statut(entetes: &HeaderMap) -> bool {
    entetes
        .get("x-forwarded-host")
        .or_else(|| entetes.get(header::HOST))
        .and_then(|v| v.to_str().ok())
        .is_some_and(|h| h.to_ascii_lowercase().starts_with("status."))
}

/// Anciennes adresses (misty-launcher.online, www.) : les pages redirigent pour de bon vers le
/// domaine principal, pour que les moteurs de recherche n'en gardent qu'une. L'API, les
/// téléchargements restent servis sur toutes les adresses (le launcher les appelle).
///
/// Le sous-domaine `status.<domaine>` sert la page de statut à sa racine (`/`), avec les
/// fichiers statiques et `/status.json` ; ses autres pages renvoient vers le domaine principal.
pub async fn domaine_principal(
    State(state): State<Arc<AppState>>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    let hote = req
        .headers()
        .get("x-forwarded-host")
        .or_else(|| req.headers().get(header::HOST))
        .and_then(|v| v.to_str().ok())
        .map(|h| h.split(':').next().unwrap_or(h).to_ascii_lowercase());
    let principal = state.site_url.split("://").nth(1).unwrap_or("").trim_end_matches('/').to_ascii_lowercase();
    let chemin = req.uri().path();
    let page = req.method() == axum::http::Method::GET
        && !["/api/", "/download/", "/skin/", "/cape/", "/player/", "/admin", "/status.json", "/indexnow"]
            .iter()
            .any(|p| chemin.starts_with(p));
    if let Some(h) = &hote
        && h.starts_with("docs.")
        && req.method() == axum::http::Method::GET
    {
        let utile = ["/css/", "/js/", "/assets/", "/vendor/", "/skin/", "/site.webmanifest", "/robots.txt"]
            .iter()
            .any(|p| chemin.starts_with(p));
        if utile {
            return next.run(req).await;
        }
        let q = Query::<PageQuery>::try_from_uri(req.uri()).unwrap_or(Query(PageQuery::default()));
        let entetes = req.headers().clone();
        let uri = req.uri().clone();
        // (le routage est déjà fait quand ce middleware tourne : on appelle les pages directement)
        match chemin {
            "/" => return docs_index(State(state.clone()), q, entetes).await,
            "/search.json" => return docs_search_index(q, entetes).await,
            _ => {}
        }
        let requete = req.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
        if let Some(reste) = chemin.strip_prefix("/docs")
            && (reste.is_empty() || reste.starts_with('/'))
        {
            return redirection_permanente(&format!("/{}{requete}", reste.trim_start_matches('/')));
        }
        let slug = chemin.trim_start_matches('/');
        if !slug.contains('/') && docs::get(Lang::En).find(slug).is_some() {
            return docs_page(State(state.clone()), Path(slug.to_string()), q, uri, entetes).await;
        }
        return redirection_permanente(&format!("{}{}", state.site_url, req.uri().path_and_query().map_or("/", |p| p.as_str())));
    }
    if let Some(h) = &hote
        && h.starts_with("shop.")
        && req.method() == axum::http::Method::GET
    {
        // les fichiers et relais dont la boutique a besoin (aperçu 3D : skins, capes, modèles)
        let utile = ["/css/", "/js/", "/assets/", "/vendor/", "/models/", "/skin/", "/cape/", "/player/", "/site.webmanifest", "/robots.txt"]
            .iter()
            .any(|p| chemin.starts_with(p));
        if utile {
            return next.run(req).await;
        }
        let q = Query::<PageQuery>::try_from_uri(req.uri()).unwrap_or(Query(PageQuery::default()));
        let entetes = req.headers().clone();
        let uri = req.uri().clone();
        // (le routage est déjà fait quand ce middleware tourne : on appelle les pages directement)
        if chemin == "/" {
            return shop(State(state.clone()), q, entetes).await;
        }
        if chemin == "/wishlist" {
            return shop_wishlist(State(state.clone()), q, entetes).await;
        }
        if let Some(cle) = chemin.strip_prefix("/item/") {
            return shop_item(State(state.clone()), Path(cle.to_string()), q, uri, entetes).await;
        }
        if let Some(cat) = shop::Catalogue::from_slug(chemin.trim_start_matches('/')) {
            return shop_catalogue(State(state.clone()), Path(cat.slug().to_string()), q, uri, entetes).await;
        }
        // /shop et /shop/<catégorie> → la même page à la racine du sous-domaine ; le reste → le site
        let requete = req.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
        let cible = match chemin.strip_prefix("/shop") {
            Some(reste) if reste.is_empty() || reste.starts_with('/') => format!("/{}{requete}", reste.trim_start_matches('/')),
            _ => format!("{}{}", state.site_url, req.uri().path_and_query().map_or("/", |p| p.as_str())),
        };
        return redirection_permanente(&cible);
    }
    if let Some(h) = &hote
        && h.starts_with("status.")
        && req.method() == axum::http::Method::GET
    {
        let statique = ["/css/", "/js/", "/assets/", "/vendor/", "/status.json", "/site.webmanifest", "/robots.txt"]
            .iter()
            .any(|p| chemin.starts_with(p));
        if chemin == "/" {
            // (le routage est déjà fait quand ce middleware tourne : on appelle la page directement)
            let q = Query::<PageQuery>::try_from_uri(req.uri()).unwrap_or(Query(PageQuery::default()));
            let entetes = req.headers().clone();
            return status_page(State(state.clone()), q, entetes).await;
        }
        if !statique {
            // /status → la racine du sous-domaine ; le reste → le site principal
            let cible = if chemin == "/status" {
                format!("/{}", req.uri().query().map(|q| format!("?{q}")).unwrap_or_default())
            } else {
                format!("{}{}", state.site_url, req.uri().path_and_query().map_or("/", |p| p.as_str()))
            };
            let mut res = StatusCode::MOVED_PERMANENTLY.into_response();
            if let Ok(v) = HeaderValue::from_str(&cible) {
                res.headers_mut().insert(header::LOCATION, v);
            }
            return res;
        }
        return next.run(req).await;
    }
    // Le tableau de bord n'existe que sur son sous-domaine (a0.<domaine>) : sur le site principal
    // (et ses anciennes adresses), /admin n'existe pas. En local (localhost), il reste ouvert pour les essais.
    if (chemin == "/admin" || chemin.starts_with("/admin/"))
        && let Some(h) = &hote
        && !h.starts_with("a0.")
        && !principal.is_empty()
        && (h == &principal || h.ends_with(&format!(".{principal}")) || h.ends_with("misty-launcher.online"))
    {
        return StatusCode::NOT_FOUND.into_response();
    }
    // Avec DOCS_URL réglée, /docs et ses pages partent vers le sous-domaine de la doc
    if let Some(reste) = chemin.strip_prefix("/docs")
        && (reste.is_empty() || reste.starts_with('/'))
        && let Some(url) = crate::views::layout::DOCS_URL.as_deref()
    {
        let requete = req.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
        return redirection_permanente(&format!("{url}/{}{requete}", reste.trim_start_matches('/')));
    }
    // Avec SHOP_URL réglée, /shop et ses catégories partent vers le sous-domaine de la boutique
    if let Some(reste) = chemin.strip_prefix("/shop")
        && (reste.is_empty() || reste.starts_with('/'))
        && let Some(url) = crate::views::layout::SHOP_URL.as_deref()
    {
        let requete = req.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
        return redirection_permanente(&format!("{url}/{}{requete}", reste.trim_start_matches('/')));
    }
    // Avec STATUS_URL réglée, /status du site principal part vers le sous-domaine
    if chemin == "/status"
        && let Some(url) = crate::views::layout::STATUS_URL.as_deref()
    {
        let cible = format!("{url}{}", req.uri().query().map(|q| format!("?{q}")).unwrap_or_default());
        let mut res = StatusCode::MOVED_PERMANENTLY.into_response();
        if let Ok(v) = HeaderValue::from_str(&cible) {
            res.headers_mut().insert(header::LOCATION, v);
        }
        return res;
    }
    if let Some(h) = hote {
        let ancien = h != principal && (h == format!("www.{principal}") || h.ends_with("misty-launcher.online"));
        if ancien && page && !principal.is_empty() {
            let cible = format!("{}{}", state.site_url, req.uri().path_and_query().map_or("/", |p| p.as_str()));
            let mut res = StatusCode::MOVED_PERMANENTLY.into_response();
            if let Ok(v) = HeaderValue::from_str(&cible) {
                res.headers_mut().insert(header::LOCATION, v);
            }
            return res;
        }
    }
    next.run(req).await
}

#[cfg(test)]
mod tests_systeme {
    use super::*;

    fn avec(nom: &'static str, valeur: &'static str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(nom, HeaderValue::from_static(valeur));
        h
    }

    #[test]
    fn detecte_linux() {
        let ua = |v| avec("user-agent", v);
        assert!(visiteur_linux(&ua("Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0")));
        assert!(visiteur_linux(&ua("Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/537.36 Chrome/131.0")));
        assert!(!visiteur_linux(&ua("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/131.0")));
        assert!(!visiteur_linux(&ua("Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 Chrome/131.0 Mobile")));
        assert!(!visiteur_linux(&ua("Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 Chrome/131.0")));
        assert!(!visiteur_linux(&HeaderMap::new()));
        assert!(visiteur_linux(&avec("sec-ch-ua-platform", "\"Linux\"")));
        assert!(!visiteur_linux(&avec("sec-ch-ua-platform", "\"Android\"")));
        assert!(!visiteur_linux(&avec("sec-ch-ua-platform", "\"Chrome OS\"")));
    }
}
