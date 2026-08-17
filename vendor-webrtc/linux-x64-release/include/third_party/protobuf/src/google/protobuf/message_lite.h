// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Authors: wink@google.com (Wink Saville),
//          kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// Defines MessageLite, the abstract interface implemented by all (lite
// and non-lite) protocol message objects.

#ifndef GOOGLE_PROTOBUF_MESSAGE_LITE_H__
#define GOOGLE_PROTOBUF_MESSAGE_LITE_H__

#include <climits>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iosfwd>
#include <memory>
#include <new>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/base/casts.h"
#include "absl/log/absl_check.h"
#include "absl/numeric/bits.h"
#include "absl/strings/cord.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/explicitly_constructed.h"
#include "google/protobuf/internal_visibility.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/metadata_lite.h"
#include "google/protobuf/port.h"


// clang-format off
#include "google/protobuf/port_def.inc"
// clang-format on

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {

template <typename T>
class RepeatedPtrField;

class FastReflectionMessageMutator;
class FastReflectionStringSetter;
class Reflection;
class Descriptor;
class AssignDescriptorsHelper;
class MessageLite;

namespace io {

class CodedInputStream;
class CodedOutputStream;
class ZeroCopyInputStream;
class ZeroCopyOutputStream;

}  // namespace io

namespace compiler {
namespace cpp {
class MessageTableTester;
}  // namespace cpp
}  // namespace compiler

namespace internal {

namespace v2 {
class TableDriven;
}  // namespace v2

class MessageCreator {
 public:
  using Func = void* (*)(const void*, void*, Arena*);

  // Use -1/0/1 to be able to use <0, ==0, >0
  enum Tag : int8_t {
    kFunc = -1,
    kZeroInit = 0,
    kMemcpy = 1,
  };

  constexpr MessageCreator()
      : allocation_size_(), tag_(), alignment_(), arena_bits_(uintptr_t{}) {}

  static constexpr MessageCreator ZeroInit(uint32_t allocation_size,
                                           uint8_t alignment,
                                           uintptr_t arena_bits = 0) {
    MessageCreator out;
    out.allocation_size_ = allocation_size;
    out.tag_ = kZeroInit;
    out.alignment_ = alignment;
    out.arena_bits_ = arena_bits;
    return out;
  }
  static constexpr MessageCreator CopyInit(uint32_t allocation_size,
                                           uint8_t alignment,
                                           uintptr_t arena_bits = 0) {
    MessageCreator out;
    out.allocation_size_ = allocation_size;
    out.tag_ = kMemcpy;
    out.alignment_ = alignment;
    out.arena_bits_ = arena_bits;
    return out;
  }
  constexpr MessageCreator(Func func, uint32_t allocation_size,
                           uint8_t alignment)
      : allocation_size_(allocation_size),
        tag_(kFunc),
        alignment_(alignment),
        func_(func) {}

  // Template for testing.
  template <bool test_call = false, typename MessageLite>
  MessageLite* New(const MessageLite* prototype_for_func,
                   const MessageLite* prototype_for_copy, Arena* arena) const;

  template <bool test_call = false, typename MessageLite>
  MessageLite* PlacementNew(const MessageLite* prototype_for_func,
                            const MessageLite* prototype_for_copy, void* mem,
                            Arena* arena) const;

  Tag tag() const { return tag_; }

  uint32_t allocation_size() const { return allocation_size_; }

  uint8_t alignment() const { return alignment_; }

  uintptr_t arena_bits() const {
    ABSL_DCHECK_NE(+tag(), +kFunc);
    return arena_bits_;
  }

 private:
  uint32_t allocation_size_;
  Tag tag_;
  uint8_t alignment_;
  union {
    Func func_;
    uintptr_t arena_bits_;
  };
};

// Allow easy change to regular int on platforms where the atomic might have a
// perf impact.
//
// CachedSize is like std::atomic<int> but with some important changes:
//
// 1) CachedSize uses Get / Set rather than load / store.
// 2) CachedSize always uses relaxed ordering.
// 3) CachedSize is assignable and copy-constructible.
// 4) CachedSize has a constexpr default constructor, and a constexpr
//    constructor that takes an int argument.
// 5) If the compiler supports the __atomic_load_n / __atomic_store_n builtins,
//    then CachedSize is trivially copyable.
//
// Developed at https://godbolt.org/z/vYcx7zYs1 ; supports gcc, clang, MSVC.
class PROTOBUF_EXPORT CachedSize {
 private:
  using Scalar = int;

 public:
  constexpr CachedSize() noexcept : atom_(Scalar{}) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr CachedSize(Scalar desired) noexcept : atom_(desired) {}

#ifdef PROTOBUF_BUILTIN_ATOMIC
  constexpr CachedSize(const CachedSize& other) = default;

  Scalar Get() const noexcept {
    return __atomic_load_n(&atom_, __ATOMIC_RELAXED);
  }

  void Set(Scalar desired) const noexcept {
    // Avoid writing the value when it is zero. This prevents writing to global
    // default instances, which might be in readonly memory.
    if (ABSL_PREDICT_FALSE(desired == 0)) {
      if (Get() == 0) return;
    }
    __atomic_store_n(&atom_, desired, __ATOMIC_RELAXED);
  }

  void SetNonZero(Scalar desired) const noexcept {
    ABSL_DCHECK_NE(desired, 0);
    __atomic_store_n(&atom_, desired, __ATOMIC_RELAXED);
  }

  void SetNoDefaultInstance(Scalar desired) const noexcept {
    __atomic_store_n(&atom_, desired, __ATOMIC_RELAXED);
  }
#else
  CachedSize(const CachedSize& other) noexcept : atom_(other.Get()) {}
  CachedSize& operator=(const CachedSize& other) noexcept {
    Set(other.Get());
    return *this;
  }

  Scalar Get() const noexcept {  //
    return atom_.load(std::memory_order_relaxed);
  }

  void Set(Scalar desired) const noexcept {
    // Avoid writing the value when it is zero. This prevents writing to global
    // default instances, which might be in readonly memory.
    if (ABSL_PREDICT_FALSE(desired == 0)) {
      if (Get() == 0) return;
    }
    atom_.store(desired, std::memory_order_relaxed);
  }

  void SetNonZero(Scalar desired) const noexcept {
    ABSL_DCHECK_NE(desired, 0);
    atom_.store(desired, std::memory_order_relaxed);
  }

  void SetNoDefaultInstance(Scalar desired) const noexcept {
    atom_.store(desired, std::memory_order_relaxed);
  }
#endif

 private:
#ifdef PROTOBUF_BUILTIN_ATOMIC
  mutable Scalar atom_;
#else
  mutable std::atomic<Scalar> atom_;
#endif
};

auto GetClassData(const MessageLite& msg);

template <const auto* kDefault, const auto* kClassData>
struct GeneratedMessageTraitsT {
  static constexpr const void* default_instance() { return kDefault; }
  static constexpr const auto* class_data() { return kClassData->base(); }
  static constexpr auto StrongPointer() { return default_instance(); }
};

template <typename T>
struct FallbackMessageTraits {
  static const void* default_instance() { return &T::default_instance(); }
  static constexpr const auto* class_data() {
    return GetClassData(T::default_instance());
  }
  // We can't make a constexpr pointer to the default, so use a function pointer
  // instead.
  static constexpr auto StrongPointer() { return &T::default_instance; }
};

template <const uint32_t* kValidationData>
struct EnumTraitsT {
  static constexpr const uint32_t* validation_data() { return kValidationData; }
};

// Traits for messages and enums.
// We use a class scope variable template, which can be specialized with a
// different type in a non-defining declaration.
// We need non-defining declarations because we might have duplicates of the
// same trait specification on each dependent coming from different .proto.h
// files.
struct MessageTraitsImpl {
  template <typename T>
  static FallbackMessageTraits<T> value;
};
template <typename T>
using MessageTraits = decltype(MessageTraitsImpl::value<T>);

struct EnumTraitsImpl {
  struct Undefined;
  template <typename T>
  static Undefined value;
};
template <typename T>
using EnumTraits = decltype(EnumTraitsImpl::value<T>);

class SwapFieldHelper;

// See parse_context.h for explanation
class ParseContext;

struct DescriptorTable;
class DescriptorPoolExtensionFinder;
class ExtensionSet;
class LazyField;
class RepeatedPtrFieldBase;
class TcParser;
struct TcParseTableBase;
class WireFormatLite;
class WeakFieldMap;
class RustMapHelper;

// We compute sizes as size_t but cache them as int.  This function converts a
// computed size to a cached size.  Since we don't proceed with serialization
// if the total size was > INT_MAX, it is not important what this function
// returns for inputs > INT_MAX.  However this case should not error or
// ABSL_CHECK-fail, because the full size_t resolution is still returned from
// ByteSizeLong() and checked against INT_MAX; we can catch the overflow
// there.
inline int ToCachedSize(size_t size) { return static_cast<int>(size); }

// We mainly calculate sizes in terms of size_t, but some functions that
// compute sizes return "int".  These int sizes are expected to always be
// positive. This function is more efficient than casting an int to size_t
// directly on 64-bit platforms because it avoids making the compiler emit a
// sign extending instruction, which we don't want and don't want to pay for.
inline size_t FromIntSize(int size) {
  // Convert to unsigned before widening so sign extension is not necessary.
  return static_cast<unsigned int>(size);
}

// For cases where a legacy function returns an integer size.  We ABSL_DCHECK()
// that the conversion will fit within an integer; if this is false then we
// are losing information.
inline int ToIntSize(size_t size) {
  ABSL_DCHECK_LE(size, static_cast<size_t>(INT_MAX));
  return static_cast<int>(size);
}

#if defined(PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE)
using GetTypeNameReturnType = absl::string_view;
#else
using GetTypeNameReturnType = std::string;
#endif


PROTOBUF_EXPORT inline const std::string& GetEmptyStringAlreadyInited() {
  return fixed_address_empty_string.get();
}

PROTOBUF_EXPORT size_t StringSpaceUsedExcludingSelfLong(const std::string& str);

struct ClassDataFull;

// Note: The order of arguments in the functions is chosen so that it has
// the same ABI as the member function that calls them. Eg the `this`
// pointer becomes the first argument in the free function.
//
// Future work:
// We could save more data by omitting any optional pointer that would
// otherwise be null. We can have some metadata in ClassData telling us if we
// have them and their offset.

struct PROTOBUF_EXPORT ClassData {
  const MessageLite* prototype;
  const internal::TcParseTableBase* tc_table;
  void (*on_demand_register_arena_dtor)(MessageLite& msg, Arena& arena);
  bool (*is_initialized)(const MessageLite&);
  void (*merge_to_from)(MessageLite& to, const MessageLite& from_msg);
  internal::MessageCreator message_creator;
#if defined(PROTOBUF_CUSTOM_VTABLE)
  void (*destroy_message)(MessageLite& msg);
  void (MessageLite::*clear)();
  size_t (*byte_size_long)(const MessageLite&);
  uint8_t* (*serialize)(const MessageLite& msg, uint8_t* ptr,
                        io::EpsCopyOutputStream* stream);
#endif  // PROTOBUF_CUSTOM_VTABLE

  // Offset of the CachedSize member.
  uint32_t cached_size_offset;
  // LITE objects (ie !descriptor_methods) collocate their name as a
  // char[] just beyond the ClassData.
  bool is_lite;
  bool is_dynamic = false;

  // In normal mode we have the small constructor to avoid the cost in
  // codegen.
#if !defined(PROTOBUF_CUSTOM_VTABLE)
  constexpr ClassData(
      const MessageLite* prototype, const internal::TcParseTableBase* tc_table,
      void (*on_demand_register_arena_dtor)(MessageLite&, Arena&),
      bool (*is_initialized)(const MessageLite&),
      void (*merge_to_from)(MessageLite& to, const MessageLite& from_msg),
      internal::MessageCreator message_creator, uint32_t cached_size_offset,
      bool is_lite
      )
      : prototype(prototype),
        tc_table(tc_table),
        on_demand_register_arena_dtor(on_demand_register_arena_dtor),
        is_initialized(is_initialized),
        merge_to_from(merge_to_from),
        message_creator(message_creator),
        cached_size_offset(cached_size_offset),
        is_lite(is_lite)
  {
  }
#endif  // !PROTOBUF_CUSTOM_VTABLE

  // But we always provide the full constructor even in normal mode to make
  // helper code simpler.
  constexpr ClassData(
      const MessageLite* prototype, const internal::TcParseTableBase* tc_table,
      void (*on_demand_register_arena_dtor)(MessageLite&, Arena&),
      bool (*is_initialized)(const MessageLite&),
      void (*merge_to_from)(MessageLite& to, const MessageLite& from_msg),
      internal::MessageCreator message_creator,
      void (*destroy_message)(MessageLite& msg),  //
#if defined(PROTOBUF_CUSTOM_VTABLE)
      void (MessageLite::*clear)(),
#endif  // PROTOBUF_CUSTOM_VTABLE
      size_t (*byte_size_long)(const MessageLite&),
      uint8_t* (*serialize)(const MessageLite& msg, uint8_t* ptr,
                            io::EpsCopyOutputStream* stream),
      uint32_t cached_size_offset, bool is_lite
      )
      : prototype(prototype),
        tc_table(tc_table),
        on_demand_register_arena_dtor(on_demand_register_arena_dtor),
        is_initialized(is_initialized),
        merge_to_from(merge_to_from),
        message_creator(message_creator),
#if defined(PROTOBUF_CUSTOM_VTABLE)
        destroy_message(destroy_message),
        clear(clear),
        byte_size_long(byte_size_long),
        serialize(serialize),
#endif  // PROTOBUF_CUSTOM_VTABLE
        cached_size_offset(cached_size_offset),
        is_lite(is_lite)
  {
  }

  const ClassDataFull& full() const;

  MessageLite* New(Arena* arena) const {
    return message_creator.New(prototype, prototype, arena);
  }

  MessageLite* PlacementNew(void* mem, Arena* arena) const {
    return message_creator.PlacementNew(prototype, prototype, mem, arena);
  }

  uint32_t allocation_size() const { return message_creator.allocation_size(); }

  uint8_t alignment() const { return message_creator.alignment(); }
};

template <size_t N>
struct ClassDataLite {
  ClassData header;
  const char type_name[N];

  constexpr const ClassData* base() const { return &header; }
};

// We use a secondary vtable for descriptor based methods. This way ClassData
// does not grow with the number of descriptor methods. This avoids extra
// costs in MessageLite.
struct PROTOBUF_EXPORT DescriptorMethods {
  absl::string_view (*get_type_name)(const ClassData* data);
  std::string (*initialization_error_string)(const MessageLite&);
  const internal::TcParseTableBase* (*get_tc_table)(const MessageLite&);
  size_t (*space_used_long)(const MessageLite&);
  std::string (*debug_string)(const MessageLite&);
};

struct PROTOBUF_EXPORT ClassDataFull : ClassData {
  constexpr ClassDataFull(ClassData base,
                          const DescriptorMethods* descriptor_methods,
                          const internal::DescriptorTable* descriptor_table,
                          void (*get_metadata_tracker)())
      : ClassData(base),
        descriptor_methods(descriptor_methods),
        descriptor_table(descriptor_table),
        reflection(),
        descriptor(),
        get_metadata_tracker(get_metadata_tracker) {}

  constexpr const ClassData* base() const { return this; }

  const DescriptorMethods* descriptor_methods;

  // Codegen types will provide a DescriptorTable to do lazy
  // registration/initialization of the reflection objects.
  // Other types, like DynamicMessage, keep the table as null but eagerly
  // populate `reflection`/`descriptor` fields.
  const internal::DescriptorTable* descriptor_table;
  // Accesses are protected by the once_flag in `descriptor_table`. When the
  // table is null these are populated from the beginning and need to
  // protection.
  mutable const Reflection* reflection;
  mutable const Descriptor* descriptor;

  // When an access tracker is installed, this function notifies the tracker
  // that GetMetadata was called.
  void (*get_metadata_tracker)();
};

inline const ClassDataFull& ClassData::full() const {
  ABSL_DCHECK(!is_lite);
  return *static_cast<const ClassDataFull*>(this);
}

}  // namespace internal

// Interface to light weight protocol messages.
//
// This interface is implemented by all protocol message objects.  Non-lite
// messages additionally implement the Message interface, which is a
// subclass of MessageLite.  Use MessageLite instead when you only need
// the subset of features which it supports -- namely, nothing that uses
// descriptors or reflection.  You can instruct the protocol compiler
// to generate classes which implement only MessageLite, not the full
// Message interface, by adding the following line to the .proto file:
//
//   option optimize_for = LITE_RUNTIME;
//
// This is particularly useful on resource-constrained systems where
// the full protocol buffers runtime library is too big.
//
// Note that on non-constrained systems (e.g. servers) when you need
// to link in lots of protocol definitions, a better way to reduce
// total code footprint is to use optimize_for = CODE_SIZE.  This
// will make the generated code smaller while still supporting all the
// same features (at the expense of speed).  optimize_for = LITE_RUNTIME
// is best when you only have a small number of message types linked
// into your binary, in which case the size of the protocol buffers
// runtime itself is the biggest problem.
//
// Users must not derive from this class. Only the protocol compiler and
// the internal library are allowed to create subclasses.
class PROTOBUF_EXPORT MessageLite {
 public:
  MessageLite(const MessageLite&) = delete;
  MessageLite& operator=(const MessageLite&) = delete;
  PROTOBUF_VIRTUAL ~MessageLite() = default;

  // Basic Operations ------------------------------------------------

  // Get the name of this message type, e.g. "foo.bar.BazProto".
  internal::GetTypeNameReturnType GetTypeName() const;

  // Construct a new instance of the same type.  Ownership is passed to the
  // caller.
  MessageLite* New() const { return New(nullptr); }

  // Construct a new instance on the arena. Ownership is passed to the caller
  // if arena is a nullptr.
  MessageLite* New(Arena* arena) const;

  // Returns the arena, if any, that directly owns this message and its internal
  // memory (Arena::Own is different in that the arena doesn't directly own the
  // internal memory). This method is used in proto's implementation for
  // swapping, moving and setting allocated, for deciding whether the ownership
  // of this message or its internal memory could be changed.
  Arena* GetArena() const { return _internal_metadata_.arena(); }

  // Clear all fields of the message and set them to their default values.
  // Clear() assumes that any memory allocated to hold parts of the message
  // will likely be needed again, so the memory used may not be freed.
  // To ensure that all memory used by a Message is freed, you must delete it.
#if defined(PROTOBUF_CUSTOM_VTABLE)
  void Clear() { (this->*_class_data_->clear)(); }
#else
  virtual void Clear() = 0;
#endif  // PROTOBUF_CUSTOM_VTABLE

  // Quickly check if all required fields have values set.
  bool IsInitialized() const;

  // This is not implemented for Lite messages -- it just returns "(cannot
  // determine missing fields for lite message)".  However, it is implemented
  // for full messages.  See message.h.
  std::string InitializationErrorString() const;

  // If |other| is the exact same class as this, calls MergeFrom(). Otherwise,
  // results are undefined (probably crash).
  void CheckTypeAndMergeFrom(const MessageLite& other);

  // These methods return a human-readable summary of the message. Note that
  // since the MessageLite interface does not support reflection, there is very
  // little information that these methods can provide. They are shadowed by
  // methods of the same name on the Message interface which provide much more
  // information. The methods here are intended primarily to facilitate code
  // reuse for logic that needs to interoperate with both full and lite protos.
  //
  // The format of the returned string is subject to change, so please do not
  // assume it will remain stable over time.
  std::string DebugString() const;
  std::string ShortDebugString() const { return DebugString(); }
  // MessageLite::DebugString is already Utf8 Safe. This is to add compatibility
  // with Message.
  std::string Utf8DebugString() const { return DebugString(); }

  // Implementation of the `AbslStringify` interface. This adds `DebugString()`
  // to the sink. Do not rely on exact format.
  template <typename Sink>
  friend void AbslStringify(Sink& sink, const google::protobuf::MessageLite& msg) {
    sink.Append(msg.DebugString());
  }

  // Parsing ---------------------------------------------------------
  // Methods for parsing in protocol buffer format.  Most of these are
  // just simple wrappers around MergeFromCodedStream().  Clear() will be
  // called before merging the input.

  // Fill the message with a protocol buffer parsed from the given input
  // stream. Returns false on a read error or if the input is in the wrong
  // format.  A successful return does not indicate the entire input is
  // consumed, ensure you call ConsumedEntireMessage() to check that if
  // applicable.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromCodedStream(
      io::CodedInputStream* input);
  // Like ParseFromCodedStream(), but accepts messages that are missing
  // required fields.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParsePartialFromCodedStream(
      io::CodedInputStream* input);
  // Read a protocol buffer from the given zero-copy input stream.  If
  // successful, the entire input will be consumed.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromZeroCopyStream(
      io::ZeroCopyInputStream* input);
  // Like ParseFromZeroCopyStream(), but accepts messages that are missing
  // required fields.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParsePartialFromZeroCopyStream(
      io::ZeroCopyInputStream* input);
  // Parse a protocol buffer from a file descriptor.  If successful, the entire
  // input will be consumed.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromFileDescriptor(
      int file_descriptor);
  // Like ParseFromFileDescriptor(), but accepts messages that are missing
  // required fields.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParsePartialFromFileDescriptor(
      int file_descriptor);
  // Parse a protocol buffer from a C++ istream.  If successful, the entire
  // input will be consumed.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromIstream(std::istream* input);
  // Like ParseFromIstream(), but accepts messages that are missing
  // required fields.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParsePartialFromIstream(
      std::istream* input);
  // Read a protocol buffer from the given zero-copy input stream, expecting
  // the message to be exactly "size" bytes long.  If successful, exactly
  // this many bytes will have been consumed from the input.
  bool MergePartialFromBoundedZeroCopyStream(io::ZeroCopyInputStream* input,
                                             int size);
  // Like ParseFromBoundedZeroCopyStream(), but accepts messages that are
  // missing required fields.
  bool MergeFromBoundedZeroCopyStream(io::ZeroCopyInputStream* input, int size);
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromBoundedZeroCopyStream(
      io::ZeroCopyInputStream* input, int size);
  // Like ParseFromBoundedZeroCopyStream(), but accepts messages that are
  // missing required fields.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParsePartialFromBoundedZeroCopyStream(
      io::ZeroCopyInputStream* input, int size);
  // Parses a protocol buffer contained in a string or Cord. Returns true on
  // success. This function takes a string in the (non-human-readable) binary
  // wire format, matching the encoding output by
  // MessageLite::SerializeToString(). If you'd like to convert a human-readable
  // string into a protocol buffer object, see
  // google::protobuf::TextFormat::ParseFromString().
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromString(absl::string_view data);
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromString(const absl::Cord& data);
  // Like ParseFromString(), but accepts messages that are missing
  // required fields.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParsePartialFromString(
      absl::string_view data);
  ABSL_ATTRIBUTE_REINITIALIZES bool ParsePartialFromString(
      const absl::Cord& data);
  // Parse a protocol buffer contained in an array of bytes.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromArray(const void* data, int size);
  // Like ParseFromArray(), but accepts messages that are missing
  // required fields.
  ABSL_ATTRIBUTE_REINITIALIZES bool ParsePartialFromArray(const void* data,
                                                          int size);


  // Reads a protocol buffer from the stream and merges it into this
  // Message.  Singular fields read from the what is
  // already in the Message and repeated fields are appended to those
  // already present.
  //
  // It is the responsibility of the caller to call input->LastTagWas()
  // (for groups) or input->ConsumedEntireMessage() (for non-groups) after
  // this returns to verify that the message's end was delimited correctly.
  //
  // ParseFromCodedStream() is implemented as Clear() followed by
  // MergeFromCodedStream().
  bool MergeFromCodedStream(io::CodedInputStream* input);

  // Like MergeFromCodedStream(), but succeeds even if required fields are
  // missing in the input.
  //
  // MergeFromCodedStream() is just implemented as MergePartialFromCodedStream()
  // followed by IsInitialized().
  bool MergePartialFromCodedStream(io::CodedInputStream* input);

  // Merge a protocol buffer contained in a string.
  bool MergeFromString(absl::string_view data);
  bool MergeFromString(const absl::Cord& data);

  // Like MergeFromString(), but accepts messages that are missing required
  // fields.
  bool MergePartialFromString(absl::string_view data);
  bool MergePartialFromString(const absl::Cord& data);


  // Serialization ---------------------------------------------------
  // Methods for serializing in protocol buffer format.  Most of these
  // are just simple wrappers around ByteSize() and SerializeWithCachedSizes().

  // Write a protocol buffer of this message to the given output.  Returns
  // false on a write error.  If the message is missing required fields,
  // this may ABSL_CHECK-fail.
  bool SerializeToCodedStream(io::CodedOutputStream* output) const;
  // Like SerializeToCodedStream(), but allows missing required fields.
  bool SerializePartialToCodedStream(io::CodedOutputStream* output) const;
  // Write the message to the given zero-copy output stream.  All required
  // fields must be set.
  bool SerializeToZeroCopyStream(io::ZeroCopyOutputStream* output) const;
  // Like SerializeToZeroCopyStream(), but allows missing required fields.
  bool SerializePartialToZeroCopyStream(io::ZeroCopyOutputStream* output) const;
  // Serialize the message and store it in the given string.  All required
  // fields must be set.
  bool SerializeToString(std::string* output) const;
  // Serialize the message and store it in the given Cord.  All required
  // fields must be set.
  bool SerializeToString(absl::Cord* output) const;
  // Like SerializeToString(), but allows missing required fields.
  bool SerializePartialToString(std::string* output) const;
  bool SerializePartialToString(absl::Cord* output) const;
  // Serialize the message and store it in the given byte array.  All required
  // fields must be set.
  bool SerializeToArray(void* data, int size) const;
  // Like SerializeToArray(), but allows missing required fields.
  bool SerializePartialToArray(void* data, int size) const;

  // Make a string encoding the message. Is equivalent to calling
  // SerializeToString() on a string and using that.  Returns the empty
  // string if SerializeToString() would have returned an error.
  // Note: If you intend to generate many such strings, you may
  // reduce heap fragmentation by instead re-using the same string
  // object with calls to SerializeToString().
  std::string SerializeAsString() const;
  // Like SerializeAsString(), but allows missing required fields.
  std::string SerializePartialAsString() const;

  // Serialize the message and write it to the given file descriptor.  All
  // required fields must be set.
  bool SerializeToFileDescriptor(int file_descriptor) const;
  // Like SerializeToFileDescriptor(), but allows missing required fields.
  bool SerializePartialToFileDescriptor(int file_descriptor) const;
  // Serialize the message and write it to the given C++ ostream.  All
  // required fields must be set.
  bool SerializeToOstream(std::ostream* output) const;
  // Like SerializeToOstream(), but allows missing required fields.
  bool SerializePartialToOstream(std::ostream* output) const;

  // Like SerializeToString(), but appends to the data to the string's
  // existing contents.  All required fields must be set.
  bool AppendToString(std::string* output) const;
  bool AppendToString(absl::Cord* output) const;
  // Like AppendToString(), but allows missing required fields.
  bool AppendPartialToString(std::string* output) const;
  bool AppendPartialToString(absl::Cord* output) const;

  // Reads a protocol buffer from a Cord and merges it into this message.
  PROTOBUF_DEPRECATE_AND_INLINE() bool MergeFromCord(const absl::Cord& data) {
    return MergeFromString(data);
  }
  // Like MergeFromCord(), but accepts messages that are missing
  // required fields.
  PROTOBUF_DEPRECATE_AND_INLINE()
  bool MergePartialFromCord(const absl::Cord& data) {
    return MergePartialFromString(data);
  }
  // Parse a protocol buffer contained in a Cord.
  PROTOBUF_DEPRECATE_AND_INLINE()
  ABSL_ATTRIBUTE_REINITIALIZES bool ParseFromCord(const absl::Cord& data) {
    return ParseFromString(data);
  }
  // Like ParseFromCord(), but accepts messages that are missing
  // required fields.
  PROTOBUF_DEPRECATE_AND_INLINE()
  ABSL_ATTRIBUTE_REINITIALIZES
  bool ParsePartialFromCord(const absl::Cord& data) {
    return ParsePartialFromString(data);
  }

  // Serialize the message and store it in the given Cord.  All required
  // fields must be set.
  PROTOBUF_DEPRECATE_AND_INLINE()
  bool SerializeToCord(absl::Cord* output) const {
    return SerializeToString(output);
  }
  // Like SerializeToCord(), but allows missing required fields.
  PROTOBUF_DEPRECATE_AND_INLINE()
  bool SerializePartialToCord(absl::Cord* output) const {
    return SerializePartialToString(output);
  }

  // Make a Cord encoding the message. Is equivalent to calling
  // SerializeToCord() on a Cord and using that.  Returns an empty
  // Cord if SerializeToCord() would have returned an error.
  absl::Cord SerializeAsCord() const;
  // Like SerializeAsCord(), but allows missing required fields.
  absl::Cord SerializePartialAsCord() const;

  // Like SerializeToCord(), but appends to the data to the Cord's existing
  // contents.  All required fields must be set.
  PROTOBUF_DEPRECATE_AND_INLINE() bool AppendToCord(absl::Cord* output) const {
    return AppendToString(output);
  }
  // Like AppendToCord(), but allows missing required fields.
  PROTOBUF_DEPRECATE_AND_INLINE()
  bool AppendPartialToCord(absl::Cord* output) const {
    return AppendPartialToString(output);
  }

  // Computes the serialized size of the message.  This recursively calls
  // ByteSizeLong() on all embedded messages.
  //
  // ByteSizeLong() is generally linear in the number of fields defined for the
  // proto.
