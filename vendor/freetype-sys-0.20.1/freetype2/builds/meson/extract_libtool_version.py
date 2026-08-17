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

"""Extract the libtool version from `configure.raw`.

This script parses the `configure.raw` file to extract the libtool version
number.  By default, the full dotted version number is printed, but
`--major`, `--minor` or `--patch` can be used to only print one of these
values instead.
"""

from __future__ import print_function

import argparse
import os
import re
import sys

# Expected input:
#
#  ...
#  version_info='23:2:17'
#  ...

RE_VERSION_INFO = re.compile(r"^version_info='(\d+):(\d+):(\d+)'")


def parse_configure_raw(header):
    major = None
    minor = None
    patch = None

    for line in header.splitlines():
        line = line.rstrip()
        m = RE_VERSION_INFO.match(line)
        if m:
            assert major == None, "version_info appears more than once!"
            major = m.group(1)
            minor = m.group(2)
            patch = m.group(3)
            continue

    assert (
        major and minor and patch
    ), "This input file is missing a version_info definition!"

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
    group.add_argument(
        "--soversion",
        action="store_true",
        help="Only print the libtool library suffix.",
    )

    parser.add_argument(
        "input",
        metavar="CONFIGURE_RAW",
        help="The input configure.raw file to parse.",
    )

    args = parser.parse_args()
    with open(args.input) as f:
        raw_file = f.read()

    version = parse_configure_raw(raw_file)

    if args.major:
        print(version[0])
    elif args.minor:
        print(version[1])
    elif args.patch:
        print(version[2])
    elif args.soversion:
        # Convert libtool version_info to the library suffix.
        # (current,revision, age) -> (current - age, age, revision)
        print(
            "%d.%s.%s"
            % (int(version[0]) - int(version[2]), version[2], version[1])
        )
    else:
        print("%s.%s.%s" % version)

    return 0


if __name__ == "__main__":
    sys.exit(main())
