#!/usr/bin/env python3
"""Generate GpuiTestNoM.ttf: one PUA glyph, no Latin glyphs.

GPUI measures an em with the width of the 'm' glyph. This font deliberately
has no 'm', so tests can verify that such fonts still load, resolve, shape
their own glyphs, and measure with `font_size` as the em fallback.

Run from this directory: `python3 make_test_font.py`.
"""

from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen

UPEM = 1000
OUT = "GpuiTestNoM.ttf"


def glyph(contours=()):
    pen = TTGlyphPen(None)
    for contour in contours:
        pen.moveTo(contour[0])
        for point in contour[1:]:
            pen.lineTo(point)
        pen.closePath()
    return pen.glyph()


def main():
    builder = FontBuilder(UPEM, isTTF=True)
    builder.setupGlyphOrder([".notdef", "uniE000"])
    builder.setupCharacterMap({0xE000: "uniE000"})
    builder.setupGlyf(
        {
            ".notdef": glyph(),
            "uniE000": glyph([[(100, 0), (900, 0), (900, 800), (100, 800)]]),
        }
    )
    builder.setupHorizontalMetrics(
        {
            ".notdef": (500, 0),
            "uniE000": (1000, 0),
        }
    )
    builder.setupHorizontalHeader(ascent=800, descent=-200)
    builder.setupNameTable(
        {
            "familyName": "GpuiTestNoM",
            "styleName": "Regular",
            "uniqueFontIdentifier": "GpuiTestNoM-Regular",
            "fullName": "GpuiTestNoM",
            "psName": "GpuiTestNoM-Regular",
            "version": "Version 1.0",
        }
    )
    builder.setupOS2(
        sTypoAscender=800,
        sTypoDescender=-200,
        usWinAscent=800,
        usWinDescent=200,
    )
    builder.setupPost()
    builder.save(OUT)


if __name__ == "__main__":
    main()
