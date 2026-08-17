//===------------------------ tysan_platform.h ----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is a part of TypeSanitizer.
//
// Platform specific information for TySan.
//===----------------------------------------------------------------------===//

#ifndef TYSAN_PLATFORM_H
#define TYSAN_PLATFORM_H

namespace __tysan {

#if defined(__x86_64__) || SANITIZER_APPLE
struct Mapping {
  static const uptr kShadowAddr = 0x010000000000ull;
  static const uptr kAppAddr = 0x550000000000ull;
  static const uptr kAppMemMsk = ~0x780000000000ull;
};
#elif defined(__aarch64__)
struct Mapping39 {
  static const uptr kShadowAddr = 0x0800000000ull;
  static const uptr kAppAddr = 0x5500000000ull;
  static const uptr kAppMemMsk = ~0x7800000000ull;
};

struct Mapping42 {
  static const uptr kShadowAddr = 0x10000000000ull;
  static const uptr kAppAddr = 0x2aa00000000ull;
  static const uptr kAppMemMsk = ~0x3c000000000ull;
};

struct Mapping48 {
  static const uptr kShadowAddr = 0x0002000000000ull;
  static const uptr kAppAddr = 0x0aaaa00000000ull;
  static const uptr kAppMemMsk = ~0x0fff800000000ull;
};
#define TYSAN_RUNTIME_VMA 1
#else
#error "TySan not supported for this platform!"
#endif

#if TYSAN_RUNTIME_VMA
extern int vmaSize;
#endif

enum MappingType { MAPPING_SHADOW_ADDR, MAPPING_APP_ADDR, MAPPING_APP_MASK };

template <typename Mapping, int Type> uptr MappingImpl(void) {
  switch (Type) {
  case MAPPING_SHADOW_ADDR:
    return Mapping::kShadowAddr;
  case MAPPING_APP_ADDR:
    return Mapping::kAppAddr;
  case MAPPING_APP_MASK:
    return Mapping::kAppMemMsk;
  }
}

template <int Type> uptr MappingArchImpl(void) {
#if defined(__aarch64__) && !SANITIZER_APPLE
  switch (vmaSize) {
  case 39:
    return MappingImpl<Mapping39, Type>();
  case 42:
    return MappingImpl<Mapping42, Type>();
  case 48:
    return MappingImpl<Mapping48, Type>();
  }
  DCHECK(0);
  return 0;
#else
  return MappingImpl<Mapping, Type>();
#endif
}

ALWAYS_INLINE
uptr ShadowAddr() { return MappingArchImpl<MAPPING_SHADOW_ADDR>(); }

ALWAYS_INLINE
uptr AppAddr() { return MappingArchImpl<MAPPING_APP_ADDR>(); }

ALWAYS_INLINE
uptr AppMask() { return MappingArchImpl<MAPPING_APP_MASK>(); }

} // namespace __tysan

#endif
