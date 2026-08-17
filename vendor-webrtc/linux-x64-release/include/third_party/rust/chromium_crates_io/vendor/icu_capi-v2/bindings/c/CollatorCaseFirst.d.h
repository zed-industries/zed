#ifndef CollatorCaseFirst_D_H
#define CollatorCaseFirst_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorCaseFirst {
  CollatorCaseFirst_Off = 0,
  CollatorCaseFirst_Lower = 1,
  CollatorCaseFirst_Upper = 2,
} CollatorCaseFirst;

typedef struct CollatorCaseFirst_option {union { CollatorCaseFirst ok; }; bool is_ok; } CollatorCaseFirst_option;



#endif // CollatorCaseFirst_D_H
