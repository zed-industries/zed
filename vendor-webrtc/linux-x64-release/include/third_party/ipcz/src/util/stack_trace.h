// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_UTIL_STACK_TRACE_H_
#define IPCZ_SRC_UTIL_STACK_TRACE_H_

#if defined(IPCZ_STANDALONE)
#include "standalone/base/stack_trace.h"  // nogncheck

namespace ipcz {

using StackTrace = ipcz::standalone::StackTrace;

}  // namespace ipcz
#else
#include "base/debug/stack_trace.h"  // nogncheck

namespace ipcz {

using StackTrace = base::debug::StackTrace;

}  // namespace ipcz
#endif

#endif  // IPCZ_SRC_UTIL_STACK_TRACE_H_
