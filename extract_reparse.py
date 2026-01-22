#!/usr/bin/env python3
"""
Script to extract and deduplicate reparse tuples from out.log.

Finds all lines containing `reparse(Some(<file_path>),<lang>)` pattern,
extracts the (file_path, lang) tuples, deduplicates, and outputs grouped by language.
"""

import argparse
import re
import sys
from collections import defaultdict


def extract_reparse_tuples(log_file_path: str) -> dict[str, set[str]]:
    """
    Extract unique file paths grouped by language from reparse log entries.

    Args:
        log_file_path: Path to the log file to process.

    Returns:
        Dictionary mapping language names to sets of file paths.
    """
    pattern = re.compile(r'reparse\(Some\(([^)]+)\),\s*(\w+)\)')

    entries_by_lang: dict[str, set[str]] = defaultdict(set)

    with open(log_file_path, 'r') as f:
        for line in f:
            match = pattern.search(line)
            if match:
                file_path = match.group(1)
                lang = match.group(2)
                entries_by_lang[lang].add(file_path)

    return entries_by_lang


def print_entries(entries_by_lang: dict[str, set[str]], lang_filter: str | None = None):
    """
    Print entries grouped by language.

    Args:
        entries_by_lang: Dictionary mapping language names to sets of file paths.
        lang_filter: Optional language to filter by (case-insensitive).
    """
    if lang_filter:
        lang_filter_lower = lang_filter.lower()
        matching_langs = [
            lang for lang in entries_by_lang.keys()
            if lang.lower() == lang_filter_lower
        ]
        if not matching_langs:
            print(f"No entries found for language: {lang_filter}", file=sys.stderr)
            sys.exit(1)
        assert len(matching_langs) == 1, f"Expected exactly one matching language, found: {matching_langs}"
        languages = matching_langs
    else:
        languages = sorted(entries_by_lang.keys())

    for lang in languages:
        paths = entries_by_lang[lang]
        print(lang)
        for path in sorted(paths):
            print(f"  {path}")
        print()


def main():
    parser = argparse.ArgumentParser(
        description="Extract and deduplicate reparse tuples from log files."
    )
    parser.add_argument(
        "log_file",
        nargs="?",
        default="out.log",
        help="Path to the log file (default: out.log)"
    )
    group = parser.add_mutually_exclusive_group()
    group.add_argument(
        "--lang",
        default=None,
        help="Filter by language (e.g., 'Python', 'Rust', 'XML')"
    )
    group.add_argument(
        "--langs",
        action="store_true",
        help="Print only the language names (no file paths)"
    )

    args = parser.parse_args()

    try:
        entries_by_lang = extract_reparse_tuples(args.log_file)
    except FileNotFoundError:
        print(f"Error: File '{args.log_file}' not found.", file=sys.stderr)
        sys.exit(1)
    except IOError as e:
        print(f"Error reading file: {e}", file=sys.stderr)
        sys.exit(1)

    if not entries_by_lang:
        print("No reparse entries found.", file=sys.stderr)
        sys.exit(1)

    if args.langs:
        for lang in sorted(entries_by_lang.keys()):
            print(lang)
    else:
        print_entries(entries_by_lang, args.lang)


if __name__ == "__main__":
    main()
