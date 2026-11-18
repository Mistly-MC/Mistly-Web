//! Boutique de cosmétiques.
//!
//! - Sans pseudo : la vitrine. L'article à la une en 3D (sur Steve, ou sur le pseudo saisi), les
//!   tendances à essayer d'un clic, et les catégories qui arrivent bientôt.
//! - `/shop/emotes`, `/shop/capes`, `/shop/hats` : un catalogue par catégorie, essayé dans l'aperçu 3D
//!   (les emotes et les chapeaux ne sont pas encore en jeu).
//! - Avec un pseudo : le studio. À gauche, le skin du joueur en 3D (vues face / profil / dos,
//!   animations, sa vraie cape, élytres) ; à droite, sa fiche et les catégories de
//!   cosmétiques, toutes « bientôt disponibles », avec la cape et les élytres Mistly à essayer.

use super::icons;
use super::layout::{self, Head, Page};
use crate::i18n::Tr;
use maud::{Markup, html};

/// Ce que la page affiche.
pub enum ShopView<'a> {
    /// La boutique, avec le pseudo saisi ou retenu (son skin dans l'aperçu) et, si besoin,
    /// l'erreur ; `open` : l'essayage est déplié d'office (un pseudo vient d'être saisi).
    Form { value: &'a str, invalid: bool, open: bool },
}

/// Pseudo Minecraft valide : 3 à 16 caractères, lettres ASCII, chiffres et `_`.
pub fn valid_username(name: &str) -> bool {
    (3..=16).contains(&name.len()) && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// Catégories de cosmétiques montrées dans « Bientôt dans la boutique ».
#[derive(Clone, Copy, PartialEq, Eq)]
enum Category {
    Capes,
    Headwear,
    Auras,
    /// Styles du pseudo au-dessus de la tête
    Tags,
    Pets,
    Emotes,
}

impl Category {
    fn text(self, tr: &Tr) -> (&str, &str) {
        match self {
            Category::Capes => (&tr.shop_cat_capes, &tr.shop_cat_capes_p),
            Category::Headwear => (&tr.shop_cat_hats, &tr.shop_cat_hats_p),
            Category::Auras => (&tr.shop_cat_effects, &tr.shop_cat_effects_p),
            Category::Tags => (&tr.shop_cat_tags, &tr.shop_cat_tags_p),
            Category::Pets => (&tr.shop_cat_pets, &tr.shop_cat_pets_p),
            Category::Emotes => (&tr.shop_cat_emotes, &tr.shop_cat_emotes_p),
        }
    }

    fn icon(self) -> Markup {
        match self {
            Category::Capes => icons::cape(),
            Category::Headwear => icons::crown(),
            Category::Auras => icons::sparkles(),
            Category::Tags => icons::tag(),
            Category::Pets => icons::paw(),
            Category::Emotes => icons::smile(),
        }
    }
}

pub fn render(p: &Page, view: ShopView) -> Markup {
    let tr = p.tr;
    let main = match view {
        ShopView::Form { value, invalid, open } => {
            let preview = valid_username(value).then_some(value);
            html! {
                section.shop-top {
                    div.col {
                        header.shop-head {
                            span.label { (tr.shop_label) }
                            h1 { (tr.shop_h1_1) " " span.soft { (tr.shop_h1_2) } }
                            p { (tr.shop_click_hint) }
                        }
                    }
                }
                // L'essayage : le studio 3D, dans la page, déplié seulement quand on clique sur un
                // cosmétique (ou ouvert d'office après une saisie de pseudo, acceptée ou refusée)
                section.tryon #tryon aria-labelledby="feat-name" hidden[!invalid && !open] {
                  div.tryon-in {
                    div.col.tryon-box {
                    button.tryon-close type="button" data-close aria-label=(tr.shop_close) { "✕" }
                        div.vitrine {
                            (stage(p, preview.unwrap_or("MHF_Steve"), preview, false))
                            div.featured {
                                span.label { (tr.shop_featured) }
                                h1 #feat-name { (tr.shop_item_cape) }
                                p #feat-text { (tr.shop_cape_p) }
                                div.badges { span.badge.free #feat-badge { (tr.shop_free_client) } }
                                div.pick role="group" aria-label=(tr.shop_collection) {
                                    @for a in &ARTICLES { (bouton_article(tr, a)) }
                                }
                                (reglages(tr, preview))
                                div.try {
                                    strong { (tr.shop_try) }
                                    p { (tr.shop_try_p) }
                                    (form(p, value, invalid, "/shop"))
                                }
                            }
                        }
                    }
                  }
                }
                (collection(p))
            }
        }
    };
    layout::render(
        p,
        Head {
            title: &tr.shop_title,
            description: &tr.shop_sub,
            jsonld: None,
            css: &["home", "patch-notes", "shop"],
            scripts: &["/vendor/skinview3d/skinview3d.bundle.js"],
            body_class: "shop",
        },
        main,
    )
}

/// Haut des pages de la boutique (studio, catalogues) : un bouton pour revenir en arrière, puis le
/// fil d'Ariane jusqu'à `ici`.
fn haut_de_page(tr: &Tr, ici: &str) -> Markup {
    html! {
        div.page-top {
            // revient à la page d'avant si elle est sur le site (voir site.js), sinon à la boutique
            a.back-btn href="/shop" data-retour { (icons::arrow_right()) span { (tr.shop_back) } }
            nav.crumbs aria-label="Breadcrumb" {
                a href="/shop" { (icons::bag()) (tr.shop_label) }
                span { "/" }
                span { (ici) }
            }
        }
    }
}

/// Une page de catalogue de la boutique (`/shop/<slug>`), ouverte depuis sa carte.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Catalogue {
    Emotes,
    Capes,
    Hats,
    Pets,
}

