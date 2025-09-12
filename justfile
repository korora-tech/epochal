# Epochal Development Tasks

# Show all available commands
default:
    @just --list

# Build the application
build:
    cargo build

# Run the main application
run:
    cargo run -p epochal-app

# Run the application with file watching
watch:
    cargo watch -x "run -p epochal-app"

# Build with Nix
nix-build:
    nix build

# Run all tests
test:
    cargo test

# Run BDD feature tests
test-bdd:
    cargo test --test integration -p bdd-tests

# Run BDD tests headless (for CI)
test-bdd-headless:
    xvfb-run -a cargo test --test integration -p bdd-tests

# Run BDD tests with specific tags
test-bdd-tags TAGS:
    cargo test --test integration -p bdd-tests -- --tags "{{TAGS}}"

# Run smoke tests only
test-smoke:
    cargo test --test integration -p bdd-tests -- --tags "@smoke"

# Run UI tests only
test-ui:
    cargo test --test integration -p bdd-tests -- --tags "@ui"

# Format all code
fmt:
    cargo fmt --all

# Run clippy
clippy:
    cargo clippy --all-targets -- -D warnings

# Check formatting and linting
check: fmt clippy
    cargo check

# Clean build artifacts
clean:
    cargo clean

# Enter development shell
dev:
    nix develop

# Compile Blueprint files manually
compile-blueprints:
    find . -name "*.blp" -exec sh -c 'blueprint-compiler compile --output "${1%.blp}.ui" "$1"' _ {} \;

# Run the application with GTK Inspector
debug:
    GTK_DEBUG=interactive cargo run -p epochal-app