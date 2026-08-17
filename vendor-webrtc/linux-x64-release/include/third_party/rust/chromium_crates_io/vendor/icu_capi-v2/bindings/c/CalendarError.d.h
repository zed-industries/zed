#ifndef CalendarError_D_H
#define CalendarError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CalendarError {
  CalendarError_Unknown = 0,
  CalendarError_OutOfRange = 1,
  CalendarError_UnknownEra = 2,
  CalendarError_UnknownMonthCode = 3,
} CalendarError;

typedef struct CalendarError_option {union { CalendarError ok; }; bool is_ok; } CalendarError_option;



#endif // CalendarError_D_H
