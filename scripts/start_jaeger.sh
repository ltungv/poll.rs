#!/usr/bin/env bash

set -exo pipefail

if ! [ -x "$(command -v docker)" ]; then
  >&2 echo "Error: docker is not installed."
  exit 1
fi

if [[ -z "${SKIP_DOCKER}" ]]; then
  CONTAINER_JAEGER_AIO=$(docker container ls -a \
    --filter 'name=poll-jaeger-aio' \
    --format '{{.ID}}')
  if [[ -n $CONTAINER_JAEGER_AIO ]]; then
    echo >&2 "there is a jaeger/all-in-one container already running"
    exit 1
  fi
fi

docker run \
-p "6831:6831/udp" \
-p "6832:6832/udp" \
-p "16686:16686/tcp" \
-p "14268:14268/tcp" \
-d \
--name "poll-jaeger-aio-$(date '+%s')" \
jaegertracing/all-in-one

>&2 echo "Jaeger is up and running!"
