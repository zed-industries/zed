#ifndef DisplayNamesStyle_D_H
#define DisplayNamesStyle_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DisplayNamesStyle {
  DisplayNamesStyle_Narrow = 0,
  DisplayNamesStyle_Short = 1,
  DisplayNamesStyle_Long = 2,
  DisplayNamesStyle_Menu = 3,
} DisplayNamesStyle;

typedef struct DisplayNamesStyle_option {union { DisplayNamesStyle ok; }; bool is_ok; } DisplayNamesStyle_option;



#endif // DisplayNamesStyle_D_H
