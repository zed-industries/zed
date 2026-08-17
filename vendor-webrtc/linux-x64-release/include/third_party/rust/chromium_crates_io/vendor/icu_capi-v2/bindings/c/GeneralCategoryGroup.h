#ifndef GeneralCategoryGroup_H
#define GeneralCategoryGroup_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "GeneralCategory.d.h"

#include "GeneralCategoryGroup.d.h"






bool icu4x_GeneralCategoryGroup_contains_mv1(GeneralCategoryGroup self, GeneralCategory val);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_complement_mv1(GeneralCategoryGroup self);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_all_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_empty_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_union_mv1(GeneralCategoryGroup self, GeneralCategoryGroup other);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_intersection_mv1(GeneralCategoryGroup self, GeneralCategoryGroup other);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_cased_letter_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_letter_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_mark_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_number_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_separator_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_other_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_punctuation_mv1(void);

GeneralCategoryGroup icu4x_GeneralCategoryGroup_symbol_mv1(void);






#endif // GeneralCategoryGroup_H
