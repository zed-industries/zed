// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_CRASH_LOGGING_H_
#define BASE_DEBUG_CRASH_LOGGING_H_

#include <stddef.h>

#include <iosfwd>
#include <memory>
#include <string_view>
#include <type_traits>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/strings/string_number_conversions.h"

namespace base {
namespace debug {

// A crash key is an annotation that is carried along with a crash report, to
// provide additional debugging information beyond a stack trace. Crash keys
// have a name and a string value.
//
// The preferred API is //components/crash/core/common:crash_key, however not
// all clients can hold a direct dependency on that target. The API provided
// in this file indirects the dependency and adds some convenience helpers that
// make the API a bit less clunky.
//
// TODO(dcheng): Some of the nicer APIs should probably be upstreamed into
// //components/crash.
//
// Preferred usage when a crash key value only needs to be set within a scope:
//
//   SCOPED_CRASH_KEY_STRING32("category", "name", "value");
//   base::debug::DumpWithoutCrashing();
//
// If the crash key is pre-allocated elsewhere, but the value only needs to be
// set within a scope:
//
//   base::debug::ScopedCrashKeyString scoper(
//       GetCrashKeyForComponent(),
//       "value");
//
// Otherwise, if the crash key needs to persist (e.g. the actual crash dump is
// triggered some time later asynchronously):
//
//   static auto* const crash_key = base::debug::AllocateCrashKeyString(
//       "name", base::debug::CrashKeySize::Size32);
//   base::debug::SetCrashKeyString(crash_key, "value");
//
//   // Do other work before calling `base::debug::DumpWithoutCrashing()` later.
//
// ***WARNING***
//
// Do *not* write this:
//
//   base::debug::SetCrashKeyString(
//       base::debug::AllocateCrashKeyString(
//           "name", base::debug::CrashKeySize::Size32),
//       "value");
//
// As this will leak a heap allocation every time the crash key is set!

// The maximum length for a crash key's value must be one of the following
// pre-determined values.
enum class CrashKeySize {
  Size32 = 32,
  Size64 = 64,
  Size256 = 256,
  Size1024 = 1024,
};

struct CrashKeyString;

// Allocates a new crash key with the specified |name| with storage for a
// value up to length |size|. This will return null if the crash key system is
// not initialized.
//
// Note: this internally allocates, so the returned pointer should always
// be cached in a variable with static storage duration, e.g.:
//   static auto* const crash_key = base::debug::AllocateCrashKeyString(...);
BASE_EXPORT CrashKeyString* AllocateCrashKeyString(const char name[],
                                                   CrashKeySize size);

// Stores |value| into the specified |crash_key|. The |crash_key| may be null
// if AllocateCrashKeyString() returned null. If |value| is longer than the
// size with which the key was allocated, it will be truncated.
BASE_EXPORT void SetCrashKeyString(CrashKeyString* crash_key,
                                   std::string_view value);

// Clears any value that was stored in |crash_key|. The |crash_key| may be
// null.
BASE_EXPORT void ClearCrashKeyString(CrashKeyString* crash_key);

// Outputs current (i.e. allocated and non-empty) crash keys to `out`.
BASE_EXPORT void OutputCrashKeysToStream(std::ostream& out);

// A scoper that sets the specified key to value for the lifetime of the
// object, and clears it on destruction.
class BASE_EXPORT [[nodiscard]] ScopedCrashKeyString {
 public:
  ScopedCrashKeyString(CrashKeyString* crash_key, std::string_view value);
  ScopedCrashKeyString(ScopedCrashKeyString&& other);
  ~ScopedCrashKeyString();

  // Disallow copy and assign.
  ScopedCrashKeyString(const ScopedCrashKeyString&) = delete;
  ScopedCrashKeyString& operator=(const ScopedCrashKeyString&) = delete;

  // Disallow move assign to keep the time at which the crash key is cleared
  // easy to reason about. Assigning over an existing instance would
  // automatically clear the key instead of at the destruction of the object.
  ScopedCrashKeyString& operator=(ScopedCrashKeyString&&) = delete;

