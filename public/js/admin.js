//  ███╗   ███╗██╗███████╗████████╗██╗     ██╗   ██╗
//  ████╗ ████║██║██╔════╝╚══██╔══╝██║     ╚██╗ ██╔╝
//  ██╔████╔██║██║███████╗   ██║   ██║      ╚████╔╝
//  ██║╚██╔╝██║██║╚════██║   ██║   ██║       ╚██╔╝
//  ██║ ╚═╝ ██║██║███████║   ██║   ███████╗   ██║
//  ╚═╝     ╚═╝╚═╝╚══════╝   ╚═╝   ╚══════╝   ╚═╝
//
//  Mistly Web · https://mistly.cc
//  Code : https://github.com/Mistly-MC/Mistly-Web
//  Discord : https://discord.gg/rdQJUgvUh3
//  Misty License 1.0 : free to use, modify and share, not to sell (LICENSE.txt)
//
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

  // ---------- Ce qui suit ne sert qu'au tableau de bord (pas à la page de connexion) ----------

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

  // La barre d'icônes : un seul rond bleu, qui glisse d'une icône à l'autre
  const rail = document.querySelector(".rail-nav");
  const rond = rail?.querySelector(".rail-pill");
  const liens = rail ? [...rail.querySelectorAll("a")] : [];
  const placerRond = (a, anime) => {
    if (!rond || !a) return;
    if (!anime) rond.style.transition = "none";
    // en colonne (ordinateur) ou en rangée (téléphone) : le rond suit l'icône dans les deux sens
    rond.style.transform = `translate(${a.offsetLeft}px, ${a.offsetTop}px)`;
    if (!anime) { void rond.offsetWidth; rond.style.transition = ""; }
    // rangée qui défile (téléphone) : l'icône ouverte reste visible
    if (rail.scrollWidth > rail.clientWidth) {
      rail.scrollTo({ left: a.offsetLeft - (rail.clientWidth - a.offsetWidth) / 2, behavior: anime ? "smooth" : "auto" });
    }
  };
  // l'icône d'une adresse : la plus longue qui la commence (« /admin » seulement pour elle-même)
  const icone = (chemin) => {
    let meilleur = null;
    for (const a of liens) {
      const h = new URL(a.href).pathname;
      const ok = h === "/admin" ? chemin === "/admin" || chemin === "/admin/" : chemin === h || chemin.startsWith(h + "/");
      if (ok && (!meilleur || h.length > new URL(meilleur.href).pathname.length)) meilleur = a;
    }
    return meilleur;
  };
  const allumer = (chemin, anime) => {
    const a = icone(chemin);
    liens.forEach(x => (x === a ? x.setAttribute("aria-current", "page") : x.removeAttribute("aria-current")));
    placerRond(a, anime);
  };
  if (rail) {
    allumer(location.pathname, false);
    rail.classList.add("ready");
    addEventListener("resize", () => allumer(location.pathname, false));
  }

  // Barre du haut : « / » place le curseur dans la recherche des joueurs
  addEventListener("keydown", e => {
    const tbq = document.getElementById("tb-q");
    if (e.key === "/" && tbq && !/input|textarea|select/i.test(document.activeElement?.tagName || "")) { e.preventDefault(); tbq.focus(); }
  });

  // Les mini-courbes des chiffres en direct : les mesures de la session (gardées pour l'onglet)
  const courbes = {};
  const dessiner = (id) => {
    const svg = document.querySelector(`svg[data-spark="${id}"]`);
    const l = courbes[id];
    if (!svg || !l) return;
    const v = l.length > 1 ? l : [l[0], l[0]];
    const max = Math.max(...v), min = Math.min(...v);
    // une valeur qui ne bouge pas : une ligne au milieu, sans aplat
    const y = (x) => (max === min ? 16 : 30 - (x - min) / (max - min) * 26);
    const pts = v.map((x, i) => `${(i / (v.length - 1) * 120).toFixed(1)},${y(x).toFixed(1)}`).join(" ");
    svg.innerHTML = (max === min ? "" : `<polygon points="0,32 ${pts} 120,32"/>`) + `<polyline points="${pts}"/>`;
  };
  const mesure = (id, v) => {
    let l = [];
    try { l = JSON.parse(sessionStorage.getItem("adm-spark-" + id)) || []; } catch {}
    l.push(Number(v) || 0);
    l = l.slice(-40);
    try { sessionStorage.setItem("adm-spark-" + id, JSON.stringify(l)); } catch {}
    courbes[id] = l;
    dessiner(id);
  };

  // Les joueurs en ligne et les chiffres en direct (vue d'ensemble), toutes les 15 secondes
  const esc = (t) => String(t ?? "").replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[c]);
  const names = { misty_cape: "Mistly Cape", misty_elytra: "Mistly Elytra", early_cape: "Early player cape", cloudly: "Cloudly (hat)" };
  const since = (s) => (s < 60 ? "< 1 min" : s < 3600 ? `${Math.floor(s / 60)} min` : `${Math.floor(s / 3600)} h ${String(Math.floor((s % 3600) / 60)).padStart(2, "0")}`);
  setInterval(async () => {
    const online = document.getElementById("online");
    if (!online || document.hidden) return;
    try {
      const r = await fetch("/admin/online.json", { cache: "no-store", headers: { "X-Mistly-Auto": "1" } });
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
        online.innerHTML = '<p class="empty">Nobody is playing with Mistly Client.</p>';
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
  }, 15000);

  // La courbe d'activité (vue d'ensemble)
  let redessiner = null;
  let attente = 0;
  addEventListener("resize", () => { clearTimeout(attente); attente = setTimeout(() => redessiner?.(), 150); });
  // Vue d'ensemble : la courbe d'activité (installations et événements par jour). Des courbes
  // lisses avec un point par jour, une bulle au survol, et la période choisie (7, 14 ou 30 jours).
  const courbe = (racine) => {
    const chart = racine.querySelector("#chart");
    if (!chart || !chart.dataset.series) { redessiner = null; return; }
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
      grad.append(el("stop", { offset: 0, "stop-color": "#8eaaf0", "stop-opacity": .3 }), el("stop", { offset: 1, "stop-color": "#8eaaf0", "stop-opacity": 0 }));
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
        bulle.innerHTML = `<b>${jour(labels[i])}</b><span><i style="background:#8eaaf0"></i>${A[i]} installs</span><span><i style="background:#5fd4e8"></i>${B[i]} events</span>`;
        bulle.style.left = x(i) + "px";
        bulle.style.top = Math.min(yA(A[i]), yB(B[i])) + "px";
        bulle.hidden = false;
      });
      svg.addEventListener("pointerleave", () => { bulle.hidden = true; ligne.setAttribute("visibility", "hidden"); });
    };
    let periode = 30;
    racine.querySelectorAll("[data-jours]").forEach(b => b.addEventListener("click", () => {
      periode = +b.dataset.jours;
      racine.querySelectorAll("[data-jours]").forEach(x => x.setAttribute("aria-pressed", String(x === b)));
      dessinerCourbe(periode);
    }));
    dessinerCourbe(periode);
    redessiner = () => dessinerCourbe(periode);
  };

  // ---------- Ce qui dépend de la page : branché au chargement et après chaque changement ----------
  const brancher = (racine) => {
    heads(racine);
    racine.querySelectorAll("svg[data-spark]").forEach(svg => mesure(svg.dataset.spark, document.getElementById(svg.dataset.spark)?.textContent));
    racine.querySelectorAll("[data-copy]").forEach(b => b.addEventListener("click", () => {
      navigator.clipboard?.writeText(b.dataset.copy).then(() => {
        b.classList.add("done");
        setTimeout(() => b.classList.remove("done"), 1200);
      });
    }));
    // Confirmation avant les actions qui ne se défont pas
    racine.querySelectorAll("form[data-confirm]").forEach(f => f.addEventListener("submit", (e) => {
      if (!confirm(f.dataset.confirm)) e.preventDefault();
    }));
    // Une règle choisie s'applique tout de suite
    racine.querySelectorAll("select[data-autosubmit]").forEach(s => s.addEventListener("change", () => s.form.requestSubmit()));
    courbe(racine);
    // Rapport : couleur des erreurs, filtre et copie
    const log = racine.querySelector(".logview");
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
      racine.querySelector("[data-filter-errors]")?.addEventListener("change", (e) => log.classList.toggle("only-errors", e.target.checked));
      const copy = racine.querySelector("[data-copy-log]");
      copy?.addEventListener("click", () => navigator.clipboard?.writeText(text).then(() => {
        copy.textContent = "Copied";
        setTimeout(() => (copy.textContent = "Copy"), 1200);
      }));
    }
    // Base : le JSON est vérifié pendant la frappe
    const db = racine.querySelector(".dbedit");
    if (db) {
      const area = db.querySelector("textarea");
      const state = racine.querySelector("#json-state");
      const check = () => {
        try {
          JSON.parse(area.value);
          db.classList.remove("invalid");
          state.textContent = "Valid JSON";
          state.className = "good";
        } catch (e) {
          db.classList.add("invalid");
          state.textContent = "Invalid JSON: " + e.message;
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
  };

  // ---------- Changer de page sans recharger ----------
  // Les liens du tableau de bord et les formulaires GET (recherches, filtres) chargent la page
  // demandée, puis on remplace le contenu et le titre. Les formulaires POST (bannir,
  // enregistrer…) gardent un envoi normal.
  const zone = document.querySelector(".scroll");
  const titre = document.querySelector(".tb-title");
  let enCours = null;
  const aller = async (url, pousser) => {
    if (!zone) { location.href = url; return; }
    const cible = new URL(url, location.href);
    allumer(cible.pathname, !calme);
    zone.classList.add("sortie");
    const moi = enCours = fetch(cible, { headers: { "X-Requested-With": "admin" } }).then(r => (r.redirected && !new URL(r.url).pathname.startsWith("/admin") ? null : r.ok || r.status === 404 ? r : null)).catch(() => null);
    const [r] = await Promise.all([moi, new Promise(ok => setTimeout(ok, calme ? 0 : 140))]);
    if (moi !== enCours) return;
    if (!r) { location.href = cible.href; return; }
    const html = await r.text();
    const doc = new DOMParser().parseFromString(html, "text/html");
    const neuve = doc.querySelector(".scroll");
    // la session a expiré : la page de connexion s'ouvre normalement
    if (!neuve) { location.href = r.url || cible.href; return; }
    zone.innerHTML = neuve.innerHTML;
    if (titre) titre.textContent = doc.querySelector(".tb-title")?.textContent || "";
    document.title = doc.title;
    const finale = new URL(r.url || cible.href);
    if (pousser) history.pushState({ admin: true }, "", finale.pathname + finale.search + cible.hash);
    allumer(finale.pathname, !calme);
    zone.scrollTop = 0;
    zone.classList.remove("sortie");
    zone.classList.add("entree");
    requestAnimationFrame(() => requestAnimationFrame(() => zone.classList.remove("entree")));
    brancher(zone);
  };
  const interne = (u) => u.origin === location.origin && u.pathname.startsWith("/admin") && !/\.json$/.test(u.pathname) && u.pathname !== "/admin/logout";
  document.addEventListener("click", (e) => {
    const a = e.target.closest("a[href]");
    if (!a || e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || a.target === "_blank" || a.hasAttribute("download")) return;
    const u = new URL(a.href, location.href);
    if (!interne(u)) return;
    if (u.pathname === location.pathname && u.search === location.search && u.hash) return;
    e.preventDefault();
    aller(u.href, true);
  });
  document.addEventListener("submit", (e) => {
    const f = e.target;
    if (e.defaultPrevented || (f.method || "get").toLowerCase() !== "get") return;
    const u = new URL(f.action || location.href, location.href);
    if (!interne(u)) return;
    e.preventDefault();
    u.search = new URLSearchParams(new FormData(f)).toString();
    aller(u.href, true);
  });
  addEventListener("popstate", () => aller(location.href, false));

  // ---------- Rafraîchissement automatique, toutes les 5 secondes ----------
  // Le contenu de la page est relu en silence et remplacé s'il a changé, sans recharger ni
  // bouger le défilement. Jamais pendant une édition : un champ actif, un formulaire modifié
  // (TODO, base, notes…), une sélection de texte ou une page cachée. Ces requêtes ne prolongent
  // pas la session (en-tête X-Mistly-Auto) : sans activité, elle expire après 5 minutes.
  let modifie = false, derniers = null, derniereUrl = location.href;
  document.addEventListener("input", (e) => { if (e.target.closest(".scroll form")) modifie = true; });
  document.addEventListener("submit", () => { modifie = false; });
  const occupe = () => {
    const actif = document.activeElement;
    if (actif && actif.closest(".scroll") && /^(INPUT|TEXTAREA|SELECT)$/.test(actif.tagName)) return true;
    if (actif?.isContentEditable) return true;
    if (String(getSelection?.() || "").length) return true;
    return modifie || document.hidden || zone?.classList.contains("sortie");
  };
  if (zone) setInterval(async () => {
    if (occupe() || enCours) return;
    try {
      const r = await fetch(location.href, { cache: "no-store", headers: { "X-Requested-With": "admin", "X-Mistly-Auto": "1" } });
      // session expirée : la page de connexion
      if (r.redirected && new URL(r.url).pathname === "/admin/login") { location.href = "/admin/login"; return; }
      if (!r.ok || occupe()) return;
      const doc = new DOMParser().parseFromString(await r.text(), "text/html");
      const neuve = doc.querySelector(".scroll");
      if (!neuve) { location.href = "/admin/login"; return; }
      // le HTML brut reçu, bloc par bloc (le DOM, lui, est modifié par la page : têtes, courbes)
      const bruts = [...neuve.children].map(n => n.outerHTML);
      const avant = derniers;
      derniers = bruts;
      // premier passage, ou une autre page entre-temps : on retient seulement
      if (!avant || derniereUrl !== location.href) { derniereUrl = location.href; return; }
      const anciens = [...zone.children], nouveaux = [...neuve.children];
      if (anciens.length !== nouveaux.length || avant.length !== bruts.length) {
        const haut = zone.scrollTop;
        zone.innerHTML = neuve.innerHTML;
        zone.scrollTop = haut;
        brancher(zone);
        return;
      }
      nouveaux.forEach((n, i) => {
        if (avant[i] === bruts[i]) return;
        anciens[i].replaceWith(n);
        brancher(n);
      });
    } catch {}
  }, 5000);
  // Retour arrière depuis une autre page : le rond revient sur la bonne icône
  addEventListener("pageshow", (e) => { if (e.persisted) allumer(location.pathname, false); });

  brancher(document);
})();
