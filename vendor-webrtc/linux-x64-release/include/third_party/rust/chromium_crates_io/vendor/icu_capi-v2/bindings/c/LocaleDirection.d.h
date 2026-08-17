#ifndef LocaleDirection_D_H
#define LocaleDirection_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LocaleDirection {
  LocaleDirection_LeftToRight = 0,
  LocaleDirection_RightToLeft = 1,
  LocaleDirection_Unknown = 2,
} LocaleDirection;

typedef struct LocaleDirection_option {union { LocaleDirection ok; }; bool is_ok; } LocaleDirection_option;



#endif // LocaleDirection_D_H
