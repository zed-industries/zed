// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUS_CHANGED_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUS_CHANGED_OBSERVER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class LocalFrame;
class Page;

class CORE_EXPORT FocusChangedObserver : public GarbageCollectedMixin {
 public:
  explicit FocusChangedObserver(Page*);
  virtual void FocusedFrameChanged() = 0;

 protected:
  bool IsFrameFocused(LocalFrame*);
  virtual ~FocusChangedObserver() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUS_CHANGED_OBSERVER_H_
