#ifndef FixedDecimalRoundingIncrement_D_H
#define FixedDecimalRoundingIncrement_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FixedDecimalRoundingIncrement {
  FixedDecimalRoundingIncrement_MultiplesOf1 = 0,
  FixedDecimalRoundingIncrement_MultiplesOf2 = 1,
  FixedDecimalRoundingIncrement_MultiplesOf5 = 2,
  FixedDecimalRoundingIncrement_MultiplesOf25 = 3,
} FixedDecimalRoundingIncrement;

typedef struct FixedDecimalRoundingIncrement_option {union { FixedDecimalRoundingIncrement ok; }; bool is_ok; } FixedDecimalRoundingIncrement_option;



#endif // FixedDecimalRoundingIncrement_D_H
