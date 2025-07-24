#!/bin/bash

# Install shell completions for git-gardener

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPLETIONS_DIR="$(dirname "$SCRIPT_DIR")/completions"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect shell
detect_shell() {
    if [[ -n "$ZSH_VERSION" ]]; then
        echo "zsh"
    elif [[ -n "$BASH_VERSION" ]]; then
        echo "bash"
    elif [[ -n "$FISH_VERSION" ]]; then
        echo "fish"
    else
        echo "unknown"
    fi
}

# Install Bash completion
install_bash_completion() {
    local completion_dirs=(
        "/usr/local/etc/bash_completion.d"
        "/etc/bash_completion.d"
        "$HOME/.local/share/bash-completion/completions"
        "$HOME/.bash_completion.d"
    )
    
    for dir in "${completion_dirs[@]}"; do
        if [[ -d "$dir" ]]; then
            log_info "Installing Bash completion to $dir"
            cp "$COMPLETIONS_DIR/git-gardener.bash" "$dir/git-gardener"
            log_info "Bash completion installed successfully"
            log_info "Restart your shell or run: source $dir/git-gardener"
            return 0
        fi
    done
    
    # Create user completion directory if none exists
    mkdir -p "$HOME/.bash_completion.d"
    cp "$COMPLETIONS_DIR/git-gardener.bash" "$HOME/.bash_completion.d/git-gardener"
    log_info "Bash completion installed to $HOME/.bash_completion.d/git-gardener"
    log_warn "Add the following line to your ~/.bashrc:"
    echo "source ~/.bash_completion.d/git-gardener"
}

# Install Zsh completion
install_zsh_completion() {
    local completion_dirs=(
        "/usr/local/share/zsh/site-functions"
        "/usr/share/zsh/site-functions"
        "$HOME/.local/share/zsh/site-functions"
    )
    
    # Check if we're in Oh My Zsh
    if [[ -n "$ZSH" && -d "$ZSH/completions" ]]; then
        completion_dirs=("$ZSH/completions" "${completion_dirs[@]}")
    fi
    
    for dir in "${completion_dirs[@]}"; do
        if [[ -d "$dir" ]]; then
            log_info "Installing Zsh completion to $dir"
            cp "$COMPLETIONS_DIR/git-gardener.zsh" "$dir/_git-gardener"
            log_info "Zsh completion installed successfully"
            log_info "Restart your shell or run: autoload -U compinit && compinit"
            return 0
        fi
    done
    
    # Create user completion directory if none exists
    mkdir -p "$HOME/.local/share/zsh/site-functions"
    cp "$COMPLETIONS_DIR/git-gardener.zsh" "$HOME/.local/share/zsh/site-functions/_git-gardener"
    log_info "Zsh completion installed to $HOME/.local/share/zsh/site-functions/_git-gardener"
    log_warn "Add the following lines to your ~/.zshrc:"
    echo "fpath=(~/.local/share/zsh/site-functions \$fpath)"
    echo "autoload -U compinit && compinit"
}

# Install Fish completion
install_fish_completion() {
    local completion_dirs=(
        "$HOME/.config/fish/completions"
        "/usr/local/share/fish/completions"
        "/usr/share/fish/completions"
    )
    
    for dir in "${completion_dirs[@]}"; do
        if [[ -d "$dir" ]]; then
            log_info "Installing Fish completion to $dir"
            cp "$COMPLETIONS_DIR/git-gardener.fish" "$dir/git-gardener.fish"
            log_info "Fish completion installed successfully"
            log_info "Restart your shell or run: fish_complete_path"
            return 0
        fi
    done
    
    # Create user completion directory if none exists
    mkdir -p "$HOME/.config/fish/completions"
    cp "$COMPLETIONS_DIR/git-gardener.fish" "$HOME/.config/fish/completions/git-gardener.fish"
    log_info "Fish completion installed to $HOME/.config/fish/completions/git-gardener.fish"
}

# Main installation logic
main() {
    local shell_type=""
    local force_shell=""
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --bash)
                force_shell="bash"
                shift
                ;;
            --zsh)
                force_shell="zsh"
                shift
                ;;
            --fish)
                force_shell="fish"
                shift
                ;;
            --all)
                force_shell="all"
                shift
                ;;
            -h|--help)
                echo "Usage: $0 [--bash|--zsh|--fish|--all]"
                echo ""
                echo "Install shell completions for git-gardener"
                echo ""
                echo "Options:"
                echo "  --bash    Install Bash completion only"
                echo "  --zsh     Install Zsh completion only"
                echo "  --fish    Install Fish completion only"
                echo "  --all     Install completions for all shells"
                echo "  -h, --help Show this help message"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    if [[ "$force_shell" == "all" ]]; then
        log_info "Installing completions for all shells"
        install_bash_completion
        install_zsh_completion
        install_fish_completion
        return 0
    fi
    
    if [[ -n "$force_shell" ]]; then
        shell_type="$force_shell"
    else
        shell_type="$(detect_shell)"
    fi
    
    case "$shell_type" in
        bash)
            install_bash_completion
            ;;
        zsh)
            install_zsh_completion
            ;;
        fish)
            install_fish_completion
            ;;
        unknown)
            log_warn "Could not detect shell. Please specify --bash, --zsh, or --fish"
            log_info "Or use --all to install completions for all shells"
            exit 1
            ;;
        *)
            log_error "Unsupported shell: $shell_type"
            exit 1
            ;;
    esac
}

# Check if completion files exist
if [[ ! -f "$COMPLETIONS_DIR/git-gardener.bash" ]]; then
    log_error "Completion files not found in $COMPLETIONS_DIR"
    exit 1
fi

main "$@"