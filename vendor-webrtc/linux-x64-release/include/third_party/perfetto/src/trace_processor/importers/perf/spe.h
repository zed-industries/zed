/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Collection of constant and utilities to parse SPE data.
// SPE packet spec can be found here:
// Arm Architecture Reference Manual for A-profile architecture
// https://developer.arm.com/documentation/ddi0487/latest/

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_H_

#include <cstddef>
#include <cstdint>
#include "perfetto/base/logging.h"
#include "perfetto/public/compiler.h"

namespace perfetto::trace_processor::perf_importer::spe {

// Test whether a given bit is set. e.g.
// IsBitSet<1>(0b0010) == true
// IsBitSet<0>(0b0010) == false
template <int bit, typename T>
inline constexpr bool IsBitSet(T value) {
  static_assert(std::is_unsigned_v<T>);
  static_assert(bit < sizeof(T) * 8);
  return value & (T(1) << bit);
}

// Index value in Address packets
enum class AddressIndex : uint8_t {
  kInstruction,
  kBranchTarget,
  kDataVirtual,
  kDataPhysical,
  kPrevBranchTarget,
  kUnknown,
  kMax = kUnknown,
};

// Index value in Counter packets
enum class CounterIndex : uint8_t {
  kTotalLatency,
  kIssueLatency,
  kTranslationLatency,
  kUnknown,
  kMax = kUnknown,
};

enum class ContextIndex : uint8_t {
  kEl1,
  kEl2,
  kUnknown,
  kMax = kUnknown,
};

// Operation class for OperationType packets
enum class OperationClass : uint8_t {
  kOther,
  kLoadOrStoreOrAtomic,
  kBranchOrExceptionReturn,
  kUnknown,
  kMax = kUnknown,
};

// Data source types for a payload of a DataSource packet
enum class DataSource : uint8_t {
  kL1D,
  kL2,
  kPeerCore,
  kLocalCluster,
  kSysCache,
  kPeerCluster,
  kRemote,
  kDram,
  kUnknown,
  kMax = kUnknown,
};

// Exception levels instructions can execute in.
enum class ExceptionLevel { kEl0, kEl1, kEl2, kEl3, kMax = kEl3 };

// Common constants to both short and extended headers
constexpr uint8_t COMMON_HEADER_MASK = 0b1111'1000;
constexpr uint8_t COMMON_HEADER_ADDRESS_PACKET = 0b1011'0000;
constexpr uint8_t COMMON_HEADER_COUNTER_PACKET = 0b1001'1000;

constexpr uint8_t COMMON_HEADER_SIZE_MASK = 0b0011'0000;
constexpr uint8_t COMMON_HEADER_SIZE_MASK_RSHIFT = 4;

constexpr uint8_t COMMON_HEADER_NO_PAYLOAD_MASK = 0b1110'0000;
constexpr uint8_t COMMON_HEADER_NO_PAYLOAD = 0b0000'0000;

// Constants for short headers
constexpr uint8_t SHORT_HEADER_PADDING = 0b0000'0000;
constexpr uint8_t SHORT_HEADER_END_PACKET = 0b0000'0001;
constexpr uint8_t SHORT_HEADER_TIMESTAMP_PACKET = 0b0111'0001;

constexpr uint8_t SHORT_HEADER_MASK_1 = 0b1100'1111;
constexpr uint8_t SHORT_HEADER_EVENTS_PACKET = 0b0100'0010;
constexpr uint8_t SHORT_HEADER_DATA_SOURCE_PACKET = 0b0100'0011;

constexpr uint8_t SHORT_HEADER_MASK_2 = 0b1111'1100;
constexpr uint8_t SHORT_HEADER_CONTEXT_PACKET = 0b0110'0100;
constexpr uint8_t SHORT_HEADER_OPERATION_TYPE_PACKET = 0b0100'1000;

constexpr uint8_t SHORT_HEADER_INDEX_MASK = 0b0000'0111;

// Constants for extended headers
constexpr uint8_t EXTENDED_HEADER_MASK = 0b1110'0000;
constexpr uint8_t EXTENDED_HEADER = 0b0010'0000;

constexpr uint8_t EXTENDED_HEADER_INDEX_MASK = 0b0000'0011;
constexpr uint8_t EXTENDED_HEADER_INDEX_LSHIFT = 3;

// OperationType packet constants
constexpr uint8_t PKT_OP_TYPE_HEADER_CLASS_MASK = 0b0000'0011;
constexpr uint8_t PKT_OP_TYPE_HEADER_CLASS_OTHER = 0b0000'0000;
constexpr uint8_t PKT_OP_TYPE_HEADER_CLASS_LD_ST_ATOMIC = 0b0000'0001;
constexpr uint8_t PKT_OP_TYPE_HEADER_CLASS_BR_ERET = 0b0000'0010;

constexpr uint8_t PKT_OP_TYPE_PAYLOAD_SUBCLASS_OTHER_MASK = 0b1111'1110;
constexpr uint8_t PKT_OP_TYPE_PAYLOAD_SUBCLASS_OTHER = 0b0000'0000;

constexpr uint8_t PKT_OP_TYPE_PAYLOAD_SUBCLASS_SVE_OTHER_MASK = 0b1000'1001;
constexpr uint8_t PKT_OP_TYPE_PAYLOAD_SUBCLASS_SVE_OTHER = 0b0000'1000;

// DataSource packet constants
constexpr uint16_t PKT_DATA_SOURCE_PAYLOAD_L1D = 0b0000'0000;
constexpr uint16_t PKT_DATA_SOURCE_PAYLOAD_L2 = 0b0000'1000;
constexpr uint16_t PKT_DATA_SOURCE_PAYLOAD_PEER_CORE = 0b0000'1001;
constexpr uint16_t PKT_DATA_SOURCE_PAYLOAD_LOCAL_CLUSTER = 0b0000'1010;
constexpr uint16_t PKT_DATA_SOURCE_PAYLOAD_SYS_CACHE = 0b0000'1011;
constexpr uint16_t PKT_DATA_SOURCE_PAYLOAD_PEER_CLUSTER = 0b0000'1100;
constexpr uint16_t PKT_DATA_SOURCE_PAYLOAD_REMOTE = 0b0000'1101;
constexpr uint16_t PKT_DATA_SOURCE_PAYLOAD_DRAM = 0b0000'1110;

// Helper to cast a value into a typed enum. Takes care of invalid inputs by
// returning the `kUnknown` value.
template <typename T>
T ToEnum(uint8_t val) {
  if (PERFETTO_LIKELY(val < static_cast<uint8_t>(T::kMax))) {
    return static_cast<T>(val);
  }
  return T::kUnknown;
}

// An SPE record is a collection of packets. An End or Timestamp packet signals
// the end of a record. Each record consists of a 1 or 2 byte header followed by
// 0 - 4 bytes of payload. The `ShortHeader`, and `ExtendedHeader` hide all the
// low level bit fiddling details of handling packets. When parsing a stream of
// SPE records you can just check the first byte in the stream to determine if
// it belongs to a short or extended header and then use the appropriate class
// to determine packet type, payload length and packet details. There are other
// helper classes to parse payloads for the different packets.

// Checks if a header bytes is a padding packet. (no payload)
inline bool IsPadding(uint8_t byte) {
  return byte == SHORT_HEADER_PADDING;
}

// Checks if a header byte corresponds to an extended header.
inline bool IsExtendedHeader(uint8_t byte) {
  return (byte & EXTENDED_HEADER_MASK) == EXTENDED_HEADER;
}

class ShortHeader {
 public:
  explicit ShortHeader(uint8_t byte) : byte_0_(byte) {
    PERFETTO_DCHECK(!IsExtendedHeader(byte));
  }

