FROM node:latest
WORKDIR /app
ADD . .
RUN npm install
CMD ["npm","start"]

# EXPOSE expose que au moment du build pas lors du run

# sudo docker build -t serveur_web_node:latest . --rm
# sudo docker run -p 3030:3030 serveur_web_node:latest
# Conseil de sécurité :
# 127.0.0.1:8000:3030
# sudo docker run -p 127.0.0.1:8000:3030 serveur_web_node:latest

## Depuis l'intérieur du conteneur
# sudo docker run -d -p 127.0.0.1:8000:3030 --name serveur_node serveur_web_node:latest
# sudo docker exec -it serveur_node curl http://localhost:3030/       ─╯
# Hello Node.js Server!%                                      
