#ifndef FixedDecimalSignedRoundingMode_D_H
#define FixedDecimalSignedRoundingMode_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FixedDecimalSignedRoundingMode {
  FixedDecimalSignedRoundingMode_Expand = 0,
  FixedDecimalSignedRoundingMode_Trunc = 1,
  FixedDecimalSignedRoundingMode_HalfExpand = 2,
  FixedDecimalSignedRoundingMode_HalfTrunc = 3,
  FixedDecimalSignedRoundingMode_HalfEven = 4,
  FixedDecimalSignedRoundingMode_Ceil = 5,
  FixedDecimalSignedRoundingMode_Floor = 6,
  FixedDecimalSignedRoundingMode_HalfCeil = 7,
  FixedDecimalSignedRoundingMode_HalfFloor = 8,
} FixedDecimalSignedRoundingMode;

typedef struct FixedDecimalSignedRoundingMode_option {union { FixedDecimalSignedRoundingMode ok; }; bool is_ok; } FixedDecimalSignedRoundingMode_option;



#endif // FixedDecimalSignedRoundingMode_D_H
