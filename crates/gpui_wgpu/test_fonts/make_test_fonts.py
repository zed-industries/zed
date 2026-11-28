# Builds the test fonts of this folder from the fonts of Fedora 45.
#
#   python3 make_test_fonts.py <folder>      (needs fontTools)
#
# Each is a subset of a font under the SIL Open Font License, renamed so that
# no system has its family and a test does not depend on the computer.
import sys
from fontTools import subset
from fontTools.ttLib import TTFont

out = sys.argv[1]
JOBS = [
    ("/usr/share/fonts/google-noto-color-emoji-fonts/Noto-COLRv1.ttf",
     "GpuiTestColorEmoji.ttf", "GPUI Test Color Emoji", "U+1F44D,U+2764,U+FE0F"),
    ("/usr/share/fonts/google-noto-emoji-fonts/NotoEmoji-Regular.ttf",
     "GpuiTestTextEmoji.ttf", "GPUI Test Text Emoji", "U+1F44D,U+2764"),
    ("/usr/share/fonts/google-noto-vf/NotoSansArabic[wght].ttf",
     "GpuiTestArabic.ttf", "GPUI Test Arabic", "U+0627,U+0628,U+0631,U+062D,U+0645"),
    ("/usr/share/fonts/vazirmatn-vf-fonts/Vazirmatn[wght].ttf",
     "GpuiTestLatinArabic.ttf", "GPUI Test Latin Arabic", "U+006D,U+0627,U+0628,U+0631,U+062D,U+0645"),
]
for source, name, family, unicodes in JOBS:
    options = subset.Options()
    options.layout_features = ["*"]
    options.name_IDs = [0, 1, 2, 3, 4, 5, 6]
    options.notdef_outline = True
    font = TTFont(source, recalcTimestamp=False)
    # A variable font is pinned to its default instance.
    if "fvar" in font:
        from fontTools.varLib import instancer
        font = instancer.instantiateVariableFont(
            font, {axis.axisTag: axis.defaultValue for axis in font["fvar"].axes}
        )
    subsetter = subset.Subsetter(options)
    subsetter.populate(unicodes=subset.parse_unicodes(unicodes))
    subsetter.subset(font)
    names = font["name"]
    postscript = family.replace(" ", "")
    for record in list(names.names):
        if record.nameID in (1, 4, 16, 21):
            record.string = family
        elif record.nameID == 3:
            record.string = postscript + "-Regular;fixture"
        elif record.nameID == 6:
            record.string = postscript + "-Regular"
        elif record.nameID == 2:
            record.string = "Regular"
    names.names = [r for r in names.names if r.nameID not in (16, 17, 21, 22, 25)]
    font.save(f"{out}/{name}")
    print(name, sorted(font.getBestCmap()), sorted(t for t in font.keys() if t in ("COLR", "CPAL", "glyf", "CBDT")))
