// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_STACK_MAPS_GC_STACK_MAP_PARSER_H_
#define TOOLS_CLANG_STACK_MAPS_GC_STACK_MAP_PARSER_H_

#include "gc_api.h"

// The stackmap section in the binary has a non-trivial layout. We give it a
// char type so it can be iterated byte-by-byte, and re-cast as necessary by the
// parser.
extern const char __LLVM_StackMaps;

namespace stackmap {

// These structs group together fields in the stackmap section to be used by the
// parser. They are packed to prevent clang adding its own alignment. We don't
// care about constants. The LLVM docs for stackmaps can be found here:
// https://llvm.org/docs/StackMaps.html#stack-map-format
//
// As per the docs, a version 3 stackmap has the following layout:
//
//    Header {
//      uint8  : Stack Map Version (current version is 3)
//      uint8  : Reserved (expected to be 0)
//      uint16 : Reserved (expected to be 0)
//    }
//    uint32 : NumFunctions
//    uint32 : NumConstants
//    uint32 : NumRecords
//    StkSizeRecord[NumFunctions] {
//      uint64 : Function Address
//      uint64 : Stack Size
//      uint64 : Record Count
//    }
//    Constants[NumConstants] {
//      uint64 : LargeConstant
//    }
//    StkMapRecord[NumRecords] {
//      uint64 : PatchPoint ID
//      uint32 : Instruction Offset
//      uint16 : Reserved (record flags)
//      uint16 : NumLocations
//      Location[NumLocations] {
//        uint8  : Register | Direct | Indirect | Constant | ConstantIndex
//        uint8  : Reserved (expected to be 0)
//        uint16 : Location Size
//        uint16 : Dwarf RegNum
//        uint16 : Reserved (expected to be 0)
//        int32  : Offset or SmallConstant
//      }
//      uint32 : Padding (only if required to align to 8 byte)
//      uint16 : Padding
//      uint16 : NumLiveOuts
//      LiveOuts[NumLiveOuts]
//        uint16 : Dwarf RegNum
//        uint8  : Reserved
//        uint8  : Size in Bytes
//      }
//      uint32 : Padding (only if required to align to 8 byte)
//    }

struct __attribute__((packed)) StkMapHeader {
  uint8_t version;
  uint8_t reserved1;
  uint16_t reserved2;
  uint32_t num_functions;
  uint32_t num_constants;
  uint32_t num_records;
};

struct __attribute__((packed)) StkSizeRecord {
  uint64_t address;
  uint64_t stack_size;
  uint64_t record_count;  // see https://reviews.llvm.org/D23487
};

struct __attribute__((packed)) StkMapRecordHeader {
  uint64_t patchpoint_id;
  uint32_t return_addr;  // from the entry of the function
  uint16_t flags;
  uint16_t num_locations;
};

enum LocationKind {
  kRegister = 0x1,
  kDirect = 0x2,
  kIndirect = 0x3,
  kConstant = 0x4,
  kConstIndex = 0x5
};

struct __attribute__((packed)) StkMapLocation {
  uint8_t kind;   // 1 byte sized `LocationKind` variant
  uint8_t flags;  // expected to be 0
  uint16_t location_size;
  uint16_t reg_num;   // Dwarf register num
  uint16_t reserved;  // expected to be 0
  int32_t offset;     // either an offset or a "Small Constant"
};

struct __attribute__((packed)) LiveOutsHeader {
  uint16_t padding;
  uint16_t num_liveouts;
};

struct __attribute__((packed)) LiveOut {
  uint16_t reg_num;  // Dwarf register num
  uint8_t flags;
  uint8_t size;  // in bytes
};

// A StackmapV3Parser encapsulates the parsing logic for reading from an
// .llvm_stackmap section in the ELF file. The .llvm_stackmap section is
// versioned and *not* backwards compatible.
class StackmapV3Parser {
 public:
  StackmapV3Parser() : cursor_(&__LLVM_StackMaps) {}

  SafepointTable Parse();

 private:
  static constexpr uint8_t kStackmapVersion = 3;
  static constexpr uint8_t kSizeConstantEntry = 8;  // size in bytes
  static constexpr uint8_t kSkipLocs = 2;

  const char* cursor_;
  const StkMapRecordHeader* cur_frame_;

  // Get a new pointer of the same type to the one passed in arg0 + some byte(s)
  // offset. Useful to prevent littering code with constant char* casting when
  // all that's needed is to bump the ptr by a set amount of bytes. Note this
  // *does not* perform any alignment.
  template <typename T, typename U>
  inline T ptr_offset(U ptr, int bytes) {
    auto* newptr = reinterpret_cast<const char*>(ptr);
    newptr += bytes;
    return reinterpret_cast<T>(newptr);
  }

  // Align a pointer to the next 8 byte boundary
  template <typename T>
  inline T* align_8(T* ptr) {
    auto* c = reinterpret_cast<const char*>(ptr);
    return reinterpret_cast<T*>(((uintptr_t)c + 7) & ~7);
  }

  // Creates a FrameRoot entry for a callsite's stack map record. This jumps
  // over and ignores a bunch of values in the stack map record that are not of
  // interest to precise stack scanning in V8 / Blink. Stack map records make up
  // the bulk of the .llvm_stackmap section. For reference, the format is shown
  // below:
  //    StkMapRecord[NumRecords] {
  //      uint64 : PatchPoint ID
  //      uint32 : Instruction Offset
  //      uint16 : Reserved (record flags)
  //      uint16 : NumLocations
  //      Location[NumLocations] {
  //        uint8  : Register | Direct | Indirect | Constant | ConstantIndex
  //        uint8  : Reserved (expected to be 0)
  //        uint16 : Location Size
  //        uint16 : Dwarf RegNum
  //        uint16 : Reserved (expected to be 0)
  //        int32  : Offset or SmallConstant
  //      }
  //      uint32 : Padding (only if required to align to 8 byte)
  //      uint16 : Padding
  //      uint16 : NumLiveOuts
  //      LiveOuts[NumLiveOuts]
  //        uint16 : Dwarf RegNum
  //        uint8  : Reserved
  //        uint8  : Size in Bytes
  //      }
  FrameRoots ParseFrame();
};

}  // namespace stackmap

#endif  // TOOLS_CLANG_STACK_MAPS_GC_STACK_MAP_PARSER_H_
