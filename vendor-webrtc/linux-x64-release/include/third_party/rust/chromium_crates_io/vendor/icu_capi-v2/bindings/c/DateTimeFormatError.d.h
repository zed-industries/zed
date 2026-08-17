#ifndef DateTimeFormatError_D_H
#define DateTimeFormatError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DateTimeFormatError {
  DateTimeFormatError_Unknown = 0,
  DateTimeFormatError_MissingInputField = 1,
  DateTimeFormatError_ZoneInfoMissingFields = 2,
  DateTimeFormatError_InvalidEra = 3,
  DateTimeFormatError_InvalidMonthCode = 4,
  DateTimeFormatError_InvalidCyclicYear = 5,
  DateTimeFormatError_NamesNotLoaded = 16,
  DateTimeFormatError_DecimalFormatterNotLoaded = 17,
  DateTimeFormatError_UnsupportedField = 18,
} DateTimeFormatError;

typedef struct DateTimeFormatError_option {union { DateTimeFormatError ok; }; bool is_ok; } DateTimeFormatError_option;



#endif // DateTimeFormatError_D_H
