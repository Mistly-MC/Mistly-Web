+++
title = "Installation"
description = "Installer, portable version or Linux archive: how to install Mistly, where it goes and how to uninstall it."
section = "start"
+++

Mistly comes in three forms: an installer and a portable version for Windows, and an archive for Linux. All are available on the [download page](/#download) and in the [GitHub releases](https://github.com/Misty-MC/Mistly-Launcher/releases), and all update themselves.

## Installer (recommended)

1. Download `Misty-Setup-<version>.exe`.
2. Run it and follow the steps. You can choose to add a desktop shortcut.
3. Start Mistly from the Start menu.

What the installer does:

- installs Mistly in `%LOCALAPPDATA%\Programs\Misty`, **without administrator rights**;
- adds a Start menu shortcut (and a desktop one if you asked for it);
- registers Mistly in Windows' **Installed apps**, so you can uninstall it cleanly.

> [!NOTE]
> `Misty.exe` and the installer are signed, but with a self-signed certificate for now: Windows SmartScreen may still show a warning the first time.

## Portable version

1. Download `Misty-<version>.zip`.
2. Extract it wherever you like, for example on a USB drive.
3. Run `Misty-<version>\Misty.exe`.

Nothing is installed in Windows. The game data still goes to `%LOCALAPPDATA%\Misty` on the PC you play on. See [Files and folders](/docs/files-and-folders).

> [!TIP]
> Both versions use the same data folder: you can switch from one to the other without losing your profiles, worlds or settings.

## Linux

Mistly runs on Linux x86_64 with glibc 2.35 or newer: Ubuntu 22.04, Debian 12, Fedora 36 and later, and the distributions based on them.

1. Download `Misty-<version>-linux-x86_64.tar.gz`.
2. Extract it where you want to keep it, for example with `tar -xzf Misty-<version>-linux-x86_64.tar.gz`.
3. Run `Misty-<version>/Misty`.

On its first start, Mistly adds itself to your applications menu. Game data goes to `~/.local/share/Misty`.

What it needs:

- **GTK 3**, for the file dialogs (installed on most desktops);
- **a keyring** (GNOME Keyring or KWallet) to keep your Microsoft sign-in between sessions.

> [!NOTE]
> On Linux, Microsoft sign-in opens in your web browser. Once you are signed in, the page turns blank: copy its full address (it starts with `https://login.live.com/oauth20_desktop.srf?code=`) and paste it into Mistly.

## Nothing else to install

The program (`Misty.exe` on Windows, `Misty` on Linux) contains everything it needs: fonts, translations and images. Java is downloaded automatically when a Minecraft version needs it.

## Uninstall

- **Installer:** uninstall Mistly from **Settings → Apps → Installed apps** in Windows. Your game data (versions, profiles, worlds) is kept unless you ask for it to be deleted.
- **Portable:** delete the extracted folder.
- **Linux:** delete the extracted folder and the menu entry `~/.local/share/applications/mistly.desktop`.

To erase **all** game data as well, use **Delete everything** in **Settings → Resources** before uninstalling, or delete the data folder yourself (`%LOCALAPPDATA%\Misty` on Windows, `~/.local/share/Misty` on Linux).

> [!WARNING]
> Deleting the data folder also deletes the worlds of every profile. Back up the `saves` folders you want to keep first.