#if defined(PROTOBUF_CUSTOM_VTABLE)
  size_t ByteSizeLong() const { return _class_data_->byte_size_long(*this); }
#else
  virtual size_t ByteSizeLong() const = 0;
#endif  // PROTOBUF_CUSTOM_VTABLE

  // Legacy ByteSize() API.
  [[deprecated("Please use ByteSizeLong() instead")]] int ByteSize() const {
    return internal::ToIntSize(ByteSizeLong());
  }

  // Serializes the message without recomputing the size.  The message must not
  // have changed since the last call to ByteSize(), and the value returned by
  // ByteSize must be non-negative.  Otherwise the results are undefined.
  void SerializeWithCachedSizes(io::CodedOutputStream* output) const {
    output->SetCur(_InternalSerialize(output->Cur(), output->EpsCopy()));
  }

  // Functions below here are not part of the public interface.  It isn't
  // enforced, but they should be treated as private, and will be private
  // at some future time.  Unfortunately the implementation of the "friend"
  // keyword in GCC is broken at the moment, but we expect it will be fixed.

  // Like SerializeWithCachedSizes, but writes directly to *target, returning
  // a pointer to the byte immediately after the last byte written.  "target"
  // must point at a byte array of at least ByteSize() bytes.  Whether to use
  // deterministic serialization, e.g., maps in sorted order, is determined by
  // CodedOutputStream::IsDefaultSerializationDeterministic().
  uint8_t* SerializeWithCachedSizesToArray(uint8_t* target) const;

  // Returns the result of the last call to ByteSize().  An embedded message's
  // size is needed both to serialize it (only true for length-prefixed
  // submessages) and to compute the outer message's size.  Caching
  // the size avoids computing it multiple times.
  // Note that the submessage size is unnecessary when using
  // group encoding / delimited since we have SGROUP/EGROUP bounds.
  //
  // ByteSize() does not automatically use the cached size when available
  // because this would require invalidating it every time the message was
  // modified, which would be too hard and expensive.  (E.g. if a deeply-nested
  // sub-message is changed, all of its parents' cached sizes would need to be
  // invalidated, which is too much work for an otherwise inlined setter
  // method.)
