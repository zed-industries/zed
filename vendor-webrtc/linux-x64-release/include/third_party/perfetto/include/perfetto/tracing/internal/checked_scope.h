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

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_CHECKED_SCOPE_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_CHECKED_SCOPE_H_

#include "perfetto/base/export.h"
#include "perfetto/base/logging.h"

namespace perfetto {
namespace internal {

#if PERFETTO_DCHECK_IS_ON()

// Checker to ensure that despite multiple scopes being present, only the active
// one is being accessed. Rules:
// - Only an active scope can create inner scopes. When this happens, it stops
// being active and the inner scope becomes active instead.
// - Only an active scope can be destroyed. When this happens, its parent scope
// becomes active.
class PERFETTO_EXPORT_COMPONENT CheckedScope {
 public:
  explicit CheckedScope(CheckedScope* parent_scope);
  ~CheckedScope();
  CheckedScope(CheckedScope&&);
  CheckedScope& operator=(CheckedScope&&);
  CheckedScope(const CheckedScope&) = delete;
  CheckedScope& operator=(const CheckedScope&) = delete;

  void Reset();

  CheckedScope* parent_scope() const { return parent_scope_; }
  bool is_active() const { return is_active_; }

 private:
  void set_is_active(bool is_active) { is_active_ = is_active; }

  bool is_active_ = true;
  CheckedScope* parent_scope_;

  bool deleted_ = false;
};

#else

// Dummy for cases when DCHECK is not enabled. Methods are marked constexpr to
// ensure that the compiler can inline and optimise them away.
class CheckedScope {
 public:
  inline explicit CheckedScope(CheckedScope*) {}
  inline ~CheckedScope() {}

  CheckedScope(const CheckedScope&) = delete;
  CheckedScope& operator=(const CheckedScope&) = delete;

  CheckedScope(CheckedScope&&) = default;
  CheckedScope& operator=(CheckedScope&&) = default;

  inline void Reset() {}

  inline CheckedScope* parent_scope() const { return nullptr; }
  inline bool is_active() const { return true; }
};

#endif  // PERFETTO_DCHECK_IS_ON()

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_CHECKED_SCOPE_H_
