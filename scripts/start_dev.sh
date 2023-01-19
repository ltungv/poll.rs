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

if [[ -z "${SKIP_DOCKER}" ]]; then
  # Reset the cluster
  docker compose down && docker compose up -d
fi

# Keep pinging postgres until it's ready to accept commands
until PGPASSWORD=admin psql -h localhost -U admin -p 5432 -d poll -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${PG_PORT} - running migrations now!"

# Run migrations
export DATABASE_URL=postgresql://admin:admin@localhost:5432/poll
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
