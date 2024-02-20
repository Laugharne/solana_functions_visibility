# Visibilité des Fonctions de smart-contracts en Rust sur Solana

![](assets/pexels-james-frid-901974.jpg)
(*Photo : James Frid - [Pexels](https://www.pexels.com/fr-fr/photo/telescope-gris-et-or-sur-batiment-901974/)*)

## TL;DR

1. Cet article vous fournira une introduction à la visibilité des fonctions des smart-contracts en Rust sur Solana.
2. Une comparaison sera faite entre Rust & Anchor sur Solana et Solidity sur Ethereum.
3. Les fonctions des smart-contracts définissent le comportement et les fonctionnalités d'un contrat sur une blockchain.
3. La visibilité des fonctions spécifie comment elles seront appelées depuis l'intérieur ou l'extérieur d'une blockchain.


## Introduction

Les smart-contracts sont au cœur de l'écosystème blockchain, offrant des fonctionnalités décentralisées et autonomes qui sous-tendent un large éventail d'applications.

Dans cet article, nous explorerons **la visibilité des fonctions des smart-contracts en Rust sur la blockchain Solana**.

Pour simplifier la compréhension, nous établirons un parallèle avec les contrats en Solidity pour Ethereum, mettant en lumière les différences entre les deux approches.


## Solidity

Les fonctions des smart-contracts en [**Solidity**](https://soliditylang.org/) (🇬🇧) sont des blocs de code qui définissent le comportement et les fonctionnalités d'un contrat sur la blockchain **Ethereum**. Voici quelques points clés à savoir à ce sujet :

- **Déclaration** : Les fonctions sont déclarées à l'intérieur du contrat à l'aide du mot-clé `function`.
- **Visibilité** : Les fonctions peuvent avoir différents niveaux de visibilité, tels que `public`, `private`, `internal` et `external`, qui déterminent comment elles peuvent être appelées depuis l'extérieur du contrat.
- **Paramètres** : Les fonctions peuvent accepter des paramètres, qui sont des valeurs fournies lors de l'appel de la fonction. Ces paramètres peuvent être de différents types de données, tels que des entiers, des chaînes de caractères, des tableaux, etc.
- **Valeurs de retour** : Les fonctions peuvent également renvoyer des valeurs après leur exécution. Vous pouvez spécifier le type de valeur de retour à l'aide du mot-clé `returns`.
- **Modificateurs** : Les modificateurs sont des morceaux de code réutilisables qui peuvent modifier le comportement d'une fonction. Ils sont souvent utilisés pour ajouter des conditions de sécurité ou des vérifications préalables à l'exécution de la fonction.
- **Evenements** : Les fonctions peuvent émettre des événements à l'aide du mot-clé `emit`. Les événements sont utiles pour notifier les clients externes de l'état ou des actions importantes effectuées par le contrat.


### Visibilité en Solidity

Voici les différents niveaux de visibilité disponibles pour les fonctions en **Solidity** :

- **Public (`public`)** : Les fonctions publiques peuvent être appelées depuis n'importe où, à la fois à l'intérieur du contrat qui les définit et depuis d'autres contrats ou depuis l'extérieur de la blockchain. Elles sont généralement utilisées pour définir des points d'entrée ou des interfaces pour interagir avec le contrat.
- **Privé (`private`)** : Les fonctions privées ne peuvent être appelées que depuis d'autres fonctions définies dans le même contrat. Elles ne sont pas accessibles depuis l'extérieur du contrat ou depuis d'autres contrats. Elles sont généralement utilisées pour encapsuler la logique interne du contrat et pour éviter toute interférence externe.
- **Interne (`internal`)** : Les fonctions internes sont similaires aux fonctions privées, mais elles peuvent également être appelées depuis les contrats dérivés (*hérités*). Elles ne sont pas accessibles depuis l'extérieur du contrat.
- **Externe (`external`)** : Les fonctions externes sont similaires aux fonctions publiques, mais elles ne peuvent être appelées que depuis l'extérieur de la blockchain (*c'est-à-dire par d'autres contrats ou par des transactions externes*). Elles ne peuvent pas être appelées depuis l'intérieur du contrat qui les définit.

### Exemple en Solidity

```solidity
// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

// Contract A
contract ContractA {
    // Variable d'état publique
    uint256 public publicVariable;

    // Variable d'état privée
    uint256 private privateVariable;

    // Constructeur du contrat
    constructor(uint256 _initialValue) {
        publicVariable = _initialValue;
        privateVariable = _initialValue;
    }

    // Fonction publique
    function publicFunction() public returns (uint256) {
        return publicVariable;
    }

    // Fonction privée
    function privateFunction() private returns (uint256) {
        return privateVariable;
    }

    // Fonction externe
    function externalFunction() external returns (uint256) {
        // Appel de la fonction privée à partir d'une fonction publique
        return privateFunction();
    }

    // Fonction interne
    function internalFunction() internal returns (uint256) {
        return privateVariable;
    }

}

// Contract B
contract ContractB {
    // Déclaration d'une instance du contrat A
    ContractA instanceOfContractA;

    // Fonction publique qui appelle une fonction publique du contrat A
    function usePublicFunctionContractA() public returns (uint256) {
        // Création d'une nouvelle instance du contrat A
        instanceOfContractA = new ContractA(10);
        // Appel de la fonction publique du contrat A
        return instanceOfContractA.publicFunction();
    }

//    // Fonction publique qui appelle une fonction interne du contrat A
//    function useInternalFunctionContractA() public returns (uint256) {
//        // Appel de la fonction interne du contrat A
//        return instanceOfContractA.internalFunction(); // Erreur de compilation : internalFunction n'est pas visible ici
//    }

}
```

En choisissant le niveau de visibilité approprié pour chaque fonction, les développeurs peuvent contrôler comment ces fonctions sont [**accessibles**](assets/outside_contracts.png) (*Remix*) et [**utilisables**](assets/abi.json) (*ABI*), ce qui contribue à la **sécurité** et à la **clarté** du contrat.


## Rust & Anchor

Dans [**Rust**](https://www.rust-lang.org/) (🇬🇧) avec le framework [**Anchor**](https://www.anchor-lang.com/) (🇬🇧) pour la blockchain **Solana**, les fonctions des smart-contracts sont définies à l'aide du langage Rust et de la bibliothèque Anchor. Voici quelques points clés à savoir :

- **Déclaration** : Les fonctions sont définies à l'intérieur d'une structure de contrat Solana et annotées avec des attributs spécifiques à Anchor.
- **Attributs spécifiques** : Anchor fournit plusieurs attributs spécifiques pour annoter les fonctions des contrats Solana, tels que `#[instruction]` pour les instructions, `#[state]` pour les états et `#[derive(Accounts)]` pour la spécification des comptes nécessaires à l'exécution de la fonction.
- **Fonctions d'instruction** : Les fonctions marquées avec l'attribut `#[instruction]` sont des instructions du contrat Solana qui peuvent être appelées depuis l'extérieur de la chaîne. Elles définissent les fonctionnalités et les actions du contrat.
- **Fonctions de vérification** : Les fonctions de vérification sont utilisées pour valider les transactions et peuvent être marquées avec l'attribut `#[instruction]` pour indiquer qu'elles sont appelées en tant qu'instructions, ou avec l'attribut `#[guard]` pour indiquer qu'elles sont utilisées pour la validation uniquement.
- **Gestion des comptes** : Anchor facilite la gestion des comptes nécessaires à l'exécution des fonctions en utilisant l'attribut `#[derive(Accounts)]`, qui spécifie les comptes impliqués dans une transaction et leur rôle (*par exemple, compte source, compte destinataire, compte d'état du contrat, etc.*).

**TO DO**

### Visibilité avec Rust & Anchor

**TO DO**

### Exemple avec Rust & Anchor

**TO DO**
```rust
// TODO
``` 


## En résumé

**TO DO**

- **Publiques / Externes** : Accessibles à la fois à l'intérieur et à l'extérieur du programme. Dans Solana, toutes les fonctions déclarées sont, **par défaut**, **publiques**. Toutes fonctions dans un module avec l'attribut `#[program]` doivent être déclarées avec le mot clef `pub`.
- **Internes** : Accessibles à l'intérieur du programme lui-même et aux programmes qui en héritent. Les fonctions à l'intérieur d'un bloc `pub mod` imbriqué ne sont pas incluses dans le programme construit, mais elles peuvent toujours être accessibles à l'intérieur ou à l'extérieur du module parent.
- **Privées** : Elles ne sont pas accessibles publiquement et ne peuvent pas être invoquées depuis l'extérieur de leur module. Pour obtenir cette visibilité privée en Rust/Solana, il faut définir une fonction dans un module spécifique avec le mot-clé `pub` (*dans `crate::<module>`*), ce qui la rend visible uniquement dans le module où elle a été définie.

**TO DO**

**Note** : Rust, n'est pas le seul langage qui permette de créer des smart-contracts sur la blockchain Solana. [**Seahorse**](https://seahorse-lang.org/) (🇬🇧), par exemple permet de les programmer en [**Python**](https://www.python.org/) (🇬🇧). Seahorse s'appuie sur Anchor ainsi que sur divers autres crates (*Rust packages*) pour fonctionner.


--------

Crédits : **[Franck Maussand](mailto:franck@maussand.net)**

N'hésitez pas à jeter un coup d'oeil sur mes précédents articles sur [**Medium**](https://medium.com/@franck.maussand) (🇫🇷 **/** 🇬🇧) !

--------


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
  - 🇬🇧 [Visibility and privacy - The Rust Reference](https://doc.rust-lang.org/beta/reference/visibility-and-privacy.html)
  - 🇬🇧 [Rust (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))
  - 🇫🇷 [Rust (langage) — Wikipédia](https://fr.wikipedia.org/wiki/Rust_(langage))

- **Anchor :**
  - 🇬🇧 [Anchor - Introduction](https://www.anchor-lang.com/)
  - 🇬🇧 [Anchor By Example - Introduction](https://examples.anchor-lang.com/)
  - 🇬🇧 [GitHub - coral-xyz/anchor: ⚓ Solana Sealevel Framework](https://github.com/coral-xyz/anchor)

- **Seahorse :**
  - 🇬🇧 [Seahorse (Beta) | Solana programs in Python](https://seahorse-lang.org/)
  - 🇬🇧 [Solana Bytes - Intro to Seahorse - YouTube](https://www.youtube.com/watch?v=Wt3kcIb98Do)

- **Sandbox :**
  - 🇬🇧 [Remix - Ethereum IDE](https://remix.ethereum.org/)
  - 🇬🇧 [Solana Playground | Solana IDE](https://beta.solpg.io/)

