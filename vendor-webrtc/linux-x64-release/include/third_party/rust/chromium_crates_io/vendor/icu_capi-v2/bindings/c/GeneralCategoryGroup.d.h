#ifndef GeneralCategoryGroup_D_H
#define GeneralCategoryGroup_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct GeneralCategoryGroup {
  uint32_t mask;
} GeneralCategoryGroup;

typedef struct GeneralCategoryGroup_option {union { GeneralCategoryGroup ok; }; bool is_ok; } GeneralCategoryGroup_option;



#endif // GeneralCategoryGroup_D_H
