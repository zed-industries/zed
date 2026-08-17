#ifndef SegmenterWordType_D_H
#define SegmenterWordType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum SegmenterWordType {
  SegmenterWordType_None = 0,
  SegmenterWordType_Number = 1,
  SegmenterWordType_Letter = 2,
} SegmenterWordType;

typedef struct SegmenterWordType_option {union { SegmenterWordType ok; }; bool is_ok; } SegmenterWordType_option;



#endif // SegmenterWordType_D_H
