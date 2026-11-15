+++
title = "Modpacks"
description = "Installe, importe, mets à jour et exporte des modpacks Modrinth ou CurseForge sous forme de profils prêts à jouer."
section = "play"
+++

Un modpack est un ensemble de mods et de réglages préparé par son auteur. Mistly en fait un nouveau [profil](/docs/profiles), avec la bonne version de Minecraft et le bon mod loader (Forge, NeoForge, Fabric ou Quilt).

## Installer un modpack

1. Ouvre l'onglet **Profils**, puis clique sur **Installer un modpack**.
2. Choisis la plateforme, **Modrinth** ou **CurseForge**, et cherche.
3. Choisis la **version** du modpack que tu veux.
4. Valide : un nouveau profil est créé et ses fichiers sont téléchargés.

Mistly installe la version du mod loader demandée par le modpack, copie ses fichiers de configuration et télécharge tous les mods.

## Importer un fichier de modpack

Tu as déjà un fichier de modpack ? Clique sur **Importer un modpack** dans l'onglet Profils et choisis un `.mrpack` Modrinth ou un `.zip` CurseForge. Mistly crée un profil avec le nom écrit dans le modpack, ses mods, ses configs et le bon mod loader.

Si le fichier n'est pas un modpack, Mistly te le dit, et un profil à moitié créé est supprimé quand l'import échoue.

## Mettre à jour un modpack

Un profil de modpack a un bouton qui liste les versions du modpack (celle installée est indiquée) et passe à celle que tu choisis. Les mods et les configs du modpack sont remplacés ; **tes mondes, tes options et les mods que tu as ajoutés toi-même sont gardés**.

Un fichier importé est reconnu quand le modpack est publié sur Modrinth ou CurseForge : il peut donc être mis à jour aussi.

## Exporter un profil

**Exporter le modpack**, sur la page d'un profil, l'enregistre en fichier `.mrpack` : mods, configs, packs de ressources, shaders et options, sans les mondes. Mistly, l'app Modrinth et Prism peuvent l'importer. Mistly Client est inclus quand sa case est cochée.

> [!NOTE]
> Lance le profil une fois avant de l'exporter : avant cela, la version du mod loader n'est pas connue.

## Téléchargements manuels

Certains auteurs CurseForge n'autorisent le téléchargement que depuis le site de CurseForge. Mistly cherche d'abord ces fichiers sur **Modrinth** (le même fichier exactement, puis le projet du même nom pour ta version et ton loader). Seuls ceux qu'il ne trouve pas sont listés après l'installation :

1. Clique sur **Ouvrir les pages** : la page de chaque fichier s'ouvre dans ton navigateur.
2. Télécharge les fichiers.
3. Place chacun dans le dossier indiqué à côté de son nom (en général le dossier `mods` du profil).

Si tout le modpack est restreint de cette façon, Mistly t'indique qu'il ne peut pas être téléchargé en dehors de CurseForge.

## Erreurs courantes

| Message | Signification |
|---|---|
| *Ce modpack n'a aucune version publiée* | L'auteur n'a pas encore publié de fichier. |
| *Le modpack n'indique pas sa version de Minecraft* | Le modpack est incomplet ; essaie une autre de ses versions. |
| *Mod loader non pris en charge* | Le modpack utilise un loader que Mistly n'installe pas. |
