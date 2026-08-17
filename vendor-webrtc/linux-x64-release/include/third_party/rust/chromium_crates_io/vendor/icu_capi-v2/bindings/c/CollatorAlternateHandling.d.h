#ifndef CollatorAlternateHandling_D_H
#define CollatorAlternateHandling_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorAlternateHandling {
  CollatorAlternateHandling_NonIgnorable = 0,
  CollatorAlternateHandling_Shifted = 1,
} CollatorAlternateHandling;

typedef struct CollatorAlternateHandling_option {union { CollatorAlternateHandling ok; }; bool is_ok; } CollatorAlternateHandling_option;



#endif // CollatorAlternateHandling_D_H
