#ifndef IsoDateTime_D_H
#define IsoDateTime_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "IsoDate.d.h"
#include "Time.d.h"




typedef struct IsoDateTime {
  IsoDate* date;
  Time* time;
} IsoDateTime;

typedef struct IsoDateTime_option {union { IsoDateTime ok; }; bool is_ok; } IsoDateTime_option;



#endif // IsoDateTime_D_H
