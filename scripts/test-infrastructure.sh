#!/bin/bash
# Infrastructure test helper
#
# Usage:
#   ./scripts/test-infrastructure.sh           # Run tests (starts container if needed)
#   ./scripts/test-infrastructure.sh start     # Start container only
#   ./scripts/test-infrastructure.sh clean     # Clean test databases
#   ./scripts/test-infrastructure.sh stop      # Stop container

set -e

export DATABASE_URL="${DATABASE_URL:-postgres://postgres:postgres@localhost:5432/postgres}"

start_db() {
    echo "🐘 Starting PostgreSQL container..."
    docker-compose up -d postgres
    echo "⏳ Waiting for PostgreSQL..."
    until pg_isready -h localhost -p 5432 -U postgres > /dev/null 2>&1; do
        sleep 1
    done
    echo "✅ PostgreSQL ready!"
}

clean_db() {
    echo "🧹 Cleaning up test databases..."
    psql "$DATABASE_URL" -t -c "SELECT 'DROP DATABASE \"' || datname || '\";' FROM pg_database WHERE datname LIKE '_sqlx_test_%';" | psql "$DATABASE_URL" 2>/dev/null || true
    echo "✅ Test databases cleaned!"
}

stop_db() {
    echo "🛑 Stopping PostgreSQL container..."
    docker-compose down 2>/dev/null || true
    echo "✅ Container stopped!"
}

run_tests() {
    # Start if not running
    if ! pg_isready -h localhost -p 5432 -U postgres > /dev/null 2>&1; then
        start_db
    fi
    
    echo "🧪 Running infrastructure tests..."
    cargo test --package infrastructure
    echo "✅ All tests passed!"
}

# Handle commands
case "${1:-test}" in
    start)
        start_db
        ;;
    clean)
        clean_db
        ;;
    stop)
        stop_db
        ;;
    test|"")
        run_tests
        ;;
    *)
        echo "Usage: $0 {start|test|clean|stop}"
        exit 1
        ;;
esac
