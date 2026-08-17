#ifndef YearStyle_D_H
#define YearStyle_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum YearStyle {
  YearStyle_Auto = 0,
  YearStyle_Full = 1,
  YearStyle_WithEra = 2,
} YearStyle;

typedef struct YearStyle_option {union { YearStyle ok; }; bool is_ok; } YearStyle_option;



#endif // YearStyle_D_H
