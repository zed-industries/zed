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

"""Parse modules.cfg and dump its output either as ftmodule.h or a list of
base extensions.
"""

from __future__ import print_function

import argparse
import os
import re
import sys

# Expected input:
#
#  ...
#  FONT_MODULES += <name>
#  HINTING_MODULES += <name>
#  RASTER_MODULES += <name>
#  AUX_MODULES += <name>
#  BASE_EXTENSIONS += <name>
#  ...


def parse_modules_cfg(input_file):

    lists = {
        "FONT_MODULES": [],
        "HINTING_MODULES": [],
        "RASTER_MODULES": [],
        "AUX_MODULES": [],
        "BASE_EXTENSIONS": [],
    }

    for line in input_file.splitlines():
        line = line.rstrip()
        # Ignore empty lines and those that start with a comment.
        if not line or line[0] == "#":
            continue

        items = line.split()
        assert len(items) == 3 and items[1] == "+=", (
            "Unexpected input line [%s]" % line
        )
        assert items[0] in lists, (
            "Unexpected configuration variable name " + items[0]
        )

        lists[items[0]].append(items[2])

    return lists


def generate_ftmodule(lists):
    result = "/* This is a generated file. */\n"
    for driver in lists["FONT_MODULES"]:
        if driver == "sfnt":  # Special case for the sfnt 'driver'.
            result += "FT_USE_MODULE( FT_Module_Class, sfnt_module_class )\n"
            continue

        name = {
            "truetype": "tt",
            "type1": "t1",
            "cid": "t1cid",
            "type42": "t42",
            "winfonts": "winfnt",
        }.get(driver, driver)
        result += (
            "FT_USE_MODULE( FT_Driver_ClassRec, %s_driver_class )\n" % name
        )

    for module in lists["HINTING_MODULES"]:
        result += (
            "FT_USE_MODULE( FT_Module_Class, %s_module_class )\n" % module
        )

    for module in lists["RASTER_MODULES"]:
        names = {
            "raster": ("ft_raster1",),
            "smooth": ("ft_smooth",),
            "svg": ("ft_svg",),
            "sdf": ("ft_sdf", "ft_bitmap_sdf"),
        }.get(module)
        for name in names:
            result += (
                "FT_USE_MODULE( FT_Renderer_Class, %s_renderer_class )\n" % name
            )

    for module in lists["AUX_MODULES"]:
        if module in ("psaux", "psnames", "otvalid", "gxvalid"):
            name = {
                "gxvalid": "gxv",
                "otvalid": "otv",
            }.get(module, module)
            result += (
                "FT_USE_MODULE( FT_Module_Class, %s_module_class )\n" % name
            )

    result += "/* EOF */\n"
    return result


def generate_main_modules(lists):
    return "\n".join(
        lists["FONT_MODULES"]
        + lists["HINTING_MODULES"]
        + lists["RASTER_MODULES"]
    )


def generate_aux_modules(lists):
    return "\n".join(lists["AUX_MODULES"])


def generate_base_extensions(lists):
    return "\n".join(lists["BASE_EXTENSIONS"])


def main():
    parser = argparse.ArgumentParser(description=__doc__)

    parser.add_argument(
        "--format",
        required=True,
        choices=(
            "ftmodule.h",
            "main-modules",
            "aux-modules",
            "base-extensions-list",
        ),
        help="Select output format.",
    )

    parser.add_argument(
        "input",
        metavar="CONFIGURE_RAW",
        help="The input configure.raw file to parse.",
    )

    parser.add_argument("--output", help="Output file (default is stdout).")

    args = parser.parse_args()
    with open(args.input) as f:
        input_data = f.read()

    lists = parse_modules_cfg(input_data)

    if args.format == "ftmodule.h":
        result = generate_ftmodule(lists)
    elif args.format == "main-modules":
        result = generate_main_modules(lists)
    elif args.format == "aux-modules":
        result = generate_aux_modules(lists)
    elif args.format == "base-extensions-list":
        result = generate_base_extensions(lists)
    else:
        assert False, "Invalid output format!"

    if args.output:
        with open(args.output, "w") as f:
            f.write(result)
    else:
        print(result)
    return 0


if __name__ == "__main__":
    sys.exit(main())
