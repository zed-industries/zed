#ifndef LineBreakOptionsV2_D_H
#define LineBreakOptionsV2_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "LineBreakStrictness.d.h"
#include "LineBreakWordOption.d.h"




typedef struct LineBreakOptionsV2 {
  LineBreakStrictness_option strictness;
  LineBreakWordOption_option word_option;
} LineBreakOptionsV2;

typedef struct LineBreakOptionsV2_option {union { LineBreakOptionsV2 ok; }; bool is_ok; } LineBreakOptionsV2_option;



#endif // LineBreakOptionsV2_D_H
