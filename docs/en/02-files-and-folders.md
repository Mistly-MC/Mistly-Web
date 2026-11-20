+++
title = "Files and folders"
description = "Where Mistly stores the game, your profiles, your accounts and its settings."
section = "help"
+++

## The data folder

Everything Mistly downloads or creates goes to **`%LOCALAPPDATA%\Misty`** on Windows and **`~/.local/share/Misty`** on Linux. On Windows, paste this path into the Explorer address bar to open it; on both systems you can use **Settings → Resources → Mistly folder**.

| File / folder | Content |
|---|---|
| `reglages.json` | Settings: last account and version, RAM, language, theme, Rich Presence, last profile… (before 0.4.0: `data.txt`, converted automatically) |
| `listpseudo.txt` | Offline accounts |
| `msa.json` | Signed-in Microsoft accounts (names only: the sign-in tokens are kept in the Windows Credential Manager, or in the keyring on Linux) |
| `instances\<id>\` | One folder per [profile](/docs/profiles) |
| `versions\`, `libraries\`, `assets\`, Java | The Minecraft installation, shared by every profile |
| `logs\misty\` | Mistly's own log (`launcher.log`) and a log file for each of the last 10 games |

## A profile's folder

Each profile in `instances\<id>\` contains:

| File / folder | Content |
|---|---|
| `instance.json` | The profile's settings: name, version, mod loader… |
| `mods\` | Mods |
| `resourcepacks\` | Resource packs |
| `shaderpacks\` | Shaders |
| `saves\` | Worlds |
| `misty-contenu.json` | What Mistly knows about the installed content |

**Open the profile folder**, on the profile's page, opens it directly.

## Game logs

Minecraft writes its logs in the game folder it runs in (the profile's folder, or the data folder with *Quick play*):

- `logs\latest.log`: the log of the last game;
- `crash-reports\`: one report per crash.

Mistly also keeps its own copies in `logs\misty\` of the data folder. **Settings → Troubleshooting** opens both folders, and the [console](/docs/console) shows the logs live.

Attach these files when you [report a problem](/docs/troubleshooting#report-a-bug).

## The application folder

- Installer: `%LOCALAPPDATA%\Programs\Misty\`, with one `Misty-<version>` folder per installed version and a `Misty.exe` next to them that starts the newest one. Shortcuts point to that `Misty.exe`, so they survive updates.
- Portable: wherever you extracted the zip.

> [!TIP]
> To give Mistly your own icon, put an `Icon.ico` file next to `Misty.exe`.
