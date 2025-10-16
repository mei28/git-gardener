# Zsh shell integration for git-gardener
# Usage: eval "$(git-gardener shell-init zsh)"

# Main shell function
ggr() {
    # Special handling for 'cd' command
    if [[ "$1" == "cd" ]]; then
        local target_path
        # Execute git-gardener cd and capture the output (worktree path)
        target_path=$(command git-gardener cd "$2" 2>&1)
        local exit_code=$?

        if [[ $exit_code -eq 0 ]]; then
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
    local curcontext="$curcontext" state line
    typeset -A opt_args

    local -a commands
    commands=(
        'init:Initialize git-gardener in the current repository'
        'add:Create a new worktree'
        'list:List all worktrees'
        'cd:Change to worktree directory'
        'remove:Remove a worktree'
        'completion:Generate shell completion scripts'
        'shell-init:Generate shell integration script'
        'help:Print help information'
    )

    _arguments -C \
        '1: :->command' \
        '*::arg:->args'

    case $state in
        command)
            _describe 'command' commands
            ;;
        args)
            case $words[1] in
                cd)
                    # Complete worktree names for cd command
                    local -a worktrees
                    worktrees=("@" ${(f)"$(git-gardener list --names-only 2>/dev/null)"})
                    _describe 'worktree' worktrees
                    ;;
                add)
                    _arguments \
                        '-b[Create a new branch]' \
                        '--new-branch[Create a new branch]' \
                        '-c[Create from specific commit]:commit:' \
                        '--commit[Create from specific commit]:commit:' \
                        '-h[Print help]' \
                        '--help[Print help]'
                    ;;
                list)
                    _arguments \
                        '--names-only[Output only worktree names]' \
                        '-h[Print help]' \
                        '--help[Print help]'
                    ;;
                remove)
                    if [[ $CURRENT -eq 2 ]]; then
                        # Complete worktree names
                        local -a worktrees
                        worktrees=(${(f)"$(git-gardener list --names-only 2>/dev/null)"})
                        _describe 'worktree' worktrees
                    else
                        _arguments \
                            '--with-branch[Also remove the branch]' \
                            '-h[Print help]' \
                            '--help[Print help]'
                    fi
                    ;;
                completion|shell-init)
                    # Complete shell names
                    _arguments '1: :(bash zsh fish)'
                    ;;
            esac
            ;;
    esac
}

# Register the completion function
compdef _ggr_completion ggr
