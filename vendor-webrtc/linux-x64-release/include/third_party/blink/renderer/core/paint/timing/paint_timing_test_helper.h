#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_TEST_HELPER_H_

#include <queue>

#include "base/check_op.h"
#include "base/functional/bind.h"
#include "base/functional/callback_forward.h"
#include "third_party/blink/renderer/core/paint/timing/paint_timing_callback_manager.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// |MockPaintTimingCallbackManager| is used to mock
// |ChromeClient::NotifyPresentationTime()|'s presentation-time queueing and
// invoking for unit-tests. Find more details in |PaintTimingCallbackManager|.
class MockPaintTimingCallbackManager final
    : public GarbageCollected<MockPaintTimingCallbackManager>,
      public PaintTimingCallbackManager {
 public:
  using CallbackQueue = std::queue<PaintTimingCallbackManager::Callback>;
  ~MockPaintTimingCallbackManager() {}
  void RegisterCallback(Callback callback) override {
    callback_queue_.push(std::move(callback));
  }
  void InvokePresentationTimeCallback(
      const base::TimeTicks& presentation_time,
      const DOMPaintTimingInfo& paint_timing_info) {
    DCHECK_GT(callback_queue_.size(), 0UL);
    std::move(callback_queue_.front())
        .Run(presentation_time, paint_timing_info);
    callback_queue_.pop();
  }

  size_t CountCallbacks() { return callback_queue_.size(); }

  void Trace(Visitor* visitor) const override {}

 private:
  CallbackQueue callback_queue_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_TEST_HELPER_H_
