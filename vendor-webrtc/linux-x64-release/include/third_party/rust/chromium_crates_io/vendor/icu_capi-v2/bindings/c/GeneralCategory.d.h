#ifndef GeneralCategory_D_H
#define GeneralCategory_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum GeneralCategory {
  GeneralCategory_Unassigned = 0,
  GeneralCategory_UppercaseLetter = 1,
  GeneralCategory_LowercaseLetter = 2,
  GeneralCategory_TitlecaseLetter = 3,
  GeneralCategory_ModifierLetter = 4,
  GeneralCategory_OtherLetter = 5,
  GeneralCategory_NonspacingMark = 6,
  GeneralCategory_SpacingMark = 8,
  GeneralCategory_EnclosingMark = 7,
  GeneralCategory_DecimalNumber = 9,
  GeneralCategory_LetterNumber = 10,
  GeneralCategory_OtherNumber = 11,
  GeneralCategory_SpaceSeparator = 12,
  GeneralCategory_LineSeparator = 13,
  GeneralCategory_ParagraphSeparator = 14,
  GeneralCategory_Control = 15,
  GeneralCategory_Format = 16,
  GeneralCategory_PrivateUse = 17,
  GeneralCategory_Surrogate = 18,
  GeneralCategory_DashPunctuation = 19,
  GeneralCategory_OpenPunctuation = 20,
  GeneralCategory_ClosePunctuation = 21,
  GeneralCategory_ConnectorPunctuation = 22,
  GeneralCategory_InitialPunctuation = 28,
  GeneralCategory_FinalPunctuation = 29,
  GeneralCategory_OtherPunctuation = 23,
  GeneralCategory_MathSymbol = 24,
  GeneralCategory_CurrencySymbol = 25,
  GeneralCategory_ModifierSymbol = 26,
  GeneralCategory_OtherSymbol = 27,
} GeneralCategory;

typedef struct GeneralCategory_option {union { GeneralCategory ok; }; bool is_ok; } GeneralCategory_option;



#endif // GeneralCategory_D_H
