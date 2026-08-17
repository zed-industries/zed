// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// MemoryPressure provides static APIs for handling memory pressure on
// platforms that have such signals, such as Android and ChromeOS.
// The app will try to discard buffers that aren't deemed essential (individual
// modules will implement their own policy).

#ifndef BASE_TRACE_EVENT_MEMORY_PRESSURE_LEVEL_PROTO_H_
#define BASE_TRACE_EVENT_MEMORY_PRESSURE_LEVEL_PROTO_H_

#include "base/base_export.h"
#include "base/memory/memory_pressure_listener.h"
#include "base/tracing/protos/chrome_track_event.pbzero.h"

namespace base {
namespace trace_event {

BASE_EXPORT perfetto::protos::pbzero::MemoryPressureLevel
MemoryPressureLevelToTraceEnum(
    MemoryPressureListener::MemoryPressureLevel memory_pressure_level);

}
}  // namespace base

#endif  // BASE_TRACE_EVENT_MEMORY_PRESSURE_LEVEL_PROTO_H_
