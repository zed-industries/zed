#ifndef CollatorNumericOrdering_D_H
#define CollatorNumericOrdering_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorNumericOrdering {
  CollatorNumericOrdering_Off = 0,
  CollatorNumericOrdering_On = 1,
} CollatorNumericOrdering;

typedef struct CollatorNumericOrdering_option {union { CollatorNumericOrdering ok; }; bool is_ok; } CollatorNumericOrdering_option;



#endif // CollatorNumericOrdering_D_H
