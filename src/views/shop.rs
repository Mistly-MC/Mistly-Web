//! Boutique de cosmétiques, rangée comme un magasin : peu de texte, des catégories, des cartes.
//!
//! - `/shop` : l'accueil. Le titre et la recherche, la barre des catégories, les nouveautés,
//!   « Découvrir » (une bannière par catégorie), puis chaque catégorie en rangée de cartes.
//! - `/shop/<catégorie>` (`capes`, `hats`, `pets`, `emotes`) : la même tête, et toutes les cartes de
//!   la catégorie.
//! - `/shop/item/<article>` : la page d'un article. L'aperçu 3D à gauche (site.js le porte dès que le
//!   joueur est chargé), et en dessous « Personnaliser l'aperçu » (couleurs, taille, pseudo…) ; la
//!   fiche à droite (statut, une phrase, le bouton pour l'obtenir), puis des articles proches.
//!
//! Servie sur `/shop` et à la racine du sous-domaine `shop.<domaine>` : les liens passent par
//! `Page::boutique`.

use super::icons;
use super::layout::{self, Head, Page};
use crate::i18n::Tr;
use maud::{Markup, html};

/// Pseudo Minecraft valide : 3 à 16 caractères, lettres ASCII, chiffres et `_`.
pub fn valid_username(name: &str) -> bool {
    (3..=16).contains(&name.len()) && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// Une catégorie de la boutique (`/shop/<slug>`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Catalogue {
    Capes,
    Hats,
    Pets,
    Emotes,
}

impl Catalogue {
    pub const ALL: [Catalogue; 4] = [Catalogue::Capes, Catalogue::Hats, Catalogue::Pets, Catalogue::Emotes];

    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.slug() == slug)
    }

    pub fn slug(self) -> &'static str {
        match self {
            Self::Capes => "capes",
            Self::Hats => "hats",
            Self::Pets => "pets",
            Self::Emotes => "emotes",
        }
    }

    fn nom(self, tr: &Tr) -> &str {
        match self {
            Self::Capes => &tr.shop_cat_capes,
            Self::Hats => &tr.shop_cat_hats,
            Self::Pets => &tr.shop_cat_pets,
            Self::Emotes => &tr.shop_cat_emotes,
        }
    }

    fn icone(self) -> Markup {
        match self {
            Self::Capes => icons::cape(),
            Self::Hats => icons::crown(),
            Self::Pets => icons::paw(),
            Self::Emotes => icons::smile(),
        }
    }

    /// L'image de sa bannière (« Découvrir »).
    fn image(self) -> &'static str {
        match self {
            Self::Capes => "cape-mistly",
            Self::Hats => "cloudly",
            Self::Pets => "minime",
            Self::Emotes => "calin",
        }
    }
}

/// Le statut d'un article : sur sa carte et sur sa fiche.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Statut {
    /// Dans Mistly Client, pour tout le monde
    Gratuit,
    /// Réservé aux premiers joueurs
    Early,
    /// Nouveau, pas encore en jeu
    Nouveau,
    /// Pas encore en jeu : seulement dans l'aperçu
    Bientot,
}

impl Statut {
    fn classe(self) -> &'static str {
        match self {
            Statut::Gratuit => "st-free",
            Statut::Early => "st-early",
            Statut::Nouveau => "st-new",
            Statut::Bientot => "st-soon",
        }
    }
    /// L'étiquette courte, sur l'image.
    fn badge(self, tr: &Tr) -> &str {
        match self {
            Statut::Gratuit => &tr.shop_available,
            Statut::Early => &tr.shop_early_badge,
            Statut::Nouveau => &tr.shop_new,
            Statut::Bientot => &tr.shop_badge,
        }
    }
    fn disponible(self) -> bool {
        matches!(self, Statut::Gratuit | Statut::Early)
    }
}

/// Un article de la boutique : un cosmétique de la collection Mistly, ou une emote.
#[derive(Clone, Copy)]
pub struct Objet {
    pub cle: &'static str,
    cat: Catalogue,
    article: Option<&'static Article>,
}

