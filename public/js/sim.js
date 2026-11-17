// Aperçu du launcher sur l'accueil : cliquer un onglet (sur la capture ou dans les pastilles)
// affiche la capture de cet écran. Rien ne fonctionne vraiment : c'est pour voir le design.
// Tant que personne n'a cliqué, les onglets défilent tout seuls (une barre se remplit sous
// l'onglet actif), pour montrer qu'on peut cliquer.
(() => {
  const sim = document.querySelector("[data-sim]");
  if (!sim) return;
  const vues = [...sim.querySelectorAll("[data-view]")];
  const zones = [...sim.querySelectorAll(".sim-hot")];
  const pastilles = [...sim.querySelectorAll(".sim-tabs [data-go]")];
  const legende = sim.querySelector(".sim-cap");
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

  let actuelle = vues[0].dataset.view;
  const afficher = nom => {
    const vue = vues.find(v => v.dataset.view === nom);
    if (!vue) return;
    actuelle = nom;
    vues.forEach(v => { v.hidden = v !== vue; });
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
  let auto = !matchMedia("(prefers-reduced-motion: reduce)").matches;
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
    const tourne = auto && visible && !document.hidden;
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