  inline bool IsPadding() { return byte_0_ == SHORT_HEADER_PADDING; }

  inline bool IsEndPacket() { return byte_0_ == SHORT_HEADER_END_PACKET; }

  inline bool IsTimestampPacket() {
    return byte_0_ == SHORT_HEADER_TIMESTAMP_PACKET;
  }

  bool IsAddressPacket() const {
    return (byte_0_ & COMMON_HEADER_MASK) == COMMON_HEADER_ADDRESS_PACKET;
  }

  AddressIndex GetAddressIndex() const {
    PERFETTO_DCHECK(IsAddressPacket());
    return ToEnum<AddressIndex>(index());
  }

  bool IsCounterPacket() const {
    return (byte_0_ & COMMON_HEADER_MASK) == COMMON_HEADER_COUNTER_PACKET;
  }

  CounterIndex GetCounterIndex() const {
    PERFETTO_DCHECK(IsCounterPacket());
    return ToEnum<CounterIndex>(index());
  }

  bool IsEventsPacket() const {
    return (byte_0_ & SHORT_HEADER_MASK_1) == SHORT_HEADER_EVENTS_PACKET;
  }

  bool IsContextPacket() const {
    return (byte_0_ & SHORT_HEADER_MASK_2) == SHORT_HEADER_CONTEXT_PACKET;
  }

