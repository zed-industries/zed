// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Defines constants needed before imports (like base.dll) are fully resolved.
// For example, constants defined here can be used by interceptions (i.e. hooks)
// in the sandbox, which run before imports are resolved, and can therefore only
// reference static variables.

#ifndef BASE_WIN_STATIC_CONSTANTS_H_
#define BASE_WIN_STATIC_CONSTANTS_H_

namespace base {
namespace win {

extern const char kApplicationVerifierDllName[];

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_STATIC_CONSTANTS_H_
