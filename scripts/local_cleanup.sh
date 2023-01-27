#!/usr/bin/env bash

set -euxo pipefail

if ! [ -x "$(command -v docker)" ]; then
  >&2 echo "Error: docker is not installed."
  exit 1
fi

function clean_docker_image() {
  local filter=$1;
  local container_id=$(docker container ls -a \
    --filter 'name=$filter' \
    --format '{{.ID}}')
  if [[ -n $container_id ]]; then
    echo >&2 "there is a jaeger/all-in-one container already running"
    echo >&2 "removing container..."
    docker kill $container_id || docker rm $container_id
  fi
}

clean_docker_image 'poll-jaeger-aio'
clean_docker_image 'poll-database-mysql'
clean_docker_image 'poll-database-postgresql'

>&2 echo "Cleaned!"
