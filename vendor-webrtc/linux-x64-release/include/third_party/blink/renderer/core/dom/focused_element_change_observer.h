// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUSED_ELEMENT_CHANGE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUSED_ELEMENT_CHANGE_OBSERVER_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class FocusedElementChangeObserver : public GarbageCollectedMixin {
 public:
  virtual void DidChangeFocus() = 0;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUSED_ELEMENT_CHANGE_OBSERVER_H_
