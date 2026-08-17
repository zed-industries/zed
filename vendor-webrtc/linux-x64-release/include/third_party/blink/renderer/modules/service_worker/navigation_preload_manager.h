// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATION_PRELOAD_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATION_PRELOAD_MANAGER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class NavigationPreloadState;
class ServiceWorkerRegistration;

class NavigationPreloadManager final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit NavigationPreloadManager(ServiceWorkerRegistration*);

  ScriptPromise<IDLUndefined> enable(ScriptState*);
  ScriptPromise<IDLUndefined> disable(ScriptState*);
  ScriptPromise<IDLUndefined> setHeaderValue(ScriptState*,
                                             const String& value,
                                             ExceptionState& exception_state);
  ScriptPromise<NavigationPreloadState> getState(ScriptState*);

  void Trace(Visitor*) const override;

 private:
  ScriptPromise<IDLUndefined> SetEnabled(bool enable, ScriptState*);

  Member<ServiceWorkerRegistration> registration_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATION_PRELOAD_MANAGER_H_
