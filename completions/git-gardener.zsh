#compdef git-gardener

# Zsh completion for git-gardener

_git_gardener() {
    local context state state_descr line
    typeset -A opt_args

    _arguments -C \
        '(-v --verbose)'{-v,--verbose}'[Increase verbosity]' \
        '(-h --help)'{-h,--help}'[Show help]' \
        '(-V --version)'{-V,--version}'[Show version]' \
        '1: :_git_gardener_commands' \
        '*:: :->args' && return 0

    case $state in
        args)
            case $words[1] in
                cd)
                    _arguments \
                        '(-h --help)'{-h,--help}'[Show help]' \
                        '1: :_git_gardener_worktrees'
                    ;;
                add)
                    _arguments \
                        '(-b --branch)'{-b,--branch}'[Branch name]:branch:' \
                        '(-p --path)'{-p,--path}'[Path]:path:_directories' \
                        '--upstream[Set upstream remote]:remote:_git_gardener_remotes' \
                        '(-c --create-branch)'{-c,--create-branch}'[Create a new branch]' \
                        '(-h --help)'{-h,--help}'[Show help]'
                    ;;
                list)
                    _arguments \
                        '(-a --all)'{-a,--all}'[Show all worktrees including prunable ones]' \
                        '--names-only[Output only worktree names]' \
                        '(-h --help)'{-h,--help}'[Show help]'
                    ;;
                config)
                    _arguments \
                        '1: :_git_gardener_config_subcommands' \
                        '2: :_git_gardener_config_keys' \
                        '3: :' \
                        '(-h --help)'{-h,--help}'[Show help]'
                    ;;
                clean)
                    _arguments \
                        '--merged[Remove merged worktrees]' \
                        '--stale[Remove stale worktrees]:days:' \
                        '--force[Force removal]' \
                        '(-h --help)'{-h,--help}'[Show help]'
                    ;;
                pull-all)
                    _arguments \
                        '(-j --parallel)'{-j,--parallel}'[Number of parallel operations]:number:' \
                        '(-h --help)'{-h,--help}'[Show help]'
                    ;;
                tui)
                    _arguments \
                        '--fullscreen[Run in fullscreen mode]' \
                        '--no-mouse[Disable mouse support]' \
                        '(-h --help)'{-h,--help}'[Show help]'
                    ;;
                init)
                    _arguments \
                        '(-f --force)'{-f,--force}'[Overwrite existing config file]' \
                        '(-h --help)'{-h,--help}'[Show help]'
                    ;;
            esac
            ;;
    esac
}

_git_gardener_commands() {
    local commands
    commands=(
        'init:Initialize git-gardener configuration'
        'add:Create a new worktree'
        'list:List all worktrees'
        'config:View or edit configuration'
        'clean:Clean up worktrees based on conditions'
        'pull-all:Pull all worktrees'
        'tui:Launch interactive TUI'
        'cd:Change to worktree directory'
        'help:Print help information'
    )
    _describe 'commands' commands
}

_git_gardener_config_subcommands() {
    local subcommands
    subcommands=(
        'view:View current configuration'
        'set:Set a configuration value'
    )
    _describe 'config subcommands' subcommands
}

_git_gardener_config_keys() {
    if [[ $words[2] == "set" ]]; then
        local keys
        keys=(
            'root_dir:Root directory for worktrees'
            'editor:Editor command'
            'post_create:Post-create hook command'
        )
        _describe 'config keys' keys
    fi
}

_git_gardener_worktrees() {
    local worktrees
    worktrees=(${(f)"$(git-gardener list --names-only 2>/dev/null)"})
    _describe 'worktrees' worktrees
}

_git_gardener_remotes() {
    local remotes
    remotes=(${(f)"$(git remote 2>/dev/null)"})
    _describe 'remotes' remotes
}

_git_gardener "$@"