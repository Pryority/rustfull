DOWN_VERSION = 1
TABLES = products blog_posts images users schema_migrations
# Define the PostgreSQL container name and port
POSTGRES_CONTAINER = postgres-rs
POSTGRES_PORT = 2345

# Define PostgreSQL environment variables
POSTGRES_USER = root
POSTGRES_PASSWORD = secret
POSTGRES_DB = postgres-rs

DATABASE_URL = "postgres://root:secret@localhost:2345/postgres-rs?sslmode=disable"
MIGRATION_DIR = ./migrations

postgres:
	docker run --name $(POSTGRES_CONTAINER) -p $(POSTGRES_PORT):5432 -e POSTGRES_USER=$(POSTGRES_USER) -e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) -e POSTGRES_DB=$(POSTGRES_DB) -d postgres:alpine

createdb:
	docker exec -it $(POSTGRES_CONTAINER) createdb --username=$(POSTGRES_USER) --owner=$(POSTGRES_USER) $(POSTGRES_DB)

mup:
	sqlx migrate run --database-url postgres://root:secret@localhost:2345/postgres-rs?sslmode=disable

mdown:
	sqlx migrate revert $(DATABASE_URL)

dropdb:
	docker exec -it postgres-rs psql -U root -d postgres -c "SELECT pg_terminate_backend (pg_stat_activity.pid) FROM pg_stat_activity WHERE pg_stat_activity.datname = 'postgres-rs';"
	docker exec -it postgres-rs dropdb --username=root postgres-rs 

droptables:
	docker exec -it postgres-rs psql -U root -d postgres-rs -c "$(foreach table,$(TABLES),DROP TABLE IF EXISTS $(table);)"

sqlx:
	sqlx generate

install:
	cd web && npm install

dev:
	air

test:
	go test -v ./cmd/server/db/sqlc

start-docker:
	docker build -t cambia .
	docker run -d --name cambia-app -p 8080:8080 cambia

stop-docker:
	docker stop cambia-app
	docker rm cambia-app

start-live-reload:
	air

start: start-docker start-live-reload

restart: dropdb createdb mup

stop: stop-docker

.PHONY: postgres createdb dropdb install dev start stop droptables mup mdown sqlx restart