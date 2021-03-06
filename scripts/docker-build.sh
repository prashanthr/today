#!/usr/bin/env bash
# TAG_NAME=$DOCKER_REPO:$TAG_NAME
if [ -z $TAG ]; then 
  TAG="today:dev"
fi

echo "Building docker image with tag $TAG..."
docker build -t $TAG .
