// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WORKLET_WORKLET_THREAD_TEST_COMMON_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WORKLET_WORKLET_THREAD_TEST_COMMON_H_

#include <memory>

#include "third_party/blink/renderer/modules/worklet/animation_and_paint_worklet_thread.h"

namespace blink {

class AnimationWorkletProxyClient;
class Document;
class PaintWorkletProxyClient;
class WorkerReportingProxy;

std::unique_ptr<AnimationAndPaintWorkletThread>
CreateThreadAndProvideAnimationWorkletProxyClient(
    Document*,
    WorkerReportingProxy*,
    AnimationWorkletProxyClient* = nullptr);

std::unique_ptr<AnimationAndPaintWorkletThread>
CreateThreadAndProvidePaintWorkletProxyClient(
    Document*,
    WorkerReportingProxy*,
    PaintWorkletProxyClient* = nullptr);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WORKLET_WORKLET_THREAD_TEST_COMMON_H_
