// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_LAUNCH_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_LAUNCH_QUEUE_H_

#include "third_party/blink/renderer/modules/launch/launch_params.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class V8LaunchConsumer;

class LaunchQueue final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  LaunchQueue();
  ~LaunchQueue() override;

  void Enqueue(LaunchParams* params);

  // IDL implementation:
  void setConsumer(V8LaunchConsumer*);

  // ScriptWrappable:
  void Trace(Visitor* visitor) const override;

 private:
  HeapVector<Member<LaunchParams>> unconsumed_launch_params_;
  Member<V8LaunchConsumer> consumer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_LAUNCH_QUEUE_H_
