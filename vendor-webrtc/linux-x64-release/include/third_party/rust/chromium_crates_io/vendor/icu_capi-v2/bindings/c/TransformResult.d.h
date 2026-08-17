#ifndef TransformResult_D_H
#define TransformResult_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum TransformResult {
  TransformResult_Modified = 0,
  TransformResult_Unmodified = 1,
} TransformResult;

typedef struct TransformResult_option {union { TransformResult ok; }; bool is_ok; } TransformResult_option;



#endif // TransformResult_D_H
