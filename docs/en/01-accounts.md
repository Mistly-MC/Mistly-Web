+++
title = "Accounts"
description = "Sign in with one or several Microsoft accounts, or play with offline accounts."
section = "play"
+++

Mistly supports two kinds of accounts. At the [first launch](/docs/first-launch) you choose between **Premium** (Microsoft) and **Offline**; afterwards, both kinds sit in a single list, opened from your account at the bottom right of the window.

## Microsoft accounts

A Microsoft account that owns **Minecraft: Java Edition** is needed to join online servers, to change your [skin and capes](/docs/skins) and to be seen by the other [Mistly Client](/docs/mistly-client) players.

1. Open the account list and choose to add a Microsoft account (or use **Settings → Accounts**).
2. A Microsoft sign-in window opens. Sign in as you would on the web.
3. Once signed in, the account shows in the list with your skin's head.

You can add **several Microsoft accounts** and switch between them. **Forget this account** removes one account without touching the others.

> [!NOTE]
> **Full sign-out** also forgets the account cached on this PC. Use it on a shared computer.

If the launcher stays open for a long time, Mistly renews the Microsoft session before a game starts, so joining a server does not fail with *Invalid session*.

### When sign-in fails

| Message | What to do |
|---|---|
| *This Microsoft account has no Xbox profile* | Sign in once on minecraft.net to create the profile. |
| *Xbox Live is not available in this account's country* | The account's country does not allow Xbox Live. |
| *This account must first be age-verified on xbox.com* | Complete the age verification on xbox.com. |
| *This is a child account* | An adult must add the account to a Microsoft family. |
| *This account does not own Minecraft Java Edition* | Buy the game, or sign in with the account that owns it. |

## Offline accounts

Offline accounts are usernames you can play with without a Microsoft account. Add them in **Settings → Accounts**: type a username and click **Add**. They show right away in the account list.

> [!WARNING]
> Servers that check accounts (most public servers) refuse offline accounts. Offline accounts are meant for single player, LAN worlds and servers that accept them.

## Several games, several accounts

Each game keeps the account that was selected when you clicked **PLAY**, even if you switch accounts while it downloads. Pick an account, play, pick another one, play again: both games run side by side.

## Where accounts are stored

- Microsoft accounts: `msa.json`
- Offline accounts: `listpseudo.txt`

Both are in `%LOCALAPPDATA%\Misty`. See [Files and folders](/docs/files-and-folders).
