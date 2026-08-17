/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_CRASH_KEYS_H_
#define INCLUDE_PERFETTO_EXT_BASE_CRASH_KEYS_H_

#include <algorithm>
#include <atomic>

#include <stdint.h>
#include <string.h>

#include "perfetto/base/compiler.h"
#include "perfetto/ext/base/string_view.h"

// Crash keys are very simple global variables with static-storage that
// are reported on crash time for managed crashes (CHECK/FATAL/Watchdog).
// - Translation units can define a CrashKey and register it at some point
//   during initialization.
// - CrashKey instances must be long-lived. They should really be just global
//   static variable in the anonymous namespace.
// Example:
// subsystem_1.cc
//   CrashKey g_client_id("ipc_client_id");
//   ...
//   OnIpcReceived(client_id) {
//      g_client_id.Set(client_id);
//      ... // Process the IPC
//      g_client_id.Clear();
//   }
//   Or equivalently:
//   OnIpcReceived(client_id) {
//      auto scoped_key = g_client_id.SetScoped(client_id);
//      ... // Process the IPC
//   }
//
// If a crash happens while processing the IPC, the crash report will
// have a line "ipc_client_id: 42".
//
// Thread safety considerations:
// CrashKeys can be registered and set/cleared from any thread.
// There is no compelling use-case to have full acquire/release consistency when
// setting a key. This means that if a thread crashes immediately after a
// crash key has been set on another thread, the value printed on the crash
// report could be incomplete. The code guarantees defined behavior and does
// not rely on null-terminated string (in the worst case 32 bytes of random
// garbage will be printed out).

// The tests live in logging_unittest.cc.

namespace perfetto {
namespace base {

constexpr size_t kCrashKeyMaxStrSize = 32;

// CrashKey instances must be long lived
class CrashKey {
 public:
  class ScopedClear {
   public:
    explicit ScopedClear(CrashKey* k) : key_(k) {}
    ~ScopedClear() {
      if (key_)
        key_->Clear();
    }
    ScopedClear(const ScopedClear&) = delete;
    ScopedClear& operator=(const ScopedClear&) = delete;
    ScopedClear& operator=(ScopedClear&&) = delete;
    ScopedClear(ScopedClear&& other) noexcept : key_(other.key_) {
      other.key_ = nullptr;
    }

   private:
    CrashKey* key_;
  };

  // constexpr so it can be used in the anon namespace without requiring a
  // global constructor.
  // |name| must be a long-lived string.
  constexpr explicit CrashKey(const char* name)
      : registered_{}, type_(Type::kUnset), name_(name), str_value_{} {}
  CrashKey(const CrashKey&) = delete;
  CrashKey& operator=(const CrashKey&) = delete;
  CrashKey(CrashKey&&) = delete;
  CrashKey& operator=(CrashKey&&) = delete;

  enum class Type : uint8_t { kUnset = 0, kInt, kStr };

  void Clear() {
    int_value_.store(0, std::memory_order_relaxed);
    type_.store(Type::kUnset, std::memory_order_relaxed);
  }

  void Set(int64_t value) {
    int_value_.store(value, std::memory_order_relaxed);
    type_.store(Type::kInt, std::memory_order_relaxed);
    if (PERFETTO_UNLIKELY(!registered_.load(std::memory_order_relaxed)))
      Register();
  }

  void Set(StringView sv) {
    size_t len = std::min(sv.size(), sizeof(str_value_) - 1);
    for (size_t i = 0; i < len; ++i)
      str_value_[i].store(sv.data()[i], std::memory_order_relaxed);
    str_value_[len].store('\0', std::memory_order_relaxed);
    type_.store(Type::kStr, std::memory_order_relaxed);
    if (PERFETTO_UNLIKELY(!registered_.load(std::memory_order_relaxed)))
      Register();
  }

  ScopedClear SetScoped(int64_t value) PERFETTO_WARN_UNUSED_RESULT {
    Set(value);
    return ScopedClear(this);
  }

  ScopedClear SetScoped(StringView sv) PERFETTO_WARN_UNUSED_RESULT {
    Set(sv);
    return ScopedClear(this);
  }

  void Register();

  int64_t int_value() const {
    return int_value_.load(std::memory_order_relaxed);
  }
  size_t ToString(char* dst, size_t len);

 private:
  std::atomic<bool> registered_;
  std::atomic<Type> type_;
  const char* const name_;
  union {
    std::atomic<char> str_value_[kCrashKeyMaxStrSize];
    std::atomic<int64_t> int_value_;
  };
};

// Fills |dst| with a string containing one line for each crash key
// (excluding the unset ones).
// Returns number of chars written, without counting the NUL terminator.
// This is used in logging.cc when emitting the crash report abort message.
size_t SerializeCrashKeys(char* dst, size_t len);

void UnregisterAllCrashKeysForTesting();

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_CRASH_KEYS_H_
