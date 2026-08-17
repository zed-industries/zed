// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef PARTITION_ALLOC_BUILD_CONFIG_H_
#define PARTITION_ALLOC_BUILD_CONFIG_H_

// This file is derived from chromium's //build/build_config.h.
//
// Differences:
// - Only the definition used by partition_alloc are included.
// - The definition can only be consumed PA_BUILDFLAG(...) macro. This avoids
//   silent failure when developers forget to include this file. This avoids the
//   need of a PRESUBMIT.py to enforce the inclusion of this file.
//
//
// This files contains the following definition:
//
// Operating system:
//   IS_IOS / IS_AIX / IS_ASMJS / IS_FREEBSD / IS_FUCHSIA / IS_LINUX / IS_MAC /
//   IS_NACL / IS_NETBSD / IS_OPENBSD / IS_QNX / IS_SOLARIS / IS_WIN
//
// Operating system family:
//   IS_APPLE / IS_BSD / IS_POSIX
//
// Compiler:
//  PA_COMPILER_GCC / PA_COMPILER_MSVC
//
// Processor:
//   PA_ARCH_CPU_ARM64 / PA_ARCH_CPU_ARMEL / PA_ARCH_CPU_BIG_ENDIAN /
//   PA_ARCH_CPU_LITTLE_ENDIAN / PA_ARCH_CPU_MIPS / PA_ARCH_CPU_MIPS64 /
//   PA_ARCH_CPU_MIPS64EL / PA_ARCH_CPU_MIPSEL / PA_ARCH_CPU_PPC64 /
//   PA_ARCH_CPU_RISCV64 / PA_ARCH_CPU_S390 / PA_ARCH_CPU_S390X /
//   PA_ARCH_CPU_X86 / PA_ARCH_CPU_X86_64
//
// Processor Family:
//  PA_ARCH_CPU_32_BITS / PA_ARCH_CPU_64_BITS / PA_ARCH_CPU_ARM_FAMILY /
//  PA_ARCH_CPU_LOONGPA_ARCH64 / PA_ARCH_CPU_PPC64_FAMILY /
//  PA_ARCH_CPU_S390_FAMILY / PA_ARCH_CPU_X86_FAMILY
//
// Compiler:
//   PA_COMPILER_GCC / PA_COMPILER_MSVC
//
// Standard library:
//   PA_LIBC_GLIBC

// Definition of PA_BUILDFLAG(...) macro.
#include "partition_alloc/buildflag.h"  // IWYU pragma: export

// Definition of PA_BUILDFLAG(IS_CHROMEOS).
#include "partition_alloc/buildflags.h"  // IWYU pragma: export

// Clangd does not detect PA_BUILDFLAG_INTERNAL_* indirect usage, so mark the
// header as "always_keep" to avoid "unused include" warning.
//
// IWYU pragma: always_keep

// A set of macros to use for platform detection.
#if defined(__native_client__)
// __native_client__ must be first, so that other IS_ defines are not set.
#define PA_IS_NACL
#elif PA_BUILDFLAG(IS_ANDROID)
// The IS_ANDROID PA_BUILDFLAG macro is defined in buildflags.h.
//
// PartitionAlloc's embedders (Chromium, Dawn, Pdfium, Skia) define different
// macros for Android builds: "ANDROID" or "SK_BUILD_FOR_ANDROID".
//
// To avoid relying on these external definitions, PartitionAlloc uses its own
// dedicated build flag.
#elif defined(__APPLE__)
// Only include TargetConditionals after testing ANDROID as some Android builds
// on the Mac have this header available and it's not needed unless the target
// is really an Apple platform.
#include <TargetConditionals.h>
#if defined(TARGET_OS_IPHONE) && TARGET_OS_IPHONE
#define PA_IS_IOS
#else
#define PA_IS_MAC
#endif  // defined(TARGET_OS_IPHONE) && TARGET_OS_IPHONE
#elif defined(__linux__)
#if !PA_BUILDFLAG(IS_CHROMEOS)
// Do not define PA_IS_LINUX on Chrome OS build.
// The IS_CHROMEOS PA_BUILDFLAG macro is defined in buildflags.h.
#define PA_IS_LINUX
#endif  // !PA_BUILDFLAG(IS_CHROMEOS)
// Include a system header to pull in features.h for glibc/uclibc macros.
#include <assert.h>
#if defined(__GLIBC__) && !defined(__UCLIBC__)
// We really are using glibc, not uClibc pretending to be glibc.
#define PA_LIBC_GLIBC
#endif
#elif defined(_WIN32)
#define PA_IS_WIN
#elif defined(__Fuchsia__)
#define PA_IS_FUCHSIA
#elif defined(__FreeBSD__)
#define PA_IS_FREEBSD
#elif defined(__NetBSD__)
#define PA_IS_NETBSD
#elif defined(__OpenBSD__)
#define PA_IS_OPENBSD
#elif defined(__sun)
#define PA_IS_SOLARIS
#elif defined(__QNXNTO__)
#define PA_IS_QNX
#elif defined(_AIX)
#define PA_IS_AIX
#elif defined(__asmjs__) || defined(__wasm__)
#define PA_IS_ASMJS
#endif

