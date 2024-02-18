#!/usr/bin/env bash
set -x 
set -eo pipefail

# Preconditions
if ! [ -x "$(command -v nc)" ]; then
    echo >&2 "Error: netcat not available"
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx not available"
    exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=newsletter}
DB_PORT=${POSTGRES_PORT:=5432}
DB_HOST=${POSTGRES_HOST:=localhost}

if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run -e POSTGRES_USER=${DB_USER} -e POSTGRES_PASSWORD=${DB_PASSWORD} \
         -e POSTGRES_DB=${DB_NAME} -p "$DB_PORT":5432 -d postgres postgres -N 1000
fi 

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL

until nc -z "${DB_HOST}" "${DB_PORT}"; do
    >&2 echo "Postgres still unavailable - sleeping"
    sleep 1
done

echo "Postgres started on port ${DB_PORT}"

sqlx database create
sqlx migrate run
