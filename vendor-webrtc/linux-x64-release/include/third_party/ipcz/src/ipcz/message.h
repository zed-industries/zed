// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/393091624): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef IPCZ_SRC_IPCZ_MESSAGE_H_
#define IPCZ_SRC_IPCZ_MESSAGE_H_

#include <cstddef>
#include <cstdint>
#include <optional>

#include "ipcz/driver_object.h"
#include "ipcz/driver_transport.h"
#include "ipcz/ipcz.h"
#include "ipcz/sequence_number.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "third_party/abseil-cpp/absl/container/inlined_vector.h"
#include "third_party/abseil-cpp/absl/types/span.h"
#include "util/safe_math.h"

namespace ipcz {

namespace internal {

// All wire structures defined in this header should be declared between this
// line and the corresponding #pragma pack(pop). Fields in these structures
// should manually aligned with explicit padding fields as needed.
#pragma pack(push, 1)

// Header which begins all messages. The header layout is versioned for
// extensibility and long-term support.
struct MessageHeader {
  // The size of the header in bytes.
  uint8_t size;

  // The header version in use by this message.
  uint8_t version;

  // Message ID assigned as part of a message's type definition via
  // IPCZ_MSG_BEGIN().
  uint8_t message_id;

  // Reserved for future use. Must be zero.
  uint8_t reserved0[5];

  // Used for sequencing messages along a NodeLink to preserve end-to-end
  // ordering, as NodeLink messages may be transmitted either across a driver
  // transport or queues in shared memory.
  SequenceNumber sequence_number;

  // Offset into the message where the unified array of DriverObjectData lives,
  // or zero if there are no driver objects attached.
  uint32_t driver_object_data_array;

  // Reserved for future use. Must be zero.
  uint32_t reserved1;
};
static_assert(sizeof(MessageHeader) == 24, "Unexpected size");

using MessageHeaderV0 = MessageHeader;
using LatestMessageHeaderVersion = MessageHeaderV0;

// Header encoding metadata about a structure within a message.
struct StructHeader {
  // The size of the structure in bytes. Used for versioning.
  uint32_t size;

  // Unused. Must be zero.
  uint32_t padding;
};
static_assert(sizeof(StructHeader) == 8, "Unexpected size");

// Header encoding metadata about any array within a message.
struct ArrayHeader {
  // The total number of bytes occupied by the array, including this header and
  // any padding for 8-byte alignment.
  uint32_t num_bytes;

  // The number of elements encoded for the array. Elements are packed into the
  // message immediately following this header.
  uint32_t num_elements;
};

// Each serialized driver object is represented as an array of bytes and an
// array of driver handles. This structure describes both arrays for a single
// driver object. For every object attached to a message, there one of these
// structs.
struct DriverObjectData {
  // Array index of the byte array which contains serialized data for this
  // driver object. This is specifically the byte index into the enclosing
  // message where the array's ArrayHeader can be found.
  uint32_t driver_data_array;

  // Every message carries a single unified array of attached driver handles.
  // This is index of the first driver handle relevant to a specific driver
  // object attachment.
  uint16_t first_driver_handle;

  // The number of driver handles belonging to this driver object, starting at
  // the index of `first_driver_handle` within the message's driver handle
  // array.
  uint16_t num_driver_handles;
};

// Encodes information about a range of driver objects. Used to encode
// DriverObject array parameters.
struct DriverObjectArrayData {
  // Index into the message's unified DriverObject array which corresponds to
  // the first DriverObject belonging to the array described by this structure.
  uint32_t first_object_index;

  // The length of this DriverObject array. DriverObjects belonging to the array
  // begin at `first_object_index` within the message's unified DriverObject
  // array, and continue for the next `num_objects` contiguous elements.
  uint32_t num_objects;
};

// Encodes an invalid driver object index. Any driver object field encoded as
// this value will deserialize to an invalid DriverObject.
constexpr uint32_t kInvalidDriverObjectIndex = 0xffffffff;

// End of wire structure definitions. Anything below this line is not meant to
// be encoded into messages.
#pragma pack(pop)

// Message macros emit metadata structures which are used at runtime to help
// validate an encoded message during deserialization. This conveys the kind of
// each field within the message's parameter structure.
enum class ParamType {
  // A parameter encoded inline within the message's primary parameter struct.
  kData,

