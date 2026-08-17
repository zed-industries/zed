#ifndef CollatorMaxVariable_D_H
#define CollatorMaxVariable_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorMaxVariable {
  CollatorMaxVariable_Space = 0,
  CollatorMaxVariable_Punctuation = 1,
  CollatorMaxVariable_Symbol = 2,
  CollatorMaxVariable_Currency = 3,
} CollatorMaxVariable;

typedef struct CollatorMaxVariable_option {union { CollatorMaxVariable ok; }; bool is_ok; } CollatorMaxVariable_option;



#endif // CollatorMaxVariable_D_H
