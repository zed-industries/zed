#!/usr/bin/env python3
"""
This script analyzes all the highlights.scm files in our embedded languages and extensions.
It counts the number of unique instances of @{name} and the languages in which they are used.

This is useful to help avoid accidentally introducing new tags when appropriate ones already exist when adding new languages.

Flags:
-v, --verbose: Include a detailed list of languages for each tag found in the highlights.scm files.
"""

import argparse
import re
from collections import defaultdict
from pathlib import Path
from typing import Any

pattern = re.compile(r'@(?!_)[a-zA-Z_.]+')

def parse_arguments():
    parser = argparse.ArgumentParser(description='Analyze highlights.scm files for unique instances and their languages.')
    parser.add_argument('-v', '--verbose', action='store_true', help='Include a list of languages for each tag.')
    return parser.parse_args()

def find_highlight_files(root_dir):
    for path in Path(root_dir).rglob('highlights.scm'):
        yield path

def count_instances(files):
    instances: defaultdict[list[Any], dict[str, Any]] = defaultdict(lambda: {'count': 0, 'languages': set()})
    for file_path in files:
        language = file_path.parent.name
        with open(file_path, "r") as file:
            text = file.read()
            matches = pattern.findall(text)
            for match in matches:
                instances[match]['count'] += 1
                instances[match]['languages'].add(language)
    return instances

def print_instances(instances, verbose=False):
    for item, details in sorted(instances.items(), key=lambda x: x[0]):
        languages = ', '.join(sorted(details['languages']))
        if verbose:
            print(f"{item} ({details['count']}) - [{languages}]")
        else:
            print(f"{item} ({details['count']})")

def main():
    args = parse_arguments()

    base_dir = Path(__file__).parent.parent
    core_path = base_dir / 'crates/languages/src'
    extension_path = base_dir / 'extensions/'

    core_instances = count_instances(find_highlight_files(core_path))
    extension_instances = count_instances(find_highlight_files(extension_path))

    unique_extension_instances = {k: v for k, v in extension_instances.items() if k not in core_instances}

    print('Shared:\n')
    print_instances(core_instances, args.verbose)

    if unique_extension_instances:
        print('\nExtension-only:\n')
        print_instances(unique_extension_instances, args.verbose)

if __name__ == '__main__':
    main()
