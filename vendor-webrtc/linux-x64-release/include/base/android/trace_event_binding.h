// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_TRACE_EVENT_BINDING_H_
#define BASE_ANDROID_TRACE_EVENT_BINDING_H_

namespace base {
namespace android {
namespace internal {

constexpr const char kJavaTraceCategory[] = "Java";
constexpr const char kToplevelTraceCategory[] = "toplevel,Java";

}  // namespace internal
}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_TRACE_EVENT_BINDING_H_
