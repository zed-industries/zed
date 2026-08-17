#ifndef DisplayNamesFallback_D_H
#define DisplayNamesFallback_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DisplayNamesFallback {
  DisplayNamesFallback_Code = 0,
  DisplayNamesFallback_None = 1,
} DisplayNamesFallback;

typedef struct DisplayNamesFallback_option {union { DisplayNamesFallback ok; }; bool is_ok; } DisplayNamesFallback_option;



#endif // DisplayNamesFallback_D_H
