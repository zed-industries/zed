#ifndef UnitsConverter_H
#define UnitsConverter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "UnitsConverter.d.h"






double icu4x_UnitsConverter_convert_double_mv1(const UnitsConverter* self, double value);

UnitsConverter* icu4x_UnitsConverter_clone_mv1(const UnitsConverter* self);


void icu4x_UnitsConverter_destroy_mv1(UnitsConverter* self);





#endif // UnitsConverter_H
