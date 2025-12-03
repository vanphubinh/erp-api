.PHONY: test test-db db-start db-clean db-stop build run clean

# Run all tests
test:
	cargo test

# Run infrastructure (database) tests
test-db:
	./scripts/test-infrastructure.sh

# Start database container
db-start:
	./scripts/test-infrastructure.sh start

# Clean test databases
db-clean:
	./scripts/test-infrastructure.sh clean

# Stop database container
db-stop:
	./scripts/test-infrastructure.sh stop

# Build the project
build:
	cargo build

# Run the server
run:
	cargo run --package http-server

# Clean build artifacts
clean:
	cargo clean
