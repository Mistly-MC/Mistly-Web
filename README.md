<div align="center">

<img src="public/assets/icone.png" alt="Mistly" width="110" />

# Mistly Web

**The website of Mistly, the Minecraft launcher light as the mist.**

Rendered server-side in Rust: pages, translations, patch notes and the pixel-art night scene.

[![Website](https://img.shields.io/badge/website-mistly.cc-8ab4f8?style=flat-square)](https://mistly.cc/)
[![Deployment](https://img.shields.io/github/deployments/Misty-MC/Misty-Web/production?label=deploy&style=flat-square)](https://github.com/Misty-MC/Misty-Web/deployments)
[![Rust](https://img.shields.io/badge/written%20in-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![axum](https://img.shields.io/badge/server-axum-6f9cff?style=flat-square)](https://github.com/tokio-rs/axum)
[![Docker](https://img.shields.io/badge/deploy-Docker-2496ED?style=flat-square&logo=docker&logoColor=white)](Dockerfile)
[![License](https://img.shields.io/badge/license-MIT-success?style=flat-square)](LICENSE)

[**Website**](https://mistly.cc/) · [**Docs**](https://mistly.cc/docs) · [**Patch notes**](https://mistly.cc/patch-notes) · [**Launcher**](https://github.com/Misty-MC/Mistly-Launcher) · [**Releases**](https://github.com/Misty-MC/Mistly-Launcher/releases)

<img src="public/assets/screens/en/home.png" alt="Mistly launcher, shown on the website" width="800" />

</div>

## 🖼️ Launcher screenshots

The screenshots shown on the site live in `public/assets/screens/<lang>/` (`en` and `fr`, 1180 × 720, night theme) and `public/assets/screens/<lang>/light/` (mist theme, shown when the site is in light mode): `home`, `skin`, `profiles`, `notes`, `textures`, `friends`, `versions`, `horizon` and `settings`. They make up the clickable launcher in the Preview section; `home` is also the hero image and the link preview image.
They are taken by the launcher itself (`MISTY_CAPTURE`, debug build), on a demo data folder (`MISTY_DATA_DIR`).

## 🗂️ Project structure

```
src/
├── main.rs            startup, routes, static files
├── routes.rs          pages, /docs, /shop, /api/latest, /download/*, /skin, /cape, /player
├── skins.rs           Minecraft profiles for the shop (Mojang API, mc-heads.net fallback)
├── seo.rs             JSON-LD, robots.txt, sitemap.xml
├── docs.rs            documentation pages (docs/<lang>/NN-slug.md)
├── legal.rs           legal notice, privacy policy, terms of use (legal/<fr|en>/*.md)
├── github.rs          GitHub releases + cache
├── i18n.rs            language negotiation and translations
├── markdown.rs        Markdown → safe HTML (release notes and docs)
├── format.rs          localized dates and sizes
├── scene.rs           pixel-art night scene and fog (SVG)
└── views/             HTML pages (maud)
    ├── layout.rs      <head>, navigation, footer
    ├── home.rs        home page
    ├── patch_notes.rs patch notes
    ├── docs.rs        documentation
    ├── not_found.rs   404
    └── icons.rs       SVG icons
locales/               one TOML file per language
docs/                  documentation, one folder per language (en is the reference)
legal/                 legal pages, in French and English (other languages show English)
public/
├── css/               base, layout, home, patch-notes, docs
├── js/site.js         smooth scrolling, navigation, animations
└── assets/            icon, Silkscreen font, screenshots
```

## 📄 License

The code is released under the [MIT License](LICENSE).
The Silkscreen, Geist and Geist Mono fonts are distributed under the SIL Open Font License (see [public/assets/FONTS.md](public/assets/FONTS.md)).
[skinview3d](https://github.com/bs-community/skinview3d) (3D skin preview in the shop) is bundled under the [MIT License](public/vendor/skinview3d/LICENSE).

---

<div align="center">
<sub>
Mistly is an independent launcher and is not affiliated with Mojang Studios or Microsoft.<br/>
Minecraft is a trademark of Mojang AB.
</sub>
</div>
