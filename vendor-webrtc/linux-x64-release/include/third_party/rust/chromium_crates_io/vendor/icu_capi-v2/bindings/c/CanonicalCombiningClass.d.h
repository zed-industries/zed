#ifndef CanonicalCombiningClass_D_H
#define CanonicalCombiningClass_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CanonicalCombiningClass {
  CanonicalCombiningClass_NotReordered = 0,
  CanonicalCombiningClass_Overlay = 1,
  CanonicalCombiningClass_HanReading = 6,
  CanonicalCombiningClass_Nukta = 7,
  CanonicalCombiningClass_KanaVoicing = 8,
  CanonicalCombiningClass_Virama = 9,
  CanonicalCombiningClass_CCC10 = 10,
  CanonicalCombiningClass_CCC11 = 11,
  CanonicalCombiningClass_CCC12 = 12,
  CanonicalCombiningClass_CCC13 = 13,
  CanonicalCombiningClass_CCC14 = 14,
  CanonicalCombiningClass_CCC15 = 15,
  CanonicalCombiningClass_CCC16 = 16,
  CanonicalCombiningClass_CCC17 = 17,
  CanonicalCombiningClass_CCC18 = 18,
  CanonicalCombiningClass_CCC19 = 19,
  CanonicalCombiningClass_CCC20 = 20,
  CanonicalCombiningClass_CCC21 = 21,
  CanonicalCombiningClass_CCC22 = 22,
  CanonicalCombiningClass_CCC23 = 23,
  CanonicalCombiningClass_CCC24 = 24,
  CanonicalCombiningClass_CCC25 = 25,
  CanonicalCombiningClass_CCC26 = 26,
  CanonicalCombiningClass_CCC27 = 27,
  CanonicalCombiningClass_CCC28 = 28,
  CanonicalCombiningClass_CCC29 = 29,
  CanonicalCombiningClass_CCC30 = 30,
  CanonicalCombiningClass_CCC31 = 31,
  CanonicalCombiningClass_CCC32 = 32,
  CanonicalCombiningClass_CCC33 = 33,
  CanonicalCombiningClass_CCC34 = 34,
  CanonicalCombiningClass_CCC35 = 35,
  CanonicalCombiningClass_CCC36 = 36,
  CanonicalCombiningClass_CCC84 = 84,
  CanonicalCombiningClass_CCC91 = 91,
  CanonicalCombiningClass_CCC103 = 103,
  CanonicalCombiningClass_CCC107 = 107,
  CanonicalCombiningClass_CCC118 = 118,
  CanonicalCombiningClass_CCC122 = 122,
  CanonicalCombiningClass_CCC129 = 129,
  CanonicalCombiningClass_CCC130 = 130,
  CanonicalCombiningClass_CCC132 = 132,
  CanonicalCombiningClass_CCC133 = 133,
  CanonicalCombiningClass_AttachedBelowLeft = 200,
  CanonicalCombiningClass_AttachedBelow = 202,
  CanonicalCombiningClass_AttachedAbove = 214,
  CanonicalCombiningClass_AttachedAboveRight = 216,
  CanonicalCombiningClass_BelowLeft = 218,
  CanonicalCombiningClass_Below = 220,
  CanonicalCombiningClass_BelowRight = 222,
  CanonicalCombiningClass_Left = 224,
  CanonicalCombiningClass_Right = 226,
  CanonicalCombiningClass_AboveLeft = 228,
  CanonicalCombiningClass_Above = 230,
  CanonicalCombiningClass_AboveRight = 232,
  CanonicalCombiningClass_DoubleBelow = 233,
  CanonicalCombiningClass_DoubleAbove = 234,
  CanonicalCombiningClass_IotaSubscript = 240,
} CanonicalCombiningClass;

typedef struct CanonicalCombiningClass_option {union { CanonicalCombiningClass ok; }; bool is_ok; } CanonicalCombiningClass_option;



#endif // CanonicalCombiningClass_D_H
