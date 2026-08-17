#ifndef AnyCalendarKind_D_H
#define AnyCalendarKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum AnyCalendarKind {
  AnyCalendarKind_Iso = 0,
  AnyCalendarKind_Gregorian = 1,
  AnyCalendarKind_Buddhist = 2,
  AnyCalendarKind_Japanese = 3,
  AnyCalendarKind_JapaneseExtended = 4,
  AnyCalendarKind_Ethiopian = 5,
  AnyCalendarKind_EthiopianAmeteAlem = 6,
  AnyCalendarKind_Indian = 7,
  AnyCalendarKind_Coptic = 8,
  AnyCalendarKind_Dangi = 9,
  AnyCalendarKind_Chinese = 10,
  AnyCalendarKind_Hebrew = 11,
  AnyCalendarKind_IslamicCivil = 12,
  AnyCalendarKind_IslamicObservational = 13,
  AnyCalendarKind_IslamicTabular = 14,
  AnyCalendarKind_IslamicUmmAlQura = 15,
  AnyCalendarKind_Persian = 16,
  AnyCalendarKind_Roc = 17,
} AnyCalendarKind;

typedef struct AnyCalendarKind_option {union { AnyCalendarKind ok; }; bool is_ok; } AnyCalendarKind_option;



#endif // AnyCalendarKind_D_H