#if defined(PROTOBUF_CUSTOM_VTABLE)
  int GetCachedSize() const { return AccessCachedSize().Get(); }
#else
  int GetCachedSize() const;
#endif

  const char* _InternalParse(const char* ptr, internal::ParseContext* ctx);

  void OnDemandRegisterArenaDtor(Arena* arena);

 protected:
  // Message implementations require access to internally visible API.
  static constexpr internal::InternalVisibility internal_visibility() {
    return internal::InternalVisibility{};
  }

  template <typename T>
  PROTOBUF_ALWAYS_INLINE static T* DefaultConstruct(Arena* arena) {
    return static_cast<T*>(Arena::DefaultConstruct<T>(arena));
  }

  template <typename T>
  static void* NewImpl(const void*, void* mem, Arena* arena) {
    return ::new (mem) T(arena);
  }
  template <typename T>
  static constexpr internal::MessageCreator GetNewImpl() {
#if defined(__cpp_if_constexpr)
    if constexpr (internal::EnableCustomNewFor<T>()) {
#else
    // Equally valid code, but might be more work for the compiler
    if (internal::EnableCustomNewFor<T>()) {
#endif
      return T::InternalNewImpl_();
    } else {
      return internal::MessageCreator(&T::PlacementNew_, sizeof(T), alignof(T));
    }
  }

#if defined(PROTOBUF_CUSTOM_VTABLE)
  template <typename T>
  static constexpr auto GetClearImpl() {
    return static_cast<void (MessageLite::*)()>(&T::Clear);
  }
#else   // PROTOBUF_CUSTOM_VTABLE
  // When custom vtables are off we avoid instantiating the functions because we
  // will not use them anyway. Less work for the compiler.
  template <typename T>
  using GetClearImpl = std::nullptr_t;
#endif  // PROTOBUF_CUSTOM_VTABLE

  template <typename T>
  PROTOBUF_ALWAYS_INLINE static T* CopyConstruct(Arena* arena, const T& from) {
    return static_cast<T*>(Arena::CopyConstruct<T>(arena, &from));
  }

  // As above, but for fields that use base class type. Eg foreign weak fields.
  static MessageLite* CopyConstruct(Arena* arena, const MessageLite& from);

  PROTOBUF_ALWAYS_INLINE static Message* CopyConstruct(Arena* arena,
                                                       const Message& from) {
    return reinterpret_cast<Message*>(
        CopyConstruct(arena, reinterpret_cast<const MessageLite&>(from)));
  }

  const internal::TcParseTableBase* GetTcParseTable() const {
    auto* data = GetClassData();
    ABSL_DCHECK(data != nullptr);

    auto* tc_table = data->tc_table;
    if (ABSL_PREDICT_FALSE(tc_table == nullptr)) {
      ABSL_DCHECK(!data->is_lite);
      return data->full().descriptor_methods->get_tc_table(*this);
    }
    return tc_table;
  }


#if defined(PROTOBUF_CUSTOM_VTABLE)
  explicit constexpr MessageLite(const internal::ClassData* data)
      : _class_data_(data) {}
  explicit MessageLite(Arena* arena, const internal::ClassData* data)
      : _internal_metadata_(arena), _class_data_(data) {}
#else   // PROTOBUF_CUSTOM_VTABLE
  constexpr MessageLite() {}
  explicit MessageLite(Arena* arena) : _internal_metadata_(arena) {}
  explicit constexpr MessageLite(const internal::ClassData*) {}
  explicit MessageLite(Arena* arena, const internal::ClassData*)
      : _internal_metadata_(arena) {}
#endif  // PROTOBUF_CUSTOM_VTABLE

  // GetClassData() returns a pointer to a ClassData struct which
  // exists in global memory and is unique to each subclass.  This uniqueness
  // property is used in order to quickly determine whether two messages are
  // of the same type.
  //
  // This is a work in progress. There are still some types (eg MapEntry) that
  // return a default table instead of a unique one.
#if defined(PROTOBUF_CUSTOM_VTABLE)
  const internal::ClassData* GetClassData() const {
    ::absl::PrefetchToLocalCache(_class_data_);
    return _class_data_;
  }
#else   // PROTOBUF_CUSTOM_VTABLE
  virtual const internal::ClassData* GetClassData() const = 0;
#endif  // PROTOBUF_CUSTOM_VTABLE

  internal::InternalMetadata _internal_metadata_;
#if defined(PROTOBUF_CUSTOM_VTABLE)
  const internal::ClassData* _class_data_;
#endif  // PROTOBUF_CUSTOM_VTABLE

  // Return the cached size object as described by
  // ClassData::cached_size_offset.
  const internal::CachedSize& AccessCachedSize() const {
    return *reinterpret_cast<const internal::CachedSize*>(
        reinterpret_cast<const char*>(this) +
        GetClassData()->cached_size_offset);
  }

 public:
  enum ParseFlags {
    // Merge vs. Parse:
    // Merge: overwrites scalar fields but appends to repeated fields in the
    //        destination; other fields in the destination remain untouched.
    // Parse: clears all fields in the destination before calling Merge.
    kMerge = 0,
    kParse = 1,
    // Default behaviour vs. Partial:
    // Default: a missing required field is deemed as parsing failure.
    // Partial: parse or merge will not give an error if input is missing
    //          required fields.
    kMergePartial = 2,
    kParsePartial = 3,
    // Default behaviour vs. Aliasing:
    // Default:  when merging, pointer is followed and expanded (deep-copy).
    // Aliasing: when merging, the destination message is allowed to retain
    //           pointers to the original structure (shallow-copy). This mostly
    //           is intended for use with STRING_PIECE.
    // NOTE: STRING_PIECE is not recommended for new usage. Prefer Cords.
    kMergeWithAliasing = 4,
    kParseWithAliasing = 5,
    kMergePartialWithAliasing = 6,
    kParsePartialWithAliasing = 7
  };

  template <ParseFlags flags, typename T>
  bool ParseFrom(const T& input);

  // Fast path when conditions match (ie. non-deterministic)
  //  uint8_t* _InternalSerialize(uint8_t* ptr) const;
#if defined(PROTOBUF_CUSTOM_VTABLE)
  uint8_t* _InternalSerialize(uint8_t* ptr,
                              io::EpsCopyOutputStream* stream) const {
    return _class_data_->serialize(*this, ptr, stream);
  }
#else   // PROTOBUF_CUSTOM_VTABLE
  virtual uint8_t* _InternalSerialize(
      uint8_t* ptr, io::EpsCopyOutputStream* stream) const = 0;
#endif  // PROTOBUF_CUSTOM_VTABLE

  // Identical to IsInitialized() except that it logs an error message.
  bool IsInitializedWithErrors() const {
    if (IsInitialized()) return true;
    LogInitializationErrorMessage();
    return false;
  }

#if defined(PROTOBUF_CUSTOM_VTABLE)
  void operator delete(MessageLite* msg, std::destroying_delete_t) {
    msg->DeleteInstance();
  }
#endif

 private:
  friend class FastReflectionMessageMutator;
  friend class AssignDescriptorsHelper;
  friend class FastReflectionStringSetter;
  friend class Message;
  friend class Reflection;
  friend class TypeId;
  friend class compiler::cpp::MessageTableTester;
  friend class internal::DescriptorPoolExtensionFinder;
  friend class internal::ExtensionSet;
  friend class internal::LazyField;
  friend class internal::SwapFieldHelper;
  friend class internal::TcParser;
  friend struct internal::TcParseTableBase;
  friend class internal::UntypedMapBase;
  friend class internal::WeakFieldMap;
  friend class internal::WireFormatLite;
  friend class internal::RustMapHelper;
  friend class internal::v2::TableDriven;
  friend internal::MessageCreator;

  template <typename Type>
  friend class Arena::InternalHelper;

  friend auto internal::GetClassData(const MessageLite& msg);

  void LogInitializationErrorMessage() const;

  bool MergeFromImpl(io::CodedInputStream* input, ParseFlags parse_flags);

  // Runs the destructor for this instance.
  void DestroyInstance();
  // Runs the destructor for this instance and deletes the memory via
  // `operator delete`
  void DeleteInstance();

  // For tests that need to inspect private _oneof_case_. It is the callers
  // responsibility to ensure T has the right member.
  template <typename T>
  static uint32_t GetOneofCaseOffsetForTesting() {
    return offsetof(T, _impl_._oneof_case_);
  }
};

