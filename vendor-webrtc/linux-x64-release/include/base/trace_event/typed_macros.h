// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TYPED_MACROS_H_
#define BASE_TRACE_EVENT_TYPED_MACROS_H_

#include "base/trace_event/trace_event.h"
#include "base/tracing_buildflags.h"
#include "build/build_config.h"

// Needed not for this file, but for every user of the TRACE_EVENT macros for
// the lambda definition. So included here for convenience.
#include "base/tracing/protos/chrome_track_event.pbzero.h"
#include "third_party/perfetto/include/perfetto/tracing/event_context.h"
#include "third_party/perfetto/include/perfetto/tracing/string_helpers.h"

#endif  // BASE_TRACE_EVENT_TYPED_MACROS_H_
