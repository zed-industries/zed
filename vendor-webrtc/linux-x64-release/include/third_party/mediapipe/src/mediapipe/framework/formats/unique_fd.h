#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_ANDROID_UNIQUE_FD_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_ANDROID_UNIQUE_FD_H_

#include <utility>

#include "absl/base/attributes.h"
#include "absl/status/statusor.h"

namespace mediapipe {

// Implementation of a unique file descriptor wrapper inspired by
// https://android.googlesource.com/platform/bionic/+/master/docs/fdsan.md
//
// This class is a wrapper around a file descriptor that ensures that the
// descriptor is closed when the wrapper goes out of scope.
//
// This class is not thread-safe.
class UniqueFd {
 public:
  UniqueFd() = default;

  // Wraps and takes ownership of the given file descriptor.
  explicit UniqueFd(int fd) { Reset(fd); }

  UniqueFd(const UniqueFd& copy) = delete;
  UniqueFd(UniqueFd&& move) { *this = std::move(move); }

  // Closes the wrapped file descriptor during destruction if it is valid.
  ~UniqueFd() { Reset(); }

  UniqueFd& operator=(const UniqueFd& copy) = delete;
  UniqueFd& operator=(UniqueFd&& move);
  // Returns a non-owned file descriptor.
  int Get() const { return fd_; }

  // Checks if a valid file descriptor is wrapped.
  bool IsValid() const { return fd_ >= 0; }

  absl::StatusOr<UniqueFd> Dup() const;

  // Releases ownership of the file descriptor and returns it.
  ABSL_MUST_USE_RESULT int Release();

  // Closes a wrapped file descriptor and resets the wrapper.
  void Reset(int new_fd = -1);

 private:
  int fd_ = -1;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_ANDROID_UNIQUE_FD_H_
