# git-gardener

[🇯🇵 日本語版](README_ja.md)

A powerful Git worktree management tool that simplifies parallel development workflows.

## Overview

`git-gardener` makes Git worktree management effortless. Whether you're working on multiple features simultaneously, testing different branches, or maintaining parallel development environments, git-gardener streamlines the entire workflow.

## Features

- **Easy Worktree Creation**: Create worktrees with minimal commands
- **Intelligent Cleanup**: Automatically detect and remove merged or stale worktrees
- **Parallel Operations**: Pull all worktrees simultaneously with parallel processing
- **Interactive TUI**: Full-featured terminal interface with visual worktree management
- **Editor Integration**: Open worktrees in your preferred editor with one keystroke
- **Flexible Configuration**: Per-repository customization via TOML config files
- **Smart Detection**: Automatically identify merged branches and old commits

## Installation

### With Nix (Recommended)

Using Nix flakes for reproducible builds:

```bash
# Install directly from the repository
nix profile install github:mei28/git-gardener

# Or clone and build locally
git clone https://github.com/mei28/git-gardener.git
cd git-gardener
nix build
./result/bin/git-gardener --help
```

### Development with Nix

```bash
# Enter development shell with all dependencies
nix develop

# Or use direnv for automatic environment loading
echo "use flake" > .envrc
direnv allow
```

### From Source

```bash
git clone https://github.com/your-username/git-gardener.git
cd git-gardener
cargo build --release
cp target/release/git-gardener /usr/local/bin/
```

### Prerequisites

- Nix (recommended) with flakes enabled, or
- Rust 1.75 or later
- Git

## Quick Start

1. **Initialize git-gardener in your repository:**
   ```bash
   git-gardener init
   ```

2. **Create a new worktree:**
   ```bash
   git-gardener add -b feature/new-feature
   ```

3. **List all worktrees:**
   ```bash
   git-gardener list
   ```

4. **Pull all worktrees in parallel:**
   ```bash
   git-gardener pull-all
   ```

5. **Clean up merged worktrees:**
   ```bash
   git-gardener clean --merged
   ```

## Commands

### `git-gardener add`
Create a new worktree for the specified branch.

```bash
# Create worktree for existing branch
git-gardener add -b feature/auth

# Create worktree with new branch
git-gardener add -b feature/new-feature --create-branch

# Specify custom path
git-gardener add -b hotfix/urgent --path ../hotfix-urgent
```

**Options:**
- `-b, --branch <BRANCH>`: Branch name (required)
- `-p, --path <PATH>`: Custom worktree path (default: `.gardener/<branch>`)
- `-c, --create-branch`: Create a new branch
- `--upstream <UPSTREAM>`: Set upstream remote

### `git-gardener list`
Display all worktrees with their status information.

```bash
git-gardener list
git-gardener list --all  # Include pruned worktrees
```

### `git-gardener clean`
Remove worktrees based on specified conditions.

```bash
# Remove merged worktrees
git-gardener clean --merged

# Remove worktrees with no commits in last 30 days
git-gardener clean --stale 30

# Force remove all worktrees (dangerous!)
git-gardener clean --force
```

**Options:**
- `--merged`: Remove worktrees for merged branches
- `--stale <DAYS>`: Remove worktrees older than N days
- `--force`: Force removal of all worktrees

### `git-gardener pull-all`
Pull latest changes for all worktrees simultaneously.

```bash
git-gardener pull-all
git-gardener pull-all --parallel 8  # Use 8 parallel jobs
```

**Options:**
- `--parallel <N>`: Number of parallel jobs (default: CPU cores)

### `git-gardener config`
View or modify configuration settings.

```bash
# View current configuration
git-gardener config view

# Set configuration values
git-gardener config set defaults.root_dir .worktrees
git-gardener config set defaults.editor "code"
```

### `git-gardener tui`
Launch the interactive terminal interface for visual worktree management.

```bash
git-gardener tui
```

**TUI Features:**
- **Visual Navigation**: Navigate worktrees with `j/k` or arrow keys
- **Quick Actions**: 
  - `a` - Create new worktree with branch input
  - `d` - Delete selected worktree (with confirmation)
  - `p` - Pull latest changes for selected worktree
  - `c` - Clean worktrees (choose merged/stale options)
  - `Enter` - Open worktree in configured editor
- **Real-time Status**: See worktree status (Clean, Dirty, Ahead, Behind, Diverged)
- **Smart Cleanup**: Interactive selection of cleanup criteria

### `git-gardener init`
Initialize git-gardener configuration file.

```bash
git-gardener init
git-gardener init --force  # Overwrite existing config
```

## Configuration

git-gardener uses a `.git/gardener.toml` configuration file for per-repository settings.

### Example Configuration

```toml
[defaults]
# Root directory for worktrees
root_dir = ".gardener"

# Commands to run after creating a worktree
post_create = [
    "cp .env.example ${WORKTREE_PATH}/.env",
    "npm install"
]

# Default editor command
editor = "code ${WORKTREE_PATH}"

[branches]
# Branch-specific configurations can be added here
```

### Configuration Options

- `defaults.root_dir`: Default directory for creating worktrees
- `defaults.post_create`: Shell commands executed after worktree creation
- `defaults.editor`: Editor command for opening worktrees

## Use Cases

### Parallel Feature Development
```bash
# Set up multiple feature branches
git-gardener add -b feature/auth --create-branch
git-gardener add -b feature/payment --create-branch
git-gardener add -b feature/dashboard --create-branch

# Work on different features simultaneously
# Each worktree is isolated with its own working directory

# Or use the TUI for visual management
git-gardener tui
# Press 'a' to create new worktrees interactively
```

### Release Management
```bash
# Maintain separate environments for different versions
git-gardener add -b release/v1.2 --path ../release-v1.2
git-gardener add -b hotfix/security --path ../hotfix
git-gardener add -b develop --path ../develop

# Keep all environments updated
git-gardener pull-all

# Or use TUI to manage and update all worktrees visually
git-gardener tui
# Press 'p' on each worktree or use pull-all
```

### Cleanup Workflow
```bash
# Regular maintenance
git-gardener clean --merged  # Remove merged feature branches
git-gardener clean --stale 7  # Remove branches inactive for a week

# Interactive cleanup with TUI
git-gardener tui
# Press 'c' to interactively select cleanup options
# Choose 'merged' and/or 'stale' criteria
```

### Editor Integration Workflow
```bash
# Configure your preferred editor
git-gardener config set defaults.editor "code ${WORKTREE_PATH}"

# Use TUI to quickly open worktrees
git-gardener tui
# Navigate to any worktree and press Enter to open in editor
```

## Development

### Building from Source

```bash
git clone https://github.com/your-username/git-gardener.git
cd git-gardener
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Project Structure

```
git-gardener/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # CLI command definitions
│   ├── commands/        # Command implementations
│   ├── git/             # Git operations
│   ├── config.rs        # Configuration handling
│   └── error.rs         # Error types
├── tests/               # Integration tests
└── docs/                # Documentation
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-feature`
3. Make your changes and add tests
4. Run tests: `cargo test`
5. Run linting: `cargo clippy`
6. Format code: `cargo fmt`
7. Commit your changes: `git commit -m 'Add new feature'`
8. Push to the branch: `git push origin feature/new-feature`
9. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI powered by [clap](https://github.com/clap-rs/clap)
- TUI built with [ratatui](https://github.com/ratatui-org/ratatui)
- Git operations via [git2](https://github.com/rust-lang/git2-rs)