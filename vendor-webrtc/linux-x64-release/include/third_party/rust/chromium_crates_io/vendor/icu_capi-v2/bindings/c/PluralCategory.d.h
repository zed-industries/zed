#ifndef PluralCategory_D_H
#define PluralCategory_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum PluralCategory {
  PluralCategory_Zero = 0,
  PluralCategory_One = 1,
  PluralCategory_Two = 2,
  PluralCategory_Few = 3,
  PluralCategory_Many = 4,
  PluralCategory_Other = 5,
} PluralCategory;

typedef struct PluralCategory_option {union { PluralCategory ok; }; bool is_ok; } PluralCategory_option;



#endif // PluralCategory_D_H
