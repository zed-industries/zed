"""Build a tiny variable test font: one wght axis, one visible glyph whose
outline and advance both change with weight. Deterministic, no external data."""
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.varLib import build
from fontTools.designspaceLib import DesignSpaceDocument, AxisDescriptor, SourceDescriptor
import os

UPM = 1000

def box(pen, thickness):
    # A filled rectangle whose width tracks `thickness`, so ink area scales with weight.
    pen.moveTo((100, 0)); pen.lineTo((100 + thickness, 0))
    pen.lineTo((100 + thickness, 700)); pen.lineTo((100, 700)); pen.closePath()

def master(weight, thickness, advance, path):
    fb = FontBuilder(UPM, isTTF=True)
    order = [".notdef", "A"]
    fb.setupGlyphOrder(order)
    fb.setupCharacterMap({0x41: "A", 0x6D: "A", 0x4E2D: "A"})  # 0x6D: load_family drops faces whose charmap has no "m". 0x4E2D: a char the bundled test primary lacks, so fallback is exercised  # 0x6D: load_family drops faces whose charmap has no "m"
    glyphs = {}
    for name in order:
        pen = TTGlyphPen(None)
        if name == "A":
            box(pen, thickness)
        glyphs[name] = pen.glyph()
    fb.setupGlyf(glyphs)
    fb.setupHorizontalMetrics({".notdef": (advance, 0), "A": (advance, 100)})
    fb.setupHorizontalHeader(ascent=800, descent=-200)
    fb.setupNameTable({"familyName": "GpuiVarTest", "styleName": "Regular",
                       "psName": "GpuiVarTest-Regular"})
    fb.setupOS2(sTypoAscender=800, sTypoDescender=-200, usWeightClass=weight)
    fb.setupPost()
    fb.save(path)
    return path

ds = DesignSpaceDocument()
axis = AxisDescriptor()
axis.minimum, axis.default, axis.maximum = 100, 400, 900
axis.name, axis.tag = "Weight", "wght"
axis.map = [(100, 100), (400, 400), (900, 900)]
ds.addAxis(axis)
for w, thick, adv in ((100, 40, 500), (400, 120, 560), (900, 300, 700)):
    p = master(w, thick, adv, f"/tmp/vf-master-{w}.ttf")
    s = SourceDescriptor(); s.path = p; s.location = {"Weight": w}
    if w == 400: s.copyLib = s.copyInfo = s.copyGroups = s.copyFeatures = True
    ds.addSource(s)

vf, _, _ = build(ds)
out = "/media/ravi/GamesRavi/oss/GpuiVarTest.ttf"
vf.save(out)
print("wrote", out, os.path.getsize(out), "bytes")
