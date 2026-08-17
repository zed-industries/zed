#ifndef BidiClass_D_H
#define BidiClass_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BidiClass {
  BidiClass_LeftToRight = 0,
  BidiClass_RightToLeft = 1,
  BidiClass_EuropeanNumber = 2,
  BidiClass_EuropeanSeparator = 3,
  BidiClass_EuropeanTerminator = 4,
  BidiClass_ArabicNumber = 5,
  BidiClass_CommonSeparator = 6,
  BidiClass_ParagraphSeparator = 7,
  BidiClass_SegmentSeparator = 8,
  BidiClass_WhiteSpace = 9,
  BidiClass_OtherNeutral = 10,
  BidiClass_LeftToRightEmbedding = 11,
  BidiClass_LeftToRightOverride = 12,
  BidiClass_ArabicLetter = 13,
  BidiClass_RightToLeftEmbedding = 14,
  BidiClass_RightToLeftOverride = 15,
  BidiClass_PopDirectionalFormat = 16,
  BidiClass_NonspacingMark = 17,
  BidiClass_BoundaryNeutral = 18,
  BidiClass_FirstStrongIsolate = 19,
  BidiClass_LeftToRightIsolate = 20,
  BidiClass_RightToLeftIsolate = 21,
  BidiClass_PopDirectionalIsolate = 22,
} BidiClass;

typedef struct BidiClass_option {union { BidiClass ok; }; bool is_ok; } BidiClass_option;



#endif // BidiClass_D_H
