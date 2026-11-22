// Kit média : les boutons « Copier » (couleurs, bannière ASCII, textes de présentation).
(() => {
  "use strict";
  document.addEventListener("click", async e => {
    const b = e.target.closest("[data-copy]");
    if (!b) return;
    try { await navigator.clipboard.writeText(b.dataset.copy); } catch { return; }
    const label = b.querySelector("span:not(.chip)") || b.querySelector("code");
    b.classList.add("done");
    if (label && b.classList.contains("bk-copy")) {
      const avant = label.textContent;
      label.textContent = b.dataset.done;
      setTimeout(() => { label.textContent = avant; b.classList.remove("done"); }, 1400);
    } else setTimeout(() => b.classList.remove("done"), 1400);
  });
})();
