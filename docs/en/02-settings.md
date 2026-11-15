+++
title = "Settings"
description = "Every tab of the settings, from the theme to memory, shared servers and troubleshooting tools."
section = "tools"
+++

Open **Settings** at the left of the bottom bar. The settings show in the middle of the window, with their categories as tabs at the top and one tile per setting. Changes are saved right away.

The **search field** finds any setting by name, whatever its tab.

## Appearance

- **Color theme**: **Mist** (misty sky by day), **Night** (starry night) or **Sync with Windows**.
- **Seasonal themes**: Halloween (October 28 to November 3) and Christmas (December 22 to 28) decorations switch on by themselves.
- **Background blur**: how much the landscape behind the launcher is blurred, from sharp to blurry.
- **Tabs**: choose which tabs show at the top of the window. Hidden tabs leave their place to the others.

A live preview of the launcher at the top of the tab changes with these settings.

## Language

English, French, Spanish, Italian, Portuguese or Russian. The interface changes immediately. Some parts may stay in English when a translation is missing.

## Accounts

- **Microsoft accounts**: sign in, add another account, forget one or sign out.
- **Offline accounts**: type a username and click **Add**; delete one with its delete button.

See [Accounts](/docs/accounts).

## Memory

The RAM given to Minecraft, with a slider. Mistly shows how much memory your PC has. The default is **4 GB**.

> [!TIP]
> 4 to 6 GB is plenty for vanilla; big modpacks run better with 8 GB or more. Do not give the game more than about half of your PC's memory: Windows and the other apps need some too.

## Behavior

- **Same servers and hotbars in every version**: your server list and saved hotbars follow you into every version and profile, so you add a server only once. The first time, the lists of your profiles are merged (no server is lost) and the servers of the official Minecraft launcher are imported.
- **Open the console when the game starts**: shows the game's logs live. See [Game console](/docs/console).
- **Discord Rich Presence**: shows Mistly as your Discord activity, with the page you are on or the version you are playing, and your player head.
- **Send anonymous usage data**: asked once, and switchable here. See [what is sent](#anonymous-usage-data).
- **Desktop shortcut**: creates a Mistly shortcut in the folder you choose. **Recreate** makes it again if you deleted it.

## Resources

- **Storage**: how much space Mistly takes (game versions, libraries, Java, mods and worlds). **Delete everything** erases it entirely; Mistly then offers to close, so it does not recreate data right away.
- **Mistly folder**: opens the folder where Mistly keeps your profiles, game files and settings. See [Files and folders](/docs/files-and-folders).
- **Simultaneous downloads**: the number of files downloaded at once when a version is installed. Lower it if downloads fail on your connection.

> [!WARNING]
> **Delete everything** cannot be undone. Back up the worlds you want to keep first.

## Troubleshooting

- **Launcher cache**: **Clear cache** deletes cached icons and search results. They are downloaded again when needed.
- **Debug info**: **Export debug info** creates a text file with your Mistly and Windows versions and your settings, to attach to a bug report.
- **Logs**: opens the folder of the game logs and crash reports. **Mistly log** opens the launcher's own log.
- **Updates**: the installed version and the **Check for updates** button. See [Updates](/docs/updates).
- **Pre-release updates**: also offers the beta versions published on GitHub. They bring new features first, but can have bugs.
- **Mistly Client ticked by default**: whether new compatible profiles get [Mistly Client](/docs/mistly-client).

## Anonymous usage data

When you allow it, Mistly sends: its version, your Windows version, the amount of RAM, the games started, and crashes with their last log lines (username, folder names and tokens are removed). It never sends your account, password or email.

Separately, while it is open, Mistly sends an anonymous signal every minute so the players online can be counted: a random id drawn at each start, and whether a game is running. The total is public at [misty-launcher.online/api/online](https://misty-launcher.online/api/online). The full details are in the [privacy policy](/privacy).