  ContextIndex GetContextIndex() const { return ToEnum<ContextIndex>(index()); }

  bool IsDataSourcePacket() const {
    return (byte_0_ & SHORT_HEADER_MASK_1) == SHORT_HEADER_DATA_SOURCE_PACKET;
  }

  DataSource GetDataSource(uint64_t payload) {
    PERFETTO_DCHECK(IsDataSourcePacket());
    switch (payload) {
      case PKT_DATA_SOURCE_PAYLOAD_L1D:
        return DataSource::kL1D;
      case PKT_DATA_SOURCE_PAYLOAD_L2:
        return DataSource::kL2;
      case PKT_DATA_SOURCE_PAYLOAD_PEER_CORE:
        return DataSource::kPeerCore;
      case PKT_DATA_SOURCE_PAYLOAD_LOCAL_CLUSTER:
        return DataSource::kLocalCluster;
      case PKT_DATA_SOURCE_PAYLOAD_SYS_CACHE:
        return DataSource::kSysCache;
      case PKT_DATA_SOURCE_PAYLOAD_PEER_CLUSTER:
        return DataSource::kPeerCluster;
      case PKT_DATA_SOURCE_PAYLOAD_REMOTE:
        return DataSource::kRemote;
      case PKT_DATA_SOURCE_PAYLOAD_DRAM:
        return DataSource::kDram;
      default:
        break;
    }
    return DataSource::kUnknown;
  }

  bool IsOperationTypePacket() const {
    return (byte_0_ & SHORT_HEADER_MASK_2) ==
           SHORT_HEADER_OPERATION_TYPE_PACKET;
  }

  OperationClass GetOperationClass() const {
    PERFETTO_DCHECK(IsOperationTypePacket());
    switch (byte_0_ & PKT_OP_TYPE_HEADER_CLASS_MASK) {
      case PKT_OP_TYPE_HEADER_CLASS_OTHER:
        return OperationClass::kOther;

      case PKT_OP_TYPE_HEADER_CLASS_LD_ST_ATOMIC:
        return OperationClass::kLoadOrStoreOrAtomic;

      case PKT_OP_TYPE_HEADER_CLASS_BR_ERET:
        return OperationClass::kBranchOrExceptionReturn;

      default:
        break;
    }
    return OperationClass::kUnknown;
  }

  bool HasPayload() const {
    return (byte_0_ & COMMON_HEADER_NO_PAYLOAD_MASK) !=
           COMMON_HEADER_NO_PAYLOAD;
  }

  uint8_t GetPayloadSize() const {
    PERFETTO_DCHECK(!IsExtendedHeader(byte_0_));
    if (!HasPayload()) {
      return 0;
    }
    return static_cast<uint8_t>(1 << ((byte_0_ & COMMON_HEADER_SIZE_MASK) >>
                                      COMMON_HEADER_SIZE_MASK_RSHIFT));
  }

