#!/usr/bin/env python
#
# Generate Rust code given Decimals and formatting strings
#

import sys
import textwrap
from decimal import *

# Decimal strings to the testing formats
INPUTS = {
    "1": [
        "{}",
        "{:.1}",
        "{:.4}",
        "{:4.1}",
        "{:>4.1}",
        "{:<4.1}",
        "{:+05.1}",
    ],
    "123456": [
        "{}",
        "{:+05.1}",
        "{:.1}",
        "{:.4}",
        "{:4.1}",
        "{:>4.3}",
        "{:>4.4}",
        "{:<4.1}",
    ],
    "9999999": [
        "{:.4}",
        "{:.8}",
    ],
    "19073.97235939614856": [
        "{}",
        "{:+05.7}",
        "{:.3}",
        "{:0.4}",
        "{:4.1}",
        "{:>8.3}",
        "{:>8.4}",
        "{:<8.1}",
    ],
    "491326e-12": [
        "{}",
        "{:+015.7}",
        "{:.3}",
        "{:0.4}",
        "{:4.1}",
        "{:>8.3}",
        "{:>8.4}",
        "{:<8.1}",
    ],
}


def main():
    from argparse import ArgumentParser, FileType

    parser = ArgumentParser(description="Create test modules for src/impl_fmt.rs")
    parser.add_argument("-o", "--output", nargs="?", default="-", type=FileType("w"))
    args = parser.parse_args()

    for dec_str, formats in INPUTS.items():
        src = make_test_module_src(dec_str, formats)
        args.output.write(src)

    return 0


def make_test_module_src(dec_str: Decimal, formats: list[str]) -> str:
    """
    Return Rust module source which tests given decimal aginst given formats
    """
    dec = Decimal(dec_str)
    mod_name = "dec_%s" % gen_name_from_dec(dec_str)

    formats_and_outputs = [
        (fmt, gen_name_from_fmt(fmt), fmt.format(dec)) for fmt in formats
    ]

    max_len_name_and_fmt = max(
        len(fmt) + len(name) for fmt, name, _ in formats_and_outputs
    )

    lines = []
    for fmt, name, value in formats_and_outputs:
        spacer = " " * (max_len_name_and_fmt - (len(fmt) + len(name)) + 2)
        lines.append(f'impl_case!(fmt_{name}:{spacer}"{fmt}" => "{value}");')

    body = textwrap.indent("\n".join(lines), "    ")
    text = textwrap.dedent(
        f"""
        mod {mod_name} {{
            use super::*;

            fn test_input() -> BigDecimal {{
                "{dec_str}".parse().unwrap()
            }}

        %s
        }}
        """
    )
    return text % body


def gen_name_from_dec(dec_str: str) -> str:
    """Given decimal input string return valid function/module name"""
    return "".join(map_fmt_char_to_name_char(c) for c in dec_str)


def gen_name_from_fmt(fmt: str) -> str:
    """Given decimal input string return valid function/module name"""
    if fmt == "{}":
        return "default"
    else:
        assert fmt.startswith("{:") and fmt.endswith("}")
        return "".join(map_fmt_char_to_name_char(c) for c in fmt[2:-1])


def map_fmt_char_to_name_char(c: str) -> str:
    """Turn special characters into identifiers"""
    match c:
        case ".":
            return "d"
        case "+":
            return "p"
        case "-":
            return "n"
        case "<":
            return "l"
        case ">":
            return "r"
        case ":":
            return "c"
        case _:
            return str(c)


if __name__ == "__main__":
    sys.exit(main())
