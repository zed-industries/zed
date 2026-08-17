#ifndef PluralCategories_D_H
#define PluralCategories_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct PluralCategories {
  bool zero;
  bool one;
  bool two;
  bool few;
  bool many;
  bool other;
} PluralCategories;

typedef struct PluralCategories_option {union { PluralCategories ok; }; bool is_ok; } PluralCategories_option;



#endif // PluralCategories_D_H
