#!/bin/bash

# Env Variables
export $(grep -v '^#' ../.env | xargs)
DOCKERFILE="../Dockerfile"

# Define the network and container names
NETWORK="my-net"
MYSQL_CONTAINER_NAME=mysql-container
APP_CONTAINER_NAME=app-container

# Stop and remove previous containers
echo "Removing old network and containers..."
docker stop $MYSQL_CONTAINER_NAME $MYSQL_CONTAINER_NAME
docker rm $MYSQL_CONTAINER_NAME $APP_CONTAINER_NAME

# Remove the previous network
docker network rm $NETWORK

# Create a new Docker network
docker network create $NETWORK


# Clone the repository
echo "Cloning repository..."
git clone -b $BRANCH_NAME https://github.com/arivami/weather-stats.git /tmp/repository

# Build the App Docker image
echo "Building Docker image..."
docker build -t $IMAGE_NAME -f $DOCKERFILE /tmp/repository


# Clean up cloned repository
echo "Cleaning up..."
rm -rf /tmp/repository

echo "Docker image build complete."

# Run both containers

#Run DB container from Docker Hub
echo "Running DB"
docker run -d --network=$NETWORK --name mysql-container \
    -e MYSQL_ROOT_PASSWORD=$DB_PASS \
    -e MYSQL_DATABASE=$DB_NAME\
    mysql:latest

# Run app container
echo "Running App"
docker run -it --network=$NETWORK --name $APP_CONTAINER_NAME  \
    -e DB_HOST=$MYSQL_CONTAINER_NAME \
    -e DB_USER=$DB_USER \
    -e DB_PASS=$DB_PASS \
    -e DB_NAME=$DB_NAME \
    $IMAGE_NAME