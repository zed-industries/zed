#ifndef FixedDecimalSign_D_H
#define FixedDecimalSign_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FixedDecimalSign {
  FixedDecimalSign_None = 0,
  FixedDecimalSign_Negative = 1,
  FixedDecimalSign_Positive = 2,
} FixedDecimalSign;

typedef struct FixedDecimalSign_option {union { FixedDecimalSign ok; }; bool is_ok; } FixedDecimalSign_option;



#endif // FixedDecimalSign_D_H
