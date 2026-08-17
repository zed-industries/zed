// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_OPTIONAL_TRACE_EVENT_H_
#define BASE_TRACE_EVENT_OPTIONAL_TRACE_EVENT_H_

#include "base/trace_event/trace_event.h"
#include "base/tracing_buildflags.h"
#include "build/buildflag.h"

// These macros are functionally equivalent to TRACE_EVENTX macros,
// but they are disabled if optional_trace_events_enabled gn flag
// defined in tracing.gni is false.

#if BUILDFLAG(OPTIONAL_TRACE_EVENTS_ENABLED)

#define OPTIONAL_TRACE_EVENT(...) TRACE_EVENT(__VA_ARGS__)
#define OPTIONAL_TRACE_EVENT0(...) TRACE_EVENT0(__VA_ARGS__)
#define OPTIONAL_TRACE_EVENT1(...) TRACE_EVENT1(__VA_ARGS__)
#define OPTIONAL_TRACE_EVENT2(...) TRACE_EVENT2(__VA_ARGS__)

#else  // BUILDFLAG(OPTIONAL_TRACE_EVENTS_ENABLED)

#define OPTIONAL_TRACE_EVENT(...)
#define OPTIONAL_TRACE_EVENT0(category, name)
#define OPTIONAL_TRACE_EVENT1(category, name, arg1_name, arg1_val)
#define OPTIONAL_TRACE_EVENT2(category, name, arg1_name, arg1_val, arg2_name, \
                              arg2_val)

#endif  // BUILDFLAG(OPTIONAL_TRACE_EVENTS_ENABLED)

#endif  // BASE_TRACE_EVENT_OPTIONAL_TRACE_EVENT_H_
