# Alias targets for starting and stopping Docker containers
start: start-docker
stop: stop-docker
shutdown: shutdown-docker
recreate: shutdown start rlog

# Alias targets for migrating the databases
mup: migrate-up

# Development target for running your Rust application with cargo-watch
dev:
	cargo watch -q -c -w ./backend/src/ -x run

# Target for running migrations
migrate-up:
	cd backend && source .env && sqlx migrate run --database-url $$DATABASE_URL
	@echo "\n⬆️\tSQLx migrations run successfully!"

# Target for reverting migrations
mdown:
	cd backend && source .env && sqlx migrate revert --database-url $$DATABASE_URL
	@echo "\n⬇️\tSQLx migrations reverted successfully!"

# Target for building and starting the PostgreSQL container
start-docker:
	cd backend && docker-compose up -d
	@echo "\n\t🐳 Docker containers postgreSQLx and postgresAdmin have been created and started."

# Target for stopping Docker containers
stop-docker:
	cd backend && docker stop postgreSQLx postgresAdmin
	@echo "\n\t🛑 Docker containers postgreSQLx and postgresAdmin have been stopped.\n"

# Target for both stopping and removing Docker containers
shutdown-docker:
	cd backend && docker stop postgreSQLx postgresAdmin
	cd backend && docker rm postgreSQLx postgresAdmin
	@echo "\n\t📛 Docker containers postgreSQLx and postgresAdmin have been stopped and removed.\n"

rlog:
	@echo "\n️🔄🌱  POSTGRES DATABASE RECREATED"

# Declare all targets as phony (no real files associated)
.PHONY: postgres start stop shutdown recreate mup mdown dev