#ifndef LanguageDisplay_D_H
#define LanguageDisplay_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LanguageDisplay {
  LanguageDisplay_Dialect = 0,
  LanguageDisplay_Standard = 1,
} LanguageDisplay;

typedef struct LanguageDisplay_option {union { LanguageDisplay ok; }; bool is_ok; } LanguageDisplay_option;



#endif // LanguageDisplay_D_H
