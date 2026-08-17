#ifndef TimeZoneAndCanonical_D_H
#define TimeZoneAndCanonical_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TimeZone.d.h"




typedef struct TimeZoneAndCanonical {
  TimeZone* time_zone;
  DiplomatStringView canonical;
} TimeZoneAndCanonical;

typedef struct TimeZoneAndCanonical_option {union { TimeZoneAndCanonical ok; }; bool is_ok; } TimeZoneAndCanonical_option;



#endif // TimeZoneAndCanonical_D_H