  // A parameter encoded as a 32-bit index elsewhere in the message. This index
  // points to encoded array contents, beginning with an ArrayHeader.
  kDataArray,

  // A parameter encoded as a single 32-bit value indexing the message's unified
  // DriverObject array, referring to a single DriverObject attached to the
  // message.
  kDriverObject,

  // A parameter encoded as a single DriverObjectArrayData structure, referring
  // to a span of contiguous DriverObjects within the message's unified
  // DriverObject array.
  kDriverObjectArray,
};

// Metadata about a single parameter declared within a message via one of the
// IPCZ_MSG_PARAM* macros. Constants of this type are genereated by such macros.
// See documentation in message_versions_declaration_macros.h and
// message_base_declaration_macros.h.
struct ParamMetadata {
  // The offset of this parameter from the start of its version's field block.
  size_t offset;

  // The size of the data expected at `offset` in order for the field to be
  // deserializable.
  size_t size;

  // If this is an array-typed field, this is the encoded size of each array
  // element expected.
  size_t array_element_size;

  // The generic type of this parameter. See ParamType above.
  ParamType type;
};

// Metadata about a single message version. Note that versions are additive:
// "version 0" refers only to the fields defined for version 0 of a message;
// "version 1" refers only to the fields added for version 1. More generally, a
// message with a "version N" block also has a block for all versions from 0 to
// N-1, and all blocks are non-overlapping.
//
// Constants of this type are genereated by IPCZ_MSG_PARAM* macros. See
// documentation in message_versions_declaration_macros.h and
// message_base_declaration_macros.h.
struct VersionMetadata {
  // The version number for the described version.
  int version_number;

  // The offset of this version's parameter data from the start of the
  // parameters structure, including the StructHeader.
  size_t offset;

  // The size of the parameter data which comprises this version.
  size_t size;

  // Metadata about all the parameters in the parameter data.
  const absl::Span<const ParamMetadata> params;
};

}  // namespace internal

// Message helps build, serialize, and deserialize ipcz-internal messages.
class IPCZ_ALIGN(8) Message {
 public:
  enum { kIncoming };

  // ReceivedDataBuffer is a fixed-size, heap-allocated data buffer which is
  // allocated uninitialized and which can be moved out of the Message which
  // allocated it. This is used strictly as storage for received message data.
  struct FreeDeleter {
    void operator()(void* ptr) { free(ptr); }
  };
  using ReceivedDataPtr = std::unique_ptr<uint8_t, FreeDeleter>;
  class ReceivedDataBuffer {
   public:
    ReceivedDataBuffer();
    explicit ReceivedDataBuffer(size_t size);
    ReceivedDataBuffer(ReceivedDataBuffer&&);
    ReceivedDataBuffer& operator=(ReceivedDataBuffer&&);
    ~ReceivedDataBuffer();

    uint8_t* data() const { return data_.get(); }
    size_t size() const { return size_; }
    bool empty() const { return size_ == 0; }

    absl::Span<uint8_t> bytes() const { return absl::MakeSpan(data(), size()); }

   private:
    ReceivedDataPtr data_;
    size_t size_ = 0;
  };

  Message();
  Message(uint8_t message_id, size_t params_size);
  ~Message();

  internal::MessageHeader& header() {
    return *reinterpret_cast<internal::MessageHeader*>(data_.data());
  }

  const internal::MessageHeader& header() const {
    return *reinterpret_cast<const internal::MessageHeader*>(data_.data());
  }

  absl::Span<uint8_t> data_view() { return data_; }

  absl::Span<uint8_t> params_data_view() {
    return absl::MakeSpan(&data_[header().size], data_.size() - header().size);
  }
  absl::Span<DriverObject> driver_objects() {
    return absl::MakeSpan(driver_objects_);
  }
  absl::Span<IpczDriverHandle> transmissible_driver_handles() {
    return absl::MakeSpan(transmissible_driver_handles_);
  }

  // Allocates additional storage in this message to hold an array of
  // `num_elements`, each with a size of `element_size` bytes. The allocated
  // storage includes space for an ArrayHeader and padding to an 8-byte
  // boundary. Returns the offset into the message payload where the ArrayHeader
  // begins. Storage for each element follows contiguously from there.
  uint32_t AllocateGenericArray(size_t element_size, size_t num_elements);

