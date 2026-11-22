//! Boutique de cosmétiques, rangée comme un magasin.
//!
//! Toutes les pages ont la barre de la boutique (collée sous la navigation) : les catégories (une
//! pastille glisse sur l'onglet choisi, et le contenu change sans recharger la page), la recherche
//! et la wishlist. Ensuite :
//! - `/shop` : un carrousel des articles à la une, « Que cherches-tu ? » (les catégories en
//!   mosaïque), les populaires (une rangée qui défile), les nouveautés et les articles vus
//!   récemment (gardés dans le navigateur).
//! - `/shop/<catégorie>` (`capes`, `hats`, `pets`, `emotes`) : ses cartes, à trier et à filtrer.
//! - `/shop/wishlist` : les articles gardés d'un clic sur leur cœur (dans le navigateur).
//! - `/shop/item/<article>` : la page d'un article. L'aperçu 3D à gauche (site.js le porte dès que le
//!   joueur est chargé) et « Personnaliser l'aperçu » ; la fiche à droite (statut, wishlist,
//!   partage, le bouton pour l'obtenir), puis des articles proches.
//!
//! boutique.js anime tout cela. Servie sur `/shop` et à la racine du sous-domaine `shop.<domaine>` :
//! les liens passent par `Page::boutique`.

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

    /// L'image de sa tuile dans « Que cherches-tu ? ».
    fn image(self) -> &'static str {
        match self {
            Self::Capes => "cape-early",
            Self::Hats => "oreilles-de-lapin",
            Self::Pets => "minime",
            Self::Emotes => "victoire",
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

fn liste(cles: &[&str]) -> Vec<Objet> {
    cles.iter().filter_map(|c| objet(c)).collect()
}

/// Les nouveautés, les populaires et les diapositives du carrousel.
const NOUVEAUTES: [&str; 5] = ["hug", "wings", "minime", "bunny", "piggyback"];
const POPULAIRES: [&str; 7] = ["cape", "elytra", "cloudly", "early", "hug", "bunny", "wave"];
/// (article, image) : les images sont en 600 × 475
const CARROUSEL: [(&str, &str); 4] = [("hug", "calin"), ("early", "cape-early"), ("cloudly", "cloudly"), ("minime", "minime")];

/// Adresse d'une image de présentation (public/assets/presentation, 600 × 475).
fn photo(nom: &str) -> String {
    format!("/assets/presentation/{nom}.jpg?v={}", layout::ASSET_VERSION.as_str())
}

/// Où l'on est dans la boutique (l'onglet allumé).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ici {
    Accueil,
    Categorie(Catalogue),
    Wishlist,
}

/// La barre de la boutique : les onglets (et leur pastille), la recherche, la wishlist. Les
/// textes dont boutique.js a besoin sont dans ses `data-`.
fn barre(p: &Page, ici: Ici) -> Markup {
    let tr = p.tr;
    html! {
        div.sx-bar #sx-bar data-added=(tr.shop_wish_added) data-removed=(tr.shop_wish_removed) data-copied=(tr.shop_link_copied)
            data-add=(tr.shop_wish_add) data-remove=(tr.shop_wish_remove) {
            div.col.sx-bar-in {
                nav.sx-tabs aria-label=(tr.shop_categories) {
                    span.sx-pill aria-hidden="true" {}
                    a href=(p.boutique("")) data-onglet aria-current=[(ici == Ici::Accueil).then_some("page")] { (icons::sparkles()) span { (tr.shop_tab_featured) } }
                    @for c in Catalogue::ALL {
                        a href=(p.boutique(c.slug())) data-onglet aria-current=[(ici == Ici::Categorie(c)).then_some("page")] { (c.icone()) span { (c.nom(tr)) } }
                    }
                }
                div.sx-tools {
                    label.sx-search {
                        (icons::search())
                        input #sx-q type="search" placeholder=(tr.shop_search) aria-label=(tr.shop_search) autocomplete="off" spellcheck="false";
                        kbd { "/" }
                    }
                    a.sx-wishbtn href=(p.boutique("wishlist")) data-onglet aria-current=[(ici == Ici::Wishlist).then_some("page")] aria-label=(tr.shop_wishlist) title=(tr.shop_wishlist) {
                        (icons::heart())
                        span.sx-wish-n hidden { "0" }
                    }
                }
            }
        }
        div.sx-toast #sx-toast role="status" aria-live="polite" hidden {}
    }
}

