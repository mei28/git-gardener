# Bash shell integration for git-gardener
# Usage: eval "$(git-gardener shell-init bash)"

# Main shell function
ggr() {
    # Special handling for 'cd' command
    if [ "$1" = "cd" ]; then
        local target_path
        # Execute git-gardener cd and capture the output (worktree path)
        target_path=$(command git-gardener cd "$2" 2>&1)
        local exit_code=$?

        if [ $exit_code -eq 0 ]; then
            # Success: change directory
            builtin cd "$target_path" || return 1
        else
            # Error: display error message
            echo "$target_path" >&2
            return $exit_code
        fi
    else
        # For all other commands, pass through to git-gardener
        command git-gardener "$@"
    fi
}

# Tab completion integration
_ggr_completion() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # Main commands
    local commands="init add list cd remove completion shell-init help"

    # Options for different commands
    case "${COMP_CWORD}" in
        1)
            # Complete main commands
            COMPREPLY=( $(compgen -W "${commands}" -- ${cur}) )
            return 0
            ;;
        *)
            # Command-specific completion
            case "${COMP_WORDS[1]}" in
                cd)
                    # Complete worktree names for cd command
                    if [[ ${COMP_CWORD} -eq 2 ]]; then
                        local worktrees=$(git-gardener list --names-only 2>/dev/null)
                        COMPREPLY=( $(compgen -W "@ ${worktrees}" -- ${cur}) )
                    fi
                    ;;
                add)
                    case "${prev}" in
                        -b|--new-branch)
                            # Allow any input for new branch name
                            return 0
                            ;;
                        -c|--commit)
                            # Complete commit hashes (simplified)
                            return 0
                            ;;
                        *)
                            # Complete options
                            local opts="-b --new-branch -c --commit -h --help"
                            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                            ;;
                    esac
                    ;;
                list)
                    # Complete list options
                    local opts="--names-only -h --help"
                    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    ;;
                remove)
                    # Complete worktree names for remove command
                    if [[ ${COMP_CWORD} -eq 2 ]]; then
                        local worktrees=$(git-gardener list --names-only 2>/dev/null)
                        COMPREPLY=( $(compgen -W "${worktrees}" -- ${cur}) )
                    else
                        # Complete options
                        local opts="--with-branch -h --help"
                        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    fi
                    ;;
                completion)
                    # Complete shell names
                    if [[ ${COMP_CWORD} -eq 2 ]]; then
                        COMPREPLY=( $(compgen -W "bash zsh fish" -- ${cur}) )
                    fi
                    ;;
                shell-init)
                    # Complete shell names
                    if [[ ${COMP_CWORD} -eq 2 ]]; then
                        COMPREPLY=( $(compgen -W "bash zsh fish" -- ${cur}) )
                    fi
                    ;;
                *)
                    # Global options
                    local opts="-h --help -V --version"
                    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    ;;
            esac
            ;;
    esac
}

# Register the completion function
complete -F _ggr_completion ggr
