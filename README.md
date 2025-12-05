# Green Optimizer
> Nuit de l'Info 2025 - Projet Green Optimizer par l'équipe ***Bon***
> Développé en Rust

## Dépendances nécessaires

- [Rust](https://www.rust-lang.org/tools/install) - Langage de programmation utilisé pour développer le projet.

## Installer le projet

Pour cloner le projet, utilisez la commande suivante :

```bash
git clone https://github.com/Vroum1/green_optimizer.git
cd green_optimizer
```

## Lancer le projet

Assurez-vous d'avoir Rust et Cargo installés sur votre machine. Vous pouvez ensuite compiler et exécuter le projet avec les commandes suivantes :

```bash
cargo build --release
cargo run --release <URL ou chemin local du fichier HTML>
```

## Exemple d'utilisation

Lancer l'analyse d'une page web :

```bash
cargo run --release https://apple.com
```

Lancer l'analyse d'un fichier HTML local :

```bash
cargo run --release ./path/to/local/file.html
```

## Fonctionnalités

- Analyse du poids d'un page web.
- Extraction et analyse des ressources (CSS, images).
- Conversion des images au format WebP pour réduire la taille.
- Minification du fichier HTML local pour optimiser la taille.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.