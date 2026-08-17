#ifndef WordBreak_D_H
#define WordBreak_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum WordBreak {
  WordBreak_Other = 0,
  WordBreak_ALetter = 1,
  WordBreak_Format = 2,
  WordBreak_Katakana = 3,
  WordBreak_MidLetter = 4,
  WordBreak_MidNum = 5,
  WordBreak_Numeric = 6,
  WordBreak_ExtendNumLet = 7,
  WordBreak_CR = 8,
  WordBreak_Extend = 9,
  WordBreak_LF = 10,
  WordBreak_MidNumLet = 11,
  WordBreak_Newline = 12,
  WordBreak_RegionalIndicator = 13,
  WordBreak_HebrewLetter = 14,
  WordBreak_SingleQuote = 15,
  WordBreak_DoubleQuote = 16,
  WordBreak_EBase = 17,
  WordBreak_EBaseGAZ = 18,
  WordBreak_EModifier = 19,
  WordBreak_GlueAfterZwj = 20,
  WordBreak_ZWJ = 21,
  WordBreak_WSegSpace = 22,
} WordBreak;

typedef struct WordBreak_option {union { WordBreak ok; }; bool is_ok; } WordBreak_option;



#endif // WordBreak_D_H
