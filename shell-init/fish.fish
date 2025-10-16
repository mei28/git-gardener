# Fish shell integration for git-gardener
# Usage: git-gardener shell-init fish | source

# Main shell function
function ggr
    # Special handling for 'cd' command
    if test "$argv[1]" = "cd"
        # Execute git-gardener cd and capture the output (worktree path)
        set -l target_path (command git-gardener cd $argv[2] 2>&1)
        set -l exit_code $status

        if test $exit_code -eq 0
            # Success: change directory
            builtin cd $target_path
        else
            # Error: display error message
            echo $target_path >&2
            return $exit_code
        end
    else
        # For all other commands, pass through to git-gardener
        command git-gardener $argv
    end
end

# Tab completion integration
complete -c ggr -e

# Main commands
complete -c ggr -f -n "__fish_use_subcommand" -a "init" -d "Initialize git-gardener in the current repository"
complete -c ggr -f -n "__fish_use_subcommand" -a "add" -d "Create a new worktree"
complete -c ggr -f -n "__fish_use_subcommand" -a "list" -d "List all worktrees"
complete -c ggr -f -n "__fish_use_subcommand" -a "cd" -d "Change to worktree directory"
complete -c ggr -f -n "__fish_use_subcommand" -a "remove" -d "Remove a worktree"
complete -c ggr -f -n "__fish_use_subcommand" -a "completion" -d "Generate shell completion scripts"
complete -c ggr -f -n "__fish_use_subcommand" -a "shell-init" -d "Generate shell integration script"
complete -c ggr -f -n "__fish_use_subcommand" -a "help" -d "Print help information"

# Global options
complete -c ggr -s h -l help -d "Print help"
complete -c ggr -s V -l version -d "Print version"

# cd command completions
complete -c ggr -f -n "__fish_seen_subcommand_from cd" -a "@" -d "Main worktree"
complete -c ggr -f -n "__fish_seen_subcommand_from cd" -a "(git-gardener list --names-only 2>/dev/null)" -d "Worktree"

# add command options
complete -c ggr -n "__fish_seen_subcommand_from add" -s b -l new-branch -d "Create a new branch"
complete -c ggr -n "__fish_seen_subcommand_from add" -s c -l commit -d "Create from specific commit"
complete -c ggr -n "__fish_seen_subcommand_from add" -s h -l help -d "Print help"

# list command options
complete -c ggr -n "__fish_seen_subcommand_from list" -l names-only -d "Output only worktree names"
complete -c ggr -n "__fish_seen_subcommand_from list" -s h -l help -d "Print help"

# remove command completions
complete -c ggr -f -n "__fish_seen_subcommand_from remove; and not __fish_seen_argument -l with-branch" -a "(git-gardener list --names-only 2>/dev/null)" -d "Worktree"
complete -c ggr -n "__fish_seen_subcommand_from remove" -l with-branch -d "Also remove the branch"
complete -c ggr -n "__fish_seen_subcommand_from remove" -s h -l help -d "Print help"

# completion command completions
complete -c ggr -f -n "__fish_seen_subcommand_from completion" -a "bash zsh fish" -d "Shell"

# shell-init command completions
complete -c ggr -f -n "__fish_seen_subcommand_from shell-init" -a "bash zsh fish" -d "Shell"
