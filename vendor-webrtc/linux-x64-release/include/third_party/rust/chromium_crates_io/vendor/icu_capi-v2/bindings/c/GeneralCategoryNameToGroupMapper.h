#ifndef GeneralCategoryNameToGroupMapper_H
#define GeneralCategoryNameToGroupMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "GeneralCategoryGroup.d.h"

#include "GeneralCategoryNameToGroupMapper.d.h"






GeneralCategoryGroup icu4x_GeneralCategoryNameToGroupMapper_get_strict_mv1(const GeneralCategoryNameToGroupMapper* self, DiplomatStringView name);

GeneralCategoryGroup icu4x_GeneralCategoryNameToGroupMapper_get_loose_mv1(const GeneralCategoryNameToGroupMapper* self, DiplomatStringView name);

GeneralCategoryNameToGroupMapper* icu4x_GeneralCategoryNameToGroupMapper_create_mv1(void);

typedef struct icu4x_GeneralCategoryNameToGroupMapper_create_with_provider_mv1_result {union {GeneralCategoryNameToGroupMapper* ok; DataError err;}; bool is_ok;} icu4x_GeneralCategoryNameToGroupMapper_create_with_provider_mv1_result;
icu4x_GeneralCategoryNameToGroupMapper_create_with_provider_mv1_result icu4x_GeneralCategoryNameToGroupMapper_create_with_provider_mv1(const DataProvider* provider);


void icu4x_GeneralCategoryNameToGroupMapper_destroy_mv1(GeneralCategoryNameToGroupMapper* self);





#endif // GeneralCategoryNameToGroupMapper_H
