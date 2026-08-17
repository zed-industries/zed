#ifndef LineBreakStrictness_D_H
#define LineBreakStrictness_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LineBreakStrictness {
  LineBreakStrictness_Loose = 0,
  LineBreakStrictness_Normal = 1,
  LineBreakStrictness_Strict = 2,
  LineBreakStrictness_Anywhere = 3,
} LineBreakStrictness;

typedef struct LineBreakStrictness_option {union { LineBreakStrictness ok; }; bool is_ok; } LineBreakStrictness_option;



#endif // LineBreakStrictness_D_H
