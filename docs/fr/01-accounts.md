+++
title = "Comptes"
description = "Connecte-toi avec un ou plusieurs comptes Microsoft, ou joue avec des comptes hors-ligne."
section = "play"
+++

Mistly gère deux sortes de comptes. Au [premier lancement](/docs/first-launch), tu choisis entre **Premium** (Microsoft) et **Hors ligne** ; ensuite, les deux sortes sont dans une seule liste, ouverte depuis ton compte en bas à droite de la fenêtre.

## Comptes Microsoft

Un compte Microsoft qui possède **Minecraft : Java Edition** est nécessaire pour rejoindre les serveurs en ligne, pour changer ton [skin et tes capes](/docs/skins) et pour être vu par les autres joueurs [Mistly Client](/docs/mistly-client).

1. Ouvre la liste des comptes et choisis d'ajouter un compte Microsoft (ou passe par **Paramètres → Comptes**).
2. Une fenêtre de connexion Microsoft s'ouvre. Connecte-toi comme sur le web.
3. Une fois connecté, le compte apparaît dans la liste avec la tête de ton skin.

Tu peux ajouter **plusieurs comptes Microsoft** et passer de l'un à l'autre. **Oublier ce compte** en retire un sans toucher aux autres.

> [!NOTE]
> La **déconnexion complète** oublie aussi le compte mémorisé sur ce PC. Utilise-la sur un ordinateur partagé.

Si le launcher reste ouvert longtemps, Mistly renouvelle la session Microsoft avant de lancer une partie, pour que la connexion à un serveur n'échoue pas avec *Session invalide*.

### Si la connexion échoue

| Message | Que faire |
|---|---|
| *Ce compte Microsoft n'a pas de profil Xbox* | Connecte-toi une fois sur minecraft.net pour créer le profil. |
| *Xbox Live n'est pas disponible dans le pays de ce compte* | Le pays du compte n'autorise pas Xbox Live. |
| *Ce compte doit d'abord être vérifié sur xbox.com* | Termine la vérification de l'âge sur xbox.com. |
| *C'est un compte enfant* | Un adulte doit ajouter le compte à une famille Microsoft. |
| *Ce compte ne possède pas Minecraft Java Edition* | Achète le jeu, ou connecte-toi avec le compte qui le possède. |

## Comptes hors-ligne

Les comptes hors-ligne sont des pseudos avec lesquels tu peux jouer sans compte Microsoft. Ajoute-les dans **Paramètres → Comptes** : tape un pseudo et clique sur **Ajouter**. Ils apparaissent tout de suite dans la liste des comptes.

> [!WARNING]
> Les serveurs qui vérifient les comptes (la plupart des serveurs publics) refusent les comptes hors-ligne. Ils sont faits pour le solo, les mondes en LAN et les serveurs qui les acceptent.

## Plusieurs parties, plusieurs comptes

Chaque partie garde le compte qui était sélectionné quand tu as cliqué sur **JOUER**, même si tu changes de compte pendant son téléchargement. Choisis un compte, joue, choisis-en un autre, rejoue : les deux parties tournent côte à côte.

## Où sont rangés les comptes

- Comptes Microsoft : `msa.json`
- Comptes hors-ligne : `listpseudo.txt`

Les deux sont dans `%LOCALAPPDATA%\Misty`. Voir [Fichiers et dossiers](/docs/files-and-folders).