impl Catalogue {
    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "emotes" => Some(Self::Emotes),
            "capes" => Some(Self::Capes),
            "hats" => Some(Self::Hats),
            "pets" => Some(Self::Pets),
            _ => None,
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Self::Emotes => "emotes",
            Self::Capes => "capes",
            Self::Hats => "hats",
            Self::Pets => "pets",
        }
    }

    /// Titre et phrase d'introduction de la page.
    fn texte(self, tr: &Tr) -> (&str, &str) {
        match self {
            Self::Emotes => (&tr.shop_cat_emotes, &tr.shop_emotes_lead),
            Self::Capes => (&tr.shop_cat_capes, &tr.shop_capes_lead),
            Self::Hats => (&tr.shop_cat_hats, &tr.shop_hats_lead),
            Self::Pets => (&tr.shop_cat_pets, &tr.shop_pets_lead),
        }
    }

    /// Les articles Mistly de la page (ailes et capes, chapeaux, ou compagnons).
    fn articles(self) -> impl Iterator<Item = &'static Article> {
        ARTICLES.iter().filter(move |a| match self {
            // les élytres se mettent avec le bouton « Élytres » de l'aperçu, pas dans la liste
            Self::Capes => matches!(a.back, Some("cape" | "wings")),
            Self::Hats => a.back.is_none(),
            Self::Pets => a.back == Some("pet"),
            Self::Emotes => false,
        })
    }
}

/// Réglages de l'article porté, montrés par site.js quand ils servent : la taille du chapeau
/// (de 60 % à 100 %, la taille d'origine ; 80 % par défaut), le pseudo dont le Minime prend le skin (le
/// sien par défaut) et l'épaule où il s'assoit ; pour les emotes à deux, le skin de l'autre joueur
/// et, sur le dos, ce que fait le porteur.
fn reglages(tr: &Tr, player: Option<&str>) -> Markup {
    html! {
        div.reglage #reglage-taille hidden {
            label for="taille-chapeau" { (tr.shop_size) }
            input #taille-chapeau type="range" min="0.6" max="1" step="0.05" value="0.8";
            output #taille-valeur for="taille-chapeau" { "80 %" }
        }
        div.reglage #reglage-minime hidden {
            label for="minime-nom" { (tr.shop_minime_name) }
            input #minime-nom type="text" value=(player.unwrap_or("")) placeholder="Steve" maxlength="16"
                autocomplete="off" autocapitalize="off" spellcheck="false";
        }
        // l'épaule où il s'assoit (droite : à droite quand on regarde le joueur)
        div.reglage #reglage-epaule hidden {
            span.etiquette { (tr.shop_shoulder) }
            div.seg.epaules role="group" aria-label=(tr.shop_shoulder) {
                button type="button" data-epaule="left" aria-pressed="false" { (tr.shop_shoulder_left) }
                button type="button" data-epaule="right" aria-pressed="true" { (tr.shop_shoulder_right) }
            }
        }
        // Câlin et sur le dos : le pseudo dont l'autre joueur prend le skin (Alex par défaut)
        div.reglage #reglage-ami hidden {
            label for="ami-nom" { (tr.shop_partner_name) }
            input #ami-nom type="text" placeholder="Alex" maxlength="16"
                autocomplete="off" autocapitalize="off" spellcheck="false";
        }
        // Sur le dos : ce que fait le porteur
        div.reglage #reglage-porteur hidden {
            span.etiquette { (tr.shop_carrier) }
            div.seg.allures role="group" aria-label=(tr.shop_carrier) {
                @for (cle, nom) in [("idle", &tr.shop_anim_idle), ("walk", &tr.shop_anim_walk), ("run", &tr.shop_anim_run), ("crouch", &tr.shop_anim_crouch)] {
                    button type="button" data-allure=(cle) aria-pressed=(if cle == "run" { "true" } else { "false" }) { (nom) }
                }
            }
        }
    }
}

