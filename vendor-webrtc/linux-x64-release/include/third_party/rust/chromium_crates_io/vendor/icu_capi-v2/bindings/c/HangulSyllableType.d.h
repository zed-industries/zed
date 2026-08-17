#ifndef HangulSyllableType_D_H
#define HangulSyllableType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum HangulSyllableType {
  HangulSyllableType_NotApplicable = 0,
  HangulSyllableType_LeadingJamo = 1,
  HangulSyllableType_VowelJamo = 2,
  HangulSyllableType_TrailingJamo = 3,
  HangulSyllableType_LeadingVowelSyllable = 4,
  HangulSyllableType_LeadingVowelTrailingSyllable = 5,
} HangulSyllableType;

typedef struct HangulSyllableType_option {union { HangulSyllableType ok; }; bool is_ok; } HangulSyllableType_option;



#endif // HangulSyllableType_D_H
