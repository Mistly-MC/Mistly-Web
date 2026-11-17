// Le launcher en haut de l'accueil : cliquer un onglet (sur la capture ou dans les pastilles)
// affiche la capture de cet écran. Rien ne fonctionne vraiment : c'est pour voir le design.
// Tant que personne n'a cliqué, les onglets défilent tout seuls (une barre se remplit sous
// l'onglet actif), pour montrer qu'on peut cliquer. Chaque changement passe par une bande de
// caractères ASCII qui balaie l'écran de gauche à droite.
(() => {
  const sim = document.querySelector("[data-sim]");
  if (!sim) return;
  const ecran = sim.querySelector(".sim-screen");
  const vues = [...sim.querySelectorAll("[data-view]")];
  const zones = [...sim.querySelectorAll(".sim-hot")];
  const pastilles = [...sim.querySelectorAll(".sim-tabs [data-go]")];
  const legende = sim.querySelector(".sim-cap");
  const toile = sim.querySelector(".sim-ascii");
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

  // ---- Transition ASCII ----
  const SIGNES = " .:-=+*#%@";
  const DUREE_BALAYAGE = 750;
  const BANDE = 0.22; // largeur de la bande, en part de la largeur de l'écran
  let balayage = null; // { sortante, fin } pendant une transition

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
    const style = getComputedStyle(sim);
    const ascii = style.getPropertyValue("--ascii-rgb").trim().split(/\s+/).join(",") || "206,218,245";
    const fond = style.getPropertyValue("--bg-rgb").trim().split(/\s+/).join(",") || "9,9,11";
    const police = style.getPropertyValue("--mono").trim() || "monospace";
    // taille des caractères selon la largeur de l'écran (plus petits sur téléphone)
    const cl = Math.max(6, Math.round(l / 110)), ch = Math.round(cl * 1.7);
    const cols = Math.ceil(l / cl), lignes = Math.ceil(h / ch);
    ctx.font = `${Math.round(ch * 0.82)}px ${police}`;
    ctx.textBaseline = "top";
    // un peu de désordre par ligne, pour un bord qui n'est pas tout droit
    const decal = Array.from({ length: lignes }, () => (Math.random() - 0.5) * 0.06);

    const debut = performance.now();
    balayage = { sortante };
    const pas = maintenant => {
      const t = Math.min((maintenant - debut) / DUREE_BALAYAGE, 1);
      const e = t < 0.5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2; // ease-in-out
      const front = -BANDE + e * (1 + 2 * BANDE); // centre de la bande, de -BANDE à 1 + BANDE
      // l'écran qui part n'est plus visible à gauche du front
      sortante.style.clipPath = `inset(0 0 0 ${Math.max(0, Math.min(1, front)) * 100}%)`;
      ctx.clearRect(0, 0, l, h);
      for (let r = 0; r < lignes; r++) {
        const fr = front + decal[r];
        const x0 = Math.max(0, Math.floor((fr - BANDE / 2) * cols));
        const x1 = Math.min(cols, Math.ceil((fr + BANDE / 2) * cols));
        for (let c = x0; c < x1; c++) {
          // 1 au centre de la bande, 0 sur ses bords
          const d = 1 - Math.abs((c + 0.5) / cols - fr) / (BANDE / 2);
          if (d <= 0) continue;
          ctx.fillStyle = `rgba(${fond},${Math.min(1, d * 1.6) * 0.92})`;
          ctx.fillRect(c * cl, r * ch, cl + 0.5, ch + 0.5);
          const s = SIGNES[Math.min(SIGNES.length - 1, Math.floor((d * 0.7 + Math.random() * 0.5) * SIGNES.length))];
          if (s === " ") continue;
          ctx.fillStyle = `rgba(${ascii},${0.35 + d * 0.65})`;
          ctx.fillText(s, c * cl, r * ch);
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
