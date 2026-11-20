+++
title = "Updates"
description = "How Mistly updates itself, how to check for a new version by hand, and how to get beta versions."
section = "help"
+++

Mistly updates itself from its [GitHub releases](https://github.com/Mistly-MC/Mistly-Launcher/releases), for both the installer and the portable version.

## Automatic updates

When Mistly starts, it checks whether a newer version was released. If so, it shows **Update available** with the new and current version numbers:

- **Update** opens a window with a progress bar (percentage, megabytes downloaded and the source), then unpacks the new version and restarts Mistly.
- **Later** keeps the current version; you will be asked again next time.

The downloaded file is checked (complete, and the right version) before it is installed. After the restart, Mistly shows **Update complete** and removes the old version.

Your data (profiles, worlds, accounts, settings) is not touched by an update.

### When GitHub is down

If GitHub cannot be reached or refuses the request, the update comes from the Mistly website, which keeps a copy of the latest version.

### Shortcuts and pins

Shortcuts (Start menu, desktop, taskbar pins) point to a `Misty.exe` whose path never changes, next to the version folders, and which starts the newest version. They keep working after every update, and older shortcuts are redirected automatically.

## Check by hand

**Settings → Troubleshooting → Updates** shows the installed version and a **Check for updates** button. It tells you whether you are already up to date.

## Beta versions

**Settings → Troubleshooting → Pre-release updates** also offers the beta and pre-release versions published on GitHub. They bring new features first, but can have bugs. Turn the setting off to go back to stable versions only.

## Mistly Client

[Mistly Client](/docs/mistly-client) has its own releases: the launcher downloads the newest one before a game, without a launcher update.

## What changed?

- In the launcher: the **Release notes** tab lists every version on the left, and the notes of the chosen one on the right, with filters for new features, fixes, the launcher and Mistly Client.
- On this site: the [patch notes](/patch-notes) page shows every release, with its files.

> [!NOTE]
> Mistly never offers to "update" to an older version than the one installed.
