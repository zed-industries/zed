#!/usr/bin/env python3
import hashlib
import os
import pathlib
import shutil
import sys
import subprocess
from pathlib import Path
from sys import platform

# harfbuzz test files that will be ignored.
IGNORE_TESTS = [
    # We disable those here because we handle MacOS tests separately.
    "macos.tests",
    "coretext.tests",
    "directwrite.tests",
    "uniscribe.tests",
    "arabic-fallback-shaping.tests",
    "emoji-clusters.tests",
]

# harfbuzz test cases that will be ignored.
IGNORE_TEST_CASES = [
    # aots tests
    # in-house tests
    # --shaper=fallback is not supported.
    "simple_002",
    # `dfont` is not supported.
    "collections_001",
    "collections_002",
    "collections_003",
    # Face index out of bounds. ttf-parser doesn't permit this.
    "collections_006",
    # no `hhea` table.
    "indic_decompose_001",
    # Resource exhaustion tests with large outputs
    "morx_34_001",
    "morx_36_001",
    # Requires support for the ltag table.
    "macos_002",
    # Custom MacOS test. A shortened version of `macos_013`, but with `--show-flags`.
    # The shaped output is correct, but a buffer flag is there, even though there shouldn't be.
    # Wasn't able to figure out the problem, but the problem occurs during kerning. In harfbuzz, it uses the `drive`
    # method, while in rustybuzz it uses `state_machine_kerning` which seems to apply some different rules for the flags.
    "macos_122",

    # This custom test fails because harfbuzz uses a set digest in AAT to abort early
    # which we don't do yet. Is basically the same as morx_20_005, but with `--show-flags`
    "glyph_flags_002",
]


def check_hb_build(hb_shape_exe):
    if not hb_shape_exe.exists():
        print("Build harfbuzz first using:")
        print("    meson builddir")
        print("    ninja -Cbuilddir")
        exit(1)


def update_font_path(tests_name, fontfile):
    if not fontfile.startswith("/"):
        fontfile = fontfile.replace("../fonts/", "")
        return f"tests/fonts/{tests_name}/{fontfile}"  # relative to the root dir
    # macos tests contain absolute paths
    else:
        return fontfile


# Converts `U+0041,U+0078` or `0041,0078` into `\u{0041}\u{0078}`
def convert_unicodes(unicodes):
    text = ""
    for i, u in enumerate(unicodes.split(",")):
        if i > 0 and i % 10 == 0:
            text += "\\\n             "

        if u.startswith("U+"):
            u = u[2:]

        text += f"\\u{{{u}}}"

    return text


def prune_test_options(options):
    options = options.replace("--shaper=ot", "")
    options = options.replace(" --font-funcs=ft", "").replace("--font-funcs=ft", "")
    options = options.replace(" --font-funcs=ot", "").replace("--font-funcs=ot", "")
    # we don't support font scaling
    options = options.replace("--font-size=1000", "")
    # We don't support glyphs > u16
    options = options.replace("--not-found-variation-selector-glyph=1000000", "--not-found-variation-selector-glyph=64000")
    options = options.strip()
    return options


def convert_test_file(
    root_dir, hb_shape_exe, tests_name, file_name, idx, data, fonts, custom
):
    fontfile, options, unicodes, glyphs_expected = data.split(";")

    # MacOS tests contain hashes, remove them.
    if "@" in fontfile:
        fontfile, _ = fontfile.split("@")

    # Some fonts contain escaped spaces, remove them.
    fontfile = fontfile.replace("\\ ", " ")
    fontfile_rs = fontfile if custom else update_font_path(tests_name, fontfile)

    unicodes_rs = convert_unicodes(unicodes)

    test_name = file_name.replace(".tests", "").replace("-", "_") + f"_{idx:03d}"
    test_name = test_name.lower()

    if test_name in IGNORE_TEST_CASES:
        return ""

    options = prune_test_options(options)

    # We have to actually run hb-shape instead of using predefined results,
    # because hb sometimes stores results for freetype and not for embedded OpenType
    # engine, which we are using.
    if len(options) != 0:
        options_list = options.split(" ")
    else:
        options_list = []

    options_list.insert(0, str(hb_shape_exe))

    abs_font_path = (
        root_dir.joinpath(fontfile_rs)
        if custom
        else (
            root_dir.joinpath("test/shape/data")
            .joinpath(tests_name)
            .joinpath("tests")
            .joinpath(fontfile)
        )
    )

    options_list.append(str(abs_font_path))
    options_list.append(f"--unicodes={unicodes}")  # no need to escape it

    glyphs_expected = subprocess.run(
        options_list, check=True, stdout=subprocess.PIPE
    ).stdout.decode()

    glyphs_expected = glyphs_expected.strip()[
        1:-1
    ]  # remove leading and trailing whitespaces and `[..]`
    glyphs_expected = glyphs_expected.replace("|", "|\\\n         ")

    options_rs = options
    options_rs = options_rs.replace('"', '\\"')
    options_rs = options_rs.replace(" --single-par", "")

    if not fontfile.startswith("/"):
        fonts.add(os.path.split(fontfile_rs)[1])

    final_string = (
        f"#[test]\n"
        f"fn {test_name}() {{\n"
        f"    assert_eq!(\n"
        f"        shape(\n"
        f'            "{fontfile_rs}",\n'
        f'            "{unicodes_rs}",\n'
        f'            "{options_rs}",\n'
        f"        ),\n"
        f'        "{glyphs_expected}"\n'
        f"    );\n"
        f"}}\n"
        "\n"
    )

    if file_name == "macos.tests":
        final_string = '#[cfg(target_os = "macos")]\n' + final_string

    return final_string


