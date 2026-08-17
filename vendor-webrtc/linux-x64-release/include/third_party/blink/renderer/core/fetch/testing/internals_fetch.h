// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_TESTING_INTERNALS_FETCH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_TESTING_INTERNALS_FETCH_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Internals;
class Response;

class InternalsFetch {
  STATIC_ONLY(InternalsFetch);

 public:
  static Vector<String> getInternalResponseURLList(Internals&, Response*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_TESTING_INTERNALS_FETCH_H_
