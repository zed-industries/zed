#ifndef DecimalGroupingStrategy_D_H
#define DecimalGroupingStrategy_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DecimalGroupingStrategy {
  DecimalGroupingStrategy_Auto = 0,
  DecimalGroupingStrategy_Never = 1,
  DecimalGroupingStrategy_Always = 2,
  DecimalGroupingStrategy_Min2 = 3,
} DecimalGroupingStrategy;

typedef struct DecimalGroupingStrategy_option {union { DecimalGroupingStrategy ok; }; bool is_ok; } DecimalGroupingStrategy_option;



#endif // DecimalGroupingStrategy_D_H
