// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

// Declarations common to all public def types.

#ifndef UPB_REFLECTION_COMMON_H_
#define UPB_REFLECTION_COMMON_H_

#include "upb/reflection/descriptor_bootstrap.h"  // IWYU pragma: export

typedef enum {
  kUpb_Syntax_Proto2 = 2,
  kUpb_Syntax_Proto3 = 3,
  kUpb_Syntax_Editions = 99
} upb_Syntax;

// Forward declarations for circular references.
typedef struct upb_DefPool upb_DefPool;
typedef struct upb_EnumDef upb_EnumDef;
typedef struct upb_EnumReservedRange upb_EnumReservedRange;
typedef struct upb_EnumValueDef upb_EnumValueDef;
typedef struct upb_ExtensionRange upb_ExtensionRange;
typedef struct upb_FieldDef upb_FieldDef;
typedef struct upb_FileDef upb_FileDef;
typedef struct upb_MessageDef upb_MessageDef;
typedef struct upb_MessageReservedRange upb_MessageReservedRange;
typedef struct upb_MethodDef upb_MethodDef;
typedef struct upb_OneofDef upb_OneofDef;
typedef struct upb_ServiceDef upb_ServiceDef;

// EVERYTHING BELOW THIS LINE IS INTERNAL - DO NOT USE /////////////////////////

typedef struct upb_DefBuilder upb_DefBuilder;

#endif /* UPB_REFLECTION_COMMON_H_ */
