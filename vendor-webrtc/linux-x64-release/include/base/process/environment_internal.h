// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains internal routines that are called by other files in
// base/process/.

#ifndef BASE_PROCESS_ENVIRONMENT_INTERNAL_H_
#define BASE_PROCESS_ENVIRONMENT_INTERNAL_H_

#include "base/base_export.h"
#include "base/containers/heap_array.h"
#include "base/environment.h"
#include "build/build_config.h"

namespace base {
namespace internal {

#if BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
// Returns a modified environment vector constructed from the given environment
// and the list of changes given in |changes|. Each key in the environment is
// matched against the first element of the pairs. In the event of a match, the
// value is replaced by the second of the pair, unless the second is empty, in
// which case the key-value is removed.
//
// This POSIX version takes and returns a POSIX-style environment block, which
// is a null-terminated list of pointers to null-terminated strings. The
// returned array will have appended to it the storage for the array itself so
// there is only one pointer to manage, but this means that you can't copy the
// array without keeping the original around.
BASE_EXPORT base::HeapArray<char*> AlterEnvironment(
    const char* const* env,
    const EnvironmentMap& changes);
#elif BUILDFLAG(IS_WIN)
// Returns a modified environment vector constructed from the given environment
// and the list of changes given in |changes|. Each key in the environment is
// matched against the first element of the pairs. In the event of a match, the
// value is replaced by the second of the pair, unless the second is empty, in
// which case the key-value is removed.
//
// This Windows version takes and returns a Windows-style environment block,
// which is a string containing several null-terminated strings followed by an
// extra terminating null character. So, e.g., the environment A=1 B=2 is
// represented as L"A=1\0B=2\0\0".
BASE_EXPORT NativeEnvironmentString
AlterEnvironment(const wchar_t* env, const EnvironmentMap& changes);
#endif  // OS_*

}  // namespace internal
}  // namespace base

#endif  // BASE_PROCESS_ENVIRONMENT_INTERNAL_H_
