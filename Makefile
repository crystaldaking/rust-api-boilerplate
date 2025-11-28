.PHONY: all build run test clean dev watch watch-test migrate install-deps docker-up docker-down docker-build docker-logs

# Default target
all: build

# Install development dependencies
install-deps:
	cargo install cargo-watch
	cargo install sqlx-cli

# Development mode with cargo-watch (auto-restart on changes)
dev: install-deps
	cargo watch -x run

# Watch and run tests on changes
watch-test: install-deps
	cargo watch -x test

# Build the project
build:
	cargo build

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Run database migrations
migrate:
	cargo sqlx migrate run

# Docker commands
docker-up:
	docker-compose up

docker-down:
	docker-compose down

docker-build:
	docker-compose up --build

docker-logs:
	docker-compose logs -f api
