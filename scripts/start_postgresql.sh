#!/usr/bin/env bash

set -eo pipefail

if ! [ -x "$(command -v docker)" ]; then
  >&2 echo "Error: docker is not installed."
  exit 1
fi

if ! [ -x "$(command -v psql)" ]; then
  >&2 echo "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  >&2 echo "Error: sqlx is not installed."
  exit 1
fi

DB_HOST="${POSTGRES_HOST:=localhost}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_USER="${POSTGRES_USER:=admin}"
DB_PASS="${POSTGRES_PASSWORD:=admin}"
DB_NAME="${POSTGRES_DB:=poll}"

if [[ -z "${SKIP_DOCKER}" ]]; then
  CONTAINER_POSTGRESQL=$(docker container ls -a \
    --filter 'name=poll-database-postgresql' \
    --format '{{.ID}}')
  if [[ -n $CONTAINER_POSTGRESQL ]]; then
    echo >&2 "there is a postgresql container already running"

    echo >&2 "killing container..."
    docker kill $CONTAINER_POSTGRESQL

    echo >&2 "removing container..."
    docker rm $CONTAINER_POSTGRESQL
  fi

  docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASS} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d \
  --name "poll-database-postgresql-$(date '+%s')" \
  postgres -N 1000
fi

# Keep pinging postgres until it's ready to accept commands
until PGPASSWORD=$DB_PASS psql -h $DB_HOST -U $DB_USER -p $DB_PORT -d $DB_NAME -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${PG_PORT} - running migrations now!"

# Run migrations
export DATABASE_URL="postgresql://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
