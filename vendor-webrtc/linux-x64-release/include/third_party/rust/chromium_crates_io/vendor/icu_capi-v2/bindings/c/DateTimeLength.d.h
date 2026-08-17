#ifndef DateTimeLength_D_H
#define DateTimeLength_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DateTimeLength {
  DateTimeLength_Long = 0,
  DateTimeLength_Medium = 1,
  DateTimeLength_Short = 2,
} DateTimeLength;

typedef struct DateTimeLength_option {union { DateTimeLength ok; }; bool is_ok; } DateTimeLength_option;



#endif // DateTimeLength_D_H
