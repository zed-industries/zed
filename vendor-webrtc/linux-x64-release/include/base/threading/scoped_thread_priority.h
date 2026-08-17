// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_SCOPED_THREAD_PRIORITY_H_
#define BASE_THREADING_SCOPED_THREAD_PRIORITY_H_

#include <atomic>
#include <optional>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/location.h"
#include "base/macros/uniquify.h"
#include "base/memory/raw_ptr.h"
#include "build/build_config.h"

namespace base {

class Location;
enum class ThreadType : int;

// All code that may load a DLL on a background thread must be surrounded by a
// scope that starts with this macro.
//
// Example:
//   Foo();
//   {
//     SCOPED_MAY_LOAD_LIBRARY_AT_BACKGROUND_PRIORITY();
//     LoadMyDll();
//   }
//   Bar();
//
// The macro raises the thread priority to match ThreadType::kDefault for the
// scope if no other thread has completed the current scope already (multiple
// threads can racily begin the initialization and will all be boosted for it).
// On Windows, loading a DLL on a background thread can lead to a priority
// inversion on the loader lock and cause huge janks.
#define SCOPED_MAY_LOAD_LIBRARY_AT_BACKGROUND_PRIORITY()                  \
  static std::atomic_bool BASE_UNIQUIFY(already_loaded){false};           \
  base::internal::ScopedMayLoadLibraryAtBackgroundPriority BASE_UNIQUIFY( \
      scoped_may_load_library_at_background_priority)(                    \
      FROM_HERE, &BASE_UNIQUIFY(already_loaded));

// Like SCOPED_MAY_LOAD_LIBRARY_AT_BACKGROUND_PRIORITY, but raises the thread
// priority every time the scope is entered. Use this around code that may
// conditionally load a DLL each time it is executed, or which repeatedly loads
// and unloads DLLs.
#define SCOPED_MAY_LOAD_LIBRARY_AT_BACKGROUND_PRIORITY_REPEATEDLY()       \
  base::internal::ScopedMayLoadLibraryAtBackgroundPriority BASE_UNIQUIFY( \
      scoped_may_load_library_at_background_priority)(FROM_HERE, nullptr);

// Boosts the current thread's priority to match the priority of threads of
// `target_thread_type` in this scope. `target_thread_type` must be lower
// priority than kRealtimeAudio, since realtime priority should only be used by
// dedicated media threads.
class BASE_EXPORT ScopedBoostPriority {
 public:
  explicit ScopedBoostPriority(ThreadType target_thread_type);
  ~ScopedBoostPriority();

  ScopedBoostPriority(const ScopedBoostPriority&) = delete;
  ScopedBoostPriority& operator=(const ScopedBoostPriority&) = delete;

 private:
  std::optional<ThreadType> original_thread_type_;
};

namespace internal {

class BASE_EXPORT ScopedMayLoadLibraryAtBackgroundPriority {
 public:
  // Boosts thread priority to match ThreadType::kDefault within its scope if
  // `already_loaded` is nullptr or set to false.
  explicit ScopedMayLoadLibraryAtBackgroundPriority(
      const Location& from_here,
      std::atomic_bool* already_loaded);

  ScopedMayLoadLibraryAtBackgroundPriority(
      const ScopedMayLoadLibraryAtBackgroundPriority&) = delete;
  ScopedMayLoadLibraryAtBackgroundPriority& operator=(
      const ScopedMayLoadLibraryAtBackgroundPriority&) = delete;

  ~ScopedMayLoadLibraryAtBackgroundPriority();

 private:
#if BUILDFLAG(IS_WIN)
  // The original priority when invoking entering the scope().
  std::optional<ThreadType> original_thread_type_;
  const raw_ptr<std::atomic_bool> already_loaded_;
#endif
};

}  // namespace internal

}  // namespace base

#endif  // BASE_THREADING_SCOPED_THREAD_PRIORITY_H_
