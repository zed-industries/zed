#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FONT_WEIGHT_FRACTION_BITS 6

typedef struct FixedPoint_FONT_WEIGHT_FRACTION_BITS {
  uint16_t value;
} FixedPoint_FONT_WEIGHT_FRACTION_BITS;

typedef struct FixedPoint_FONT_WEIGHT_FRACTION_BITS FontWeightFixedPoint;

typedef struct FontWeight {
  FontWeightFixedPoint _0;
} FontWeight;
#define FontWeight_NORMAL (FontWeight){ ._0 = (FontWeightFixedPoint){ .value = (400 << FONT_WEIGHT_FRACTION_BITS) } }

void root(struct FontWeight w);
