FROM debian:latest
RUN apt-get update && apt-get install -y postgresql-client

# Crée un réseau docker

# sudo docker network create my-network
# sudo docker network ls

# Crée 2 conteneurs 

# sudo docker run -d --name postgres-conteneur --network my-network -e POSTGRES_USER=john -e POSTGRES_PASSWORD=monmotdepasse -e POSTGRES_DB=john postgres
# sudo docker build -t client1 . --rm
# sudo docker run --rm --network my-network -it client1 psql -h postgres-conteneur -U john -d john

# sudo docker build -t client2 . --rm
# sudo docker run --rm -it client2 psql -h postgres-conteneur -U john -d john

