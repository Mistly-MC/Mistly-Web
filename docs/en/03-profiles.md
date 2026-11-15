+++
title = "Profiles"
description = "One profile per way of playing: its own version, mod loader, mods and worlds."
section = "play"
+++

A profile is a separate Minecraft setup, like an instance in other launchers. Each profile has:

- its own **version** and **mod loader**;
- its own **game folder**: `mods`, `resourcepacks`, `shaderpacks`, `saves` (worlds) and options.

Versions, libraries and assets stay **shared** between profiles: nothing is downloaded twice. By default your **server list and saved hotbars** are shared too, so a server added once shows in every profile (this can be turned off in [Settings](/docs/settings#behavior)).

## Create a profile

1. Open the **Profiles** tab.
2. Click **New profile**.
3. Give it a name, pick the version and the mod loader.
4. Click **Create profile**.

To start from a ready-made set of mods, use **Install a modpack** or **Import a modpack** instead. See [Modpacks](/docs/modpacks).

## The profile cards

The Profiles tab shows your profiles as cards with the picture of their Minecraft version, their version, their mod loader and a **Modpack** label when they come from one. Filters at the top keep only one mod loader.

At the bottom of each card: when it was last played, and **Play**, **Manage** and **Delete**.

## Play with a profile

Click the profile in the left column of the window (or **Play** on its card), then **PLAY**. *Quick play* uses the shared game folder, without a profile.

## Manage a profile

**Manage** opens the profile:

- see and manage its **Content**, or **Discover** new mods, packs and shaders. See [Mods, packs and shaders](/docs/mods-and-content);
- tick **Mistly Client** to add the in-game client to the profile (Fabric profiles on a supported version). See [Mistly Client](/docs/mistly-client);
- **Rename** it or **Change its icon**;
- **Export the modpack** to share it. See [Modpacks](/docs/modpacks#export-a-profile);
- **Open the profile folder** in the Windows Explorer.

> [!WARNING]
> Deleting a profile permanently erases its mods, packs, worlds and settings.

## Where profiles live

Each profile is a folder in `%LOCALAPPDATA%\Misty\instances\`. See [Files and folders](/docs/files-and-folders).
