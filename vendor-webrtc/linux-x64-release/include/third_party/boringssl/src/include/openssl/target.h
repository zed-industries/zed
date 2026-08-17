// Copyright 2023 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_TARGET_H
#define OPENSSL_HEADER_TARGET_H

// Preprocessor symbols that define the target platform.
//
// This file may be included in C, C++, and assembler and must be compatible
// with each environment. It is separated out only to share code between
// <openssl/base.h> and <openssl/asm_base.h>. Prefer to include those headers
// instead.

#if defined(__x86_64) || defined(_M_AMD64) || defined(_M_X64)
#define OPENSSL_64_BIT
#define OPENSSL_X86_64
#elif defined(__x86) || defined(__i386) || defined(__i386__) || defined(_M_IX86)
#define OPENSSL_32_BIT
#define OPENSSL_X86
#elif defined(__AARCH64EL__) || defined(_M_ARM64)
#define OPENSSL_64_BIT
#define OPENSSL_AARCH64
#elif defined(__ARMEL__) || defined(_M_ARM)
#define OPENSSL_32_BIT
#define OPENSSL_ARM
#elif defined(__MIPSEL__) && !defined(__LP64__)
#define OPENSSL_32_BIT
#define OPENSSL_MIPS
#elif defined(__MIPSEL__) && defined(__LP64__)
#define OPENSSL_64_BIT
#define OPENSSL_MIPS64
#elif defined(__riscv) && __SIZEOF_POINTER__ == 8
#define OPENSSL_64_BIT
#define OPENSSL_RISCV64
#elif defined(__riscv) && __SIZEOF_POINTER__ == 4
#define OPENSSL_32_BIT
#elif defined(__pnacl__)
#define OPENSSL_32_BIT
#define OPENSSL_PNACL
#elif defined(__wasm__)
#define OPENSSL_32_BIT
#elif defined(__asmjs__)
#define OPENSSL_32_BIT
#elif defined(__myriad2__)
#define OPENSSL_32_BIT
#else
// The list above enumerates the platforms that BoringSSL supports. For these
// platforms we keep a reasonable bar of not breaking them: automated test
// coverage, for one, but also we need access to these types for machines for
// fixing them.
//
// However, we know that anything that seems to work will soon be expected
// to work and, quickly, the implicit expectation is that every machine will
// always work. So this list serves to mark the boundary of what we guarantee.
// Of course, you can run the code any many more machines, but then you're
// taking on the burden of fixing it and, if you're doing that, then you must
// be able to carry local patches. In which case patching this list is trivial.
//
// BoringSSL will only possibly work on standard 32-bit and 64-bit
// two's-complement, little-endian architectures. Functions will not produce
// the correct answer on other systems. Run the crypto_test binary, notably
// crypto/compiler_test.cc, before trying a new architecture.
#error "Unknown target CPU"
#endif

#if defined(__APPLE__)
#define OPENSSL_APPLE
#endif

#if defined(_WIN32)
#define OPENSSL_WINDOWS
#endif

// Trusty and Android baremetal aren't Linux but currently define __linux__.
// As a workaround, we exclude them here.
// We also exclude nanolibc/CrOS EC. nanolibc/CrOS EC sometimes build for a
// non-Linux target (which should not define __linux__), but also sometimes
// build for Linux. Although technically running in Linux userspace, this lacks
// all the libc APIs we'd normally expect on Linux, so we treat it as a
// non-Linux target.
//
// TODO(b/169780122): Remove this workaround once Trusty no longer defines it.
// TODO(b/291101350): Remove this workaround once Android baremetal no longer
// defines it.
#if defined(__linux__) && !defined(__TRUSTY__) && \
    !defined(ANDROID_BAREMETAL) && !defined(OPENSSL_NANOLIBC) && \
    !defined(CROS_EC)
#define OPENSSL_LINUX
#endif

#if defined(__Fuchsia__)
#define OPENSSL_FUCHSIA
#endif

// Trusty is Android's TEE target. See
// https://source.android.com/docs/security/features/trusty
//
// Defining this on any other platform is not supported. Other embedded
// platforms must introduce their own defines.
#if defined(__TRUSTY__)
#define OPENSSL_TRUSTY
#define OPENSSL_NO_FILESYSTEM
#define OPENSSL_NO_POSIX_IO
#define OPENSSL_NO_SOCK
#define OPENSSL_NO_THREADS_CORRUPT_MEMORY_AND_LEAK_SECRETS_IF_THREADED
#endif

