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
      const s2 = dlx.querySelector(".s2");
      s2.textContent = href.includes("linux") ? s2.dataset.linux : href.includes("portable") ? s2.dataset.portable : s2.dataset.setup;
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
  // le menu Docs s'ouvre au survol : il referme les autres, pour ne pas se superposer à eux
  document.querySelectorAll(".navdd").forEach(d => d.addEventListener("mouseenter", () => menus.forEach(m => { m.open = false; })));

  // Accordéons (FAQ, patch notes) : la hauteur glisse à l'ouverture et à la fermeture. Dans la
  // FAQ (details name="faq"), ouvrir une question referme celle qui était ouverte.
  const accordeons = [...document.querySelectorAll(".qa details, details.rel")];
  accordeons.forEach(d => {
    const summary = d.querySelector("summary");
    if (!summary || reduce || !d.animate) return;   // sans animation : le navigateur gère name="faq"
    // le groupe est géré ici (sinon le navigateur fermerait les autres d'un coup, sans glisser)
    const groupe = d.getAttribute("name");
    d.removeAttribute("name");
    d.dataset.groupe = groupe || "";
    let anim = null;
    const glisser = (de, a, fin) => {
      anim?.cancel();
      d.style.overflow = "hidden";
      anim = d.animate({ height: [`${de}px`, `${a}px`] }, { duration: 320, easing: "cubic-bezier(.25,.8,.25,1)" });
      anim.onfinish = () => { anim = null; d.style.overflow = ""; fin?.(); };
    };
    d.fermer = () => {
      if (!d.open || d.classList.contains("closing")) return;
      d.classList.add("closing");
      glisser(d.offsetHeight, summary.offsetHeight, () => { d.open = false; d.classList.remove("closing"); });
    };
    summary.addEventListener("click", e => {
      e.preventDefault();
      if (d.open && !d.classList.contains("closing")) { d.fermer(); return; }
      if (groupe) accordeons.forEach(autre => { if (autre !== d && autre.dataset.groupe === groupe) autre.fermer?.(); });
      const depart = d.offsetHeight;
      d.classList.remove("closing");
      d.open = true;
      glisser(depart, d.scrollHeight);
    });
  });

  // Patch notes : les anciens liens vers une version (ex. /patch-notes#v0.3.1) mènent à sa page
  const openFromHash = () => {
    const id = decodeURIComponent(location.hash.slice(1));
    const el = id ? document.getElementById(id) : null;
    if (el?.classList.contains("pn-card")) location.replace(el.href);
    else if (el && el.tagName === "DETAILS") el.open = true;
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

    // Pseudo au-dessus de la tête, comme en jeu (le bouton « Pseudo » le cache ou le montre)
    let pseudoVisible = true;
    const nameBtn = stage.querySelector('[data-toggle="name"]');
    nameBtn?.addEventListener("click", () => {
      pseudoVisible = !pseudoVisible;
      nameBtn.setAttribute("aria-pressed", String(pseudoVisible));
      if (viewer.nameTag) viewer.nameTag.visible = pseudoVisible;
    });
    const setName = name => {
      viewer.nameTag = name
        ? new sv.NameTagObject(name, { font: "48px Silkscreen", height: 2.4, margin: [4, 8, 2, 8], textStyle: "#eaf1ff", backgroundStyle: "rgba(10,15,29,.55)" })
        : null;
      if (viewer.nameTag) viewer.nameTag.visible = pseudoVisible;
      // (un pseudo recréé reprend sa hauteur normale : on le remonte au-dessus du chapeau)
      placerPseudo();   // setName ne sert qu'après le chargement de l'aperçu (loadPlayer)
    };

    // ---- Emotes (aperçu de celles qui arrivent sur Mistly Client) ----
    // Elles sont animées dans Blockbench : public/models/emotes.bbmodel (format Bedrock), une
    // animation par emote, plus « hug_duo » (le câlin sur un joueur). Le rig a deux joueurs :
    // « joueur » (head, body, right_arm…) et « ami » (ami_head, ami_body…), caché par une échelle 0
    // dans les emotes en solo. Le fichier n'est chargé qu'à la première emote jouée.
    let emotesBB = null, chargementEmotes = null;
    const lireEmotes = projet => Object.fromEntries((projet.animations || []).map(a => {
      const os = {};
      for (const an of Object.values(a.animators || {})) {
        if (!an.keyframes?.length) continue;
        const canaux = os[an.name] ??= {};
        for (const k of an.keyframes) {
          const p = k.data_points?.[0];
          if (p) (canaux[k.channel] ??= []).push({ t: k.time, v: [p.x, p.y, p.z].map(x => parseFloat(x) || 0), interp: k.interpolation });
        }
        Object.values(canaux).forEach(c => c.sort((x, y) => x.t - y.t));
      }
      return [a.name, { duree: a.length || 1, boucle: a.loop === "loop", os }];
    }));
    const chargerEmotes = () => chargementEmotes ??= fetch(`/models/emotes.bbmodel?v=${new URL(document.querySelector('script[src*="site.js"]').src).searchParams.get("v") || ""}`)
      .then(r => (r.ok ? r.json() : Promise.reject(r.status)))
      .then(projet => { emotesBB = lireEmotes(projet); })
      .catch(() => {});
    // valeur d'une piste à l'instant t, entre ses images clés (linéaire, palier ou Catmull-Rom,
    // comme dans Blockbench)
    const catmull = (p0, p1, p2, p3, u) => 0.5 * (2 * p1 + (p2 - p0) * u + (2 * p0 - 5 * p1 + 4 * p2 - p3) * u * u + (3 * p1 - p0 - 3 * p2 + p3) * u * u * u);
    const valeur = (cles, t, defaut) => {
      if (!cles?.length) return defaut;
      const n = cles.length;
      if (t <= cles[0].t) return cles[0].v;
      if (t >= cles[n - 1].t) return cles[n - 1].v;
      let i = 0;
      while (cles[i + 1].t < t) i++;
      const a = cles[i], b = cles[i + 1], u = (t - a.t) / (b.t - a.t);
      if (a.interp === "step") return a.v;
      if (a.interp === "catmullrom" || b.interp === "catmullrom") {
        const p0 = (cles[i - 1] || a).v, p3 = (cles[i + 2] || b).v;
        return a.v.map((x, c) => catmull(p0[c], x, b.v[c], p3[c], u));
      }
      return a.v.map((x, c) => x + (b.v[c] - x) * u);
    };
    // Blockbench → skinview3d : le rig regarde vers -z, le joueur du site vers +z (demi-tour autour
    // de y), d'où x et z inversés. Les rotations sont en degrés, dans l'ordre ZYX de Blockbench.
    const ZERO = [0, 0, 0], UN = [1, 1, 1], DEG = Math.PI / 180;
    const poserOs = (objet, os, t) => {
      const r = valeur(os?.rotation, t, ZERO), p = valeur(os?.position, t, ZERO);
      objet.rotation.order = "ZYX";
      objet.rotation.set(-r[0] * DEG, r[1] * DEG, -r[2] * DEG);
      const base = objet.userData.repos ??= objet.position.clone();
      objet.position.set(base.x - p[0], base.y + p[1], base.z - p[2]);
    };
    const PARTIES = [["head", "head"], ["body", "body"], ["right_arm", "rightArm"], ["left_arm", "leftArm"], ["right_leg", "rightLeg"], ["left_leg", "leftLeg"]];
    // remet un joueur dans l'ordre de rotation habituel (pour les autres animations)
    const ordreNormal = p => [p, ...PARTIES.map(([, cle]) => p.skin[cle])].forEach(o => { o.rotation.order = "XYZ"; });
    // Pour essayer le câlin sur quelqu'un : un deuxième joueur (skin d'Alex) en face, qui rend le
    // câlin. Le bouton « Sur un joueur » de l'aperçu le montre ou le cache.
    const ami = new sv.PlayerObject();
    ami.visible = false;
    ami.backEquipment = null;
    viewer.playerWrapper.add(ami);
    let avecAmi = true, amiNom = null;
    // Son skin : celui du pseudo tapé dans « Skin de l'autre joueur », Alex sinon
    const champAmi = document.getElementById("ami-nom");
    const habillerAmi = async () => {
      if (!viewer.skinTexture) return;
      const nom = champAmi?.value.trim().match(/^[A-Za-z0-9_]{3,16}$/)?.[0] || "MHF_Alex";
      if (nom === amiNom) return;
      // en attendant (ou si son skin ne charge pas) : le même skin que le joueur
      if (amiNom === null) ami.skin.map = viewer.skinTexture;
      amiNom = nom;
      const info = nom === "MHF_Alex" ? { slim: true } : await fetch(`/player/${nom}`).then(r => (r.ok ? r.json() : null)).catch(() => null);
      const image = new Image();
      image.onload = () => {
        if (amiNom !== nom) return;
        ami.skin.map = textureDeSkin(image);
        ami.skin.modelType = info?.slim ? "slim" : "default";
      };
      image.src = `/skin/${nom}`;
    };
    let attenteAmi;
    champAmi?.addEventListener("input", () => {
      clearTimeout(attenteAmi);
      attenteAmi = setTimeout(habillerAmi, 450);
    });
    // Sur le dos : le porteur reste sur place, marche, court ou s'accroupit (une animation chacun)
    let allure = "run";
    const boutonsAllure = [...document.querySelectorAll("[data-allure]")];
    boutonsAllure.forEach(b => b.addEventListener("click", () => {
      allure = b.dataset.allure;
      boutonsAllure.forEach(x => x.setAttribute("aria-pressed", String(x === b)));
    }));
    // les réglages des emotes à deux, montrés seulement quand ils servent
    const reglageAmi = document.getElementById("reglage-ami"), reglagePorteur = document.getElementById("reglage-porteur");
    const majReglagesEmote = key => {
      if (reglageAmi) reglageAmi.hidden = !(key === "piggyback" || (key === "hug" && avecAmi));
      if (reglagePorteur) reglagePorteur.hidden = key !== "piggyback";
    };
    // Joue une animation du .bbmodel sur le joueur (et l'ami, s'il a une échelle non nulle)
    const jouerEmote = (anim, joueur, t) => {
      poserOs(joueur, anim.os.joueur, t);
      PARTIES.forEach(([nom, cle]) => poserOs(joueur.skin[cle], anim.os[nom], t));
      ami.visible = valeur(anim.os.ami?.scale, t, ZERO)[0] > 0.01;
      if (!ami.visible) return;
      const e = valeur(anim.os.ami.scale, t, UN);
      ami.scale.set(e[0], e[1], e[2]);
      poserOs(ami, anim.os.ami, t);
      PARTIES.forEach(([nom, cle]) => poserOs(ami.skin[cle], anim.os["ami_" + nom], t));
    };
    const NOMS_EMOTES = ["hug", "wave", "clap", "dance", "cheer", "facepalm", "piggyback"];
    const EMOTES = Object.fromEntries(NOMS_EMOTES.map(cle => [cle, class extends sv.PlayerAnimation {
      animate(joueur) {
        // « Sur un joueur » : le câlin à deux
        const nom = cle === "hug" && avecAmi ? "hug_duo" : cle === "piggyback" && allure !== "run" ? "piggyback_" + allure : cle;
        const anim = emotesBB?.[nom];
        if (!anim) return;
        jouerEmote(anim, joueur, anim.boucle ? this.progress % anim.duree : Math.min(this.progress, anim.duree));
        // sur le dos : le pseudo passe au-dessus de la tête de l'ami (remis en place au changement d'animation)
        if (cle === "piggyback" && viewer.nameTag) viewer.nameTag.position.y = viewer.nameTagYOffset + 11;
      }
    }]));
    // pose figée (mouvements réduits) : le moment le plus parlant de chaque emote
    const POSE_FIXE = { hug: 2, wave: 1, clap: 0.35, dance: 0.6, cheer: 0.2, facepalm: 1.5, piggyback: 0.1 };

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
      ami.scale.set(1, 1, 1);
      ordreNormal(viewer.playerObject);
      ordreNormal(ami);
      if (key in EMOTES) chargerEmotes();
      if (key === "hug" || key === "piggyback") habillerAmi();
      if (amiBtn) amiBtn.hidden = key !== "hug";
      majReglagesEmote(key);
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
      majReglagesEmote(animActuelle);
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
        obj.userData.reposY = obj.rotation.y;
        nommes[g.name] = obj;
        for (const enfant of entree.children || []) { const n = noeud(enfant, g.origin || [0, 0, 0]); if (n) obj.add(n); }
        return obj;
      };
      const racine = objetVide();
      racine.position.set(0, 0, 0);
      for (const entree of m.outliner) { const n = noeud(entree, [0, 24, 0]); if (n) racine.add(n); }
      racine.userData.nommes = nommes;
      // la texture reste modifiable (couleur des ailes)
      racine.userData.toile = toile;
      racine.userData.texture = tex;
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
    // Taille du chapeau porté (le curseur « Taille », de 60 % à 100 %, sa taille d'origine ; 80 % par
    // défaut) : il rapetisse à partir du haut de la tête (y = 8), où il est posé
    let tailleChapeau = 0.8;
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
    const HAUTEUR_CHAPEAU = { cloudly: 8, bunny: 13 };
    const placerPseudo = () => {
      viewer.nameTagYOffset = (viewer.playerObject.ears.visible ? 25 : 20) + (HAUTEUR_CHAPEAU[chapeauPorte] || 0) * tailleChapeau;
      if (viewer.nameTag) viewer.nameTag.position.y = viewer.nameTagYOffset;
    };
    const reglageTaille = document.getElementById("reglage-taille");
    const curseurTaille = document.getElementById("taille-chapeau");
    curseurTaille?.addEventListener("input", () => {
      tailleChapeau = Math.min(1, Math.max(0.6, Number(curseurTaille.value) || 1));
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
    // épaule : 1 à droite quand on regarde le joueur (son épaule gauche, côté +x), -1 de l'autre côté
    let epaule = 1;
    const placerMinime = () => {
      const haut = corps.innerLayer.position.y + 6;
      const hanches = (minime.skin.position.y + minime.skin.leftLeg.position.y) * ECHELLE_MINIME;
      // un peu vers l'extérieur de l'épaule : sa tête ne touche pas celle du joueur
      minime.position.set(7 * epaule, haut - hanches + 0.4, -0.6);
    };
    let minimePorte = false, minimeNom = null, bouge = 0;
    const habillerMinime = async nom => {
      if (!nom || nom === minimeNom) return;
      minimeNom = nom;
      const info = await fetch(`/player/${nom}`).then(r => r.ok ? r.json() : null).catch(() => null);
      const image = new Image();
      image.onload = () => {
        if (minimeNom !== nom || !viewer.skinTexture) return;
        minime.skin.map = textureDeSkin(image);
        minime.skin.modelType = info?.slim ? "slim" : "default";
      };
      image.src = `/skin/${nom}`;
    };
    // Texture d'un skin chargé (image), prête pour un PlayerObject : même genre que celle du joueur
    // (pixels nets, mêmes couleurs). Les anciens skins 64 × 32 sont convertis.
    function textureDeSkin(image) {
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
      return tex;
    }
    // assis, les jambes en avant ; il fait coucou de son bras extérieur, l'autre posé
    // assis à califourchon sur l'épaule : les jambes écartées pendent devant, les bras posés
    // dessus. (Pour skinview3d, une rotation.z positive écarte le bras ou la jambe gauche, une
    // négative le droit.)
    const poserMinime = maintenant => {
      const t = maintenant / 1000, s = minime.skin;
      s.rightLeg.rotation.x = s.leftLeg.rotation.x = -1.25;
      s.rightLeg.rotation.z = -0.38; s.leftLeg.rotation.z = 0.38;
      // les deux bras baissés, posés devant sur les jambes, qui se balancent à peine
      const balance = Math.sin(t * 2) * 0.06;
      s.leftArm.rotation.x = s.rightArm.rotation.x = -0.75 + balance;
      s.leftArm.rotation.z = 0.12;
      s.rightArm.rotation.z = -0.12;
      // la tête regarde un peu vers l'extérieur, loin de celle du joueur
      s.head.rotation.y = Math.sin(t * 0.8) * 0.2 + epaule * 0.2;
      s.head.rotation.z = -epaule * 0.05;
    };
    const animerMinime = maintenant => {
      poserMinime(maintenant);
      bouge = minimePorte && !reduce ? requestAnimationFrame(animerMinime) : 0;
    };
    const reglageMinime = document.getElementById("reglage-minime");
    const reglageEpaule = document.getElementById("reglage-epaule");
    const boutonsEpaule = [...document.querySelectorAll("[data-epaule]")];
    boutonsEpaule.forEach(b => b.addEventListener("click", () => {
      epaule = b.dataset.epaule === "left" ? -1 : 1;
      press(boutonsEpaule, b);
      // l'autre bras redescend avant que la pose ne change de côté
      minime.skin.leftArm.rotation.set(0, 0, 0);
      minime.skin.rightArm.rotation.set(0, 0, 0);
      placerMinime();
      poserMinime(performance.now());
    }));
    const champMinime = document.getElementById("minime-nom");
    const porterMinime = on => {
      minimePorte = on;
      if (reglageMinime) reglageMinime.hidden = !on;
      if (reglageEpaule) reglageEpaule.hidden = !on;
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
    // Ailes de brume : un modèle Blockbench (public/models/mist_wings.bbmodel) accroché au corps, il
    // suit donc le joueur quand il s'accroupit. Portées en plus de la cape, un peu derrière elle. Elles
    // battent doucement, plus vite et plus fort pendant le vol.
    let ailesPortees = false, ailes = null, battement = 0;
    // Couleurs des ailes. Dans la texture du modèle, la membrane occupe les lignes du haut (au-dessus
    // de ZONE_OS) et les os les lignes suivantes, jusqu'à ZONE_FIN (en dessous : le corps de
    // référence, jamais affiché). Chaque pixel d'une zone prend la teinte voulue à la même
    // luminosité : du plus sombre au plus clair de ses 4 tons. null : la couleur d'origine du modèle
    // (membrane cramoisie, os beiges).
    const ZONE_OS = 32, ZONE_FIN = 96;
    const PRESETS_AILES = {
      // Mistly : une membrane du noir (en haut) au bleu jusqu'au blanc des nuages (le bord
      // découpé), des os du noir au bleu acier. vertical : part du dégradé qui suit la hauteur du
      // pixel dans la membrane plutôt que sa luminosité.
      misty: { membrane: ["#04050c", "#1d3a8a", "#5b8cf0", "#ffffff"], os: ["#030305", "#151824", "#2e3448", "#9bb8f0"], vertical: 0.85 },
      crimson: { membrane: null, os: null },
      amethyst: { membrane: ["#1d0833", "#4a1680", "#8a3fd0", "#d2a6ff"], os: ["#120a1c", "#3a2a4a", "#8a78a0", "#efe6ff"] },
      jade: { membrane: ["#04241a", "#0d5c3c", "#1f9e66", "#8be8b4"], os: null },
      gold: { membrane: ["#2e1a04", "#7a4c0c", "#d0951f", "#ffe08a"], os: ["#1a0f05", "#5a3a18", "#b98a4a", "#fff0c8"] },
      shadow: { membrane: ["#07070b", "#1a1a24", "#34344a", "#7a7a96"], os: ["#030305", "#1e1e26", "#4a4a5a", "#c8c8d8"] },
      snow: { membrane: ["#4a5468", "#9aa6bd", "#d6deec", "#ffffff"], os: ["#2a3346", "#7a8aa6", "#c9d6ee", "#ffffff"] },
    };
    // Une couleur choisie librement : ses 4 tons, de presque noir à presque blanc
    const hexRgb = h => [1, 3, 5].map(k => parseInt(h.slice(k, k + 2), 16));
    const rgbHex = c => "#" + c.map(v => Math.round(v).toString(16).padStart(2, "0")).join("");
    const melange = (a, b, f) => a.map((v, i) => v + (b[i] - v) * f);
    const degrade = h => { const c = hexRgb(h); return [melange(c, [0, 0, 0], 0.88), melange(c, [0, 0, 0], 0.55), c, melange(c, [255, 255, 255], 0.7)].map(rgbHex); };
    // le choix : un preset, et éventuellement une couleur libre pour la membrane et pour les os
    let choixAiles = { preset: "misty", membrane: null, os: null };
    try { Object.assign(choixAiles, JSON.parse(localStorage.getItem("mistly-ailes") || "{}")); } catch { }
    if (!PRESETS_AILES[choixAiles.preset]) choixAiles.preset = "misty";
    const tonsAiles = partie => {
      const libre = choixAiles[partie];
      return /^#[0-9a-f]{6}$/i.test(libre || "") ? degrade(libre) : PRESETS_AILES[choixAiles.preset][partie];
    };
    const teindreAiles = m => {
      const { toile, texture } = m.userData;
      if (!toile) return;
      const g2 = toile.getContext("2d");
      const L = toile.width, H = Math.min(ZONE_FIN, toile.height);
      m.userData.origine ??= g2.getImageData(0, 0, L, H);
      const src = m.userData.origine.data, img = new ImageData(new Uint8ClampedArray(src), L, H);
      const lum = i => 0.299 * src[i] + 0.587 * src[i + 1] + 0.114 * src[i + 2];
      for (const [partie, debut, fin] of [["membrane", 0, ZONE_OS], ["os", ZONE_OS, H]]) {
        const tons = tonsAiles(partie);
        if (!tons) continue;
        const a = debut * L * 4, b = fin * L * 4;
        // bas[x] : la dernière ligne de la colonne x, pour que le bord découpé soit partout le plus clair
        let min = 255, max = 0;
        const bas = new Array(L).fill(debut);
        for (let i = a; i < b; i += 4) if (src[i + 3]) { const l = lum(i); min = Math.min(min, l); max = Math.max(max, l); bas[(i / 4) % L] = Math.floor(i / 4 / L); }
        const vertical = partie === "membrane" && !choixAiles.membrane ? PRESETS_AILES[choixAiles.preset].vertical || 0 : 0;
        const rgb = tons.map(hexRgb);
        for (let i = a; i < b; i += 4) {
          if (!src[i + 3]) continue;
          const hauteur = (Math.floor(i / 4 / L) - debut) / (bas[(i / 4) % L] - debut || 1);
          const t = ((1 - vertical) * (lum(i) - min) / (max - min || 1) + vertical * hauteur) * (rgb.length - 1);
          const k = Math.min(rgb.length - 2, Math.floor(t)), f = t - k;
          for (let c = 0; c < 3; c++) img.data[i + c] = Math.round(rgb[k][c] + (rgb[k + 1][c] - rgb[k][c]) * f);
        }
      }
      g2.putImageData(img, 0, 0);
      texture.needsUpdate = true;
    };
    const reglagesAiles = [...document.querySelectorAll("[data-reglage-ailes]")];
    const pastillesAiles = [...document.querySelectorAll("[data-teinte]")];
    const choixLibres = { membrane: document.getElementById("ailes-membrane"), os: document.getElementById("ailes-os") };
    const afficherChoix = () => {
      pastillesAiles.forEach(b => b.setAttribute("aria-pressed", String(b.dataset.teinte === choixAiles.preset && !choixAiles.membrane && !choixAiles.os)));
      // les sélecteurs montrent le ton principal de ce qui est porté (beige / cramoisi : l'origine)
      for (const [partie, champ] of Object.entries(choixLibres)) {
        if (champ) champ.value = tonsAiles(partie)?.[2] ?? (partie === "os" ? "#d9bf8c" : "#a01a37");
      }
    };
    const appliquerAiles = () => {
      try { localStorage.setItem("mistly-ailes", JSON.stringify(choixAiles)); } catch { }
      afficherChoix();
      ailes?.then(teindreAiles);
    };
    afficherChoix();
    pastillesAiles.forEach(b => b.addEventListener("click", () => {
      choixAiles = { preset: b.dataset.teinte, membrane: null, os: null };
      appliquerAiles();
    }));
    for (const [partie, champ] of Object.entries(choixLibres)) {
      champ?.addEventListener("input", () => { choixAiles[partie] = champ.value; appliquerAiles(); });
    }
    // contre le dos, ou derrière la cape et les élytres quand elles sont portées
    const placerAiles = m => { m.position.z = back.mode === "cape" || back.mode === "elytra" ? -1.5 : -0.4; };
    const chargerAiles = () => ailes ??= fetch(`/models/mist_wings.bbmodel?v=${versionSite}`)
      .then(r => r.json())
      .then(m => new Promise((ok, ko) => {
        const image = new Image();
        image.onload = () => ok(construireModele(m, image));
        image.onerror = ko;
        image.src = m.textures[0].source;
      }))
      .then(modele => {
        // le modèle est fait autour du cou (y = 24) ; le corps de skinview3d a son origine en son milieu (y = 18)
        modele.position.set(0, 6, 0);
        placerAiles(modele);
        teindreAiles(modele);
        modele.visible = ailesPortees;
        corps.add(modele);
        return modele;
      });
    const battre = maintenant => {
      const t = maintenant / 1000, vol = animActuelle === "fly";
      // ampleur en radians, ajoutée à la pose de repos : les ailes se replient vers l'arrière puis
      // reviennent (vues de dos, elles se referment et s'ouvrent), sans jamais passer devant le corps
      const vitesse = vol ? 6 : 2.2, ampleur = vol ? 0.8 : 0.35;
      ailes?.then(m => {
        placerAiles(m);
        for (const nom of ["wing_left", "wing_right"]) {
          const aile = m.userData.nommes[nom], bout = m.userData.nommes[nom + "_tip"];
          const battu = 0.5 + 0.5 * Math.sin(t * vitesse);
          if (aile) aile.rotation.y = aile.userData.reposY + Math.sign(aile.userData.reposY) * ampleur * battu;
          // le lobe du bas suit avec un léger retard
          if (bout) bout.rotation.z = bout.userData.repos + Math.sign(bout.userData.repos) * ampleur * 0.4 * (0.5 + 0.5 * Math.sin(t * vitesse - 0.7));
        }
      });
      battement = ailesPortees && !reduce ? requestAnimationFrame(battre) : 0;
    };
    const porterAiles = on => {
      ailesPortees = on;
      reglagesAiles.forEach(r => { r.hidden = !on; });
      if (on) ailes?.then(placerAiles);
      if (on) chargerAiles().then(m => { m.visible = ailesPortees; }).catch(() => {});
      else ailes?.then(m => { m.visible = false; });
      if (on && !battement && !reduce) battement = requestAnimationFrame(battre);
      equipButtons.filter(b => b.dataset.back === "wings").forEach(b => {
        b.setAttribute("aria-pressed", String(on));
        if (!b.querySelector(".thumb")) b.textContent = on ? b.dataset.unequip : b.dataset.equip;
        b.closest(".item")?.classList.toggle("equipped", on);
      });
    };
    const featName = document.getElementById("feat-name"), featText = document.getElementById("feat-text");
    equipButtons.forEach(b => b.addEventListener("click", () => {
      const src = b.dataset.src || "misty";
      const hat = b.dataset.back === "none";   // un chapeau : porté en plus de la cape
      const pet = b.dataset.back === "pet";    // un compagnon : sur l'épaule, en plus du reste
      const ailesBtn = b.dataset.back === "wings";   // des ailes : en plus de la cape
      const on = hat ? chapeauPorte === b.dataset.thumb : pet ? minimePorte : ailesBtn ? ailesPortees : back.mode === b.dataset.back && back.source === src;
      if (hat) porterChapeau(on ? null : b.dataset.thumb);
      else if (pet) porterMinime(!on);
      else if (ailesBtn) porterAiles(!on);
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
      piggyback: { bras: [16, 16], jambes: [14, -8], haut: 3, passager: true },
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
      // Sur le dos : l'ami (cheveux roux) dépasse au-dessus de la tête du porteur
      if (pose.passager) {
        g.fillStyle = "#e3a56d"; g.fillRect(17, y0 - 7, 7, 7);
        g.fillStyle = "#d77a2e"; g.fillRect(17, y0 - 8, 7, 3); g.fillRect(23, y0 - 6, 1, 4);
        g.fillStyle = "#3a5f2e"; g.fillRect(19, y0 - 3, 1, 1); g.fillRect(22, y0 - 3, 1, 1);
      }
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
      if (pose.passager) { g.fillStyle = "#5fae4f"; g.fillRect(8, y0 + 8, 14, 2); g.fillStyle = "#e3a56d"; g.fillRect(8, y0 + 10, 2, 2); g.fillRect(20, y0 + 10, 2, 2); }
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
      } else if (c.dataset.thumb === "wings") {
        // Ailes de brume : des ailes de dragon aux couleurs de Mistly, comme le modèle par défaut
        // (o : os, k : bout des griffes, d : membrane sombre, r : membrane bleue, w : bord blanc).
        // Chaque ligne est une moitié, reflétée de l'autre côté.
        const moities = [".....k.", ".k..ko.", "..o.oo.", "oooooo.", "drrodo.", ".rroro.", "..wkrk.", "...w.w."];
        const rows = moities.map(m => m + "." + [...m].reverse().join(""));
        const tons = { o: "#3a4258", k: "#9bb8f0", r: "#4a7fe6", d: "#14275e", w: "#f2f7ff" };
        const px = Math.floor(c.width / 17);
        const ox = (c.width - 15 * px) / 2, oy = (c.height - rows.length * px) / 2;
        rows.forEach((row, y) => [...row].forEach((ch, x) => {
          if (!tons[ch]) return;
          g.fillStyle = tons[ch];
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
