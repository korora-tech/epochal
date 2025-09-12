# GitHub Actions CI/CD

This repository uses GitHub Actions for continuous integration and deployment.

## Workflows

### CI (`ci.yml`)
Main CI pipeline that runs on every push and pull request:
- **Formatting check**: Ensures code follows Rust formatting standards
- **Clippy**: Runs Rust linter to catch common mistakes
- **Build**: Compiles the project
- **Unit tests**: Runs unit tests
- **BDD tests**: Runs behavior-driven development tests with xvfb (headless)
- **Release build**: Creates optimized release builds on main branch

### Cross-Platform Build (`cross-platform.yml`)
Tests builds on multiple platforms:
- Linux (Ubuntu)
- macOS
- Windows (commented out - needs GTK4 setup)

### BDD Tests (`bdd-tests.yml`)
Specialized workflow for BDD tests using Nix environment:
- Uses Nix for reproducible builds
- Runs Cucumber tests with xvfb
- Generates JUnit XML reports
- Uploads test results as artifacts

### PR Labeler (`pr-labeler.yml`)
Automatically labels pull requests based on changed files:
- `documentation`: Changes to markdown files
- `tests`: Changes to test files
- `ci`: Changes to CI/CD configuration
- `app`: Changes to main application
- `dependencies`: Changes to dependencies
- `blueprint`: Changes to Blueprint UI files

## Dependabot

Configured to automatically create PRs for:
- Rust dependencies (weekly)
- GitHub Actions updates (weekly)

## Required Secrets

No additional secrets required beyond the default `GITHUB_TOKEN`.

## Running Tests Locally

To run the same tests that run in CI:

```bash
# Unit tests
cargo test --lib

# BDD tests (requires xvfb)
xvfb-run -a cargo test --test integration -p bdd-tests

# With Nix
nix develop --command just test-bdd-headless
```

## Caching

All workflows use GitHub Actions caching for:
- Cargo registry
- Cargo git dependencies
- Build artifacts

This significantly speeds up build times after the first run.