/// Une carte : l'image et son étiquette, le cœur de la wishlist, le nom et le statut. Toute la
/// carte ouvre la page de l'article.
fn carte(p: &Page, o: &Objet, ordre: usize) -> Markup {
    let tr = p.tr;
    let s = o.statut();
    let nom = o.nom(tr);
    html! {
        div.sx-card data-cle=(o.cle) data-nom=(nom.to_lowercase()) data-ordre=(ordre) data-dispo=(if s.disponible() { "1" } else { "0" }) data-cat=(o.cat.slug()) {
            a.sx-link href=(p.boutique(&format!("item/{}", o.cle))) aria-label=(nom) {}
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
            button.sx-heart type="button" data-wish=(o.cle) aria-pressed="false" aria-label=(tr.shop_wish_add) title=(tr.shop_wish_add) { (icons::heart()) }
            span.sx-foot {
                strong { (nom) }
                span.sx-state.(s.classe()) { @if s.disponible() { (icons::check()) } (s.badge(tr)) }
            }
        }
    }
}

/// Un titre de section, en police pixel, avec sa phrase et un lien « Tout voir ».
fn titre_section(p: &Page, titre: &str, phrase: Option<&str>, tout: Option<&str>) -> Markup {
    html! {
        div.sx-row-head {
            div {
                h2.px { (titre) }
                @if let Some(s) = phrase { p { (s) } }
            }
            @if let Some(lien) = tout { a.sx-all href=(lien) data-onglet { (p.tr.shop_view_all) (icons::arrow_right()) } }
        }
    }
}

fn page_boutique(p: &Page, titre: &str, description: &str, main: Markup, item: bool) -> Markup {
    let scripts: &[&str] = if item { &["/vendor/skinview3d/skinview3d.bundle.js", "/js/boutique.js"] } else { &["/js/boutique.js"] };
    layout::render(
        p,
        Head { title: titre, description, jsonld: None, css: &["home", "patch-notes", "shop"], scripts, body_class: "shop" },
        main,
    )
}

/// Toutes les cartes, cachées : boutique.js y prend celles des articles vus récemment et de la
/// wishlist.
fn reserve(p: &Page) -> Markup {
    html! { div.sx-pool hidden { @for (i, o) in objets().iter().enumerate() { (carte(p, o, i)) } } }
}

