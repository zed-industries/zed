// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CONTEXT_MENU_INSETS_CHANGED_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CONTEXT_MENU_INSETS_CHANGED_OBSERVER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace gfx {
class Insets;
}

namespace blink {

class LocalFrame;

// This observer is used to register for context menu notifications that are
// sent from Browser process to |LocalFrame|.
class CORE_EXPORT ContextMenuInsetsChangedObserver
    : public GarbageCollectedMixin {
 public:
  // If nullptr, this removes context menu inset settings completely. If non-
  // nullptr, sets the provided insets.
  virtual void ContextMenuInsetsChanged(const gfx::Insets*) = 0;

 protected:
  explicit ContextMenuInsetsChangedObserver(LocalFrame&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CONTEXT_MENU_INSETS_CHANGED_OBSERVER_H_
