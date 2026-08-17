#ifndef DateTimeFormatterLoadError_D_H
#define DateTimeFormatterLoadError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DateTimeFormatterLoadError {
  DateTimeFormatterLoadError_Unknown = 0,
  DateTimeFormatterLoadError_UnsupportedLength = 2051,
  DateTimeFormatterLoadError_DuplicateField = 2057,
  DateTimeFormatterLoadError_TypeTooSpecific = 2058,
  DateTimeFormatterLoadError_DataMarkerNotFound = 1,
  DateTimeFormatterLoadError_DataIdentifierNotFound = 2,
  DateTimeFormatterLoadError_DataInvalidRequest = 3,
  DateTimeFormatterLoadError_DataInconsistentData = 4,
  DateTimeFormatterLoadError_DataDowncast = 5,
  DateTimeFormatterLoadError_DataDeserialize = 6,
  DateTimeFormatterLoadError_DataCustom = 7,
  DateTimeFormatterLoadError_DataIo = 8,
} DateTimeFormatterLoadError;

typedef struct DateTimeFormatterLoadError_option {union { DateTimeFormatterLoadError ok; }; bool is_ok; } DateTimeFormatterLoadError_option;



#endif // DateTimeFormatterLoadError_D_H
