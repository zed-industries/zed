#ifndef LineBreakWordOption_D_H
#define LineBreakWordOption_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LineBreakWordOption {
  LineBreakWordOption_Normal = 0,
  LineBreakWordOption_BreakAll = 1,
  LineBreakWordOption_KeepAll = 2,
} LineBreakWordOption;

typedef struct LineBreakWordOption_option {union { LineBreakWordOption ok; }; bool is_ok; } LineBreakWordOption_option;



#endif // LineBreakWordOption_D_H