// NOTE: Adding a new port? Please follow
// https://chromium.googlesource.com/chromium/src/+/main/docs/new_port_policy.md

#if defined(PA_IS_MAC) || defined(PA_IS_IOS)
#define PA_IS_APPLE
#endif

#if defined(PA_IS_FREEBSD) || defined(PA_IS_NETBSD) || defined(PA_IS_OPENBSD)
#define PA_IS_BSD
#endif

#if defined(PA_IS_AIX) || defined(PA_IS_ASMJS) || defined(PA_IS_FREEBSD) ||   \
    defined(PA_IS_IOS) || defined(PA_IS_LINUX) || defined(PA_IS_CHROMEOS) ||  \
    defined(PA_IS_MAC) || defined(PA_IS_NACL) || defined(PA_IS_NETBSD) ||     \
    defined(PA_IS_OPENBSD) || defined(PA_IS_QNX) || defined(PA_IS_SOLARIS) || \
    PA_BUILDFLAG(IS_ANDROID) || PA_BUILDFLAG(IS_CHROMEOS)
#define PA_IS_POSIX
#endif

// Compiler detection. Note: clang masquerades as GCC on POSIX and as MSVC on
// Windows.
#if defined(__GNUC__)
#define PA_COMPILER_GCC
#elif defined(_MSC_VER)
#define PA_COMPILER_MSVC
#endif
// ------

// Processor architecture detection.  For more info on what's defined, see:
//   http://msdn.microsoft.com/en-us/library/b0084kay.aspx
//   http://www.agner.org/optimize/calling_conventions.pdf
//   or with gcc, run: "echo | gcc -E -dM -"
#if defined(_M_X64) || defined(__x86_64__)
#define PA_ARCH_CPU_X86_FAMILY
#define PA_ARCH_CPU_X86_64
#define PA_ARCH_CPU_64_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#elif defined(_M_IX86) || defined(__i386__)
#define PA_ARCH_CPU_X86_FAMILY
#define PA_ARCH_CPU_X86
#define PA_ARCH_CPU_32_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#elif defined(__s390x__)
#define PA_ARCH_CPU_S390_FAMILY
#define PA_ARCH_CPU_S390X
#define PA_ARCH_CPU_64_BITS
#define PA_ARCH_CPU_BIG_ENDIAN
#elif defined(__s390__)
#define PA_ARCH_CPU_S390_FAMILY
#define PA_ARCH_CPU_S390
#define PA_ARCH_CPU_BIG_ENDIAN
#elif (defined(__PPC64__) || defined(__PPC__)) && defined(__BIG_ENDIAN__)
#define PA_ARCH_CPU_PPC64_FAMILY
#define PA_ARCH_CPU_PPC64
#define PA_ARCH_CPU_64_BITS
#define PA_ARCH_CPU_BIG_ENDIAN
#elif defined(__PPC64__)
#define PA_ARCH_CPU_PPC64_FAMILY
#define PA_ARCH_CPU_PPC64
#define PA_ARCH_CPU_64_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#elif defined(__ARMEL__)
#define PA_ARCH_CPU_ARM_FAMILY
#define PA_ARCH_CPU_ARMEL
#define PA_ARCH_CPU_32_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#elif defined(__aarch64__) || defined(_M_ARM64)
#define PA_ARCH_CPU_ARM_FAMILY
#define PA_ARCH_CPU_ARM64
#define PA_ARCH_CPU_64_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#elif defined(__pnacl__) || defined(__asmjs__) || defined(__wasm__)
#define PA_ARCH_CPU_32_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#elif defined(__MIPSEL__)
#if defined(__LP64__)
#define PA_ARCH_CPU_MIPS64EL
#define PA_ARCH_CPU_64_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#else
#define PA_ARCH_CPU_MIPSEL
#define PA_ARCH_CPU_32_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#endif
#elif defined(__MIPSEB__)
#if defined(__LP64__)
#define PA_ARCH_CPU_MIPS64
#define PA_ARCH_CPU_64_BITS
#define PA_ARCH_CPU_BIG_ENDIAN
#else
#define PA_ARCH_CPU_MIPS
#define PA_ARCH_CPU_32_BITS
#define PA_ARCH_CPU_BIG_ENDIAN
#endif
#elif defined(__loongarch__)
#define PA_ARCH_CPU_LITTLE_ENDIAN
#if __loongarch_grlen == 64
#define PA_ARCH_CPU_LOONGARCH64
#define PA_ARCH_CPU_64_BITS
#else
#define PA_ARCH_CPU_32_BITS
#endif
#elif defined(__riscv) && (__riscv_xlen == 64)
#define PA_ARCH_CPU_RISCV64
#define PA_ARCH_CPU_64_BITS
#define PA_ARCH_CPU_LITTLE_ENDIAN
#endif

