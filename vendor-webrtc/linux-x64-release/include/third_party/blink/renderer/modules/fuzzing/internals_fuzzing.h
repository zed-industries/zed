// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FUZZING_INTERNALS_FUZZING_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FUZZING_INTERNALS_FUZZING_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Internals;

class InternalsFuzzing final {
  STATIC_ONLY(InternalsFuzzing);

 public:
  static ScriptPromise<IDLUndefined> runFuzzer(ScriptState* context,
                                               Internals&,
                                               const String& fuzzer_id,
                                               V8BufferSource* fuzzer_data);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FUZZING_INTERNALS_FUZZING_H_
