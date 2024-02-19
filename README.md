# Comprendre la Visibilité des Fonctions des Smart Contracts en Rust sur Solana

![](assets/pexels-james-frid-901974.jpg)
(*Photo : James Frid - [Pexels](https://www.pexels.com/fr-fr/photo/telescope-gris-et-or-sur-batiment-901974/)*)

## TL;DR

1. Cet article vous fournira une introduction à la visibilité des fonctions des smart contracts en Rust sur Solana.
2. Une comparaison sera faite avec Solidity sur Ethereum.
3. Les fonctions des smart-contracts définissent le comportement et les fonctionnalités d'un contrat sur une blockchain.
3. La visibilité des fonctions détermine comment elles peuvent être appelées depuis l'intérieur ou l'extérieur d'une blockchain.


## Introduction

Les smart contracts sont au cœur de l'écosystème blockchain, offrant des fonctionnalités décentralisées et autonomes qui sous-tendent un large éventail d'applications.

Dans cet article, nous explorerons la visibilité des fonctions des smart contracts en Rust sur la blockchain Solana.

Pour simplifier la compréhension, nous établirons un parallèle avec les contrats en Solidity pour Ethereum, mettant en lumière les différences entre les deux approches.


## Solidity

Les fonctions des smart contracts en **Solidity** sont des blocs de code qui définissent le comportement et les fonctionnalités d'un contrat sur la blockchain **Ethereum**. Voici quelques points clés à savoir sur les fonctions des smart contracts en Solidity :

1. **Déclaration :** Les fonctions sont déclarées à l'intérieur du contrat à l'aide du mot-clé `function`.
2. **Visibilité :** Les fonctions peuvent avoir différents niveaux de visibilité, tels que `public`, `private`, `internal` et `external`, qui déterminent comment elles peuvent être appelées depuis l'extérieur du contrat.
3. **Paramètres :** Les fonctions peuvent accepter des paramètres, qui sont des valeurs fournies lors de l'appel de la fonction. Ces paramètres peuvent être de différents types de données, tels que des entiers, des chaînes de caractères, des tableaux, etc.
4. **Valeurs de retour :** Les fonctions peuvent également renvoyer des valeurs après leur exécution. Vous pouvez spécifier le type de valeur de retour à l'aide du mot-clé `returns`.
5. **Modificateurs :** Les modificateurs sont des morceaux de code réutilisables qui peuvent modifier le comportement d'une fonction. Ils sont souvent utilisés pour ajouter des conditions de sécurité ou des vérifications préalables à l'exécution de la fonction.
6. **Evenements :** Les fonctions peuvent émettre des événements à l'aide du mot-clé `emit`. Les événements sont utiles pour notifier les clients externes de l'état ou des actions importantes effectuées par le contrat.


### Visibilité en Solidity

Voici les différents niveaux de visibilité disponibles pour les fonctions en **Solidity** :

1. **Public (`public`) :** Les fonctions publiques peuvent être appelées depuis n'importe où, à la fois à l'intérieur du contrat qui les définit et depuis d'autres contrats ou depuis l'extérieur de la blockchain. Elles sont généralement utilisées pour définir des points d'entrée ou des interfaces pour interagir avec le contrat.
2. **Privé (`private`) :** Les fonctions privées ne peuvent être appelées que depuis d'autres fonctions définies dans le même contrat. Elles ne sont pas accessibles depuis l'extérieur du contrat ou depuis d'autres contrats. Elles sont généralement utilisées pour encapsuler la logique interne du contrat et pour éviter toute interférence externe.
3. **Interne (`internal`) :** Les fonctions internes sont similaires aux fonctions privées, mais elles peuvent également être appelées depuis les contrats dérivés (*hérités*). Elles ne sont pas accessibles depuis l'extérieur du contrat.
4. **Externe (`external`) :** Les fonctions externes sont similaires aux fonctions publiques, mais elles ne peuvent être appelées que depuis l'extérieur de la blockchain (*c'est-à-dire par d'autres contrats ou par des transactions externes*). Elles ne peuvent pas être appelées depuis l'intérieur du contrat qui les définit.

### Exemple en Solidity

```solidity
contract MainContract {

    function publicFunction() public {
        // Code de la fonction
    }

    function privateFunction() private {
        // Code de la fonction
    }

    function internalFunction() internal {
        // Code de la fonction
    }

    function externalFunction() external {
        // Code de la fonction
    }
}

contract OtherContract {
    // TO DO
}

```

En choisissant le niveau de visibilité approprié pour chaque fonction, les développeurs peuvent contrôler comment ces fonctions sont accessibles et utilisables, ce qui contribue à la **sécurité** et à la **clarté** du contrat.


## Rust & Anchor

Dans **Rust** avec le framework **Anchor** pour la blockchain **Solana**, les fonctions des smart contracts sont définies à l'aide du langage Rust et de la bibliothèque Anchor. Voici quelques points clés à savoir sur les fonctions des smart contracts en Rust avec Anchor :

1. **Déclaration :** Les fonctions sont définies à l'intérieur d'une structure de contrat Solana et annotées avec des attributs spécifiques à Anchor.
2. **Attributs spécifiques à Anchor :** Anchor fournit plusieurs attributs spécifiques pour annoter les fonctions des contrats Solana, tels que `#[instruction]` pour les instructions, `#[state]` pour les états et `#[derive(Accounts)]` pour la spécification des comptes nécessaires à l'exécution de la fonction.
3. **Fonctions d'instruction :** Les fonctions marquées avec l'attribut `#[instruction]` sont des instructions du contrat Solana qui peuvent être appelées depuis l'extérieur de la chaîne. Elles définissent les fonctionnalités et les actions du contrat.
4. **Fonctions de vérification :** Les fonctions de vérification sont utilisées pour valider les transactions et peuvent être marquées avec l'attribut `#[instruction]` pour indiquer qu'elles sont appelées en tant qu'instructions, ou avec l'attribut `#[guard]` pour indiquer qu'elles sont utilisées pour la validation uniquement.
5. **Gestion des comptes :** Anchor facilite la gestion des comptes nécessaires à l'exécution des fonctions en utilisant l'attribut `#[derive(Accounts)]`, qui spécifie les comptes impliqués dans une transaction et leur rôle (*par exemple, compte source, compte destinataire, compte d'état du contrat, etc.*).

**TO DO**

### Visibilité avec Rust & Anchor

**TO DO**

### Exemple avec Rust & Anchor

**TO DO**


## Conclusion

**TO DO**


## Ressources additionnelles

- **Blockchains :**
  - 🇬🇧 [Home | ethereum.org](https://ethereum.org)
  - 🇬🇧 [Web3 Infrastructure for Everyone | Solana](https://solana.com/)

- **Solidity :**
  - 🇬🇧 [Home | Solidity Programming Language](https://soliditylang.org/)
  - 🇬🇧 [Solidity - Wikipedia](https://en.wikipedia.org/wiki/Solidity)
  - 🇫🇷 [Solidity — Wikipédia](https://fr.wikipedia.org/wiki/Solidity)
  - 🇫🇷 [Solidity — Documentation Solidity (latest)](https://solidity-fr.readthedocs.io/fr/latest/)

- **Rust :**
  - 🇬🇧 [Rust Programming Language](https://www.rust-lang.org/)
  - 🇬🇧 [Rust (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))
  - 🇫🇷 [Rust (langage) — Wikipédia](https://fr.wikipedia.org/wiki/Rust_(langage))

- **Anchor :**
  - 🇬🇧 [Anchor - Introduction](https://www.anchor-lang.com/)
  - 🇬🇧 [Anchor By Example - Introduction](https://examples.anchor-lang.com/)
  - 🇬🇧 [GitHub - coral-xyz/anchor: ⚓ Solana Sealevel Framework](https://github.com/coral-xyz/anchor)

- **Seahorse :**
  - 🇬🇧 [Seahorse (Beta) | Solana programs in Python](https://seahorse-lang.org/)
  - 🇬🇧 [How to Write Solana Programs in Python Using Seahorse](https://www.alchemy.com/overviews/solana-seahorse)

- **Sandbox :**
  - 🇬🇧 [Remix - Ethereum IDE](https://remix.ethereum.org/)
  - 🇬🇧 [Solana Playground | Solana IDE](https://beta.solpg.io/)

