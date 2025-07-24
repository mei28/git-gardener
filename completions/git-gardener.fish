# Fish completion for git-gardener

# Helper function to get worktree names
function __git_gardener_worktrees
    git-gardener list --names-only 2>/dev/null
end

# Helper function to get git remotes
function __git_gardener_remotes
    git remote 2>/dev/null
end

# Helper function to get git branches
function __git_gardener_branches
    git branch 2>/dev/null | sed 's/^[ *]*//'
end

# Main command completions
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'init' -d 'Initialize git-gardener configuration'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'add' -d 'Create a new worktree'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'list' -d 'List all worktrees'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'config' -d 'View or edit configuration'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'clean' -d 'Clean up worktrees based on conditions'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'pull-all' -d 'Pull all worktrees'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'tui' -d 'Launch interactive TUI'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'cd' -d 'Change to worktree directory'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'remove' -d 'Remove a worktree'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'prune' -d 'Prune worktree information'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'move' -d 'Move a worktree to a new location'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'completion' -d 'Generate shell completion scripts'
complete -f -c git-gardener -n '__fish_use_subcommand' -a 'help' -d 'Print help information'

# Global options
complete -c git-gardener -s v -l verbose -d 'Increase verbosity'
complete -c git-gardener -s h -l help -d 'Show help'
complete -c git-gardener -s V -l version -d 'Show version'

# init command
complete -c git-gardener -n '__fish_seen_subcommand_from init' -s f -l force -d 'Overwrite existing config file'

# add command
complete -c git-gardener -n '__fish_seen_subcommand_from add' -s b -l branch -d 'Branch name' -r -a '(__git_gardener_branches)'
complete -c git-gardener -n '__fish_seen_subcommand_from add' -s p -l path -d 'Path' -r -a '(__fish_complete_directories)'
complete -c git-gardener -n '__fish_seen_subcommand_from add' -l upstream -d 'Set upstream remote' -r -a '(__git_gardener_remotes)'
complete -c git-gardener -n '__fish_seen_subcommand_from add' -s c -l create-branch -d 'Create a new branch'

# list command
complete -c git-gardener -n '__fish_seen_subcommand_from list' -s a -l all -d 'Show all worktrees including prunable ones'
complete -c git-gardener -n '__fish_seen_subcommand_from list' -l names-only -d 'Output only worktree names'

# config command
complete -c git-gardener -n '__fish_seen_subcommand_from config' -a 'view' -d 'View current configuration'
complete -c git-gardener -n '__fish_seen_subcommand_from config' -a 'set' -d 'Set a configuration value'

# config set keys
complete -c git-gardener -n '__fish_seen_subcommand_from config; and __fish_seen_subcommand_from set' -a 'root_dir' -d 'Root directory for worktrees'
complete -c git-gardener -n '__fish_seen_subcommand_from config; and __fish_seen_subcommand_from set' -a 'editor' -d 'Editor command'
complete -c git-gardener -n '__fish_seen_subcommand_from config; and __fish_seen_subcommand_from set' -a 'post_create' -d 'Post-create hook command'

# clean command
complete -c git-gardener -n '__fish_seen_subcommand_from clean' -l merged -d 'Remove merged worktrees'
complete -c git-gardener -n '__fish_seen_subcommand_from clean' -l stale -d 'Remove stale worktrees' -r
complete -c git-gardener -n '__fish_seen_subcommand_from clean' -l force -d 'Force removal'

# pull-all command
complete -c git-gardener -n '__fish_seen_subcommand_from pull-all' -s j -l parallel -d 'Number of parallel operations' -r

# tui command
complete -c git-gardener -n '__fish_seen_subcommand_from tui' -l fullscreen -d 'Run in fullscreen mode'
complete -c git-gardener -n '__fish_seen_subcommand_from tui' -l no-mouse -d 'Disable mouse support'

# cd command - complete with worktree names
complete -c git-gardener -n '__fish_seen_subcommand_from cd' -a '(__git_gardener_worktrees)' -d 'Worktree name'

# remove command - complete with worktree names
complete -c git-gardener -n '__fish_seen_subcommand_from remove' -s f -l force -d 'Force removal'
complete -c git-gardener -n '__fish_seen_subcommand_from remove' -a '(__git_gardener_worktrees)' -d 'Worktree name'

# prune command
complete -c git-gardener -n '__fish_seen_subcommand_from prune' -l dry-run -d 'Show what would be removed'

# move command - complete with worktree names and directories
complete -c git-gardener -n '__fish_seen_subcommand_from move; and test (count (commandline -opc)) -eq 2' -a '(__git_gardener_worktrees)' -d 'Worktree name'
complete -c git-gardener -n '__fish_seen_subcommand_from move; and test (count (commandline -opc)) -eq 3' -a '(__fish_complete_directories)' -d 'New path'

# completion command - complete with shell names
complete -c git-gardener -n '__fish_seen_subcommand_from completion' -a 'bash zsh fish' -d 'Shell type'