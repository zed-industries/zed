// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_FORBIDDEN_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_FORBIDDEN_SCOPE_H_

#include <optional>

#include "base/auto_reset.h"
#include "third_party/blink/renderer/platform/bindings/v8_throw_exception.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/stack_util.h"
#include "third_party/blink/renderer/platform/wtf/wtf.h"

namespace blink {

class BlinkLifecycleScopeWillBeScriptForbidden;

// Scoped disabling of script execution.
class PLATFORM_EXPORT ScriptForbiddenScope final {
  STACK_ALLOCATED();

 public:
  ScriptForbiddenScope() { Enter(); }
  ScriptForbiddenScope(const ScriptForbiddenScope&) = delete;
  ScriptForbiddenScope& operator=(const ScriptForbiddenScope&) = delete;
  ~ScriptForbiddenScope() { Exit(); }

  class PLATFORM_EXPORT AllowUserAgentScript final {
    STACK_ALLOCATED();

   public:
    AllowUserAgentScript() : saved_counter_(&GetMutableCounter(), 0) {
      if (IsMainThread()) [[likely]] {
        saved_blink_counter_.emplace(&g_blink_lifecycle_counter_, 0);
      }
    }
    AllowUserAgentScript(const AllowUserAgentScript&) = delete;
    AllowUserAgentScript& operator=(const AllowUserAgentScript&) = delete;
    ~AllowUserAgentScript() { DCHECK(!IsScriptForbidden()); }

   private:
    base::AutoReset<unsigned> saved_counter_;
    std::optional<base::AutoReset<unsigned>> saved_blink_counter_;
  };

  static bool IsScriptForbidden() {
    if (!WTF::MayNotBeMainThread()) [[likely]] {
      return g_main_thread_counter_ > 0;
    }
    return GetMutableCounter() > 0;
  }

  // Returns whether we are in a blink lifecycle scope. This should be checked
  // from any location in which we are about to run potentially arbitrary
  // script. It is not safe to run script during the blink lifecycle unless
  // we either check whether it dirtied anything and rerun style/layout, or,
  // can guarantee that script cannot dirty style / layout (e.g. worklet
  // scopes). Use AllowUserAgentScript to annotate known safe points to run
  // script.
  // TODO(crbug.com/1196853): Remove this once we have discovered and fixed
  // sources of attempted script execution during blink lifecycle.
  static bool WillBeScriptForbidden() {
    if (IsMainThread()) [[likely]] {
      return g_blink_lifecycle_counter_ > 0;
    }
    // Blink lifecycle scope is never entered on other threads.
    return false;
  }

  static void ThrowScriptForbiddenException(v8::Isolate* isolate) {
    V8ThrowException::ThrowError(isolate, "Script execution is forbidden.");
  }

 private:
  static void Enter() {
    if (!WTF::MayNotBeMainThread()) [[likely]] {
      ++g_main_thread_counter_;
    } else {
      ++GetMutableCounter();
    }
  }
  static void Exit() {
    DCHECK(IsScriptForbidden());
    if (!WTF::MayNotBeMainThread()) [[likely]] {
      --g_main_thread_counter_;
    } else {
      --GetMutableCounter();
    }
  }

  static void EnterBlinkLifecycle() {
    DCHECK(IsMainThread());
    ++g_blink_lifecycle_counter_;
  }
  static void ExitBlinkLifecycle() {
    DCHECK(IsMainThread());
    --g_blink_lifecycle_counter_;
  }

  static unsigned& GetMutableCounter();

  static unsigned g_main_thread_counter_;

  // TODO(https://crbug.com/1196853): Remove once
  // BlinkLifecycleScopeWillBeScriptForbidden can be removed.
  static unsigned g_blink_lifecycle_counter_;

  // V8GCController is exceptionally allowed to call Enter/Exit.
  friend class V8GCController;
  friend class BlinkLifecycleScopeWillBeScriptForbidden;
};

// Temporarily separate class for identifying cases in which adding a script
// forbidden scope to the blink lifecycle update is causing operations to
// be skipped leading to crashes, see https://crbug.com/1196853.
// TODO(https://crbug.com/1196853): Remove this class and use
// ScriptForbiddenScope once failures are fixed.
class PLATFORM_EXPORT BlinkLifecycleScopeWillBeScriptForbidden final {
  STACK_ALLOCATED();

 public:
  BlinkLifecycleScopeWillBeScriptForbidden() {
    ScriptForbiddenScope::EnterBlinkLifecycle();
  }
  BlinkLifecycleScopeWillBeScriptForbidden(
      const BlinkLifecycleScopeWillBeScriptForbidden&) = delete;
  BlinkLifecycleScopeWillBeScriptForbidden& operator=(
      const BlinkLifecycleScopeWillBeScriptForbidden&) = delete;
  ~BlinkLifecycleScopeWillBeScriptForbidden() {
    ScriptForbiddenScope::ExitBlinkLifecycle();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_FORBIDDEN_SCOPE_H_
