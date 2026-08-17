#ifndef MEDIAPIPE_FRAMEWORK_DEPS_MLOCK_HELPERS_H_
#define MEDIAPIPE_FRAMEWORK_DEPS_MLOCK_HELPERS_H_
#include "absl/status/status.h"

namespace mediapipe {
// Uses `mlock`/`VirtualLock` to pin memory pages.
absl::Status LockMemory(const void* base_address, size_t length);
// Unlocks a previously locked memory region.
absl::Status UnlockMemory(const void* base_address, size_t length);
}  // namespace mediapipe
#endif  // MEDIAPIPE_FRAMEWORK_DEPS_MLOCK_HELPERS_H_
