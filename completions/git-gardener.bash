# Bash completion for git-gardener

_git_gardener() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    # Main commands
    local commands="init add list config clean pull-all tui cd remove prune move help"
    
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
                        COMPREPLY=( $(compgen -W "${worktrees}" -- ${cur}) )
                    fi
                    ;;
                add)
                    case "${prev}" in
                        -b|--branch)
                            # Complete branch names from git
                            local branches=$(git branch 2>/dev/null | sed 's/^[ *]*//')
                            COMPREPLY=( $(compgen -W "${branches}" -- ${cur}) )
                            ;;
                        -p|--path)
                            # Complete directory paths
                            COMPREPLY=( $(compgen -d -- ${cur}) )
                            ;;
                        --upstream)
                            # Complete remote names
                            local remotes=$(git remote 2>/dev/null)
                            COMPREPLY=( $(compgen -W "${remotes}" -- ${cur}) )
                            ;;
                        *)
                            # Complete options
                            local opts="-b --branch -p --path --upstream -c --create-branch -h --help"
                            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                            ;;
                    esac
                    ;;
                list)
                    # Complete list options
                    local opts="-a --all --names-only -h --help"
                    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    ;;
                config)
                    if [[ ${COMP_CWORD} -eq 2 ]]; then
                        # Complete config subcommands
                        COMPREPLY=( $(compgen -W "view set" -- ${cur}) )
                    elif [[ ${COMP_CWORD} -eq 3 && "${COMP_WORDS[2]}" == "set" ]]; then
                        # Complete config keys
                        local keys="root_dir editor post_create"
                        COMPREPLY=( $(compgen -W "${keys}" -- ${cur}) )
                    fi
                    ;;
                clean)
                    # Complete clean options
                    local opts="--merged --stale --force -h --help"
                    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    ;;
                pull-all)
                    case "${prev}" in
                        -j|--parallel)
                            # No completion for numbers
                            ;;
                        *)
                            local opts="-j --parallel -h --help"
                            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                            ;;
                    esac
                    ;;
                tui)
                    # Complete TUI options
                    local opts="--fullscreen --no-mouse -h --help"
                    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    ;;
                remove)
                    # Complete worktree names for remove command
                    if [[ ${COMP_CWORD} -eq 2 ]]; then
                        local worktrees=$(git-gardener list --names-only 2>/dev/null)
                        COMPREPLY=( $(compgen -W "${worktrees}" -- ${cur}) )
                    else
                        # Complete options
                        local opts="-f --force -h --help"
                        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    fi
                    ;;
                prune)
                    # Complete prune options
                    local opts="--dry-run -h --help"
                    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    ;;
                move)
                    if [[ ${COMP_CWORD} -eq 2 ]]; then
                        # Complete worktree names for first argument
                        local worktrees=$(git-gardener list --names-only 2>/dev/null)
                        COMPREPLY=( $(compgen -W "${worktrees}" -- ${cur}) )
                    elif [[ ${COMP_CWORD} -eq 3 ]]; then
                        # Complete directory paths for second argument
                        COMPREPLY=( $(compgen -d -- ${cur}) )
                    fi
                    ;;
                *)
                    # Global options
                    local opts="-v --verbose -h --help -V --version"
                    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    ;;
            esac
            ;;
    esac
}

# Register the completion function
complete -F _git_gardener git-gardener