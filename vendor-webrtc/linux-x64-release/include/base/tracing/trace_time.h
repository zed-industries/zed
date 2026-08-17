// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACING_TRACE_TIME_H_
#define BASE_TRACING_TRACE_TIME_H_

#include "build/build_config.h"
#include "third_party/perfetto/protos/perfetto/common/builtin_clock.pbzero.h"

namespace base {
namespace tracing {

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_FUCHSIA)
// Linux, Android, and Fuchsia all use CLOCK_MONOTONIC. See crbug.com/166153
// about efforts to unify base::TimeTicks across all platforms.
constexpr perfetto::protos::pbzero::BuiltinClock kTraceClockId =
    perfetto::protos::pbzero::BUILTIN_CLOCK_MONOTONIC;
#else
// Mac and Windows TimeTicks advance when sleeping, so are closest to BOOTTIME
// in behavior.
// TODO(eseckler): Support specifying Mac/Win platform clocks in BuiltinClock.
constexpr perfetto::protos::pbzero::BuiltinClock kTraceClockId =
    perfetto::protos::pbzero::BUILTIN_CLOCK_BOOTTIME;
#endif

// Returns CLOCK_BOOTTIME on systems that support it, otherwise falls back to
// TRACE_TIME_TICKS_NOW().
int64_t TraceBootTicksNow();

}  // namespace tracing
}  // namespace base

#endif  // BASE_TRACING_TRACE_TIME_H_
