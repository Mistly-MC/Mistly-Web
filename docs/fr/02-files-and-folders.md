+++
title = "Fichiers et dossiers"
description = "Où Mistly range le jeu, tes profils, tes comptes et ses réglages."
section = "help"
+++

## Le dossier de données

Tout ce que Mistly télécharge ou crée va dans **`%LOCALAPPDATA%\Misty`** sous Windows et **`~/.local/share/Misty`** sous Linux. Sous Windows, colle ce chemin dans la barre d'adresse de l'Explorateur pour l'ouvrir ; sur les deux systèmes, tu peux utiliser **Paramètres → Ressources → Dossier de Mistly**.

| Fichier / dossier | Contenu |
|---|---|
| `reglages.json` | Réglages : dernier compte et dernière version, RAM, langue, thème, Rich Presence, dernier profil… (avant 0.4.0 : `data.txt`, converti automatiquement) |
| `listpseudo.txt` | Comptes hors-ligne |
| `msa.json` | Comptes Microsoft connectés (les noms seulement : les jetons de connexion sont gardés dans le Gestionnaire d'identification de Windows, ou dans le trousseau sous Linux) |
| `instances\<id>\` | Un dossier par [profil](/docs/profiles) |
| `versions\`, `libraries\`, `assets\`, Java | L'installation de Minecraft, partagée par tous les profils |
| `logs\misty\` | Le journal de Mistly (`launcher.log`) et un fichier de log pour chacune des 10 dernières parties |

## Le dossier d'un profil

Chaque profil de `instances\<id>\` contient :

| Fichier / dossier | Contenu |
|---|---|
| `instance.json` | Les réglages du profil : nom, version, mod loader… |
| `mods\` | Mods |
| `resourcepacks\` | Packs de ressources |
| `shaderpacks\` | Shaders |
| `saves\` | Mondes |
| `misty-contenu.json` | Ce que Mistly sait du contenu installé |

**Ouvrir le dossier du profil**, sur la page du profil, l'ouvre directement.

## Les journaux du jeu

Minecraft écrit ses journaux dans le dossier de jeu utilisé (celui du profil, ou le dossier de données en *Jeu rapide*) :

- `logs\latest.log` : le journal de la dernière partie ;
- `crash-reports\` : un rapport par plantage.

Mistly garde aussi ses propres copies dans `logs\misty\` du dossier de données. **Paramètres → Dépannage** ouvre les deux dossiers, et la [console](/docs/console) affiche les logs en direct.

Joins ces fichiers quand tu [signales un problème](/docs/troubleshooting#signaler-un-bug).

## Le dossier de l'application

- Installeur : `%LOCALAPPDATA%\Programs\Misty\`, avec un dossier `Misty-<version>` par version installée et, à côté, un `Misty.exe` qui lance la plus récente. Les raccourcis pointent vers ce `Misty.exe` : ils survivent donc aux mises à jour.
- Portable : là où tu as extrait le zip.

> [!TIP]
> Pour donner ta propre icône à Mistly, place un fichier `Icon.ico` à côté de `Misty.exe`.