// A `std::type_info` equivalent for protobuf message types.
// This class is preferred over using `typeid` for a few reasons:
//  - It works with RTTI disabled.
//  - It works for `DynamicMessage` types.
//  - It works in custom vtable mode.
//
// Usage:
//  - Instead of `typeid(Type)` use `TypeId::Get<Type>()`
//  - Instead of `typeid(expr)` use `TypeId::Get(expr)`
//
// Supports all relationals including <=>, and supports hashing via
// `absl::Hash`.
class TypeId {
 public:
  static TypeId Get(const MessageLite& msg) {
    return TypeId(msg.GetClassData());
  }

  template <typename T>
  static TypeId Get() {
    return TypeId(internal::MessageTraits<T>::class_data());
  }

  // Name of the message type.
  // Equivalent to `.GetTypeName()` on the message.
  absl::string_view name() const;

  friend constexpr bool operator==(TypeId a, TypeId b) {
    return a.data_ == b.data_;
  }
  friend constexpr bool operator!=(TypeId a, TypeId b) { return !(a == b); }
  friend constexpr bool operator<(TypeId a, TypeId b) {
    return a.data_ < b.data_;
  }
  friend constexpr bool operator>(TypeId a, TypeId b) {
    return a.data_ > b.data_;
  }
  friend constexpr bool operator<=(TypeId a, TypeId b) {
    return a.data_ <= b.data_;
  }
  friend constexpr bool operator>=(TypeId a, TypeId b) {
    return a.data_ >= b.data_;
  }

#if defined(__cpp_impl_three_way_comparison) && \
    __cpp_impl_three_way_comparison >= 201907L
  friend constexpr auto operator<=>(TypeId a, TypeId b) {
    return a.data_ <=> b.data_;
  }
#endif

