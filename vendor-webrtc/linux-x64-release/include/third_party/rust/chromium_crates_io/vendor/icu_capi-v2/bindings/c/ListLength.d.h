#ifndef ListLength_D_H
#define ListLength_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ListLength {
  ListLength_Wide = 0,
  ListLength_Short = 1,
  ListLength_Narrow = 2,
} ListLength;

typedef struct ListLength_option {union { ListLength ok; }; bool is_ok; } ListLength_option;



#endif // ListLength_D_H
