#ifndef MeasureUnitParser_H
#define MeasureUnitParser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MeasureUnit.d.h"

#include "MeasureUnitParser.d.h"






MeasureUnit* icu4x_MeasureUnitParser_parse_mv1(const MeasureUnitParser* self, DiplomatStringView unit_id);


void icu4x_MeasureUnitParser_destroy_mv1(MeasureUnitParser* self);





#endif // MeasureUnitParser_H
