+++
title = "Dépannage"
description = "Les problèmes les plus courants, ce qu'ils veulent dire et comment les régler. Et comment signaler un bug."
section = "help"
+++

## La liste des versions ne se charge pas

*Impossible de charger les versions* signifie que les serveurs de Mojang n'ont pas répondu. Vérifie ta connexion Internet, puis redémarre Mistly.

## Un mod loader n'apparaît pas

Mistly ne propose que les loaders qui existent pour la version choisie. NeoForge commence à la 1.20.1, Fabric et Quilt à la 1.14, et LiteLoader s'arrête à la 1.12.2. Voir [Versions et mod loaders](/docs/versions-and-loaders).

## Un mod refuse de s'installer

| Message | Que faire |
|---|---|
| *… n'a pas de version compatible avec …* | Le mod n'existe pas pour cette version et ce loader. Essaie une autre version de Minecraft ou un autre loader. |
| *L'auteur de … n'autorise pas le téléchargement depuis d'autres applications que CurseForge* | Télécharge le fichier depuis sa page CurseForge et place-le dans le dossier indiqué. |
| *Choisis un mod loader (Fabric, Forge…) pour installer des mods* | Le profil est en Vanilla ou OptiFine. Crée un profil avec un mod loader. |
| *Shaders non pris en charge* | Utilise un profil OptiFine, ou installe Iris (Fabric, NeoForge) ou Oculus (Forge). |

## Le jeu ne démarre pas ou plante

1. Regarde l'erreur affichée dans Mistly (*Échec du lancement*, *Échec du téléchargement*…).
2. Ouvre la [console](/docs/console) (la tuile **Logs**) : elle s'ouvre toute seule après un plantage. Filtre sur **Erreurs** pour trouver la ligne qui compte.
3. Pour les fichiers complets, lis `logs\latest.log`, ou le fichier le plus récent de `crash-reports\`. Voir [Fichiers et dossiers](/docs/files-and-folders#les-journaux-du-jeu).
4. Si tu utilises beaucoup de mods, augmente la mémoire dans les [Paramètres](/docs/settings).
5. Désactive un par un les derniers mods ajoutés, depuis l'onglet **Contenu** du profil.

## « Session invalide » en rejoignant un serveur

La session Microsoft avait expiré. Mistly la renouvelle maintenant avant de lancer une partie ; si cela arrive encore, ferme le jeu et clique à nouveau sur **JOUER**.

## Les téléchargements échouent

Baisse **Téléchargements simultanés** dans **Paramètres → Ressources**, puis réessaie.

## La connexion Microsoft échoue

Le message indique ce qui ne va pas avec le compte : pas de profil Xbox, vérification d'âge, compte enfant, jeu non possédé… Voir [Comptes](/docs/accounts#si-la-connexion-échoue).

## « Le jeu est peut-être encore ouvert »

Certains fichiers ne peuvent pas être supprimés pendant que Minecraft tourne. Ferme le jeu, puis réessaie.

## « Limite de requêtes atteinte »

Mojang limite temporairement le nombre de demandes. Mistly affiche ton dernier skin et tes capes connus et réessaie tout seul. Voir [Skin et cosmétiques](/docs/skins).

## Te faire aider

Le plus rapide : dans la [console](/docs/console), clique sur **Envoyer les logs** et donne le code de rapport sur le [serveur Discord](https://discord.gg/rdQJUgvUh3).

## Signaler un bug

Ouvre une issue sur [GitHub](https://github.com/Mistly-MC/Mistly-Launcher/issues) avec :

- la version de Mistly (Paramètres → Dépannage), ou le fichier créé par **Exporter les infos** ;
- la version de Minecraft et le mod loader ;
- ce que tu as fait, ce que tu attendais et ce qui s'est passé ;
- `logs\latest.log` et, après un plantage, le rapport de `crash-reports\`.
