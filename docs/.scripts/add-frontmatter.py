#!/usr/bin/env python3
"""
Batch-add YAML frontmatter to markdown files that don't have it.

Usage:
    python3 add-frontmatter.py [--dry-run]
"""

import os
import re
import sys
from pathlib import Path

DOCS_SRC = Path(__file__).parent.parent / "src"

# Files to skip
SKIP_FILES = {"SUMMARY.md"}

# Custom descriptions for files where auto-generation won't work well
CUSTOM_DESCRIPTIONS = {
    "languages.md": "Overview of programming language support in Zed, including built-in and extension-based languages.",
    "extensions.md": "Extend Zed with themes, language support, AI tools, and more through the extension system.",
    "all-actions.md": "Complete reference of all available actions and commands in Zed.",
    "troubleshooting.md": "Common issues and solutions for Zed on all platforms.",
    "development.md": "Guide to building and developing Zed from source.",
    "authentication.md": "Sign in to Zed to access collaboration features and AI services.",
    "telemetry.md": "What data Zed collects and how to control telemetry settings.",
    "performance.md": "Performance profiling and optimization for Zed development.",
    "worktree-trust.md": "Configure which folders Zed trusts for running code and extensions.",
}

# Title overrides for cleaner display
TITLE_OVERRIDES = {
    "cpp.md": "C++",
    "csharp.md": "C#",
    "sh.md": "Shell Script",
    "rst.md": "reStructuredText",
}


def extract_title(content: str, filename: str) -> str:
    """Extract title from H1 heading or use filename."""
    # Check for title override first
    if filename in TITLE_OVERRIDES:
        return TITLE_OVERRIDES[filename]
    
    match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
    if match:
        title = match.group(1).strip()
        # Clean up common patterns
        title = re.sub(r'^How to Set Up\s+', '', title)
        title = re.sub(r'\s+in Zed$', '', title)
        return title
    
    # Fallback to filename
    return filename.replace('.md', '').replace('-', ' ').title()


def extract_description(content: str, filepath: str) -> str:
    """Extract description from first meaningful paragraph."""
    filename = os.path.basename(filepath)
    
    # Check for custom description first
    if filename in CUSTOM_DESCRIPTIONS:
        return CUSTOM_DESCRIPTIONS[filename]
    
    # For language files, generate a standard description
    if "/languages/" in filepath:
        lang_name = extract_title(content, filename)
        return f"Configure {lang_name} language support in Zed, including language servers, formatting, and debugging."
    
    # For extension files
    if "/extensions/" in filepath:
        title = extract_title(content, filename)
        return f"{title} for Zed extensions."
    
    # For migration files
    if "/migrate/" in filepath:
        editor_name = extract_title(content, filename)
        return f"Guide for migrating from {editor_name} to Zed, including settings and keybindings."
    
    # For development files
    if "/development/" in filepath:
        title = extract_title(content, filename)
        return f"Guide to {title.lower()} for Zed development."
    
    # For collaboration files
    if "/collaboration/" in filepath:
        title = extract_title(content, filename)
        return f"Use {title.lower()} in Zed for real-time collaboration."
    
    # Try to find first paragraph after title
    lines = content.split('\n')
    in_content = False
    paragraph_lines = []
    
    for line in lines:
        stripped = line.strip()
        
        # Skip title
        if stripped.startswith('# '):
            in_content = True
            continue
        
        # Skip empty lines before first paragraph
        if in_content and not stripped:
            if paragraph_lines:
                break
            continue
        
        # Skip list items, code blocks, headers
        if stripped.startswith(('-', '*', '```', '#', '|', '>')):
            if paragraph_lines:
                break
            continue
        
        if in_content and stripped:
            paragraph_lines.append(stripped)
    
    if paragraph_lines:
        desc = ' '.join(paragraph_lines)
        # Truncate at 160 chars for SEO
        if len(desc) > 160:
            desc = desc[:157] + "..."
        # Remove any markdown links
        desc = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', desc)
        return desc
    
    # Fallback
    title = extract_title(content, filename)
    return f"Documentation for {title} in Zed."


def has_frontmatter(content: str) -> bool:
    """Check if file already has YAML frontmatter."""
    return content.startswith('---\n')


def add_frontmatter(filepath: Path, dry_run: bool = False) -> bool:
    """Add frontmatter to a file if it doesn't have it."""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    if has_frontmatter(content):
        return False
    
    title = extract_title(content, filepath.name)
    description = extract_description(content, str(filepath))
    
    # Escape quotes in description
    description = description.replace('"', '\\"')
    
    frontmatter = f'''---
title: {title}
description: "{description}"
---

'''
    
    new_content = frontmatter + content
    
    if dry_run:
        print(f"Would add frontmatter to: {filepath}")
        print(f"  Title: {title}")
        print(f"  Description: {description[:80]}...")
        print()
    else:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Added frontmatter to: {filepath}")
    
    return True


def main():
    dry_run = "--dry-run" in sys.argv
    
    if dry_run:
        print("DRY RUN - No files will be modified\n")
    
    count = 0
    for filepath in DOCS_SRC.rglob("*.md"):
        if filepath.name in SKIP_FILES:
            continue
        
        if add_frontmatter(filepath, dry_run):
            count += 1
    
    print(f"\n{'Would update' if dry_run else 'Updated'} {count} files")


if __name__ == "__main__":
    main()
