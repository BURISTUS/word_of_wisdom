#!/bin/bash

docker rm $(docker ps -aq) --force

docker rmi $(docker images -a -a)

docker compose up -d

# docker build -t renat_tcp . --cache-from renat_tcp:latest
