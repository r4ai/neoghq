# neoghq

![Build Status](https://github.com/r4ai/neoghq/actions/workflows/ci.yml/badge.svg)
[![Coverage](https://codecov.io/gh/r4ai/neoghq/branch/main/graph/badge.svg)](https://codecov.io/gh/r4ai/neoghq)

A modern Git repository manager with worktree support - an alternative to ghq.

## 🚀 Features

- **Git Worktree-Based**: Manage multiple branches of the same repository in separate directories
- **Hierarchical CLI**: Intuitive command structure with `repo` and `worktree` subcommands
- **Efficient Workflow**: No need for stash/unstash when switching branches
- **Repository Management**: Clone, list, and manage repositories easily
- **Configuration Support**: TOML-based configuration with environment variable support

## 📦 Installation

```bash
# Build from source (requires Rust)
cargo install --git https://github.com/r4ai/neoghq
```

## 🔧 Usage

### Basic Commands

```bash
# Show help
neoghq help

# Show root directory
neoghq root
```

### Repository Operations

```bash
# Clone a repository
neoghq repo clone https://github.com/user/repo

# List all repositories
neoghq repo list

# Switch to repository directory (TODO)
neoghq repo switch user/repo

# Create new repository (TODO)
neoghq repo create https://github.com/user/new-repo
```

### Worktree Operations

```bash
# List all worktrees
neoghq worktree list

# Create new worktree (TODO)
neoghq worktree create feature/new-feature

# Switch to worktree (TODO)
neoghq worktree switch feature/new-feature

# Remove worktree (TODO)
neoghq worktree remove feature/old-feature

# Clean merged worktrees (TODO)
neoghq worktree clean

# Show worktree status (TODO)
neoghq worktree status
```

## 📁 Directory Structure

neoghq organizes repositories using a structured directory layout:

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

## ⚙️ Configuration

Configuration file location: `~/.config/neoghq/config.toml`

```toml
[general]
root = "~/src/repos"  # neoghq root directory

[git]
default_branch = "main"  # default branch name

[clone]
protocol = "ssh"  # default protocol (ssh/https)
```

You can also use environment variables:
- `NEOGHQ_ROOT`: Override the root directory

## 🏗️ Development Status

### ✅ Implemented
- Hierarchical CLI structure with clap
- Command dispatcher with 100% test coverage
- Repository cloning and listing
- Worktree listing
- Configuration management
- Basic Git operations

### 🚧 In Progress
- Remaining worktree operations (create, switch, remove, clean, status)
- Remaining repository operations (create, switch)

## 🧪 Development

### Prerequisites

- Rust 2024 edition
- Git

### Building

```bash
# Clone the repository
git clone https://github.com/r4ai/neoghq
cd neoghq

# Build
cargo build

# Run tests
cargo test
# or using task runner
task test

# Check coverage
cargo llvm-cov --text
```

### Testing Guidelines

- Use TDD methodology following Red → Green → Refactor cycles
- Maintain 100% code coverage for new functionality
- Use `tempfile` crate for temporary directories in tests
- Follow the test naming pattern: `test_<function>_<scenario>`

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Follow TDD methodology
4. Ensure tests pass and maintain coverage
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 🙏 Acknowledgments

- Inspired by [ghq](https://github.com/x-motemen/ghq) - the original Git repository organizer
- Built with [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [git2](https://github.com/rust-lang/git2-rs) for Git operations