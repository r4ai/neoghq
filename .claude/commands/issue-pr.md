FOLLOW t-wada style TDD: RED -> GREEN -> REFACTOR.
COMMIT at EACH phase. ALWAYS.

Create a PR with the implementation for the following issue.
Ensure the PR is linked to the issue. Use the gh CLI for all GitHub operations.

## Instructions for neoghq project:

1. Always maintain 100% test coverage
2. Use `task test` to run tests and check coverage
3. Use `cargo llvm-cov --text | rg -U "(.*\.rs:)|(\s+0\|)|(.*\s*\^0)"` to check uncovered regions
4. Break features into smallest possible increments
5. Add comprehensive error handling with user-friendly messages
6. Follow the hierarchical command structure (repo/worktree subcommands)
7. Add unit tests for all functions, integration tests for commands
8. Update README.md with new functionality

$ARGUMENTS
