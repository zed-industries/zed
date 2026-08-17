#ifndef LocaleFallbackPriority_D_H
#define LocaleFallbackPriority_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LocaleFallbackPriority {
  LocaleFallbackPriority_Language = 0,
  LocaleFallbackPriority_Region = 1,
} LocaleFallbackPriority;

typedef struct LocaleFallbackPriority_option {union { LocaleFallbackPriority ok; }; bool is_ok; } LocaleFallbackPriority_option;



#endif // LocaleFallbackPriority_D_H
