#ifndef WeekendContainsDay_D_H
#define WeekendContainsDay_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct WeekendContainsDay {
  bool monday;
  bool tuesday;
  bool wednesday;
  bool thursday;
  bool friday;
  bool saturday;
  bool sunday;
} WeekendContainsDay;

typedef struct WeekendContainsDay_option {union { WeekendContainsDay ok; }; bool is_ok; } WeekendContainsDay_option;



#endif // WeekendContainsDay_D_H
