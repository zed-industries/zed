// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_LEX_ROUND_TRIP_H_
#define UPB_LEX_ROUND_TRIP_H_

// Must be last.
#include "upb/port/def.inc"

// Encodes a float or double that is round-trippable, but as short as possible.
// These routines are not fully optimal (not guaranteed to be shortest), but are
// short-ish and match the implementation that has been used in protobuf since
// the beginning.

// The given buffer size must be at least kUpb_RoundTripBufferSize.
enum { kUpb_RoundTripBufferSize = 32 };

#ifdef __cplusplus
extern "C" {
#endif

void _upb_EncodeRoundTripDouble(double val, char* buf, size_t size);
void _upb_EncodeRoundTripFloat(float val, char* buf, size_t size);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_LEX_ROUND_TRIP_H_ */
