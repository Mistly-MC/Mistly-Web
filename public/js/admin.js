// Tableau de bord de l'équipe : têtes des joueurs, copie, confirmations, chiffres en direct,
// filtre du journal d'un rapport et vérification du JSON de la base.
(() => {
  "use strict";
  const calme = matchMedia("(prefers-reduced-motion: reduce)").matches;

  // Fond : une brume de caractères qui dérive lentement (plus présente sur la page de connexion)
  const bg = document.querySelector(".adm-bg");
  if (bg) {
    const g = bg.getContext("2d");
    const login = document.body.classList.contains("login-page");
    const RAMP = " .·:-=+*#%@";
    const CW = 9, CH = 16;
    // Plein écran, ou la taille de son cadre (page de connexion)
    let cols = 0, rows = 0, dpr = 1, W = 0, H = 0;
    const size = () => {
      dpr = Math.min(devicePixelRatio || 1, 2);
      W = bg.clientWidth || innerWidth; H = bg.clientHeight || innerHeight;
      bg.width = W * dpr; bg.height = H * dpr;
      cols = Math.ceil(W / CW) + 1; rows = Math.ceil(H / CH) + 1;
    };
    size();
    addEventListener("resize", size);
    const hash = (x, y) => { const h = Math.sin(x * 127.1 + y * 311.7) * 43758.5453; return h - Math.floor(h); };
    const noise = (x, y) => {
      const xi = Math.floor(x), yi = Math.floor(y), xf = x - xi, yf = y - yi;
      const u = xf * xf * (3 - 2 * xf), v = yf * yf * (3 - 2 * yf);
      const a = hash(xi, yi), b = hash(xi + 1, yi), c = hash(xi, yi + 1), d = hash(xi + 1, yi + 1);
      return a + (b - a) * u + (c - a) * v + (a - b - c + d) * u * v;
    };
    const fbm = (x, y) => noise(x, y) * .55 + noise(x * 2.1, y * 2.1) * .3 + noise(x * 4.3, y * 4.3) * .15;
    let mx = -1e4, my = -1e4;
    addEventListener("pointermove", e => { const r = bg.getBoundingClientRect(); mx = e.clientX - r.left; my = e.clientY - r.top; });
    const draw = (t) => {
      g.setTransform(dpr, 0, 0, dpr, 0, 0);
      g.clearRect(0, 0, W, H);
      g.font = "12px 'Geist Mono', ui-monospace, monospace";
      g.textBaseline = "top";
      const s = t / 1000;
      for (let y = 0; y < rows; y++) {
        // Plus dense vers le bas, comme une mer de nuages
        const bas = login ? .35 + (y / rows) * .8 : .35 + (y / rows) * .5;
        for (let x = 0; x < cols; x++) {
          let n = fbm(x * .055 + s * .05, y * .1 - s * .018) * bas;
          const dx = x * CW - mx, dy = y * CH - my, dd = dx * dx + dy * dy;
          if (dd < 22000) n += (1 - dd / 22000) * .35;
          if (n < .22) continue;
          const k = Math.min(RAMP.length - 1, Math.floor((n - .22) * RAMP.length * 2.2));
          g.fillStyle = `rgba(206,218,245,${Math.min(.42, (n - .2) * (login ? .9 : .6))})`;
          g.fillText(RAMP[k], x * CW, y * CH);
        }
      }
    };
    if (calme) draw(0);
    else {
      let last = 0;
      const loop = (t) => {
        if (!document.hidden && t - last > 66) { draw(t); last = t; }
        requestAnimationFrame(loop);
      };
      requestAnimationFrame(loop);
    }
  }

  // Durée de la session (depuis la connexion, gardée pour l'onglet)
  const uptime = document.getElementById("uptime");
  if (uptime) {
    let debut = Number(sessionStorage.getItem("adm-start")) || Date.now();
    try { sessionStorage.setItem("adm-start", debut); } catch {}
    const pad = n => String(n).padStart(2, "0");
    const tic = () => {
      const s = Math.floor((Date.now() - debut) / 1000);
      uptime.textContent = `${pad(Math.floor(s / 3600))}:${pad(Math.floor(s / 60) % 60)}:${pad(s % 60)}`;
    };
    tic(); setInterval(tic, 1000);
  }

  // Connexion : afficher le mot de passe, secousse après un refus, bouton qui patiente à l'envoi
  const login = document.getElementById("login");
  if (login) {
    const pw = document.getElementById("pw");
    const eye = login.querySelector(".lg-eye");
    eye.addEventListener("click", () => {
      const voir = pw.type === "password";
      pw.type = voir ? "text" : "password";
      eye.setAttribute("aria-pressed", String(voir));
      eye.textContent = voir ? "cacher" : "voir";
      pw.focus();
    });
    if (login.dataset.error === "1") { login.classList.add("denied"); pw.select(); }
    login.addEventListener("submit", () => {
      login.querySelector(".lg-btn.solid").classList.add("busy");
      try { sessionStorage.setItem("adm-start", Date.now()); } catch {}
    });
  }

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

  // Barre du haut : « / » place le curseur dans la recherche des joueurs
  const tbq = document.getElementById("tb-q");
  addEventListener("keydown", e => {
    if (e.key === "/" && tbq && !/input|textarea|select/i.test(document.activeElement?.tagName || "")) { e.preventDefault(); tbq.focus(); }
  });

  // Les mini-courbes des chiffres en direct : les mesures de la session (gardées pour l'onglet)
  const courbes = {};
  const mesure = (id, v) => {
    let l = [];
    try { l = JSON.parse(sessionStorage.getItem("adm-spark-" + id)) || []; } catch {}
    l.push(Number(v) || 0);
    l = l.slice(-40);
    try { sessionStorage.setItem("adm-spark-" + id, JSON.stringify(l)); } catch {}
    courbes[id] = l;
    dessiner(id);
  };
  const dessiner = (id) => {
    const svg = document.querySelector(`svg[data-spark="${id}"]`);
    const l = courbes[id];
    if (!svg || !l) return;
    const v = l.length > 1 ? l : [l[0], l[0]];
    const max = Math.max(...v, 1), min = Math.min(...v, 0);
    const pts = v.map((x, i) => `${(i / (v.length - 1) * 120).toFixed(1)},${(30 - (x - min) / (max - min || 1) * 26).toFixed(1)}`).join(" ");
    svg.innerHTML = `<polygon points="0,32 ${pts} 120,32"/><polyline points="${pts}"/>`;
  };
  document.querySelectorAll("svg[data-spark]").forEach(svg => {
    const id = svg.dataset.spark;
    mesure(id, document.getElementById(id)?.textContent);
  });

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
        const set = (id, v) => {
          const e = document.getElementById(id);
          if (!e || e.textContent === String(v)) return;
          e.textContent = v;
          e.classList.remove("bump"); void e.offsetWidth; e.classList.add("bump");
        };
        set("n-launchers", d.launchers);
        set("n-client", d.players.length);
        mesure("n-launchers", d.launchers);
        mesure("n-client", d.players.length);
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
