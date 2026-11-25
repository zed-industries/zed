# GPUI Variable Font Fixture

`GPUIVariableFontFixture.ttf` is a renamed, unhinted subset of Roboto Flex 3.200
containing the characters in `Start End`. It retains all 13 variable axes and
68–76 `gvar` tuples per visible glyph. These exceed ttf-parser's default limit
of 32 tuples without `gvar-alloc`, causing SVG text to disappear.

The original font and license are from
[google/fonts at commit 3e5667764ccfea32ce0c3d58a3a4756de890adc1](https://github.com/google/fonts/tree/3e5667764ccfea32ce0c3d58a3a4756de890adc1/ofl/robotoflex).
The original font's SHA-256 is
`9b523f7d82593df0107173849ebb8c817471a1df4b4fb2c3cbf40cfd810c8281`.

To recreate the subset with fontTools 4.61.1, save the original as
`RobotoFlex.ttf`, then run:

```text
python -m fontTools.subset RobotoFlex.ttf --text="Start End" --no-hinting --no-recalc-timestamp --output-file=GPUIVariableFontFixture.ttf
```

Rename the subset's font name records: family (ID 1) to
`GPUI Variable Font Fixture`, full name (ID 4) to
`GPUI Variable Font Fixture Regular`, and PostScript name (ID 6) to
`GPUIVariableFontFixture-Regular`.

Copyright 2017 The Roboto Flex Project Authors. The font remains licensed under
the SIL Open Font License 1.1; see [OFL.txt](OFL.txt).
