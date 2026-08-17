// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_JNI_EXPORT_H_
#define JNI_ZERO_JNI_EXPORT_H_

#if defined(COMPONENT_BUILD)
#define JNI_ZERO_COMPONENT_BUILD_EXPORT __attribute__((visibility("default")))
#else
#define JNI_ZERO_COMPONENT_BUILD_EXPORT
#endif

#if defined(__i386__)
// Dalvik JIT generated code doesn't guarantee 16-byte stack alignment on
// x86 - use force_align_arg_pointer to realign the stack at the JNI
// boundary. crbug.com/655248
#define JNI_ZERO_BOUNDARY_EXPORT \
  extern "C" __attribute__((visibility("default"), force_align_arg_pointer))
#else
#define JNI_ZERO_BOUNDARY_EXPORT \
  extern "C" __attribute__((visibility("default")))
#endif

#if defined(__clang__) && __has_attribute(noinline)
#define JNI_ZERO_NEVER_INLINE [[clang::noinline]]
#elif __has_attribute(noinline)
#define JNI_ZERO_NEVER_INLINE __attribute__((noinline))
#else
#define JNI_ZERO_NEVER_INLINE
#endif

#if defined(__clang__) && __has_attribute(always_inline)
#define JNI_ZERO_ALWAYS_INLINE [[clang::always_inline]]
#elif __has_attribute(always_inline)
#define JNI_ZERO_ALWAYS_INLINE __attribute__((__always_inline__))
#else
#define JNI_ZERO_ALWAYS_INLINE
#endif

// extern "C" used to ensure symbol is not within a namespace.
#define JNI_ZERO_MUXED_ENTRYPOINT extern "C" JNI_ZERO_ALWAYS_INLINE

#endif  // JNI_ZERO_JNI_EXPORT_H_