  template <typename H>
  friend H AbslHashValue(H state, TypeId id) {
    return H::combine(std::move(state), id.data_);
  }

 private:
  constexpr explicit TypeId(const internal::ClassData* data) : data_(data) {}

  const internal::ClassData* data_;
};

namespace internal {

inline auto GetClassData(const MessageLite& msg) { return msg.GetClassData(); }

template <bool alias>
bool MergeFromImpl(absl::string_view input, MessageLite* msg,
                   const internal::TcParseTableBase* tc_table,
                   MessageLite::ParseFlags parse_flags);
extern template PROTOBUF_EXPORT_TEMPLATE_DECLARE bool MergeFromImpl<false>(
    absl::string_view input, MessageLite* msg,
    const internal::TcParseTableBase* tc_table,
    MessageLite::ParseFlags parse_flags);
extern template PROTOBUF_EXPORT_TEMPLATE_DECLARE bool MergeFromImpl<true>(
    absl::string_view input, MessageLite* msg,
    const internal::TcParseTableBase* tc_table,
    MessageLite::ParseFlags parse_flags);

template <bool alias>
bool MergeFromImpl(io::ZeroCopyInputStream* input, MessageLite* msg,
                   const internal::TcParseTableBase* tc_table,
                   MessageLite::ParseFlags parse_flags);
extern template PROTOBUF_EXPORT_TEMPLATE_DECLARE bool MergeFromImpl<false>(
    io::ZeroCopyInputStream* input, MessageLite* msg,
    const internal::TcParseTableBase* tc_table,
    MessageLite::ParseFlags parse_flags);
extern template PROTOBUF_EXPORT_TEMPLATE_DECLARE bool MergeFromImpl<true>(
    io::ZeroCopyInputStream* input, MessageLite* msg,
    const internal::TcParseTableBase* tc_table,
    MessageLite::ParseFlags parse_flags);

struct BoundedZCIS {
  io::ZeroCopyInputStream* zcis;
  int limit;
};

template <bool alias>
bool MergeFromImpl(BoundedZCIS input, MessageLite* msg,
                   const internal::TcParseTableBase* tc_table,
                   MessageLite::ParseFlags parse_flags);
extern template PROTOBUF_EXPORT_TEMPLATE_DECLARE bool MergeFromImpl<false>(
    BoundedZCIS input, MessageLite* msg,
    const internal::TcParseTableBase* tc_table,
    MessageLite::ParseFlags parse_flags);
extern template PROTOBUF_EXPORT_TEMPLATE_DECLARE bool MergeFromImpl<true>(
    BoundedZCIS input, MessageLite* msg,
    const internal::TcParseTableBase* tc_table,
    MessageLite::ParseFlags parse_flags);

template <typename T>
struct SourceWrapper;

template <bool alias, typename T>
bool MergeFromImpl(const SourceWrapper<T>& input, MessageLite* msg,
                   const internal::TcParseTableBase* tc_table,
                   MessageLite::ParseFlags parse_flags) {
  return input.template MergeInto<alias>(msg, tc_table, parse_flags);
}

}  // namespace internal

