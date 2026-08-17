//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_UTIL_THD_H
#define GRPC_SRC_CORE_UTIL_THD_H

/// Internal thread interface.

#include <grpc/support/port_platform.h>
#include <grpc/support/thd_id.h>
#include <stddef.h>

#include <memory>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "absl/log/check.h"

namespace grpc_core {
namespace internal {

/// Base class for platform-specific thread-state
class ThreadInternalsInterface {
 public:
  virtual ~ThreadInternalsInterface() {}
  virtual void Start() = 0;
  virtual void Join() = 0;
};

}  // namespace internal

class Thread {
 public:
  // Send a signal to the thread.
  // This is not supported on all platforms
  static void Signal(gpr_thd_id tid, int sig);
  // Kill the running thread. Likely not a clean operation.
  // This is not supported on all platforms.
  static void Kill(gpr_thd_id tid);

  class Options {
   public:
    Options() : joinable_(true), tracked_(true), stack_size_(0) {}
    /// Set whether the thread is joinable or detached.
    Options& set_joinable(bool joinable) {
      joinable_ = joinable;
      return *this;
    }
    bool joinable() const { return joinable_; }

    /// Set whether the thread is tracked for fork support.
    Options& set_tracked(bool tracked) {
      tracked_ = tracked;
      return *this;
    }
    bool tracked() const { return tracked_; }

    /// Sets thread stack size (in bytes). Sets to 0 will use the default stack
    /// size which is 64KB for Windows threads and 2MB for Posix(x86) threads.
    Options& set_stack_size(size_t bytes) {
      stack_size_ = bytes;
      return *this;
    }
    size_t stack_size() const { return stack_size_; }

   private:
    bool joinable_;
    bool tracked_;
    size_t stack_size_;
  };
  /// Default constructor only to allow use in structs that lack constructors
  /// Does not produce a validly-constructed thread; must later
  /// use placement new to construct a real thread. Does not init mu_ and cv_
  Thread() : state_(FAKE), impl_(nullptr) {}

  /// Normal constructor to create a thread with name \a thd_name,
  /// which will execute a thread based on function \a thd_body
  /// with argument \a arg once it is started.
  /// The optional \a success argument indicates whether the thread
  /// is successfully created.
  /// The optional \a options can be used to set the thread detachable.
  Thread(const char* thd_name, void (*thd_body)(void* arg), void* arg,
         bool* success = nullptr, const Options& options = Options());

  Thread(const char* thd_name, absl::AnyInvocable<void()> fn,
         bool* success = nullptr, const Options& options = Options())
      : Thread(
            thd_name,
            [](void* p) {
              std::unique_ptr<absl::AnyInvocable<void()>> fn_from_p(
                  static_cast<absl::AnyInvocable<void()>*>(p));
              (*fn_from_p)();
            },
            new absl::AnyInvocable<void()>(std::move(fn)), success, options) {}

  /// Move constructor for thread. After this is called, the other thread
  /// no longer represents a living thread object
  Thread(Thread&& other) noexcept
      : state_(other.state_), impl_(other.impl_), options_(other.options_) {
    other.state_ = MOVED;
    other.impl_ = nullptr;
    other.options_ = Options();
  }

  /// Move assignment operator for thread. After this is called, the other
  /// thread no longer represents a living thread object. Not allowed if this
  /// thread actually exists
  Thread& operator=(Thread&& other) noexcept {
    if (this != &other) {
      // TODO(vjpai): if we can be sure that all Thread's are actually
      // constructed, then we should assert CHECK(impl_ == nullptr) here.
      // However, as long as threads come in structures that are
      // allocated via gpr_malloc, this will not be the case, so we cannot
      // assert it for the time being.
      state_ = other.state_;
      impl_ = other.impl_;
      options_ = other.options_;
      other.state_ = MOVED;
      other.impl_ = nullptr;
      other.options_ = Options();
    }
    return *this;
  }

  /// The destructor is strictly optional; either the thread never came to life
  /// and the constructor itself killed it, or it has already been joined and
  /// the Join function kills it, or it was detached (non-joinable) and it has
  /// run to completion and is now killing itself. The destructor shouldn't have
  /// to do anything.
  ~Thread() { CHECK(!options_.joinable() || impl_ == nullptr); }

  void Start() {
    if (impl_ != nullptr) {
      CHECK(state_ == ALIVE);
      state_ = STARTED;
      impl_->Start();
      // If the Thread is not joinable, then the impl_ will cause the deletion
      // of this Thread object when the thread function completes. Since no
      // other operation is allowed to a detached thread after Start, there is
      // no need to change the value of the impl_ or state_ . The next operation
      // on this object will be the deletion, which will trigger the destructor.
    } else {
      CHECK(state_ == FAILED);
    }
  }

  // It is only legal to call Join if the Thread is created as joinable.
  void Join() {
    if (impl_ != nullptr) {
      impl_->Join();
      delete impl_;
      state_ = DONE;
      impl_ = nullptr;
    } else {
      CHECK(state_ == FAILED);
    }
  }

 private:
  Thread(const Thread&) = delete;
  Thread& operator=(const Thread&) = delete;

  /// The thread states are as follows:
  /// FAKE -- just a phony placeholder Thread created by the default constructor
  /// ALIVE -- an actual thread of control exists associated with this thread
  /// STARTED -- the thread of control has been started
  /// DONE -- the thread of control has completed and been joined
  /// FAILED -- the thread of control never came alive
  /// MOVED -- contents were moved out and we're no longer tracking them
  enum ThreadState { FAKE, ALIVE, STARTED, DONE, FAILED, MOVED };
  ThreadState state_;
  internal::ThreadInternalsInterface* impl_;
  Options options_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_THD_H
