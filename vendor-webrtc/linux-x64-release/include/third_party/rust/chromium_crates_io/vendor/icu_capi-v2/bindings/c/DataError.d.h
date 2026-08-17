#ifndef DataError_D_H
#define DataError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DataError {
  DataError_Unknown = 0,
  DataError_MarkerNotFound = 1,
  DataError_IdentifierNotFound = 2,
  DataError_InvalidRequest = 3,
  DataError_InconsistentData = 4,
  DataError_Downcast = 5,
  DataError_Deserialize = 6,
  DataError_Custom = 7,
  DataError_Io = 8,
} DataError;

typedef struct DataError_option {union { DataError ok; }; bool is_ok; } DataError_option;



#endif // DataError_D_H
