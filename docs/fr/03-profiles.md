+++
title = "Profils"
description = "Un profil par façon de jouer : sa version, son mod loader, ses mods et ses mondes."
section = "play"
+++

Un profil est une installation de Minecraft à part, comme une instance dans d'autres launchers. Chaque profil a :

- sa propre **version** et son **mod loader** ;
- son propre **dossier de jeu** : `mods`, `resourcepacks`, `shaderpacks`, `saves` (les mondes) et les options.

Les versions, les bibliothèques et les ressources restent **partagées** entre les profils : rien n'est téléchargé deux fois. Par défaut, ta **liste de serveurs et tes barres d'objets enregistrées** sont partagées aussi : un serveur ajouté une fois apparaît dans tous les profils (désactivable dans les [Paramètres](/docs/settings#comportement)).

## Créer un profil

1. Ouvre l'onglet **Profils**.
2. Clique sur **Nouveau profil**.
3. Donne-lui un nom, choisis la version et le mod loader.
4. Clique sur **Créer le profil**.

Pour partir d'un ensemble de mods tout prêt, utilise plutôt **Installer un modpack** ou **Importer un modpack**. Voir [Modpacks](/docs/modpacks).

## Les cartes de profil

L'onglet Profils affiche tes profils en cartes, avec l'image de leur version de Minecraft, leur version, leur mod loader et une étiquette **Modpack** quand ils viennent d'un modpack. Les filtres du haut ne gardent qu'un mod loader.

En bas de chaque carte : la date de la dernière partie, et **Jouer**, **Gérer** et **Supprimer**.

## Jouer avec un profil

Clique sur le profil dans la colonne de gauche de la fenêtre (ou sur **Jouer** sur sa carte), puis sur **JOUER**. *Jeu rapide* utilise le dossier de jeu commun, sans profil.

## Gérer un profil

**Gérer** ouvre le profil :

- voir et gérer son **Contenu**, ou **Découvrir** de nouveaux mods, packs et shaders. Voir [Mods, packs et shaders](/docs/mods-and-content) ;
- cocher **Mistly Client** pour ajouter le client en jeu au profil (profils Fabric d'une version compatible). Voir [Mistly Client](/docs/mistly-client) ;
- le **renommer** ou **changer son icône** ;
- **Exporter le modpack** pour le partager. Voir [Modpacks](/docs/modpacks#exporter-un-profil) ;
- **ouvrir le dossier du profil** dans l'Explorateur Windows.

> [!WARNING]
> Supprimer un profil efface définitivement ses mods, ses packs, ses mondes et ses réglages.

## Où sont les profils

Chaque profil est un dossier dans `%LOCALAPPDATA%\Misty\instances\`. Voir [Fichiers et dossiers](/docs/files-and-folders).
