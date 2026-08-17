// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_CHILD_PROCESS_BINDING_TYPES_H_
#define BASE_ANDROID_CHILD_PROCESS_BINDING_TYPES_H_

namespace base {
namespace android {

// Defines the state of bindgings with child process. See ChildProcessConnection
// to see what the bindings are.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base
enum class ChildBindingState {
  UNBOUND,
  WAIVED,
  NOT_PERCEPTIBLE,
  VISIBLE,
  STRONG,
  MAX_VALUE = STRONG
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_CHILD_PROCESS_BINDING_TYPES_H_
