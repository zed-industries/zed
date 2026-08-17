#ifndef TitlecaseOptionsV1_D_H
#define TitlecaseOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "LeadingAdjustment.d.h"
#include "TrailingCase.d.h"




typedef struct TitlecaseOptionsV1 {
  LeadingAdjustment_option leading_adjustment;
  TrailingCase_option trailing_case;
} TitlecaseOptionsV1;

typedef struct TitlecaseOptionsV1_option {union { TitlecaseOptionsV1 ok; }; bool is_ok; } TitlecaseOptionsV1_option;



#endif // TitlecaseOptionsV1_D_H
