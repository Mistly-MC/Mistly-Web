+++
title = "Mises à jour"
description = "Comment Mistly se met à jour tout seul, comment vérifier à la main, et comment recevoir les versions bêta."
section = "help"
+++

Mistly se met à jour depuis ses [releases GitHub](https://github.com/Misty-MC/Mistly-Launcher/releases), pour l'installeur comme pour la version portable.

## Mises à jour automatiques

Au démarrage, Mistly vérifie si une version plus récente est sortie. Si oui, il affiche **Mise à jour disponible** avec le numéro de la nouvelle version et celui de la version actuelle :

- **Mettre à jour** ouvre une fenêtre avec une barre de progression (pourcentage, mégaoctets téléchargés et source), puis décompresse la nouvelle version et redémarre Mistly.
- **Plus tard** garde la version actuelle ; la question sera reposée la prochaine fois.

Le fichier téléchargé est vérifié (complet, et de la bonne version) avant d'être installé. Après le redémarrage, Mistly affiche **Mise à jour terminée** et supprime l'ancienne version.

Tes données (profils, mondes, comptes, réglages) ne sont pas touchées par une mise à jour.

### Quand GitHub ne répond pas

Si GitHub est injoignable ou refuse la demande, la mise à jour vient du site de Mistly, qui garde une copie de la dernière version.

### Raccourcis et épingles

Les raccourcis (menu Démarrer, bureau, épingles de la barre des tâches) pointent vers un `Misty.exe` dont le chemin ne change jamais, à côté des dossiers de version, et qui lance la version la plus récente. Ils continuent de fonctionner après chaque mise à jour, et les anciens raccourcis sont redirigés automatiquement.

## Vérifier à la main

**Paramètres → Dépannage → Mises à jour** affiche la version installée et un bouton **Rechercher les mises à jour**. Il t'indique si tu es déjà à jour.

## Versions bêta

**Paramètres → Dépannage → Mises à jour pré-release** propose aussi les versions bêta et pré-release publiées sur GitHub. Elles apportent les nouveautés en premier, mais peuvent avoir des bugs. Désactive le réglage pour revenir aux seules versions stables.

## Mistly Client

[Mistly Client](/docs/mistly-client) a ses propres versions : le launcher télécharge la plus récente avant une partie, sans mise à jour du launcher.

## Qu'est-ce qui a changé ?

- Dans le launcher : l'onglet **Notes de version** liste toutes les versions à gauche, et les notes de celle choisie à droite, avec des filtres pour les nouveautés, les corrections, le launcher et Mistly Client.
- Sur ce site : la page des [patch notes](/patch-notes) affiche chaque version, avec ses fichiers.

> [!NOTE]
> Mistly ne propose jamais de « mettre à jour » vers une version plus ancienne que celle installée.
