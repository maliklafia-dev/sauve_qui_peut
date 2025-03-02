# 🏆 Sauve qui peut - Client TCP en Rust 🦀

## 📌 Description
**Sauve qui peut** est un client Rust conçu pour interagir avec un **serveur TCP** dans le cadre d’un jeu de l'abirinthe où l'objectif est d'en sortir.  
Il permet d’enregistrer une équipe, d’inscrire un joueur et de communiquer avec le serveur via des **messages JSON** pour effectuer des actions en jeu. (se déplacer devant, à droite, dèrrière, à droite)

## 🚀 Fonctionnalités
- **Connexion à un serveur TCP**  
- **Enregistrement d’une équipe avec un nom unique**  
- **Inscription d’un joueur avec un token d’enregistrement**  
- **Envoi et réception de messages JSON**  
- **Gestion d’une boucle de jeu avec des actions dynamiques**  
- **Tests unitaires avec un serveur mocké**  

---

## 📦 Installation

### 1️⃣ Prérequis
- **Rust** installé ([`rustup`](https://rustup.rs/) recommandé)
- Un serveur TCP fonctionnel (ou utilisation du serveur mocké pour les tests)

### 2️⃣ Cloner le projet
```sh
git clone https://github.com/maliklafia-dev/sauve_qui_peut.git
cd client/src
```

### 📁 Structure du projet
![image](https://github.com/user-attachments/assets/96d6a282-d40e-4b00-bbc0-c973fda4b715)

