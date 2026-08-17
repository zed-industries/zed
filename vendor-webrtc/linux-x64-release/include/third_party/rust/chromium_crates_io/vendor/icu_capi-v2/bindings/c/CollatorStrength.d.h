#ifndef CollatorStrength_D_H
#define CollatorStrength_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorStrength {
  CollatorStrength_Primary = 0,
  CollatorStrength_Secondary = 1,
  CollatorStrength_Tertiary = 2,
  CollatorStrength_Quaternary = 3,
  CollatorStrength_Identical = 4,
} CollatorStrength;

typedef struct CollatorStrength_option {union { CollatorStrength ok; }; bool is_ok; } CollatorStrength_option;



#endif // CollatorStrength_D_H
