#ifndef IndicSyllabicCategory_D_H
#define IndicSyllabicCategory_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum IndicSyllabicCategory {
  IndicSyllabicCategory_Other = 0,
  IndicSyllabicCategory_Avagraha = 1,
  IndicSyllabicCategory_Bindu = 2,
  IndicSyllabicCategory_BrahmiJoiningNumber = 3,
  IndicSyllabicCategory_CantillationMark = 4,
  IndicSyllabicCategory_Consonant = 5,
  IndicSyllabicCategory_ConsonantDead = 6,
  IndicSyllabicCategory_ConsonantFinal = 7,
  IndicSyllabicCategory_ConsonantHeadLetter = 8,
  IndicSyllabicCategory_ConsonantInitialPostfixed = 9,
  IndicSyllabicCategory_ConsonantKiller = 10,
  IndicSyllabicCategory_ConsonantMedial = 11,
  IndicSyllabicCategory_ConsonantPlaceholder = 12,
  IndicSyllabicCategory_ConsonantPrecedingRepha = 13,
  IndicSyllabicCategory_ConsonantPrefixed = 14,
  IndicSyllabicCategory_ConsonantSucceedingRepha = 15,
  IndicSyllabicCategory_ConsonantSubjoined = 16,
  IndicSyllabicCategory_ConsonantWithStacker = 17,
  IndicSyllabicCategory_GeminationMark = 18,
  IndicSyllabicCategory_InvisibleStacker = 19,
  IndicSyllabicCategory_Joiner = 20,
  IndicSyllabicCategory_ModifyingLetter = 21,
  IndicSyllabicCategory_NonJoiner = 22,
  IndicSyllabicCategory_Nukta = 23,
  IndicSyllabicCategory_Number = 24,
  IndicSyllabicCategory_NumberJoiner = 25,
  IndicSyllabicCategory_PureKiller = 26,
  IndicSyllabicCategory_RegisterShifter = 27,
  IndicSyllabicCategory_SyllableModifier = 28,
  IndicSyllabicCategory_ToneLetter = 29,
  IndicSyllabicCategory_ToneMark = 30,
  IndicSyllabicCategory_Virama = 31,
  IndicSyllabicCategory_Visarga = 32,
  IndicSyllabicCategory_Vowel = 33,
  IndicSyllabicCategory_VowelDependent = 34,
  IndicSyllabicCategory_VowelIndependent = 35,
  IndicSyllabicCategory_ReorderingKiller = 36,
} IndicSyllabicCategory;

typedef struct IndicSyllabicCategory_option {union { IndicSyllabicCategory ok; }; bool is_ok; } IndicSyllabicCategory_option;



#endif // IndicSyllabicCategory_D_H
