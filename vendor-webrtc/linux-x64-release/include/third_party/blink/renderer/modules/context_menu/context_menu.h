// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTEXT_MENU_CONTEXT_MENU_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTEXT_MENU_CONTEXT_MENU_H_

#include "third_party/blink/renderer/core/frame/context_menu_insets_changed_observer.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace gfx {
class Insets;
}

namespace blink {

class ContextMenu final : public GarbageCollected<ContextMenu>,
                          public Supplement<LocalFrame>,
                          public ContextMenuInsetsChangedObserver {
 public:
  static const char kSupplementName[];

  explicit ContextMenu(LocalFrame&);
  ContextMenu(const ContextMenu&) = delete;
  ContextMenu& operator=(const ContextMenu&) = delete;
  ~ContextMenu() = default;

  static ContextMenu* From(LocalFrame&);
  static void ProvideTo(LocalFrame&);

  // The context menu insets changed. Insets are in DIPs, relative to the local
  // frame bounds. Passing nullptr means the inset env() variables should be
  // removed.
  void ContextMenuInsetsChanged(const gfx::Insets*) final;

  void Trace(Visitor*) const override;

 private:
  WeakMember<LocalFrame> frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTEXT_MENU_CONTEXT_MENU_H_
