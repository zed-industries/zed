#!/bin/bash

set -e

echo "🔄 Updating from upstream Zed repository..."

# Fetch latest changes from upstream
git fetch upstream

# Switch to custom-main branch
git checkout custom-main

# Create a backup branch before updating
git branch backup-$(date +%Y%m%d-%H%M%S) custom-main

# Rebase custom changes on top of latest upstream
git rebase upstream/main

echo "✅ Custom main updated with upstream changes"

# Update feature branches
for branch in $(git branch | grep "feature/" | sed 's/*//' | tr -d ' '); do
    echo "🔄 Updating $branch..."
    git checkout $branch
    git rebase custom-main
    echo "✅ $branch updated"
done

git checkout custom-main
echo "🎉 All branches updated successfully!" 