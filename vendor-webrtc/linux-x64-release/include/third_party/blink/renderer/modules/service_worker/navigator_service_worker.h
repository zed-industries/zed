// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATOR_SERVICE_WORKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATOR_SERVICE_WORKER_H_

#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ExceptionState;
class LocalDOMWindow;
class Navigator;
class ScriptState;
class ServiceWorkerContainer;

class MODULES_EXPORT NavigatorServiceWorker {
  STATIC_ONLY(NavigatorServiceWorker);

 public:
  static ServiceWorkerContainer* From(LocalDOMWindow&);
  static ServiceWorkerContainer* serviceWorker(ScriptState*,
                                               Navigator&,
                                               ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATOR_SERVICE_WORKER_H_
