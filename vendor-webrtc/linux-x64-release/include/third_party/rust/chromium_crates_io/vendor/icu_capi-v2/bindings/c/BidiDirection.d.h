#ifndef BidiDirection_D_H
#define BidiDirection_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BidiDirection {
  BidiDirection_Ltr = 0,
  BidiDirection_Rtl = 1,
  BidiDirection_Mixed = 2,
} BidiDirection;

typedef struct BidiDirection_option {union { BidiDirection ok; }; bool is_ok; } BidiDirection_option;



#endif // BidiDirection_D_H