// The part below can be generated with the following script:
// https://paste.googleplex.com/6324671838683136
//
// It transform the defines above into PA_BUILDFLAG_INTERNAL_* defines, then
// undef the original define.
//
// Usage of PA_BUILDFLAG(...) macro is better than raw define, because it avoids
// silent failure when developers forget to include this file.

#if defined(PA_ARCH_CPU_32_BITS)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_32_BITS() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_32_BITS() (0)
#endif
#undef PA_ARCH_CPU_32_BITS

#if defined(PA_ARCH_CPU_64_BITS)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_64_BITS() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_64_BITS() (0)
#endif
#undef PA_ARCH_CPU_64_BITS

#if defined(PA_ARCH_CPU_ARM64)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_ARM64() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_ARM64() (0)
#endif
#undef PA_ARCH_CPU_ARM64

#if defined(PA_ARCH_CPU_ARMEL)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_ARMEL() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_ARMEL() (0)
#endif
#undef PA_ARCH_CPU_ARMEL

#if defined(PA_ARCH_CPU_ARM_FAMILY)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_ARM_FAMILY() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_ARM_FAMILY() (0)
#endif
#undef PA_ARCH_CPU_ARM_FAMILY

#if defined(PA_ARCH_CPU_BIG_ENDIAN)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_BIG_ENDIAN() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_BIG_ENDIAN() (0)
#endif
#undef PA_ARCH_CPU_BIG_ENDIAN

#if defined(PA_ARCH_CPU_LITTLE_ENDIAN)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_LITTLE_ENDIAN() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_LITTLE_ENDIAN() (0)
#endif
#undef PA_ARCH_CPU_LITTLE_ENDIAN

#if defined(PA_ARCH_CPU_LOONGARCH64)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_LOONGARCH64() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_LOONGARCH64() (0)
#endif
#undef PA_ARCH_CPU_LOONGARCH64

#if defined(PA_ARCH_CPU_MIPS)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_MIPS() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_MIPS() (0)
#endif
#undef PA_ARCH_CPU_MIPS

#if defined(PA_ARCH_CPU_MIPS64)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_MIPS64() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_MIPS64() (0)
#endif
#undef PA_ARCH_CPU_MIPS64

#if defined(PA_ARCH_CPU_MIPS64EL)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_MIPS64EL() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_MIPS64EL() (0)
#endif
#undef PA_ARCH_CPU_MIPS64EL

#if defined(PA_ARCH_CPU_MIPSEL)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_MIPSEL() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_MIPSEL() (0)
#endif
#undef PA_ARCH_CPU_MIPSEL

#if defined(PA_ARCH_CPU_PPC64)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_PPC64() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_PPC64() (0)
#endif
#undef PA_ARCH_CPU_PPC64

#if defined(PA_ARCH_CPU_PPC64_FAMILY)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_PPC64_FAMILY() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_PPC64_FAMILY() (0)
#endif
#undef PA_ARCH_CPU_PPC64_FAMILY

#if defined(PA_ARCH_CPU_RISCV64)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_RISCV64() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_RISCV64() (0)
#endif
#undef PA_ARCH_CPU_RISCV64

#if defined(PA_ARCH_CPU_S390)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_S390() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_S390() (0)
#endif
#undef PA_ARCH_CPU_S390

#if defined(PA_ARCH_CPU_S390_FAMILY)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_S390_FAMILY() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_S390_FAMILY() (0)
#endif
#undef PA_ARCH_CPU_S390_FAMILY

#if defined(PA_ARCH_CPU_S390X)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_S390X() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_S390X() (0)
#endif
#undef PA_ARCH_CPU_S390X

