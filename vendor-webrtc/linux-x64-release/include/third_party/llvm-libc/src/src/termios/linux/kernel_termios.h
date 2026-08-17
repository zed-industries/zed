//===-- Definition of kernel's version of struct termios --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_TERMIOS_LINUX_KERNEL_TERMIOS_H
#define LLVM_LIBC_SRC_TERMIOS_LINUX_KERNEL_TERMIOS_H

#include "src/__support/macros/config.h"
#include <stddef.h>
#include <termios.h>

namespace LIBC_NAMESPACE_DECL {

// The kernel's struct termios is different from the libc's struct termios. The
// kernel's syscalls expect the size and layout of its definition of struct
// termios. So, we define a flavor of struct termios which matches that of the
// kernel so that we can translate between the libc version and the kernel
// version when passing struct termios objects to syscalls.

// NOTE: The definitions here are generic definitions valid for most target
// architectures including x86_64 and aarch64. Definitions on some architectures
// deviate from these generic definitions. Adjustments have to be made for those
// architectures.

constexpr size_t KERNEL_NCCS = 19;

struct kernel_termios {
  tcflag_t c_iflag;
  tcflag_t c_oflag;
  tcflag_t c_cflag;
  tcflag_t c_lflag;
  cc_t c_line;
  cc_t c_cc[KERNEL_NCCS];
};

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_TERMIOS_LINUX_KERNEL_TERMIOS_H
