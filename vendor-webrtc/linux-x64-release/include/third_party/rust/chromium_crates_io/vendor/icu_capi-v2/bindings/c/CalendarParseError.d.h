#ifndef CalendarParseError_D_H
#define CalendarParseError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CalendarParseError {
  CalendarParseError_Unknown = 0,
  CalendarParseError_InvalidSyntax = 1,
  CalendarParseError_OutOfRange = 2,
  CalendarParseError_MissingFields = 3,
  CalendarParseError_UnknownCalendar = 4,
} CalendarParseError;

typedef struct CalendarParseError_option {union { CalendarParseError ok; }; bool is_ok; } CalendarParseError_option;



#endif // CalendarParseError_D_H
