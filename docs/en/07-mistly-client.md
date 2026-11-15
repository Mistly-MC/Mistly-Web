+++
title = "Mistly Client"
description = "The in-game side of Mistly: a HUD, zoom, fullbright, a wardrobe and cosmetics the other Mistly players can see."
section = "play"
+++

Mistly Client is a **Fabric mod** that brings Mistly into the game, like PvP clients do: a main menu and loading screen in Mistly's colors, a HUD you arrange yourself, and the cosmetics of the [Skin tab](/docs/skins).

It is not built into the launcher: Mistly downloads it from GitHub the first time a game needs it, and checks for a newer release before you play. A new Mistly Client version reaches you without a launcher update.

## Supported versions

Mistly Client runs on Minecraft **1.21.11, 26.1, 26.1.1, 26.1.2, 26.2 and 26.3**, with Fabric.

## Play with Mistly Client

Two ways:

- **From the version picker.** Turn on the **Mistly Client** filter, select a version and confirm with the **Choose …** button. Mistly starts Fabric with Mistly Client for you.
- **In a profile.** On a Fabric profile of a supported version, tick the **Mistly Client** box next to Content and Discover. Ticked, the mod is added to the profile; unticked, it is removed. Each profile keeps its own choice.

**Settings → Troubleshooting → Mistly Client ticked by default** chooses whether the box starts ticked on new profiles.

## In game

### Menus

The main menu shows the Mistly landscape, the usual entries and your character in 3D with what you wear. The Singleplayer, Multiplayer and Options menus use the same look, and Mojang's loading screen is replaced by Mistly's. The Mistly menu can be turned off, and back on from the classic menu.

### HUD

Press **Right Shift** in game to arrange the HUD: drag a module to move it, click it to turn it on or off, scroll to resize it. Modules with their own options have a gear in the Mistly settings.

| Module | What it shows |
|---|---|
| FPS, coordinates, clock | The basics |
| Ping | Your ping, measured by Mistly and refreshed every second. Green under 80 ms, yellow under 150, orange under 250, red above |
| CPS and keystrokes | Clicks per second and the keys you press |
| Armor | Your armor and its durability |
| Reach | The distance of the last player you hit, in blocks |
| Combo | The hits you land in a row without being hit back |
| Music | The song playing on your PC (Spotify, Deezer, a browser…), read from Windows. Nothing to sign in to |

### Other features

- **Zoom**: hold `C`, and use the mouse wheel to zoom more or less. The key can be changed in Options → Controls.
- **Fullbright**: everything is lit, caves and nights included.
- **Hitboxes**: shows the hitboxes around you, like `F3 + B`, without opening F3.
- **Wardrobe**: equip your cosmetics in game. What you equip there is kept by the launcher.

## The Mistly network

To show your cosmetics and the Mistly cloud next to your name (above your head, in the player list and in the chat) to the other Mistly players, the game proves your Microsoft account to the Mistly site the same way it does when it joins a server. Your Minecraft token never leaves the game. The site then knows that you are online and what you wear.

**Mistly network**, in the in-game Mistly settings, turns this off. The [Friends](/docs/friends) page relies on it.

## Good to know

- If a part of Mistly Client does not fit a Minecraft version, only that feature is turned off: the game still starts.
- At each start, Mistly keeps only the newest Mistly Client file for the Minecraft version in the profile's `mods` folder and deletes the older ones.
- Modpacks that ship an older Fabric Loader work: Mistly Client only asks for the loader version it needs.
