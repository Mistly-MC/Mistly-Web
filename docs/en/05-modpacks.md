+++
title = "Modpacks"
description = "Install, import, update and export Modrinth or CurseForge modpacks as ready-to-play profiles."
section = "play"
+++

A modpack is a set of mods and settings put together by its author. Mistly turns it into a new [profile](/docs/profiles), with the right Minecraft version and mod loader (Forge, NeoForge, Fabric or Quilt).

## Install a modpack

1. Open the **Profiles** tab, then click **Install a modpack**.
2. Pick the platform, **Modrinth** or **CurseForge**, and search.
3. Choose the **version** of the modpack you want.
4. Confirm: a new profile is created and its files are downloaded.

Mistly installs the mod loader version the modpack asks for, copies its configuration files and downloads every mod.

## Import a modpack file

Already have a modpack file? Click **Import a modpack** in the Profiles tab and pick a Modrinth `.mrpack` or a CurseForge `.zip`. Mistly creates a profile with the name written in the modpack, its mods, its configs and the right mod loader.

If the file is not a modpack, Mistly says so, and a half-created profile is removed when the import fails.

## Update a modpack

A modpack profile has a button that lists the versions of the modpack (the installed one is marked) and switches to the one you pick. The modpack's mods and configs are replaced; **your worlds, your options and the mods you added yourself are kept**.

An imported file is recognized when the modpack is published on Modrinth or CurseForge, so it can be updated too.

## Export a profile

**Export the modpack**, on a profile's page, saves it as a `.mrpack` file: mods, configs, resource packs, shaders and options, without the worlds. Mistly, the Modrinth App and Prism can import it. Mistly Client goes with it when its box is ticked.

> [!NOTE]
> Launch the profile once before exporting it: until then, the mod loader version is not known.

## Manual downloads

Some CurseForge authors only allow downloads from the CurseForge website. Mistly first looks for those files on **Modrinth** (the exact same file, then the project with the same name for your version and loader). Only the files it cannot find are listed after the installation:

1. Click **Open the pages**: each file's page opens in your browser.
2. Download the files.
3. Put each one in the folder indicated next to its name (usually the profile's `mods` folder).

If the whole modpack is restricted this way, Mistly tells you it cannot be downloaded outside CurseForge.

## Common errors

| Message | Meaning |
|---|---|
| *This modpack has no published version* | The author has not published a file yet. |
| *The modpack doesn't specify its Minecraft version* | The modpack is incomplete; try another version of it. |
| *Unsupported mod loader* | The modpack uses a loader Mistly does not install. |
