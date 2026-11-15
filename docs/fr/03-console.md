+++
title = "Console du jeu"
description = "Suis les logs du jeu en direct, filtre-les et copie-les, arrête un jeu bloqué ou envoie les logs pour te faire aider."
section = "tools"
+++

La console affiche ce que le jeu écrit pendant qu'il tourne, pour Minecraft comme pour [Horizon](/docs/horizon). Elle s'ouvre en grande tuile sous la rangée du haut de la fenêtre, qui reste utilisable.

## L'ouvrir

- Clique sur la tuile **Logs** en haut à droite ; clique à nouveau dessus (ou sur un onglet) pour la fermer.
- Par défaut, elle s'ouvre au lancement d'une partie, et toute seule quand un jeu plante. **Paramètres → Comportement → Ouvrir la console au lancement du jeu** change cela.

## Lire les logs

- Les lignes sont colorées par niveau : informations, avertissements, erreurs et messages de Mistly. Les erreurs gardent leur trace complète.
- **Tout / Avertissements / Erreurs** filtrent les lignes et affichent chacun un compteur. Le champ de recherche trouve un mot.
- La vue suit les dernières lignes jusqu'à ce que tu remontes ; **Suivre** te ramène en bas.

La console garde les 5 000 dernières lignes de chaque partie. Le log complet reste écrit dans le fichier de log du jeu. Voir [Fichiers et dossiers](/docs/files-and-folders#les-journaux-du-jeu).

## Copier

- Clique sur une ligne pour la sélectionner ; fais glisser ou `Maj`+clic pour en sélectionner plusieurs ; `Ctrl+A` sélectionne tout.
- `Ctrl+C` ou le bouton de copie copie la sélection.
- **Dossier des logs** ouvre le dossier des fichiers de log.

## Plusieurs parties

Quand plusieurs parties tournent en même temps, la console partage l'écran : un panneau par partie, côte à côte, puis 2 × 2 pour quatre parties, jusqu'à 3 × 2. Chaque panneau a son état, ses compteurs et ses boutons. Double-clique sur un panneau (ou utilise son bouton) pour l'agrandir.

## Arrêter un jeu

**Arrêter le jeu** met fin à un jeu bloqué. Un jeu que tu arrêtes là, ou que tu fermes toi-même (Quitter, la croix de la fenêtre, `Alt+F4`), n'est pas compté comme un plantage.

## Envoyer les logs

**Envoyer les logs** envoie les dernières lignes du jeu à l'équipe de Mistly et te donne un **code de rapport**, copié pour toi. Donne ce code sur le [serveur Discord](https://discord.gg/rdQJUgvUh3) pour te faire aider : il permet à l'équipe de retrouver tes logs.
