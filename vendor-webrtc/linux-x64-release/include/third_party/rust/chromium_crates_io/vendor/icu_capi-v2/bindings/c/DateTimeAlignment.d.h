#ifndef DateTimeAlignment_D_H
#define DateTimeAlignment_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DateTimeAlignment {
  DateTimeAlignment_Auto = 0,
  DateTimeAlignment_Column = 1,
} DateTimeAlignment;

typedef struct DateTimeAlignment_option {union { DateTimeAlignment ok; }; bool is_ok; } DateTimeAlignment_option;



#endif // DateTimeAlignment_D_H
