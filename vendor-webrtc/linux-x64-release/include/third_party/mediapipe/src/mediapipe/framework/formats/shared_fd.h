#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_SHARED_FD_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_SHARED_FD_H_

#include <cstddef>
#include <memory>
#include <utility>

#include "absl/status/statusor.h"
#include "mediapipe/framework/formats/unique_fd.h"

namespace mediapipe {

// Provides a shared ownership for a file descriptor.
//
// File descriptor is closed as soon as last SharedFd is destroyed.
// (Uses `std::shared_ptr` internally and can be used in the same way: copy,
// move, assign/compare with nullptr, use in conditional statements.)
class SharedFd {
 public:
  // `fd` a valid file descriptor.
  explicit SharedFd(UniqueFd fd)
      : fd_(std::make_shared<UniqueFd>(std::move(fd))) {}

  // Constructs empty SharedFd (fd == nullptr evaluates to true)
  SharedFd() = default;

  ~SharedFd() = default;

  // Copyable
  SharedFd(const SharedFd&) = default;
  SharedFd& operator=(const SharedFd&) = default;

  // Moveable
  SharedFd(SharedFd&& other) = default;
  SharedFd& operator=(SharedFd&& other) = default;

  // Resets this SharedFd object (fd == nullptr will evaluate to true).
  SharedFd& operator=(std::nullptr_t other) {
    fd_ = other;
    return *this;
  }

  bool operator==(std::nullptr_t other) const { return fd_ == other; }
  bool operator!=(std::nullptr_t other) const { return !operator==(other); };

  // SharedFd can be used in conditional statements:
  // ```
  // if (fd) {
  //   int raw_fd = fd.Get();
  // }
  // ```
  explicit operator bool() const { return operator!=(nullptr); }

  // Gets raw file descriptor for read purposes.
  int Get() const { return fd_->Get(); }

  // Duplicates file descriptor.
  absl::StatusOr<UniqueFd> Dup() const { return fd_->Dup(); }

 private:
  std::shared_ptr<UniqueFd> fd_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_SHARED_FD_H_
