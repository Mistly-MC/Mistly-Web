+++
title = "Troubleshooting"
description = "The most common problems, what they mean and how to fix them. And how to report a bug."
section = "help"
+++

## The version list doesn't load

*Could not load versions* means Mojang's servers could not be reached. Check your Internet connection, then restart Mistly.

## A mod loader is missing

Mistly only offers the loaders that exist for the chosen version. NeoForge starts at 1.20.1, Fabric and Quilt at 1.14, and LiteLoader stops at 1.12.2. See [Versions and mod loaders](/docs/versions-and-loaders).

## A mod won't install

| Message | What to do |
|---|---|
| *… has no version compatible with …* | The mod does not exist for this version and loader. Try another version of Minecraft or another loader. |
| *The author of … doesn't allow downloads from apps other than CurseForge* | Download the file from its CurseForge page and put it in the folder indicated. |
| *Pick a mod loader (Fabric, Forge…) to install mods* | The profile is Vanilla or OptiFine. Create a profile with a mod loader. |
| *Shaders not supported* | Use an OptiFine profile, or install Iris (Fabric, NeoForge) or Oculus (Forge). |

## The game doesn't start or crashes

1. Check the error shown in Mistly (*Launch failed*, *Download failed*…).
2. Open the [console](/docs/console) (the **Logs** tile): it opens on its own after a crash. Filter on **Errors** to find the line that matters.
3. For the full files, read `logs\latest.log`, or the newest file in `crash-reports\`. See [Files and folders](/docs/files-and-folders#game-logs).
4. If you use many mods, raise the memory in [Settings](/docs/settings).
5. Disable the most recently added mods one by one from the profile's **Content** tab.

## "Invalid session" when joining a server

The Microsoft session had expired. Mistly now renews it before a game starts; if it still happens, close the game and click **PLAY** again.

## Downloads fail

Lower **Simultaneous downloads** in **Settings → Resources**, then try again.

## Microsoft sign-in fails

The message says what is wrong with the account: no Xbox profile, age verification, child account, game not owned… See [Accounts](/docs/accounts#when-sign-in-fails).

## "The game may still be running"

Some files cannot be deleted while Minecraft is open. Close the game, then try again.

## "Request limit reached"

Mojang temporarily limits the number of requests. Mistly shows your last known skin and capes and retries by itself. See [Skin and cosmetics](/docs/skins).

## Get help

The quickest way: in the [console](/docs/console), click **Send logs** and share the report code on the [Discord server](https://discord.gg/rdQJUgvUh3).

## Report a bug

Open an issue on [GitHub](https://github.com/Misty-MC/Mistly-Launcher/issues) with:

- the Mistly version (Settings → Troubleshooting), or the file made by **Export debug info**;
- the Minecraft version and the mod loader;
- what you did, what you expected and what happened;
- `logs\latest.log` and, after a crash, the report from `crash-reports\`.