 private:
  raw_ptr<CrashKeyString> crash_key_;
};

// Internal helpers for the SCOPED_CRASH_KEY_... helper macros defined below.
//
// The first static_assert that checks the length of |key_name| is a
// compile-time equivalent of the DCHECK in
// crash_reporter::internal::CrashKeyStringImpl::Set that restricts the name of
// a crash key to 40 characters.
//
// The second static_assert that checks for reserved characters is a compile
// time equivalent of the DCHECK in base::debug::AllocateCrashKeyString.
#define SCOPED_CRASH_KEY_STRING_INTERNAL2(category, name, nonce, data,  \
                                          key_size)                     \
  static_assert(::std::size(category "-" name) < 40,                    \
                "Crash key names must be shorter than 40 characters."); \
  static_assert(::std::string_view(category "-" name).find(':') ==      \
                    ::std::string_view::npos,                           \
                "Crash key names must not contain the ':' character."); \
  ::base::debug::ScopedCrashKeyString scoped_crash_key_helper##nonce(   \
      [] {                                                              \
        static auto* const key = ::base::debug::AllocateCrashKeyString( \
            category "-" name, key_size);                               \
        return key;                                                     \
      }(),                                                              \
      (data))

// This indirection is needed to expand __COUNTER__.
#define SCOPED_CRASH_KEY_STRING_INTERNAL(category, name, nonce, data, \
                                         key_size)                    \
  SCOPED_CRASH_KEY_STRING_INTERNAL2(category, name, nonce, data, key_size)

// Helper macros for putting a local variable crash key on the stack before
// causing a crash or calling CrashWithoutDumping(). `category` and `name`
// should be string literals.
//
//   SCOPED_CRASH_KEY_STRING32("MyCategory", "key_name", "value");
//
// will set the crash key annotation named "MyCategory-key_name" to "value"
// while in scope.
#define SCOPED_CRASH_KEY_STRING32(category, name, data)                 \
  SCOPED_CRASH_KEY_STRING_INTERNAL(category, name, __COUNTER__, (data), \
                                   ::base::debug::CrashKeySize::Size32)

#define SCOPED_CRASH_KEY_STRING64(category, name, data)                 \
  SCOPED_CRASH_KEY_STRING_INTERNAL(category, name, __COUNTER__, (data), \
                                   ::base::debug::CrashKeySize::Size64)

#define SCOPED_CRASH_KEY_STRING256(category, name, data)                \
  SCOPED_CRASH_KEY_STRING_INTERNAL(category, name, __COUNTER__, (data), \
                                   ::base::debug::CrashKeySize::Size256)

#define SCOPED_CRASH_KEY_STRING1024(category, name, data)               \
  SCOPED_CRASH_KEY_STRING_INTERNAL(category, name, __COUNTER__, (data), \
                                   ::base::debug::CrashKeySize::Size1024)

#define SCOPED_CRASH_KEY_BOOL(category, name, data)                       \
  static_assert(std::is_same_v<std::decay_t<decltype(data)>, bool>,       \
                "SCOPED_CRASH_KEY_BOOL must be passed a boolean value."); \
  SCOPED_CRASH_KEY_STRING32(category, name, (data) ? "true" : "false")

#define SCOPED_CRASH_KEY_NUMBER(category, name, data) \
  SCOPED_CRASH_KEY_STRING32(category, name, ::base::NumberToString(data))

////////////////////////////////////////////////////////////////////////////////
// The following declarations are used to initialize the crash key system
// in //base by providing implementations for the above functions.

// The virtual interface that provides the implementation for the crash key
// API. This is implemented by a higher-layer component, and the instance is
// set using the function below.
class CrashKeyImplementation {
 public:
  virtual ~CrashKeyImplementation() = default;

  virtual CrashKeyString* Allocate(const char name[], CrashKeySize size) = 0;
  virtual void Set(CrashKeyString* crash_key, std::string_view value) = 0;
  virtual void Clear(CrashKeyString* crash_key) = 0;
  virtual void OutputCrashKeysToStream(std::ostream& out) = 0;
};

// Initializes the crash key system in base by replacing the existing
// implementation, if it exists, with |impl|. The |impl| is copied into base.
BASE_EXPORT void SetCrashKeyImplementation(
    std::unique_ptr<CrashKeyImplementation> impl);

// The base structure for a crash key, storing the allocation metadata.
struct CrashKeyString {
  constexpr CrashKeyString(const char name[], CrashKeySize size)
      : name(name), size(size) {}
  const char* const name;
  const CrashKeySize size;
};

}  // namespace debug
}  // namespace base

#endif  // BASE_DEBUG_CRASH_LOGGING_H_
