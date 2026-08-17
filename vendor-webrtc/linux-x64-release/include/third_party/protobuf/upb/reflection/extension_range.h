// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_EXTENSION_RANGE_H_
#define UPB_REFLECTION_EXTENSION_RANGE_H_

#include "upb/reflection/common.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

int32_t upb_ExtensionRange_Start(const upb_ExtensionRange* r);
int32_t upb_ExtensionRange_End(const upb_ExtensionRange* r);

bool upb_ExtensionRange_HasOptions(const upb_ExtensionRange* r);
const UPB_DESC(ExtensionRangeOptions) *
    upb_ExtensionRange_Options(const upb_ExtensionRange* r);
const UPB_DESC(FeatureSet) *
    upb_ExtensionRange_ResolvedFeatures(const upb_ExtensionRange* e);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_EXTENSION_RANGE_H_ */
