#ifndef UtcOffsets_D_H
#define UtcOffsets_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "UtcOffset.d.h"




typedef struct UtcOffsets {
  UtcOffset* standard;
  UtcOffset* daylight;
} UtcOffsets;

typedef struct UtcOffsets_option {union { UtcOffsets ok; }; bool is_ok; } UtcOffsets_option;



#endif // UtcOffsets_D_H
