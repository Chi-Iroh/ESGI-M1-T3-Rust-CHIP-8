# Émulateur CHIP-8

Brad Dos Santos Patatas  
Robin Chaussy  
Thomas Sayen

## Démarrer le projet

```bash
cargo run
```

## Norme de commits

La norme [conventional commits](https://www.conventionalcommits.org/) a été utilisée pour ce projet.  

```bash
bash setup-git-hooks.sh
```
Cette commande va installer un `hook` git pour vérifier le message de commit, et le bloquer s'il n'est pas conforme.  

## Tests

Les programmes de test se trouvent dans le dossier `test-ch8`.  
Pour en exécuter un, utiliser la commande ci-dessous :  
```bash
cargo run -- -i test-ch8/fichier_de_test.ch8
```