/// L'accueil de la boutique.
pub fn render(p: &Page) -> Markup {
    let tr = p.tr;
    let tous = objets();
    let main = html! {
        (barre(p, Ici::Accueil))
        div #sx-page data-titre=(tr.shop_title) {
            // Le carrousel : les diapositives voisines dépassent sur les côtés
            section.sx-hero {
                div.col {
                    div.sx-carousel #sx-carousel aria-roledescription="carousel" {
                        div.sx-track {
                            @for (i, (cle, image)) in CARROUSEL.iter().enumerate() {
                                @if let Some(o) = objet(cle) {
                                    @let s = o.statut();
                                    a.sx-slide href=(p.boutique(&format!("item/{cle}"))) aria-roledescription="slide" aria-label=(format!("{} / {}", i + 1, CARROUSEL.len())) {
                                        img src=(photo(image)) alt="" width="600" height="475" loading=(if i == 0 { "eager" } else { "lazy" });
                                        span.sx-slide-txt {
                                            span.sx-kicker.(s.classe()) { (s.badge(tr)) " · " (o.cat.nom(tr)) }
                                            strong.px { (o.nom(tr)) }
                                            span.sx-slide-p { (o.texte(tr)) }
                                            span.btn.play.sm { (tr.shop_try_on) (icons::arrow_right()) }
                                        }
                                    }
                                }
                            }
                        }
                        button.sx-nav.prev type="button" aria-label=(tr.shop_prev) { (icons::arrow_left()) }
                        button.sx-nav.next type="button" aria-label=(tr.shop_next) { (icons::arrow_right()) }
                        div.sx-dots role="tablist" {
                            @for i in 0..CARROUSEL.len() {
                                button type="button" role="tab" aria-label=(format!("{} / {}", i + 1, CARROUSEL.len())) aria-selected=(if i == 0 { "true" } else { "false" }) {}
                            }
                        }
                    }
                }
            }
            div.col.sx-body {
                // Que cherches-tu ? Les catégories en mosaïque
                section.sx-row.sx-bento-sec {
                    (titre_section(p, &tr.shop_looking, Some(&tr.shop_looking_p), None))
                    div.sx-bento {
                        @for (c, classe) in [(Catalogue::Capes, "b-capes"), (Catalogue::Emotes, "b-emotes"), (Catalogue::Hats, "b-hats"), (Catalogue::Pets, "b-pets")] {
                            @let n = tous.iter().filter(|o| o.cat == c).count();
                            a.sx-tile.(classe) href=(p.boutique(c.slug())) data-onglet {
                                span.sx-tile-txt { strong.px { (c.nom(tr)) } span { (icons::layers()) (n) } }
                                img src=(photo(c.image())) alt="" width="600" height="475" loading="lazy";
                            }
                        }
                        a.sx-tile.b-all href=(p.boutique("capes")) data-onglet {
                            span.sx-fan aria-hidden="true" {
                                @for image in ["elytres-mistly", "cloudly", "cape-mistly"] {
                                    img src=(photo(image)) alt="" width="600" height="475" loading="lazy";
                                }
                            }
                            span.sx-tile-txt { strong.px { (tr.shop_all_cosmetics) } span { (icons::layers()) (tous.len()) } }
                            (icons::arrow_right())
                        }
                    }
                }
                // Populaires : une rangée qui défile, avec des flèches
                section.sx-row {
                    (titre_section(p, &tr.shop_popular, Some(&tr.shop_popular_p), None))
                    div.sx-scroller {
                        button.sx-nav.prev type="button" aria-label=(tr.shop_prev) { (icons::arrow_left()) }
                        div.sx-strip { @for (i, o) in liste(&POPULAIRES).iter().enumerate() { (carte(p, o, i)) } }
                        button.sx-nav.next type="button" aria-label=(tr.shop_next) { (icons::arrow_right()) }
                    }
                }
                section.sx-row {
                    (titre_section(p, &tr.shop_new_t, None, None))
                    div.sx-grid.cinq { @for (i, o) in liste(&NOUVEAUTES).iter().enumerate() { (carte(p, o, i)) } }
                }
                // Vus récemment : rempli par boutique.js, caché tant qu'il est vide
                section.sx-row.sx-recent hidden {
                    (titre_section(p, &tr.shop_recent, None, None))
                    div.sx-grid {}
                }
                p.sx-empty hidden { (tr.shop_no_result) }
                (reserve(p))
            }
        }
    };
    page_boutique(p, &tr.shop_title, &tr.shop_sub, main, false)
}

/// Le haut d'une page de liste (catégorie, wishlist) : le titre en police pixel et le nombre.
fn entete_liste(titre: &str, n: usize) -> Markup {
    html! {
        div.sx-pagehead {
            h1.px { (titre) }
            span.sx-count { (icons::layers()) span.n { (n) } }
        }
    }
}

/// Une catégorie de la boutique : ses cartes, à trier et à filtrer.
pub fn render_catalogue(p: &Page, cat: Catalogue) -> Markup {
    let tr = p.tr;
    let liste: Vec<Objet> = objets().into_iter().filter(|o| o.cat == cat).collect();
    let titre = format!("{} – {}", cat.nom(tr), tr.shop_title);
    let main = html! {
        (barre(p, Ici::Categorie(cat)))
        div #sx-page data-titre=(titre) {
            div.col.sx-body {
                (entete_liste(cat.nom(tr), liste.len()))
                div.sx-toolbar {
                    div.sx-chips role="group" {
                        button type="button" data-filtre="all" aria-pressed="true" { (tr.shop_filter_all) }
                        button type="button" data-filtre="1" aria-pressed="false" { (icons::check()) (tr.shop_available) }
                        button type="button" data-filtre="0" aria-pressed="false" { (tr.shop_badge) }
                    }
                    label.sx-sort {
                        span { (tr.shop_sort) }
                        select #sx-sort {
                            option value="reco" { (tr.shop_sort_reco) }
                            option value="az" { (tr.shop_sort_az) }
                            option value="dispo" { (tr.shop_sort_avail) }
                        }
                    }
                }
                section.sx-row.sx-list {
                    div.sx-grid { @for (i, o) in liste.iter().enumerate() { (carte(p, o, i)) } }
                }
                p.sx-empty hidden { (tr.shop_no_result) }
            }
        }
    };
    page_boutique(p, &titre, &tr.shop_mega_p, main, false)
}

