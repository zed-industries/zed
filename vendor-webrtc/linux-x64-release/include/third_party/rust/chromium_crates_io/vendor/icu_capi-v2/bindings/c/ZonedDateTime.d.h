#ifndef ZonedDateTime_D_H
#define ZonedDateTime_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Date.d.h"
#include "Time.d.h"
#include "TimeZoneInfo.d.h"




typedef struct ZonedDateTime {
  Date* date;
  Time* time;
  TimeZoneInfo* zone;
} ZonedDateTime;

typedef struct ZonedDateTime_option {union { ZonedDateTime ok; }; bool is_ok; } ZonedDateTime_option;



#endif // ZonedDateTime_D_H
