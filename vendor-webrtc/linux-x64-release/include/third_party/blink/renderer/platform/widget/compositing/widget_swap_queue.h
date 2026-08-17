// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_WIDGET_SWAP_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_WIDGET_SWAP_QUEUE_H_

#include <map>
#include "base/synchronization/lock.h"
#include "third_party/blink/public/mojom/widget/platform_widget.mojom-blink.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Queue used to keep track of which VisualStateRequestCallback should be
// invoked after a particular compositor frame swap. The callbacks are
// guaranteed to be processed after the frame is processed, but there is no
// guarantee that nothing else happens between processing the frame and
// processing the callback.
class WidgetSwapQueue {
 public:
  using VisualStateRequestCallback =
      mojom::blink::WidgetCompositor::VisualStateRequestCallback;
  WidgetSwapQueue() = default;
  ~WidgetSwapQueue() = default;
  WidgetSwapQueue(const WidgetSwapQueue&) = delete;
  WidgetSwapQueue& operator=(const WidgetSwapQueue&) = delete;

  // Returns true if there are no callback in the queue.
  bool Empty() const { return queue_.empty(); }

  // Queues the callback to be returned on a matching Drain call.
  //
  // |source_frame_number| - frame number to queue |callback| for.
  // |callback| - callback to queue. The method takes ownership of |callback|.
  // |is_first| - output parameter. Set to true if this was the first message
  //              enqueued for the given source_frame_number.
  void Queue(int source_frame_number,
             VisualStateRequestCallback callback,
             bool* is_first);

  // The method will append cllbacks queued for frame numbers lower or equal to
  // |source_frame_number|
  //
  // |source_frame_number| - swapped frame number.
  void Drain(int source_frame_number);

  // The method will clear |next_callbacks_| after copying to |callbacks|.
  //
  // |callbacks| - vector to store callbacks.
  void GetCallbacks(Vector<VisualStateRequestCallback>* callbacks);

 private:
  base::Lock lock_;
  std::map<int, Vector<VisualStateRequestCallback>> queue_
      ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)");
  Vector<VisualStateRequestCallback> next_callbacks_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_WIDGET_SWAP_QUEUE_H_