template <MessageLite::ParseFlags flags, typename T>
bool MessageLite::ParseFrom(const T& input) {
  if (flags & kParse) Clear();
  constexpr bool alias = (flags & kMergeWithAliasing) != 0;
  const internal::TcParseTableBase* tc_table;
  PROTOBUF_ALWAYS_INLINE_CALL tc_table = GetTcParseTable();
  return internal::MergeFromImpl<alias>(input, this, tc_table, flags);
}

// ===================================================================
// Shutdown support.


// Shut down the entire protocol buffers library, deleting all static-duration
// objects allocated by the library or by generated .pb.cc files.
//
// There are two reasons you might want to call this:
// * You use a draconian definition of "memory leak" in which you expect
//   every single malloc() to have a corresponding free(), even for objects
//   which live until program exit.
// * You are writing a dynamically-loaded library which needs to clean up
//   after itself when the library is unloaded.
//
// It is safe to call this multiple times.  However, it is not safe to use
// any other part of the protocol buffers library after
// ShutdownProtobufLibrary() has been called. Furthermore this call is not
// thread safe, user needs to synchronize multiple calls.
PROTOBUF_EXPORT void ShutdownProtobufLibrary();

namespace internal {

// Register a function to be called when ShutdownProtocolBuffers() is called.
PROTOBUF_EXPORT void OnShutdown(void (*func)());
// Run an arbitrary function on an arg
PROTOBUF_EXPORT void OnShutdownRun(void (*f)(const void*), const void* arg);

template <typename T>
T* OnShutdownDelete(T* p) {
  OnShutdownRun([](const void* pp) { delete static_cast<const T*>(pp); }, p);
  return p;
}

template <bool test_call, typename MessageLite>
PROTOBUF_ALWAYS_INLINE MessageLite* MessageCreator::PlacementNew(
    const MessageLite* prototype_for_func,
    const MessageLite* prototype_for_copy, void* mem, Arena* arena) const {
  ABSL_DCHECK_EQ(reinterpret_cast<uintptr_t>(mem) % alignment_, 0u);
  const Tag as_tag = tag();
  // When the feature is not enabled we skip the `as_tag` check since it is
  // unnecessary. Except for testing, where we want to test the copy logic even
  // when we can't use it for real messages.
  constexpr bool kMustBeFunc = !test_call && !internal::EnableCustomNew();
  static_assert(kFunc < 0 && !(kZeroInit < 0) && !(kMemcpy < 0),
                "Only kFunc must be the only negative value");
  if (ABSL_PREDICT_FALSE(kMustBeFunc || as_tag < 0)) {
    PROTOBUF_DEBUG_COUNTER("MessageCreator.Func").Inc();
    return static_cast<MessageLite*>(func_(prototype_for_func, mem, arena));
  }

  char* dst = static_cast<char*>(mem);
  const size_t size = allocation_size_;
  const char* src = reinterpret_cast<const char*>(prototype_for_copy);

  // These are a bit more efficient than calling normal memset/memcpy because:
  //  - We know the minimum size is 16. We have a fallback for when it is not.
  //  - We can "underflow" the buffer because those are the MessageLite bytes
  //    we will set later.
  if (as_tag == kZeroInit) {
    // Make sure the input is really all zeros.
    ABSL_DCHECK(std::all_of(src + sizeof(MessageLite), src + size,
                            [](auto c) { return c == 0; }));

    if (sizeof(MessageLite) != 16) {
      memset(dst, 0, size);
    } else if (size <= 32) {
      memset(dst + size - 16, 0, 16);
    } else if (size <= 64) {
      memset(dst + 16, 0, 16);
      memset(dst + size - 32, 0, 32);
    } else {
      for (size_t offset = 16; offset + 64 < size; offset += 64) {
        absl::PrefetchToLocalCacheForWrite(dst + offset + 64);
        memset(dst + offset, 0, 64);
      }
      memset(dst + size - 64, 0, 64);
    }
  } else {
    ABSL_DCHECK_EQ(+as_tag, +kMemcpy);

    if (sizeof(MessageLite) != 16) {
      memcpy(dst, src, size);
    } else if (size <= 32) {
      memcpy(dst + size - 16, src + size - 16, 16);
    } else if (size <= 64) {
      memcpy(dst + 16, src + 16, 16);
      memcpy(dst + size - 32, src + size - 32, 32);
    } else {
      for (size_t offset = 16; offset + 64 < size; offset += 64) {
        absl::PrefetchToLocalCache(src + offset + 64);
        absl::PrefetchToLocalCacheForWrite(dst + offset + 64);
        memcpy(dst + offset, src + offset, 64);
      }
      memcpy(dst + size - 64, src + size - 64, 64);
    }
  }

  if (arena_bits() != 0) {
    if (as_tag == kZeroInit) {
      PROTOBUF_DEBUG_COUNTER("MessageCreator.ZeroArena").Inc();
    } else {
      PROTOBUF_DEBUG_COUNTER("MessageCreator.McpyArena").Inc();
    }
  } else {
    if (as_tag == kZeroInit) {
      PROTOBUF_DEBUG_COUNTER("MessageCreator.Zero").Inc();
    } else {
      PROTOBUF_DEBUG_COUNTER("MessageCreator.Mcpy").Inc();
    }
  }

  if (internal::PerformDebugChecks() || arena != nullptr) {
    if (uintptr_t offsets = arena_bits()) {
      do {
        const size_t offset = absl::countr_zero(offsets) * sizeof(Arena*);
        ABSL_DCHECK_LE(offset + sizeof(Arena*), size);
        // Verify we are overwriting a null pointer. If we are not, there is a
        // bug somewhere.
        ABSL_DCHECK_EQ(*reinterpret_cast<Arena**>(dst + offset), nullptr);
        memcpy(dst + offset, &arena, sizeof(arena));
        offsets &= offsets - 1;
      } while (offsets != 0);
    }
  }

  // The second memcpy overwrites part of the first, but the compiler should
  // avoid the double-write. It's easier than trying to avoid the overlap.
  memcpy(dst, static_cast<const void*>(prototype_for_copy),
         sizeof(MessageLite));
  memcpy(dst + PROTOBUF_FIELD_OFFSET(MessageLite, _internal_metadata_), &arena,
         sizeof(arena));
  return Launder(reinterpret_cast<MessageLite*>(mem));
}

template <bool test_call, typename MessageLite>
PROTOBUF_ALWAYS_INLINE MessageLite* MessageCreator::New(
    const MessageLite* prototype_for_func,
    const MessageLite* prototype_for_copy, Arena* arena) const {
  return PlacementNew<test_call>(prototype_for_func, prototype_for_copy,
                                 arena != nullptr
                                     ? arena->AllocateAligned(allocation_size_)
                                     : ::operator new(allocation_size_),
                                 arena);
}

}  // namespace internal

