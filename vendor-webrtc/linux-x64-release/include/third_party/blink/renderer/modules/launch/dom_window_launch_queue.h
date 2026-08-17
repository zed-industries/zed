// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_DOM_WINDOW_LAUNCH_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_DOM_WINDOW_LAUNCH_QUEUE_H_

#include "third_party/blink/renderer/modules/launch/launch_params.h"
#include "third_party/blink/renderer/modules/launch/launch_queue.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LocalDOMWindow;
class KURL;

class DOMWindowLaunchQueue final
    : public GarbageCollected<DOMWindowLaunchQueue>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  explicit DOMWindowLaunchQueue(LocalDOMWindow& window);

  // IDL Interface.
  static LaunchQueue* launchQueue(LocalDOMWindow&);

  static void UpdateLaunchFiles(LocalDOMWindow*,
                                HeapVector<Member<FileSystemHandle>>);
  // TODO(crbug.com/1250225): Unify UpdateLaunchFiles() into this method.
  static void EnqueueLaunchParams(LocalDOMWindow*, const KURL& launch_url);

  void Trace(Visitor*) const override;

 private:
  static DOMWindowLaunchQueue* FromState(LocalDOMWindow* window);

  Member<LaunchQueue> launch_queue_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_DOM_WINDOW_LAUNCH_QUEUE_H_
