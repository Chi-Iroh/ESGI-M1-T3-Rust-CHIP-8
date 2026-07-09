# Programmes de test

Les programmes de test se trouvent dans le dossier `test-ch8`.  
Pour en exécuter un, utiliser la commande ci-dessous :  
```bash
cargo run -- -i test-ch8/fichier_de_test.ch8
```

## add_registers.ch8

Met 1, 2 et 3 dans V1, V2 et V3.  
Ajoute V1, V2 et V3 à V0.  
V0 contient 6.  

## jmp.ch8

Se déplace au 6e octet du code, en sautant 4 octets mis à 0.  
Met 0x0A dans V0.  

## function.ch8

Ajoute 1 à V0.  
Appelle une fonction où on ajoute 2 à V0, on return.  
Ajoute 3 à V0, V0 contient 6.  

## if.ch8

Ajoute 1 à V0.  
Si V0 vaut 1 (et c'est le cas), saute l'instruction suivante (qui ajoute 0x0A à V0).  
V0 vaut donc 1.  
