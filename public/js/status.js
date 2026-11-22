// Page de statut : thème clair ou sombre, période affichée (30 jours ou 60 minutes), bulles des
// barres à l'heure locale, compteur « vérifié il y a » et mise à jour toutes les minutes sans
// recharger la page.
(() => {
  "use strict";
  const html = document.documentElement;

  // Thème : le même réglage que sur le site
  document.querySelector("[data-theme-toggle]")?.addEventListener("click", () => {
    const sombre = html.dataset.theme ? html.dataset.theme === "dark" : !matchMedia("(prefers-color-scheme: light)").matches;
    html.dataset.theme = sombre ? "light" : "dark";
    try { localStorage.setItem("theme", html.dataset.theme); } catch {}
  });

  // Langue : un seul menu, refermé en cliquant ailleurs
  const lang = document.querySelector(".sp-lang");
  document.addEventListener("click", e => { if (lang?.open && !lang.contains(e.target)) lang.open = false; });

  // 30 jours ou 60 minutes
  document.querySelectorAll(".sp-seg [data-view]").forEach(b => b.addEventListener("click", () => {
    const min = b.dataset.view === "min";
    html.classList.toggle("st-min", min);
    try { localStorage.setItem("st-view", min ? "min" : "days"); } catch {}
  }));

  // Heures locales dans les bulles des minutes
  const local = root => root.querySelectorAll(".sp-bars [data-t]").forEach(i => {
    const t = new Date(+i.dataset.t * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    i.dataset.tip = i.dataset.tip.replace(/^\d\d:\d\d UTC/, t);
  });
  local(document);

  // La bulle qui suit les barres
  const tip = document.getElementById("sp-tip");
  const montrer = i => {
    const r = i.getBoundingClientRect();
    tip.textContent = i.dataset.tip;
    tip.hidden = false;
    const demi = tip.offsetWidth / 2 + 8;
    tip.style.left = Math.min(innerWidth - demi, Math.max(demi, r.left + r.width / 2)) + "px";
    tip.style.top = r.top + "px";
  };
  document.addEventListener("pointerover", e => {
    const i = e.target.closest?.(".sp-bars i");
    if (i) montrer(i); else tip.hidden = true;
  });
  addEventListener("scroll", () => { tip.hidden = true; }, { passive: true });

  // « Vérifié il y a 12 s » qui avance tout seul
  let modele = null, depart = 0, base = 0;
  const age = () => {
    const el = document.getElementById("sp-age");
    if (!el) return;
    if (+el.dataset.age !== base || !modele) {
      base = +el.dataset.age; depart = Date.now();
      modele = el.textContent.replace(String(base), "{0}");
    }
    el.textContent = modele.replace("{0}", base + Math.floor((Date.now() - depart) / 1000));
  };
  setInterval(age, 1000);

  // Mise à jour toutes les minutes : on remplace les blocs qui changent
  setInterval(async () => {
    if (document.hidden) return;
    const texte = await fetch(location.href, { cache: "no-store" }).then(r => r.ok ? r.text() : null).catch(() => null);
    if (!texte) return;
    const suivant = new DOMParser().parseFromString(texte, "text/html");
    for (const id of ["sp-hero", "sp-groups", "sp-log"]) {
      const a = document.getElementById(id), b = suivant.getElementById(id);
      if (a && b) a.replaceWith(b);
    }
    document.body.dataset.state = suivant.body.dataset.state;
    modele = null;
    local(document);
  }, 60_000);
})();
