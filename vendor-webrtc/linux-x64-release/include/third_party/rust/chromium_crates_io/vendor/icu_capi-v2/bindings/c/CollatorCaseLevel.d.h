#ifndef CollatorCaseLevel_D_H
#define CollatorCaseLevel_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorCaseLevel {
  CollatorCaseLevel_Off = 0,
  CollatorCaseLevel_On = 1,
} CollatorCaseLevel;

typedef struct CollatorCaseLevel_option {union { CollatorCaseLevel ok; }; bool is_ok; } CollatorCaseLevel_option;



#endif // CollatorCaseLevel_D_H
