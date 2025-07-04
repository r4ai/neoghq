# neoghq

![Build Status](https://github.com/r4ai/neoghq/actions/workflows/ci.yml/badge.svg)
[![Coverage](https://codecov.io/gh/r4ai/neoghq/branch/main/graph/badge.svg)](https://codecov.io/gh/r4ai/neoghq)

A modern Git repository manager with worktree support - an alternative to ghq.

## Features

- **Git Worktree-Based**: Manage multiple branches of the same repository in separate directories
- **Hierarchical CLI**: Intuitive command structure with `repo` and `worktree` subcommands
- **Efficient Workflow**: No need for stash/unstash when switching branches

## Installation

```bash
cargo install --git https://github.com/r4ai/neoghq
```

## Usage

```bash
# Show help
neoghq help

# Repository operations
neoghq repo clone https://github.com/user/repo
neoghq repo list

# Worktree operations
neoghq worktree list
neoghq worktree create feature/new-feature
neoghq worktree switch feature/new-feature
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