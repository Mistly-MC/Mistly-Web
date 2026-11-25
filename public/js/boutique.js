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
// Boutique : la barre (pastille qui glisse sur l'onglet choisi, contenu remplacé sans recharger),
// la recherche, la wishlist et les articles vus récemment (gardés dans le navigateur), le
// carrousel, les rangées qui défilent, le tri et les filtres d'une catégorie, et le partage.
(() => {
  "use strict";
  const barre = document.getElementById("sx-bar");
  if (!barre) return;
  const calme = matchMedia("(prefers-reduced-motion: reduce)").matches;
  const textes = barre.dataset;

  // ---------- Stockage (le navigateur seulement) ----------
  const lire = (cle) => { try { return JSON.parse(localStorage.getItem(cle)) || []; } catch { return []; } };
  const ecrire = (cle, v) => { try { localStorage.setItem(cle, JSON.stringify(v)); } catch {} };
  const WISH = "mistly-wishlist", VUS = "mistly-vus";

  // ---------- Petit message en bas de l'écran ----------
  const toast = document.getElementById("sx-toast");
  let minuteur = 0;
  const annoncer = (texte) => {
    toast.textContent = texte;
    toast.hidden = false;
    requestAnimationFrame(() => toast.classList.add("on"));
    clearTimeout(minuteur);
    minuteur = setTimeout(() => { toast.classList.remove("on"); setTimeout(() => { toast.hidden = true; }, 250); }, 2200);
  };

  // ---------- Wishlist ----------
  const compteur = barre.querySelector(".sx-wish-n");
  const majWishlist = (racine = document) => {
    const liste = lire(WISH);
    compteur.textContent = liste.length;
    compteur.hidden = liste.length === 0;
    racine.querySelectorAll("[data-wish]").forEach(b => {
      const on = liste.includes(b.dataset.wish);
      b.setAttribute("aria-pressed", String(on));
      const libelle = on ? textes.remove : textes.add;
      b.setAttribute("aria-label", libelle);
      b.title = libelle;
      const span = b.querySelector("span");
      if (span) span.textContent = libelle;
    });
    // la page de la wishlist : seulement les articles gardés
    const page = document.querySelector(".sx-wishlist");
    if (page) {
      let n = 0;
      page.querySelectorAll(".sx-card").forEach(c => { const ok = liste.includes(c.dataset.cle); c.hidden = !ok; if (ok) n++; });
      page.hidden = n === 0;
      document.querySelector(".sx-wish-empty").hidden = n > 0;
      const nb = document.querySelector(".sx-pagehead .n");
      if (nb) nb.textContent = n;
    }
  };
  document.addEventListener("click", e => {
    const b = e.target.closest("[data-wish]");
    if (!b) return;
    e.preventDefault();
    const cle = b.dataset.wish;
    let liste = lire(WISH);
    const ajout = !liste.includes(cle);
    liste = ajout ? [...liste, cle] : liste.filter(x => x !== cle);
    ecrire(WISH, liste);
    if (ajout && !calme) { b.classList.remove("pop"); void b.offsetWidth; b.classList.add("pop"); }
    compteur.classList.remove("bump"); void compteur.offsetWidth; compteur.classList.add("bump");
    annoncer(ajout ? textes.added : textes.removed);
    majWishlist();
  });

  // ---------- Partager : copie l'adresse de la page ----------
  document.addEventListener("click", async e => {
    const b = e.target.closest("[data-share]");
    if (!b) return;
    const url = location.href.split("?")[0];
    if (navigator.share && matchMedia("(pointer: coarse)").matches) {
      try { await navigator.share({ title: b.dataset.share, url }); return; } catch {}
    }
    try { await navigator.clipboard.writeText(url); annoncer(textes.copied); } catch {}
  });

  // ---------- Onglets : la pastille glisse, puis le contenu change sans recharger ----------
  const onglets = barre.querySelector(".sx-tabs");
  const pastille = onglets.querySelector(".sx-pill");
  const placer = (a, anime) => {
    if (!a) { pastille.style.opacity = "0"; return; }
    if (!anime) pastille.style.transition = "none";
    pastille.style.opacity = "1";
    pastille.style.width = a.offsetWidth + "px";
    pastille.style.transform = `translateX(${a.offsetLeft}px)`;
    if (!anime) { void pastille.offsetWidth; pastille.style.transition = ""; }
  };
  const ongletActif = () => onglets.querySelector("a[aria-current]");
  placer(ongletActif(), false);
  onglets.classList.add("ready");
  addEventListener("resize", () => placer(ongletActif(), false));
  // au survol, la pastille suit la souris puis revient
  onglets.querySelectorAll("a").forEach(a => {
    a.addEventListener("mouseenter", () => { onglets.classList.add("survol"); placer(a, true); });
  });
  onglets.addEventListener("mouseleave", () => { onglets.classList.remove("survol"); placer(ongletActif(), true); });

  const memeSite = (a) => a.origin === location.origin;
  let chargement = null;
  const aller = async (url, pousser) => {
    const page = document.getElementById("sx-page");
    if (!page) { location.href = url; return; }
    // l'onglet visé s'allume tout de suite
    const vise = [...barre.querySelectorAll("[data-onglet]")].find(a => a.href.split("#")[0] === url.split("#")[0]);
    barre.querySelectorAll("[aria-current]").forEach(a => a.removeAttribute("aria-current"));
    if (vise) vise.setAttribute("aria-current", "page");
    placer(onglets.contains(vise) ? vise : null, true);
    page.classList.add("sortie");
    const moi = chargement = fetch(url).then(r => (r.ok ? r.text() : Promise.reject())).catch(() => null);
    const [html] = await Promise.all([moi, new Promise(r => setTimeout(r, calme ? 0 : 200))]);
    if (moi !== chargement) return;
    const doc = html && new DOMParser().parseFromString(html, "text/html");
    const neuve = doc?.getElementById("sx-page");
    if (!neuve) { location.href = url; return; }
    page.replaceWith(neuve);
    document.title = neuve.dataset.titre || doc.title;
    if (pousser) history.pushState({ boutique: true }, "", url);
    scrollTo({ top: 0, behavior: calme ? "auto" : "smooth" });
    neuve.classList.add("entree");
    requestAnimationFrame(() => requestAnimationFrame(() => neuve.classList.remove("entree")));
    brancher(neuve);
  };
  document.addEventListener("click", e => {
    const a = e.target.closest("a[data-onglet]");
    if (!a || e.metaKey || e.ctrlKey || e.shiftKey || e.button !== 0 || !memeSite(a)) return;
    if (!document.getElementById("sx-page")) return;   // page d'un article : navigation normale
    e.preventDefault();
    aller(a.href, true);
  });
  addEventListener("popstate", () => { if (document.getElementById("sx-page")) aller(location.href, false); });

  // ---------- Recherche : filtre les cartes pendant la frappe ----------
  const champ = document.getElementById("sx-q");
  const simple = t => t.normalize("NFD").replace(/[̀-ͯ]/g, "").toLowerCase().trim();
  const filtrer = () => {
    const page = document.getElementById("sx-page");
    if (!page) return;
    const q = simple(champ.value);
    page.classList.toggle("recherche", q !== "");
    let total = 0;
    page.querySelectorAll(".sx-row").forEach(r => {
      if (r.classList.contains("sx-wishlist")) return;
      const cartes = [...r.querySelectorAll(".sx-card")];
      if (!cartes.length) { r.classList.toggle("masque", q !== ""); return; }
      let n = 0;
      cartes.forEach(c => {
        const ok = !q || simple(c.dataset.nom || "").includes(q);
        c.classList.toggle("masque", !ok);
        if (ok) n++;
      });
      r.classList.toggle("masque", n === 0);
      total += n;
    });
    const vide = page.querySelector(".sx-empty");
    if (vide) vide.hidden = total > 0 || !q;
    page.querySelector(".sx-hero")?.classList.toggle("masque", q !== "");
  };
  champ?.addEventListener("input", filtrer);
  addEventListener("keydown", e => {
    if (e.key === "/" && champ && document.activeElement !== champ && !/input|textarea|select/i.test(document.activeElement?.tagName || "")) {
      e.preventDefault();
      champ.focus();
    }
    if (e.key === "Escape" && document.activeElement === champ) { champ.value = ""; filtrer(); champ.blur(); }
  });

  // ---------- Ce qui dépend de la page (branché à nouveau après chaque changement d'onglet) ----------
  const brancher = (racine) => {
    majWishlist(racine);
    if (champ?.value) filtrer();

    // Vus récemment : des copies des cartes de la réserve
    const recents = racine.querySelector(".sx-recent");
    if (recents) {
      const vus = lire(VUS).slice(0, 5);
      const grille = recents.querySelector(".sx-grid");
      grille.replaceChildren(...vus.map(cle => racine.querySelector(`.sx-pool .sx-card[data-cle="${cle}"]`)?.cloneNode(true)).filter(Boolean));
      recents.hidden = grille.children.length === 0;
      majWishlist(recents);
    }

    // Le carrousel : flèches, points, glisser au doigt, défilement tout seul (en pause au survol)
    const carrousel = racine.querySelector("#sx-carousel");
    if (carrousel) {
      const piste = carrousel.querySelector(".sx-track");
      const slides = [...piste.children];
      const points = [...carrousel.querySelectorAll(".sx-dots button")];
      let i = 0, auto = 0;
      const aller = (n) => {
        i = (n + slides.length) % slides.length;
        piste.style.setProperty("--i", i);
        slides.forEach((s, k) => { s.classList.toggle("on", k === i); s.tabIndex = k === i ? 0 : -1; });
        points.forEach((p, k) => p.setAttribute("aria-selected", String(k === i)));
      };
      const demarrer = () => { if (!calme) { clearInterval(auto); auto = setInterval(() => aller(i + 1), 6500); } };
      carrousel.querySelector(".prev").addEventListener("click", () => { aller(i - 1); demarrer(); });
      carrousel.querySelector(".next").addEventListener("click", () => { aller(i + 1); demarrer(); });
      points.forEach((p, k) => p.addEventListener("click", () => { aller(k); demarrer(); }));
      // un clic sur une diapositive voisine la met au centre au lieu d'ouvrir l'article
      slides.forEach((s, k) => s.addEventListener("click", e => { if (k !== i) { e.preventDefault(); aller(k); demarrer(); } }));
      carrousel.addEventListener("mouseenter", () => clearInterval(auto));
      carrousel.addEventListener("mouseleave", demarrer);
      let x0 = null;
      piste.addEventListener("pointerdown", e => { x0 = e.clientX; });
      piste.addEventListener("pointerup", e => {
        if (x0 === null) return;
        const dx = e.clientX - x0; x0 = null;
        if (Math.abs(dx) > 50) { aller(i + (dx < 0 ? 1 : -1)); demarrer(); }
      });
      carrousel.addEventListener("keydown", e => {
        if (e.key === "ArrowLeft") { aller(i - 1); demarrer(); }
        if (e.key === "ArrowRight") { aller(i + 1); demarrer(); }
      });
      aller(0);
      demarrer();
    }

    // Les rangées qui défilent : les flèches avancent d'une page, et s'éteignent au bout
    racine.querySelectorAll(".sx-scroller").forEach(sc => {
      const bande = sc.querySelector(".sx-strip");
      const [g, d] = sc.querySelectorAll(".sx-nav");
      const maj = () => {
        g.disabled = bande.scrollLeft < 4;
        d.disabled = bande.scrollLeft + bande.clientWidth > bande.scrollWidth - 4;
      };
      g.addEventListener("click", () => bande.scrollBy({ left: -bande.clientWidth * .9, behavior: calme ? "auto" : "smooth" }));
      d.addEventListener("click", () => bande.scrollBy({ left: bande.clientWidth * .9, behavior: calme ? "auto" : "smooth" }));
      bande.addEventListener("scroll", maj, { passive: true });
      maj();
    });

    // Une catégorie : filtres (disponibles ou bientôt) et tri
    const grille = racine.querySelector(".sx-list .sx-grid");
    if (grille) {
      const cartes = [...grille.children];
      let filtre = "all";
      const appliquer = () => {
        const tri = racine.querySelector("#sx-sort").value;
        const ordre = [...cartes].sort((a, b) =>
          tri === "az" ? a.dataset.nom.localeCompare(b.dataset.nom)
          : tri === "dispo" ? (b.dataset.dispo - a.dataset.dispo) || (a.dataset.ordre - b.dataset.ordre)
          : a.dataset.ordre - b.dataset.ordre);
        let n = 0;
        ordre.forEach(c => {
          grille.append(c);
          const ok = filtre === "all" || c.dataset.dispo === filtre;
          c.hidden = !ok;
          if (ok) n++;
        });
        const nb = racine.querySelector(".sx-pagehead .n");
        if (nb) nb.textContent = n;
        // une petite entrée des cartes
        if (!calme) grille.animate([{ opacity: .4, transform: "translateY(6px)" }, { opacity: 1, transform: "none" }], { duration: 260, easing: "ease-out" });
      };
      racine.querySelectorAll("[data-filtre]").forEach(b => b.addEventListener("click", () => {
        filtre = b.dataset.filtre;
        racine.querySelectorAll("[data-filtre]").forEach(x => x.setAttribute("aria-pressed", String(x === b)));
        appliquer();
      }));
      racine.querySelector("#sx-sort")?.addEventListener("change", appliquer);
    }
  };

  // ---------- La page d'un article : retenue dans « Vus récemment » ----------
  const article = document.querySelector("[data-item]");
  if (article) ecrire(VUS, [article.dataset.item, ...lire(VUS).filter(x => x !== article.dataset.item)].slice(0, 12));

  brancher(document);
})();