std::string ShortFormat(const MessageLite& message_lite);
std::string Utf8Format(const MessageLite& message_lite);

// Cast functions for message pointer/references.
// This is the supported API to cast from a Message/MessageLite to derived
// types. These work even when RTTI is disabled on message types.
//
// The template parameter is simplified and the return type is inferred from the
// input. Eg just `DynamicCastMessage<Foo>(x)` instead of
// `DynamicCastMessage<const Foo*>(x)`.
//
// `DynamicCastMessage` is similar to `dynamic_cast`, returns `nullptr` when the
// input is not an instance of `T`. The overloads that take a reference will
// throw std::bad_cast on mismatch, or terminate if compiled without exceptions.
//
// `DownCastMessage` is a lightweight function for downcasting base
// `MessageLite` pointer to derived type, where it only does type checking if
// !NDEBUG. It should only be used when the caller is certain that the input
// message is of instance `T`.
template <typename T>
const T* DynamicCastMessage(const MessageLite* from) {
  static_assert(std::is_base_of<MessageLite, T>::value, "");

  // We might avoid the call to T::GetClassData() altogether if T were to
  // expose the class data pointer.
  if (from == nullptr || TypeId::Get<T>() != TypeId::Get(*from)) {
    return nullptr;
  }

  return static_cast<const T*>(from);
}

