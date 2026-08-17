#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_AHWB_USAGE_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_AHWB_USAGE_H_

#include <functional>
#include <list>
#include <vector>

#include "absl/functional/any_invocable.h"

namespace mediapipe {

// Holds AHWB on-complete function and release callbacks. This is used
// to manage resources and perform synchronization when using AHWB with of
// asynchronous inference operations (e.g. with DarwiNN interpreter).
struct TensorAhwbUsage {
  // Callback function that signals when it is safe to release AHWB.
  // If the input parameter is 'true' a forced finished is requested.
  using FinishingFunc = absl::AnyInvocable<bool(bool)>;

  // Callback function that is invoked when the tensor is being released. (E.g.
  // release interpreter buffer handles.)
  using ReleaseCallback = absl::AnyInvocable<void()>;

  // Function that signals when it is safe to release AHWB.
  // If the input parameter is 'true' then wait for the writing to be
  // finished.
  mutable FinishingFunc is_complete_fn;

  // Callbacks to release any associated resources. (E.g. imported interpreter
  // buffer handles.)
  std::vector<ReleaseCallback> release_callbacks;

  // Returns true if the usage is complete.
  bool IsComplete() const;

  // Triggers a blocked force complete and clears usage.
  void Reset();
};

// Blocks until all usages are force completed and erases them from the list.
void CompleteAndEraseUsages(std::list<TensorAhwbUsage>& ahwb_usages);

// Removes already completed usages from the list.
void EraseCompletedUsages(std::list<TensorAhwbUsage>& ahwb_usages);

// Returns true if the usages are incomplete.
bool HasIncompleteUsages(const std::list<TensorAhwbUsage>& ahwb_usages);

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_AHWB_USAGE_H_
