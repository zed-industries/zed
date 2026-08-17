#ifndef TimePrecision_D_H
#define TimePrecision_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum TimePrecision {
  TimePrecision_Hour = 0,
  TimePrecision_Minute = 1,
  TimePrecision_MinuteOptional = 2,
  TimePrecision_Second = 3,
  TimePrecision_Subsecond1 = 4,
  TimePrecision_Subsecond2 = 5,
  TimePrecision_Subsecond3 = 6,
  TimePrecision_Subsecond4 = 7,
  TimePrecision_Subsecond5 = 8,
  TimePrecision_Subsecond6 = 9,
  TimePrecision_Subsecond7 = 10,
  TimePrecision_Subsecond8 = 11,
  TimePrecision_Subsecond9 = 12,
} TimePrecision;

typedef struct TimePrecision_option {union { TimePrecision ok; }; bool is_ok; } TimePrecision_option;



#endif // TimePrecision_D_H