/// Bouton d'un article Mistly à essayer : vignette et nom (vitrine et pages de catalogue).
fn bouton_article(tr: &Tr, a: &Article) -> Markup {
    html! {
        button.equip type="button" data-back=(a.back.unwrap_or("none")) data-src=(a.src) data-thumb=(a.thumb)
            data-name=(a.nom(tr)) data-text=(a.texte(tr)) aria-pressed="false"
            data-badge=(a.badge(tr)) data-early=[a.early.then_some("1")] data-soon=[a.bientot.then_some("1")]
            data-equip=(tr.shop_equip) data-unequip=(tr.shop_unequip) {
            canvas.thumb data-thumb=(a.thumb) width="30" height="48" aria-hidden="true" {}
            span { (a.nom(tr)) }
        }
    }
}

/// Une page de catalogue (emotes, capes, chapeaux) : l'aperçu 3D à gauche, sur le dernier pseudo
/// choisi (ou Steve), et la liste à droite. Un clic joue l'emote ou porte l'article ; la case du
/// pseudo, en dessous, met son propre skin dans l'aperçu.
pub fn render_catalogue(p: &Page, cat: Catalogue, player: Option<&str>, value: &str, invalid: bool) -> Markup {
    let tr = p.tr;
    let (titre, intro) = cat.texte(tr);
    let nombre = if cat == Catalogue::Emotes { EMOTES.len() } else { cat.articles().count() };
    // la fiche montre d'abord le premier de la liste
    let (premier, premier_texte, premier_badge) = match cat.articles().next() {
        Some(a) => (a.nom(tr), a.texte(tr), a.badge(tr)),
        None => {
            let (nom, texte) = emote_texte(tr, EMOTES[0]);
            (nom, texte, tr.shop_new.as_str())
        }
    };
    let main = html! {
        section.studio.emotes-page data-catalogue=(cat.slug()) {
            div.wrap {
                (haut_de_page(tr, titre))
                header.emotes-head {
                    h1 { (titre) sup { "(" (nombre) ")" } }
                    p { (intro) }
                }
                div.vitrine {
                    (stage(p, player.unwrap_or("MHF_Steve"), player, false))
                    div.featured {
                        span.label { (titre) }
                        h1 #feat-name { (premier) }
                        p #feat-text { (premier_texte) }
                        div.badges { span.badge #feat-badge { (premier_badge) } }
                        div.pick.emote-pick role="group" aria-label=(titre) {
                            @if cat == Catalogue::Emotes {
                                @for (i, cle) in EMOTES.iter().enumerate() {
                                    @let (nom, texte) = emote_texte(tr, cle);
                                    button.emote-choice type="button" data-try=(cle) data-name=(nom) data-text=(texte)
                                        data-badge=(if *cle == "hug" { &tr.shop_new } else { &tr.shop_badge })
                                        aria-pressed=(if i == 0 { "true" } else { "false" }) {
                                        canvas.thumb data-thumb=(format!("emote-{cle}")) width="30" height="48" aria-hidden="true" {}
                                        span { (nom) }
                                    }
                                }
                            } @else {
                                @for a in cat.articles() { (bouton_article(tr, a)) }
                            }
                        }
                        (reglages(tr, player))
                        div.try {
                            strong { (tr.shop_try) }
                            p { (tr.shop_try_p) }
                            (form(p, value, invalid, &format!("/shop/{}", cat.slug())))
                        }
                    }
                }
            }
        }
    };
    let titre_page = format!("{titre} – {}", tr.shop_title);
    layout::render(
        p,
        Head {
            title: &titre_page,
            description: intro,
            jsonld: None,
            css: &["home", "patch-notes", "shop"],
            scripts: &["/vendor/skinview3d/skinview3d.bundle.js"],
            body_class: "shop",
        },
        main,
    )
}

/// Un article de la collection Mistly.
struct Article {
    /// Vignette dessinée par site.js
    thumb: &'static str,
    /// Porté dans le dos (`cape`, `elytra`, ou `wings` en plus de la cape), sur l'épaule (`pet`) ;
    /// aucun pour un chapeau
    back: Option<&'static str>,
    /// Texture utilisée dans l'aperçu 3D (`misty` ou `early`)
    src: &'static str,
    early: bool,
    /// Pas encore dans Mistly Client : seulement dans l'aperçu du site (badge « Bientôt »)
    bientot: bool,
}

const ARTICLES: [Article; 7] = [
    Article { thumb: "cape", back: Some("cape"), src: "misty", early: false, bientot: false },
    Article { thumb: "elytra", back: Some("elytra"), src: "misty", early: false, bientot: false },
    // Ailes de brume : portées en plus de la cape (public/models/mist_wings.bbmodel)
    Article { thumb: "wings", back: Some("wings"), src: "", early: false, bientot: true },
    Article { thumb: "early", back: Some("cape"), src: "early", early: true, bientot: false },
    Article { thumb: "cloudly", back: None, src: "", early: false, bientot: false },
    Article { thumb: "bunny", back: None, src: "", early: false, bientot: true },
    Article { thumb: "minime", back: Some("pet"), src: "", early: false, bientot: true },
];

impl Article {
    fn nom<'a>(&self, tr: &'a Tr) -> &'a str {
        match self.thumb {
            "cape" => &tr.shop_item_cape,
            "elytra" => &tr.shop_item_elytra,
            "early" => &tr.shop_early_t,
            "bunny" => &tr.shop_bunny_t,
            "minime" => &tr.shop_minime_t,
            "wings" => &tr.shop_wings_t,
            _ => &tr.shop_cloudly_t,
        }
    }
    fn texte<'a>(&self, tr: &'a Tr) -> &'a str {
        match self.thumb {
            "cape" => &tr.shop_cape_p,
            "elytra" => &tr.shop_elytra_p,
            "early" => &tr.shop_early_p,
            "bunny" => &tr.shop_bunny_p,
            "minime" => &tr.shop_minime_p,
            "wings" => &tr.shop_wings_p,
            _ => &tr.shop_cloudly_p,
        }
    }
    /// Le badge de l'article : premiers joueurs, bientôt, ou gratuit avec Mistly Client.
    fn badge<'a>(&self, tr: &'a Tr) -> &'a str {
        if self.early {
            &tr.shop_early_badge
        } else if self.bientot {
            &tr.shop_badge
        } else {
            &tr.shop_free_client
        }
    }
}

