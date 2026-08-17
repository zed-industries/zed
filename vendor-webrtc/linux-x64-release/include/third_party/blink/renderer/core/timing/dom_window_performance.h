// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_DOM_WINDOW_PERFORMANCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_DOM_WINDOW_PERFORMANCE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/timing/window_performance.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class LocalDOMWindow;

class CORE_EXPORT DOMWindowPerformance final
    : public GarbageCollected<DOMWindowPerformance>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  static DOMWindowPerformance& From(LocalDOMWindow&);
  static WindowPerformance* performance(LocalDOMWindow&);

  explicit DOMWindowPerformance(LocalDOMWindow&);
  DOMWindowPerformance(const DOMWindowPerformance&) = delete;
  DOMWindowPerformance& operator=(const DOMWindowPerformance&) = delete;

  void Trace(Visitor*) const override;

 private:
  WindowPerformance* performance();

  Member<WindowPerformance> performance_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_DOM_WINDOW_PERFORMANCE_H_