#if defined(PA_ARCH_CPU_X86)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_X86() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_X86() (0)
#endif
#undef PA_ARCH_CPU_X86

#if defined(PA_ARCH_CPU_X86_64)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_X86_64() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_X86_64() (0)
#endif
#undef PA_ARCH_CPU_X86_64

#if defined(PA_ARCH_CPU_X86_FAMILY)
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_X86_FAMILY() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_ARCH_CPU_X86_FAMILY() (0)
#endif
#undef PA_ARCH_CPU_X86_FAMILY

#if defined(PA_COMPILER_GCC)
#define PA_BUILDFLAG_INTERNAL_PA_COMPILER_GCC() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_COMPILER_GCC() (0)
#endif
#undef PA_COMPILER_GCC

#if defined(PA_COMPILER_MSVC)
#define PA_BUILDFLAG_INTERNAL_PA_COMPILER_MSVC() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_COMPILER_MSVC() (0)
#endif
#undef PA_COMPILER_MSVC

#if defined(PA_IS_AIX)
#define PA_BUILDFLAG_INTERNAL_IS_AIX() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_AIX() (0)
#endif
#undef PA_IS_AIX

#if defined(PA_IS_APPLE)
#define PA_BUILDFLAG_INTERNAL_IS_APPLE() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_APPLE() (0)
#endif
#undef PA_IS_APPLE

#if defined(PA_IS_ASMJS)
#define PA_BUILDFLAG_INTERNAL_IS_ASMJS() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_ASMJS() (0)
#endif
#undef PA_IS_ASMJS

#if defined(PA_IS_BSD)
#define PA_BUILDFLAG_INTERNAL_IS_BSD() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_BSD() (0)
#endif
#undef PA_IS_BSD

#if defined(PA_IS_FREEBSD)
#define PA_BUILDFLAG_INTERNAL_IS_FREEBSD() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_FREEBSD() (0)
#endif
#undef PA_IS_FREEBSD

#if defined(PA_IS_FUCHSIA)
#define PA_BUILDFLAG_INTERNAL_IS_FUCHSIA() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_FUCHSIA() (0)
#endif
#undef PA_IS_FUCHSIA

#if defined(PA_IS_IOS)
#define PA_BUILDFLAG_INTERNAL_IS_IOS() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_IOS() (0)
#endif
#undef PA_IS_IOS

#if defined(PA_IS_LINUX)
#define PA_BUILDFLAG_INTERNAL_IS_LINUX() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_LINUX() (0)
#endif
#undef PA_IS_LINUX

#if defined(PA_IS_MAC)
#define PA_BUILDFLAG_INTERNAL_IS_MAC() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_MAC() (0)
#endif
#undef PA_IS_MAC

#if defined(PA_IS_NACL)
#define PA_BUILDFLAG_INTERNAL_IS_NACL() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_NACL() (0)
#endif
#undef PA_IS_NACL

#if defined(PA_IS_NETBSD)
#define PA_BUILDFLAG_INTERNAL_IS_NETBSD() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_NETBSD() (0)
#endif
#undef PA_IS_NETBSD

#if defined(PA_IS_OPENBSD)
#define PA_BUILDFLAG_INTERNAL_IS_OPENBSD() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_OPENBSD() (0)
#endif
#undef PA_IS_OPENBSD

#if defined(PA_IS_POSIX)
#define PA_BUILDFLAG_INTERNAL_IS_POSIX() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_POSIX() (0)
#endif
#undef PA_IS_POSIX

#if defined(PA_IS_QNX)
#define PA_BUILDFLAG_INTERNAL_IS_QNX() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_QNX() (0)
#endif
#undef PA_IS_QNX

#if defined(PA_IS_SOLARIS)
#define PA_BUILDFLAG_INTERNAL_IS_SOLARIS() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_SOLARIS() (0)
#endif
#undef PA_IS_SOLARIS

#if defined(PA_IS_WIN)
#define PA_BUILDFLAG_INTERNAL_IS_WIN() (1)
#else
#define PA_BUILDFLAG_INTERNAL_IS_WIN() (0)
#endif
#undef PA_IS_WIN

#if defined(PA_LIBC_GLIBC)
#define PA_BUILDFLAG_INTERNAL_PA_LIBC_GLIBC() (1)
#else
#define PA_BUILDFLAG_INTERNAL_PA_LIBC_GLIBC() (0)
#endif
#undef PA_LIBC_GLIBC

#endif  // PARTITION_ALLOC_BUILD_CONFIG_H_
