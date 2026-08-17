// Copyright 2018 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_WIN_CONTEXT_WRAPPERS_H_
#define CRASHPAD_UTIL_WIN_CONTEXT_WRAPPERS_H_

#include <windows.h>

#include "build/build_config.h"

namespace crashpad {

//! \brief Retrieve program counter from `CONTEXT` structure for different
//!     architectures supported by Windows.
inline void* ProgramCounterFromCONTEXT(const CONTEXT* context) {
#if defined(ARCH_CPU_X86)
  return reinterpret_cast<void*>(context->Eip);
#elif defined(ARCH_CPU_X86_64)
  return reinterpret_cast<void*>(context->Rip);
#elif defined(ARCH_CPU_ARM64)
  return reinterpret_cast<void*>(context->Pc);
#else
#error Unsupported Windows Arch
#endif  // ARCH_CPU_X86
}

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_WIN_CONTEXT_WRAPPERS_H_
