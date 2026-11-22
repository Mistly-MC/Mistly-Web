// Boutique : la recherche filtre les cartes pendant la frappe. Les rangées sans résultat se
// cachent, et un message apparaît quand plus rien ne correspond. « / » place le curseur dedans.
(() => {
  "use strict";
  const champ = document.getElementById("sx-q");
  if (!champ) return;
  const rangees = [...document.querySelectorAll(".sx-row")];
  const vide = document.querySelector(".sx-empty");
  // sans accents ni majuscules : « elytres » trouve « Élytres »
  const simple = t => t.normalize("NFD").replace(/[̀-ͯ]/g, "").toLowerCase().trim();
  const filtrer = () => {
    const q = simple(champ.value);
    let total = 0;
    for (const r of rangees) {
      const cartes = [...r.querySelectorAll(".sx-card")];
      // « Découvrir » (les bannières) n'a pas de cartes : caché pendant une recherche
      if (!cartes.length) { r.hidden = q !== ""; continue; }
      let n = 0;
      for (const c of cartes) {
        const ok = !q || simple(c.dataset.nom || "").includes(q);
        c.hidden = !ok;
        if (ok) n++;
      }
      r.hidden = n === 0;
      total += n;
    }
    if (vide) vide.hidden = total > 0;
  };
  champ.addEventListener("input", filtrer);
  addEventListener("keydown", e => {
    if (e.key === "/" && document.activeElement !== champ && !/input|textarea/i.test(document.activeElement?.tagName || "")) {
      e.preventDefault();
      champ.focus();
    }
  });
})();
