// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_WRAPPER_MODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_WRAPPER_MODE_H_

namespace blink {

// A list of modes for HeapMojo wrappers.
// TODO(crbug.com/1058076) This is just a temporary thing to keep the existing
// behavior during the release freeze.
enum class HeapMojoWrapperMode {
  // [Recommended] Resets the mojo connection when 1) the owner object is
  // garbage-collected
  // and 2) the associated ExecutionContext is detached.
  kWithContextObserver,
  // [Deprecated] Resets the mojo connection when the owner object is
  // garbage-collected.
  // But, it will not reset the mojo connection when the associated
  // ExecutionContext is detached.
  kForceWithoutContextObserver,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_WRAPPER_MODE_H_
