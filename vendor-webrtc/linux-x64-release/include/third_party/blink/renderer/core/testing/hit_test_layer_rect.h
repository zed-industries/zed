/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_HIT_TEST_LAYER_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_HIT_TEST_LAYER_RECT_H_

#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/geometry/dom_rect_read_only.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// TODO(pdr): This is used for dumping touch_action_region for a given layer. It
// would be useful to have the associated TouchAction for each HitTestLayerRect.
class HitTestLayerRect final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  HitTestLayerRect(DOMRectReadOnly* layer_rect, DOMRectReadOnly* hit_test_rect)
      : layer_rect_(layer_rect), hit_test_rect_(hit_test_rect) {}

  DOMRectReadOnly* layerRect() const { return layer_rect_.Get(); }
  DOMRectReadOnly* hitTestRect() const { return hit_test_rect_.Get(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(layer_rect_);
    visitor->Trace(hit_test_rect_);
    ScriptWrappable::Trace(visitor);
  }

 private:
  Member<DOMRectReadOnly> layer_rect_;
  Member<DOMRectReadOnly> hit_test_rect_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_HIT_TEST_LAYER_RECT_H_
