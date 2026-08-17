#ifndef ZonedIsoDateTime_D_H
#define ZonedIsoDateTime_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "IsoDate.d.h"
#include "Time.d.h"
#include "TimeZoneInfo.d.h"




typedef struct ZonedIsoDateTime {
  IsoDate* date;
  Time* time;
  TimeZoneInfo* zone;
} ZonedIsoDateTime;

typedef struct ZonedIsoDateTime_option {union { ZonedIsoDateTime ok; }; bool is_ok; } ZonedIsoDateTime_option;



#endif // ZonedIsoDateTime_D_H
