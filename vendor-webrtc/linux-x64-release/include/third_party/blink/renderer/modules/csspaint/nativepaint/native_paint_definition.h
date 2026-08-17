// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_NATIVE_PAINT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_NATIVE_PAINT_DEFINITION_H_

#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/animation/keyframe.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"
#include "third_party/blink/renderer/core/css/cssom/paint_worklet_input.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/workers/worker_backing_thread.h"
#include "third_party/blink/renderer/modules/csspaint/paint_definition.h"
#include "third_party/blink/renderer/modules/csspaint/paint_worklet_proxy_client.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class LocalFrame;

class MODULES_EXPORT NativePaintDefinition : public PaintDefinition {
 public:
  ~NativePaintDefinition() override = default;

  // Unregister the painter to ensure that there is no memory leakage on the
  // compositor thread.
  void UnregisterProxyClient();

  void Trace(Visitor* visitor) const override;

  int GetWorkletId() const;

 protected:
  NativePaintDefinition(LocalFrame*, PaintWorkletInput::PaintWorkletInputType);
  NativePaintDefinition() = default;

  // Register the PaintWorkletProxyClient to the compositor thread that
  // will hold a cross thread persistent pointer to it. This should be called
  // during the construction of native paint worklets, to ensure that the proxy
  // client is ready on the compositor thread when dispatching a paint job.
  void RegisterProxyClient(LocalFrame*,
                           PaintWorkletInput::PaintWorkletInputType);

  int worklet_id_;
  Member<PaintWorkletProxyClient> proxy_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_NATIVE_PAINT_DEFINITION_H_
