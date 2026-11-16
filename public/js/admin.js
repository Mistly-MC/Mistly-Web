// Tableau de bord de l'équipe : têtes des joueurs, copie, confirmations, chiffres en direct,
// filtre du journal d'un rapport et vérification du JSON de la base.
(() => {
  "use strict";

  // Têtes : visage + calque du chapeau, depuis le skin relayé par le site (/skin/<pseudo>).
  const skins = new Map();
  const skin = (name) => {
    if (!skins.has(name)) {
      skins.set(name, new Promise((ok, ko) => {
        const img = new Image();
        img.onload = () => ok(img);
        img.onerror = ko;
        img.src = "/skin/" + encodeURIComponent(name);
      }));
    }
    return skins.get(name);
  };
  const heads = (root) => {
    for (const c of root.querySelectorAll("canvas.head")) {
      if (c.dataset.drawn) continue;
      c.dataset.drawn = "1";
      const g = c.getContext("2d");
      g.imageSmoothingEnabled = false;
      g.fillStyle = "#24304d";
      g.fillRect(0, 0, 8, 8);
      const name = c.dataset.name;
      if (!name) continue;
      skin(name).then((img) => {
        g.clearRect(0, 0, 8, 8);
        g.drawImage(img, 8, 8, 8, 8, 0, 0, 8, 8);
        if (img.height >= 64) g.drawImage(img, 40, 8, 8, 8, 0, 0, 8, 8);
      }).catch(() => {});
    }
  };
  heads(document);

  // Copier l'UUID
  for (const b of document.querySelectorAll("[data-copy]")) {
    b.addEventListener("click", () => {
      navigator.clipboard?.writeText(b.dataset.copy).then(() => {
        b.classList.add("done");
        setTimeout(() => b.classList.remove("done"), 1200);
      });
    });
  }

  // Confirmation avant les actions qui ne se défont pas
  for (const f of document.querySelectorAll("form[data-confirm]")) {
    f.addEventListener("submit", (e) => {
      if (!confirm(f.dataset.confirm)) e.preventDefault();
    });
  }

  // Une règle choisie s'applique tout de suite
  for (const s of document.querySelectorAll("select[data-autosubmit]")) {
    s.addEventListener("change", () => s.form.requestSubmit());
  }

  // Vue d'ensemble : chiffres et joueurs en ligne toutes les 15 secondes
  const online = document.getElementById("online");
  if (online) {
    const esc = (t) => String(t ?? "").replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[c]);
    const names = { misty_cape: "Cape Mistly", misty_elytra: "Élytres Mistly", early_cape: "Cape early player", cloudly: "Cloudly (chapeau)" };
    const since = (s) => (s < 60 ? "< 1 min" : s < 3600 ? `${Math.floor(s / 60)} min` : `${Math.floor(s / 3600)} h ${String(Math.floor((s % 3600) / 60)).padStart(2, "0")}`);
    const tick = async () => {
      if (document.hidden) return;
      try {
        const r = await fetch("/admin/online.json", { cache: "no-store" });
        if (!r.ok) return;
        const d = await r.json();
        const set = (id, v) => { const e = document.getElementById(id); if (e) e.textContent = v; };
        set("n-launchers", d.launchers);
        set("n-client", d.players.length);
        if (!d.players.length) {
          online.innerHTML = '<p class="empty">Personne n\'est en jeu avec Mistly Client.</p>';
          return;
        }
        const rows = d.players.map((p) => {
          const wear = [p.cosmetic, p.hat].filter(Boolean).map((c) => names[c] || c).join(" + ") || "—";
          const label = p.name || p.uuid.slice(0, 8) + "…";
          return `<tr><td><a class="who" href="/admin/players/${esc(p.uuid)}"><canvas class="head" width="8" height="8" ${p.name ? `data-name="${esc(p.name)}"` : ""} style="width:24px;height:24px" aria-hidden="true"></canvas><span>${esc(label)}</span></a></td><td class="muted">${esc(wear)}</td><td class="num muted">${since(p.since)}</td></tr>`;
        }).join("");
        online.innerHTML = `<table class="tbl"><thead><tr><th>Joueur</th><th>Porte</th><th class="num">Depuis</th></tr></thead><tbody>${rows}</tbody></table>`;
        heads(online);
      } catch {}
    };
    setInterval(tick, 15000);
  }

  // Rapport : couleur des erreurs, filtre et copie
  const log = document.querySelector(".logview");
  if (log) {
    const text = log.textContent;
    log.textContent = "";
    for (const line of text.split("\n")) {
      const s = document.createElement("span");
      s.className = "l" + (/ERROR|FATAL|Exception|Caused by|^\s+at /.test(line) ? " e" : /WARN/.test(line) ? " w" : "");
      s.textContent = line + "\n";
      log.append(s);
    }
    log.scrollTop = log.scrollHeight;
    document.querySelector("[data-filter-errors]")?.addEventListener("change", (e) => log.classList.toggle("only-errors", e.target.checked));
    const copy = document.querySelector("[data-copy-log]");
    copy?.addEventListener("click", () => navigator.clipboard?.writeText(text).then(() => {
      copy.textContent = "Copié";
      setTimeout(() => (copy.textContent = "Copier"), 1200);
    }));
  }

  // Base : le JSON est vérifié pendant la frappe
  const db = document.querySelector(".dbedit");
  if (db) {
    const area = db.querySelector("textarea");
    const state = document.getElementById("json-state");
    const check = () => {
      try {
        JSON.parse(area.value);
        db.classList.remove("invalid");
        state.textContent = "JSON valide";
        state.className = "good";
      } catch (e) {
        db.classList.add("invalid");
        state.textContent = "JSON invalide : " + e.message;
        state.className = "bad";
      }
    };
    area.addEventListener("input", check);
    db.addEventListener("submit", (e) => { if (db.classList.contains("invalid")) e.preventDefault(); });
    area.addEventListener("keydown", (e) => {
      if (e.key === "Tab") {
        e.preventDefault();
        area.setRangeText("  ", area.selectionStart, area.selectionEnd, "end");
      }
    });
  }
})();
