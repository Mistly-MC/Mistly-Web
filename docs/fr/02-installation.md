+++
title = "Installation"
description = "Installeur, version portable ou archive Linux : comment installer Mistly, où il s'installe et comment le désinstaller."
section = "start"
+++

Mistly existe sous trois formes : un installeur et une version portable pour Windows, et une archive pour Linux. Toutes sont disponibles sur la [page de téléchargement](/#download) et dans les [releases GitHub](https://github.com/Mistly-MC/Mistly-Launcher/releases), et toutes se mettent à jour toutes seules.

## Installeur (recommandé)

1. Télécharge `Misty-Setup-<version>.exe`.
2. Lance-le et suis les étapes. Tu peux choisir d'ajouter un raccourci sur le bureau.
3. Démarre Mistly depuis le menu Démarrer.

Ce que fait l'installeur :

- il installe Mistly dans `%LOCALAPPDATA%\Programs\Misty`, **sans droits administrateur** ;
- il ajoute un raccourci dans le menu Démarrer (et sur le bureau si tu l'as demandé) ;
- il inscrit Mistly dans les **Applications installées** de Windows, pour une désinstallation propre.

> [!NOTE]
> `Misty.exe` et l'installeur sont signés, mais avec un certificat auto-signé pour l'instant : Windows SmartScreen peut encore afficher un avertissement la première fois.

## Version portable

1. Télécharge `Misty-<version>.zip`.
2. Extrais-le où tu veux, par exemple sur une clé USB.
3. Lance `Misty-<version>\Misty.exe`.

Rien n'est installé dans Windows. Les données du jeu vont quand même dans `%LOCALAPPDATA%\Misty` sur le PC où tu joues. Voir [Fichiers et dossiers](/docs/files-and-folders).

> [!TIP]
> Les deux versions utilisent le même dossier de données : tu peux passer de l'une à l'autre sans perdre tes profils, tes mondes ni tes réglages.

## Linux

Mistly fonctionne sous Linux x86_64 avec glibc 2.35 ou plus récent : Ubuntu 22.04, Debian 12, Fedora 36 et versions suivantes, et les distributions basées dessus.

1. Télécharge `Misty-<version>-linux-x86_64.tar.gz`.
2. Extrais-la là où tu veux la garder, par exemple avec `tar -xzf Misty-<version>-linux-x86_64.tar.gz`.
3. Lance `Misty-<version>/Misty`.

Au premier lancement, Mistly s'ajoute à ton menu des applications. Les données de jeu vont dans `~/.local/share/Misty`.

Ce qu'il lui faut :

- **GTK 3**, pour les fenêtres de choix de fichier (déjà installé sur la plupart des bureaux) ;
- **un trousseau** (GNOME Keyring ou KWallet) pour garder ta connexion Microsoft d'une session à l'autre.

> [!NOTE]
> Sous Linux, la connexion Microsoft s'ouvre dans ton navigateur. Une fois connecté, la page devient blanche : copie son adresse complète (elle commence par `https://login.live.com/oauth20_desktop.srf?code=`) et colle-la dans Mistly.

## Rien d'autre à installer

Le programme (`Misty.exe` sous Windows, `Misty` sous Linux) contient tout ce dont il a besoin : polices, traductions et images. Java est téléchargé automatiquement quand une version de Minecraft en a besoin.

## Désinstaller

- **Installeur :** désinstalle Mistly depuis **Paramètres → Applications → Applications installées** dans Windows. Tes données de jeu (versions, profils, mondes) sont gardées, sauf si tu demandes leur suppression.
- **Portable :** supprime le dossier extrait.
- **Linux :** supprime le dossier extrait et l'entrée du menu `~/.local/share/applications/mistly.desktop`.

Pour effacer aussi **toutes** les données de jeu, utilise **Tout supprimer** dans **Paramètres → Ressources** avant de désinstaller, ou supprime toi-même le dossier de données (`%LOCALAPPDATA%\Misty` sous Windows, `~/.local/share/Misty` sous Linux).

> [!WARNING]
> Supprimer le dossier de données supprime aussi les mondes de tous les profils. Sauvegarde d'abord les dossiers `saves` que tu veux garder.
