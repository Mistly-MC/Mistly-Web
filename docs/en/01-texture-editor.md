+++
title = "Texture pack editor"
description = "Create a resource pack by editing Minecraft's textures pixel by pixel, then export it as a .zip."
section = "tools"
+++

> [!WARNING]
> The texture pack editor is **experimental**: you may run into bugs.

Open the **Texture packs** tab to see your packs as cards showing the textures you changed. **Edit**, **Export**, **Open folder** and **Delete** sit at the bottom of each card.

## Create a pack

1. Click **Create**.
2. Give it a **name** and a **description**.
3. Pick the **Minecraft version** it is for, from 1.0 to the latest.
4. Click **Create pack**.

The original textures of that version are downloaded **once**, then reused for every pack of the same version.

## Start from an existing pack

- **Import** opens a resource pack you already have (`.zip`) so you can edit it.
- **Browse** searches Modrinth and CurseForge for resource packs and imports the one you pick into your packs, ready to edit.

## Edit textures

The editor shows every texture of the game, sorted into categories (building blocks, redstone, entities, interface…) with a counter. Use the search box to find one by name.

- **All textures**: every texture of the version.
- **My textures**: only the ones you edited.

Click a texture to open it, then draw.

### Tools and shortcuts

| Tool | Shortcut |
|---|---|
| Pencil | `B` |
| Eraser | `E` |
| Paint bucket | `G` |
| Color picker | `I`, or right-click for a quick pick |
| Undo / Redo | `Ctrl+Z` / `Ctrl+Y` |
| Save | `Ctrl+S` |
| Zoom | `Ctrl` + mouse wheel |

The side panel shows the **recent colors** and the **colors of the texture**. The **Grid** button shows the pixel grid. **Reset** restores the original texture and removes your version from the pack.

The texture you are working on is saved when you leave the page.

### External editor

Prefer Paint.NET or another program? Click **External editor** and choose your program (for example `paintdotnet.exe`; cancel to use the default app). Edit and save in your program: Mistly reloads the texture automatically when the file changes.

## Pack icon

In the editor, **Change the pack icon** sets the image shown in Minecraft's resource pack list.

## Export and use the pack

1. Click **Export** and choose where to save the `.zip`.
2. Put the zip in the `resourcepacks` folder of a [profile](/docs/profiles).
3. In Minecraft, enable it in **Options → Resource Packs**.

Mistly writes the right `pack_format` for the version, including very recent ones. For Minecraft 1.0 to 1.5.2, which came before resource packs, the zip uses the old texture pack layout (textures at the root, description in `pack.txt`).