// nanolibc is a particular minimal libc implementation. Defining this on any
// other platform is not supported. Other embedded platforms must introduce
// their own defines.
#if defined(OPENSSL_NANOLIBC)
#define OPENSSL_NO_FILESYSTEM
#define OPENSSL_NO_POSIX_IO
#define OPENSSL_NO_SOCK
#define OPENSSL_NO_THREADS_CORRUPT_MEMORY_AND_LEAK_SECRETS_IF_THREADED
#endif

// Android baremetal is an embedded target that uses a subset of bionic.
// Defining this on any other platform is not supported. Other embedded
// platforms must introduce their own defines.
#if defined(ANDROID_BAREMETAL)
#define OPENSSL_NO_FILESYSTEM
#define OPENSSL_NO_POSIX_IO
#define OPENSSL_NO_SOCK
#define OPENSSL_NO_THREADS_CORRUPT_MEMORY_AND_LEAK_SECRETS_IF_THREADED
#endif

// CROS_EC is an embedded target for ChromeOS Embedded Controller. Defining
// this on any other platform is not supported. Other embedded platforms must
// introduce their own defines.
//
// https://chromium.googlesource.com/chromiumos/platform/ec/+/HEAD/README.md
#if defined(CROS_EC)
#define OPENSSL_NO_FILESYSTEM
#define OPENSSL_NO_POSIX_IO
#define OPENSSL_NO_SOCK
#define OPENSSL_NO_THREADS_CORRUPT_MEMORY_AND_LEAK_SECRETS_IF_THREADED
#endif

// Zephyr is an open source RTOS, optimized for embedded devices.
// Defining this on any other platform is not supported. Other embedded
// platforms must introduce their own defines.
//
// Zephyr supports multithreading with cooperative and preemptive scheduling.
// It also implements POSIX Threads (pthread) API, so it's not necessary to
// implement BoringSSL internal threading API using some custom API.
//
// https://www.zephyrproject.org/
#if defined(__ZEPHYR__)
#define OPENSSL_NO_FILESYSTEM
#define OPENSSL_NO_POSIX_IO
#define OPENSSL_NO_SOCK
#endif

#if defined(__ANDROID_API__)
#define OPENSSL_ANDROID
#endif

#if defined(__FreeBSD__)
#define OPENSSL_FREEBSD
#endif

#if defined(__OpenBSD__)
#define OPENSSL_OPENBSD
#endif

// BoringSSL requires platform's locking APIs to make internal global state
// thread-safe, including the PRNG. On some single-threaded embedded platforms,
// locking APIs may not exist, so this dependency may be disabled with the
// following build flag.
//
// IMPORTANT: Doing so means the consumer promises the library will never be
// used in any multi-threaded context. It causes BoringSSL to be globally
// thread-unsafe. Setting it inappropriately will subtly and unpredictably
// corrupt memory and leak secret keys.
//
// Do not set this flag on any platform where threads are possible. BoringSSL
// maintainers will not provide support for any consumers that do so. Changes
// which break such unsupported configurations will not be reverted.
#if !defined(OPENSSL_NO_THREADS_CORRUPT_MEMORY_AND_LEAK_SECRETS_IF_THREADED)
#define OPENSSL_THREADS
#endif

#if defined(__has_feature)
#if __has_feature(address_sanitizer)
#define OPENSSL_ASAN
#endif
#if __has_feature(thread_sanitizer)
#define OPENSSL_TSAN
#endif
#if __has_feature(memory_sanitizer)
#define OPENSSL_MSAN
#define OPENSSL_ASM_INCOMPATIBLE
#endif
#if __has_feature(hwaddress_sanitizer)
#define OPENSSL_HWASAN
#endif
#endif

// Disable 32-bit Arm assembly on Apple platforms. The last iOS version that
// supported 32-bit Arm was iOS 10.
#if defined(OPENSSL_APPLE) && defined(OPENSSL_ARM)
#define OPENSSL_ASM_INCOMPATIBLE
#endif

#if defined(OPENSSL_ASM_INCOMPATIBLE)
#undef OPENSSL_ASM_INCOMPATIBLE
#if !defined(OPENSSL_NO_ASM)
#define OPENSSL_NO_ASM
#endif
#endif  // OPENSSL_ASM_INCOMPATIBLE

// We do not detect any features at runtime on several 32-bit Arm platforms.
// Apple platforms and OpenBSD require NEON and moved to 64-bit to pick up Armv8
// extensions. Android baremetal does not aim to support 32-bit Arm at all, but
// it simplifies things to make it build.
#if defined(OPENSSL_ARM) && !defined(OPENSSL_STATIC_ARMCAP) && \
    (defined(OPENSSL_APPLE) || defined(OPENSSL_OPENBSD) ||     \
     defined(ANDROID_BAREMETAL))
#define OPENSSL_STATIC_ARMCAP
#endif

#endif  // OPENSSL_HEADER_TARGET_H