/// Les emotes du catalogue, dans l'ordre affiché. Un aperçu sur le site : elles ne sont pas
/// encore dans Mistly Client. Leurs animations et leurs vignettes sont dans `site.js`.
const EMOTES: [&str; 7] = ["hug", "piggyback", "wave", "clap", "dance", "cheer", "facepalm"];

/// Nom et description d'une emote.
fn emote_texte<'a>(tr: &'a Tr, cle: &str) -> (&'a str, &'a str) {
    match cle {
        "hug" => (&tr.shop_anim_hug, &tr.shop_emote_hug_p),
        "wave" => (&tr.shop_emote_wave, &tr.shop_emote_wave_p),
        "clap" => (&tr.shop_emote_clap, &tr.shop_emote_clap_p),
        "dance" => (&tr.shop_emote_dance, &tr.shop_emote_dance_p),
        "cheer" => (&tr.shop_emote_cheer, &tr.shop_emote_cheer_p),
        "piggyback" => (&tr.shop_emote_piggyback, &tr.shop_emote_piggyback_p),
        _ => (&tr.shop_emote_facepalm, &tr.shop_emote_facepalm_p),
    }
}

/// Image de présentation d'un article ou d'une emote, rendue sur le skin EnderMythex
/// (public/assets/presentation, 600 × 475, le personnage sans le titre), s'il y en a une.
fn presentation(cle: &str) -> Option<&'static str> {
    Some(match cle {
        "cape" => "cape-mistly",
        "early" => "cape-early",
        "elytra" => "elytres-mistly",
        "cloudly" => "cloudly",
        "bunny" => "oreilles-de-lapin",
        "minime" => "minime",
        "emote-hug" => "calin",
        "emote-cheer" => "victoire",
        "emote-wave" => "salut",
        "emote-piggyback" => "piggyback",
        _ => return None,
    })
}

