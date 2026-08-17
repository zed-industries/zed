// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// -----------------------------------------------------------------------------
// Ruby <-> upb data conversion functions. Strictly free of dependencies on
// Ruby interpreter internals.

#ifndef RUBY_PROTOBUF_SHARED_CONVERT_H_
#define RUBY_PROTOBUF_SHARED_CONVERT_H_

#include "ruby-upb.h"
#include "shared_message.h"

bool shared_Msgval_IsEqual(upb_MessageValue val1, upb_MessageValue val2,
                           upb_CType type, const upb_MessageDef* msgdef,
                           upb_Status* status);

uint64_t shared_Msgval_GetHash(upb_MessageValue val, upb_CType type,
                               const upb_MessageDef* msgdef, uint64_t seed,
                               upb_Status* status);

#endif  // RUBY_PROTOBUF_SHARED_CONVERT_H_
