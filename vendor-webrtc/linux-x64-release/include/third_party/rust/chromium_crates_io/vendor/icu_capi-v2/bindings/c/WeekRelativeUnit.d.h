#ifndef WeekRelativeUnit_D_H
#define WeekRelativeUnit_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum WeekRelativeUnit {
  WeekRelativeUnit_Previous = 0,
  WeekRelativeUnit_Current = 1,
  WeekRelativeUnit_Next = 2,
} WeekRelativeUnit;

typedef struct WeekRelativeUnit_option {union { WeekRelativeUnit ok; }; bool is_ok; } WeekRelativeUnit_option;



#endif // WeekRelativeUnit_D_H
