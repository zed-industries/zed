#!/usr/bin/env python3
#
# Copyright (C) 2020-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.

"""Extract the FreeType version numbers from `<freetype/freetype.h>`.

This script parses the header to extract the version number defined there.
By default, the full dotted version number is printed, but `--major`,
`--minor` or `--patch` can be used to only print one of these values
instead.
"""

from __future__ import print_function

import argparse
import os
import re
import sys

# Expected input:
#
#  ...
#  #define FREETYPE_MAJOR  2
#  #define FREETYPE_MINOR  10
#  #define FREETYPE_PATCH  2
#  ...

RE_MAJOR = re.compile(r"^ \#define \s+ FREETYPE_MAJOR \s+ (.*) $", re.X)
RE_MINOR = re.compile(r"^ \#define \s+ FREETYPE_MINOR \s+ (.*) $", re.X)
RE_PATCH = re.compile(r"^ \#define \s+ FREETYPE_PATCH \s+ (.*) $", re.X)


def parse_freetype_header(header):
    major = None
    minor = None
    patch = None

    for line in header.splitlines():
        line = line.rstrip()
        m = RE_MAJOR.match(line)
        if m:
            assert major == None, "FREETYPE_MAJOR appears more than once!"
            major = m.group(1)
            continue

        m = RE_MINOR.match(line)
        if m:
            assert minor == None, "FREETYPE_MINOR appears more than once!"
            minor = m.group(1)
            continue

        m = RE_PATCH.match(line)
        if m:
            assert patch == None, "FREETYPE_PATCH appears more than once!"
            patch = m.group(1)
            continue

    assert (
        major and minor and patch
    ), "This header is missing one of FREETYPE_MAJOR, FREETYPE_MINOR or FREETYPE_PATCH!"

    return (major, minor, patch)


def main():
    parser = argparse.ArgumentParser(description=__doc__)

    group = parser.add_mutually_exclusive_group()
    group.add_argument(
        "--major",
        action="store_true",
        help="Only print the major version number.",
    )
    group.add_argument(
        "--minor",
        action="store_true",
        help="Only print the minor version number.",
    )
    group.add_argument(
        "--patch",
        action="store_true",
        help="Only print the patch version number.",
    )

    parser.add_argument(
        "input",
        metavar="FREETYPE_H",
        help="The input freetype.h header to parse.",
    )

    args = parser.parse_args()
    with open(args.input) as f:
        header = f.read()

    version = parse_freetype_header(header)

    if args.major:
        print(version[0])
    elif args.minor:
        print(version[1])
    elif args.patch:
        print(version[2])
    else:
        print("%s.%s.%s" % version)

    return 0


if __name__ == "__main__":
    sys.exit(main())
