#ifndef BidiMirroringGlyph_D_H
#define BidiMirroringGlyph_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BidiPairedBracketType.d.h"




typedef struct BidiMirroringGlyph {
  OptionChar mirroring_glyph;
  bool mirrored;
  BidiPairedBracketType paired_bracket_type;
} BidiMirroringGlyph;

typedef struct BidiMirroringGlyph_option {union { BidiMirroringGlyph ok; }; bool is_ok; } BidiMirroringGlyph_option;



#endif // BidiMirroringGlyph_D_H