  // Simple template helper for AllocateGenericArray, using the size of the
  // type argument as the element size.
  template <typename ElementType>
  uint32_t AllocateArray(size_t num_elements) {
    return AllocateGenericArray(sizeof(ElementType), num_elements);
  }

  // Allocates an array and populates its elements in-place, returning the array
  // offset to use as a field value.
  template <typename ElementType>
  uint32_t AllocateAndSetArray(absl::Span<const ElementType> elements) {
    const auto offset = AllocateArray<ElementType>(elements.size());
    const auto view = GetArrayView<ElementType>(offset);
    std::copy(elements.begin(), elements.end(), view.begin());
    return offset;
  }

  // Appends a single driver object to this message, and returns its index into
  // the message's DriverObject array. This index should be stored as the value
  // for whatever IPCZ_MSG_PARAM_DRIVER_OBJECT() parameter corresponds to the
  // appended object.
  //
  // Note that this does NOT serialize `object` yet. Serialization of all
  // attached objects occurs during Serialize().
  uint32_t AppendDriverObject(DriverObject object);

  // Appends all driver objects in `objects` to this message and returns a
  // DriverObjectArrayData describing the starting index and length of a span
  // within the message's DriverObject array. This structure should be stored as
  // the value for whatever IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY() parameter
  // corresponds to the attached sequence of objects.
  //
  // Note that this does NOT serialize `object` yet. Serialization of all
  // attached objects occurs during Serialize().
  internal::DriverObjectArrayData AppendDriverObjects(
      absl::Span<DriverObject> objects);

  // Takes ownership of a DriverObject that was attached to this message, given
  // an index into the message's unified DriverObject array. This should be the
  // same index returned by a prior call to AppendDriverObject() when
  // serializing the original message.
  DriverObject TakeDriverObject(uint32_t index);

  // Returns a span of DriverObjects (within `driver_objects_`) corresponding to
  // the span described by `data`. This should be the same structure value
  // returned by a prior call to AppendDriverObjects() when serializing the
  // original message.
  absl::Span<DriverObject> GetDriverObjectArrayView(
      const internal::DriverObjectArrayData& data);

  // Returns the address of the first element of an array whose header begins
  // at `offset` bytes from the beginning of this message.
  void* GetArrayData(size_t offset) {
    // NOTE: Any offset plugged into this method must be validated ahead of
    // time.
    ABSL_ASSERT(CheckAdd(offset, sizeof(internal::ArrayHeader)) <=
                data_.size());
    auto& header = *reinterpret_cast<internal::ArrayHeader*>(&data_[offset]);
    return &header + 1;
  }

  // Template helper which returns a view into a serialized array's contents,
  // given an array whose header begins at `offset` bytes from the beginning of
  // this message. If `offset` is zero, this returns an empty span.
  template <typename ElementType>
  absl::Span<ElementType> GetArrayView(size_t offset) {
    if (!offset) {
      return {};
    }

    // NOTE: Any offset plugged into this method must be validated ahead of
    // time.
    ABSL_ASSERT(CheckAdd(offset, sizeof(internal::ArrayHeader)) <=
                data_.size());
    auto& header = *reinterpret_cast<internal::ArrayHeader*>(&data_[offset]);

    // The ArrayHeader itself must also have been validated already to ensure
    // that the span of array contents will not exceed the bounds of `data_`.
    ABSL_ASSERT(CheckAdd(CheckMul(sizeof(ElementType),
                                  static_cast<size_t>(header.num_elements)),
                         sizeof(internal::ArrayHeader)) <= data_.size());
    return absl::MakeSpan(reinterpret_cast<ElementType*>(&header + 1),
                          header.num_elements);
  }

  // Helper to retrieve a typed value from the message given an absolute byte
  // offset from the start of the message.
  template <typename T>
  T& GetValueAt(size_t data_offset) {
    // NOTE: Any offset plugged into this method must be validated ahead of
    // time.
    ABSL_ASSERT(CheckAdd(data_offset, sizeof(T)) <= data_.size());
    return *reinterpret_cast<T*>(&data_[data_offset]);
  }

