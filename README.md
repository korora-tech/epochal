# Epochal

A modern GTK4 Rust application built with Blueprint UI, managed by Nix flake with Crane.

## Quick Start

```bash
# Enter development environment
nix develop

# Build and run
cargo run -p epochal-app

# Or build with Nix
nix build
```

## Development

The project uses:
- **Nix flake** with **Crane** for reproducible builds
- **GTK4** with **libadwaita** for modern UI
- **Blueprint UI** for declarative interface design
- **Cargo workspace** for modular architecture

### Project Structure

```
epochal/
├── flake.nix                    # Nix flake configuration
├── Cargo.toml                   # Workspace configuration with BDD tests
├── features/                    # BDD Feature files (Gherkin)
│   ├── app_lifecycle.feature    # Application startup/shutdown tests
│   ├── system_tray.feature      # System tray integration tests
│   ├── ui_interactions.feature  # UI component tests
│   └── user_workflows.feature   # End-to-end workflow tests
├── tests/
│   ├── behaviours.rs            # BDD test runner
│   └── behaviours/              # BDD test implementation
│       ├── common/              # Test utilities and World pattern
│       └── steps/               # Step definitions for each domain
├── crates/
│   └── epochal-app/              # Main GTK4 application
│       ├── src/main.rs          # Application entry point
│       ├── build.rs             # Build script
│       └── ui/                  # Blueprint UI files
│           └── window.blp       # Main window layout
└── data/
    ├── resources.gresource.xml  # Resource bundle
    └── com.korora.Epochal.desktop # Desktop entry
```

### Development Commands

```bash
# Auto-rebuild on changes
cargo watch -x "run -p epochal-app"

# Format code
cargo fmt

# Check with clippy
cargo clippy

# Build with Nix (for distribution)
nix build

# Check formatting and clippy with Nix
nix flake check
```

### BDD Testing

The project uses Cucumber-rs for Behavior-Driven Development testing:

```bash
# Run all BDD tests
just test-bdd
cargo test --test behaviours

# Run tests headless (for CI)
just test-bdd-headless
xvfb-run cargo test --test behaviours

# Run specific test categories
just test-smoke          # @smoke tagged scenarios
just test-ui             # @ui tagged scenarios

# Run tests with specific tags
just test-bdd-tags "@integration"
```

**Test Categories:**
- `@smoke` - Essential functionality tests
- `@ui` - User interface component tests
- `@integration` - System integration tests
- `@headless` - Tests that run without display
- `@tray` - System tray functionality tests

## Blueprint UI

Blueprint files (`*.blp`) are automatically compiled to GTK UI files (`*.ui`) during build.
The GTK Inspector is available during development with `Ctrl+Shift+I`.

## Features

- Modern GTK4/Adwaita interface
- Responsive layout with breakpoints
- Sidebar navigation
- Blueprint UI compilation
- GResource bundling
- Desktop integration
- Nix reproducible builds