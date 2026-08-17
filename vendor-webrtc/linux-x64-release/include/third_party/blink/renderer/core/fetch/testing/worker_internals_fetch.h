// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_TESTING_WORKER_INTERNALS_FETCH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_TESTING_WORKER_INTERNALS_FETCH_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class WorkerGlobalScope;
class WorkerInternals;
class Response;

class WorkerInternalsFetch {
  STATIC_ONLY(WorkerInternalsFetch);

 public:
  static Vector<String> getInternalResponseURLList(WorkerInternals&, Response*);
  static ScriptPromise<IDLLong> getInitialResourcePriority(ScriptState*,
                                                           WorkerInternals&,
                                                           const String& url,
                                                           WorkerGlobalScope*);

 private:
  static void ResolveResourcePriority(ScriptPromiseResolver<IDLLong>*,
                                      int resource_load_priority);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_TESTING_WORKER_INTERNALS_FETCH_H_
