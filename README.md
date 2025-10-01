<div align="center">

 <img src="https://www.docker.com/wp-content/uploads/2023/05/symbol_blue-docker-logo.png" alt="Docker Logo" style="width:300px;">
 
---

[Présentation](#présentation) | [Objectifs](#objectifs) | [Structure](#structure) | [Exercices](#exercices) | [Technologies](#technologies)

</div>

---

## Présentation  

**Projet** : Apprentissage Docker Compose  

### Contexte  

Ce dépôt contient les travaux pratiques et exercices réalisés dans le cadre du cours de **Virtualisation et Conteneurisation**. Il met l'accent sur l'apprentissage de Docker Compose pour orchestrer des applications multi-conteneurs.

### Objectifs  

Ce projet vise à maîtriser :
- La configuration et déploiement d'applications multi-conteneurs avec Docker Compose
- L'orchestration de services (bases de données, serveurs web, applications)
- Les bonnes pratiques de conteneurisation
- L'intégration continue avec Gitea
- La gestion des volumes, réseaux et secrets

## Structure du Dépôt  

Le dépôt est organisé en plusieurs dossiers thématiques :

### [**TD5**](TD5) : **Travaux Dirigés 5**  
- **Exo1** : Premier exercice avec messages et images personnalisées
- **Exo2** : Application Node.js avec conteneurisation
- **Exo3** : Configuration PostgreSQL avec secrets
- **Exo4** : Client PostgreSQL avec Debian et healthchecks

### [**TD6**](TD6) : **Travaux Dirigés 6**  
- Configuration Kubernetes
- Déploiements et services
- Volumes persistants
- Applications web conteneurisées

### [**TD7**](TD7) : **Pipeline CI/CD avec Gitea**  
- **Gitea** : Serveur Git avec interface web et PostgreSQL
- **Actions Runner** : Runner configuré pour pipeline CI/CD
- **Webapp** : Application Rust avec tests automatisés
- **MongoDB** : Service de base de données pour les tests
- **Documentation** : Screenshots et archive finale

## Exercices Réalisés

### ✅ TD5 - Docker Compose Fondamentaux
- [x] **Exo1** : Création d'images personnalisées et messages
- [x] **Exo2** : Application Node.js multi-conteneurs  
- [x] **Exo3** : Base de données PostgreSQL avec gestion des secrets
- [x] **Exo4** : Client PostgreSQL avec vérifications de santé

### ✅ TD6 - Kubernetes et Orchestration
- [x] Configuration de déploiements Kubernetes
- [x] Gestion des volumes persistants
- [x] Services et expositions d'applications

### ✅ TD7 - Pipeline CI/CD avec Gitea Actions
- [x] Configuration serveur Gitea avec PostgreSQL
- [x] Gitea Actions Runner avec réseau Docker personnalisé
- [x] Pipeline CI/CD pour application Rust
- [x] Service MongoDB pour tests automatisés
- [x] Variables d'environnement et configuration complète
- [x] Documentation et screenshots de fonctionnement

## Technologies Utilisées

<div align="center">

&nbsp; ![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)
&nbsp; ![Docker Compose](https://img.shields.io/badge/Docker%20Compose-2496ED?style=for-the-badge&logo=docker&logoColor=white)
&nbsp; ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white)
&nbsp; ![MongoDB](https://img.shields.io/badge/MongoDB-47A248?style=for-the-badge&logo=mongodb&logoColor=white)
&nbsp; ![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=node.js&logoColor=white)
&nbsp; ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
&nbsp; ![Kubernetes](https://img.shields.io/badge/Kubernetes-326CE5?style=for-the-badge&logo=kubernetes&logoColor=white)
&nbsp; ![Gitea](https://img.shields.io/badge/Gitea-609926?style=for-the-badge&logo=gitea&logoColor=white)
&nbsp; ![Debian](https://img.shields.io/badge/Debian-D70A53?style=for-the-badge&logo=debian&logoColor=white)

</div>

## Installation et Utilisation

### Prérequis
- Docker Engine 20.10+
- Docker Compose 2.0+
- Git

### Démarrage Rapide

```bash
# Cloner le dépôt
git clone https://github.com/nathanverdier/Docker_Compose.git
cd Docker_Compose

# Choisir un exercice
cd TD5/Exo1  # ou tout autre dossier d'exercice

# Démarrer les services
docker-compose up -d

# Voir les logs
docker-compose logs -f

# Arrêter les services
docker-compose down
```

### Configuration Spécifique

Chaque exercice contient sa propre documentation dans son dossier respectif avec :
- Instructions spécifiques
- Variables d'environnement nécessaires
- Commandes de test

## Résolution de Problèmes

### Conflits de Ports
Si vous rencontrez des erreurs de ports déjà utilisés :
```bash
# Vérifier les ports utilisés
sudo ss -tlnp | grep :PORT_NUMBER

# Modifier le port dans compose.yaml
ports:
  - "PORT_LIBRE:PORT_CONTAINER"
```

### Volumes et Permissions
```bash
# Réinitialiser les volumes
docker-compose down -v
docker-compose up -d
```

## Branche Principale  

**Main**  

## Développeur  

<div align="center">  

<a href="https://github.com/nathanverdier">  
<img src="https://www.proservices-informatique.fr/wp-content/uploads/2023/11/abonnement-assistance-maintenance-informatique.png" height="50px">  
</a>  

<strong>VERDIER Nathan</strong>  

*Étudiant en 4ème année - Virtualisation et Conteneurisation*

</div>

---

<div align="center">

**Cours** : Virtualisation et Conteneurisation  
**Année** : 2025-2026  
**Institution** : ESIEA - Laval

</div>
