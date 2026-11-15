+++
title = "Versions et mod loaders"
description = "Le sélecteur de version, les versions de Minecraft que Mistly affiche, et les mod loaders proposés pour chacune."
section = "play"
+++

## Le sélecteur de version

Clique sur la tuile de version dans la barre du bas pour ouvrir le sélecteur au milieu de la fenêtre (clique à nouveau dessus pour le fermer).

- À gauche, une **affiche par mise à jour** (1.21 Tricky Trials, 1.20 Trails & Tales, 26.x…), chacune avec son image en pixel art.
- À droite, la mise à jour choisie avec ses **versions** et les **mod loaders** disponibles pour la version que tu sélectionnes.
- Un seul bouton dit ce que tu t'apprêtes à choisir, par exemple **Choisir 1.21.4 · Fabric**. Rien ne change tant que tu n'as pas cliqué dessus.

En haut, une **barre de recherche** retrouve vite une version, et des filtres choisissent ce qui est listé : **Releases**, **Snapshots** (versions de test de la prochaine release), **Bêtas**, **Alphas** (les toutes premières versions du jeu), ainsi que [Mistly Client](/docs/mistly-client) et [Horizon](/docs/horizon), qui se choisissent de la même façon.

## Mod loaders

Un mod loader est ce qui permet à Minecraft de charger des mods. Quand tu choisis une version, Mistly demande à chaque loader s'il existe pour cette version, et n'affiche que ceux qui existent. *Vanilla* (sans loader) est toujours disponible.

| Loader | Versions de Minecraft | Remarques |
|---|---|---|
| Forge | 1.7.10 et suivantes | Le loader historique, une immense bibliothèque de mods. |
| NeoForge | 1.20.1 et suivantes | Fork de Forge pour les versions récentes. |
| Fabric | 1.14 et suivantes | Léger et vite mis à jour. |
| Quilt | 1.14 et suivantes | Fork de Fabric ; les profils Quilt chargent aussi les mods Fabric. |
| OptiFine | 1.7.2 et suivantes | Performances et shaders. N'installe pas d'autres mods. |
| LiteLoader | 1.5.2 à 1.12.2 | Pour les anciennes versions. |

Ces plages sont un premier filtre : un loader peut quand même manquer pour une version précise s'il n'a jamais été publié pour elle. Pour Quilt, Mistly prend le dernier loader stable et ignore les versions bêta.

> [!NOTE]
> Les mods ne s'installent que sur un profil avec un mod loader (Forge, NeoForge, Fabric, Quilt ou LiteLoader). Les profils Vanilla et OptiFine acceptent les packs de ressources, les datapacks et, pour OptiFine, les shaders.

## Shaders

Minecraft ne sait pas charger de shaders tout seul. Mistly t'indique ce qu'il faut quand tu installes un shader :

- **OptiFine** charge les shaders directement ;
- avec **Fabric** ou **NeoForge**, il faut le mod **Iris** ;
- avec **Forge**, il faut le mod **Oculus**.

Quand le mod nécessaire manque, Mistly propose de l'installer avec le shader.

## Ce qui est téléchargé

Le premier lancement d'une version télécharge le jeu, ses bibliothèques et ses ressources, ainsi que la version de Java qu'il lui faut. Le loader s'installe par-dessus. Tout est partagé entre les profils : chaque version n'est téléchargée qu'une fois. Une tuile **Annuler** arrête un téléchargement en cours.
