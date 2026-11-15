+++
title = "Game console"
description = "Follow the game's logs live, filter and copy them, stop a frozen game or send the logs to get help."
section = "tools"
+++

The console shows what the game writes while it runs, for Minecraft and for [Horizon](/docs/horizon). It opens as a large tile under the top row of the window, which stays usable.

## Open it

- Click the **Logs** tile at the top right; click it again (or a tab) to close the console.
- By default it opens when a game starts, and on its own when a game crashes. **Settings → Behavior → Open the console when the game starts** changes that.

## Read the logs

- Lines are colored by level: information, warnings, errors and Mistly's own messages. Errors keep their full stack trace.
- **All / Warnings / Errors** filter the lines and show a counter each. The search field finds a word.
- The view follows the newest lines until you scroll up; **Follow** brings you back down.

The console keeps the last 5,000 lines of each game. The full log is still written to the game's log file. See [Files and folders](/docs/files-and-folders#game-logs).

## Copy

- Click a line to select it; drag or `Shift`+click to select several; `Ctrl+A` selects everything.
- `Ctrl+C` or the copy button copies the selection.
- **Logs folder** opens the folder of the log files.

## Several games

When several games run at once, the console splits the screen: one panel per game, side by side, then 2 × 2 for four games, up to 3 × 2. Each panel has its own state, counters and buttons. Double-click a panel (or use its button) to maximize it.

## Stop a game

**Stop the game** ends a game that froze. A game you stop there, or close yourself (Quit, the window's X, `Alt+F4`), is not counted as a crash.

## Send logs

**Send logs** sends the last lines of the game to the Mistly team and gives you a **report code**, copied for you. Share that code on the [Discord server](https://discord.gg/rdQJUgvUh3) to get help: it lets the team find your logs.
