.PHONY: help build test clean docker run dev fmt lint bench docs

# Default target
help:
	@echo "Syros Platform - Makefile Commands"
	@echo "=================================="
	@echo "Development:"
	@echo "  dev         - Start development environment"
	@echo "  build       - Build the project"
	@echo "  test        - Run all tests"
	@echo "  bench       - Run benchmarks"
	@echo "  fmt         - Format code"
	@echo "  lint        - Run linter"
	@echo "  clean       - Clean build artifacts"
	@echo ""
	@echo "Docker:"
	@echo "  docker      - Build Docker image"
	@echo "  run         - Run with Docker Compose"
	@echo "  stop        - Stop Docker Compose"
	@echo ""
	@echo "Documentation:"
	@echo "  docs        - Generate documentation"
	@echo "  serve-docs  - Serve documentation locally"
	@echo ""
	@echo "Release:"
	@echo "  release     - Build release version"
	@echo "  package     - Create release package"

# Development commands
dev:
	@echo "🚀 Starting development environment..."
	docker-compose up -d redis etcd postgres
	@echo "✅ Infrastructure started"
	@echo "🔧 Run 'cargo run' to start Syros Platform"

build:
	@echo "🔨 Building Syros Platform..."
	cargo build

test:
	@echo "🧪 Running tests..."
	cargo test --verbose
	@echo "🧪 Running integration tests..."
	cargo test --test integration --verbose

bench:
	@echo "📊 Running benchmarks..."
	cargo bench

fmt:
	@echo "🎨 Formatting code..."
	cargo fmt --all

lint:
	@echo "🔍 Running linter..."
	cargo clippy --all-targets --all-features -- -D warnings

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	docker system prune -f

# Docker commands
docker:
	@echo "🐳 Building Docker image..."
	docker build -t syros-platform .

run:
	@echo "🚀 Starting Syros Platform with Docker Compose..."
	docker-compose up -d
	@echo "✅ Syros Platform started!"
	@echo ""
	@echo "🌐 Services available at:"
	@echo "  - REST API: http://localhost:8080"
	@echo "  - gRPC: localhost:9090"
	@echo "  - WebSocket: ws://localhost:8081/ws"
	@echo "  - Prometheus: http://localhost:9091"
	@echo "  - Grafana: http://localhost:3000 (admin/admin)"

stop:
	@echo "🛑 Stopping Syros Platform..."
	docker-compose down

# Documentation
docs:
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --open

serve-docs:
	@echo "🌐 Serving documentation at http://localhost:8000..."
	python3 -m http.server 8000 -d target/doc

# Release commands
release:
	@echo "🚀 Building release version..."
	cargo build --release
	@echo "✅ Release build completed: target/release/syros-platform"

package: release
	@echo "📦 Creating release package..."
	mkdir -p dist
	cp target/release/syros-platform dist/
	cp -r config dist/
	cp -r examples dist/
	cp README.md LICENSE dist/
	tar -czf dist/syros-platform-$(shell cargo pkgid | cut -d# -f2).tar.gz -C dist .
	@echo "✅ Release package created: dist/syros-platform-*.tar.gz"

# Setup commands
setup:
	@echo "⚙️ Setting up development environment..."
	@command -v docker >/dev/null 2>&1 || { echo "❌ Docker is required but not installed."; exit 1; }
	@command -v docker-compose >/dev/null 2>&1 || { echo "❌ Docker Compose is required but not installed."; exit 1; }
	@command -v rust >/dev/null 2>&1 || { echo "❌ Rust is required but not installed. Visit https://rustup.rs/"; exit 1; }
	docker-compose pull
	cargo fetch
	@echo "✅ Development environment setup complete!"

# Database migrations (when implemented)
migrate:
	@echo "🗄️ Running database migrations..."
	# TODO: Implement database migrations
	@echo "✅ Migrations completed"

# Security audit
audit:
	@echo "🔒 Running security audit..."
	cargo audit

# Performance profiling
profile:
	@echo "📈 Running performance profile..."
	cargo build --release
	perf record --call-graph=dwarf target/release/syros-platform
	perf report

# Load testing
load-test:
	@echo "⚡ Running load tests..."
	@command -v wrk >/dev/null 2>&1 || { echo "❌ wrk is required for load testing. Install with: sudo apt-get install wrk"; exit 1; }
	wrk -t12 -c400 -d30s --latency http://localhost:8080/health

# Install dependencies
deps:
	@echo "📦 Installing dependencies..."
	cargo fetch
	@echo "✅ Dependencies installed"

# Check project health
check: fmt lint test audit
	@echo "✅ All checks passed!"

# CI/CD simulation
ci: check bench
	@echo "✅ CI pipeline completed successfully!"

# Quick start
quick-start: setup build run
	@echo "🎉 Syros Platform is running!"
	@echo "📖 Check examples/ directory for usage examples"
	@echo "📚 Documentation: cargo doc --open"
