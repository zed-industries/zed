#ifndef DateTimeMismatchedCalendarError_D_H
#define DateTimeMismatchedCalendarError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AnyCalendarKind.d.h"




typedef struct DateTimeMismatchedCalendarError {
  AnyCalendarKind this_kind;
  AnyCalendarKind_option date_kind;
} DateTimeMismatchedCalendarError;

typedef struct DateTimeMismatchedCalendarError_option {union { DateTimeMismatchedCalendarError ok; }; bool is_ok; } DateTimeMismatchedCalendarError_option;



#endif // DateTimeMismatchedCalendarError_D_H
