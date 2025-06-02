#!/bin/bash

PATCHES_DIR="patches"
mkdir -p $PATCHES_DIR

case "$1" in
    "export")
        echo "📦 Exporting patches..."
        # Export each feature branch as a patch
        for branch in $(git branch | grep "feature/" | sed 's/*//' | tr -d ' '); do
            patch_name=$(echo $branch | sed 's/feature\///')
            echo "Exporting $branch to $PATCHES_DIR/$patch_name.patch"
            git format-patch custom-main..$branch --stdout > $PATCHES_DIR/$patch_name.patch
        done
        echo "✅ Patches exported to $PATCHES_DIR/"
        ;;
    
    "apply")
        echo "🔧 Applying patches..."
        # Apply all patches in the patches directory
        for patch in $PATCHES_DIR/*.patch; do
            if [ -f "$patch" ]; then
                echo "Applying $(basename $patch)..."
                git am "$patch" || {
                    echo "❌ Failed to apply $patch"
                    echo "Run 'git am --abort' to cancel or resolve conflicts manually"
                    exit 1
                }
            fi
        done
        echo "✅ All patches applied successfully!"
        ;;
    
    "clean")
        echo "🧹 Cleaning up patch branches..."
        git checkout custom-main
        for branch in $(git branch | grep "feature/" | sed 's/*//' | tr -d ' '); do
            git branch -D $branch
        done
        echo "✅ Feature branches removed"
        ;;
    
    *)
        echo "Usage: $0 {export|apply|clean}"
        echo "  export - Export feature branches as patches"
        echo "  apply  - Apply patches to current branch"
        echo "  clean  - Remove all feature branches"
        exit 1
        ;;
esac 