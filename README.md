# Precheck

A Rust CLI tool for pre-commit code review and automated commit message generation.

## Features

- **Review**: Run code review checks on staged changes
- **Message**: Generate commit messages from staged changes  
- **Commit**: Run review and commit with generated message

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Review staged changes
precheck review

# Generate commit message
precheck message

# Review and commit
precheck commit
```

## Dependencies

- `clap` - CLI argument parsing
- `git2` - Git repository operations
- `regex` - Pattern matching
- `colored` - Colored terminal output
- `anyhow` - Error handling

## Development

```bash
cargo build
cargo test
```

Requires a Git repository with staged changes to function properly.