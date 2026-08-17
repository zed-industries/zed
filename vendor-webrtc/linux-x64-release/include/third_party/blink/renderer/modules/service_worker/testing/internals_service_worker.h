// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_TESTING_INTERNALS_SERVICE_WORKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_TESTING_INTERNALS_SERVICE_WORKER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Internals;
class ScriptState;
class ServiceWorker;

class InternalsServiceWorker {
  STATIC_ONLY(InternalsServiceWorker);

 public:
  static ScriptPromise<IDLUndefined> terminateServiceWorker(ScriptState*,
                                                            Internals&,
                                                            ServiceWorker*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_TESTING_INTERNALS_SERVICE_WORKER_H_
