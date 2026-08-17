#ifndef LocaleFallbackConfig_D_H
#define LocaleFallbackConfig_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "LocaleFallbackPriority.d.h"




typedef struct LocaleFallbackConfig {
  LocaleFallbackPriority priority;
} LocaleFallbackConfig;

typedef struct LocaleFallbackConfig_option {union { LocaleFallbackConfig ok; }; bool is_ok; } LocaleFallbackConfig_option;



#endif // LocaleFallbackConfig_D_H
