# neoghq

![Build Status](https://github.com/r4ai/neoghq/actions/workflows/ci.yml/badge.svg)
[![Coverage](https://codecov.io/gh/r4ai/neoghq/branch/main/graph/badge.svg)](https://codecov.io/gh/r4ai/neoghq)

A modern Git repository manager with worktree support - an alternative to ghq.

## Features

- **Git Worktree-Based**: Manage multiple branches of the same repository in separate directories
- **Hierarchical CLI**: Intuitive command structure with `repo` and `worktree` subcommands
- **Efficient Workflow**: No need for stash/unstash when switching branches
- **Multiple Host Support**: Works with GitHub, GitLab, Bitbucket, and custom Git hosts
- **Shell Integration**: Commands output paths for easy shell integration

## Installation

```bash
cargo install --git https://github.com/r4ai/neoghq
```

## Usage

### Quick Start

```bash
# Clone a repository
neoghq repo clone https://github.com/user/repo

# Navigate to the repository (outputs path for shell integration)
cd $(neoghq repo switch user/repo)

# Create a new worktree for a feature branch
neoghq worktree create feature/awesome-feature

# Switch to the new worktree
neoghq worktree switch feature/awesome-feature
```

### Complete Command Reference

```bash
# Show help
neoghq help

# Repository operations
neoghq repo clone https://github.com/user/repo       # Clone repository and create default worktree
neoghq repo clone git@github.com:user/repo.git       # Clone with SSH URL
neoghq repo create https://github.com/user/repo      # Create new repository structure
neoghq repo switch user/repo                         # Navigate to repository directory
neoghq repo list                                     # List all managed repositories

# Worktree operations
neoghq worktree list                                 # List all worktrees
neoghq worktree create feature/new-feature           # Create new worktree
neoghq worktree switch feature/new-feature           # Switch to worktree
neoghq worktree remove feature/old-feature           # Remove worktree
neoghq worktree clean                                # Clean merged worktrees
neoghq worktree status                               # Show worktree status

# Utility
neoghq root                                          # Show neoghq root directory
```

### Typical Workflow

```bash
# Clone a repository
neoghq repo clone https://github.com/user/awesome-project

# Navigate to the repository
cd $(neoghq repo switch user/awesome-project)

# Create a new worktree for a feature branch
neoghq worktree create feature/add-authentication

# Switch to the new worktree
cd $(neoghq worktree switch feature/add-authentication)

# Work on your feature...
# When done, remove the worktree
neoghq worktree remove feature/add-authentication

# Clean up merged worktrees
neoghq worktree clean
```

## Directory Structure

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

## Configuration

Configuration file: `~/.config/neoghq/config.toml`

```toml
[general]
root = "~/src/repos"

[git]
default_branch = "main"

[clone]
protocol = "ssh"
```

Environment variables:

- `NEOGHQ_ROOT`: Override the root directory

## Development

```bash
git clone https://github.com/r4ai/neoghq
cd neoghq
cargo build
cargo test
```

## License

MIT License - see [LICENSE](LICENSE) for details.
