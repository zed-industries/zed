#!/bin/bash

# This script manages prompt overrides for the Zed editor.
#
# It provides functionality to:
# 1. Link the current repository's prompt templates to Zed's configuration.
# 2. Create and link a separate Git worktree for prompt management.
# 3. Unlink previously linked prompt overrides.
#
# Usage:
#   ./script_name.sh link             # Link current repo's prompts
#   ./script_name.sh link --worktree  # Create and link a separate worktree
#   ./script_name.sh unlink           # Remove existing prompt override link
#
# The script ensures proper Git branch and worktree setup when using the
# --worktree option. It also provides informative output and error handling.

if [ "$1" = "link" ]; then
    # Remove existing link (or directory)
    rm -rf ~/.config/zed/prompt_overrides
    if [ "$2" = "--worktree" ]; then
        # Check if 'prompts' branch exists, create if not
        if ! git show-ref --quiet refs/heads/prompts; then
            git branch prompts
        fi
        # Check if 'prompts' worktree exists
        if git worktree list | grep -q "../zed_prompts"; then
            echo "Worktree already exists at ../zed_prompts."
        else
            # Create worktree if it doesn't exist
            git worktree add ../zed_prompts prompts || git worktree add ../zed_prompts -b prompts
        fi
        ln -sf "$(realpath "$(pwd)/../zed_prompts/assets/prompts")" ~/.config/zed/prompt_overrides
        echo "Linked $(realpath "$(pwd)/../zed_prompts/assets/prompts") to ~/.config/zed/prompt_overrides"
        echo -e "\033[0;33mDon't forget you have it linked, or your prompts will go stale\033[0m"
    else
        ln -sf "$(pwd)/assets/prompts" ~/.config/zed/prompt_overrides
        echo "Linked $(pwd)/assets/prompts to ~/.config/zed/prompt_overrides"
    fi
elif [ "$1" = "unlink" ]; then
    if [ -e ~/.config/zed/prompt_overrides ]; then
        # Remove symbolic link
        rm -rf ~/.config/zed/prompt_overrides
        echo "Unlinked ~/.config/zed/prompt_overrides"
    else
        echo -e "\033[33mWarning: No file exists at ~/.config/zed/prompt_overrides\033[0m"
    fi
else
    echo "This script helps you manage prompt overrides for Zed."
    echo "You can link this directory to have Zed use the contents of your current repo templates as your active prompts,"
    echo "or store your modifications in a separate Git worktree."
    echo
    echo "Usage: $0 [link [--worktree]|unlink]"
    echo
    echo "Options:"
    echo "  link               Create a symbolic link from ./assets/prompts to ~/.config/zed/prompt_overrides"
    echo "  link --worktree    Create a 'prompts' Git worktree in ../prompts, then link ../prompts/assets/prompts"
    echo "                     to ~/.config/zed/prompt_overrides"
    echo "  unlink             Remove the symbolic link at ~/.config/zed/prompt_overrides"
    exit 1
fi