/// La wishlist : les cartes des articles gardés (boutique.js les montre d'après le navigateur).
pub fn render_wishlist(p: &Page) -> Markup {
    let tr = p.tr;
    let titre = format!("{} – {}", tr.shop_wishlist, tr.shop_title);
    let main = html! {
        (barre(p, Ici::Wishlist))
        div #sx-page data-titre=(titre) {
            div.col.sx-body {
                (entete_liste(&tr.shop_wishlist, 0))
                section.sx-row.sx-wishlist {
                    div.sx-grid { @for (i, o) in objets().iter().enumerate() { (carte(p, o, i)) } }
                }
                div.sx-wish-empty hidden {
                    span.ico { (icons::heart()) }
                    p { (tr.shop_wish_empty) }
                    a.btn.play href=(p.boutique("")) data-onglet { (icons::sparkles()) span { (tr.shop_tab_featured) } }
                }
            }
        }
    };
    page_boutique(p, &titre, &tr.shop_wish_empty, main, false)
}

/// La page d'un article : l'aperçu 3D (porté d'office) et ses réglages, la fiche, puis des
/// articles proches.
pub fn render_objet(p: &Page, o: &Objet, player: Option<&str>, value: &str, invalid: bool) -> Markup {
    let tr = p.tr;
    let s = o.statut();
    let tous = objets();
    let i = tous.iter().position(|x| x.cle == o.cle).unwrap_or(0);
    let precedent = tous[(i + tous.len() - 1) % tous.len()];
    let suivant = tous[(i + 1) % tous.len()];
    let proches: Vec<Objet> = {
        let mut l: Vec<Objet> = tous.iter().filter(|x| x.cat == o.cat && x.cle != o.cle).copied().collect();
        l.extend(tous.iter().filter(|x| x.cat != o.cat).copied());
        l.truncate(4);
        l
    };
    let ici = p.boutique(&format!("item/{}", o.cle));
    let main = html! {
        (barre(p, Ici::Categorie(o.cat)))
        section.studio.sx-item data-catalogue=(o.cat.slug()) data-item=(o.cle) {
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
                    h1.px { (o.nom(tr)) }
                    span.sx-badge.(s.classe()) { (s.badge(tr)) }
                    div.sx-step {
                        a href=(p.boutique(&format!("item/{}", precedent.cle))) aria-label=(format!("{} : {}", tr.shop_prev, precedent.nom(tr))) title=(precedent.nom(tr)) { (icons::arrow_left()) }
                        a href=(p.boutique(&format!("item/{}", suivant.cle))) aria-label=(format!("{} : {}", tr.shop_next, suivant.nom(tr))) title=(suivant.nom(tr)) { (icons::arrow_right()) }
                    }
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
                        div.sx-actions {
                            button.btn.sx-wishbig type="button" data-wish=(o.cle) aria-pressed="false" { (icons::heart()) span { (tr.shop_wish_add) } }
                            button.btn.sx-share type="button" data-share=(o.nom(tr)) aria-label=(tr.shop_share) title=(tr.shop_share) { (icons::copy()) span { (tr.shop_share) } }
                        }
                    }
                }
                section.sx-row {
                    (titre_section(p, &tr.shop_more, None, Some(&p.boutique(o.cat.slug()))))
                    div.sx-grid { @for (i, x) in proches.iter().enumerate() { (carte(p, x, i)) } }
                }
            }
        }
    };
    let titre = format!("{} – {}", o.nom(tr), tr.shop_title);
    page_boutique(p, &titre, o.texte(tr), main, true)
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
