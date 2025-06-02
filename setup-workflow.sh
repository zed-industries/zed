#!/bin/bash

set -e

echo "🚀 Setting up Zed custom workflow..."

export FORK_URL="https://github.com/scv9/zed"

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Not in a git repository"
    exit 1
fi

# Setup remotes
echo "🔄 Setting up git remotes..."
if ! git remote get-url fork > /dev/null 2>&1; then
    # echo "Please enter your fork URL (e.g., https://github.com/YOUR_USERNAME/zed.git):"
    # read FORK_URL
    git remote add fork "$FORK_URL"
fi

if ! git remote get-url upstream > /dev/null 2>&1; then
    if git remote get-url origin | grep -q "zed-industries/zed"; then
        git remote rename origin upstream
    else
        git remote add upstream "https://github.com/zed-industries/zed.git"
    fi
fi

# Create custom-main branch
echo "🌿 Creating custom-main branch..."
git fetch upstream
if ! git show-ref --verify --quiet refs/heads/custom-main; then
    git checkout -b custom-main upstream/main
    git push fork custom-main
else
    echo "custom-main branch already exists"
fi

# Create feature branches for existing work
echo "🔧 Setting up feature branches..."

# Analyze current changes to suggest branch organization
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  You have uncommitted changes. Please commit or stash them first."
    echo "Suggested feature branches based on modified files:"
    
    git status --porcelain | while read status file; do
        case "$file" in
            *lmstudio*|*language_models*)
                echo "  feature/lmstudio-fixes - $file"
                ;;
            *agent*|*tool_use*)
                echo "  feature/conversation-loop-fix - $file"
                ;;
            *uide*|*assistant_tools*)
                echo "  feature/uide-improvements - $file"
                ;;
            *)
                echo "  feature/misc-improvements - $file"
                ;;
        esac
    done
fi

# Create .gitignore entries for workflow files
echo "📝 Adding workflow files to .gitignore..."
cat >> .gitignore << EOF

# Custom workflow files
patches/
backup-*
.gitworkflow.local
EOF

echo "✅ Workflow setup complete!"
echo ""
echo "Next steps:"
echo "1. Commit your current changes to appropriate feature branches"
echo "2. Use './update-from-upstream.sh' to pull upstream changes"
echo "3. Use './manage-patches.sh export' to create portable patches"
echo "4. Edit '.gitworkflow' to document your modifications" 