impl Objet {
    fn nom<'a>(&self, tr: &'a Tr) -> &'a str {
        self.article.map_or_else(|| emote_texte(tr, self.cle).0, |a| a.nom(tr))
    }
    fn texte<'a>(&self, tr: &'a Tr) -> &'a str {
        self.article.map_or_else(|| emote_texte(tr, self.cle).1, |a| a.texte(tr))
    }
    fn statut(&self) -> Statut {
        match self.article {
            Some(a) if a.early => Statut::Early,
            Some(a) if !a.bientot => Statut::Gratuit,
            None if self.cle == "hug" => Statut::Nouveau,
            _ => Statut::Bientot,
        }
    }
    /// La clé de son visuel (`art_carte`).
    fn art(&self) -> String {
        if self.article.is_some() { self.cle.to_string() } else { format!("emote-{}", self.cle) }
    }
}

/// Tous les articles, dans l'ordre de la boutique.
pub fn objets() -> Vec<Objet> {
    let cat = |a: &Article| match a.back {
        Some("pet") => Catalogue::Pets,
        None => Catalogue::Hats,
        _ => Catalogue::Capes,
    };
    let mut liste: Vec<Objet> = ARTICLES.iter().map(|a| Objet { cle: a.thumb, cat: cat(a), article: Some(a) }).collect();
    liste.extend(EMOTES.iter().map(|e| Objet { cle: e, cat: Catalogue::Emotes, article: None }));
    liste
}

/// L'article `cle`, s'il existe.
pub fn objet(cle: &str) -> Option<Objet> {
    objets().into_iter().find(|o| o.cle == cle)
}

/// Les nouveautés, en tête de la boutique.
const NOUVEAUTES: [&str; 5] = ["hug", "wings", "minime", "bunny", "piggyback"];

/// Adresse d'une image de présentation (public/assets/presentation, 600 × 475).
fn photo(nom: &str) -> String {
    format!("/assets/presentation/{nom}.jpg?v={}", layout::ASSET_VERSION.as_str())
}

/// La tête de la boutique : le titre et la recherche, puis la barre des catégories.
fn tete(p: &Page, actif: Option<Catalogue>) -> Markup {
    let tr = p.tr;
    html! {
        section.shop-top.sx-top {
            div.col {
                div.sx-head {
                    div {
                        h1 { (tr.shop_label) }
                        p { (tr.shop_mega_p) }
                    }
                    label.sx-search {
                        (icons::search())
                        input #sx-q type="search" placeholder=(tr.shop_search) aria-label=(tr.shop_search) autocomplete="off" spellcheck="false";
                    }
                }
                nav.sx-tabs aria-label=(tr.shop_categories) {
                    a href=(p.boutique("")) aria-current=[actif.is_none().then_some("page")] { (icons::sparkles()) (tr.shop_tab_featured) }
                    span.sx-sep aria-hidden="true" {}
                    @for c in Catalogue::ALL {
                        a href=(p.boutique(c.slug())) aria-current=[(actif == Some(c)).then_some("page")] { (c.icone()) (c.nom(tr)) }
                    }
                }
            }
        }
    }
}

/// Une carte : l'image et son étiquette, le nom et le statut. Elle ouvre la page de l'article.
fn carte(p: &Page, o: &Objet) -> Markup {
    let tr = p.tr;
    let s = o.statut();
    html! {
        a.sx-card href=(p.boutique(&format!("item/{}", o.cle))) data-nom=(o.nom(tr).to_lowercase()) {
            span.sx-art {
                @if presentation(&o.art()).is_some() {
                    (art_carte(&o.art(), 60, 96))
                } @else {
                    // pas de photo : l'icône de sa catégorie (les vignettes en pixels ont besoin de l'aperçu 3D)
                    span.item-art.sx-icon { (if o.cle == "wings" { icons::wings() } else { o.cat.icone() }) }
                }
                span.sx-badge.(s.classe()) { (s.badge(tr)) }
                span.sx-cat { (o.cat.nom(tr)) }
            }
            span.sx-foot {
                strong { (o.nom(tr)) }
                span.sx-state.(s.classe()) { @if s.disponible() { (icons::check()) } (s.badge(tr)) }
            }
        }
    }
}

