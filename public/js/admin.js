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
    // La brume dérive seule : sans suivre la souris, elle reste fluide. Les caractères sont rangés
    // par opacité (8 niveaux) pour changer de couleur le moins souvent possible.
    const NIVEAUX = 8;
    const draw = (t) => {
      g.setTransform(dpr, 0, 0, dpr, 0, 0);
      g.clearRect(0, 0, W, H);
      g.font = "12px 'Geist Mono', ui-monospace, monospace";
      g.textBaseline = "top";
      const s = t / 1000;
      const lots = Array.from({ length: NIVEAUX }, () => []);
      for (let y = 0; y < rows; y++) {
        // Plus dense vers le bas, comme une mer de nuages
        const bas = login ? .35 + (y / rows) * .8 : .35 + (y / rows) * .5;
        for (let x = 0; x < cols; x++) {
          const n = fbm(x * .055 + s * .05, y * .1 - s * .018) * bas;
          if (n < .22) continue;
          const k = Math.min(RAMP.length - 1, Math.floor((n - .22) * RAMP.length * 2.2));
          const a = Math.min(.42, (n - .2) * (login ? .9 : .6));
          lots[Math.min(NIVEAUX - 1, Math.floor(a / .42 * NIVEAUX))].push(RAMP[k], x * CW, y * CH);
        }
      }
      lots.forEach((l, i) => {
        if (!l.length) return;
        g.fillStyle = `rgba(206,218,245,${((i + .5) / NIVEAUX * .42).toFixed(3)})`;
        for (let j = 0; j < l.length; j += 3) g.fillText(l[j], l[j + 1], l[j + 2]);
      });
    };
    if (calme) draw(0);
    else {
      let last = 0;
      const loop = (t) => {
        if (!document.hidden && t - last > 33) { draw(t); last = t; }
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
      eye.textContent = voir ? "hide" : "show";
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

  // La barre d'icônes : le rond rose part de l'icône de la page d'avant et glisse jusqu'à
  // celle de la page ouverte ; un clic le fait partir tout de suite vers la nouvelle.
  const rail = document.querySelector(".rail-nav");
  if (rail) {
    const rond = rail.querySelector(".rail-pill");
    const liens = [...rail.querySelectorAll("a")];
    const vers = (a, anime) => {
      if (!a) return;
      if (!anime) rond.style.transition = "none";
      rond.style.transform = `translateY(${a.offsetTop}px)`;
      if (!anime) { void rond.offsetWidth; rond.style.transition = ""; }
    };
    const ici = liens.findIndex(a => a.hasAttribute("aria-current"));
    let avant = -1;
    try { avant = Number(sessionStorage.getItem("adm-rail") ?? -1); } catch {}
    vers(liens[avant >= 0 && avant < liens.length ? avant : ici], false);
    rail.classList.add("ready");
    requestAnimationFrame(() => requestAnimationFrame(() => vers(liens[ici], !calme)));
    try { sessionStorage.setItem("adm-rail", ici); } catch {}
    liens.forEach(a => a.addEventListener("click", () => {
      liens.forEach(x => x.removeAttribute("aria-current"));
      a.setAttribute("aria-current", "page");
      vers(a, !calme);
    }));
    addEventListener("resize", () => vers(liens[ici], false));
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
    const names = { misty_cape: "Mistly Cape", misty_elytra: "Mistly Elytra", early_cape: "Early player cape", cloudly: "Cloudly (hat)" };
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
        // le même gabarit que la page (liste_en_ligne dans views/admin.rs)
        const rows = d.players.map((p) => {
          const wear = [p.cosmetic, p.hat].filter(Boolean).map((c) => names[c] || c).join(" + ") || "—";
          const label = p.name || p.uuid.slice(0, 8) + "…";
          return `<li><canvas class="head" width="8" height="8" ${p.name ? `data-name="${esc(p.name)}"` : ""} style="width:34px;height:34px" aria-hidden="true"></canvas><a href="/admin/players/${esc(p.uuid)}"><strong>${esc(label)}</strong><span>${esc(wear)}</span></a><span class="since">${since(p.since)}</span></li>`;
        }).join("");
        online.innerHTML = `<ul class="msgs">${rows}</ul>`;
        heads(online);
      } catch {}
    };
    setInterval(tick, 15000);
  }

  // Vue d'ensemble : la courbe d'activité (installations et événements par jour). Des courbes
  // lisses avec un point par jour, une bulle au survol, et la période choisie (7, 14 ou 30 jours).
  const chart = document.getElementById("chart");
  if (chart && chart.dataset.series) {
    let data = null;
    try { data = JSON.parse(chart.dataset.series); } catch {}
    const MOIS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    const jour = (d) => { const [, m, j] = d.split("-"); return `${MOIS[+m - 1]} ${+j}`; };
    const NS = "http://www.w3.org/2000/svg";
    const el = (nom, attrs) => { const e = document.createElementNS(NS, nom); for (const k in attrs) e.setAttribute(k, attrs[k]); return e; };
    // une courbe lisse qui passe par chaque point (Catmull-Rom vers Bézier)
    const lisse = (pts) => {
      if (pts.length < 2) return "";
      let d = `M${pts[0][0]},${pts[0][1]}`;
      for (let i = 0; i < pts.length - 1; i++) {
        const p0 = pts[i - 1] || pts[i], p1 = pts[i], p2 = pts[i + 1], p3 = pts[i + 2] || p2;
        const c1 = [p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6];
        const c2 = [p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6];
        d += ` C${c1[0].toFixed(1)},${c1[1].toFixed(1)} ${c2[0].toFixed(1)},${c2[1].toFixed(1)} ${p2[0].toFixed(1)},${p2[1].toFixed(1)}`;
      }
      return d;
    };
    const dessinerCourbe = (n) => {
      if (!data || data.labels.length < 2) return;
      const labels = data.labels.slice(-n), A = data.a.slice(-n), B = data.b.slice(-n);
      const W = chart.clientWidth, H = chart.clientHeight, g = 34, b = 26, h = 10;
      const L = W - g - 8, Hh = H - b - h;
      // chaque courbe sur sa propre échelle (les événements sont bien plus nombreux)
      const maxA = Math.max(...A, 1), maxB = Math.max(...B, 1);
      const x = (i) => g + (labels.length === 1 ? L / 2 : i / (labels.length - 1) * L);
      const yA = (v) => h + Hh - v / maxA * Hh * .92;
      const yB = (v) => h + Hh - v / maxB * Hh * .92;
      const svg = el("svg", { viewBox: `0 0 ${W} ${H}`, "aria-hidden": "true" });
      const defs = el("defs", {});
      const grad = el("linearGradient", { id: "g-a", x1: 0, y1: 0, x2: 0, y2: 1 });
      grad.append(el("stop", { offset: 0, "stop-color": "#a99cf0", "stop-opacity": .28 }), el("stop", { offset: 1, "stop-color": "#a99cf0", "stop-opacity": 0 }));
      defs.append(grad); svg.append(defs);
      // la grille et ses valeurs (installations)
      const grille = el("g", { class: "grid" });
      for (let k = 0; k <= 4; k++) {
        const yy = h + Hh - k / 4 * Hh * .92;
        grille.append(el("line", { x1: g, x2: W - 8, y1: yy, y2: yy }));
        const t = el("text", { x: g - 10, y: yy + 4, "text-anchor": "end", class: "lbl" });
        t.textContent = Math.round(maxA * k / 4);
        grille.append(t);
      }
      svg.append(grille);
      // les dates en bas (6 au plus)
      const pas = Math.max(1, Math.ceil(labels.length / 6));
      labels.forEach((d, i) => {
        if (i % pas && i !== labels.length - 1) return;
        const t = el("text", { x: x(i), y: H - 6, "text-anchor": "middle", class: "lbl" });
        t.textContent = jour(d);
        svg.append(t);
      });
      const pa = A.map((v, i) => [x(i), yA(v)]), pb = B.map((v, i) => [x(i), yB(v)]);
      svg.append(el("path", { d: `${lisse(pa)} L${x(A.length - 1)},${h + Hh} L${x(0)},${h + Hh} Z`, class: "f-a" }));
      for (const [pts, c] of [[pb, "c-b"], [pa, "c-a"]]) svg.append(el("path", { d: lisse(pts), class: `${c} curve` }));
      // un point par jour (un sur deux au-delà de 14 jours)
      const tous = labels.length <= 14 ? 1 : 2;
      pa.forEach((p, i) => { if (i % tous === 0) svg.append(el("circle", { cx: p[0], cy: p[1], r: 4.5, class: "d-a" })); });
      pb.forEach((p, i) => { if (i % tous === 0) svg.append(el("circle", { cx: p[0], cy: p[1], r: 4.5, class: "d-b" })); });
      const ligne = el("line", { y1: h, y2: h + Hh, class: "hover-line", visibility: "hidden" });
      svg.append(ligne);
      chart.replaceChildren(svg);
      // le tracé se dessine de gauche à droite
      svg.querySelectorAll(".curve").forEach(p => p.style.setProperty("--len", Math.ceil(p.getTotalLength())));
      // la bulle au survol : le jour le plus proche
      const bulle = document.createElement("div");
      bulle.className = "chart-tip"; bulle.hidden = true;
      chart.append(bulle);
      svg.addEventListener("pointermove", (e) => {
        const r = svg.getBoundingClientRect();
        const i = Math.max(0, Math.min(labels.length - 1, Math.round((e.clientX - r.left - g) / L * (labels.length - 1))));
        ligne.setAttribute("x1", x(i)); ligne.setAttribute("x2", x(i)); ligne.setAttribute("visibility", "visible");
        bulle.innerHTML = `<b>${jour(labels[i])}</b><span><i style="background:#a99cf0"></i>${A[i]} installs</span><span><i style="background:#f2a7dc"></i>${B[i]} events</span>`;
        bulle.style.left = x(i) + "px";
        bulle.style.top = Math.min(yA(A[i]), yB(B[i])) + "px";
        bulle.hidden = false;
      });
      svg.addEventListener("pointerleave", () => { bulle.hidden = true; ligne.setAttribute("visibility", "hidden"); });
    };
    let periode = 30;
    document.querySelectorAll("[data-jours]").forEach(b => b.addEventListener("click", () => {
      periode = +b.dataset.jours;
      document.querySelectorAll("[data-jours]").forEach(x => x.setAttribute("aria-pressed", String(x === b)));
      dessinerCourbe(periode);
    }));
    dessinerCourbe(periode);
    let attente = 0;
    addEventListener("resize", () => { clearTimeout(attente); attente = setTimeout(() => dessinerCourbe(periode), 150); });
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
      copy.textContent = "Copied";
      setTimeout(() => (copy.textContent = "Copy"), 1200);
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
