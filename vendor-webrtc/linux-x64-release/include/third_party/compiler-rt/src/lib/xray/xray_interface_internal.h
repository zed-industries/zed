//===-- xray_interface_internal.h -------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is a part of XRay, a dynamic runtime instrumentation system.
//
// Implementation of the API functions. See also include/xray/xray_interface.h.
//
//===----------------------------------------------------------------------===//
#ifndef XRAY_INTERFACE_INTERNAL_H
#define XRAY_INTERFACE_INTERNAL_H

#include "sanitizer_common/sanitizer_platform.h"
#include "xray/xray_interface.h"
#include <cstddef>
#include <cstdint>
#include <utility>

extern "C" {
// The following functions have to be defined in assembler, on a per-platform
// basis. See xray_trampoline_*.S files for implementations.
extern void __xray_FunctionEntry();
extern void __xray_FunctionExit();
extern void __xray_FunctionTailExit();
extern void __xray_ArgLoggerEntry();
extern void __xray_CustomEvent();
extern void __xray_TypedEvent();
#if defined(__s390x__)
extern void __xray_FunctionEntryVec();
extern void __xray_FunctionExitVec();
#endif
}

extern "C" {

struct XRaySledEntry {
#if SANITIZER_WORDSIZE == 64
  uint64_t Address;
  uint64_t Function;
  unsigned char Kind;
  unsigned char AlwaysInstrument;
  unsigned char Version;
  unsigned char Padding[13]; // Need 32 bytes
  uint64_t function() const {
    // The target address is relative to the location of the Function variable.
    return reinterpret_cast<uint64_t>(&Function) + Function;
  }
  uint64_t address() const {
    // The target address is relative to the location of the Address variable.
    return reinterpret_cast<uint64_t>(&Address) + Address;
  }
#elif SANITIZER_WORDSIZE == 32
  uint32_t Address;
  uint32_t Function;
  unsigned char Kind;
  unsigned char AlwaysInstrument;
  unsigned char Version;
  unsigned char Padding[5]; // Need 16 bytes
  uint32_t function() const {
    // The target address is relative to the location of the Function variable.
    return reinterpret_cast<uint32_t>(&Function) + Function;
  }
  uint32_t address() const {
    // The target address is relative to the location of the Address variable.
    return reinterpret_cast<uint32_t>(&Address) + Address;
  }
#else
#error "Unsupported word size."
#endif
};

struct XRayFunctionSledIndex {
  const XRaySledEntry *Begin;
  size_t Size;
  // For an entry in the xray_fn_idx section, the address is relative to the
  // location of the Begin variable.
  const XRaySledEntry *fromPCRelative() const {
    return reinterpret_cast<const XRaySledEntry *>(uintptr_t(&Begin) +
                                                   uintptr_t(Begin));
  }
};

struct XRayTrampolines {
  void (*EntryTrampoline)();
  void (*ExitTrampoline)();
  void (*TailExitTrampoline)();
  void (*LogArgsTrampoline)();

  XRayTrampolines() {
    // These resolve to the definitions in the respective executable or DSO.
    EntryTrampoline = __xray_FunctionEntry;
    ExitTrampoline = __xray_FunctionExit;
    TailExitTrampoline = __xray_FunctionTailExit;
    LogArgsTrampoline = __xray_ArgLoggerEntry;
  }
};

extern int32_t __xray_register_dso(const XRaySledEntry *SledsBegin,
                                   const XRaySledEntry *SledsEnd,
                                   const XRayFunctionSledIndex *FnIndexBegin,
                                   const XRayFunctionSledIndex *FnIndexEnd,
                                   XRayTrampolines Trampolines);

extern bool __xray_deregister_dso(int32_t ObjId);
}

namespace __xray {

constexpr uint32_t XRayNFnBits = 24;
constexpr uint32_t XRayNObjBits = 8;

constexpr uint32_t XRayFnBitMask = 0x00FFFFFF;
constexpr uint32_t XRayObjBitMask = 0xFF000000;

constexpr size_t XRayMaxFunctions = 1 << XRayNFnBits;
constexpr size_t XRayMaxObjects = 1 << XRayNObjBits;

inline int32_t MakePackedId(int32_t FnId, int32_t ObjId) {
  return ((ObjId << XRayNFnBits) & XRayObjBitMask) | (FnId & XRayFnBitMask);
}

inline std::pair<int32_t, int32_t> UnpackId(int32_t PackedId) {
  uint32_t ObjId = (PackedId & XRayObjBitMask) >> XRayNFnBits;
  uint32_t FnId = PackedId & XRayFnBitMask;
  return {ObjId, FnId};
}

struct XRaySledMap {
  const XRaySledEntry *Sleds;
  size_t Entries;
  const XRayFunctionSledIndex *SledsIndex;
  size_t Functions;
  XRayTrampolines Trampolines;
  bool FromDSO;
  bool Loaded;
};

bool patchFunctionEntry(bool Enable, uint32_t FuncId, const XRaySledEntry &Sled,
                        const XRayTrampolines &Trampolines, bool LogArgs);
bool patchFunctionExit(bool Enable, uint32_t FuncId, const XRaySledEntry &Sled,
                       const XRayTrampolines &Trampolines);
bool patchFunctionTailExit(bool Enable, uint32_t FuncId,
                           const XRaySledEntry &Sled,
                           const XRayTrampolines &Trampolines);
bool patchCustomEvent(bool Enable, uint32_t FuncId, const XRaySledEntry &Sled);
bool patchTypedEvent(bool Enable, uint32_t FuncId, const XRaySledEntry &Sled);

} // namespace __xray

#endif