/// Une rangée titrée de cartes, avec « Tout voir ».
fn rangee(p: &Page, titre: &str, tout: Option<&str>, objets: &[Objet]) -> Markup {
    html! {
        section.sx-row {
            div.sx-row-head {
                h2 { (titre) sup { "(" (objets.len()) ")" } }
                @if let Some(lien) = tout { a.sx-all href=(lien) { (p.tr.shop_view_all) (icons::arrow_right()) } }
            }
            div.sx-grid { @for o in objets { (carte(p, o)) } }
        }
    }
}

fn page_boutique(p: &Page, titre: &str, description: &str, main: Markup, scripts: &[&str]) -> Markup {
    layout::render(
        p,
        Head { title: titre, description, jsonld: None, css: &["home", "patch-notes", "shop"], scripts, body_class: "shop" },
        main,
    )
}

/// L'accueil de la boutique.
pub fn render(p: &Page) -> Markup {
    let tr = p.tr;
    let tous = objets();
    let nouveautes: Vec<Objet> = NOUVEAUTES.iter().filter_map(|c| objet(c)).collect();
    let main = html! {
        (tete(p, None))
        div.col.sx-body {
            (rangee(p, &tr.shop_new_t, None, &nouveautes))
            section.sx-row {
                div.sx-row-head { h2 { (tr.shop_discover) } }
                div.sx-banners {
                    @for c in Catalogue::ALL {
                        @let n = tous.iter().filter(|o| o.cat == c).count();
                        a.sx-banner href=(p.boutique(c.slug())) {
                            img src=(photo(c.image())) alt="" width="600" height="475" loading="lazy";
                            span.sx-banner-txt { strong { (c.nom(tr)) } span { (n) " " (tr.shop_items) } }
                        }
                    }
                }
            }
            @for c in Catalogue::ALL {
                @let liste: Vec<Objet> = tous.iter().filter(|o| o.cat == c).copied().collect();
                (rangee(p, c.nom(tr), Some(&p.boutique(c.slug())), &liste))
            }
            p.sx-empty hidden { (tr.shop_no_result) }
        }
    };
    page_boutique(p, &tr.shop_title, &tr.shop_sub, main, &["/js/boutique.js"])
}

/// Une catégorie de la boutique.
pub fn render_catalogue(p: &Page, cat: Catalogue) -> Markup {
    let tr = p.tr;
    let liste: Vec<Objet> = objets().into_iter().filter(|o| o.cat == cat).collect();
    let main = html! {
        (tete(p, Some(cat)))
        div.col.sx-body {
            (rangee(p, cat.nom(tr), None, &liste))
            p.sx-empty hidden { (tr.shop_no_result) }
        }
    };
    let titre = format!("{} – {}", cat.nom(tr), tr.shop_title);
    page_boutique(p, &titre, &tr.shop_mega_p, main, &["/js/boutique.js"])
}

