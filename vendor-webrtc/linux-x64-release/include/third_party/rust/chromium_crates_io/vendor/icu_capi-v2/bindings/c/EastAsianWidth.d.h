#ifndef EastAsianWidth_D_H
#define EastAsianWidth_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum EastAsianWidth {
  EastAsianWidth_Neutral = 0,
  EastAsianWidth_Ambiguous = 1,
  EastAsianWidth_Halfwidth = 2,
  EastAsianWidth_Fullwidth = 3,
  EastAsianWidth_Narrow = 4,
  EastAsianWidth_Wide = 5,
} EastAsianWidth;

typedef struct EastAsianWidth_option {union { EastAsianWidth ok; }; bool is_ok; } EastAsianWidth_option;



#endif // EastAsianWidth_D_H
