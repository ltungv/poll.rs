#!/usr/bin/env bash

set -eo pipefail

if ! [ -x "$(command -v docker)" ]; then
  >&2 echo "Error: docker is not installed."
  exit 1
fi

if ! [ -x "$(command -v mysql)" ]; then
  >&2 echo "Error: mysql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  >&2 echo "Error: sqlx is not installed."
  exit 1
fi

DB_HOST="${MYSQL_HOST:=0.0.0.0}"
DB_PORT="${MYSQL_PORT:=3306}"
DB_USER="${MYSQL_USER:=admin}"
DB_PASS="${MYSQL_PASSWORD:=admin}"
DB_NAME="${MYSQL_DB:=poll}"
DB_ROOT_PASS="${MYSQL_ROOT_PASSWORD:=root}"

if [[ -z "${SKIP_DOCKER}" ]]; then
  CONTAINER_MYSQL=$(docker container ls -a \
    --filter 'name=poll-database-mysql' \
    --format '{{.ID}}')
  if [[ -n $CONTAINER_MYSQL ]]; then
    echo >&2 "there is a mysql container already running"
    echo >&2 "removing container..."
    docker kill $CONTAINER_MYSQL || docker rm $CONTAINER_MYSQL
  fi

  docker run \
  -e MYSQL_USER=${DB_USER} \
  -e MYSQL_PASSWORD=${DB_PASS} \
  -e MYSQL_ROOT_PASSWORD=${DB_ROOT_PASS} \
  -e MYSQL_DATABASE=${DB_NAME} \
  -p "${DB_PORT}":3306 \
  -d \
  --name "poll-database-mysql-$(date '+%s')" \
  mysql
fi

# Keep pinging mysql until it's ready to accept commands
until mysql -h $DB_HOST -P $DB_PORT -D $DB_NAME -u $DB_USER -p$DB_PASS -e 'quit'; do
  >&2 echo "MySQL is still unavailable - sleeping"
  sleep 1
done

>&2 echo "MySQL is up and running on port ${PG_PORT} - running migrations now!"

# Run migrations
export DATABASE_URL="mysql://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run

>&2 echo "MySQL has been migrated, ready to go!"
