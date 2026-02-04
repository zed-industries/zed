#!/bin/bash
# Setup script for maintaining the 'home' branch separate from upstream Zed

set -e

echo "🏠 Zed Cachy Experiment - Repository Setup"
echo "=========================================="
echo ""

# Check if we're on the home branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "home" ]; then
    echo "⚠️  Warning: You're on branch '$CURRENT_BRANCH', not 'home'"
    echo "It's recommended to run this script from the 'home' branch."
    read -p "Do you want to switch to 'home' branch? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git checkout home
    fi
fi

# Check for upstream remote
if git remote | grep -q "^upstream$"; then
    echo "⚠️  Found 'upstream' remote pointing to:"
    git remote get-url upstream
    echo ""
    echo "According to BRANCH_STRATEGY.md, the 'home' branch should remain"
    echo "separate from upstream Zed updates to avoid breaking the experiment."
    echo ""
    read -p "Do you want to remove the upstream remote? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git remote remove upstream
        echo "✅ Removed upstream remote"
    else
        echo "⚠️  Keeping upstream remote. Please be careful not to merge from it!"
        echo "   See BRANCH_STRATEGY.md for safe cherry-picking practices."
    fi
else
    echo "✅ No upstream remote found - good!"
    echo "   The repository is configured to stay separate from Zed upstream."
fi

echo ""
echo "📋 Setup complete!"
echo ""
echo "Next steps:"
echo "1. If this is a new repository clone, the default branch should be set to 'home'"
echo "   on GitHub (see BRANCH_STRATEGY.md for instructions)"
echo "2. Read BRANCH_STRATEGY.md to understand the branching strategy"
echo "3. Always create feature branches from 'home', not from any upstream branch"
echo ""
echo "For more information, see:"
echo "  - BRANCH_STRATEGY.md - Branch management strategy"
echo "  - MODEL_EXPERIMENT_OVERVIEW.md - About this experiment"
echo ""
