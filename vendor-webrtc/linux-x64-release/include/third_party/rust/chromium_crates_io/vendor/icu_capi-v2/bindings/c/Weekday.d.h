#ifndef Weekday_D_H
#define Weekday_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum Weekday {
  Weekday_Monday = 1,
  Weekday_Tuesday = 2,
  Weekday_Wednesday = 3,
  Weekday_Thursday = 4,
  Weekday_Friday = 5,
  Weekday_Saturday = 6,
  Weekday_Sunday = 7,
} Weekday;

typedef struct Weekday_option {union { Weekday ok; }; bool is_ok; } Weekday_option;



#endif // Weekday_D_H
