// Petites améliorations côté navigateur. Tout le contenu est rendu par le serveur Rust :
// sans ce script, le site reste entièrement utilisable.
(() => {
  const reduce = matchMedia("(prefers-reduced-motion: reduce)").matches;
  const HEADER_OFFSET = 80; // même valeur que scroll-padding-top dans base.css

  // Barre de navigation opaque dès qu'on a défilé
  const nav = document.querySelector(".nav");
  const onScroll = () => nav && nav.classList.toggle("scrolled", scrollY > 8);
  addEventListener("scroll", onScroll, { passive: true });
  onScroll();

  // ---------- ASCII qui bouge ----------
  const MONO = '"Geist Mono", ui-monospace, Menlo, Consolas, monospace';
  // Bruit de valeur 2D lissé, puis fractal (3 octaves)
  const hash = (x, y) => {
    let h = (x * 374761393 + y * 668265263) ^ 0x5bd1e995;
    h = Math.imul(h ^ (h >>> 13), 1274126177);
    return ((h ^ (h >>> 16)) >>> 0) / 4294967295;
  };
  const smooth = t => t * t * (3 - 2 * t);
  const noise = (x, y) => {
    const xi = Math.floor(x), yi = Math.floor(y), xf = smooth(x - xi), yf = smooth(y - yi);
    const a = hash(xi, yi), b = hash(xi + 1, yi), c = hash(xi, yi + 1), d = hash(xi + 1, yi + 1);
    return a + (b - a) * xf + (c - a) * yf + (a - b - c + d) * xf * yf;
  };
  const fbm = (x, y) => noise(x, y) * 0.55 + noise(x * 2.1, y * 2.1) * 0.3 + noise(x * 4.3, y * 4.3) * 0.15;
  const RAMP = " .,-~:;=+*";
  // Anime `draw(t)` à environ `fps` images par seconde, seulement quand l'élément est visible et
  // l'onglet ouvert ; une seule image si les animations sont coupées.
  const animate = (el, draw, fps = 12) => {
    let visible = true, last = 0;
    if ("IntersectionObserver" in window) new IntersectionObserver(e => { visible = e[0].isIntersecting; }).observe(el);
    draw(performance.now() / 1000);
    if (reduce) return;
    const loop = now => {
      requestAnimationFrame(loop);
      if (!visible || document.hidden || now - last < 1000 / fps) return;
      last = now;
      draw(now / 1000);
    };
    requestAnimationFrame(loop);
  };

  // Brume en caractères qui dérive par-dessus le paysage du haut de page
  const canvas = document.querySelector(".ascii-bg");
  if (canvas && canvas.getContext) {
    const g = canvas.getContext("2d");
    const SIZE = 12, LINE = 14, font = `${SIZE}px ${MONO}`;
    let cols = 0, rows = 0, cw = 7;
    const layout = () => {
      const dpr = Math.min(devicePixelRatio || 1, 2);
      canvas.width = Math.round(canvas.clientWidth * dpr);
      canvas.height = Math.round(canvas.clientHeight * dpr);
      g.setTransform(dpr, 0, 0, dpr, 0, 0);
      g.font = font;
      g.textBaseline = "top";
      cw = g.measureText("M").width || 7;
      cols = Math.ceil(canvas.clientWidth / cw) + 1;
      rows = Math.ceil(canvas.clientHeight / LINE) + 1;
    };
    const draw = t => {
      g.clearRect(0, 0, canvas.width, canvas.height);
      const buckets = Array.from({ length: 8 }, () => []);
      for (let r = 0; r < rows; r++) {
        const y = r / rows;
        // Deux bancs de brume : un au-dessus des montagnes, un plus épais au pied
        const band = 0.55 * Math.exp(-(((y - 0.5) / 0.1) ** 2)) + Math.exp(-(((y - 0.8) / 0.13) ** 2));
        if (band < 0.05) continue;
        for (let c = 0; c < cols; c++) {
          const f = (fbm(c * 0.04 - t * 0.06 * (0.6 + y), r * 0.13 + t * 0.015) - 0.42) * 2.2 * band;
          if (f < 0.1) continue;
          const k = Math.min(RAMP.length - 1, Math.floor(f * RAMP.length));
          buckets[Math.min(7, Math.floor(f * 8))].push(c, r, RAMP[k]);
        }
      }
      buckets.forEach((list, i) => {
        if (!list.length) return;
        g.fillStyle = `rgba(214,226,255,${(0.08 + i * 0.07).toFixed(2)})`;
        for (let k = 0; k < list.length; k += 3) g.fillText(list[k + 2], list[k] * cw, list[k + 1] * LINE);
      });
    };
    let resizeTimer;
    addEventListener("resize", () => { clearTimeout(resizeTimer); resizeTimer = setTimeout(() => { layout(); draw(performance.now() / 1000); }, 150); });
    (document.fonts ? document.fonts.load(font).catch(() => {}) : Promise.resolve()).then(() => {
      layout();
      animate(canvas, draw, 14);
      canvas.classList.add("on");
    });
  }

  // ---------- Téléchargement : écran plein avec un nuage qui se forme en caractères ASCII ----------
  const dlx = document.getElementById("dlx");
  if (dlx) {
    const art = dlx.querySelector(".dlx-art");
    const g = art.getContext("2d");
    const track = dlx.querySelector(".dlx-track"), pct = dlx.querySelector(".dlx-pct"), label = dlx.querySelector(".dlx-label");
    const CW = 10, CH = 18, BAR = 28;
    // Le nuage du logo, en pixels (un pixel = deux caractères de large)
    const CLOUD = [
      "         #####          ",
      "       #########        ",
      "      ###########       ",
      "      ############ ###  ",
      "   ##################### ",
      "  #######################",
      "  #######################",
      "   ##################### ",
      "                         ",
      "     ########   ####     ",
    ];
    const NOISE = "01#%@*+=-:.";
    let parts = [], cols = 0, rows = 0, t0 = 0, raf = 0, frame0 = 0, started = false;
    const layout = () => {
      const dpr = Math.min(devicePixelRatio || 1, 2);
      art.width = innerWidth * dpr; art.height = innerHeight * dpr;
      g.setTransform(dpr, 0, 0, dpr, 0, 0);
      g.font = `bold 14px ${MONO}`;
      g.textBaseline = "top";
      cols = Math.ceil(innerWidth / CW); rows = Math.ceil(innerHeight / CH);
      // Taille du nuage : un pixel = 2 caractères sur 1 ligne, ou 4 sur 2 sur un grand écran
      const S = cols >= 120 && rows >= 44 ? 2 : 1;
      const h = CLOUD.length, w = Math.max(...CLOUD.map(l => l.length)) * 2 * S;
      const ox = Math.floor((cols - w) / 2), oy = Math.max(2, Math.floor(rows * 0.27 - (h * S) / 2));
      parts = [];
      CLOUD.forEach((line, r) => [...line].forEach((ch, c) => {
        if (ch !== "#") return;
        for (let k = 0; k < 2 * S; k++) for (let m = 0; m < S; m++) {
          const i = parts.length;
          parts.push({
            tx: ox + (c * 2 * S) + k, ty: oy + r * S + m,
            sx: Math.random() * cols, sy: -2 - Math.random() * rows * 0.6,
            delay: Math.random() * 0.9 + (r / h) * 0.3, dur: 0.8 + Math.random() * 0.6,
            // dessous du nuage plus sombre, barres du bas en `=`
            final: r >= 8 ? "=" : r <= 1 ? "@" : hash(c + k, r) > 0.82 ? "%" : "#",
            shade: r >= 8 ? 0.55 : 1 - r / (h * 1.6), seed: i,
          });
        }
      }));
    };
    const ease = k => 1 - Math.pow(1 - k, 3);
    const draw = now => {
      const t = (now - t0) / 1000;
      g.clearRect(0, 0, innerWidth, innerHeight);
      // Brume de fond, plus légère au centre
      g.fillStyle = "rgba(170,190,240,.10)";
      for (let r = 0; r < rows; r += 1) for (let c = 0; c < cols; c += 2) {
        const v = fbm(c * 0.05 - t * 0.12, r * 0.12 + t * 0.03);
        if (v > 0.6) g.fillText(v > 0.7 ? "~" : ".", c * CW, r * CH);
      }
      // Les caractères tombent puis se posent à leur place dans le nuage
      let posed = 0;
      for (const p of parts) {
        const k = reduce ? 1 : Math.max(0, Math.min(1, (t - p.delay) / p.dur));
        if (k <= 0) continue;
        const e = ease(k);
        const x = p.sx + (p.tx - p.sx) * e, y = p.sy + (p.ty - p.sy) * e;
        let ch, a;
        if (k < 1) {
          ch = NOISE[Math.floor(hash(p.seed, Math.floor(t * 20)) * NOISE.length)];
          a = 0.35 + k * 0.5;
        } else {
          posed++;
          // Une fois posé : une vague de lumière traverse le nuage
          const wave = Math.max(0, 1 - Math.abs(p.tx / cols - ((t * 0.35) % 1.6 - 0.3)) * 6);
          ch = hash(p.seed, Math.floor(t * 3)) > 0.985 ? "*" : p.final;
          a = Math.min(1, 0.5 + p.shade * 0.5 + wave * 0.4);
        }
        g.fillStyle = `rgba(226,234,255,${a.toFixed(2)})`;
        g.fillText(ch, x * CW, y * CH);
      }
      // Progression : suit la formation du nuage
      const prog = Math.min(1, reduce ? 1 : posed / parts.length * 0.85 + Math.min(t / 2.6, 1) * 0.15);
      const n = Math.round(prog * BAR);
      track.textContent = "[" + "#".repeat(n) + "-".repeat(BAR - n) + "]";
      pct.textContent = `${Math.round(prog * 100)}%`;
      if (prog >= 1 && !dlx.classList.contains("done")) {
        dlx.classList.add("done");
        label.textContent = "✓ " + label.dataset.started;
      }
      raf = requestAnimationFrame(draw);
    };
    const close = () => {
      dlx.classList.remove("open");
      cancelAnimationFrame(raf);
      document.documentElement.style.overflow = "";
      setTimeout(() => { dlx.hidden = true; }, 250);
    };
    const open = href => {
      const portable = href.includes("portable");
      const s2 = dlx.querySelector(".s2");
      s2.textContent = portable ? s2.dataset.portable : s2.dataset.setup;
      dlx.querySelector("#dlx-again").href = href;
      dlx.classList.remove("done");
      label.textContent = label.dataset.preparing;
      dlx.hidden = false;
      document.documentElement.style.overflow = "hidden";
      requestAnimationFrame(() => dlx.classList.add("open"));
      layout();
      t0 = performance.now();
      cancelAnimationFrame(raf);
      raf = requestAnimationFrame(draw);
      dlx.querySelector("[data-dlx-close]").focus({ preventScroll: true });
      // Le fichier part pendant l'animation ; la page reste là (réponse en pièce jointe)
      setTimeout(() => {
        const frame = document.createElement("iframe");
        frame.hidden = true;
        frame.src = href;
        document.body.append(frame);
        setTimeout(() => frame.remove(), 60_000);
      }, 700);
    };
    document.addEventListener("click", e => {
      const a = e.target.closest('a[href^="/download/"]');
      if (!a || a.closest(".dlx") || e.metaKey || e.ctrlKey || e.shiftKey || e.button !== 0) return;
      e.preventDefault();
      open(a.getAttribute("href"));
    });
    dlx.querySelector("[data-dlx-close]").addEventListener("click", close);
    document.addEventListener("keydown", e => { if (e.key === "Escape" && !dlx.hidden) close(); });
    addEventListener("resize", () => { if (!dlx.hidden) layout(); });
  }

  // La souris laisse une traînée de caractères ASCII qui ondulent puis s'effacent
  const trail = document.querySelector(".cursor-ascii");
  if (trail && !reduce && matchMedia("(hover: hover) and (pointer: fine)").matches) {
    const g = trail.getContext("2d");
    const CELL_W = 9, CELL_H = 15, RADIUS = 70;
    const CHARS = " .:-=+*#%@";
    let cols = 0, rows = 0, heat = new Float32Array(0), running = false, last = 0;
    let mx = -999, my = -999, px = -999, py = -999;
    const size = () => {
      const dpr = Math.min(devicePixelRatio || 1, 2);
      trail.width = innerWidth * dpr; trail.height = innerHeight * dpr;
      g.setTransform(dpr, 0, 0, dpr, 0, 0);
      g.font = `12px ${MONO}`;
      g.textBaseline = "top";
      cols = Math.ceil(innerWidth / CELL_W); rows = Math.ceil(innerHeight / CELL_H);
      heat = new Float32Array(cols * rows);
    };
    size();
    addEventListener("resize", size);
    // Chauffe les cases autour d'un point, plus fort au centre
    const warm = (x, y, k) => {
      const c0 = Math.max(0, Math.floor((x - RADIUS) / CELL_W)), c1 = Math.min(cols - 1, Math.ceil((x + RADIUS) / CELL_W));
      const r0 = Math.max(0, Math.floor((y - RADIUS) / CELL_H)), r1 = Math.min(rows - 1, Math.ceil((y + RADIUS) / CELL_H));
      for (let r = r0; r <= r1; r++) for (let c = c0; c <= c1; c++) {
        const d = Math.hypot(c * CELL_W + CELL_W / 2 - x, r * CELL_H + CELL_H / 2 - y) / RADIUS;
        if (d < 1) heat[r * cols + c] = Math.min(1, heat[r * cols + c] + (1 - d) * (1 - d) * k);
      }
    };
    const frame = now => {
      if (now - last < 33) { requestAnimationFrame(frame); return; }
      last = now;
      // Le long du trajet depuis la dernière image, pour une traînée continue
      const steps = Math.min(12, Math.ceil(Math.hypot(mx - px, my - py) / 12));
      for (let i = 1; i <= steps; i++) warm(px + (mx - px) * i / steps, py + (my - py) * i / steps, 0.22);
      px = mx; py = my;
      g.clearRect(0, 0, innerWidth, innerHeight);
      const t = now / 1000;
      let alive = false;
      for (let r = 0; r < rows; r++) for (let c = 0; c < cols; c++) {
        const i = r * cols + c, h = heat[i];
        if (h < 0.02) { heat[i] = 0; continue; }
        alive = true;
        heat[i] = h * 0.93;
        // Un bruit qui bouge : la traînée frémit au lieu d'être un simple rond
        const v = h * (0.55 + 0.45 * fbm(c * 0.35 + t * 1.6, r * 0.5 - t));
        const ch = CHARS[Math.min(CHARS.length - 1, Math.floor(v * CHARS.length))];
        if (ch === " ") continue;
        g.fillStyle = `rgba(200,216,255,${Math.min(0.85, v * 0.95).toFixed(3)})`;
        g.fillText(ch, c * CELL_W, r * CELL_H);
      }
      if (alive || mx !== px) requestAnimationFrame(frame);
      else running = false;
    };
    addEventListener("pointermove", e => {
      if (px < -900) { px = e.clientX; py = e.clientY; }
      mx = e.clientX; my = e.clientY;
      if (!running) { running = true; requestAnimationFrame(frame); }
    }, { passive: true });
    // En défilant, la traînée reste collée à la page : on l'efface
    addEventListener("scroll", () => heat.fill(0), { passive: true });
  }

  // Le nom en lettres de blocs du pied de page : quelques blocs qui scintillent (la vague de lumière
  // est un dégradé CSS ; le texte reste d'un seul tenant, sans coutures entre les caractères)
  const name = document.querySelector(".ascii-name");
  if (name) {
    const base = name.textContent;
    animate(name, t => {
      const tick = Math.floor(t * 5);
      let i = 0;
      name.textContent = base.replace(/█/g, () => (hash(i++, tick) > 0.985 ? "▓" : "█"));
    }, 5);
  }

  // Cartes « bientôt » de la boutique : un bruit de caractères qui ondule
  document.querySelectorAll(".ascii-noise").forEach((pre, i) => {
    let cols = 0, rows = 0;
    const size = () => {
      cols = Math.ceil(pre.clientWidth / 7.2);
      rows = Math.ceil(pre.clientHeight / 13);
    };
    size();
    addEventListener("resize", size);
    animate(pre, t => {
      let out = "";
      for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {
          const v = fbm(c * 0.09 + i * 7 + t * 0.35, r * 0.18 - t * 0.2);
          out += v > 0.62 ? "#" : v > 0.55 ? "+" : v > 0.48 ? ":" : v > 0.42 ? "." : " ";
        }
        out += "\n";
      }
      pre.textContent = out;
    }, 10);
  });

  // Menus déroulants (langue, menu mobile) : fermeture au clic à côté ou avec Échap
  const menus = [...document.querySelectorAll("details.langdd, details.burger")];
  document.addEventListener("click", e => menus.forEach(m => { if (!m.contains(e.target)) m.open = false; }));
  document.addEventListener("keydown", e => { if (e.key === "Escape") menus.forEach(m => { m.open = false; }); });
  // Un lien choisi dans un menu le referme
  menus.forEach(m => m.querySelectorAll("a").forEach(a => a.addEventListener("click", () => { m.open = false; })));
  // Un seul menu ouvert à la fois
  menus.forEach(m => m.addEventListener("toggle", () => { if (m.open) menus.forEach(o => { if (o !== m) o.open = false; }); }));

  // Page de statut : heures locales dans les bulles, et mise à jour toutes les minutes sans recharger
  if (document.body.classList.contains("status-page")) {
    const local = root => root.querySelectorAll(".st-bars i[data-t]").forEach(i => {
      const t = new Date(+i.dataset.t * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
      i.title = i.title.replace(/^\d\d:\d\d UTC/, t);
    });
    local(document);
    setInterval(async () => {
      if (document.hidden) return;
      const html = await fetch(location.href, { cache: "no-store" }).then(r => r.ok ? r.text() : null).catch(() => null);
      if (!html) return;
      const next = new DOMParser().parseFromString(html, "text/html");
      for (const sel of [".st-summary", "section.status"]) {
        const a = document.querySelector(sel), b = next.querySelector(sel);
        if (a && b) a.replaceWith(b);
      }
      local(document);
    }, 60_000);
  }

  // Accordéons (FAQ, patch notes) : la hauteur glisse à l'ouverture et à la fermeture
  document.querySelectorAll(".qa details, details.rel").forEach(d => {
    const summary = d.querySelector("summary");
    if (!summary || reduce || !d.animate) return;
    let anim = null;
    const glisser = (de, a, fin) => {
      anim?.cancel();
      d.style.overflow = "hidden";
      anim = d.animate({ height: [`${de}px`, `${a}px`] }, { duration: 320, easing: "cubic-bezier(.25,.8,.25,1)" });
      anim.onfinish = () => { anim = null; d.style.overflow = ""; fin?.(); };
    };
    summary.addEventListener("click", e => {
      e.preventDefault();
      const depart = d.offsetHeight;
      if (d.open && !d.classList.contains("closing")) {
        d.classList.add("closing");
        glisser(depart, summary.offsetHeight, () => { d.open = false; d.classList.remove("closing"); });
      } else {
        d.classList.remove("closing");
        d.open = true;
        glisser(depart, d.scrollHeight);
      }
    });
  });

  // Patch notes : ouvrir la version visée par l'ancre (ex. /patch-notes#v0.3.1)
  const openFromHash = () => {
    const id = decodeURIComponent(location.hash.slice(1));
    const el = id && document.getElementById(id);
    if (el && el.tagName === "DETAILS") el.open = true;
  };
  openFromHash();
  addEventListener("hashchange", openFromHash);

  // Apparition au défilement
  const items = document.querySelectorAll(".reveal");
  if ("IntersectionObserver" in window && !reduce) {
    const io = new IntersectionObserver(entries => entries.forEach(e => {
      if (e.isIntersecting) { e.target.classList.add("in"); io.unobserve(e.target); }
    }), { threshold: 0.12 });
    items.forEach(el => io.observe(el));
  } else {
    items.forEach(el => el.classList.add("in"));
  }

  // Cliquer sur la page où l'on est déjà (lien de la barre, logo…) : on remonte en haut
  // en douceur au lieu de recharger la page.
  document.addEventListener("click", e => {
    const a = e.target.closest("a[href]");
    if (!a || e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || a.target === "_blank") return;
    const url = new URL(a.href, location.href);
    if (url.origin !== location.origin || url.pathname !== location.pathname || url.search || url.hash) return;
    e.preventDefault();
    if (location.hash) history.replaceState(null, "", location.pathname + location.search);
    scrollTo({ top: 0, behavior: reduce ? "auto" : "smooth" });
  });

  // ---------- Boutique : studio 3D du skin (skinview3d) ----------
  const stage = document.getElementById("stage");
  let stageReady = Promise.resolve();   // le joueur est chargé dans l'aperçu
  if (stage && window.skinview3d) {
    const sv = window.skinview3d;
    const canvas = document.getElementById("skin3d");
    const studio = stage.classList.contains("studio-stage");
    const viewer = new sv.SkinViewer({ canvas, width: canvas.clientWidth || 360, height: canvas.clientHeight || 420, zoom: studio ? 0.72 : 0.62, fov: 36 });
    viewer.globalLight.intensity = 2.4;
    viewer.cameraLight.intensity = 0.8;
    viewer.controls.enableZoom = studio;
    viewer.controls.enablePan = false;
    viewer.autoRotate = !reduce;
    viewer.autoRotateSpeed = 0.7;
    const press = (buttons, active) => buttons.forEach(b => b.setAttribute("aria-pressed", String(b === active)));

    // Pseudo au-dessus de la tête, comme en jeu
    const setName = name => {
      viewer.nameTag = name
        ? new sv.NameTagObject(name, { font: "48px Silkscreen", height: 2.4, margin: [4, 8, 2, 8], textStyle: "#eaf1ff", backgroundStyle: "rgba(10,15,29,.55)" })
        : null;
    };

    // Animations : repos, marche, course, vol, salut, accroupi
    const ANIMS = { idle: sv.IdleAnimation, walk: sv.WalkingAnimation, run: sv.RunningAnimation, fly: sv.FlyingAnimation, crouch: sv.CrouchAnimation };
    const SPEED = { walk: 0.6, run: 0.8 };
    const animButtons = [...stage.querySelectorAll("[data-anim]")];
    const setAnim = key => {
      const Anim = ANIMS[key];
      viewer.animation = Anim ? new Anim() : null;
      if (viewer.animation) { viewer.animation.speed = SPEED[key] ?? 1; viewer.animation.paused = reduce; }
      press(animButtons, animButtons.find(b => b.dataset.anim === key));
      // Voler sans rien dans le dos : on sort les élytres
      if (key === "fly" && back.mode === "none") setBack("elytra", ownCape ? "own" : "misty");
    };
    animButtons.forEach(b => b.addEventListener("click", () => setAnim(b.dataset.anim)));

    // Cape et élytres : la vraie cape du joueur (Mojang) ou la texture Mistly
    // Texture de cape et d'élytres. `pal` : couleur du haut et écart jusqu'au bas ; `emblem` : nuage ou étoile.
    const capeTexture = (pal = [[26, 40, 77], [70, 105, 160]], emblem = "cloud") => {
      const c = document.createElement("canvas");
      c.width = 64; c.height = 32;
      const g = c.getContext("2d");
      const shade = (x, y, h, x0) => {
        const t = y / h, d = ((x + y) % 2) * 6 - 3 + (((x - x0) * 7 + y * 3) % 5) - 2;
        const [[r0, g0, b0], [dr, dg, db]] = pal;
        return `rgb(${Math.round(r0 + dr * t + d)},${Math.round(g0 + dg * t + d)},${Math.round(b0 + db * t + d)})`;
      };
      const fill = (x0, y0, w, h) => {
        for (let y = y0; y < y0 + h; y++) for (let x = x0; x < x0 + w; x++) { g.fillStyle = shade(x, y - y0, h, x0); g.fillRect(x, y, 1, 1); }
      };
      const cloud = (ox, oy) => {
        const rows = ["..###...", ".######.", "########", "########", ".######."];
        rows.forEach((row, y) => [...row].forEach((ch, x) => {
          if (ch !== "#") return;
          g.fillStyle = y === 4 ? "#b9cdf5" : "#eaf1ff"; g.fillRect(ox + x, oy + y, 1, 1);
        }));
        g.fillStyle = "#cfe0ff"; g.fillRect(ox, oy + 6, 6, 1); g.fillRect(ox + 4, oy + 8, 4, 1);
      };
      // Cape (patron 64×32) : face extérieure (1,1) en dégradé avec un nuage ; face intérieure (12,1)
      // et tranches en bleu nuit uni, pour qu'elle se lise comme une étoffe fine et non comme un bloc.
      const star = (ox, oy) => {
        const rows = ["...#...", "...#...", ".#####.", "#######", ".#####.", "..#.#..", ".#...#."];
        rows.forEach((row, y) => [...row].forEach((ch, x) => {
          if (ch === "#") { g.fillStyle = y === 3 ? "#fff6dc" : "#ffe3a3"; g.fillRect(ox + x, oy + y, 1, 1); }
        }));
      };
      fill(1, 1, 10, 16);
      if (emblem === "star") star(2, 5); else cloud(2, 5);
      g.fillStyle = "#121b33"; g.fillRect(12, 1, 10, 16);            // intérieur
      g.fillStyle = "#1c2a4d";
      g.fillRect(1, 0, 10, 1); g.fillRect(11, 0, 10, 1);              // dessus, dessous
      g.fillRect(0, 1, 1, 16); g.fillRect(11, 1, 1, 16);              // côtés
      // Élytres : même masque que les capes officielles de Mojang. Seule la face de dos est peinte
      // (x 34→42, y 2→21, plus l'attache en haut) ; les autres faces restent transparentes, sinon
      // l'aile apparaît en double et épaisse. Chaque ligne : [première, dernière] colonne opaque.
      const MASK = [[31,33],[32,33],[34,42],[34,42],[35,42],[35,42],[35,42],[35,42],[35,42],[35,42],[35,42],[35,42],[36,42],[36,42],[36,42],[36,42],[37,41],[37,41],[37,41],[37,41],[38,40],[38,39]];
      const MOON = ["..##", ".#..", "#...", "#...", ".#..", "..##"];   // croissant de lune
      for (let y = 0; y < MASK.length; y++) {
        const [x0, x1] = MASK[y];
        for (let x = x0; x <= x1; x++) {
          const t = y / 21, d = ((x * 7 + y * 3) % 5) - 2;
          let r = 24 + 62 * t + d, gg = 36 + 98 * t + d, b = 70 + 150 * t + d * 1.5;
          const k = ((y - (x - 35) * 0.7) % 4.5 + 4.5) % 4.5;               // plumes en diagonale
          if (y > 7 && y < 20 && k < 1) { r += 70; gg += 72; b += 30; }
          if (x === x0) { r += 40; gg += 45; b += 28; }                    // liseré
          if (MOON[y - 4]?.[x - 37] === "#") { r = 234; gg = 241; b = 255; }
          g.fillStyle = `rgb(${Math.min(r, 245) | 0},${Math.min(gg, 248) | 0},${Math.min(b, 255) | 0})`;
          g.fillRect(x, y, 1, 1);
        }
      }
      return c;
    };
    const misty = capeTexture();
    const early = capeTexture([[58, 22, 48], [190, 120, 30]], "star");   // aube : prune vers or
    let ownCape = null;                       // URL de la cape du joueur, s'il en a une
    const back = { mode: "none", source: "own" };
    const capeBtn = stage.querySelector('[data-toggle="cape"]');
    const elytraBtn = stage.querySelector('[data-toggle="elytra"]');
    const equipButtons = [...document.querySelectorAll(".equip")];
    const setBack = (mode, source) => {
      back.mode = mode; back.source = source;
      const texture = mode === "none" ? null : (source === "own" && ownCape ? ownCape : source === "early" ? early : misty);
      viewer.loadCape(texture, texture ? { backEquipment: mode } : undefined);
      capeBtn?.setAttribute("aria-pressed", String(mode === "cape" && source === "own" && !!ownCape));
      elytraBtn?.setAttribute("aria-pressed", String(mode === "elytra"));
      equipButtons.forEach(b => {
        const on = source === (b.dataset.src || "misty") && b.dataset.back === mode;
        b.setAttribute("aria-pressed", String(on));
        // Les boutons de la vitrine gardent leur vignette et leur nom ; ceux du studio disent Équiper / Retirer
        if (!b.querySelector(".thumb")) b.textContent = on ? b.dataset.unequip : b.dataset.equip;
        b.closest(".item")?.classList.toggle("equipped", on);
        // Avertissement : ce cosmétique ne s'affiche pas encore en jeu
        const warn = b.closest(".cat-panel")?.querySelector(".equip-warn");
        if (warn) warn.hidden = !on;
      });
    };
    capeBtn?.addEventListener("click", () => setBack(back.mode === "cape" && back.source === "own" ? "none" : "cape", "own"));
    elytraBtn?.addEventListener("click", () => {
      if (back.mode === "elytra") setBack(ownCape ? "cape" : "none", "own");
      else setBack("elytra", ownCape && back.source === "own" ? "own" : "misty");
    });
    const featName = document.getElementById("feat-name"), featText = document.getElementById("feat-text");
    const hatNote = document.getElementById("hat-note");
    equipButtons.forEach(b => b.addEventListener("click", () => {
      const src = b.dataset.src || "misty";
      const hat = b.dataset.back === "none";   // Cloudly : un chapeau, pas encore dans l'aperçu 3D
      const on = !hat && back.mode === b.dataset.back && back.source === src;
      if (hat) setBack(ownCape ? "cape" : "none", "own");
      else setBack(on ? (ownCape ? "cape" : "none") : b.dataset.back, on ? "own" : src);
      if (hat) b.setAttribute("aria-pressed", "true");
      if (hatNote) hatNote.hidden = !hat;
      // Vitrine : le nom et la description de l'article choisi
      if (!on && featName && b.dataset.name) {
        featName.textContent = b.dataset.name;
        featText.textContent = b.dataset.text;
        const badge = document.getElementById("feat-badge");
        badge.textContent = b.dataset.badge;
        badge.className = "badge " + (b.dataset.early ? "early" : "free");
      }
    }));
    // Vignettes des articles Mistly, dessinées depuis la texture
    document.querySelectorAll("canvas.thumb").forEach(c => {
      const g = c.getContext("2d");
      g.imageSmoothingEnabled = false;
      if (c.dataset.thumb === "cape") g.drawImage(misty, 1, 1, 10, 16, 0, 0, c.width, c.height);
      else if (c.dataset.thumb === "early") g.drawImage(early, 1, 1, 10, 16, 0, 0, c.width, c.height);
      else if (c.dataset.thumb === "cloudly") {
        // Cloudly : un petit nuage en pixels, centré
        const rows = ["...####...", "..######..", ".########.", "##########", "##########", ".########."];
        const px = Math.floor(c.width / 12);
        const ox = (c.width - 10 * px) / 2, oy = (c.height - 6 * px) / 2;
        rows.forEach((row, y) => [...row].forEach((ch, x) => {
          if (ch !== "#") return;
          g.fillStyle = y >= 4 ? "#b9cdf5" : "#f2f6ff";
          g.fillRect(ox + x * px, oy + y * px, px, px);
        }));
      } else {
        // Élytres : une aile et son reflet
        const w = c.width / 2;
        g.drawImage(misty, 34, 2, 9, 20, 0, 0, w, c.height);
        g.save(); g.scale(-1, 1); g.drawImage(misty, 34, 2, 9, 20, -c.width, 0, w, c.height); g.restore();
      }
    });

    // Vues face / profil / dos (la rotation automatique s'arrête)
    const rotateBtn = stage.querySelector('[data-toggle="rotate"]');
    const viewButtons = [...stage.querySelectorAll("[data-view]")];
    const ANGLE = { front: 0, side: Math.PI / 2, back: Math.PI };
    let tween = 0;
    const setView = key => {
      viewer.autoRotate = false;
      rotateBtn?.setAttribute("aria-pressed", "false");
      viewer.resetCameraPose();   // caméra à sa distance d'origine (zoom et orbite annulés)
      viewer.controls.update();
      const wrap = viewer.playerWrapper, from = ((wrap.rotation.y % (2 * Math.PI)) + 2 * Math.PI) % (2 * Math.PI);
      let to = ANGLE[key];
      if (to - from > Math.PI) to -= 2 * Math.PI; else if (from - to > Math.PI) to += 2 * Math.PI;
      const t0 = performance.now(), dur = reduce ? 0 : 450;
      cancelAnimationFrame(tween);
      const step = now => {
        const k = dur ? Math.min(1, (now - t0) / dur) : 1, e = 1 - Math.pow(1 - k, 3);
        wrap.rotation.y = from + (to - from) * e;
        if (k < 1) tween = requestAnimationFrame(step);
      };
      tween = requestAnimationFrame(step);
      press(viewButtons, viewButtons.find(b => b.dataset.view === key));
    };
    viewButtons.forEach(b => b.addEventListener("click", () => setView(b.dataset.view)));
    rotateBtn?.addEventListener("click", () => {
      viewer.autoRotate = !viewer.autoRotate;
      rotateBtn.setAttribute("aria-pressed", String(viewer.autoRotate));
      if (viewer.autoRotate) press(viewButtons, null);
    });

    const input = document.getElementById("player");   // champ du pseudo (formulaire), s'il y en a un

    // Charge le joueur : infos (pseudo exact, UUID, bras fins, cape) puis skin en 3D
    // `fresh` : bouton « Actualiser », on ignore les caches (serveur et navigateur).
    const loadPlayer = async (name, fresh = false) => {
      let info = null;
      const bust = fresh ? `?t=${Date.now()}` : "";
      if (name) {
        const res = await fetch(`/player/${name}${fresh ? "?fresh=1" : ""}`, fresh ? { cache: "no-store" } : {}).catch(() => null);
        if (res?.ok) info = await res.json();
        else if (studio && res?.status === 404) document.getElementById("pmissing").hidden = false;
      }
      const skin = (name ? `/skin/${name}` : `/skin/${stage.dataset.skin}`) + bust;
      const model = info ? (info.slim ? "slim" : "default") : "auto-detect";
      await viewer.loadSkin(skin, { model }).catch(() => viewer.loadSkin("/skin/MHF_Steve").catch(() => {}));
      setName(info?.name ?? name);
      ownCape = info?.cape ? `/cape/${info.name}${bust}` : null;
      if (capeBtn) capeBtn.disabled = !ownCape;
      // Comme NameMC : la vraie cape est portée d'office. Lors d'un rafraîchissement, on garde
      // un cosmétique Mistly équipé.
      if (fresh && back.source !== "own") setBack(back.mode, back.source);
      else if (!studio && !name) setBack("cape", "misty");   // vitrine : l'article à la une, porté par Steve
      else setBack(ownCape ? "cape" : "none", "own");
      if (studio && info) {
        document.getElementById("pname").textContent = info.name;
        const chip = document.getElementById("pmodel");
        chip.textContent = info.slim ? chip.dataset.slim : chip.dataset.classic;
        const uuid = document.getElementById("puuid");
        if (info.uuid) { uuid.querySelector("code").textContent = info.uuid; uuid.hidden = false; }
      }
    };
    setAnim("walk");
    stageReady = loadPlayer(stage.dataset.name).finally(() => stage.classList.add("ready"));

    // Actualiser : recharge le skin et la cape (après un changement chez Mojang)
    const reloadBtn = document.getElementById("reload");
    reloadBtn?.addEventListener("click", () => {
      if (reloadBtn.classList.contains("busy")) return;
      const name = input?.value.trim().match(/^[A-Za-z0-9_]{3,16}$/)?.[0] || stage.dataset.name;
      reloadBtn.classList.add("busy");
      loadPlayer(name, true).finally(() => setTimeout(() => reloadBtn.classList.remove("busy"), 400));
    });

    // Copier l'UUID
    const uuidBtn = document.getElementById("puuid");
    uuidBtn?.addEventListener("click", () => {
      navigator.clipboard?.writeText(uuidBtn.querySelector("code").textContent).then(() => {
        const old = uuidBtn.title; uuidBtn.title = uuidBtn.dataset.copied; uuidBtn.classList.add("done");
        setTimeout(() => { uuidBtn.title = old; uuidBtn.classList.remove("done"); }, 1400);
      });
    });

    // Onglets des catégories de cosmétiques
    const tabs = [...document.querySelectorAll(".cat-tabs [role=tab]")];
    tabs.forEach(tab => tab.addEventListener("click", () => {
      tabs.forEach(t => t.setAttribute("aria-selected", String(t === tab)));
      document.querySelectorAll(".cat-panel").forEach(panel => { panel.hidden = panel.id !== tab.getAttribute("aria-controls"); });
    }));

    // Taille du rendu = taille affichée ; pas de rendu quand l'aperçu est hors de l'écran.
    const resize = () => { viewer.width = canvas.clientWidth; viewer.height = canvas.clientHeight; };
    if ("ResizeObserver" in window) new ResizeObserver(resize).observe(canvas);
    if ("IntersectionObserver" in window) {
      new IntersectionObserver(([e]) => { viewer.renderPaused = !e.isIntersecting; }).observe(stage);
    }

    // Aperçu en direct pendant qu'on tape son pseudo (formulaire)
    let timer;
    input?.addEventListener("input", () => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        const name = input.value.trim();
        if (/^[A-Za-z0-9_]{3,16}$/.test(name)) loadPlayer(name);
      }, 450);
    });
  }

  // Boutique : un clic sur un cosmétique ouvre l'essayage (studio 3D) avec cet article porté
  const tryon = document.getElementById("tryon");
  if (tryon && tryon.showModal) {
    document.querySelectorAll("[data-try]").forEach(card => card.addEventListener("click", () => {
      tryon.showModal();
      stageReady.then(() => {
        const b = tryon.querySelector(`.equip[data-thumb="${card.dataset.try}"]`);
        if (b && b.getAttribute("aria-pressed") !== "true") b.click();
      });
    }));
    tryon.querySelector("[data-close]")?.addEventListener("click", () => tryon.close());
    // Un clic à côté de la fenêtre la ferme
    tryon.addEventListener("click", e => { if (e.target === tryon) tryon.close(); });
  }

  // ---------- Documentation ----------
  // Recherche instantanée : filtre les pages sur leur titre, description et intertitres,
  // puis sur tout leur texte dès que l'index plein texte est chargé (au premier caractère tapé).
  let fullText = null, loading = null;
  const loadIndex = () => loading ??= fetch(`/docs/search.json?lang=${document.documentElement.lang}`)
    .then(r => r.json())
    .then(list => { fullText = new Map(list.map(e => [e.slug, e.text])); })
    .catch(() => { fullText = new Map(); });
  document.querySelectorAll(".docs-search input").forEach(input => {
    const scope = input.closest(".docs-side") || input.closest(".docs-hero")?.parentElement || document;
    const filter = () => {
      // Chaque mot doit commencer un mot du texte (« ram » trouve « RAM », pas « programme »).
      const words = input.value.trim().toLowerCase().split(/\s+/).filter(Boolean)
        .map(w => new RegExp(`(^|[^\\p{L}\\p{N}])${w.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}`, "u"));
      scope.querySelectorAll(".docs-nav, .docs-map").forEach(list => {
        let shown = 0;
        list.querySelectorAll("li[data-search]").forEach(li => {
          const body = fullText?.get(li.dataset.slug) ?? "";
          const ok = words.every(w => w.test(li.dataset.search) || w.test(body));
          li.hidden = !ok;
          if (ok) shown++;
        });
        list.querySelectorAll(".nav-sec, .map-sec").forEach(sec => {
          sec.hidden = !sec.querySelector("li[data-search]:not([hidden])");
        });
        const empty = list.querySelector(".docs-empty");
        if (empty) empty.hidden = shown > 0;
      });
    };
    input.addEventListener("input", () => {
      filter();
      if (!fullText) loadIndex().then(filter);
    });
    input.addEventListener("focus", () => { loadIndex(); }, { once: true });
    input.addEventListener("keydown", e => {
      if (e.key === "Enter") {
        const first = scope.querySelector("li[data-search]:not([hidden]) a");
        if (first) location.href = first.href;
      } else if (e.key === "Escape") {
        input.value = ""; filter(); input.blur();
      }
    });
  });
  // « / » place le curseur dans la recherche
  document.addEventListener("keydown", e => {
    if (e.key !== "/" || e.ctrlKey || e.metaKey || /^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement.tagName)) return;
    const input = [...document.querySelectorAll(".docs-search input")].find(i => i.offsetParent);
    if (input) { e.preventDefault(); input.focus(); }
  });

  // Bouton « copier » sur les blocs de code
  document.querySelectorAll(".md-doc pre").forEach(pre => {
    const btn = document.createElement("button");
    btn.type = "button"; btn.className = "copy"; btn.textContent = "Copy";
    btn.addEventListener("click", () => {
      navigator.clipboard?.writeText(pre.querySelector("code")?.innerText ?? pre.innerText).then(() => {
        btn.textContent = "✓"; btn.classList.add("done");
        setTimeout(() => { btn.textContent = "Copy"; btn.classList.remove("done"); }, 1500);
      });
    });
    pre.appendChild(btn);
  });

  // Sommaire : met en avant la section en cours de lecture
  const tocLinks = [...document.querySelectorAll(".docs-toc a")];
  if (tocLinks.length && "IntersectionObserver" in window) {
    const byId = new Map(tocLinks.map(a => [decodeURIComponent(a.hash.slice(1)), a]));
    const visible = new Set();
    const spy = new IntersectionObserver(entries => {
      entries.forEach(e => e.isIntersecting ? visible.add(e.target.id) : visible.delete(e.target.id));
      const current = [...byId.keys()].find(id => visible.has(id));
      if (current) tocLinks.forEach(a => a.classList.toggle("active", a === byId.get(current)));
    }, { rootMargin: "-80px 0px -60% 0px" });
    byId.forEach((_, id) => { const h = document.getElementById(id); if (h) spy.observe(h); });
  }

  if (reduce) return;

  // Défilement fluide à la molette (inertie douce) et sur les liens d'ancre de la page
  let target = scrollY, current = scrollY, raf = 0;
  const clamp = v => Math.max(0, Math.min(document.documentElement.scrollHeight - innerHeight, v));
  const jump = y => scrollTo({ top: y, behavior: "instant" });
  const frame = () => {
    current += (target - current) * 0.1;
    if (Math.abs(target - current) < 0.5) { current = target; jump(current); raf = 0; return; }
    jump(current);
    raf = requestAnimationFrame(frame);
  };
  const animateTo = y => {
    if (!raf) current = scrollY;
    target = clamp(y);
    if (!raf) raf = requestAnimationFrame(frame);
  };
  const innerScrollable = (el, dy) => {
    for (; el && el !== document.body && el !== document.documentElement; el = el.parentElement) {
      const oy = getComputedStyle(el).overflowY;
      if ((oy === "auto" || oy === "scroll") && el.scrollHeight > el.clientHeight) {
        if (dy > 0 ? el.scrollTop + el.clientHeight < el.scrollHeight - 1 : el.scrollTop > 0) return true;
      }
    }
    return false;
  };

  addEventListener("click", e => {
    const a = e.target.closest("a[href*='#']");
    if (!a || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey) return;
    const url = new URL(a.href, location.href);
    if (url.pathname !== location.pathname || url.search !== location.search || !url.hash) return;
    const id = decodeURIComponent(url.hash.slice(1));
    const el = document.getElementById(id);
    if (!el && id !== "top") return;
    e.preventDefault();
    history.pushState(null, "", url.hash);
    if (el && el.tagName === "DETAILS") el.open = true;
    animateTo(id === "top" || !el ? 0 : el.getBoundingClientRect().top + scrollY - HEADER_OFFSET);
  });

  if (!matchMedia("(pointer: fine)").matches) return;
  addEventListener("wheel", e => {
    if (e.ctrlKey || e.defaultPrevented) return;
    let dy = e.deltaY;
    if (e.deltaMode === 1) dy *= 16; else if (e.deltaMode === 2) dy *= innerHeight;
    if (!dy || Math.abs(e.deltaX) > Math.abs(dy) || innerScrollable(e.target, dy)) return;
    e.preventDefault();
    animateTo((raf ? target : scrollY) + dy);
  }, { passive: false });
  addEventListener("scroll", () => { if (!raf) target = current = scrollY; }, { passive: true });
  ["mousedown", "keydown", "touchstart"].forEach(t =>
    addEventListener(t, () => { cancelAnimationFrame(raf); raf = 0; }, { passive: true }));
})();
