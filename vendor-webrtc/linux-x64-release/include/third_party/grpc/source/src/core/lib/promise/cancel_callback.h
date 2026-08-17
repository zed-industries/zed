// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_CANCEL_CALLBACK_H
#define GRPC_SRC_CORE_LIB_PROMISE_CANCEL_CALLBACK_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/promise/context.h"
#include "src/core/lib/promise/detail/promise_like.h"
#include "src/core/lib/resource_quota/arena.h"

namespace grpc_core {

namespace cancel_callback_detail {

template <typename Fn>
class Handler {
 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION explicit Handler(Fn fn)
      : fn_(std::move(fn)) {}
  Handler(const Handler&) = delete;
  Handler& operator=(const Handler&) = delete;
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION ~Handler() {
    if (!done_) {
      promise_detail::Context<Arena> ctx(arena_.get());
      fn_();
    }
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION Handler(Handler&& other) noexcept
      : fn_(std::move(other.fn_)), done_(other.done_) {
    other.done_ = true;
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION Handler& operator=(
      Handler&& other) noexcept {
    fn_ = std::move(other.fn_);
    done_ = other.done_;
    other.done_ = true;
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION void Done() { done_ = true; }

 private:
  Fn fn_;
  // Since cancellation happens at destruction time we need to either capture
  // context here (via the arena), or make sure that no promise is destructed
  // without an Arena context on the stack. The latter is an eternal game of
  // whackamole, so we're choosing the former for now.
  // TODO(ctiller): re-evaluate at some point in the future.
  RefCountedPtr<Arena> arena_ =
      HasContext<Arena>() ? GetContext<Arena>()->Ref() : nullptr;
  bool done_ = false;
};

}  // namespace cancel_callback_detail

// Wrap main_fn so that it calls cancel_fn if the promise is destroyed prior to
// completion.
// Returns a promise with the same result type as main_fn.
template <typename MainFn, typename CancelFn>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline auto OnCancel(MainFn main_fn,
                                                          CancelFn cancel_fn) {
  return [on_cancel =
              cancel_callback_detail::Handler<CancelFn>(std::move(cancel_fn)),
          main_fn = promise_detail::PromiseLike<MainFn>(
              std::move(main_fn))]() mutable {
    auto r = main_fn();
    if (r.ready()) {
      on_cancel.Done();
    }
    return r;
  };
}

// Similar to OnCancel, but returns a factory that uses main_fn to construct the
// resulting promise. If the factory is dropped without being called, cancel_fn
// is called.
template <typename MainFn, typename CancelFn>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline auto OnCancelFactory(
    MainFn main_fn, CancelFn cancel_fn) {
  return [on_cancel =
              cancel_callback_detail::Handler<CancelFn>(std::move(cancel_fn)),
          main_fn = std::move(main_fn)]() mutable {
    auto r = main_fn();
    on_cancel.Done();
    return r;
  };
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_CANCEL_CALLBACK_H
