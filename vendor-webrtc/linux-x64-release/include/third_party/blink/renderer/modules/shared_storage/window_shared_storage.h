// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_WINDOW_SHARED_STORAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_WINDOW_SHARED_STORAGE_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ExceptionState;
class SharedStorage;
class LocalDOMWindow;

// Implement the sharedStorage attribute under Window.
class WindowSharedStorage {
  STATIC_ONLY(WindowSharedStorage);

 public:
  static SharedStorage* sharedStorage(LocalDOMWindow&, ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_WINDOW_SHARED_STORAGE_H_