/// Le visuel d'une carte : son image de présentation, sinon sa vignette en pixels (site.js).
fn art_carte(cle: &str, l: u32, h: u32) -> Markup {
    html! {
        @if let Some(image) = presentation(cle) {
            span.item-art.photo {
                img src=(format!("/assets/presentation/{image}.jpg?v={}", layout::ASSET_VERSION.as_str())) alt="" width="600" height="475" loading="lazy";
            }
        } @else {
            span.item-art.emote[cle.starts_with("emote-")] { canvas.thumb data-thumb=(cle) width=(l) height=(h) aria-hidden="true" {} }
        }
    }
}

/// Carte d'une emote (tendances, catalogue) : un clic la joue dans l'essayage.
fn carte_emote(tr: &Tr, cle: &str, nouveau: bool) -> Markup {
    let (nom, texte) = emote_texte(tr, cle);
    let badge = if nouveau { &tr.shop_new } else { &tr.shop_badge };
    html! {
        button.item-card.reveal type="button" data-try=(cle) data-carte-emote="" data-name=(nom) data-text=(texte) data-badge=(tr.shop_badge) {
            (art_carte(&format!("emote-{cle}"), 90, 144))
            span.item-body {
                strong { (nom) }
                span.desc { (texte) }
                span.item-foot {
                    span.badge.new[nouveau] { (badge) }
                    span.try-it { (tr.shop_try_on) (icons::arrow_right()) }
                }
            }
        }
    }
}

/// Les tendances : la nouveauté puis les pages les plus vues (cape, élytres, Cloudly).
const TENDANCES: [&str; 3] = ["cape", "elytra", "cloudly"];

/// Les tendances, puis les catégories qui arrivent bientôt.
fn collection(p: &Page) -> Markup {
    let tr = p.tr;
    let bientot = [Category::Headwear, Category::Capes, Category::Tags, Category::Auras, Category::Pets, Category::Emotes];
    html! {
        section.blk {
            div.col {
                h2.blk-title { (tr.shop_trending) sup { "(" (TENDANCES.len() + 1) ")" } }
                div.cells.c4 {
                    // la nouveauté d'abord : le Câlin
                    (carte_emote(tr, "hug", true))
                    @for a in ARTICLES.iter().filter(|a| TENDANCES.contains(&a.thumb)) {
                        button.item-card.reveal type="button" data-try=(a.thumb) {
                            (art_carte(a.thumb, 60, 96))
                            span.item-body {
                                strong { (a.nom(tr)) }
                                span.desc { (a.texte(tr)) }
                                span.item-foot {
                                    @if a.early { span.badge.early { (tr.shop_early_badge) } } @else { span.badge.free { (tr.shop_available) } }
                                    span.try-it { (tr.shop_try_on) (icons::arrow_right()) }
                                }
                            }
                        }
                    }
                }
            }
        }
        div.col.hatch aria-hidden="true" {}
        section.blk {
            div.col {
                h2.blk-title { (tr.shop_coming_t) sup { "(" (bientot.len()) ")" } }
                div.cells.c3 {
                    @for c in bientot {
                        @let (titre, texte) = c.text(tr);
                        // Chapeaux, capes, compagnons et emotes ont déjà leur page : la carte l'ouvre, sans « Bientôt »
                        @let page = match c {
                            Category::Headwear => Some(Catalogue::Hats),
                            Category::Capes => Some(Catalogue::Capes),
                            Category::Emotes => Some(Catalogue::Emotes),
                            Category::Pets => Some(Catalogue::Pets),
                            _ => None,
                        };
                        @let corps = html! {
                            pre.ascii-noise aria-hidden="true" {}
                            span.soon-body {
                                span.ico { (c.icon()) }
                                strong { (titre) }
                                span.desc { (texte) }
                                span.soon-foot {
                                    @if page.is_some() {
                                        span {}
                                        span.try-it { (tr.shop_try_on) (icons::arrow_right()) }
                                    } @else {
                                        span.badge { (tr.shop_badge) }
                                    }
                                }
                            }
                        };
                        @if let Some(cat) = page {
                            a.soon-card.clickable.reveal href=(format!("/shop/{}", cat.slug())) { (corps) }
                        } @else {
                            div.soon-card.reveal { (corps) }
                        }
                    }
                }
            }
        }
    }
}

