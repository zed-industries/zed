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

"""Toggle settings in `ftoption.h` file based on command-line arguments.

This script takes an `ftoption.h` file as input and rewrites
`#define`/`#undef` lines in it based on `--enable=CONFIG_VARNAME` or
`--disable=CONFIG_VARNAME` arguments passed to it, where `CONFIG_VARNAME` is
configuration variable name, such as `FT_CONFIG_OPTION_USE_LZW`, that may
appear in the file.

Note that if one of `CONFIG_VARNAME` is not found in the input file, this
script exits with an error message listing the missing variable names.
"""

import argparse
import os
import re
import sys


def main():
    parser = argparse.ArgumentParser(description=__doc__)

    parser.add_argument(
        "input", metavar="FTOPTION_H", help="Path to input ftoption.h file."
    )

    parser.add_argument("--output", help="Output to file instead of stdout.")

    parser.add_argument(
        "--enable",
        action="append",
        default=[],
        help="Enable a given build option (e.g. FT_CONFIG_OPTION_USE_LZW).",
    )

    parser.add_argument(
        "--disable",
        action="append",
        default=[],
        help="Disable a given build option.",
    )

    args = parser.parse_args()

    common_options = set(args.enable) & set(args.disable)
    if common_options:
        parser.error(
            "Options cannot be both enabled and disabled: %s"
            % sorted(common_options)
        )
        return 1

    with open(args.input) as f:
        input_file = f.read()

    options_seen = set()

    new_lines = []
    for line in input_file.splitlines():
        # Expected formats:
        #   #define <CONFIG_VAR>
        #   /* #define <CONFIG_VAR> */
        #   #undef <CONFIG_VAR>
        line = line.rstrip()
        if line.startswith("/* #define ") and line.endswith(" */"):
            option_name = line[11:-3].strip()
            option_enabled = False
        elif line.startswith("#define "):
            option_name = line[8:].strip()
            option_enabled = True
        elif line.startswith("#undef "):
            option_name = line[7:].strip()
            option_enabled = False
        else:
            new_lines.append(line)
            continue

        options_seen.add(option_name)
        if option_enabled and option_name in args.disable:
            line = "#undef " + option_name
        elif not option_enabled and option_name in args.enable:
            line = "#define " + option_name
        new_lines.append(line)

    result = "\n".join(new_lines) + "\n"

    # Sanity check that all command-line options were actually processed.
    cmdline_options = set(args.enable) | set(args.disable)
    assert cmdline_options.issubset(
        options_seen
    ), "Could not find options in input file: " + ", ".join(
        sorted(cmdline_options - options_seen)
    )

    if args.output:
        with open(args.output, "w") as f:
            f.write(result)
    else:
        print(result)

    return 0


if __name__ == "__main__":
    sys.exit(main())
