# Contributing to Mistly Web

Thanks for wanting to help! This repository is the website [mistly.cc](https://mistly.cc): pages, documentation, translations, patch notes, the shop, the status page and the team dashboard. The launcher itself lives in [Mistly-Launcher](https://github.com/Mistly-MC/Mistly-Launcher) and the in-game mod in [Mistly-Client](https://github.com/Mistly-MC/Mistly-Client).

By taking part, you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Ways to help

- **Report a bug** on the site: open an issue with the page address, what you expected, what happened, and your browser and device (a screenshot helps a lot).
- **Fix a translation**: the site's texts are in `locales/<lang>.toml` (`en.toml` is the reference). The documentation is in `docs/<lang>/`.
- **Improve the documentation**: `docs/en/NN-slug.md` is the reference, the other languages follow it.
- **Fix or add a feature**: for anything bigger than a small fix, open an issue (or ask on [Discord](https://discord.gg/rdQJUgvUh3)) before you start, so that we can agree on the idea first.

Bugs in the launcher or in the game go to the [launcher's issues](https://github.com/Mistly-MC/Mistly-Launcher/issues), not here.

## Run the site locally

You need [Rust](https://www.rust-lang.org/tools/install) (stable, edition 2024).

```bash
git clone https://github.com/Mistly-MC/Mistly-Web.git
cd Mistly-Web
cargo run            # http://localhost:3000
cargo test           # must pass before every pull request
```

Useful variables (all optional):

- `PORT`: the port to listen on (`3000` by default);
- `MISTY_CACHE_DIR`: where the site keeps its data (a temporary folder by default);
- `ADMIN_PASSWORD`: 12 characters or more, opens the team dashboard on `/admin`;
- `GITHUB_TOKEN`: raises the GitHub API quota used for releases and patch notes.

The files in `public/` (CSS, JS, images) are read from disk: refresh the page to see a change. Changes to Rust code, `locales/`, `docs/` or `legal/` need a restart (`cargo run` again).

Never commit a `.env` file, a token or a password.

## Code style

- Rust, rendered server-side with [axum](https://github.com/tokio-rs/axum) and [maud](https://maud.lambda.xyz/). No JavaScript framework: small scripts in `public/js/`.
- Identifiers and comments are in **French**, like the rest of the code. Please keep that style.
- Texts shown to visitors are never hard-coded: add the key to **every** file in `locales/` (in English if you don't speak the language, a translator will follow).
- Pages must work on a phone (no horizontal scrolling at 360 px wide) and in both the dark and light themes.
- Run `cargo fmt` on the files you changed, and make sure `cargo clippy --all-targets` adds no new warning.

## Pull requests

1. Fork the repository and create a branch from `main`.
2. Keep each pull request focused on one change.
3. Write commit messages in English, in the imperative, without a final period: `Fix the dashboard layout on phones`. Add a body that explains *why* when it is not obvious.
4. Describe what you changed and how you tested it, with a screenshot for visual changes.
5. A team member reviews it. Every push to `main` is deployed to mistly.cc automatically, so only the team merges.

## License

Mistly Web is distributed under the [Misty License 1.0](LICENSE.txt): free to use, modify and share, but not to sell. By submitting a contribution, you agree that it is distributed under this license (see section 4 of the license).
