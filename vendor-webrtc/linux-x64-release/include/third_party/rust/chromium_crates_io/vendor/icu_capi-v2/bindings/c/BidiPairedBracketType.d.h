#ifndef BidiPairedBracketType_D_H
#define BidiPairedBracketType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BidiPairedBracketType {
  BidiPairedBracketType_Open = 0,
  BidiPairedBracketType_Close = 1,
  BidiPairedBracketType_None = 2,
} BidiPairedBracketType;

typedef struct BidiPairedBracketType_option {union { BidiPairedBracketType ok; }; bool is_ok; } BidiPairedBracketType_option;



#endif // BidiPairedBracketType_D_H