/// Aperçu 3D (skinview3d) : vues, animations, cape et élytres.
/// Le skin et la cape sont relayés par le serveur (`/skin/…`, `/cape/…`, `/player/…`).
fn stage(p: &Page, skin: &str, name: Option<&str>, studio: bool) -> Markup {
    let tr = p.tr;
    let anims = [
        ("idle", &tr.shop_anim_idle),
        ("walk", &tr.shop_anim_walk),
        ("run", &tr.shop_anim_run),
        ("fly", &tr.shop_anim_fly),
        ("crouch", &tr.shop_anim_crouch),
    ];
    html! {
        div.stage.studio-stage[studio] #stage data-skin=(skin) data-name=(name.unwrap_or("")) {
            div.stage-top {
                div.seg role="group" aria-label=(tr.shop_view) {
                    button type="button" data-view="front" aria-pressed="true" { (tr.shop_view_front) }
                    button type="button" data-view="side" aria-pressed="false" { (tr.shop_view_side) }
                    button type="button" data-view="back" aria-pressed="false" { (tr.shop_view_back) }
                }
                div.toggles {
                    button.toggle #reload type="button" title=(tr.shop_reload_hint) aria-label=(tr.shop_reload_hint) { (icons::refresh()) span { (tr.shop_reload) } }
                    button.toggle type="button" data-toggle="rotate" aria-pressed="true" { (icons::orbit()) span { (tr.shop_rotate) } }
                    // le pseudo au-dessus de la tête : affiché ou caché
                    button.toggle type="button" data-toggle="name" aria-pressed="true" { (icons::tag()) span { (tr.shop_show_name) } }
                }
            }
            div.halo aria-hidden="true" {}
            canvas #skin3d role="img" aria-label=(tr.shop_preview) {}
            div.stage-bar {
                // l'indication au-dessus des boutons, dans la barre : ils ne la cachent plus
                p.drag { (tr.shop_drag) }
                div.seg.anims role="group" aria-label=(tr.shop_anim) {
                    @for (key, label) in anims {
                        button type="button" data-anim=(key) aria-pressed=(if key == "walk" { "true" } else { "false" }) { (label) }
                    }
                }
                div.toggles {
                    // Câlin : avec un deuxième joueur en face (montré seulement pendant le câlin)
                    button.toggle type="button" data-toggle="partner" aria-pressed="true" hidden { (icons::user()) span { (tr.shop_partner) } }
                    button.toggle type="button" data-toggle="cape" aria-pressed="false" disabled title=(tr.shop_no_cape) { (icons::cape()) span { (tr.shop_own_cape) } }
                    button.toggle type="button" data-toggle="elytra" aria-pressed="false" { (icons::wings()) span { (tr.shop_elytra) } }
                }
            }
        }
    }
}

/// Formulaire du pseudo. `action` : la page qui l'affiche ensuite (la boutique, ou un catalogue
/// qui garde le joueur dans son aperçu).
fn form(p: &Page, value: &str, invalid: bool, action: &str) -> Markup {
    let tr = p.tr;
    html! {
        form.shop-form method="get" action=(action) novalidate[invalid] {
            label for="player" { (tr.shop_username) }
            div.field.error[invalid] {
                span.prefix { (icons::user()) }
                input #player name="player" type="text" value=(value) placeholder=(tr.shop_placeholder)
                    minlength="3" maxlength="16" pattern="[A-Za-z0-9_]{3,16}" required
                    autocomplete="off" autocapitalize="off" spellcheck="false"
                    aria-invalid=[invalid.then_some("true")] aria-describedby="player-help";
                button.btn.play type="submit" { (tr.shop_continue) " →" }
            }
            @if invalid {
                p.help.err #player-help role="alert" { (tr.shop_invalid) }
            } @else {
                p.help #player-help { (tr.shop_hint) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usernames() {
        for ok in ["Steve", "abc", "Misty_x_x", "A1234567890123_5"] {
            assert!(valid_username(ok), "{ok}");
        }
        for ko in [
            "",
            "ab",
            "trop_long_pseudo_17",
            "é_accent",
            "a b c",
            "<script>",
            "../x",
        ] {
            assert!(!valid_username(ko), "{ko}");
        }
    }
}