# Returns an iterator over single test cases in a test file
def read_test_cases(path):
    with open(path, "r") as f:
        idx = 0
        for test in f.read().splitlines():
            # skip comments and empty lines
            if test.startswith("#") or len(test) == 0:
                continue

            yield idx, test
            idx += 1


# Convert all test files in a folder into Rust tests and write them into a file.
def convert_test_folder(root_dir, hb_shape_exe, tests_dir, tests_name, custom):
    files = sorted(os.listdir(tests_dir))
    files = [f for f in files if f.endswith(".tests") and f not in IGNORE_TESTS]

    return convert_test_files(
        root_dir, hb_shape_exe, tests_dir, tests_name, files, custom
    )


def convert_test_files(root_dir, hb_shape_exe, tests_dir, tests_name, files, custom):
    fonts = set()

    rust_code = (
        "// WARNING: this file was generated by ../scripts/gen-shaping-tests.py\n"
        "\n"
        "use crate::shape;\n"
        "\n"
    )

    for file in files:
        path = tests_dir / file

        for idx, test in read_test_cases(path):
            rust_code += convert_test_file(
                root_dir, hb_shape_exe, tests_name, file, idx + 1, test, fonts, custom
            )

    tests_name_snake_case = tests_name.replace("-", "_")
    with open(f"../tests/shaping/{tests_name_snake_case}.rs", "w") as f:
        f.write(rust_code)

    return fonts


def main():
    if len(sys.argv) != 2:
        print("Usage: gen-shaping-tests.py /path/to/harfbuzz-src")
        exit(1)

    hb_dir = Path(sys.argv[1])
    assert hb_dir.exists()

    rb_root = pathlib.Path(__file__).parent.parent

    # Check that harfbuzz was built.
    hb_shape_exe = hb_dir.joinpath("builddir/util/hb-shape")
    check_hb_build(hb_shape_exe)

    def to_hb_absolute(name):
        return hb_dir / f"test/shape/data/{name}/tests"

    # First we convert all harfbuzz tests that are not disabled
    test_dir_names = ["aots", "in-house", "text-rendering-tests"]
    for test_dir_name in test_dir_names:
        tests_dir = to_hb_absolute(test_dir_name)

        dir_used_fonts = convert_test_folder(
            hb_dir, hb_shape_exe, tests_dir, test_dir_name, False
        )
        for filename in dir_used_fonts:
            shutil.copy(
                hb_dir / f"test/shape/data/{test_dir_name}/fonts/{filename}",
                f"../tests/fonts/{test_dir_name}",
            )

    # Next we convert harfbuzz MacOS tests as well as custom MacOS tests, but only if the person running this
    # script is also running MacOS, otherwise they won't have the system fonts and
    # thus can't run the tests.
    if platform == "darwin":
        # macos.tests are not directly copied from harfbuzz, but instead from
        # `macos.tests` in this folder. See the README for more information.
        tests_dir = rb_root / "tests" / "custom"
        convert_test_files(
            rb_root, hb_shape_exe, tests_dir, "macos", ["macos.tests"], False
        )

    # Next we convert all of the custom tests (except MacOS tests). The test files themselves
    # are in the same format as the harfbuzz ones (i.e. they contain the arguments in the same form as
    # harfbuzz tests, but are instead stored in the rustybuzz folder. In addition to that, font paths
    # are relative to fonts stored inside of rustybuzz and not harfbuzz)
    convert_test_folder(
        rb_root, hb_shape_exe, rb_root / "tests" / "custom", "custom", True
    )


if __name__ == "__main__":
    main()
