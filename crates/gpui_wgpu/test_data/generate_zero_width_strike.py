"""Build a tiny bitmap-strike test font that reproduces the swash `chunks(0)` panic.

The font carries EBLC/EBDT embedded bitmap strikes at bitDepth 1, indexFormat 3,
imageFormat 1 - the shape shipped by real monospace faces such as Anonymous Pro.
Glyph "A" has a normal 4x6 mask; glyph "space" has a 0x0 mask, which is what
drives swash's `src.chunks(((w * bits) + 7) / 8)` to a zero chunk size.

Outlines are plain rectangles authored here rather than derived from any existing
typeface, so the file carries no third-party licence. Deterministic, no external
data. Regenerate with `python3 generate_zero_width_strike.py`.
"""

import os

from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib.tables import E_B_D_T_, E_B_L_C_
from fontTools.ttLib.tables.BitmapGlyphMetrics import SmallGlyphMetrics
from fontTools.ttLib.tables.E_B_D_T_ import ebdt_bitmap_format_1
from fontTools.ttLib.tables.E_B_L_C_ import (
    SbitLineMetrics,
    Strike,
    eblc_index_sub_table_3,
)

UPM = 1000
PPEMS = (10, 11, 12, 13)
GLYPH_ORDER = [".notdef", "space", "A"]

# (ppem, "A" mask width, "A" mask height). "space" is 0x0 at every ppem, which is
# the degenerate case under test.
STRIKE_SIZES = {10: (4, 6), 11: (5, 7), 12: (6, 8), 13: (7, 9)}


def box(pen, width, height):
    pen.moveTo((100, 0))
    pen.lineTo((100 + width, 0))
    pen.lineTo((100 + width, height))
    pen.lineTo((100, height))
    pen.closePath()


def rows_to_bitmap(rows, width):
    """Pack a list of row-strings into 1-bit-per-pixel, byte-aligned per row."""
    if width == 0:
        return b""
    out = bytearray()
    for row in rows:
        bits = 0
        for x in range(width):
            bits = (bits << 1) | (1 if row[x] == "#" else 0)
        bits <<= (8 - (width % 8)) % 8
        nbytes = (width + 7) // 8
        out += bits.to_bytes(nbytes, "big")
    return bytes(out)


def make_bitmap_glyph(width, height):
    glyph = ebdt_bitmap_format_1(data=b"", ttFont=None)
    # imageFormat 1 carries small metrics, not big ones.
    metrics = SmallGlyphMetrics()
    metrics.height = height
    metrics.width = width
    metrics.BearingX = 0
    metrics.BearingY = height
    metrics.Advance = max(width, 1)
    glyph.metrics = metrics
    rows = ["#" * width for _ in range(height)]
    glyph.imageData = rows_to_bitmap(rows, width)
    return glyph


def build_strike(font, ppem):
    width, height = STRIKE_SIZES[ppem]

    strike = Strike()
    bitmap_size = strike.bitmapSizeTable
    bitmap_size.colorRef = 0
    bitmap_size.flags = 1
    bitmap_size.bitDepth = 1
    bitmap_size.ppemX = ppem
    bitmap_size.ppemY = ppem
    bitmap_size.hori = SbitLineMetrics()
    bitmap_size.vert = SbitLineMetrics()
    for line in (bitmap_size.hori, bitmap_size.vert):
        line.ascender = height
        line.descender = 0
        line.widthMax = max(width, 1)
        line.caretSlopeNumerator = 0
        line.caretSlopeDenominator = 1
        line.caretOffset = 0
        line.minOriginSB = 0
        line.minAdvanceSB = 0
        line.maxBeforeBL = 0
        line.minAfterBL = 0
        line.pad1 = 0
        line.pad2 = 0

    # indexFormat 3: short offsets. imageFormat 1: byte-aligned, small metrics.
    index_table = eblc_index_sub_table_3(data=b"", ttFont=font)
    index_table.indexFormat = 3
    index_table.imageFormat = 1
    index_table.imageSize = 0
    index_table.names = ["space", "A"]
    strike.indexSubTables = [index_table]
    return strike


def main():
    fb = FontBuilder(UPM, isTTF=True)
    fb.setupGlyphOrder(GLYPH_ORDER)
    # 0x6D maps to "A" so a family-loading path that requires "m" keeps the face.
    fb.setupCharacterMap({0x20: "space", 0x41: "A", 0x6D: "A"})

    glyphs = {}
    for name in GLYPH_ORDER:
        pen = TTGlyphPen(None)
        if name == "A":
            box(pen, 400, 700)
        glyphs[name] = pen.glyph()
    fb.setupGlyf(glyphs)
    fb.setupHorizontalMetrics(
        {".notdef": (600, 0), "space": (600, 0), "A": (600, 100)}
    )
    fb.setupHorizontalHeader(ascent=800, descent=-200)
    fb.setupNameTable(
        {
            "familyName": "GpuiBitmapStrikeTest",
            "styleName": "Regular",
            "psName": "GpuiBitmapStrikeTest-Regular",
        }
    )
    fb.setupOS2(sTypoAscender=800, sTypoDescender=-200, usWeightClass=400)
    fb.setupPost()

    font = fb.font

    eblc = E_B_L_C_.table_E_B_L_C_()
    ebdt = E_B_D_T_.table_E_B_D_T_()
    eblc.version = 2.0
    ebdt.version = 2.0
    eblc.strikes = []
    ebdt.strikeData = []

    for ppem in PPEMS:
        width, height = STRIKE_SIZES[ppem]
        eblc.strikes.append(build_strike(font, ppem))
        ebdt.strikeData.append(
            {
                # The degenerate glyph: a 0x0 mask at bitDepth 1.
                "space": make_bitmap_glyph(0, 0),
                "A": make_bitmap_glyph(width, height),
            }
        )

    font["EBLC"] = eblc
    font["EBDT"] = ebdt

    # fontTools stamps head.created/modified with the current time, which would
    # make every regeneration differ from the committed file. Pin them so the
    # output is byte-reproducible and a maintainer can diff it.
    font["head"].created = 0
    font["head"].modified = 0

    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "zero_width_bitmap_strike.ttf")
    font.save(out)
    print("wrote", out, os.path.getsize(out), "bytes")


if __name__ == "__main__":
    main()
