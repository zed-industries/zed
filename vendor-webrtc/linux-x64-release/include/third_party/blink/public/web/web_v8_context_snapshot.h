// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_V8_CONTEXT_SNAPSHOT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_V8_CONTEXT_SNAPSHOT_H_

#include "third_party/blink/public/platform/web_common.h"
#include "v8/include/v8-snapshot.h"

namespace blink {

// WebV8ContextSnapshot is an API to take a snapshot of V8 context.
// This API should be used only by tools/v8_context_snapshot, which runs during
// Chromium's build step.
class BLINK_EXPORT WebV8ContextSnapshot {
 public:
  static v8::StartupData TakeSnapshot(v8::Isolate* isolate);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_V8_CONTEXT_SNAPSHOT_H_
