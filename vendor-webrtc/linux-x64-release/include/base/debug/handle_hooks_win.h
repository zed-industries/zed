// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_HANDLE_HOOKS_WIN_H_
#define BASE_DEBUG_HANDLE_HOOKS_WIN_H_

#include "base/base_export.h"
#include "base/win/windows_types.h"
#include "build/build_config.h"

namespace base {
namespace debug {

// Provides the ability to intercept functions which could possibly close
// handles in support of the handle tracker.
// This is a currently a container class for static functions because there is
// ongoing work to make the patches unhook, currently blocked by test failures.
// See https://crbug.com/1327397.
class BASE_EXPORT HandleHooks {
 public:
  HandleHooks() = delete;

  HandleHooks(const HandleHooks&) = delete;
  HandleHooks& operator=(const HandleHooks&) = delete;

  // Patch IAT for a specified module.
  static void AddIATPatch(HMODULE module);
  // Add an EAT patch on kernel32.dll. This patch does not get removed. This is
  // only supported on 32-bit because the EAT only supports 32-bit RVAs.
#if defined(ARCH_CPU_32_BITS)
  static void AddEATPatch();
#endif
  // Patch IAT for all currently loaded modules.
  static void PatchLoadedModules();
};

}  // namespace debug
}  // namespace base

#endif  // BASE_DEBUG_HANDLE_HOOKS_WIN_H_
