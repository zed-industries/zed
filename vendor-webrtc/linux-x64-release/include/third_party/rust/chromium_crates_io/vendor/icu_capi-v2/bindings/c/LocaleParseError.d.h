#ifndef LocaleParseError_D_H
#define LocaleParseError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LocaleParseError {
  LocaleParseError_Unknown = 0,
  LocaleParseError_Language = 1,
  LocaleParseError_Subtag = 2,
  LocaleParseError_Extension = 3,
} LocaleParseError;

typedef struct LocaleParseError_option {union { LocaleParseError ok; }; bool is_ok; } LocaleParseError_option;



#endif // LocaleParseError_D_H
