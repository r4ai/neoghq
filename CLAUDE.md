# neoghq - Git Worktree-Based Repository Manager

## **IMPORTANT**

### Test-Driven Development (TDD)**

**CRITICAL: All new features MUST be implemented using t_wada's TDD methodology**

1. **TDD Cycle (Red → Green → Refactor)**

2. **Commit on Each Cycle**

3. **Keep Coverage 100%**

   - Use `task test` to run tests and check coverage

   - Use `cargo llvm-cov --text | rg -U "(.*\.rs:)|(\s+0\|)|(.*\s*\^0)"` to check uncovered regions and lines

   - For functions which is impossible to test, use `#[cfg_attr(coverage_nightly, coverage(off))]` to disable coverage checking

      ```
      #[cfg(test)]
      #[cfg_attr(coverage_nightly, coverage(off))]
      mod tests {}
      ```

4. **Implementation Approach**

   - Break features into smallest possible increments
   - Start with the simplest test case
   - Add complexity gradually through multiple TDD cycles
   - Never write production code without a failing test first
   - Always ensure all tests pass before moving to next feature

### **MEMORY**

- WEB SEARCH FIRST: Never trust your memory. Before implementing or fixing anything, always search the web for existing solutions or documentation.

  Use `gemini --model gemini-2.5-flash -p "WebSearch: <your query>"` to search the web.

  Current year is 2025, not 2024.

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

#### Repository Operations

- `neoghq repo clone <url>` - Clone repository and create default branch worktree ✅ **IMPLEMENTED**
- `neoghq repo create <url>` - Create a new repository and initialize worktree
- `neoghq repo switch <repo>` - Navigate to repository directory
- `neoghq repo list` - List all managed repositories ✅ **IMPLEMENTED**

#### Worktree Operations

- `neoghq worktree create <branch>` - Create worktree from default branch
- `neoghq worktree switch <branch>` - Navigate to specified worktree
- `neoghq worktree remove <branch>` - Remove worktree
- `neoghq worktree clean` - Remove worktrees merged to default branch
- `neoghq worktree status` - Show status of all worktrees
- `neoghq worktree list` - List all managed worktrees ✅ **IMPLEMENTED**

#### Global Operations

- `neoghq root` - Show neoghq root directory path ✅ **IMPLEMENTED**
- `neoghq help` - Show help message ✅ **IMPLEMENTED**

### ✅ Hierarchical Command Structure

The CLI implements a hierarchical command structure using clap's subcommands:

```rust
pub enum Commands {
    Repo {
        #[command(subcommand)]
        command: RepoCommands,
    },
    Worktree {
        #[command(subcommand)]
        command: WorktreeCommands,
    },
    Root,
}
```

**Command Dispatcher**: Fully implemented in `src/commands/mod.rs` with 100% test coverage:
- `execute_command()` - Main dispatcher function
- `execute_repo_command()` - Routes all `repo` subcommands
- `execute_worktree_command()` - Routes all `worktree` subcommands

### Typical Workflow

```bash
# Clone a repository
neoghq repo clone https://github.com/r4ai/readme

# Navigate to the repository
neoghq repo switch r4ai/readme

# Create a new worktree for a feature branch
neoghq worktree create feature/ci

# Switch to the new worktree
neoghq worktree switch feature/ci

# After work is done, remove the worktree
neoghq worktree remove feature/ci

# Clean up merged worktrees
neoghq worktree clean

# List all repositories and worktrees
neoghq repo list
neoghq worktree list
```

## Architecture

### Module Structure

```
src/
├── main.rs             # Entry point
├── cli.rs              # CLI argument parsing
├── commands/
│   ├── repo/
│   │   ├── clone.rs    # Clone command implementation
│   │   ├── create.rs   # Create command implementation
│   │   ├── switch.rs   # Repo switch command implementation
│   │   ├── list.rs     # Repo list command implementation
│   │   └── mod.rs      # Repo commands module
│   ├── worktree/
│   │   ├── create.rs   # Worktree create command implementation
│   │   ├── switch.rs   # Worktree switch command implementation
│   │   ├── remove.rs   # Remove command implementation
│   │   ├── clean.rs    # Clean command implementation
│   │   ├── status.rs   # Status command implementation
│   │   ├── list.rs     # Worktree list command implementation
│   │   └── mod.rs      # Worktree commands module
│   ├── root.rs         # Root command implementation
│   └── mod.rs          # Commands module
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

### Coding Standards

1. **Error Handling**

   - Use `anyhow::Result<T>`
   - User-facing error messages support localization

2. **Testing**
   - Unit tests: Implemented within each module using `#[cfg(test)]`
   - Integration tests: Implemented in `tests/` directory
   - **Test Requirements**:
     - Use `tempfile` crate for temporary directories/files in tests
     - Achieve 100% code coverage for all new functionality
     - Test both success and error paths
     - Use descriptive test names following `test_<function>_<scenario>` pattern

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

## Development Status

### ✅ Completed Features

1. **Basic CLI structure** - Hierarchical command structure with clap
2. **Command dispatcher** - Routes commands to appropriate handlers with 100% test coverage
3. **Repository operations** - Clone and list commands implemented
4. **Worktree operations** - List command implemented
5. **Configuration management** - TOML-based configuration with environment support
6. **Core test suite** - 60+ tests with comprehensive coverage

### 🚧 In Progress

1. **Worktree management functionality** - Create, switch, remove, clean, status commands
2. **Repository management** - Create and switch commands

### 📋 Future Development Tasks

1. Implement remaining worktree operations
2. Implement remaining repository operations
3. Performance optimization
4. Enhanced error handling and user experience
5. Documentation improvements
6. Shell integration and completion