 private:
  friend class ExtendedHeader;

  uint8_t index() const { return byte_0_ & SHORT_HEADER_INDEX_MASK; }

  uint8_t byte_0_;
};

class ExtendedHeader {
 public:
  ExtendedHeader(uint8_t byte_0, uint8_t byte_1)
      : byte_0_(byte_0), short_header_(byte_1) {
    PERFETTO_DCHECK(IsExtendedHeader(byte_0));
  }

  bool IsAddressPacket() const { return short_header_.IsAddressPacket(); }

  AddressIndex GetAddressIndex() const { return ToEnum<AddressIndex>(index()); }

  bool IsCounterPacket() const { return short_header_.IsCounterPacket(); }

  CounterIndex GetCounterIndex() const { return ToEnum<CounterIndex>(index()); }

  inline uint8_t GetPayloadSize() { return short_header_.GetPayloadSize(); }

 private:
  uint8_t byte_1() const { return short_header_.byte_0_; }

  uint8_t index() const {
    return static_cast<uint8_t>((byte_0_ & EXTENDED_HEADER_INDEX_MASK)
                                << EXTENDED_HEADER_INDEX_LSHIFT) +
           short_header_.index();
  }

  uint8_t byte_0_;
  ShortHeader short_header_;
};

enum class OperationOtherSubclass : uint8_t {
  kOther,
  kSveVecOp,
  kUnknown,
  kMax = kUnknown
};
class OperationTypeOtherPayload {
 public:
  explicit OperationTypeOtherPayload(uint8_t payload) : payload_(payload) {}

  OperationOtherSubclass subclass() const {
    if ((payload_ & PKT_OP_TYPE_PAYLOAD_SUBCLASS_OTHER_MASK) ==
        PKT_OP_TYPE_PAYLOAD_SUBCLASS_OTHER) {
      return OperationOtherSubclass::kOther;
    }
    if ((payload_ & PKT_OP_TYPE_PAYLOAD_SUBCLASS_SVE_OTHER_MASK) ==
        PKT_OP_TYPE_PAYLOAD_SUBCLASS_SVE_OTHER) {
      return OperationOtherSubclass::kSveVecOp;
    }
    return OperationOtherSubclass::kUnknown;
  }

 private:
  uint8_t payload_;
};

class OperationTypeLdStAtPayload {
 public:
  explicit OperationTypeLdStAtPayload(uint8_t payload) : payload_(payload) {}

  bool IsStore() const { return IsBitSet<0>(payload_); }

 private:
  uint8_t payload_;
};

namespace internal {
inline uint64_t GetPacketAddressAddress(uint64_t payload) {
  return payload & 0x0FFFFFFFFFFFFFFF;
}

inline bool GetPacketAddressNs(uint64_t payload) {
  return IsBitSet<63>(payload);
}

inline ExceptionLevel GetPacketAddressEl(uint64_t payload) {
  return static_cast<ExceptionLevel>((payload >> 61) & 0x03);
}

inline bool GetPacketAddressNse(uint64_t payload) {
  return IsBitSet<60>(payload);
}
}  // namespace internal

struct InstructionVirtualAddress {
  explicit InstructionVirtualAddress(uint64_t payload)
      : address(internal::GetPacketAddressAddress(payload)),
        el(internal::GetPacketAddressEl(payload)),
        ns(internal::GetPacketAddressNs(payload)),
        nse(internal::GetPacketAddressNse(payload)) {}
  uint64_t address;
  ExceptionLevel el;
  bool ns;
  bool nse;
};

struct DataVirtualAddress {
  explicit DataVirtualAddress(uint64_t payload)
      : address(internal::GetPacketAddressAddress(payload)) {}
  uint64_t address;
};

struct DataPhysicalAddress {
  explicit DataPhysicalAddress(uint64_t payload)
      : address(internal::GetPacketAddressAddress(payload)) {}
  uint64_t address;
};

}  // namespace perfetto::trace_processor::perf_importer::spe

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_H_
