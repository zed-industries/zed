// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_APPLICATION_STATE_PROTO_ANDROID_H_
#define BASE_TRACE_EVENT_APPLICATION_STATE_PROTO_ANDROID_H_

#include "base/android/application_status_listener.h"
#include "base/base_export.h"
#include "third_party/perfetto/protos/perfetto/trace/track_event/chrome_application_state_info.pbzero.h"

#define TRACE_APPLICATION_STATE(state)                                  \
  TRACE_EVENT_INSTANT(                                                  \
      "Java", "ApplicationState", perfetto::Track::Global(0),           \
      [state](perfetto::EventContext ctx) {                             \
        ctx.event()                                                     \
            ->set_chrome_application_state_info()                       \
            ->set_application_state(                                    \
                base::trace_event::ApplicationStateToTraceEnum(state)); \
      });

namespace base {
namespace trace_event {

BASE_EXPORT
perfetto::protos::pbzero::ChromeApplicationStateInfo::ChromeApplicationState
ApplicationStateToTraceEnum(base::android::ApplicationState state);

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_APPLICATION_STATE_PROTO_ANDROID_H_