  // Helper to retrieve a typed value from the message given a byte offset from
  // the start of the message's parameter data. Note the distinction between
  // this and GetValueAt(), as this offset is taken from the end of the message
  // header, while GetValueAt() (and most other offset-diven methods here)
  // interprets the offset as relative to the beginning of the message itself.
  //
  // This method is used in conjunection with parameter metadata generated by
  // macros at compile-time.
  template <typename T>
  T& GetParamValueAt(size_t param_offset) {
    // NOTE: Any offset plugged into this method must be validated ahead of
    // time.
    ABSL_ASSERT(CheckAdd(param_offset, sizeof(T)) <= params_data_view().size());
    return GetValueAt<T>(GetDataOffset(&params_data_view()[param_offset]));
  }

  // Checks and indicates whether this message can be transmitted over
  // `transport`, which depends on whether the driver is able to transmit all of
  // the attached driver objects over that transport.
  bool CanTransmitOn(const DriverTransport& transport);

  // Attempts to finalize a message for transmission over `transport`. If the
  // message has no DriverObjects attached, this is trivially a no-op.
  //
  // Otherwise, any attached DriverObjects are serialized into inlined data
  // and/or transmissible handles at this time. This may result in re-allocation
  // of the Message's underlying storage.
  //
  // In any case, upon return the resulting Message can be transmitted to
  // another node and be deserialized from there. If the returned Message has at
  // least one element in transmissible_driver_handles(), the Message must be
  // transmitted over `transport`. Otherwise it can be transmitted by any other
  // suitable mechanism for data transmission, such as shared memory.
  //
  // NOTE: It is invalid to call this on a message for which
  // `CanTransmitOn(transport)` does not return true, and doing so results in
  // unspecified behavior.
  bool Serialize(const DriverTransport& transport);

  // Validates and deserializes a Message of an unrecognized type. DriverObjects
  // are deserialized and much of the message structure is validated, but the
  // parameter layout is not known and is therefore neither validated nor
  // exposed.
  bool DeserializeUnknownType(const DriverTransport::RawMessage& message,
                              const DriverTransport& transport);

  // For a Message whose contents were received from another node, this takes
  // ownership of the heap-allocated copy of those contents. Invalidates this
  // Message.
  ReceivedDataBuffer TakeReceivedData() &&;

  void SetEnvelope(DriverObject envelope);

  DriverObject TakeEnvelope();

 protected:
  // Returns `x` aligned above to the nearest 8-byte boundary.
  constexpr size_t Align(size_t x) { return (x + 7) & ~7; }

  // Returns the relative offset of an address which falls within the message's
  // data payload.
  uint32_t GetDataOffset(const void* data) {
    return static_cast<uint32_t>(static_cast<const uint8_t*>(data) -
                                 data_.data());
  }

  // Common helper for vaidation of an incoming message header and basic data
  // payload size.
  bool CopyDataAndValidateHeader(absl::Span<const uint8_t> data);

  // Common helper to validate an encoded parameter structure against a specific
  // message definition. Must only be called on a Message with `data_` already
  // populated, the header already validated, and DriverObjects already
  // deserialized into `driver_objects_`.
  bool ValidateParameters(size_t params_size,
                          absl::Span<const internal::VersionMetadata> versions);

  // Attempts to deserialize a message from raw `data` and `handles` into `this`
  // message object, given the `params_size`, `params_current_version` and
  // `params_metadata`, which are all generated from message macros at build
  // time to describe a specific ipcz-internal message.
  //
  // `transport` is the transport from which the incoming data and handles were
  // received.
  bool DeserializeFromTransport(
      size_t params_size,
      absl::Span<const internal::VersionMetadata> versions,
      const DriverTransport::RawMessage& message,
      const DriverTransport& transport);

  // Attempts to deserialize a message from raw `data`, given a set of already
  // deserialized DriverObjects in `objects`. The objects and data here have
  // been extracted from a message relayed opaquely through the broker. While
  // each DriverObject has already been validated and deserialized, the
  // message-specific parameter data and object-field assignments must be
  // validated here.
  bool DeserializeFromRelay(
      size_t params_size,
      absl::Span<const internal::VersionMetadata> versions,
      absl::Span<const uint8_t> data,
      absl::Span<DriverObject> objects);

