// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// -----------------------------------------------------------------------------
// Ruby Message functions. Strictly free of dependencies on
// Ruby interpreter internals.

#ifndef RUBY_PROTOBUF_SHARED_MESSAGE_H_
#define RUBY_PROTOBUF_SHARED_MESSAGE_H_

#include "ruby-upb.h"

// Returns a hash value for the given message.
uint64_t shared_Message_Hash(const upb_Message* msg, const upb_MessageDef* m,
                             uint64_t seed, upb_Status* status);

#endif  // RUBY_PROTOBUF_SHARED_MESSAGE_H_
