// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_USE_CASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_USE_CASE_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/perfetto/include/perfetto/tracing/string_helpers.h"

namespace blink::scheduler {

// `UseCase` encapsulates several high-level states that are use to apply
// different scheduling policies, e.g. compositor task queue priority, RAIL
// mode, and task deferral.
enum class UseCase {
  // No active use case detected.
  kNone,
  // A page is loading, first contentful paint has not been detected, and a
  // user gesture has not yet been detected.
  kEarlyLoading,
  // A page is loading, first contentful paint has been detected, and a user
  // gesture has not yet been detected.
  kLoading,
  // A gesture has recently started and we are about to run main thread touch
  // listeners to find out the actual gesture type. To minimize touch latency,
  // only input handling work should run in this state.
  //
  // Note: this `UseCase` only occurs when touchstart event handlers are on the
  // critical path, i.e. they are non-passive.
  kTouchstart,
  // A continuous gesture (e.g., scroll, pinch) which is being driven by the
  // compositor thread.
  kCompositorGesture,
  // A continuous gesture (e.g., scroll, pinch) which is being driven by the
  // compositor thread but also observed by the main thread. An example is
  // synchronized scrolling where a scroll listener on the main thread changes
  // page layout based on the current scroll position.
  kSynchronizedGesture,
  // A continuous gesture (e.g., scroll) which is being handled by the main
  // thread.
  kMainThreadGesture,
  // An unspecified touch gesture which is being handled by the main thread.
  // Note that since we don't have a full view of the use case, we should be
  // careful to prioritize all work equally.
  //
  // TODO(crbug.com/40589651): Try to remove this `UseCase`.
  kMainThreadCustomInputHandling,
  // A discrete input (e.g. keypress, click), was detected and we're waiting for
  // the subsequent paint.
  kDiscreteInputResponse,

  kMaxValue = kDiscreteInputResponse
};

PLATFORM_EXPORT perfetto::StaticString UseCaseToString(UseCase);

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_USE_CASE_H_
