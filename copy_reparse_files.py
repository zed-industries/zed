#!/usr/bin/env python3
"""
Script to copy files listed in extract_reparse.py output to the current directory.

Reads the output of `extract_reparse.py --lang <LANG>` and copies each file
to the current working directory, preserving the directory structure.

Usage:
    python3 extract_reparse.py --lang XML > xml.out
    python3 copy_reparse_files.py --prefix <prefix> [input_file] [--out <output_path>]

Arguments:
    prefix      Path prefix to prepend to each relative path
    input_file  Input file containing extract_reparse.py output (default: stdin)
    output_path Optional output directory (default: current working directory)

Example:
    python3 copy_reparse_files.py --prefix /path/to/source xml.out
    python3 copy_reparse_files.py --prefix /path/to/source xml.out --out /path/to/dest
"""

import argparse
import os
import re
import shutil
import sys


def parse_paths(input_file) -> list[str]:
    """
    Parse file paths from extract_reparse.py output.

    Args:
        input_file: File object to read from.

    Returns:
        List of relative file paths.
    """
    paths = []
    pattern = re.compile(r'^\s*"(.+)"\s*$')

    for line in input_file:
        match = pattern.match(line)
        if match:
            paths.append(match.group(1))

    return paths


def copy_files(paths: list[str], prefix: str, output_dir: str, dry_run: bool = False) -> tuple[int, int]:
    """
    Copy files to the output directory.

    Args:
        paths: List of relative file paths.
        prefix: Path prefix to prepend to each path.
        output_dir: Directory to copy files to.
        dry_run: If True, only print what would be done without copying.

    Returns:
        Tuple of (successful_copies, failed_copies).
    """
    success_count = 0
    fail_count = 0

    for relative_path in paths:
        source_path = os.path.join(prefix, relative_path)
        dest_path = os.path.join(output_dir, relative_path)

        if dry_run:
            print(f"Would copy: {source_path} -> {dest_path}")
            success_count += 1
            continue

        try:
            dest_dir = os.path.dirname(dest_path)
            if dest_dir:
                os.makedirs(dest_dir, exist_ok=True)

            shutil.copy2(source_path, dest_path)
            print(f"Copied: {relative_path}")
            success_count += 1
        except FileNotFoundError:
            print(f"Not found: {source_path}", file=sys.stderr)
            fail_count += 1
        except PermissionError:
            print(f"Permission denied: {source_path}", file=sys.stderr)
            fail_count += 1
        except IOError as e:
            print(f"Error copying {source_path}: {e}", file=sys.stderr)
            fail_count += 1

    return success_count, fail_count


def main():
    parser = argparse.ArgumentParser(
        description="Copy files listed in extract_reparse.py output to the current directory."
    )
    parser.add_argument(
        "--prefix", "-p",
        required=True,
        help="Path prefix to prepend to each relative path"
    )
    parser.add_argument(
        "input_file",
        nargs="?",
        default=None,
        help="Input file containing extract_reparse.py output (default: stdin)"
    )
    parser.add_argument(
        "--dry-run", "-n",
        action="store_true",
        help="Print what would be copied without actually copying"
    )
    parser.add_argument(
        "--out", "-o",
        default=None,
        help="Output directory (default: current working directory)"
    )

    args = parser.parse_args()

    if args.input_file:
        try:
            with open(args.input_file, 'r') as f:
                paths = parse_paths(f)
        except FileNotFoundError:
            print(f"Error: Input file '{args.input_file}' not found.", file=sys.stderr)
            sys.exit(1)
        except IOError as e:
            print(f"Error reading input file: {e}", file=sys.stderr)
            sys.exit(1)
    else:
        paths = parse_paths(sys.stdin)

    if not paths:
        print("No paths found in input.", file=sys.stderr)
        sys.exit(1)

    prefix = os.path.expanduser(args.prefix)
    output_dir = os.path.expanduser(args.out) if args.out else os.getcwd()

    if not os.path.isdir(prefix):
        print(f"Error: Prefix path '{prefix}' is not a directory.", file=sys.stderr)
        sys.exit(1)

    if not os.path.isdir(output_dir):
        print(f"Error: Output path '{output_dir}' is not a directory.", file=sys.stderr)
        sys.exit(1)

    success, failed = copy_files(paths, prefix, output_dir, dry_run=args.dry_run)

    print(f"\nSummary: {success} copied, {failed} failed")

    if failed > 0:
        sys.exit(1)


if __name__ == "__main__":
    main()
