#ifndef TimeZoneVariant_D_H
#define TimeZoneVariant_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum TimeZoneVariant {
  TimeZoneVariant_Standard = 0,
  TimeZoneVariant_Daylight = 1,
} TimeZoneVariant;

typedef struct TimeZoneVariant_option {union { TimeZoneVariant ok; }; bool is_ok; } TimeZoneVariant_option;



#endif // TimeZoneVariant_D_H
