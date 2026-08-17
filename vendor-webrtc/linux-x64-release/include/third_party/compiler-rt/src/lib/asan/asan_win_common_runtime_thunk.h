//===-- asan_win_common_runtime_thunk.h -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is a part of AddressSanitizer, an address sanity checker.
//
// This file defines things that need to be present in the application modules
// to interact with the ASan DLL runtime correctly and can't be implemented
// using the default "import library" generated when linking the DLL.
//
//===----------------------------------------------------------------------===//

#if defined(SANITIZER_STATIC_RUNTIME_THUNK) || \
    defined(SANITIZER_DYNAMIC_RUNTIME_THUNK)
#  include "sanitizer_common/sanitizer_win_defs.h"

#  pragma section(".CRT$XIB", long, \
                  read)  // C initializer (during C init before dyninit)
#  pragma section(".CRT$XID", long, \
                  read)  // First C initializer after CRT initializers
#  pragma section(".CRT$XCAB", long, \
                  read)  // First C++ initializer after startup initializers

#  pragma section(".CRT$XTW", long, read)  // First ASAN globals terminator
#  pragma section(".CRT$XTY", long, read)  // Last ASAN globals terminator

#  pragma section(".CRT$XLAB", long, read)  // First TLS initializer

#  ifdef SANITIZER_STATIC_RUNTIME_THUNK
extern "C" void __asan_initialize_static_thunk();
#  endif

#endif  // defined(SANITIZER_STATIC_RUNTIME_THUNK) ||
        // defined(SANITIZER_DYNAMIC_RUNTIME_THUNK)