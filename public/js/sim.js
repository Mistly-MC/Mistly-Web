// Le launcher en haut de l'accueil : cliquer un onglet (sur la capture ou dans les pastilles)
// affiche la capture de cet écran. Rien ne fonctionne vraiment : c'est pour voir le design.
// Tant que personne n'a cliqué, les onglets défilent tout seuls (une barre se remplit sous
// l'onglet actif), pour montrer qu'on peut cliquer. Chaque changement passe derrière un nuage
// en pixels qui traverse l'écran de gauche à droite.
(() => {
  const sim = document.querySelector("[data-sim]");
  if (!sim) return;
  const ecran = sim.querySelector(".sim-screen");
  const vues = [...sim.querySelectorAll("[data-view]")];
  const zones = [...sim.querySelectorAll(".sim-hot")];
  const pastilles = [...sim.querySelectorAll(".sim-tabs [data-go]")];
  const legende = sim.querySelector(".sim-cap");
  const toile = sim.querySelector(".sim-nuage");
  const calme = matchMedia("(prefers-reduced-motion: reduce)").matches;
  const DUREE = 3500;

  // Les autres captures se chargent dès qu'on approche, pour que le changement soit immédiat
  let precharge = false;
  const precharger = () => {
    if (precharge) return;
    precharge = true;
    sim.querySelectorAll("img[loading=lazy]").forEach(i => { i.loading = "eager"; });
  };
  sim.addEventListener("pointerenter", precharger);
  sim.addEventListener("focusin", precharger);

  // ---- Transition : un nuage en pixels traverse l'écran ----
  const DUREE_BALAYAGE = 900;
  let balayage = null; // { sortante, image } pendant une transition

  const finirBalayage = () => {
    if (!balayage) return;
    cancelAnimationFrame(balayage.image);
    balayage.sortante.classList.remove("sim-sortante");
    balayage.sortante.style.clipPath = "";
    balayage.sortante.hidden = true;
    toile.getContext("2d").clearRect(0, 0, toile.width, toile.height);
    balayage = null;
  };

  const balayer = sortante => {
    // d'abord sortir l'ancien écran du flux, sinon les deux s'empilent et faussent la hauteur
    sortante.classList.add("sim-sortante");
    const ctx = toile.getContext("2d");
    const ratio = devicePixelRatio || 1;
    const l = ecran.clientWidth, h = ecran.clientHeight;
    toile.width = Math.round(l * ratio);
    toile.height = Math.round(h * ratio);
    ctx.setTransform(ratio, 0, 0, ratio, 0, 0);
    // nuage blanc et lumineux le jour, bleuté la nuit
    const fond = (getComputedStyle(sim).getPropertyValue("--bg-rgb").trim().split(/\s+/).map(Number));
    const jour = fond.length === 3 && fond[0] + fond[1] + fond[2] > 380;
    const [clair, ombre, bord] = jour
      ? ["255,255,255", "196,211,238", "226,234,249"]
      : ["226,234,250", "150,170,214", "200,214,242"];
    // gros pixels, comme la lune et le paysage du launcher
    const p = Math.max(4, Math.round(l / 120));
    const cols = Math.ceil(l / p), lignes = Math.ceil(h / p);
    // le nuage : des boules qui se chevauchent, en colonne du haut en bas de l'écran (en pixels
    // d'écran, centrées sur le front qui avance)
    const boules = [];
    for (let y = -0.08 * h; y < h * 1.1; y += l * (0.1 + Math.random() * 0.04)) {
      boules.push({ x: (Math.random() - 0.5) * l * 0.06, y, r: l * (0.1 + Math.random() * 0.05) });
      // une boule plus petite qui dépasse d'un côté ou de l'autre : les bosses du nuage
      const cote = Math.random() < 0.5 ? -1 : 1;
      boules.push({ x: cote * l * (0.1 + Math.random() * 0.06), y: y + l * (0.02 + Math.random() * 0.04), r: l * (0.055 + Math.random() * 0.035) });
    }
    const etendue = Math.max(...boules.map(b => Math.abs(b.x) + b.r));
    const LARGEUR = (2 * etendue) / l;

    const debut = performance.now();
    balayage = { sortante };
    const pas = maintenant => {
      const t = Math.min((maintenant - debut) / DUREE_BALAYAGE, 1);
      const e = t < 0.5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2; // ease-in-out
      const front = (-LARGEUR / 2 + e * (1 + LARGEUR)) * l; // centre du nuage, en pixels
      // l'écran qui part n'est plus visible à gauche du centre du nuage
      sortante.style.clipPath = `inset(0 0 0 ${Math.max(0, Math.min(1, front / l)) * 100}%)`;
      ctx.clearRect(0, 0, l, h);
      const c0 = Math.max(0, Math.floor((front - etendue) / p)), c1 = Math.min(cols, Math.ceil((front + etendue) / p));
      for (let r = 0; r < lignes; r++) {
        const cy = (r + 0.5) * p;
        for (let c = c0; c < c1; c++) {
          const cx = (c + 0.5) * p - front;
          // profondeur dans la boule la plus profonde, et position verticale dans chaque boule
          let profondeur = -1, v = 0;
          for (const b of boules) {
            const d = Math.hypot(cx - b.x, cy - b.y);
            if (d > b.r || b.r - d < profondeur) continue;
            profondeur = b.r - d;
            v = (cy - b.y) / b.r; // -1 en haut de la boule, 1 en bas
          }
          if (profondeur < 0) continue;
          // chaque bosse : le haut éclairé, un croissant d'ombre en bas, le bord en damier
          const couleur = profondeur < p ? ((c + r) % 2 ? (v < 0.2 ? clair : bord) : null) : v > 0.42 ? ombre : v > 0.3 ? ((c + r) % 2 ? ombre : bord) : clair;
          if (!couleur) continue;
          ctx.fillStyle = `rgb(${couleur})`;
          ctx.fillRect(c * p, r * p, p, p);
        }
      }
      if (t < 1) balayage.image = requestAnimationFrame(pas);
      else finirBalayage();
    };
    balayage.image = requestAnimationFrame(pas);
  };

  let actuelle = vues[0].dataset.view;
  const afficher = nom => {
    const vue = vues.find(v => v.dataset.view === nom);
    if (!vue || nom === actuelle) return;
    finirBalayage();
    const ancienne = vues.find(v => v.dataset.view === actuelle);
    actuelle = nom;
    vues.forEach(v => { v.hidden = v !== vue && v !== ancienne; });
    vue.hidden = false;
    if (calme || !ancienne) {
      if (ancienne) ancienne.hidden = true;
    } else {
      balayer(ancienne);
    }
    zones.forEach(z => {
      const sur = z.dataset.on;
      z.hidden = sur !== "" && !sur.split(" ").includes(nom);
    });
    pastilles.forEach(b => b.setAttribute("aria-selected", String(b.dataset.go === nom)));
    legende.querySelector("strong").textContent = vue.dataset.t;
    legende.querySelector("span").textContent = vue.dataset.p;
  };

  // ---- Défilement automatique ----
  const ordre = pastilles.map(b => b.dataset.go);
  let auto = !calme;
  let visible = false, minuteur = null, reste = DUREE, debut = 0;

  const arreterMinuteur = () => {
    if (minuteur) { clearTimeout(minuteur); minuteur = null; reste -= performance.now() - debut; }
  };
  const suivant = () => {
    minuteur = null;
    const i = ordre.indexOf(actuelle);
    afficher(ordre[(i + 1) % ordre.length]);
    reste = DUREE;
    // relance la barre de progression de l'onglet qui vient de s'activer
    sim.classList.remove("auto");
    void sim.offsetWidth;
    sim.classList.add("auto");
    maj();
  };
  const maj = () => {
    if (!auto) return;
    const tourne = visible && !document.hidden;
    sim.classList.toggle("en-pause", !tourne);
    if (tourne && !minuteur) {
      precharger();
      debut = performance.now();
      minuteur = setTimeout(suivant, Math.max(reste, 0));
    } else if (!tourne) {
      arreterMinuteur();
    }
  };

  if (auto) {
    sim.style.setProperty("--sim-duree", DUREE + "ms");
    sim.classList.add("auto", "en-pause");
    new IntersectionObserver(e => { visible = e[0].isIntersecting; maj(); }, { threshold: 0.4 }).observe(sim);
    document.addEventListener("visibilitychange", maj);
  }

  sim.addEventListener("click", e => {
    const b = e.target.closest("[data-go]");
    if (!b) return;
    // la personne a pris la main : plus de défilement automatique
    if (auto) {
      auto = false;
      arreterMinuteur();
      sim.classList.remove("auto", "en-pause");
    }
    afficher(b.dataset.go);
  });
})();
