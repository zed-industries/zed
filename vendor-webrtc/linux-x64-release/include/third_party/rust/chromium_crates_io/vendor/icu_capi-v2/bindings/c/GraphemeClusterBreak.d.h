#ifndef GraphemeClusterBreak_D_H
#define GraphemeClusterBreak_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum GraphemeClusterBreak {
  GraphemeClusterBreak_Other = 0,
  GraphemeClusterBreak_Control = 1,
  GraphemeClusterBreak_CR = 2,
  GraphemeClusterBreak_Extend = 3,
  GraphemeClusterBreak_L = 4,
  GraphemeClusterBreak_LF = 5,
  GraphemeClusterBreak_LV = 6,
  GraphemeClusterBreak_LVT = 7,
  GraphemeClusterBreak_T = 8,
  GraphemeClusterBreak_V = 9,
  GraphemeClusterBreak_SpacingMark = 10,
  GraphemeClusterBreak_Prepend = 11,
  GraphemeClusterBreak_RegionalIndicator = 12,
  GraphemeClusterBreak_EBase = 13,
  GraphemeClusterBreak_EBaseGAZ = 14,
  GraphemeClusterBreak_EModifier = 15,
  GraphemeClusterBreak_GlueAfterZwj = 16,
  GraphemeClusterBreak_ZWJ = 17,
} GraphemeClusterBreak;

typedef struct GraphemeClusterBreak_option {union { GraphemeClusterBreak ok; }; bool is_ok; } GraphemeClusterBreak_option;



#endif // GraphemeClusterBreak_D_H
