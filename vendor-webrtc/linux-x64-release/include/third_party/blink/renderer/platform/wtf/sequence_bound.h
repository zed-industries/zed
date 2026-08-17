// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SEQUENCE_BOUND_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SEQUENCE_BOUND_H_

#include "base/functional/bind.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/sequence_bound.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_functional.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace WTF {
namespace internal {

template <typename T>
struct IsCrossThreadOnceFunctionImpl : std::false_type {};

template <typename R, typename... Args>
struct IsCrossThreadOnceFunctionImpl<CrossThreadOnceFunction<R(Args...)>>
    : std::true_type {};

template <typename T>
using IsCrossThreadOnceFunction =
    IsCrossThreadOnceFunctionImpl<std::decay_t<T>>;

struct SequenceBoundBindTraits {
  template <typename Signature>
  using CrossThreadTask = WTF::CrossThreadOnceFunction<Signature>;

  template <typename Functor, typename... Args>
  static inline auto BindOnce(Functor&& functor, Args&&... args) {
    return CrossThreadBindOnce(std::forward<Functor>(functor),
                               std::forward<Args>(args)...);
  }

  template <typename T>
  static inline auto Unretained(T ptr) {
    return CrossThreadUnretained(ptr);
  }

  template <typename Signature>
  static inline bool PostTask(base::SequencedTaskRunner& task_runner,
                              const base::Location& location,
                              CrossThreadTask<Signature>&& task) {
    return task_runner.PostDelayedTask(
        location, ConvertToBaseOnceCallback(std::move(task)),
        base::TimeDelta());
  }

  static inline bool PostTaskAndReply(base::SequencedTaskRunner& task_runner,
                                      const base::Location& location,
                                      CrossThreadOnceClosure&& task,
                                      CrossThreadOnceClosure&& reply) {
    return task_runner.PostTaskAndReply(
        location, ConvertToBaseOnceCallback(std::move(task)),
        ConvertToBaseOnceCallback(std::move(reply)));
  }

  template <typename TaskReturnType, typename ReplyArgType>
  static inline bool PostTaskAndReplyWithResult(
      base::SequencedTaskRunner& task_runner,
      const base::Location& location,
      CrossThreadOnceFunction<TaskReturnType()>&& task,
      CrossThreadOnceFunction<void(ReplyArgType)>&& reply) {
    return task_runner.PostTaskAndReplyWithResult(
        location, ConvertToBaseOnceCallback(std::move(task)),
        ConvertToBaseOnceCallback(std::move(reply)));
  }

  template <template <typename> class CallbackType>
  static constexpr bool IsCrossThreadTask =
      IsCrossThreadOnceFunction<CallbackType<void()>>::value;
};

}  // namespace internal

template <typename T>
using SequenceBound =
    base::SequenceBound<T, WTF::internal::SequenceBoundBindTraits>;

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SEQUENCE_BOUND_H_
