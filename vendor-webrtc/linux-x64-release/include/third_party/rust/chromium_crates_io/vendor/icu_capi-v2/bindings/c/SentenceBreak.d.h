#ifndef SentenceBreak_D_H
#define SentenceBreak_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum SentenceBreak {
  SentenceBreak_Other = 0,
  SentenceBreak_ATerm = 1,
  SentenceBreak_Close = 2,
  SentenceBreak_Format = 3,
  SentenceBreak_Lower = 4,
  SentenceBreak_Numeric = 5,
  SentenceBreak_OLetter = 6,
  SentenceBreak_Sep = 7,
  SentenceBreak_Sp = 8,
  SentenceBreak_STerm = 9,
  SentenceBreak_Upper = 10,
  SentenceBreak_CR = 11,
  SentenceBreak_Extend = 12,
  SentenceBreak_LF = 13,
  SentenceBreak_SContinue = 14,
} SentenceBreak;

typedef struct SentenceBreak_option {union { SentenceBreak ok; }; bool is_ok; } SentenceBreak_option;



#endif // SentenceBreak_D_H
