# Debit Trainer 🎸

Application d'entraînement au débit rythmique à la guitare (travail des doubles-croches avec 3 notes jouées sur 4 et un quart de soupir).

L'application propose un rendu vectoriel temps réel sous forme de tablature 4/4 conforme au standard d'édition musicale (clé de sol, métrique 4/4, coups de médiator Bas/Haut, hampes, doubles ligatures et quarts de soupir), avec suivi lumineux du curseur synchronisé au métronome.

---

## 🚀 Modalités de lancement

L'application fonctionne sous deux formes : **application de bureau native** (recommandée pour la latence audio minimale) ou **application Web dans le navigateur via WebAssembly**.

### 1. Version Bureau (Desktop native)

#### Prérequis
- Avoir installé [Rust et Cargo](https://rustup.rs/) (édition 2021+).

#### Lancement
Depuis la racine du projet :

```bash
# Lancement en mode développement
cargo run

# Ou en mode release (optimisé)
cargo run --release
```

---

### 2. Version Web (WebAssembly / Navigateur)

L'application est entièrement compilable en WebAssembly grâce à [`eframe` / `egui`](https://github.com/emilk/egui) et gère l'audio web via l'API Web Audio (`web-sys`).

#### Prérequis
1. **La cible WebAssembly pour Rust :**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. **Le gestionnaire de build web `trunk` :**
   ```bash
   cargo install trunk
   ```

#### Lancement dans le navigateur
Lancez le serveur de développement local avec rechargement à chaud et ouverture automatique :

```bash
trunk serve --open
```

Si le navigateur ne s'ouvre pas automatiquement, rendez-vous à l'adresse :
👉 **[http://127.0.0.1:8080](http://127.0.0.1:8080)**

#### Compiler pour déploiement web (fichiers statiques)
Pour générer les fichiers statiques HTML, JS et WASM prêts pour la production :

```bash
trunk build --release
```
Les fichiers générés se trouveront dans le dossier `dist/`.

---

## 🛠️ Commandes de test et de validation

```bash
# Lancer les tests unitaires
cargo test

# Vérifier la compilation native
cargo check

# Vérifier la compilation WebAssembly
cargo check --target wasm32-unknown-unknown

# Linter avec Clippy
cargo clippy
```

---

## 🎼 Fonctionnalités clés

- **Visualisation Tablature 4/4 Responsive :**
  - Mesure plafonnée à la largeur idéale de ~95 px par temps (calibrée sur partition).
  - Organisation automatique à 1 ou 2 mesures par ligne selon la largeur de l'écran.
- **Entraînement par blocs :**
  - Choix du nombre de répétitions par motif (1 à 10 fois).
  - File d'attente aléatoire sans répétition immédiate d'un même motif.
  - Aperçu compact des 2 prochains motifs à venir.
- **Contrôles Audio :**
  - Échantillons audio réels (clic de métronome et note de guitare Sol4 en frette 5 corde de Ré).
  - Réglage indépendant des volumes du clic et de la guitare.
  - Vitesse réglable de 40 à 220 BPM.
