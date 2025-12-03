.PHONY: test test-all test-unit test-repo test-service test-api db-start db-clean db-stop build run clean

# =============================================================================
# Testing
# =============================================================================

# Run all tests (unit + integration)
test-all: db-start
	cargo test --workspace
	@$(MAKE) db-clean

# Run unit tests only (no DB required) - fast!
test-unit:
	cargo test --package domain
	cargo test --package shared

# Run repository tests (infrastructure)
test-repo: db-start
	cargo test --package infrastructure
	@$(MAKE) db-clean

# Run service layer tests (application)
test-service: db-start
	cargo test --package application
	@$(MAKE) db-clean

# Run API tests (http-server)
test-api: db-start
	cargo test --package http-server
	@$(MAKE) db-clean

# Default: run all DB tests
test: db-start
	cargo test --package infrastructure
	cargo test --package application
	cargo test --package http-server
	@$(MAKE) db-clean

# =============================================================================
# Database Management
# =============================================================================

# Start database container
db-start:
	./scripts/test-infrastructure.sh start

# Clean test databases
db-clean:
	./scripts/test-infrastructure.sh clean

# Stop database container
db-stop:
	./scripts/test-infrastructure.sh stop

# =============================================================================
# Development
# =============================================================================

# Build the project
build:
	cargo build

# Run the server
run:
	cargo run --package http-server

# Clean build artifacts
clean:
	cargo clean

# Check code
check:
	cargo check --workspace

# Format code
fmt:
	cargo fmt --all

# Lint
lint:
	cargo clippy --workspace
