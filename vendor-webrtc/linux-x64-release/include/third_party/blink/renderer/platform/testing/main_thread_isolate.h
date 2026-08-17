// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_MAIN_THREAD_ISOLATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_MAIN_THREAD_ISOLATE_H_

#include "base/memory/raw_ptr.h"
#include "v8/include/v8-forward.h"

namespace blink::test {

// Scoped main thread isolate for testing.
class MainThreadIsolate {
 public:
  MainThreadIsolate();
  ~MainThreadIsolate();

  v8::Isolate* isolate() { return isolate_.get(); }

 private:
  raw_ptr<v8::Isolate> isolate_;
};

}  // namespace blink::test

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_MAIN_THREAD_ISOLATE_H_
