#ifndef LineBreak_D_H
#define LineBreak_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LineBreak {
  LineBreak_Unknown = 0,
  LineBreak_Ambiguous = 1,
  LineBreak_Alphabetic = 2,
  LineBreak_BreakBoth = 3,
  LineBreak_BreakAfter = 4,
  LineBreak_BreakBefore = 5,
  LineBreak_MandatoryBreak = 6,
  LineBreak_ContingentBreak = 7,
  LineBreak_ClosePunctuation = 8,
  LineBreak_CombiningMark = 9,
  LineBreak_CarriageReturn = 10,
  LineBreak_Exclamation = 11,
  LineBreak_Glue = 12,
  LineBreak_Hyphen = 13,
  LineBreak_Ideographic = 14,
  LineBreak_Inseparable = 15,
  LineBreak_InfixNumeric = 16,
  LineBreak_LineFeed = 17,
  LineBreak_Nonstarter = 18,
  LineBreak_Numeric = 19,
  LineBreak_OpenPunctuation = 20,
  LineBreak_PostfixNumeric = 21,
  LineBreak_PrefixNumeric = 22,
  LineBreak_Quotation = 23,
  LineBreak_ComplexContext = 24,
  LineBreak_Surrogate = 25,
  LineBreak_Space = 26,
  LineBreak_BreakSymbols = 27,
  LineBreak_ZWSpace = 28,
  LineBreak_NextLine = 29,
  LineBreak_WordJoiner = 30,
  LineBreak_H2 = 31,
  LineBreak_H3 = 32,
  LineBreak_JL = 33,
  LineBreak_JT = 34,
  LineBreak_JV = 35,
  LineBreak_CloseParenthesis = 36,
  LineBreak_ConditionalJapaneseStarter = 37,
  LineBreak_HebrewLetter = 38,
  LineBreak_RegionalIndicator = 39,
  LineBreak_EBase = 40,
  LineBreak_EModifier = 41,
  LineBreak_ZWJ = 42,
  LineBreak_Aksara = 43,
  LineBreak_AksaraPrebase = 44,
  LineBreak_AksaraStart = 45,
  LineBreak_ViramaFinal = 46,
  LineBreak_Virama = 47,
} LineBreak;

typedef struct LineBreak_option {union { LineBreak ok; }; bool is_ok; } LineBreak_option;



#endif // LineBreak_D_H
