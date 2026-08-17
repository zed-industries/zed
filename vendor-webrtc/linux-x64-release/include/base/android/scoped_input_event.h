// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_SCOPED_INPUT_EVENT_H_
#define BASE_ANDROID_SCOPED_INPUT_EVENT_H_

#include <android/input.h>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/trace_event/base_tracing.h"

namespace base::android {

// Class to manage lifecycle of AInputEvent.
// The class should only be instantiated on Android S+, since
// AInputEvent_release was added only in Android S(31).
class BASE_EXPORT ScopedInputEvent {
 public:
  explicit ScopedInputEvent(const AInputEvent* event);
  ~ScopedInputEvent();

  ScopedInputEvent(ScopedInputEvent&& other);
  ScopedInputEvent& operator=(ScopedInputEvent&& other);

  // Move only type.
  ScopedInputEvent(const ScopedInputEvent&) = delete;
  ScopedInputEvent& operator=(const ScopedInputEvent&) = delete;

  explicit operator bool() const { return !!a_input_event_; }

  const AInputEvent* a_input_event() const { return a_input_event_.get(); }

#if BUILDFLAG(ENABLE_BASE_TRACING)
  void WriteIntoTrace(
      perfetto::TracedProto<perfetto::protos::pbzero::EventForwarder> forwarder)
      const;
#endif  // BUILDFLAG(ENABLE_BASE_TRACING)

 private:
  void DestroyIfNeeded();

  raw_ptr<const AInputEvent> a_input_event_ = nullptr;
};

}  // namespace base::android

#endif  // BASE_ANDROID_SCOPED_INPUT_EVENT_H_
