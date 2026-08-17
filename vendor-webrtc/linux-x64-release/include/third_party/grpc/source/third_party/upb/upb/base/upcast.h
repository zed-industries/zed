// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_BASE_UPCAST_H_
#define UPB_BASE_UPCAST_H_

// Must be last.
#include "upb/port/def.inc"

// This macro provides a way to upcast message pointers in a way that is
// somewhat more bulletproof than blindly casting a pointer. Example:
//
// typedef struct {
//   upb_Message UPB_PRIVATE(base);
// } pkg_FooMessage;
//
// void f(pkg_FooMessage* msg) {
//   upb_Decode(UPB_UPCAST(msg), ...);
// }

#define UPB_UPCAST(x) (&(x)->base##_dont_copy_me__upb_internal_use_only)

#include "upb/port/undef.inc"

#endif /* UPB_BASE_UPCAST_H_ */
