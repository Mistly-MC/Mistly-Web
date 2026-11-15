+++
title = "Mistly Client"
description = "Mistly côté jeu : un HUD, un zoom, le fullbright, un vestiaire et des cosmétiques que les autres joueurs Mistly voient."
section = "play"
+++

Mistly Client est un **mod Fabric** qui amène Mistly dans le jeu, comme le font les clients PvP : un menu principal et un écran de chargement aux couleurs de Mistly, un HUD que tu arranges toi-même, et les cosmétiques de l'[onglet Skin](/docs/skins).

Il n'est pas intégré au launcher : Mistly le télécharge depuis GitHub la première fois qu'une partie en a besoin, et vérifie s'il existe une version plus récente avant de jouer. Une nouvelle version de Mistly Client t'arrive donc sans mise à jour du launcher.

## Versions compatibles

Mistly Client fonctionne sur Minecraft **1.21.11, 26.1, 26.1.1, 26.1.2, 26.2 et 26.3**, avec Fabric.

## Jouer avec Mistly Client

Deux façons :

- **Depuis le sélecteur de version.** Active le filtre **Mistly Client**, sélectionne une version et valide avec le bouton **Choisir …**. Mistly lance Fabric avec Mistly Client pour toi.
- **Dans un profil.** Sur un profil Fabric d'une version compatible, coche la case **Mistly Client** à côté de Contenu et Découvrir. Cochée, le mod est ajouté au profil ; décochée, il est retiré. Chaque profil garde son choix.

**Paramètres → Dépannage → Mistly Client coché par défaut** choisit si la case est cochée d'office sur les nouveaux profils.

## En jeu

### Menus

Le menu principal affiche le paysage de Mistly, les entrées habituelles et ton personnage en 3D avec ce que tu portes. Les menus Solo, Multijoueur et Options ont le même habillage, et l'écran de chargement de Mojang est remplacé par celui de Mistly. Le menu Mistly peut être désactivé, puis réactivé depuis le menu classique.

### HUD

Appuie sur **Maj droite** en jeu pour arranger le HUD : fais glisser un module pour le déplacer, clique dessus pour l'activer ou le désactiver, utilise la molette pour le redimensionner. Les modules qui ont leurs propres options ont un engrenage dans les réglages Mistly.

| Module | Ce qu'il affiche |
|---|---|
| FPS, coordonnées, horloge | Les bases |
| Ping | Ton ping, mesuré par Mistly et rafraîchi chaque seconde. Vert sous 80 ms, jaune sous 150, orange sous 250, rouge au-delà |
| CPS et touches | Les clics par seconde et les touches que tu presses |
| Armure | Ton armure et sa durabilité |
| Reach | La distance du dernier joueur que tu as touché, en blocs |
| Combo | Les coups que tu enchaînes sans être touché en retour |
| Musique | Le morceau qui joue sur ton PC (Spotify, Deezer, un navigateur…), lu depuis Windows. Aucune connexion à faire |

### Autres fonctions

- **Zoom** : maintiens `C`, et utilise la molette pour zoomer plus ou moins. La touche se change dans Options → Contrôles.
- **Fullbright** : tout est éclairé, grottes et nuits comprises.
- **Hitboxes** : affiche les hitboxes autour de toi, comme `F3 + B`, sans ouvrir F3.
- **Vestiaire** : équipe tes cosmétiques en jeu. Ce que tu y équipes est gardé par le launcher.

## Le réseau Mistly

Pour montrer tes cosmétiques et le nuage Mistly à côté de ton pseudo (au-dessus de ta tête, dans la liste des joueurs et dans le chat) aux autres joueurs Mistly, le jeu prouve ton compte Microsoft au site de Mistly de la même façon que lorsqu'il rejoint un serveur. Ton jeton Minecraft ne quitte jamais le jeu. Le site sait alors que tu es en ligne et ce que tu portes.

**Réseau Mistly**, dans les réglages Mistly en jeu, désactive tout cela. La page [Amis](/docs/friends) en dépend.

## Bon à savoir

- Si une partie de Mistly Client ne convient pas à une version de Minecraft, seule cette fonction est désactivée : le jeu démarre quand même.
- À chaque lancement, Mistly ne garde dans le dossier `mods` du profil que le fichier Mistly Client le plus récent pour la version de Minecraft, et supprime les anciens.
- Les modpacks livrés avec un Fabric Loader plus ancien fonctionnent : Mistly Client ne demande que la version du loader dont il a besoin.
