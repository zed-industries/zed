#ifndef LeadingAdjustment_D_H
#define LeadingAdjustment_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LeadingAdjustment {
  LeadingAdjustment_Auto = 0,
  LeadingAdjustment_None = 1,
  LeadingAdjustment_ToCased = 2,
} LeadingAdjustment;

typedef struct LeadingAdjustment_option {union { LeadingAdjustment ok; }; bool is_ok; } LeadingAdjustment_option;



#endif // LeadingAdjustment_D_H
