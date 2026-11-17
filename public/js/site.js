    // Blockbench// Petites améliorations côté navigateur. Tout le contenu est rendu par le serveur Rust :
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
  // Couleur des caractères ASCII selon le thème (clairs sur la nuit, bleu foncé sur le thème clair)
  const asciiRGB = () => getComputedStyle(document.documentElement).getPropertyValue("--ascii-rgb").trim() || "206 218 245";
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
      const tint = asciiRGB();
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
        g.fillStyle = `rgb(${tint} / ${(0.08 + i * 0.07).toFixed(2)})`;
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

  // Thème clair / sombre : le choix est gardé ; sans choix, on suit le système
  document.querySelectorAll("[data-theme-toggle]").forEach(b => b.addEventListener("click", () => {
    const root = document.documentElement;
    const actuel = root.dataset.theme || (matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark");
    const suivant = actuel === "light" ? "dark" : "light";
    root.dataset.theme = suivant;
    try { localStorage.setItem("theme", suivant); } catch {}
  }));

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
    const local = root => root.querySelectorAll(".st-bars [data-t]").forEach(i => {
      const t = new Date(+i.dataset.t * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
      i.title = i.title.replace(/^\d\d:\d\d UTC/, t);
    });
    local(document);
    setInterval(async () => {
      if (document.hidden) return;
      const html = await fetch(location.href, { cache: "no-store" }).then(r => r.ok ? r.text() : null).catch(() => null);
      if (!html) return;
      const next = new DOMParser().parseFromString(html, "text/html");
      for (const sel of [".term", ".st-group"]) {
        const a = [...document.querySelectorAll(sel)], b = [...next.querySelectorAll(sel)];
        if (a.length === b.length) a.forEach((el, i) => el.replaceWith(b[i]));
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
      // (un pseudo recréé reprend sa hauteur normale : on le remonte au-dessus du chapeau)
      placerPseudo();   // setName ne sert qu'après le chargement de l'aperçu (loadPlayer)
    };

    // ---- Emotes (aperçu de celles qui arrivent sur Mistly Client) ----
    // Chacune pose le joueur selon le temps écoulé (en secondes), en boucle. Pour skinview3d, une
    // rotation.z positive rapproche le bras droit du corps (et éloigne le gauche).
    const lisse = x => (x <= 0 ? 0 : x >= 1 ? 1 : x * x * (3 - 2 * x));
    // 0 → 1 entre a et b, puis 1 → 0 entre c et d
    const fenetre = (t, a, b, c, d) => lisse((t - a) / (b - a)) * (1 - lisse((t - c) / (d - c)));
    const POSES = {
      // Câlin : les bras s'ouvrent, se referment devant (sur le joueur en face), on se berce,
      // la tête penchée, puis les bras se rouvrent et retombent
      // (autour : à quel point les bras se referment ; leve : à quelle hauteur ils passent)
      hug: (p, temps, { autour = 0.8, leve = 1.35, tourne = 0 } = {}) => {
        const t = temps % 6, s = p.skin;
        const ouvert = Math.max(fenetre(t, 0, 0.6, 0.6, 1.2), fenetre(t, 4.6, 5.2, 5.2, 5.9));
        const serre = fenetre(t, 0.6, 1.2, 4.6, 5.2);
        const berce = Math.sin((t - 1.2) * Math.PI / 0.9) * serre;
        const presse = 0.08 * Math.sin(t * 5) * serre;
        s.rightArm.rotation.x = -0.3 * ouvert - leve * serre;
        s.leftArm.rotation.x = -0.3 * ouvert - leve * serre;
        s.rightArm.rotation.z = -1.25 * ouvert + (autour + presse) * serre;
        s.leftArm.rotation.z = 1.25 * ouvert - (autour + presse) * serre;
        s.body.rotation.x = -0.06 * ouvert + 0.08 * serre;
        s.head.rotation.x = -0.12 * ouvert + 0.18 * serre;
        s.head.rotation.z = 0.28 * berce;
        s.head.rotation.y = tourne * serre;   // à deux : la tête tournée (négatif : vers sa droite)
        p.rotation.z = 0.07 * berce;
      },
      // Salut : la main droite levée qui s'agite
      wave: (p, temps) => {
        const t = temps % 2.4, s = p.skin, k = fenetre(t, 0, 0.35, 2.0, 2.4);
        s.rightArm.rotation.z = -(2.55 + 0.35 * Math.sin(t * 9)) * k;
        s.rightArm.rotation.x = -0.15 * k;
        s.head.rotation.z = -0.12 * k;
        s.leftArm.rotation.z = 0.08 * k;
      },
      // Applaudir : les mains se rejoignent devant, en rythme, avec un petit rebond
      clap: (p, temps) => {
        const t = temps % 2, s = p.skin, k = fenetre(t, 0, 0.25, 1.75, 2);
        const tape = Math.abs(Math.sin(t * Math.PI * 3));
        s.rightArm.rotation.x = -1.15 * k;
        s.leftArm.rotation.x = -1.15 * k;
        s.rightArm.rotation.z = (0.2 + 0.38 * tape) * k;
        s.leftArm.rotation.z = -(0.2 + 0.38 * tape) * k;
        s.head.rotation.x = -0.1 * k;
        p.position.y = 0.5 * tape * k;
      },
      // Danse : bras qui montent l'un après l'autre, hanches qui tournent, petits sauts
      dance: (p, temps) => {
        const t = temps % 2.4, s = p.skin, w = t * Math.PI * 2 / 1.2, k = fenetre(t, 0, 0.2, 2.2, 2.4);
        s.rightArm.rotation.z = -(1.3 + 1.0 * Math.sin(w)) * k;
        s.leftArm.rotation.z = (1.3 - 1.0 * Math.sin(w)) * k;
        s.rightLeg.rotation.x = 0.35 * Math.sin(w * 2) * k;
        s.leftLeg.rotation.x = -0.35 * Math.sin(w * 2) * k;
        s.head.rotation.z = 0.2 * Math.sin(w) * k;
        p.rotation.y = 0.35 * Math.sin(w) * k;
        p.position.y = 0.9 * Math.abs(Math.sin(w * 2)) * k;
      },
      // Victoire : les deux bras en l'air et des sauts de joie
      cheer: (p, temps) => {
        const t = temps % 2.4, s = p.skin, k = fenetre(t, 0, 0.25, 2.1, 2.4);
        const saut = Math.max(0, Math.sin(t * Math.PI * 2 / 0.8));
        s.rightArm.rotation.z = -(2.75 - 0.25 * saut) * k;
        s.leftArm.rotation.z = (2.75 - 0.25 * saut) * k;
        s.rightLeg.rotation.z = -0.12 * saut * k;
        s.leftLeg.rotation.z = 0.12 * saut * k;
        s.head.rotation.x = -0.25 * k;
        p.position.y = 3 * saut * k;
      },
      // Facepalm : la main droite sur le visage, la tête qui tombe et secoue doucement
      facepalm: (p, temps) => {
        const t = temps % 3.2, s = p.skin, k = fenetre(t, 0, 0.45, 2.7, 3.2);
        s.rightArm.rotation.x = -2.25 * k;
        s.rightArm.rotation.z = 0.55 * k;
        s.head.rotation.x = 0.35 * k;
        s.head.rotation.y = 0.18 * Math.sin(t * 4) * k;
        s.body.rotation.x = 0.1 * k;
        s.leftArm.rotation.z = 0.05 * k;
      },
    };
    // Pour essayer le câlin sur quelqu'un : un deuxième joueur (skin d'Alex) en face, qui rend le
    // câlin. Le bouton « Sur un joueur » de l'aperçu le montre ou le cache.
    const ami = new sv.PlayerObject();
    ami.visible = false;
    ami.backEquipment = null;
    viewer.playerWrapper.add(ami);
    let avecAmi = true, amiHabille = false;
    const habillerAmi = () => {
      if (amiHabille || !viewer.skinTexture) return;
      amiHabille = true;
      // en attendant (ou si le skin d'Alex ne charge pas) : le même skin que le joueur
      ami.skin.map = viewer.skinTexture;
      const img = new Image();
      img.onload = () => {
        if (img.width !== 64 || img.height !== 64) return;
        const c = document.createElement("canvas");
        c.width = c.height = 64;
        c.getContext("2d").drawImage(img, 0, 0);
        // même genre de texture que celle du joueur (pixels nets, mêmes couleurs)
        const modele = viewer.skinTexture;
        const tex = new modele.constructor(c);
        tex.magFilter = modele.magFilter;
        tex.minFilter = modele.minFilter;
        tex.colorSpace = modele.colorSpace;
        tex.needsUpdate = true;
        ami.skin.map = tex;
        ami.skin.modelType = "slim";
      };
      img.src = "/skin/MHF_Alex";
    };
    // les deux face à face, vus de profil ; les bras de l'un passent par-dessus les épaules de l'autre,
    // ceux de l'autre autour de la taille
    // Un bras ne porte qu'à 10 pixels de l'épaule : pour que les mains arrivent dans le dos de
    // l'autre, les deux se rapprochent et se décalent un peu de côté, la tête de chacun passant à
    // côté de celle de l'autre. Les deux tournent la tête vers leur droite, du côté où ils sont
    // décalés : les visages regardent chacun ailleurs, joue contre joue, comme dans un vrai câlin.
    const calinADeux = (joueur, temps) => {
      joueur.position.set(-3.3, 0, 2.1);
      joueur.rotation.y = Math.PI / 2;
      POSES.hug(joueur, temps, { autour: 0.45, leve: 1.7, tourne: -0.45 });
      ami.visible = true;
      ami.position.set(3.3, 0, -2.1);
      ami.rotation.y = -Math.PI / 2;
      POSES.hug(ami, temps, { autour: 0.5, leve: 1.2, tourne: -0.45 });
    };
    const EMOTES = Object.fromEntries(Object.entries(POSES).map(([cle, pose]) => [cle, class extends sv.PlayerAnimation {
      animate(joueur) {
        if (cle === "hug" && avecAmi) calinADeux(joueur, this.progress);
        else pose(joueur, this.progress);
      }
    }]));
    // pose figée (mouvements réduits) : le moment le plus parlant de chaque emote
    const POSE_FIXE = { hug: 2, wave: 1, clap: 0.35, dance: 0.6, cheer: 0.2, facepalm: 1.5 };

    // Animations : repos, marche, course, vol, accroupi, et les emotes
    const ANIMS = { idle: sv.IdleAnimation, walk: sv.WalkingAnimation, run: sv.RunningAnimation, fly: sv.FlyingAnimation, crouch: sv.CrouchAnimation, ...EMOTES };
    const SPEED = { walk: 0.6, run: 0.8 };
    const animButtons = [...stage.querySelectorAll("[data-anim]")];
    const emoteButtons = [...document.querySelectorAll("[data-emote]")];
    const amiBtn = stage.querySelector('[data-toggle="partner"]');
    let animActuelle = "walk";
    const setAnim = key => {
      const Anim = ANIMS[key];
      animActuelle = key;
      ami.visible = false;
      ami.resetJoints();
      ami.rotation.set(0, 0, 0);
      if (key === "hug") habillerAmi();
      if (amiBtn) amiBtn.hidden = key !== "hug";
      viewer.animation = Anim ? new Anim() : null;
      if (viewer.animation) { viewer.animation.speed = SPEED[key] ?? 1; viewer.animation.paused = reduce; }
      // animations figées (mouvements réduits) : une emote s'affiche quand même, dans sa pose
      if (key in POSE_FIXE && reduce) { viewer.animation.progress = POSE_FIXE[key]; viewer.animation.paused = false; viewer.animation.speed = 0; }
      press(animButtons, animButtons.find(b => b.dataset.anim === key));
      emoteButtons.forEach(b => {
        const on = b.dataset.emote === key;
        b.setAttribute("aria-pressed", String(on));
        b.textContent = on ? b.dataset.stop : b.dataset.play;
        b.closest(".item")?.classList.toggle("equipped", on);
      });
      // Voler sans rien dans le dos : on sort les élytres
      if (key === "fly" && back.mode === "none") setBack("elytra", ownCape ? "own" : "misty");
    };
    animButtons.forEach(b => b.addEventListener("click", () => setAnim(b.dataset.anim)));
    // Maj enfoncée : le joueur s'accroupit, comme en jeu ; relâchée : il reprend son animation
    // (sauf pendant qu'on tape un pseudo, où Maj sert aux majuscules)
    let avantMaj = null;
    const relacherMaj = () => {
      if (avantMaj === null) return;
      const a = avantMaj;
      avantMaj = null;
      setAnim(a);
    };
    addEventListener("keydown", e => {
      if (e.key !== "Shift" || e.repeat || avantMaj !== null || e.target.closest?.("input, textarea, [contenteditable]")) return;
      avantMaj = animActuelle;
      setAnim("crouch");
      // accroupi tout de suite et immobile (pas l'animation qui se baisse et se relève en boucle)
      viewer.animation.runOnce = true;
      viewer.animation.progress = 0.125;
      viewer.animation.paused = false;
    });
    addEventListener("keyup", e => { if (e.key === "Shift") relacherMaj(); });
    addEventListener("blur", relacherMaj);
    // « Sur un joueur » : le câlin avec ou sans l'ami en face
    amiBtn?.addEventListener("click", () => {
      avecAmi = !avecAmi;
      amiBtn.setAttribute("aria-pressed", String(avecAmi));
      if (!avecAmi) {
        ami.visible = false;
        viewer.playerObject.position.set(0, 0, 0);
        viewer.playerObject.rotation.set(0, 0, 0);
      }
    });
    // Bouton d'une emote (onglet Emotes du studio) : la jouer, ou l'arrêter et revenir à la marche
    emoteButtons.forEach(b => b.addEventListener("click", () => setAnim(b.getAttribute("aria-pressed") === "true" ? "walk" : b.dataset.emote)));

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
    // Cape early player : la texture faite dans Blockbench (public/models/early_cape.bbmodel,
    // exportée en public/assets/capes/early.png). Les vignettes et la cape portée se redessinent
    // quand elle est chargée.
    const versionSite = new URL(document.querySelector('script[src*="site.js"]').src).searchParams.get("v") || "";
    const early = document.createElement("canvas");
    early.width = 64; early.height = 32;
    const imageEarly = new Image();
    imageEarly.onload = () => {
      early.getContext("2d").drawImage(imageEarly, 0, 0);
      dessinerVignettes();
      if (back.source === "early") setBack(back.mode, "early");
    };
    imageEarly.src = `/assets/capes/early.png?v=${versionSite}`;
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
        if (b.dataset.back === "none" || b.dataset.back === "pet") return;   // chapeau, compagnon : portés à part
        const on = source === (b.dataset.src || "misty") && b.dataset.back === mode;
        b.setAttribute("aria-pressed", String(on));
        // Les boutons de la vitrine gardent leur vignette et leur nom ; ceux du studio disent Équiper / Retirer
        if (!b.querySelector(".thumb")) b.textContent = on ? b.dataset.unequip : b.dataset.equip;
        b.closest(".item")?.classList.toggle("equipped", on);
      });
    };
    capeBtn?.addEventListener("click", () => setBack(back.mode === "cape" && back.source === "own" ? "none" : "cape", "own"));
    elytraBtn?.addEventListener("click", () => {
      if (back.mode === "elytra") setBack(ownCape ? "cape" : "none", "own");
      else setBack("elytra", ownCape && back.source === "own" ? "own" : "misty");
    });
    // Chapeaux : des modèles Blockbench (public/models/<clé>.bbmodel, format Modded Entity, UV en
    // boîte comme dans Minecraft), lus ici et posés sur la tête du joueur 3D. Les objets 3D sont
    // faits avec les classes du modèle du joueur (skinview3d n'expose pas three.js).
    const tete = viewer.playerObject.skin.head;
    const cubeModele = tete.innerLayer;
    // Blockbench : y vers le haut, la tête de 24 à 32, le devant au nord (-z). Ici : la tête de 0 à
    // 8, le devant vers +z ; on tourne donc le modèle d'un demi-tour autour de y.
    const versIci = ([x, y, z]) => [-x, y - 24, -z];
    const angles = (r = [0, 0, 0]) => [-r[0], r[1], -r[2]].map(a => a * Math.PI / 180);
    // faces du cube de three.js, dans son ordre : +x, -x, +y, -y, +z, -z (après le demi-tour)
    const FACES = ["west", "east", "up", "down", "north", "south"];
    const objetVide = () => new cubeModele.constructor();
    const construireModele = (m, image) => {
      const L = m.resolution?.width || 64, H = m.resolution?.height || 64;
      const toile = document.createElement("canvas");
      toile.width = L; toile.height = H;
      toile.getContext("2d").drawImage(image, 0, 0);
      const modeleTex = viewer.skinTexture;
      const tex = new modeleTex.constructor(toile);
      tex.magFilter = modeleTex.magFilter;
      tex.minFilter = modeleTex.minFilter;
      tex.colorSpace = modeleTex.colorSpace;
      tex.needsUpdate = true;
      const matiere = new cubeModele.material.constructor({ map: tex, transparent: true, alphaTest: 0.5, emissive: 0xffffff, emissiveMap: tex, emissiveIntensity: 0.25 });
      const cubes = new Map(m.elements.map(e => [e.uuid, e]));
      const groupes = new Map((m.groups || []).map(g => [g.uuid, g]));
      const nommes = {};
      const cubeVers = (e, origineParent) => {
        const [fx, fy, fz] = e.from, [tx, ty, tz] = e.to;
        const geo = new cubeModele.geometry.constructor(tx - fx, ty - fy, tz - fz);
        const uv = [];
        for (const nom of FACES) {
          const [x1, y1, x2, y2] = e.faces[nom].uv;
          // coins haut-gauche, haut-droit, bas-gauche, bas-droit de la face vue de dehors
          uv.push(x1 / L, 1 - y1 / H, x2 / L, 1 - y1 / H, x1 / L, 1 - y2 / H, x2 / L, 1 - y2 / H);
        }
        geo.attributes.uv.array.set(uv);
        geo.attributes.uv.needsUpdate = true;
        const pivot = objetVide();
        const o = versIci(e.origin || [0, 0, 0]), p = versIci(origineParent);
        pivot.position.set(o[0] - p[0], o[1] - p[1], o[2] - p[2]);
        pivot.rotation.order = "ZYX";
        pivot.rotation.set(...angles(e.rotation));
        const maille = new cubeModele.constructor(geo, matiere);
        const c = versIci([(fx + tx) / 2, (fy + ty) / 2, (fz + tz) / 2]);
        maille.position.set(c[0] - o[0], c[1] - o[1], c[2] - o[2]);
        pivot.add(maille);
        return pivot;
      };
      const noeud = (entree, origineParent) => {
        if (typeof entree === "string") return cubes.has(entree) ? cubeVers(cubes.get(entree), origineParent) : null;
        const g = groupes.get(entree.uuid) || entree;
        if (g.name === "ref") return null;   // la tête de référence, seulement pour modéliser
        const obj = objetVide();
        const o = versIci(g.origin || [0, 0, 0]), p = versIci(origineParent);
        obj.position.set(o[0] - p[0], o[1] - p[1], o[2] - p[2]);
        obj.rotation.order = "ZYX";
        obj.rotation.set(...angles(g.rotation));
        obj.userData.repos = obj.rotation.z;
        nommes[g.name] = obj;
        for (const enfant of entree.children || []) { const n = noeud(enfant, g.origin || [0, 0, 0]); if (n) obj.add(n); }
        return obj;
      };
      const racine = objetVide();
      racine.position.set(0, 0, 0);
      for (const entree of m.outliner) { const n = noeud(entree, [0, 24, 0]); if (n) racine.add(n); }
      racine.userData.nommes = nommes;
      racine.userData.y = 0;
      return racine;
    };
    const chapeaux = {};
    const chargerChapeau = cle => chapeaux[cle] ??= fetch(`/models/${cle === "bunny" ? "bunny_ears" : cle}.bbmodel?v=${versionSite}`)
      .then(r => r.json())
      .then(m => new Promise((ok, ko) => {
        const image = new Image();
        image.onload = () => ok(construireModele(m, image));
        image.onerror = ko;
        image.src = m.textures[0].source;
      }))
      .then(modele => { modele.visible = false; tete.add(modele); return modele; });
    let chapeauPorte = null, flotte = 0;
    // Taille du chapeau porté (le curseur « Taille », entre 60 et 140 %) : le chapeau grandit ou
    // rapetisse à partir du haut de la tête (y = 8), où il est posé
    let tailleChapeau = 1;
    const ajusterTaille = m => {
      m.scale.setScalar(tailleChapeau);
      m.userData.y = 8 * (1 - tailleChapeau);
      m.position.y = m.userData.y;
    };
    // le nuage flotte doucement, comme en jeu ; les oreilles frémissent
    const flotter = maintenant => {
      const t = maintenant / 1000;
      chapeaux[chapeauPorte]?.then(m => {
        if (chapeauPorte === "cloudly") m.position.y = m.userData.y + Math.sin(t * 1.6) * 0.45 * tailleChapeau;
        if (chapeauPorte === "bunny") ["ear_left", "ear_right"].forEach((n, i) => {
          const o = m.userData.nommes[n];
          if (o) o.rotation.z = o.userData.repos + Math.sin(t * 2.2 + i * 1.3) * 0.05;
        });
      });
      flotte = chapeauPorte && !reduce ? requestAnimationFrame(flotter) : 0;
    };
    // le pseudo au-dessus de la tête monte au-dessus du chapeau porté
    const HAUTEUR_CHAPEAU = { cloudly: 10, bunny: 13 };
    const placerPseudo = () => {
      viewer.nameTagYOffset = (viewer.playerObject.ears.visible ? 25 : 20) + (HAUTEUR_CHAPEAU[chapeauPorte] || 0) * tailleChapeau;
      if (viewer.nameTag) viewer.nameTag.position.y = viewer.nameTagYOffset;
    };
    const reglageTaille = document.getElementById("reglage-taille");
    const curseurTaille = document.getElementById("taille-chapeau");
    curseurTaille?.addEventListener("input", () => {
      tailleChapeau = Math.min(1.4, Math.max(0.6, Number(curseurTaille.value) || 1));
      document.getElementById("taille-valeur").textContent = `${Math.round(tailleChapeau * 100)} %`;
      Object.values(chapeaux).forEach(p => p.then(ajusterTaille));
      placerPseudo();
    });
    // porte un chapeau (sa clé, comme « cloudly »), ou aucun (null)
    const porterChapeau = cle => {
      chapeauPorte = cle;
      placerPseudo();
      if (reglageTaille) reglageTaille.hidden = !cle;
      // le modèle se charge la première fois qu'on le porte
      Object.entries(chapeaux).forEach(([k, p]) => p.then(m => { m.visible = k === chapeauPorte; }));
      if (cle) chargerChapeau(cle).then(m => { ajusterTaille(m); m.visible = chapeauPorte === cle; }).catch(() => {});
      equipButtons.filter(b => b.dataset.back === "none").forEach(b => {
        const on = b.dataset.thumb === cle;
        b.setAttribute("aria-pressed", String(on));
        if (!b.querySelector(".thumb")) b.textContent = on ? b.dataset.unequip : b.dataset.equip;
        b.closest(".item")?.classList.toggle("equipped", on);
      });
      if (cle && !flotte) flotte = requestAnimationFrame(flotter);
    };

    // Minime : un petit joueur assis sur l'épaule gauche, avec le skin du pseudo choisi (le sien
    // par défaut). Il est accroché au corps du joueur, donc il suit quand celui-ci s'accroupit.
    const ECHELLE_MINIME = 0.38;
    const minime = new sv.PlayerObject();
    minime.visible = false;
    minime.backEquipment = null;
    minime.scale.setScalar(ECHELLE_MINIME);
    const corps = viewer.playerObject.skin.body;
    corps.add(minime);
    // l'épaule gauche (côté +x) : le haut du corps, au-dessus du bras ; le Minime s'assoit dessus,
    // ses hanches à cette hauteur, un peu en arrière pour que ses jambes pendent devant
    const placerMinime = () => {
      const haut = corps.innerLayer.position.y + 6;
      const hanches = (minime.skin.position.y + minime.skin.leftLeg.position.y) * ECHELLE_MINIME;
      minime.position.set(6, haut - hanches + 0.4, -0.6);
    };
    let minimePorte = false, minimeNom = null, bouge = 0;
    const habillerMinime = async nom => {
      if (!nom || nom === minimeNom) return;
      minimeNom = nom;
      const info = await fetch(`/player/${nom}`).then(r => r.ok ? r.json() : null).catch(() => null);
      const image = new Image();
      image.onload = () => {
        if (minimeNom !== nom || !viewer.skinTexture) return;
        const c = document.createElement("canvas");
        c.width = 64; c.height = 64;
        const g = c.getContext("2d");
        g.drawImage(image, 0, 0, 64, image.height === 32 ? 32 : 64);
        // ancien skin (64 × 32, comme celui de Notch) : le bras et la jambe gauches n'existaient pas,
        // on les recopie en miroir depuis les droits, comme le fait skinview3d pour le joueur
        if (image.height === 32) {
          g.save();
          g.scale(-1, 1);
          const copie = (sx, sy, l, h, dx, dy) => g.drawImage(c, sx, sy, l, h, -dx, dy, -l, h);
          copie(4, 16, 4, 4, 20, 48); copie(8, 16, 4, 4, 24, 48); copie(0, 20, 4, 12, 24, 52); copie(4, 20, 4, 12, 20, 52);
          copie(8, 20, 4, 12, 16, 52); copie(12, 20, 4, 12, 28, 52); copie(44, 16, 4, 4, 36, 48); copie(48, 16, 4, 4, 40, 48);
          copie(40, 20, 4, 12, 40, 52); copie(44, 20, 4, 12, 36, 52); copie(48, 20, 4, 12, 32, 52); copie(52, 20, 4, 12, 44, 52);
          g.restore();
          // la couche « chapeau » de ces skins est souvent pleine (noire) : sans pixel transparent, on l'efface
          const chapeau = g.getImageData(32, 0, 32, 16).data;
          let transparent = false;
          for (let i = 3; i < chapeau.length; i += 4) if (chapeau[i] < 128) { transparent = true; break; }
          if (!transparent) g.clearRect(32, 0, 32, 16);
        }
        const modele = viewer.skinTexture;
        const tex = new modele.constructor(c);
        tex.magFilter = modele.magFilter;
        tex.minFilter = modele.minFilter;
        tex.colorSpace = modele.colorSpace;
        tex.needsUpdate = true;
        minime.skin.map = tex;
        minime.skin.modelType = info?.slim ? "slim" : "default";
      };
      image.src = `/skin/${nom}`;
    };
    // assis, les jambes en avant ; il fait coucou de son bras extérieur, l'autre posé
    const animerMinime = maintenant => {
      const t = maintenant / 1000, s = minime.skin;
      s.rightLeg.rotation.x = s.leftLeg.rotation.x = -Math.PI / 2;
      s.rightLeg.rotation.z = 0.12; s.leftLeg.rotation.z = -0.12;
      s.leftArm.rotation.z = 2.5 + Math.sin(t * 6) * 0.3;
      s.rightArm.rotation.x = -0.4;
      s.rightArm.rotation.z = 0.15;
      s.head.rotation.y = Math.sin(t * 0.8) * 0.25;
      bouge = minimePorte && !reduce ? requestAnimationFrame(animerMinime) : 0;
    };
    const reglageMinime = document.getElementById("reglage-minime");
    const champMinime = document.getElementById("minime-nom");
    const porterMinime = on => {
      minimePorte = on;
      if (reglageMinime) reglageMinime.hidden = !on;
      if (on) {
        const nom = champMinime?.value.trim().match(/^[A-Za-z0-9_]{3,16}$/)?.[0] || stage.dataset.name || "MHF_Steve";
        habillerMinime(nom);
        placerMinime();
        if (!bouge) bouge = requestAnimationFrame(animerMinime);
      }
      minime.visible = on;
      equipButtons.filter(b => b.dataset.back === "pet").forEach(b => b.setAttribute("aria-pressed", String(on)));
    };
    let attenteMinime;
    champMinime?.addEventListener("input", () => {
      clearTimeout(attenteMinime);
      attenteMinime = setTimeout(() => {
        const nom = champMinime.value.trim();
        if (/^[A-Za-z0-9_]{3,16}$/.test(nom)) habillerMinime(nom);
      }, 450);
    });
    const featName = document.getElementById("feat-name"), featText = document.getElementById("feat-text");
    equipButtons.forEach(b => b.addEventListener("click", () => {
      const src = b.dataset.src || "misty";
      const hat = b.dataset.back === "none";   // un chapeau : porté en plus de la cape
      const pet = b.dataset.back === "pet";    // un compagnon : sur l'épaule, en plus du reste
      const on = hat ? chapeauPorte === b.dataset.thumb : pet ? minimePorte : back.mode === b.dataset.back && back.source === src;
      if (hat) porterChapeau(on ? null : b.dataset.thumb);
      else if (pet) porterMinime(!on);
      else setBack(on ? (ownCape ? "cape" : "none") : b.dataset.back, on ? "own" : src);
      // Vitrine : le nom et la description de l'article choisi
      if (!on && featName && b.dataset.name) {
        featName.textContent = b.dataset.name;
        featText.textContent = b.dataset.text;
        const badge = document.getElementById("feat-badge");
        badge.textContent = b.dataset.badge;
        badge.className = "badge" + (b.dataset.early ? " early" : b.dataset.soon ? "" : " free");
      }
    }));
    // Une emote demandée par la boutique (tendances, catalogue) : jouée dans l'aperçu, avec son
    // nom et sa description dans la fiche à côté
    stage.addEventListener("emote", e => {
      const { cle, nom, texte, badge } = e.detail;
      setAnim(cle);
      if (featName && nom) {
        featName.textContent = nom;
        featText.textContent = texte;
        const b = document.getElementById("feat-badge");
        b.textContent = badge;
        b.className = cle === "hug" ? "badge new" : "badge";
        equipButtons.forEach(x => x.closest(".pick") && x.setAttribute("aria-pressed", "false"));
      }
    });

    // Vignette d'une emote : un petit personnage en pixels dans la pose de l'emote, vu de face.
    // Angles des bras et des jambes en degrés depuis la verticale (positif : vers l'extérieur).
    const POSES_2D = {
      hug: { bras: [-118, -118], tete: 8, deco: "coeur" },
      wave: { bras: [8, 152], tete: -6, deco: "vagues" },
      clap: { bras: [-38, -38], deco: "etoiles" },
      dance: { bras: [138, 28], jambes: [22, 0], haut: -1, tete: 8, deco: "note" },
      cheer: { bras: [158, 158], jambes: [10, 10], haut: -4, deco: "etincelles" },
      facepalm: { bras: [-162, 6], tete: 10, deco: "goutte" },
    };
    const COULEURS = { cheveux: "#3b2a1d", peau: "#c08d6b", yeux: "#3a3f9e", haut: "#2fa3a9", bas: "#3f45b3", pieds: "#4c4c55" };
    const dessinerEmote = (c, cle) => {
      const pose = POSES_2D[cle];
      if (!pose) return;
      // dessin en petit (30 × 48), net, puis agrandi pixel par pixel
      const p = document.createElement("canvas");
      p.width = 30; p.height = 48;
      const g = p.getContext("2d");
      const y0 = 10 + (pose.haut || 0);
      const membre = (x, y, angle, h, couleurs) => {
        g.save(); g.translate(x, y); g.rotate(angle * Math.PI / 180);
        couleurs.forEach(([de, a, coul]) => { g.fillStyle = coul; g.fillRect(-2, de, 4, a - de); });
        g.restore();
      };
      const [jg, jd] = pose.jambes || [0, 0];
      membre(13, y0 + 20, jg, 12, [[0, 9, COULEURS.bas], [9, 12, COULEURS.pieds]]);
      membre(17, y0 + 20, -jd, 12, [[0, 9, COULEURS.bas], [9, 12, COULEURS.pieds]]);
      g.fillStyle = COULEURS.haut; g.fillRect(11, y0 + 8, 8, 12);
      g.save(); g.translate(15, y0 + 8); g.rotate((pose.tete || 0) * Math.PI / 180);
      g.fillStyle = COULEURS.peau; g.fillRect(-4, -8, 8, 8);
      g.fillStyle = COULEURS.cheveux; g.fillRect(-4, -8, 8, 3);
      g.fillStyle = COULEURS.yeux; g.fillRect(-3, -4, 1, 1); g.fillRect(2, -4, 1, 1);
      g.restore();
      // bras par-dessus le corps (croisés pour le câlin, sur le visage pour le facepalm)
      const [bg, bd] = pose.bras;
      membre(9, y0 + 9, bg, 12, [[0, 4, COULEURS.haut], [4, 12, COULEURS.peau]]);
      membre(21, y0 + 9, -bd, 12, [[0, 4, COULEURS.haut], [4, 12, COULEURS.peau]]);
      // petit décor qui dit l'émotion
      const px = (x, y, coul) => { g.fillStyle = coul; g.fillRect(x, y, 1, 1); };
      const motif = (lignes, x, y, coul) => lignes.forEach((l, j) => [...l].forEach((ch, i) => ch === "#" && px(x + i, y + j, coul)));
      if (pose.deco === "coeur") motif([".#.#.", "#####", "#####", ".###.", "..#.."], 12, 1, "#ff7aa8");
      if (pose.deco === "note") motif(["..##", "..#.", "..#.", "###.", "##.."], 22, 2, "#ffd36b");
      if (pose.deco === "etoiles") { motif([".#.", "###", ".#."], 5, 12, "#ffd36b"); motif([".#.", "###", ".#."], 22, 10, "#ffd36b"); }
      if (pose.deco === "etincelles") { motif(["#.#", ".#.", "#.#"], 3, 4, "#ffd36b"); motif(["#.#", ".#.", "#.#"], 24, 3, "#ffd36b"); }
      if (pose.deco === "vagues") { motif(["#..", ".#.", "..#"], 24, 4, "#9cc0ff"); motif(["#.", ".#"], 27, 9, "#9cc0ff"); }
      if (pose.deco === "goutte") motif([".#", "##", "##"], 22, 4, "#7fb8ff");
      // pixels nets : chaque pixel est plein ou vide
      const img = g.getImageData(0, 0, p.width, p.height);
      for (let i = 3; i < img.data.length; i += 4) img.data[i] = img.data[i] > 110 ? 255 : 0;
      g.putImageData(img, 0, 0);
      const d = c.getContext("2d");
      d.imageSmoothingEnabled = false;
      const echelle = Math.min(c.width / p.width, c.height / p.height);
      d.drawImage(p, (c.width - p.width * echelle) / 2, (c.height - p.height * echelle) / 2, p.width * echelle, p.height * echelle);
    };
    // Vignettes des articles Mistly, dessinées depuis la texture (et celles des emotes)
    const dessinerVignettes = () => document.querySelectorAll("canvas.thumb").forEach(c => {
      if (c.dataset.thumb.startsWith("emote-")) return dessinerEmote(c, c.dataset.thumb.slice(6));
      const g = c.getContext("2d");
      g.clearRect(0, 0, c.width, c.height);
      g.imageSmoothingEnabled = false;
      if (c.dataset.thumb === "cape") g.drawImage(misty, 1, 1, 10, 16, 0, 0, c.width, c.height);
      else if (c.dataset.thumb === "early") g.drawImage(early, 1, 1, 10, 16, 0, 0, c.width, c.height);
      else if (c.dataset.thumb === "minime") dessinerEmote(c, "wave");   // un petit joueur qui fait coucou
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
      } else if (c.dataset.thumb === "bunny") {
        // Oreilles de lapin : deux oreilles blanches, l'intérieur rose
        const rows = [".##....##.", "#pp#..#pp#", "#pp#..#pp#", "#pp#..#pp#", "#pp#..#pp#", "#pp#..#pp#", ".##....##.", ".##....##."];
        const px = Math.floor(c.width / 12);
        const ox = (c.width - 10 * px) / 2, oy = (c.height - rows.length * px) / 2;
        rows.forEach((row, y) => [...row].forEach((ch, x) => {
          if (ch === ".") return;
          g.fillStyle = ch === "p" ? "#f3a3bd" : "#f5f5f8";
          g.fillRect(ox + x * px, oy + y * px, px, px);
        }));
      } else {
        // Élytres : une aile et son reflet
        const w = c.width / 2;
        g.drawImage(misty, 34, 2, 9, 20, 0, 0, w, c.height);
        g.save(); g.scale(-1, 1); g.drawImage(misty, 34, 2, 9, 20, -c.width, 0, w, c.height); g.restore();
      }
    });
    dessinerVignettes();

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

    // Catalogue des emotes (/shop/emotes) : chaque bouton joue son emote
    const choix = [...document.querySelectorAll(".emote-choice")];
    const jouerChoix = b => {
      press(choix, b);
      const d = b.dataset;
      stage.dispatchEvent(new CustomEvent("emote", { detail: { cle: d.try, nom: d.name, texte: d.text, badge: d.badge } }));
    };
    choix.forEach(b => b.addEventListener("click", () => jouerChoix(b)));
    // Une page de catalogue (emotes, capes, chapeaux) s'ouvre sur le premier de sa liste
    const premierChoix = document.querySelector("[data-catalogue] .pick button");
    if (premierChoix) stageReady.then(() => premierChoix.click());

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

  // Bouton « Retour » des pages de la boutique : la page d'avant si elle est sur le site
  // (en gardant sa position), sinon le lien vers la boutique
  document.querySelectorAll("[data-retour]").forEach(a => a.addEventListener("click", e => {
    let avant = null;
    try { avant = document.referrer ? new URL(document.referrer).origin : null; } catch {}
    if (avant === location.origin && history.length > 1) { e.preventDefault(); history.back(); }
  }));

  // Boutique : un clic sur un cosmétique déplie l'essayage (studio 3D) dans la page, avec cet article porté
  const tryon = document.getElementById("tryon");
  if (tryon) {
    const ouvrir = () => {
      if (!tryon.hidden && tryon.classList.contains("open")) return;
      tryon.hidden = false;
      requestAnimationFrame(() => requestAnimationFrame(() => tryon.classList.add("open")));
    };
    const fermer = () => {
      tryon.classList.remove("open");
      setTimeout(() => { if (!tryon.classList.contains("open")) tryon.hidden = true; }, 460);
    };
    if (!tryon.hidden) ouvrir();
    document.querySelectorAll("[data-try]").forEach(card => card.addEventListener("click", () => {
      ouvrir();
      const haut = tryon.getBoundingClientRect().top + scrollY - 90;
      scrollTo({ top: haut, behavior: matchMedia("(prefers-reduced-motion: reduce)").matches ? "auto" : "smooth" });
      stageReady.then(() => {
        // une emote à jouer, ou un cosmétique à porter
        if (card.dataset.carteEmote !== undefined) {
          const d = card.dataset;
          document.getElementById("stage")?.dispatchEvent(new CustomEvent("emote", { detail: { cle: d.try, nom: d.name, texte: d.text, badge: d.badge } }));
          return;
        }
        const b = tryon.querySelector(`.equip[data-thumb="${card.dataset.try}"]`);
        if (b && b.getAttribute("aria-pressed") !== "true") b.click();
      });
    }));
    tryon.querySelector("[data-close]")?.addEventListener("click", fermer);
    addEventListener("keydown", e => { if (e.key === "Escape" && tryon.classList.contains("open")) fermer(); });
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
    // `/docs?q=…` (recherche lancée depuis les résultats de Google) : on filtre tout de suite
    const q = new URLSearchParams(location.search).get("q");
    if (q && input.closest(".docs-hero")) { input.value = q; filter(); loadIndex().then(filter); }
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
