# Alias targets for starting and stopping Docker containers
start: start-docker
stop: stop-docker

# Alias targets for migrating the databases
mup: migrate-up

# Alias target that stops & removes the database containers, 
# then creates and starts them up again
restart: stop start rlog

rlog:
	@echo "\n️🔄🌱  POSTGRES DATABASE RESTARTED"

# Development target for running your Rust application with cargo-watch
dev:
	cargo watch -q -c -w src/ -x run

# Target for running migrations
migrate-up:
	@source .env && sqlx migrate run --database-url $$DATABASE_URL
	@echo "\n⬆️\tSQLx migrations run successfully!"

# Target for reverting migrations
mdown:
	@source .env && sqlx migrate revert --database-url $$DATABASE_URL
	@echo "\n⬇️\tSQLx migrations reverted successfully!"

# Target for building and starting the PostgreSQL container
start-docker:
	docker-compose up -d
	@echo "\n\t🐳 Docker containers postgreSQLx and postgresAdmin have been created and started."

# Target for stopping and removing Docker containers
stop-docker:
	docker stop postgreSQLx postgresAdmin
	docker rm postgreSQLx postgresAdmin
	@echo "\n\t📛 Docker containers postgreSQLx and postgresAdmin have been stopped and removed.\n"

# Declare all targets as phony (no real files associated)
.PHONY: postgres start stop restart mup mdown dev