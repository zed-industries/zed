// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_POPIN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_POPIN_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/v8_popin_context_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CORE_EXPORT WindowPopin final : public GarbageCollected<WindowPopin>,
                                      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  explicit WindowPopin(LocalDOMWindow&);
  ~WindowPopin() = default;
  void Trace(Visitor*) const override;

  static Vector<V8PopinContextType> popinContextTypesSupported(LocalDOMWindow&);
  Vector<V8PopinContextType> popinContextTypesSupported();

  static std::optional<V8PopinContextType> popinContextType(LocalDOMWindow&);
  std::optional<V8PopinContextType> popinContextType();

 private:
  static WindowPopin& From(LocalDOMWindow&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_POPIN_H_
