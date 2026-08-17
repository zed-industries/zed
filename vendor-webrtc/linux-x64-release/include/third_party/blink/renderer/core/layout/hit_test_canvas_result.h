// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_CANVAS_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_CANVAS_RESULT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"

namespace blink {

class CORE_EXPORT HitTestCanvasResult final
    : public GarbageCollected<HitTestCanvasResult> {
 public:
  HitTestCanvasResult(String id, Member<Element> control);

  String GetId() const;
  Element* GetControl() const;

  void Trace(Visitor*) const;

 private:
  String id_;
  Member<Element> control_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_CANVAS_RESULT_H_