  // Inlined storage for this message's data. Used when constructing outgoing
  // messages, since most are small and can avoid additional heap allocation
  // before hitting the wire.
  std::optional<absl::InlinedVector<uint8_t, 128>> inlined_data_;

  // Heap storage for this message's data, as received from a transport.
  std::optional<ReceivedDataBuffer> received_data_;

  // A view over *either* `received_data_` *or* `inlined_data_`, or empty if
  // neither is present.
  //
  // This is the raw serialized data for this message. It always begins with a
  // MessageHeader (or potentially some newer or older version thereof), whose
  // actual size is determined by the header's `size` field. After that many
  // bytes, a parameters structure immediately follows, as generated by an
  // invocation of IPCZ_MSG_BEGIN()/IPCZ_MSG_END(). After fixed parameters, any
  // number of dynamicaly inlined allocations may follow (e.g. for array
  // contents, driver objects, etc.)
  absl::Span<uint8_t> data_;

  // Collection of DriverObjects attached to this message. These are attached
  // while building a message (e.g. by calling AppendDriverObject), and they are
  // consumed by Serialize() to encode the objects for transmission. Serialized
  // objects are represented by some combination of data within the message,
  // and zero or more transmissible driver handles which accumulate in
  // `transmissible_driver_handles_` during serialization.
  //
  // On deserialization, driver object data and transmissible handles are fed
  // back to the driver and used to reconstruct this list. Deserialized objects
  // may be extracted from the message by calling TakeDriverObject().
  //
  // Since each driver object may serialize to any number of bytes and
  // transmissible handles, there is generally NOT a 1:1 correpsondence between
  // this list and `transmissible_driver_handles_`.
  absl::InlinedVector<DriverObject, 2> driver_objects_;

  // Collection of driver handles which the driver knows how to transmit as-is,
  // in conjunction with (but out-of-band from) the payload in `data_`. This
  // is populated by Serialize() if the driver emits any transmissible handles
  // as outputs when serializing any of the objects in `driver_objects_`.
  //
  // On deserialization this set of handles is consumed; and in combination with
  // encoded object data, is used by the driver to reconstruct
  // `driver_objects_`.
  //
  // Since each driver object may serialize to any number of bytes and
  // transmissible handles, there is generally NOT a 1:1 correpsondence between
  // this list and `driver_objects_`.
  absl::InlinedVector<IpczDriverHandle, 2> transmissible_driver_handles_;

  DriverObject envelope_;
};

// Template helper to wrap the Message type for a specific macro-generated
// parameter structure. This primarily exists for safe, convenient construction
// of message payloads with correct header information and no leaky padding
// bits, as well as for convenient access to parameters within size-validated,
// deserialized messages.
//
// When an IPCZ_MSG_BEGIN() macro is used to declare a new Foo message, it will
// emit both a msg::Foo_Params structure for the fixed wire data of the message
// parameters, as well as a msg::Foo which is an alias for an instance of this
// template, namely Message<msg::Foo_Params>.
template <typename ParamDataType>
class MessageWithParams : public Message {
 public:
  MessageWithParams() : Message(ParamDataType::kId, sizeof(ParamDataType)) {
    ParamDataType& p = *(new (&params()) ParamDataType());
    p.header.size = sizeof(p);
    p.header.padding = 0;
  }

  // Special constructor which avoids initializing storage that won't be used
  // anyway.
  explicit MessageWithParams(decltype(Message::kIncoming)) : Message() {}

  ~MessageWithParams() = default;

  // Convenient accessors for the message's main parameters struct, whose
  // location depends on the size of the header. Note that because this may be
  // used to access parameters within messages using a newer or older header
  // than what's defined above in MessageHeader, we index based on the header's
  // encoded size rather than the compile-time size of MessageHeader.
  //
  // If this object was deserialized from the wire, it must already have been
  // validated to have an enough space for `header().size` bytes plus the size
  // of ParamDataType or some older version thereof. Safe access to newer
  // versions' fields is managed by the ParamDataType itself.
  ParamDataType& params() {
    return *reinterpret_cast<ParamDataType*>(&data_[header().size]);
  }

  const ParamDataType& params() const {
    return *reinterpret_cast<const ParamDataType*>(&data_[header().size]);
  }
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_MESSAGE_H_
