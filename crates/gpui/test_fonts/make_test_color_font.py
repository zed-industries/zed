#!/usr/bin/env python3
"""Generate GpuiTestColor.ttf: one COLRv0 color glyph ("A").

Tests use this to verify that color-glyph capability is detected from font
tables (COLR/CPAL), not from a font's PostScript name.

Run from this directory: `python3 make_test_color_font.py`.
"""

from fontTools.colorLib.builder import buildCOLR, buildCPAL
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen

UPEM = 1000
OUT = "GpuiTestColor.ttf"


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
    builder.setupGlyphOrder([".notdef", "A", "layer0", "space"])
    builder.setupCharacterMap({0x41: "A", 0x6D: "space"})
    builder.setupGlyf(
        {
            ".notdef": glyph(),
            "A": glyph(),
            "layer0": glyph([[(200, 0), (800, 0), (800, 600), (200, 600)]]),
            "space": glyph(),
        }
    )
    builder.setupHorizontalMetrics(
        {
            ".notdef": (500, 0),
            "A": (600, 0),
            "layer0": (600, 0),
            "space": (600, 0),
        }
    )
    builder.setupHorizontalHeader(ascent=800, descent=-200)
    builder.setupNameTable(
        {
            "familyName": "GpuiTestColor",
            "styleName": "Regular",
            "uniqueFontIdentifier": "GpuiTestColor-Regular",
            "fullName": "GpuiTestColor",
            "psName": "GpuiTestColor-Regular",
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

    builder.font["CPAL"] = buildCPAL([[(1.0, 0.0, 0.0, 1.0)]])
    builder.font["COLR"] = buildCOLR({"A": [("layer0", 0)]}, version=0)
    builder.save(OUT)


if __name__ == "__main__":
    main()
