#!/bin/env python3

import os
from pathlib import Path

def process_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    modified_lines = []
    in_conflict = False
    after_equals = False
    keep_lines = []

    for line in lines:
        if line.startswith('<<<<<<<'):
            in_conflict = True
            after_equals = False
            keep_lines = []
            continue
        elif line.startswith('======='):
            after_equals = True
            continue
        elif line.startswith('>>>>>>>'):
            in_conflict = False
            after_equals = False
            modified_lines.extend(keep_lines)
            continue

        if in_conflict:
            if after_equals:
                keep_lines.append(line)
        else:
            modified_lines.append(line)

    # Only write if changes were made
    if lines != modified_lines:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(modified_lines)
        print(f"Processed: {filepath}")
        return True
    return False

def main():
    # Get current directory
    current_dir = Path('.')

    # Find all .rs files recursively
    rust_files = list(current_dir.rglob('*.rs'))

    files_processed = 0
    files_modified = 0

    # Process each file
    for filepath in rust_files:
        try:
            files_processed += 1
            if process_file(filepath):
                files_modified += 1
        except Exception as e:
            print(f"Error processing {filepath}: {str(e)}")

    print(f"\nSummary:")
    print(f"Files processed: {files_processed}")
    print(f"Files modified: {files_modified}")

if __name__ == "__main__":
    main()