/// La page d'un article : l'aperçu 3D (porté d'office) et ses réglages, la fiche, puis des
/// articles proches.
pub fn render_objet(p: &Page, o: &Objet, player: Option<&str>, value: &str, invalid: bool) -> Markup {
    let tr = p.tr;
    let s = o.statut();
    let proches: Vec<Objet> = {
        let tous = objets();
        let mut l: Vec<Objet> = tous.iter().filter(|x| x.cat == o.cat && x.cle != o.cle).copied().collect();
        l.extend(tous.iter().filter(|x| x.cat != o.cat).copied());
        l.truncate(4);
        l
    };
    let ici = p.boutique(&format!("item/{}", o.cle));
    let main = html! {
        section.studio.sx-item data-catalogue=(o.cat.slug()) {
            div.col {
                nav.sx-crumbs aria-label="Breadcrumb" {
                    a href=(p.boutique("")) { (tr.shop_label) }
                    (icons::chevron())
                    a href=(p.boutique(o.cat.slug())) { (o.cat.nom(tr)) }
                    (icons::chevron())
                    span { (o.nom(tr)) }
                }
                div.sx-title {
                    a.sx-back href=(p.boutique(o.cat.slug())) data-retour aria-label=(tr.shop_back) { (icons::arrow_left()) }
                    h1 { (o.nom(tr)) }
                    span.sx-badge.(s.classe()) { (s.badge(tr)) }
                }
                div.sx-item-grid {
                    div.sx-left {
                        div.sx-stage {
                            (stage(p, player.unwrap_or("MHF_Steve"), player, false))
                        }
                        div.sx-panel {
                            h2 { (icons::palette()) (tr.shop_customize) }
                            (reglages(tr, player))
                            (form(p, value, invalid, &ici))
                        }
                        // le bouton que site.js clique dès que le joueur est chargé : l'article est porté
                        div.pick hidden {
                            @if let Some(a) = o.article {
                                (bouton_article(tr, a))
                            } @else {
                                @let (nom, texte) = emote_texte(tr, o.cle);
                                button.emote-choice type="button" data-try=(o.cle) data-name=(nom) data-text=(texte) data-badge=(s.badge(tr)) aria-pressed="false" { (nom) }
                            }
                        }
                    }
                    aside.sx-info {
                        div.sx-price {
                            span.sx-state.(s.classe()) { @if s.disponible() { (icons::check()) } (s.badge(tr)) }
                            strong { @if s.disponible() { (tr.shop_free_client) } @else { (tr.shop_soon) } }
                        }
                        p.sx-desc { (o.texte(tr)) }
                        ul.sx-facts {
                            li { (icons::user()) (tr.shop_trust_1) }
                            li { (o.cat.icone()) (o.cat.nom(tr)) }
                            li { (icons::sparkles()) (tr.shop_trust_3) }
                        }
                        @if s.disponible() {
                            a.btn.play.sx-get href=(p.lien("/#download")) { (icons::download()) span { (tr.shop_get) } }
                        } @else {
                            button.btn.sx-get type="button" disabled { (tr.shop_soon) }
                        }
                    }
                }
                (rangee(p, &tr.shop_more, Some(&p.boutique(o.cat.slug())), &proches))
            }
        }
    };
    let titre = format!("{} – {}", o.nom(tr), tr.shop_title);
    page_boutique(p, &titre, o.texte(tr), main, &["/vendor/skinview3d/skinview3d.bundle.js"])
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
        // Ailes de brume : des couleurs toutes faites (Mistly par défaut), puis une couleur libre pour
        // la membrane et pour les os. Les tons sont définis dans site.js (PRESETS_AILES) ; la
        // pastille montre le ton principal de la membrane.
        div.reglage #reglage-ailes data-reglage-ailes hidden {
            span.etiquette { (tr.shop_wings_color) }
            div.teintes role="group" aria-label=(tr.shop_wings_color) {
                @for (cle, nom, ton) in [
                    ("misty", &tr.shop_tint_misty, "linear-gradient(135deg,#04050c,#4a7fe6 55%,#f2f7ff)"),
                    ("crimson", &tr.shop_tint_crimson, "#a01a37"),
                    ("amethyst", &tr.shop_tint_amethyst, "#8a3fd0"), ("jade", &tr.shop_tint_jade, "#1f9e66"),
                    ("gold", &tr.shop_tint_gold, "#d0951f"), ("shadow", &tr.shop_tint_shadow, "#34344a"),
                    ("snow", &tr.shop_tint_snow, "#d6deec"),
                ] {
                    button.teinte type="button" data-teinte=(cle) aria-pressed=(if cle == "misty" { "true" } else { "false" })
                        title=(nom) aria-label=(nom) style=(format!("--c:{ton}")) {}
                }
            }
        }
        div.reglage.couleurs-libres data-reglage-ailes hidden {
            label for="ailes-membrane" { (tr.shop_wings_membrane) }
            input #ailes-membrane type="color" value="#4a7fe6";
            label for="ailes-os" { (tr.shop_wings_bones) }
            input #ailes-os type="color" value="#2e3448";
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
