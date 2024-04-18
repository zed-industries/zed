import os
import re
import collections
from pathlib import Path

# Initialize a regular expression to match @ instances
pattern = re.compile(r'(@[a-zA-Z.]+)')

def find_highlight_files(root_dir):
    """Find all highlight.scm files within a specified root directory."""
    for path in Path(root_dir).rglob('**/highlights.scm'):
        yield path

def count_instances(files):
    """Count all unique instances of @{name} in the given files."""
    instances = collections.Counter()
    for file_path in files:
        with open(file_path, "r") as file:
            text = file.read()
            matches = pattern.findall(text)
            instances.update(matches)
    return instances

def main():
    # Navigate up one directory from the script location
    base_dir = Path(__file__).parent.parent

    # Define paths to core languages and extensions
    core_path = base_dir / 'crates/languages/src'
    extension_path = base_dir / 'extensions/astro/languages'

    # Find and count occurrences in core and extension highlight files
    core_files = find_highlight_files(core_path)
    extension_files = find_highlight_files(extension_path)

    core_instances = count_instances(core_files)
    extension_instances = count_instances(extension_files)

    # Calculate the instances unique to extensions
    unique_extension_instances = extension_instances - core_instances

    # Sort and display the shared instances
    print('Shared:\n')
    for item, count in sorted(core_instances.items()):
        print(f"{item} ({count})")

    # If there are any unique extension instances, display them
    if unique_extension_instances:
        print('\nExtension-only:\n')
        for item, count in sorted(unique_extension_instances.items()):
            print(f"{item} ({count})")

if __name__ == '__main__':
    main()
