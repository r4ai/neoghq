# neoghq - Git Worktree-Based Repository Manager

## Project Overview

neoghq is a Rust-based repository management tool developed as an alternative to the traditional ghq.
Its main feature is a design that assumes the use of git worktree, enabling efficient management of multiple branches of the same repository.

### Key Differences from ghq

- **Git worktree-based**: Manage multiple branches of the same repository in separate directories
- **Branch-based management**: Operations are based on branches rather than repositories
- **Improved workflow efficiency**: No need for stash/unstash when switching branches

## Technical Specifications

### Basic Design Principles

1. **Directory Structure**

   ```
   $NEOGHQ_ROOT/
   ├── github.com/
   │   └── user/
   │       └── repo/
   │           ├── main/           # main branch worktree
   │           ├── feature-a/      # feature-a branch worktree
   │           └── .git/           # bare repository
   └── gitlab.com/
       └── ...
   ```

2. **Repository Format**: Bare repository + git worktree
3. **Configuration Management**: TOML format configuration files
4. **Concurrent Processing**: Support for simultaneous operations on multiple repositories

### Core Features

#### Command List

- `neoghq get, clone <url> [branch]` - Clone repository and create worktree
- `neoghq list` - List managed worktrees
- `neoghq remove, rm <path>` - Remove worktree
- `neoghq root` - Show neoghq root directory path
- `neoghq create` - Create a new repository and initialize worktree
- `neoghq help` - Show help message

#### Extended Features

- `neoghq switch <repo> <branch>` - Navigate to specified branch worktree
- `neoghq clean` - Automatically remove unnecessary worktrees
- `neoghq status` - List status of all worktrees

## Architecture

### Module Structure

```
src/
├── main.rs             # Entry point
├── cli.rs              # CLI argument parsing
├── commands/
│   ├── get.rs          # Get command implementation
│   ├── list.rs         # List command implementation
│   ├── remove.rs       # Remove command implementation
│   ├── root.rs         # Root command implementation
│   ├── create.rs       # Create command implementation
│   ├── switch.rs       # Switch command implementation
│   ├── clean.rs        # Clean command implementation
│   └── status.rs       # Status command implementation
├── config.rs           # Configuration management
└── error.rs            # Error handling
```

### Dependencies

Key external crates:

- `clap` - CLI argument parsing
- `serde` - Configuration file serialization
- `toml` - Configuration file format support
- `tokio` - Asynchronous processing
- `git2` - Git operations
- `anyhow`, `thiserror` - Error handling

## Development Guidelines

### Test-Driven Development (TDD)

**CRITICAL: All new features MUST be implemented using t_wada's TDD methodology**

1. **TDD Cycle (Red → Green → Refactor)**

   - **Red**: Write a failing test first
   - **Green**: Write minimal code to make the test pass
   - **Refactor**: Improve code while keeping tests green
   - Repeat this cycle for each small increment of functionality

2. **Commit Strategy**

   - Commit frequently at each TDD step
   - Each commit should represent one complete TDD cycle

3. **Implementation Approach**
   - Break features into smallest possible increments
   - Start with the simplest test case
   - Add complexity gradually through multiple TDD cycles
   - Never write production code without a failing test first
   - Always ensure all tests pass before moving to next feature

### Coding Standards

1. **Naming Conventions**

   - Function names: snake_case
   - Structs: PascalCase
   - Constants: SCREAMING_SNAKE_CASE

2. **Error Handling**

   - Use `anyhow::Result<T>`
   - User-facing error messages support localization

3. **Testing**
   - Unit tests: Implemented within each module
   - Integration tests: Implemented in `tests/` directory
   - Actual Git operation tests using temporary directories
   - Follow TDD principles: test first, then implement

### Build & Test Procedures

```bash
# Build
cargo build

# Run tests
cargo test

# Release build
cargo build --release

# Lint
cargo clippy

# Format
cargo fmt

# Generate documentation
cargo doc --open
```

### Debugging

```bash
# Run with debug build
RUST_LOG=debug cargo run -- <command>

# Test in temporary directory
NEOGHQ_ROOT=/tmp/neoghq-test cargo run -- <command>
```

## Configuration File

Default configuration file path: `~/.config/neoghq/config.toml`

```toml
[general]
root = "~/src/repos"  # neoghq root directory

[git]
default_branch = "main"  # default branch name

[clone]
protocol = "ssh"  # default protocol (ssh/https)
```

## Future Development Tasks

1. Implement basic CLI structure
2. Implement Git operations module
3. Implement worktree management functionality
4. Implement configuration file management
5. Enhance test suite
6. Performance optimization
7. Documentation improvements
