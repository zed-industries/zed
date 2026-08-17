// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_SERVICE_WORKER_GLOBAL_SCOPE_CONTENT_INDEX_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_SERVICE_WORKER_GLOBAL_SCOPE_CONTENT_INDEX_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ServiceWorkerGlobalScopeContentIndex {
  STATIC_ONLY(ServiceWorkerGlobalScopeContentIndex);

 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(contentdelete, kContentdelete)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_SERVICE_WORKER_GLOBAL_SCOPE_CONTENT_INDEX_H_
