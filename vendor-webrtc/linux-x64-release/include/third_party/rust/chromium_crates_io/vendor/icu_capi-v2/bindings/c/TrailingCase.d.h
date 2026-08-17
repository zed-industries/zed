#ifndef TrailingCase_D_H
#define TrailingCase_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum TrailingCase {
  TrailingCase_Lower = 0,
  TrailingCase_Unchanged = 1,
} TrailingCase;

typedef struct TrailingCase_option {union { TrailingCase ok; }; bool is_ok; } TrailingCase_option;



#endif // TrailingCase_D_H
