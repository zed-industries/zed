// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_RENDER_WIDGET_SIGNALS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_RENDER_WIDGET_SIGNALS_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace blink {
namespace scheduler {

class WidgetSchedulerImpl;

class PLATFORM_EXPORT RenderWidgetSignals {
  USING_FAST_MALLOC(RenderWidgetSignals);

 public:
  class PLATFORM_EXPORT Observer {
   public:
    virtual ~Observer() = default;

    // If |hidden| is true then all render widgets managed by this renderer
    // process have been hidden.
    // If |hidden| is false at least one render widget managed by this renderer
    // process has become visible and the renderer is no longer hidden.
    // Will be called on the main thread.
    virtual void SetAllRenderWidgetsHidden(bool hidden) = 0;
  };

  explicit RenderWidgetSignals(Observer* observer);

  void WriteIntoTrace(perfetto::TracedValue context) const;

 private:
  friend class WidgetSchedulerImpl;

  void IncNumVisibleRenderWidgets();
  void DecNumVisibleRenderWidgets();

  raw_ptr<Observer> observer_;  // NOT OWNED
  int num_visible_render_widgets_ = 0;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_RENDER_WIDGET_SIGNALS_H_