template <typename T>
T* DynamicCastMessage(MessageLite* from) {
  return const_cast<T*>(
      DynamicCastMessage<T>(static_cast<const MessageLite*>(from)));
}

namespace internal {
[[noreturn]] PROTOBUF_EXPORT void FailDynamicCast(const MessageLite& from,
                                                  const MessageLite& to);
}  // namespace internal

template <typename T>
const T& DynamicCastMessage(const MessageLite& from) {
  const T* destination_message = DynamicCastMessage<T>(&from);
  if (ABSL_PREDICT_FALSE(destination_message == nullptr)) {
    // If exceptions are enabled, throw.
    // Otherwise, log a fatal error.
#if defined(ABSL_HAVE_EXCEPTIONS)
    throw std::bad_cast();
#endif
    // Move the logging into an out-of-line function to reduce bloat in the
    // caller.
    internal::FailDynamicCast(from, T::default_instance());
  }
  return *destination_message;
}

template <typename T>
T& DynamicCastMessage(MessageLite& from) {
  return const_cast<T&>(
      DynamicCastMessage<T>(static_cast<const MessageLite&>(from)));
}

template <typename T>
const T* DownCastMessage(const MessageLite* from) {
  internal::StrongReferenceToType<T>();
  ABSL_DCHECK(DynamicCastMessage<T>(from) == from)
      << "Cannot downcast " << from->GetTypeName() << " to "
      << T::default_instance().GetTypeName();
  return static_cast<const T*>(from);
}

template <typename T>
T* DownCastMessage(MessageLite* from) {
  return const_cast<T*>(
      DownCastMessage<T>(static_cast<const MessageLite*>(from)));
}

template <typename T>
const T& DownCastMessage(const MessageLite& from) {
  return *DownCastMessage<T>(&from);
}

template <typename T>
T& DownCastMessage(MessageLite& from) {
  return *DownCastMessage<T>(&from);
}

template <>
inline const MessageLite* DynamicCastMessage(const MessageLite* from) {
  return from;
}
template <>
inline const MessageLite* DownCastMessage(const MessageLite* from) {
  return from;
}

// Deprecated names for the cast functions.
// Prefer the ones above.
template <typename T>
PROTOBUF_DEPRECATE_AND_INLINE()
const T* DynamicCastToGenerated(const MessageLite* from) {
  return DynamicCastMessage<T>(from);
}

template <typename T>
PROTOBUF_DEPRECATE_AND_INLINE()
T* DynamicCastToGenerated(MessageLite* from) {
  return DynamicCastMessage<T>(from);
}

template <typename T>
PROTOBUF_DEPRECATE_AND_INLINE()
const T& DynamicCastToGenerated(const MessageLite& from) {
  return DynamicCastMessage<T>(from);
}

template <typename T>
PROTOBUF_DEPRECATE_AND_INLINE()
T& DynamicCastToGenerated(MessageLite& from) {
  return DynamicCastMessage<T>(from);
}

template <typename T>
PROTOBUF_DEPRECATE_AND_INLINE()
const T* DownCastToGenerated(const MessageLite* from) {
  return DownCastMessage<T>(from);
}

template <typename T>
PROTOBUF_DEPRECATE_AND_INLINE()
T* DownCastToGenerated(MessageLite* from) {
  return DownCastMessage<T>(from);
}

template <typename T>
PROTOBUF_DEPRECATE_AND_INLINE()
const T& DownCastToGenerated(const MessageLite& from) {
  return DownCastMessage<T>(from);
}

template <typename T>
PROTOBUF_DEPRECATE_AND_INLINE()
T& DownCastToGenerated(MessageLite& from) {
  return DownCastMessage<T>(from);
}

// Overloads for `std::shared_ptr` to substitute `std::dynamic_pointer_cast`
template <typename T>
std::shared_ptr<T> DynamicCastMessage(std::shared_ptr<MessageLite> ptr) {
  if (auto* res = DynamicCastMessage<T>(ptr.get())) {
    // Use aliasing constructor to keep the same control block.
    return std::shared_ptr<T>(std::move(ptr), res);
  } else {
    return nullptr;
  }
}

template <typename T>
std::shared_ptr<const T> DynamicCastMessage(
    std::shared_ptr<const MessageLite> ptr) {
  if (auto* res = DynamicCastMessage<T>(ptr.get())) {
    // Use aliasing constructor to keep the same control block.
    return std::shared_ptr<const T>(std::move(ptr), res);
  } else {
    return nullptr;
  }
}

}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_MESSAGE_LITE_H__
