#ifndef DateTime_D_H
#define DateTime_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Date.d.h"
#include "Time.d.h"




typedef struct DateTime {
  Date* date;
  Time* time;
} DateTime;

typedef struct DateTime_option {union { DateTime ok; }; bool is_ok; } DateTime_option;



#endif // DateTime_D_H
