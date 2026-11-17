// Aperçu du launcher sur l'accueil : cliquer un onglet (sur la capture ou dans les pastilles)
// affiche la capture de cet écran. Rien ne fonctionne vraiment : c'est pour voir le design.
(() => {
  const sim = document.querySelector("[data-sim]");
  if (!sim) return;
  const vues = [...sim.querySelectorAll("[data-view]")];
  const zones = [...sim.querySelectorAll(".sim-hot")];
  const pastilles = [...sim.querySelectorAll(".sim-tabs [data-go]")];
  const legende = sim.querySelector(".sim-cap");

  // Les autres captures se chargent dès que la souris approche, pour que le changement soit immédiat
  const precharger = () => sim.querySelectorAll("img[loading=lazy]").forEach(i => { i.loading = "eager"; });
  sim.addEventListener("pointerenter", precharger, { once: true });
  sim.addEventListener("focusin", precharger, { once: true });

  const afficher = nom => {
    const vue = vues.find(v => v.dataset.view === nom);
    if (!vue) return;
    vues.forEach(v => { v.hidden = v !== vue; });
    zones.forEach(z => {
      const sur = z.dataset.on;
      z.hidden = sur !== "" && !sur.split(" ").includes(nom);
    });
    pastilles.forEach(b => b.setAttribute("aria-selected", String(b.dataset.go === nom)));
    legende.querySelector("strong").textContent = vue.dataset.t;
    legende.querySelector("span").textContent = vue.dataset.p;
  };

  sim.addEventListener("click", e => {
    const b = e.target.closest("[data-go]");
    if (b) afficher(b.dataset.go);
  });
})();
