// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOM_VIEWPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOM_VIEWPORT_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class LocalDOMWindow;
class DOMRect;

class CORE_EXPORT DOMViewport final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit DOMViewport(LocalDOMWindow*);
  ~DOMViewport() override;

  void Trace(Visitor*) const override;

  std::optional<HeapVector<Member<DOMRect>>> segments() const;

 private:
  Member<LocalDOMWindow> window_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOM_VIEWPORT_H_
