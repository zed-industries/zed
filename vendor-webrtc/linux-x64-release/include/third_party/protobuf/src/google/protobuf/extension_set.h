// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// This header is logically internal, but is made public because it is used
// from protocol-compiler-generated code, which may reside in other components.

#ifndef GOOGLE_PROTOBUF_EXTENSION_SET_H__
#define GOOGLE_PROTOBUF_EXTENSION_SET_H__

#include <algorithm>
#include <atomic>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <initializer_list>
#include <string>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "google/protobuf/stubs/common.h"
#include "absl/base/call_once.h"
#include "absl/base/casts.h"
#include "absl/base/prefetch.h"
#include "absl/container/btree_map.h"
#include "absl/log/absl_check.h"
#include "google/protobuf/generated_enum_util.h"
#include "google/protobuf/internal_visibility.h"
#include "google/protobuf/port.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/parse_context.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "google/protobuf/wire_format_lite.h"

// clang-format off
#include "google/protobuf/port_def.inc"  // Must be last
// clang-format on

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif


namespace google {
namespace protobuf {
class Arena;
class Descriptor;       // descriptor.h
class FieldDescriptor;  // descriptor.h
class DescriptorPool;   // descriptor.h
class MessageLite;      // message_lite.h
class Message;          // message.h
class MessageFactory;   // message.h
class Reflection;       // message.h
class UnknownFieldSet;  // unknown_field_set.h
class FeatureSet;
namespace internal {
struct DescriptorTable;
class FieldSkipper;     // wire_format_lite.h
class ReflectionVisit;  // message_reflection_util.h
class WireFormat;
struct DynamicExtensionInfoHelper;
void InitializeLazyExtensionSet();
}  // namespace internal
}  // namespace protobuf
}  // namespace google
namespace pb {
class CppFeatures;
}  // namespace pb

namespace google {
namespace protobuf {
namespace internal {

class InternalMetadata;

namespace v2 {
class TableDriven;
}  // namespace v2

// Used to store values of type WireFormatLite::FieldType without having to
// #include wire_format_lite.h.  Also, ensures that we use only one byte to
// store these values, which is important to keep the layout of
// ExtensionSet::Extension small.
typedef uint8_t FieldType;

// Version of the above which takes an argument.  This is needed to deal with
// extensions that are not compiled in.
typedef bool EnumValidityFuncWithArg(const void* arg, int number);

enum class LazyAnnotation : int8_t {
  kUndefined = 0,
  kLazy = 1,
  kEager = 2,
};

// Information about a registered extension.
struct ExtensionInfo {
  constexpr ExtensionInfo() : enum_validity_check() {}
  constexpr ExtensionInfo(const MessageLite* extendee, int param_number,
                          FieldType type_param, bool isrepeated, bool ispacked)
      : message(extendee),
        number(param_number),
        type(type_param),
        is_repeated(isrepeated),
        is_packed(ispacked),
        enum_validity_check() {}
  constexpr ExtensionInfo(const MessageLite* extendee, int param_number,
                          FieldType type_param, bool isrepeated, bool ispacked,
                          LazyEagerVerifyFnType verify_func,
                          LazyAnnotation islazy = LazyAnnotation::kUndefined)
      : message(extendee),
        number(param_number),
        type(type_param),
        is_repeated(isrepeated),
        is_packed(ispacked),
        is_lazy(islazy),
        enum_validity_check(),
        lazy_eager_verify_func(verify_func) {}

  const MessageLite* message = nullptr;
  int number = 0;

  FieldType type = 0;
  bool is_repeated = false;
  bool is_packed = false;
  LazyAnnotation is_lazy = LazyAnnotation::kUndefined;

  struct EnumValidityCheck {
    // TODO: Fully remove the function pointer approach.
    EnumValidityFuncWithArg* func;
    const void* arg;

    bool IsValid(int value) const {
      return func != nullptr ? func(arg, value)
                             : internal::ValidateEnum(
                                   value, static_cast<const uint32_t*>(arg));
    }
  };

  struct MessageInfo {
    const MessageLite* prototype;
    // The TcParse table used for this object.
    // Never null. (except in platforms that don't constant initialize default
    // instances)
    const internal::TcParseTableBase* tc_table;
  };

  union {
    EnumValidityCheck enum_validity_check;
    MessageInfo message_info;
  };

  // The descriptor for this extension, if one exists and is known.  May be
  // nullptr.  Must not be nullptr if the descriptor for the extension does not
  // live in the same pool as the descriptor for the containing type.
  const FieldDescriptor* descriptor = nullptr;

  // If this field is potentially lazy this function can be used as a cheap
  // verification of the raw bytes.
  // If nullptr then no verification is performed.
  LazyEagerVerifyFnType lazy_eager_verify_func = nullptr;
};


// An ExtensionFinder is an object which looks up extension definitions.  It
// must implement this method:
//
// bool Find(int number, ExtensionInfo* output);

// GeneratedExtensionFinder is an ExtensionFinder which finds extensions
// defined in .proto files which have been compiled into the binary.
class PROTOBUF_EXPORT GeneratedExtensionFinder {
 public:
  explicit GeneratedExtensionFinder(const MessageLite* extendee)
      : extendee_(extendee) {}

  // Returns true and fills in *output if found, otherwise returns false.
  bool Find(int number, ExtensionInfo* output);

 private:
  const MessageLite* extendee_;
};

// Note:  extension_set_heavy.cc defines DescriptorPoolExtensionFinder for
// finding extensions from a DescriptorPool.

// This is an internal helper class intended for use within the protocol buffer
// library and generated classes.  Clients should not use it directly.  Instead,
// use the generated accessors such as GetExtension() of the class being
// extended.
//
// This class manages extensions for a protocol message object.  The
// message's HasExtension(), GetExtension(), MutableExtension(), and
// ClearExtension() methods are just thin wrappers around the embedded
// ExtensionSet.  When parsing, if a tag number is encountered which is
// inside one of the message type's extension ranges, the tag is passed
// off to the ExtensionSet for parsing.  Etc.
class PROTOBUF_EXPORT ExtensionSet {
 public:
  constexpr ExtensionSet() : ExtensionSet(nullptr) {}
  ExtensionSet(const ExtensionSet& rhs) = delete;

  // Arena enabled constructors: for internal use only.
  ExtensionSet(internal::InternalVisibility, Arena* arena)
      : ExtensionSet(arena) {}

  // TODO: make constructor private, and migrate `ArenaInitialized`
  // to `InternalVisibility` overloaded constructor(s).
  explicit constexpr ExtensionSet(Arena* arena);
  ExtensionSet(ArenaInitialized, Arena* arena) : ExtensionSet(arena) {}

  ExtensionSet& operator=(const ExtensionSet&) = delete;
  ~ExtensionSet();

  // These are called at startup by protocol-compiler-generated code to
  // register known extensions.  The registrations are used by ParseField()
  // to look up extensions for parsed field numbers.  Note that dynamic parsing
  // does not use ParseField(); only protocol-compiler-generated parsing
  // methods do.
  static void RegisterExtension(const MessageLite* extendee, int number,
                                FieldType type, bool is_repeated,
                                bool is_packed);
  static void RegisterEnumExtension(const MessageLite* extendee, int number,
                                    FieldType type, bool is_repeated,
                                    bool is_packed,
                                    const uint32_t* validation_data);
  static void RegisterMessageExtension(const MessageLite* extendee, int number,
                                       FieldType type, bool is_repeated,
                                       bool is_packed,
                                       const MessageLite* prototype,
                                       LazyEagerVerifyFnType verify_func,
                                       LazyAnnotation is_lazy);

  // In weak descriptor mode we register extensions in two phases.
  // This function determines if it is the right time to register a particular
  // extension.
  // During "preregistration" we only register extensions that have all their
  // types linked in.
  struct WeakPrototypeRef {
    const internal::DescriptorTable* table;
    int index;
  };
  static bool ShouldRegisterAtThisTime(
      std::initializer_list<WeakPrototypeRef> messages,
      bool is_preregistration);

  // =================================================================

  // Add all fields which are currently present to the given vector.  This
  // is useful to implement Reflection::ListFields(). Descriptors are appended
  // in increasing tag order.
  void AppendToList(const Descriptor* extendee, const DescriptorPool* pool,
                    std::vector<const FieldDescriptor*>* output) const;

  // =================================================================
  // Accessors
  //
  // Generated message classes include type-safe templated wrappers around
  // these methods.  Generally you should use those rather than call these
  // directly, unless you are doing low-level memory management.
  //
  // When calling any of these accessors, the extension number requested
  // MUST exist in the DescriptorPool provided to the constructor.  Otherwise,
  // the method will fail an assert.  Normally, though, you would not call
  // these directly; you would either call the generated accessors of your
  // message class (e.g. GetExtension()) or you would call the accessors
  // of the reflection interface.  In both cases, it is impossible to
  // trigger this assert failure:  the generated accessors only accept
  // linked-in extension types as parameters, while the Reflection interface
  // requires you to provide the FieldDescriptor describing the extension.
  //
  // When calling any of these accessors, a protocol-compiler-generated
  // implementation of the extension corresponding to the number MUST
  // be linked in, and the FieldDescriptor used to refer to it MUST be
  // the one generated by that linked-in code.  Otherwise, the method will
  // die on an assert failure.  The message objects returned by the message
  // accessors are guaranteed to be of the correct linked-in type.
  //
  // These methods pretty much match Reflection except that:
  // - They're not virtual.
  // - They identify fields by number rather than FieldDescriptors.
  // - They identify enum values using integers rather than descriptors.
  // - Strings provide Mutable() in addition to Set() accessors.

  bool Has(int number) const;
  int ExtensionSize(int number) const;  // Size of a repeated extension.
  int NumExtensions() const;            // The number of extensions
  FieldType ExtensionType(int number) const;
  void ClearExtension(int number);

  // singular fields -------------------------------------------------

  int32_t GetInt32(int number, int32_t default_value) const;
  int64_t GetInt64(int number, int64_t default_value) const;
  uint32_t GetUInt32(int number, uint32_t default_value) const;
  uint64_t GetUInt64(int number, uint64_t default_value) const;
  float GetFloat(int number, float default_value) const;
  double GetDouble(int number, double default_value) const;
  bool GetBool(int number, bool default_value) const;
  int GetEnum(int number, int default_value) const;
  const std::string& GetString(int number,
                               const std::string& default_value) const;
  const MessageLite& GetMessage(int number,
                                const MessageLite& default_value) const;
  const MessageLite& GetMessage(int number, const Descriptor* message_type,
                                MessageFactory* factory) const;

  // |descriptor| may be nullptr so long as it is known that the descriptor for
  // the extension lives in the same pool as the descriptor for the containing
  // type.
#define desc const FieldDescriptor* descriptor  // avoid line wrapping
  void SetInt32(int number, FieldType type, int32_t value, desc);
  void SetInt64(int number, FieldType type, int64_t value, desc);
  void SetUInt32(int number, FieldType type, uint32_t value, desc);
  void SetUInt64(int number, FieldType type, uint64_t value, desc);
  void SetFloat(int number, FieldType type, float value, desc);
  void SetDouble(int number, FieldType type, double value, desc);
  void SetBool(int number, FieldType type, bool value, desc);
  void SetEnum(int number, FieldType type, int value, desc);
  void SetString(int number, FieldType type, std::string value, desc);
  std::string* MutableString(int number, FieldType type, desc);
  MessageLite* MutableMessage(int number, FieldType type,
                              const MessageLite& prototype, desc);
  MessageLite* MutableMessage(const FieldDescriptor* descriptor,
                              MessageFactory* factory);
  // Adds the given message to the ExtensionSet, taking ownership of the
  // message object. Existing message with the same number will be deleted.
  // If "message" is nullptr, this is equivalent to "ClearExtension(number)".
  void SetAllocatedMessage(int number, FieldType type,
                           const FieldDescriptor* descriptor,
                           MessageLite* message);
  void UnsafeArenaSetAllocatedMessage(int number, FieldType type,
                                      const FieldDescriptor* descriptor,
                                      MessageLite* message);
  [[nodiscard]] MessageLite* ReleaseMessage(int number,
                                            const MessageLite& prototype);
  MessageLite* UnsafeArenaReleaseMessage(int number,
                                         const MessageLite& prototype);

  [[nodiscard]] MessageLite* ReleaseMessage(const FieldDescriptor* descriptor,
                                            MessageFactory* factory);
  MessageLite* UnsafeArenaReleaseMessage(const FieldDescriptor* descriptor,
                                         MessageFactory* factory);
#undef desc
  Arena* GetArena() const { return arena_; }

  // repeated fields -------------------------------------------------

  // Fetches a RepeatedField extension by number; returns |default_value|
  // if no such extension exists. User should not touch this directly; it is
  // used by the GetRepeatedExtension() method.
  const void* GetRawRepeatedField(int number, const void* default_value) const;
  // Fetches a mutable version of a RepeatedField extension by number,
  // instantiating one if none exists. Similar to above, user should not use
  // this directly; it underlies MutableRepeatedExtension().
  void* MutableRawRepeatedField(int number, FieldType field_type, bool packed,
                                const FieldDescriptor* desc);

  // This is an overload of MutableRawRepeatedField to maintain compatibility
  // with old code using a previous API. This version of
  // MutableRawRepeatedField() will ABSL_CHECK-fail on a missing extension.
  // (E.g.: borg/clients/internal/proto1/proto2_reflection.cc.)
  void* MutableRawRepeatedField(int number);

  int32_t GetRepeatedInt32(int number, int index) const;
  int64_t GetRepeatedInt64(int number, int index) const;
  uint32_t GetRepeatedUInt32(int number, int index) const;
  uint64_t GetRepeatedUInt64(int number, int index) const;
  float GetRepeatedFloat(int number, int index) const;
  double GetRepeatedDouble(int number, int index) const;
  bool GetRepeatedBool(int number, int index) const;
  int GetRepeatedEnum(int number, int index) const;
  const std::string& GetRepeatedString(int number, int index) const;
  const MessageLite& GetRepeatedMessage(int number, int index) const;

  void SetRepeatedInt32(int number, int index, int32_t value);
  void SetRepeatedInt64(int number, int index, int64_t value);
  void SetRepeatedUInt32(int number, int index, uint32_t value);
  void SetRepeatedUInt64(int number, int index, uint64_t value);
  void SetRepeatedFloat(int number, int index, float value);
  void SetRepeatedDouble(int number, int index, double value);
  void SetRepeatedBool(int number, int index, bool value);
  void SetRepeatedEnum(int number, int index, int value);
  void SetRepeatedString(int number, int index, std::string value);
  std::string* MutableRepeatedString(int number, int index);
  MessageLite* MutableRepeatedMessage(int number, int index);

#define desc const FieldDescriptor* descriptor  // avoid line wrapping
  void AddInt32(int number, FieldType type, bool packed, int32_t value, desc);
  void AddInt64(int number, FieldType type, bool packed, int64_t value, desc);
  void AddUInt32(int number, FieldType type, bool packed, uint32_t value, desc);
  void AddUInt64(int number, FieldType type, bool packed, uint64_t value, desc);
  void AddFloat(int number, FieldType type, bool packed, float value, desc);
  void AddDouble(int number, FieldType type, bool packed, double value, desc);
  void AddBool(int number, FieldType type, bool packed, bool value, desc);
  void AddEnum(int number, FieldType type, bool packed, int value, desc);
  void AddString(int number, FieldType type, std::string value, desc);
  std::string* AddString(int number, FieldType type, desc);
  MessageLite* AddMessage(int number, FieldType type,
                          const MessageLite& prototype, desc);
  MessageLite* AddMessage(const FieldDescriptor* descriptor,
                          MessageFactory* factory);
  void AddAllocatedMessage(const FieldDescriptor* descriptor,
                           MessageLite* new_entry);
  void UnsafeArenaAddAllocatedMessage(const FieldDescriptor* descriptor,
                                      MessageLite* new_entry);
#undef desc

  void RemoveLast(int number);
  [[nodiscard]] MessageLite* ReleaseLast(int number);
  MessageLite* UnsafeArenaReleaseLast(int number);
  void SwapElements(int number, int index1, int index2);

  // =================================================================
  // convenience methods for implementing methods of Message
  //
  // These could all be implemented in terms of the other methods of this
  // class, but providing them here helps keep the generated code size down.

  void Clear();
  void MergeFrom(const MessageLite* extendee, const ExtensionSet& other);
  void Swap(const MessageLite* extendee, ExtensionSet* other);
  void InternalSwap(ExtensionSet* other);
  void SwapExtension(const MessageLite* extendee, ExtensionSet* other,
                     int number);
  void UnsafeShallowSwapExtension(ExtensionSet* other, int number);
  bool IsInitialized(const MessageLite* extendee) const;

  // Lite parser
  const char* ParseField(uint64_t tag, const char* ptr,
                         const MessageLite* extendee,
                         internal::InternalMetadata* metadata,
                         internal::ParseContext* ctx);
  // Full parser
  const char* ParseField(uint64_t tag, const char* ptr, const Message* extendee,
                         internal::InternalMetadata* metadata,
                         internal::ParseContext* ctx);
  template <typename Msg>
  const char* ParseMessageSet(const char* ptr, const Msg* extendee,
                              InternalMetadata* metadata,
                              internal::ParseContext* ctx) {
    while (!ctx->Done(&ptr)) {
      uint32_t tag;
      ptr = ReadTag(ptr, &tag);
      GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
      if (tag == WireFormatLite::kMessageSetItemStartTag) {
        ptr = ctx->ParseGroupInlined(ptr, tag, [&](const char* ptr) {
          return ParseMessageSetItem(ptr, extendee, metadata, ctx);
        });
        GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
      } else {
        if (tag == 0 || (tag & 7) == 4) {
          ctx->SetLastTag(tag);
          return ptr;
        }
        ptr = ParseField(tag, ptr, extendee, metadata, ctx);
        GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
      }
    }
    return ptr;
  }

  // Write all extension fields with field numbers in the range
  //   [start_field_number, end_field_number)
  // to the output stream, using the cached sizes computed when ByteSize() was
  // last called.  Note that the range bounds are inclusive-exclusive.
  void SerializeWithCachedSizes(const MessageLite* extendee,
                                int start_field_number, int end_field_number,
                                io::CodedOutputStream* output) const {
    output->SetCur(_InternalSerialize(extendee, start_field_number,
                                      end_field_number, output->Cur(),
                                      output->EpsCopy()));
  }

  // Same as SerializeWithCachedSizes, but without any bounds checking.
  // The caller must ensure that target has sufficient capacity for the
  // serialized extensions.
  //
  // Returns a pointer past the last written byte.
  uint8_t* _InternalSerialize(const MessageLite* extendee,
                              int start_field_number, int end_field_number,
                              uint8_t* target,
                              io::EpsCopyOutputStream* stream) const {
    if (flat_size_ == 0) {
      assert(!is_large());
      return target;
    }
    return _InternalSerializeImpl(extendee, start_field_number,
                                  end_field_number, target, stream);
  }

  // Same as _InternalSerialize, but do not verify the range of field numbers.
  uint8_t* _InternalSerializeAll(const MessageLite* extendee, uint8_t* target,
                                 io::EpsCopyOutputStream* stream) const {
    if (flat_size_ == 0) {
      assert(!is_large());
      return target;
    }
    return _InternalSerializeAllImpl(extendee, target, stream);
  }

  // Like above but serializes in MessageSet format.
  void SerializeMessageSetWithCachedSizes(const MessageLite* extendee,
                                          io::CodedOutputStream* output) const {
    output->SetCur(InternalSerializeMessageSetWithCachedSizesToArray(
        extendee, output->Cur(), output->EpsCopy()));
  }
  uint8_t* InternalSerializeMessageSetWithCachedSizesToArray(
      const MessageLite* extendee, uint8_t* target,
      io::EpsCopyOutputStream* stream) const;

  // For backward-compatibility, versions of two of the above methods that
  // serialize deterministically iff SetDefaultSerializationDeterministic()
  // has been called.
  uint8_t* SerializeWithCachedSizesToArray(int start_field_number,
                                           int end_field_number,
                                           uint8_t* target) const;
  uint8_t* SerializeMessageSetWithCachedSizesToArray(
      const MessageLite* extendee, uint8_t* target) const;

  // Returns the total serialized size of all the extensions.
  size_t ByteSize() const;

  // Like ByteSize() but uses MessageSet format.
  size_t MessageSetByteSize() const;

  // Returns (an estimate of) the total number of bytes used for storing the
  // extensions in memory, excluding sizeof(*this).  If the ExtensionSet is
  // for a lite message (and thus possibly contains lite messages), the results
  // are undefined (might work, might crash, might corrupt data, might not even
  // be linked in).  It's up to the protocol compiler to avoid calling this on
  // such ExtensionSets (easy enough since lite messages don't implement
  // SpaceUsed()).
  size_t SpaceUsedExcludingSelfLong() const;

  // This method just calls SpaceUsedExcludingSelfLong() but it can not be
  // inlined because the definition of SpaceUsedExcludingSelfLong() is not
  // included in lite runtime and when an inline method refers to it MSVC
  // will complain about unresolved symbols when building the lite runtime
  // as .dll.
  int SpaceUsedExcludingSelf() const;

  static constexpr size_t InternalGetArenaOffset(internal::InternalVisibility) {
    return PROTOBUF_FIELD_OFFSET(ExtensionSet, arena_);
  }

 private:
  template <typename Type>
  friend class PrimitiveTypeTraits;

  template <typename Type>
  friend class RepeatedPrimitiveTypeTraits;

  template <typename Type>
  friend class EnumTypeTraits;

  template <typename Type>
  friend class RepeatedEnumTypeTraits;

  friend class google::protobuf::Reflection;
  friend class google::protobuf::internal::ReflectionVisit;
  friend struct google::protobuf::internal::DynamicExtensionInfoHelper;
  friend class google::protobuf::internal::WireFormat;
  friend class google::protobuf::internal::v2::TableDriven;

  friend void internal::InitializeLazyExtensionSet();

  static bool FieldTypeIsPointer(FieldType type);

  const int32_t& GetRefInt32(int number, const int32_t& default_value) const;
  const int64_t& GetRefInt64(int number, const int64_t& default_value) const;
  const uint32_t& GetRefUInt32(int number, const uint32_t& default_value) const;
  const uint64_t& GetRefUInt64(int number, const uint64_t& default_value) const;
  const float& GetRefFloat(int number, const float& default_value) const;
  const double& GetRefDouble(int number, const double& default_value) const;
  const bool& GetRefBool(int number, const bool& default_value) const;
  const int& GetRefEnum(int number, const int& default_value) const;
  const int32_t& GetRefRepeatedInt32(int number, int index) const;
  const int64_t& GetRefRepeatedInt64(int number, int index) const;
  const uint32_t& GetRefRepeatedUInt32(int number, int index) const;
  const uint64_t& GetRefRepeatedUInt64(int number, int index) const;
  const float& GetRefRepeatedFloat(int number, int index) const;
  const double& GetRefRepeatedDouble(int number, int index) const;
  const bool& GetRefRepeatedBool(int number, int index) const;
  const int& GetRefRepeatedEnum(int number, int index) const;

  size_t GetMessageByteSizeLong(int number) const;
  uint8_t* InternalSerializeMessage(int number, const MessageLite* prototype,
                                    uint8_t* target,
                                    io::EpsCopyOutputStream* stream) const;

  // Implementation of _InternalSerialize for non-empty map_.
  uint8_t* _InternalSerializeImpl(const MessageLite* extendee,
                                  int start_field_number, int end_field_number,
                                  uint8_t* target,
                                  io::EpsCopyOutputStream* stream) const;
  // Implementation of _InternalSerializeAll for non-empty map_.
  uint8_t* _InternalSerializeAllImpl(const MessageLite* extendee,
                                     uint8_t* target,
                                     io::EpsCopyOutputStream* stream) const;
  // Implementation of _InternalSerialize for large map_.
  // Extracted as a separate method to avoid inlining and to reuse in
  // _InternalSerializeAllImpl.
  uint8_t* _InternalSerializeImplLarge(const MessageLite* extendee,
                                       int start_field_number,
                                       int end_field_number, uint8_t* target,
                                       io::EpsCopyOutputStream* stream) const;
  // Interface of a lazily parsed singular message extension.
  class PROTOBUF_EXPORT LazyMessageExtension {
   public:
    LazyMessageExtension() = default;
    LazyMessageExtension(const LazyMessageExtension&) = delete;
    LazyMessageExtension& operator=(const LazyMessageExtension&) = delete;
    virtual ~LazyMessageExtension() = default;

    virtual LazyMessageExtension* Clone(Arena* arena,
                                        const LazyMessageExtension& other,
                                        Arena* other_arena) const = 0;
    virtual const MessageLite& GetMessage(const MessageLite& prototype,
                                          Arena* arena) const = 0;
    virtual const MessageLite& GetMessageIgnoreUnparsed(
        const MessageLite& prototype, Arena* arena) const = 0;
    virtual MessageLite* MutableMessage(const MessageLite& prototype,
                                        Arena* arena) = 0;
    virtual void SetAllocatedMessage(MessageLite* message, Arena* arena) = 0;
    virtual void UnsafeArenaSetAllocatedMessage(MessageLite* message,
                                                Arena* arena) = 0;
    [[nodiscard]] virtual MessageLite* ReleaseMessage(
        const MessageLite& prototype, Arena* arena) = 0;
    virtual MessageLite* UnsafeArenaReleaseMessage(const MessageLite& prototype,
                                                   Arena* arena) = 0;

    virtual bool IsInitialized(const MessageLite* prototype,
                               Arena* arena) const = 0;
    virtual bool IsEagerSerializeSafe(const MessageLite* prototype,
                                      Arena* arena) const = 0;

    [[deprecated("Please use ByteSizeLong() instead")]] virtual int ByteSize()
        const {
      return internal::ToIntSize(ByteSizeLong());
    }
    virtual size_t ByteSizeLong() const = 0;
    virtual size_t SpaceUsedLong() const = 0;

    virtual std::variant<size_t, const MessageLite*> UnparsedSizeOrMessage()
        const = 0;

    virtual void MergeFrom(const MessageLite* prototype,
                           const LazyMessageExtension& other, Arena* arena,
                           Arena* other_arena) = 0;
    virtual void MergeFromMessage(const MessageLite& msg, Arena* arena) = 0;
    virtual void Clear() = 0;

    virtual const char* _InternalParse(const MessageLite& prototype,
                                       Arena* arena, const char* ptr,
                                       ParseContext* ctx) = 0;
    virtual uint8_t* WriteMessageToArray(
        const MessageLite* prototype, int number, uint8_t* target,
        io::EpsCopyOutputStream* stream) const = 0;


   private:
    virtual void UnusedKeyMethod();  // Dummy key method to avoid weak vtable.
  };
  // Give access to function defined below to see LazyMessageExtension.
  static LazyMessageExtension* MaybeCreateLazyExtensionImpl(Arena* arena);
  static LazyMessageExtension* MaybeCreateLazyExtension(Arena* arena) {
    auto* f = maybe_create_lazy_extension_.load(std::memory_order_relaxed);
    return f != nullptr ? f(arena) : nullptr;
  }
  static std::atomic<LazyMessageExtension* (*)(Arena* arena)>
      maybe_create_lazy_extension_;

  // We can't directly use std::atomic for Extension::cached_size because
  // Extension needs to be trivially copyable.
  class TrivialAtomicInt {
   public:
    int operator()() const {
      return reinterpret_cast<const AtomicT*>(int_)->load(
          std::memory_order_relaxed);
    }
    void set(int v) {
      reinterpret_cast<AtomicT*>(int_)->store(v, std::memory_order_relaxed);
    }

   private:
    using AtomicT = std::atomic<int>;
    alignas(AtomicT) char int_[sizeof(AtomicT)];
  };

  struct Extension {
    // Some helper methods for operations on a single Extension.
    uint8_t* InternalSerializeFieldWithCachedSizesToArray(
        const MessageLite* extendee, const ExtensionSet* extension_set,
        int number, uint8_t* target, io::EpsCopyOutputStream* stream) const;
    uint8_t* InternalSerializeMessageSetItemWithCachedSizesToArray(
        const MessageLite* extendee, const ExtensionSet* extension_set,
        int number, uint8_t* target, io::EpsCopyOutputStream* stream) const;
    size_t ByteSize(int number) const;
    size_t MessageSetItemByteSize(int number) const;
    void Clear();
    int GetSize() const;
    void Free();
    size_t SpaceUsedExcludingSelfLong() const;
    bool IsInitialized(const ExtensionSet* ext_set, const MessageLite* extendee,
                       int number, Arena* arena) const;
    const void* PrefetchPtr() const {
      ABSL_DCHECK_EQ(is_pointer, is_repeated || FieldTypeIsPointer(type));
      // We don't want to prefetch invalid/null pointers so if there isn't a
      // pointer to prefetch, then return `this`.
      return is_pointer ? absl::bit_cast<const void*>(ptr) : this;
    }

    // The order of these fields packs Extension into 24 bytes when using 8
    // byte alignment. Consider this when adding or removing fields here.

    // We need a separate named union for pointer values to allow for
    // prefetching the pointer without undefined behavior.
    union Pointer {
      std::string* string_value;
      MessageLite* message_value;
      LazyMessageExtension* lazymessage_value;

      RepeatedField<int32_t>* repeated_int32_t_value;
      RepeatedField<int64_t>* repeated_int64_t_value;
      RepeatedField<uint32_t>* repeated_uint32_t_value;
      RepeatedField<uint64_t>* repeated_uint64_t_value;
      RepeatedField<float>* repeated_float_value;
      RepeatedField<double>* repeated_double_value;
      RepeatedField<bool>* repeated_bool_value;
      RepeatedField<int>* repeated_enum_value;
      RepeatedPtrField<std::string>* repeated_string_value;
      RepeatedPtrField<MessageLite>* repeated_message_value;
    };

    union {
      int32_t int32_t_value;
      int64_t int64_t_value;
      uint32_t uint32_t_value;
      uint64_t uint64_t_value;
      float float_value;
      double double_value;
      bool bool_value;
      int enum_value;
      Pointer ptr;
    };

    FieldType type;
    bool is_repeated;

    // Whether the extension is a pointer. This is used for prefetching.
    bool is_pointer : 1;

    // For singular types, indicates if the extension is "cleared".  This
    // happens when an extension is set and then later cleared by the caller.
    // We want to keep the Extension object around for reuse, so instead of
    // removing it from the map, we just set is_cleared = true.
    //
    // This is always set to false for repeated types.
    // The size of the RepeatedField simply becomes zero when cleared.
    bool is_cleared : 1;

    // For singular message types, indicates whether lazy parsing is enabled
    // for this extension. This field is only valid when type == TYPE_MESSAGE
    // and !is_repeated because we only support lazy parsing for singular
    // message types currently. If is_lazy = true, the extension is stored in
    // lazymessage_value. Otherwise, the extension will be message_value.
    bool is_lazy : 1;

    // For repeated types, this indicates if the [packed=true] option is set.
    bool is_packed;

    // For packed fields, the size of the packed data is recorded here when
    // ByteSize() is called then used during serialization.
    mutable TrivialAtomicInt cached_size;

    // The descriptor for this extension, if one exists and is known.  May be
    // nullptr.  Must not be nullptr if the descriptor for the extension does
    // not live in the same pool as the descriptor for the containing type.
    const FieldDescriptor* descriptor;
  };

  // The Extension struct is small enough to be passed by value so we use it
  // directly as the value type in mappings rather than use pointers. We use
  // sorted maps rather than hash-maps because we expect most ExtensionSets will
  // only contain a small number of extensions, and we want AppendToList and
  // deterministic serialization to order fields by field number. In flat mode,
  // the number of elements is small enough that linear search is faster than
  // binary search.

  struct KeyValue {
    int first;
    Extension second;
  };

  using LargeMap = absl::btree_map<int, Extension>;

  // Wrapper API that switches between flat-map and LargeMap.

  // Finds a key (if present) in the ExtensionSet.
  const Extension* FindOrNull(int key) const;
  Extension* FindOrNull(int key);

  // Helper-functions that only inspect the LargeMap.
  const Extension* FindOrNullInLargeMap(int key) const;
  Extension* FindOrNullInLargeMap(int key);

  // Inserts a new (key, Extension) into the ExtensionSet (and returns true), or
  // finds the already-existing Extension for that key (returns false).
  // The Extension* will point to the new-or-found Extension.
  std::pair<Extension*, bool> Insert(int key);
  // Same as insert for the large map.
  std::pair<Extension*, bool> InternalInsertIntoLargeMap(int key);

  // Grows the flat_capacity_.
  // If flat_capacity_ > kMaximumFlatCapacity, converts to LargeMap.
  void GrowCapacity(size_t minimum_new_capacity);

  static constexpr uint16_t kMaximumFlatCapacity = 256;

  // Reserves capacity for the flat_capacity_ when the ExtensionSet is
  // IsCompletelyEmpty.
  // minimum_new_capacity must be <= kMaximumFlatCapacity.
  void InternalReserveSmallCapacityFromEmpty(size_t minimum_new_capacity);

  bool is_large() const { return static_cast<int16_t>(flat_size_) < 0; }

  // Removes a key from the ExtensionSet.
  void Erase(int key);

  // Returns the number of elements in the ExtensionSet, including cleared
  // extensions.
  size_t Size() const {
    return ABSL_PREDICT_FALSE(is_large()) ? map_.large->size() : flat_size_;
  }

  // For use as `PrefetchFunctor`s in `ForEach`.
  struct Prefetch {
    void operator()(const void* ptr) const { absl::PrefetchToLocalCache(ptr); }
  };
  struct PrefetchNta {
    void operator()(const void* ptr) const {
      absl::PrefetchToLocalCacheNta(ptr);
    }
  };

  template <typename Iterator, typename KeyValueFunctor,
            typename PrefetchFunctor>
  static void ForEachPrefetchImpl(Iterator it, Iterator end,
                                  KeyValueFunctor func,
                                  PrefetchFunctor prefetch_func) {
    // Note: based on arena's ChunkList::Cleanup().
    // Prefetch distance 16 performs better than 8 in load tests.
    constexpr int kPrefetchDistance = 16;
    Iterator prefetch = it;
    // Prefetch the first kPrefetchDistance extensions.
    for (int i = 0; prefetch != end && i < kPrefetchDistance; ++prefetch, ++i) {
      prefetch_func(prefetch->second.PrefetchPtr());
    }
    // For the middle extensions, call func and then prefetch the extension
    // kPrefetchDistance after the current one.
    for (; prefetch != end; ++it, ++prefetch) {
      func(it->first, it->second);
      prefetch_func(prefetch->second.PrefetchPtr());
    }
    // Call func on the rest without prefetching.
    for (; it != end; ++it) func(it->first, it->second);
  }

  // Similar to std::for_each, but returning void.
  // Each Iterator is decomposed into ->first and ->second fields, so
  // that the KeyValueFunctor can be agnostic vis-a-vis KeyValue-vs-std::pair.
  // Applies a functor to the <int, Extension&> pairs in sorted order and
  // prefetches ahead.
  template <typename KeyValueFunctor, typename PrefetchFunctor>
  void ForEach(KeyValueFunctor func, PrefetchFunctor prefetch_func) {
    if (ABSL_PREDICT_FALSE(is_large())) {
      ForEachPrefetchImpl(map_.large->begin(), map_.large->end(),
                          std::move(func), std::move(prefetch_func));
      return;
    }
    ForEachPrefetchImpl(flat_begin(), flat_end(), std::move(func),
                        std::move(prefetch_func));
  }
  // As above, but const.
  template <typename KeyValueFunctor, typename PrefetchFunctor>
  void ForEach(KeyValueFunctor func, PrefetchFunctor prefetch_func) const {
    if (ABSL_PREDICT_FALSE(is_large())) {
      ForEachPrefetchImpl(map_.large->begin(), map_.large->end(),
                          std::move(func), std::move(prefetch_func));
      return;
    }
    ForEachPrefetchImpl(flat_begin(), flat_end(), std::move(func),
                        std::move(prefetch_func));
  }

  // As above, but without prefetching. This is for use in cases where we never
  // use the pointed-to extension values in `func`.
  template <typename Iterator, typename KeyValueFunctor>
  static void ForEachNoPrefetch(Iterator begin, Iterator end,
                                KeyValueFunctor func) {
    for (Iterator it = begin; it != end; ++it) func(it->first, it->second);
  }

  // Applies a functor to the <int, Extension&> pairs in sorted order.
  template <typename KeyValueFunctor>
  void ForEachNoPrefetch(KeyValueFunctor func) {
    if (ABSL_PREDICT_FALSE(is_large())) {
      ForEachNoPrefetch(map_.large->begin(), map_.large->end(),
                        std::move(func));
      return;
    }
    ForEachNoPrefetch(flat_begin(), flat_end(), std::move(func));
  }

  // As above, but const.
  template <typename KeyValueFunctor>
  void ForEachNoPrefetch(KeyValueFunctor func) const {
    if (ABSL_PREDICT_FALSE(is_large())) {
      ForEachNoPrefetch(map_.large->begin(), map_.large->end(),
                        std::move(func));
      return;
    }
    ForEachNoPrefetch(flat_begin(), flat_end(), std::move(func));
  }

  // Returns true if nothing is allocated in the ExtensionSet.
  bool IsCompletelyEmpty() const {
    return flat_size_ == 0 && flat_capacity_ == 0;
  }

  // Implementation of MergeFrom into the empty ExtensionSet from a small
  // `other`.
  // This is used in all types of copy.
  // PRECONDITIONs:
  // 1. `this.IsCompletelyEmpty()`.
  // 2. `other` is small (!other.is_large()).
  void InternalMergeFromSmallToEmpty(const MessageLite* extendee,
                                     const ExtensionSet& other);
  // Implementation of MergeFrom for general case.
  void InternalMergeFromSlow(const MessageLite* extendee,
                             const ExtensionSet& other);
  // Merges new or existing Extension from other_extension.
  void InternalExtensionMergeFrom(const MessageLite* extendee, int number,
                                  const Extension& other_extension,
                                  Arena* other_arena);
  // Merges newly created uninitialized Extension from other_extension.
  void InternalExtensionMergeFromIntoUninitializedExtension(
      Extension& dst_extension, const MessageLite* extendee, int number,
      const Extension& other_extension, Arena* other_arena);

  inline static bool is_packable(WireFormatLite::WireType type) {
    switch (type) {
      case WireFormatLite::WIRETYPE_VARINT:
      case WireFormatLite::WIRETYPE_FIXED64:
      case WireFormatLite::WIRETYPE_FIXED32:
        return true;
      case WireFormatLite::WIRETYPE_LENGTH_DELIMITED:
      case WireFormatLite::WIRETYPE_START_GROUP:
      case WireFormatLite::WIRETYPE_END_GROUP:
        return false;

        // Do not add a default statement. Let the compiler complain when
        // someone
        // adds a new wire type.
    }
    Unreachable();  // switch handles all possible enum values
    return false;
  }

  // Returns true and fills field_number and extension if extension is found.
  // Note to support packed repeated field compatibility, it also fills whether
  // the tag on wire is packed, which can be different from
  // extension->is_packed (whether packed=true is specified).
  template <typename ExtensionFinder>
  bool FindExtensionInfoFromTag(uint32_t tag, ExtensionFinder* extension_finder,
                                int* field_number, ExtensionInfo* extension,
                                bool* was_packed_on_wire) {
    *field_number = WireFormatLite::GetTagFieldNumber(tag);
    WireFormatLite::WireType wire_type = WireFormatLite::GetTagWireType(tag);
    return FindExtensionInfoFromFieldNumber(wire_type, *field_number,
                                            extension_finder, extension,
                                            was_packed_on_wire);
  }

  // Returns true and fills extension if extension is found.
  // Note to support packed repeated field compatibility, it also fills whether
  // the tag on wire is packed, which can be different from
  // extension->is_packed (whether packed=true is specified).
  template <typename ExtensionFinder>
  bool FindExtensionInfoFromFieldNumber(int wire_type, int field_number,
                                        ExtensionFinder* extension_finder,
                                        ExtensionInfo* extension,
                                        bool* was_packed_on_wire) const {
    if (!extension_finder->Find(field_number, extension)) {
      return false;
    }

    ABSL_DCHECK(extension->type > 0 &&
                extension->type <= WireFormatLite::MAX_FIELD_TYPE);
    auto real_type = static_cast<WireFormatLite::FieldType>(extension->type);

    WireFormatLite::WireType expected_wire_type =
        WireFormatLite::WireTypeForFieldType(real_type);

    // Check if this is a packed field.
    *was_packed_on_wire = false;
    if (extension->is_repeated &&
        wire_type == WireFormatLite::WIRETYPE_LENGTH_DELIMITED &&
        is_packable(expected_wire_type)) {
      *was_packed_on_wire = true;
      return true;
    }
    // Otherwise the wire type must match.
    return expected_wire_type == wire_type;
  }

  // Find the prototype for a LazyMessage from the extension registry. Returns
  // null if the extension is not found.
  const MessageLite* GetPrototypeForLazyMessage(const MessageLite* extendee,
                                                int number) const;

  // Returns true if extension is present and lazy.
  bool HasLazy(int number) const;

  // Gets the extension with the given number, creating it if it does not
  // already exist.  Returns true if the extension did not already exist.
  bool MaybeNewExtension(int number, const FieldDescriptor* descriptor,
                         Extension** result);

  // Gets the repeated extension for the given descriptor, creating it if
  // it does not exist.
  Extension* MaybeNewRepeatedExtension(const FieldDescriptor* descriptor);

  bool FindExtension(int wire_type, uint32_t field, const MessageLite* extendee,
                     const internal::ParseContext* /*ctx*/,
                     ExtensionInfo* extension, bool* was_packed_on_wire) {
    GeneratedExtensionFinder finder(extendee);
    return FindExtensionInfoFromFieldNumber(wire_type, field, &finder,
                                            extension, was_packed_on_wire);
  }
  inline bool FindExtension(int wire_type, uint32_t field,
                            const Message* extendee,
                            const internal::ParseContext* ctx,
                            ExtensionInfo* extension, bool* was_packed_on_wire);
  // Used for MessageSet only
  const char* ParseFieldMaybeLazily(uint64_t tag, const char* ptr,
                                    const MessageLite* extendee,
                                    internal::InternalMetadata* metadata,
                                    internal::ParseContext* ctx) {
    // Lite MessageSet doesn't implement lazy.
    return ParseField(tag, ptr, extendee, metadata, ctx);
  }
  const char* ParseFieldMaybeLazily(uint64_t tag, const char* ptr,
                                    const Message* extendee,
                                    internal::InternalMetadata* metadata,
                                    internal::ParseContext* ctx);
  const char* ParseMessageSetItem(const char* ptr, const MessageLite* extendee,
                                  internal::InternalMetadata* metadata,
                                  internal::ParseContext* ctx);
  const char* ParseMessageSetItem(const char* ptr, const Message* extendee,
                                  internal::InternalMetadata* metadata,
                                  internal::ParseContext* ctx);

  // Implemented in extension_set_inl.h to keep code out of the header file.
  template <typename T>
  const char* ParseFieldWithExtensionInfo(int number, bool was_packed_on_wire,
                                          const ExtensionInfo& info,
                                          internal::InternalMetadata* metadata,
                                          const char* ptr,
                                          internal::ParseContext* ctx);
  template <typename Msg, typename T>
  const char* ParseMessageSetItemTmpl(const char* ptr, const Msg* extendee,
                                      internal::InternalMetadata* metadata,
                                      internal::ParseContext* ctx);

  // Hack:  RepeatedPtrFieldBase declares ExtensionSet as a friend.  This
  //   friendship should automatically extend to ExtensionSet::Extension, but
  //   unfortunately some older compilers (e.g. GCC 3.4.4) do not implement this
  //   correctly.  So, we must provide helpers for calling methods of that
  //   class.

  // Defined in extension_set_heavy.cc.
  static inline size_t RepeatedMessage_SpaceUsedExcludingSelfLong(
      RepeatedPtrFieldBase* field);

  KeyValue* flat_begin() {
    assert(!is_large());
    return map_.flat;
  }
  const KeyValue* flat_begin() const {
    assert(!is_large());
    return map_.flat;
  }
  KeyValue* flat_end() {
    assert(!is_large());
    return map_.flat + flat_size_;
  }
  const KeyValue* flat_end() const {
    assert(!is_large());
    return map_.flat + flat_size_;
  }

  static KeyValue* AllocateFlatMap(Arena* arena,
                                   uint16_t powerof2_flat_capacity);
  static void DeleteFlatMap(const KeyValue* flat, uint16_t flat_capacity);

  Arena* arena_;

  // Manual memory-management:
  // map_.flat is an allocated array of flat_capacity_ elements.
  // [map_.flat, map_.flat + flat_size_) is the currently-in-use prefix.
  uint16_t flat_capacity_;
  uint16_t flat_size_;  // negative int16_t(flat_size_) indicates is_large()
  union AllocatedData {
    KeyValue* flat;

    // If flat_capacity_ > kMaximumFlatCapacity, switch to LargeMap,
    // which guarantees O(n lg n) CPU but larger constant factors.
    LargeMap* large;
  } map_;
};

constexpr ExtensionSet::ExtensionSet(Arena* arena)
    : arena_(arena), flat_capacity_(0), flat_size_(0), map_{nullptr} {}

// These are just for convenience...
inline void ExtensionSet::SetString(int number, FieldType type,
                                    std::string value,
                                    const FieldDescriptor* descriptor) {
  MutableString(number, type, descriptor)->assign(std::move(value));
}
inline void ExtensionSet::SetRepeatedString(int number, int index,
                                            std::string value) {
  MutableRepeatedString(number, index)->assign(std::move(value));
}
inline void ExtensionSet::AddString(int number, FieldType type,
                                    std::string value,
                                    const FieldDescriptor* descriptor) {
  AddString(number, type, descriptor)->assign(std::move(value));
}
// ===================================================================
// Glue for generated extension accessors

// -------------------------------------------------------------------
// Template magic

// First we have a set of classes representing "type traits" for different
// field types.  A type traits class knows how to implement basic accessors
// for extensions of a particular type given an ExtensionSet.  The signature
// for a type traits class looks like this:
//
//   class TypeTraits {
//    public:
//     typedef ? ConstType;
//     typedef ? MutableType;
//     // TypeTraits for singular fields and repeated fields will define the
//     // symbol "Singular" or "Repeated" respectively. These two symbols will
//     // be used in extension accessors to distinguish between singular
//     // extensions and repeated extensions. If the TypeTraits for the passed
//     // in extension doesn't have the expected symbol defined, it means the
//     // user is passing a repeated extension to a singular accessor, or the
//     // opposite. In that case the C++ compiler will generate an error
//     // message "no matching member function" to inform the user.
//     typedef ? Singular
//     typedef ? Repeated
//
//     static inline ConstType Get(int number, const ExtensionSet& set);
//     static inline void Set(int number, ConstType value, ExtensionSet* set);
//     static inline MutableType Mutable(int number, ExtensionSet* set);
//
//     // Variants for repeated fields.
//     static inline ConstType Get(int number, const ExtensionSet& set,
//                                 int index);
//     static inline void Set(int number, int index,
//                            ConstType value, ExtensionSet* set);
//     static inline MutableType Mutable(int number, int index,
//                                       ExtensionSet* set);
//     static inline void Add(int number, ConstType value, ExtensionSet* set);
//     static inline MutableType Add(int number, ExtensionSet* set);
//     This is used by the ExtensionIdentifier constructor to register
//     the extension at dynamic initialization.
//   };
//
// Not all of these methods make sense for all field types.  For example, the
// "Mutable" methods only make sense for strings and messages, and the
// repeated methods only make sense for repeated types.  So, each type
// traits class implements only the set of methods from this signature that it
// actually supports.  This will cause a compiler error if the user tries to
// access an extension using a method that doesn't make sense for its type.
// For example, if "foo" is an extension of type "optional int32", then if you
// try to write code like:
//   my_message.MutableExtension(foo)
// you will get a compile error because PrimitiveTypeTraits<int32_t> does not
// have a "Mutable()" method.

// -------------------------------------------------------------------
// PrimitiveTypeTraits

// Since the ExtensionSet has different methods for each primitive type,
// we must explicitly define the methods of the type traits class for each
// known type.
template <typename Type>
class PrimitiveTypeTraits {
 public:
  typedef Type ConstType;
  typedef Type MutableType;
  using InitType = ConstType;
  static const ConstType& FromInitType(const InitType& v) { return v; }
  typedef PrimitiveTypeTraits<Type> Singular;
  static constexpr bool kLifetimeBound = false;

  static inline ConstType Get(int number, const ExtensionSet& set,
                              ConstType default_value);

  static inline const ConstType* GetPtr(int number, const ExtensionSet& set,
                                        const ConstType& default_value);
  static inline void Set(int number, FieldType field_type, ConstType value,
                         ExtensionSet* set);
};

template <typename Type>
class RepeatedPrimitiveTypeTraits {
 public:
  typedef Type ConstType;
  typedef Type MutableType;
  using InitType = ConstType;
  static const ConstType& FromInitType(const InitType& v) { return v; }
  typedef RepeatedPrimitiveTypeTraits<Type> Repeated;
  static constexpr bool kLifetimeBound = false;

  typedef RepeatedField<Type> RepeatedFieldType;

  static inline Type Get(int number, const ExtensionSet& set, int index);
  static inline const Type* GetPtr(int number, const ExtensionSet& set,
                                   int index);
  static inline const RepeatedField<ConstType>* GetRepeatedPtr(
      int number, const ExtensionSet& set);
  static inline void Set(int number, int index, Type value, ExtensionSet* set);
  static inline void Add(int number, FieldType field_type, bool is_packed,
                         Type value, ExtensionSet* set);

  static inline const RepeatedField<ConstType>& GetRepeated(
      int number, const ExtensionSet& set);
  static inline RepeatedField<Type>* MutableRepeated(int number,
                                                     FieldType field_type,
                                                     bool is_packed,
                                                     ExtensionSet* set);

  static const RepeatedFieldType* GetDefaultRepeatedField();
};

class PROTOBUF_EXPORT RepeatedPrimitiveDefaults {
 private:
  template <typename Type>
  friend class RepeatedPrimitiveTypeTraits;
  static const RepeatedPrimitiveDefaults* default_instance();
  RepeatedField<int32_t> default_repeated_field_int32_t_;
  RepeatedField<int64_t> default_repeated_field_int64_t_;
  RepeatedField<uint32_t> default_repeated_field_uint32_t_;
  RepeatedField<uint64_t> default_repeated_field_uint64_t_;
  RepeatedField<double> default_repeated_field_double_;
  RepeatedField<float> default_repeated_field_float_;
  RepeatedField<bool> default_repeated_field_bool_;
};

#define PROTOBUF_DEFINE_PRIMITIVE_TYPE(TYPE, METHOD)                           \
  template <>                                                                  \
  inline TYPE PrimitiveTypeTraits<TYPE>::Get(                                  \
      int number, const ExtensionSet& set, TYPE default_value) {               \
    return set.Get##METHOD(number, default_value);                             \
  }                                                                            \
  template <>                                                                  \
  inline const TYPE* PrimitiveTypeTraits<TYPE>::GetPtr(                        \
      int number, const ExtensionSet& set, const TYPE& default_value) {        \
    return &set.GetRef##METHOD(number, default_value);                         \
  }                                                                            \
  template <>                                                                  \
  inline void PrimitiveTypeTraits<TYPE>::Set(int number, FieldType field_type, \
                                             TYPE value, ExtensionSet* set) {  \
    set->Set##METHOD(number, field_type, value, nullptr);                      \
  }                                                                            \
                                                                               \
  template <>                                                                  \
  inline TYPE RepeatedPrimitiveTypeTraits<TYPE>::Get(                          \
      int number, const ExtensionSet& set, int index) {                        \
    return set.GetRepeated##METHOD(number, index);                             \
  }                                                                            \
  template <>                                                                  \
  inline const TYPE* RepeatedPrimitiveTypeTraits<TYPE>::GetPtr(                \
      int number, const ExtensionSet& set, int index) {                        \
    return &set.GetRefRepeated##METHOD(number, index);                         \
  }                                                                            \
  template <>                                                                  \
  inline void RepeatedPrimitiveTypeTraits<TYPE>::Set(                          \
      int number, int index, TYPE value, ExtensionSet* set) {                  \
    set->SetRepeated##METHOD(number, index, value);                            \
  }                                                                            \
  template <>                                                                  \
  inline void RepeatedPrimitiveTypeTraits<TYPE>::Add(                          \
      int number, FieldType field_type, bool is_packed, TYPE value,            \
      ExtensionSet* set) {                                                     \
    set->Add##METHOD(number, field_type, is_packed, value, nullptr);           \
  }                                                                            \
  template <>                                                                  \
  inline const RepeatedField<TYPE>*                                            \
  RepeatedPrimitiveTypeTraits<TYPE>::GetDefaultRepeatedField() {               \
    return &RepeatedPrimitiveDefaults::default_instance()                      \
                ->default_repeated_field_##TYPE##_;                            \
  }                                                                            \
  template <>                                                                  \
  inline const RepeatedField<TYPE>&                                            \
  RepeatedPrimitiveTypeTraits<TYPE>::GetRepeated(int number,                   \
                                                 const ExtensionSet& set) {    \
    return *reinterpret_cast<const RepeatedField<TYPE>*>(                      \
        set.GetRawRepeatedField(number, GetDefaultRepeatedField()));           \
  }                                                                            \
  template <>                                                                  \
  inline const RepeatedField<TYPE>*                                            \
  RepeatedPrimitiveTypeTraits<TYPE>::GetRepeatedPtr(int number,                \
                                                    const ExtensionSet& set) { \
    return &GetRepeated(number, set);                                          \
  }                                                                            \
  template <>                                                                  \
  inline RepeatedField<TYPE>*                                                  \
  RepeatedPrimitiveTypeTraits<TYPE>::MutableRepeated(                          \
      int number, FieldType field_type, bool is_packed, ExtensionSet* set) {   \
    return reinterpret_cast<RepeatedField<TYPE>*>(                             \
        set->MutableRawRepeatedField(number, field_type, is_packed, nullptr)); \
  }

PROTOBUF_DEFINE_PRIMITIVE_TYPE(int32_t, Int32)
PROTOBUF_DEFINE_PRIMITIVE_TYPE(int64_t, Int64)
PROTOBUF_DEFINE_PRIMITIVE_TYPE(uint32_t, UInt32)
PROTOBUF_DEFINE_PRIMITIVE_TYPE(uint64_t, UInt64)
PROTOBUF_DEFINE_PRIMITIVE_TYPE(float, Float)
PROTOBUF_DEFINE_PRIMITIVE_TYPE(double, Double)
PROTOBUF_DEFINE_PRIMITIVE_TYPE(bool, Bool)

#undef PROTOBUF_DEFINE_PRIMITIVE_TYPE

// -------------------------------------------------------------------
// StringTypeTraits

// Strings support both Set() and Mutable().
class PROTOBUF_EXPORT StringTypeTraits {
 public:
  typedef const std::string& ConstType;
  typedef std::string* MutableType;
  using InitType = ConstType;
  static ConstType FromInitType(InitType v) { return v; }
  typedef StringTypeTraits Singular;
  static constexpr bool kLifetimeBound = true;

  static inline const std::string& Get(int number, const ExtensionSet& set,
                                       ConstType default_value) {
    return set.GetString(number, default_value);
  }
  static inline const std::string* GetPtr(int number, const ExtensionSet& set,
                                          ConstType default_value) {
    return &Get(number, set, default_value);
  }
  static inline void Set(int number, FieldType field_type,
                         const std::string& value, ExtensionSet* set) {
    set->SetString(number, field_type, value, nullptr);
  }
  static inline std::string* Mutable(int number, FieldType field_type,
                                     ExtensionSet* set) {
    return set->MutableString(number, field_type, nullptr);
  }
};

class PROTOBUF_EXPORT RepeatedStringTypeTraits {
 public:
  typedef const std::string& ConstType;
  typedef std::string* MutableType;
  using InitType = ConstType;
  static ConstType FromInitType(InitType v) { return v; }
  typedef RepeatedStringTypeTraits Repeated;
  static constexpr bool kLifetimeBound = true;

  typedef RepeatedPtrField<std::string> RepeatedFieldType;

  static inline const std::string& Get(int number, const ExtensionSet& set,
                                       int index) {
    return set.GetRepeatedString(number, index);
  }
  static inline const std::string* GetPtr(int number, const ExtensionSet& set,
                                          int index) {
    return &Get(number, set, index);
  }
  static inline const RepeatedPtrField<std::string>* GetRepeatedPtr(
      int number, const ExtensionSet& set) {
    return &GetRepeated(number, set);
  }
  static inline void Set(int number, int index, const std::string& value,
                         ExtensionSet* set) {
    set->SetRepeatedString(number, index, value);
  }
  static inline std::string* Mutable(int number, int index, ExtensionSet* set) {
    return set->MutableRepeatedString(number, index);
  }
  static inline void Add(int number, FieldType field_type, bool /*is_packed*/,
                         const std::string& value, ExtensionSet* set) {
    set->AddString(number, field_type, value, nullptr);
  }
  static inline std::string* Add(int number, FieldType field_type,
                                 ExtensionSet* set) {
    return set->AddString(number, field_type, nullptr);
  }
  static inline const RepeatedPtrField<std::string>& GetRepeated(
      int number, const ExtensionSet& set) {
    return *reinterpret_cast<const RepeatedPtrField<std::string>*>(
        set.GetRawRepeatedField(number, GetDefaultRepeatedField()));
  }

  static inline RepeatedPtrField<std::string>* MutableRepeated(
      int number, FieldType field_type, bool is_packed, ExtensionSet* set) {
    return reinterpret_cast<RepeatedPtrField<std::string>*>(
        set->MutableRawRepeatedField(number, field_type, is_packed, nullptr));
  }

  static const RepeatedFieldType* GetDefaultRepeatedField();

 private:
  static void InitializeDefaultRepeatedFields();
  static void DestroyDefaultRepeatedFields();
};

// -------------------------------------------------------------------
// EnumTypeTraits

// ExtensionSet represents enums using integers internally, so we have to
// static_cast around.
template <typename Type>
class EnumTypeTraits {
 public:
  typedef Type ConstType;
  typedef Type MutableType;
  using InitType = ConstType;
  static const ConstType& FromInitType(const InitType& v) { return v; }
  typedef EnumTypeTraits<Type> Singular;
  static constexpr bool kLifetimeBound = false;

  static inline ConstType Get(int number, const ExtensionSet& set,
                              ConstType default_value) {
    return static_cast<Type>(set.GetEnum(number, default_value));
  }
  static inline const ConstType* GetPtr(int number, const ExtensionSet& set,
                                        const ConstType& default_value) {
    return reinterpret_cast<const Type*>(
        &set.GetRefEnum(number, default_value));
  }
  static inline void Set(int number, FieldType field_type, ConstType value,
                         ExtensionSet* set) {
    ABSL_DCHECK(
        internal::ValidateEnum(value, EnumTraits<Type>::validation_data()));
    set->SetEnum(number, field_type, value, nullptr);
  }
};

template <typename Type>
class RepeatedEnumTypeTraits {
 public:
  typedef Type ConstType;
  typedef Type MutableType;
  using InitType = ConstType;
  static const ConstType& FromInitType(const InitType& v) { return v; }
  typedef RepeatedEnumTypeTraits<Type> Repeated;
  static constexpr bool kLifetimeBound = false;

  typedef RepeatedField<Type> RepeatedFieldType;

  static inline ConstType Get(int number, const ExtensionSet& set, int index) {
    return static_cast<Type>(set.GetRepeatedEnum(number, index));
  }
  static inline const ConstType* GetPtr(int number, const ExtensionSet& set,
                                        int index) {
    return reinterpret_cast<const Type*>(
        &set.GetRefRepeatedEnum(number, index));
  }
  static inline void Set(int number, int index, ConstType value,
                         ExtensionSet* set) {
    ABSL_DCHECK(
        internal::ValidateEnum(value, EnumTraits<Type>::validation_data()));
    set->SetRepeatedEnum(number, index, value);
  }
  static inline void Add(int number, FieldType field_type, bool is_packed,
                         ConstType value, ExtensionSet* set) {
    ABSL_DCHECK(
        internal::ValidateEnum(value, EnumTraits<Type>::validation_data()));
    set->AddEnum(number, field_type, is_packed, value, nullptr);
  }
  static inline const RepeatedField<Type>& GetRepeated(
      int number, const ExtensionSet& set) {
    // Hack: the `Extension` struct stores a RepeatedField<int> for enums.
    // RepeatedField<int> cannot implicitly convert to RepeatedField<EnumType>
    // so we need to do some casting magic. See message.h for similar
    // contortions for non-extension fields.
    return *reinterpret_cast<const RepeatedField<Type>*>(
        set.GetRawRepeatedField(number, GetDefaultRepeatedField()));
  }
  static inline const RepeatedField<Type>* GetRepeatedPtr(
      int number, const ExtensionSet& set) {
    return &GetRepeated(number, set);
  }
  static inline RepeatedField<Type>* MutableRepeated(int number,
                                                     FieldType field_type,
                                                     bool is_packed,
                                                     ExtensionSet* set) {
    return reinterpret_cast<RepeatedField<Type>*>(
        set->MutableRawRepeatedField(number, field_type, is_packed, nullptr));
  }

  static const RepeatedFieldType* GetDefaultRepeatedField() {
    // Hack: as noted above, repeated enum fields are internally stored as a
    // RepeatedField<int>. We need to be able to instantiate global static
    // objects to return as default (empty) repeated fields on non-existent
    // extensions. We would not be able to know a-priori all of the enum types
    // (values of |Type|) to instantiate all of these, so we just re-use
    // int32_t's default repeated field object.
    return reinterpret_cast<const RepeatedField<Type>*>(
        RepeatedPrimitiveTypeTraits<int32_t>::GetDefaultRepeatedField());
  }
};

// -------------------------------------------------------------------
// MessageTypeTraits

// ExtensionSet guarantees that when manipulating extensions with message
// types, the implementation used will be the compiled-in class representing
// that type.  So, we can static_cast down to the exact type we expect.
template <typename Type>
class MessageTypeTraits {
 public:
  typedef const Type& ConstType;
  typedef Type* MutableType;
  using InitType = const void*;
  static ConstType FromInitType(InitType v) {
    return *static_cast<const Type*>(v);
  }
  typedef MessageTypeTraits<Type> Singular;
  static constexpr bool kLifetimeBound = true;

  static inline ConstType Get(int number, const ExtensionSet& set,
                              ConstType default_value) {
    return static_cast<const Type&>(set.GetMessage(number, default_value));
  }
  static inline std::nullptr_t GetPtr(int /* number */,
                                      const ExtensionSet& /* set */,
                                      ConstType /* default_value */) {
    // Cannot be implemented because of forward declared messages?
    return nullptr;
  }
  static inline MutableType Mutable(int number, FieldType field_type,
                                    ExtensionSet* set) {
    return static_cast<Type*>(set->MutableMessage(
        number, field_type, Type::default_instance(), nullptr));
  }
  static inline void SetAllocated(int number, FieldType field_type,
                                  MutableType message, ExtensionSet* set) {
    set->SetAllocatedMessage(number, field_type, nullptr, message);
  }
  static inline void UnsafeArenaSetAllocated(int number, FieldType field_type,
                                             MutableType message,
                                             ExtensionSet* set) {
    set->UnsafeArenaSetAllocatedMessage(number, field_type, nullptr, message);
  }
  [[nodiscard]] static inline MutableType Release(int number,
                                                  FieldType /* field_type */,
                                                  ExtensionSet* set) {
    return static_cast<Type*>(
        set->ReleaseMessage(number, Type::default_instance()));
  }
  static inline MutableType UnsafeArenaRelease(int number,
                                               FieldType /* field_type */,
                                               ExtensionSet* set) {
    return static_cast<Type*>(
        set->UnsafeArenaReleaseMessage(number, Type::default_instance()));
  }
};

// Used by WireFormatVerify to extract the verify function from the registry.
LazyEagerVerifyFnType FindExtensionLazyEagerVerifyFn(
    const MessageLite* extendee, int number);

// forward declaration.
class RepeatedMessageGenericTypeTraits;

template <typename Type>
class RepeatedMessageTypeTraits {
 public:
  typedef const Type& ConstType;
  typedef Type* MutableType;
  using InitType = const void*;
  static ConstType FromInitType(InitType v) {
    return *static_cast<const Type*>(v);
  }
  typedef RepeatedMessageTypeTraits<Type> Repeated;
  static constexpr bool kLifetimeBound = true;

  typedef RepeatedPtrField<Type> RepeatedFieldType;

  static inline ConstType Get(int number, const ExtensionSet& set, int index) {
    return static_cast<const Type&>(set.GetRepeatedMessage(number, index));
  }
  static inline std::nullptr_t GetPtr(int /* number */,
                                      const ExtensionSet& /* set */,
                                      int /* index */) {
    // Cannot be implemented because of forward declared messages?
    return nullptr;
  }
  static inline std::nullptr_t GetRepeatedPtr(int /* number */,
                                              const ExtensionSet& /* set */) {
    // Cannot be implemented because of forward declared messages?
    return nullptr;
  }
  static inline MutableType Mutable(int number, int index, ExtensionSet* set) {
    return static_cast<Type*>(set->MutableRepeatedMessage(number, index));
  }
  static inline MutableType Add(int number, FieldType field_type,
                                ExtensionSet* set) {
    return static_cast<Type*>(
        set->AddMessage(number, field_type, Type::default_instance(), nullptr));
  }
  static inline const RepeatedPtrField<Type>& GetRepeated(
      int number, const ExtensionSet& set) {
    // See notes above in RepeatedEnumTypeTraits::GetRepeated(): same
    // casting hack applies here, because a RepeatedPtrField<MessageLite>
    // cannot naturally become a RepeatedPtrType<Type> even though Type is
    // presumably a message. google::protobuf::Message goes through similar contortions
    // with a reinterpret_cast<>.
    return *reinterpret_cast<const RepeatedPtrField<Type>*>(
        set.GetRawRepeatedField(number, GetDefaultRepeatedField()));
  }
  static inline RepeatedPtrField<Type>* MutableRepeated(int number,
                                                        FieldType field_type,
                                                        bool is_packed,
                                                        ExtensionSet* set) {
    return reinterpret_cast<RepeatedPtrField<Type>*>(
        set->MutableRawRepeatedField(number, field_type, is_packed, nullptr));
  }

  static const RepeatedFieldType* GetDefaultRepeatedField();
};

template <typename Type>
inline const typename RepeatedMessageTypeTraits<Type>::RepeatedFieldType*
RepeatedMessageTypeTraits<Type>::GetDefaultRepeatedField() {
  static auto instance = OnShutdownDelete(new RepeatedFieldType);
  return instance;
}

// -------------------------------------------------------------------
// ExtensionIdentifier

// This is the type of actual extension objects.  E.g. if you have:
//   extend Foo {
//     optional int32 bar = 1234;
//   }
// then "bar" will be defined in C++ as:
//   ExtensionIdentifier<Foo, PrimitiveTypeTraits<int32_t>, 5, false> bar(1234);
//
// Note that we could, in theory, supply the field number as a template
// parameter, and thus make an instance of ExtensionIdentifier have no
// actual contents.  However, if we did that, then using an extension
// identifier would not necessarily cause the compiler to output any sort
// of reference to any symbol defined in the extension's .pb.o file.  Some
// linkers will actually drop object files that are not explicitly referenced,
// but that would be bad because it would cause this extension to not be
// registered at static initialization, and therefore using it would crash.

template <typename ExtendeeType, typename TypeTraitsType, FieldType field_type,
          bool is_packed>
class ExtensionIdentifier {
 public:
  typedef TypeTraitsType TypeTraits;
  typedef ExtendeeType Extendee;

  constexpr ExtensionIdentifier(int number,
                                typename TypeTraits::InitType default_value)
      : number_(number), default_value_(default_value) {}

  inline int number() const { return number_; }
  typename TypeTraits::ConstType default_value() const {
    return TypeTraits::FromInitType(default_value_);
  }

  typename TypeTraits::ConstType const& default_value_ref() const {
    return TypeTraits::FromInitType(default_value_);
  }

 private:
  const int number_;
  typename TypeTraits::InitType default_value_;
};

// -------------------------------------------------------------------
// Generated accessors


}  // namespace internal

// Call this function to ensure that this extensions's reflection is linked into
// the binary:
//
//   google::protobuf::LinkExtensionReflection(Foo::my_extension);
//
// This will ensure that the following lookup will succeed:
//
//   DescriptorPool::generated_pool()->FindExtensionByName("Foo.my_extension");
//
// This is often relevant for parsing extensions in text mode.
//
// As a side-effect, it will also guarantee that anything else from the same
// .proto file will also be available for lookup in the generated pool.
//
// This function does not actually register the extension, so it does not need
// to be called before the lookup.  However it does need to occur in a function
// that cannot be stripped from the binary (ie. it must be reachable from main).
//
// Best practice is to call this function as close as possible to where the
// reflection is actually needed.  This function is very cheap to call, so you
// should not need to worry about its runtime overhead except in tight loops (on
// x86-64 it compiles into two "mov" instructions).
template <typename ExtendeeType, typename TypeTraitsType,
          internal::FieldType field_type, bool is_packed>
void LinkExtensionReflection(
    const google::protobuf::internal::ExtensionIdentifier<
        ExtendeeType, TypeTraitsType, field_type, is_packed>& extension) {
  internal::StrongReference(extension);
}

// Returns the field descriptor for a generated extension identifier.  This is
// useful when doing reflection over generated extensions.
template <typename ExtendeeType, typename TypeTraitsType,
          internal::FieldType field_type, bool is_packed,
          typename PoolType = DescriptorPool>
const FieldDescriptor* GetExtensionReflection(
    const google::protobuf::internal::ExtensionIdentifier<
        ExtendeeType, TypeTraitsType, field_type, is_packed>& extension) {
  return PoolType::generated_pool()->FindExtensionByNumber(
      google::protobuf::internal::ExtensionIdentifier<ExtendeeType, TypeTraitsType,
                                            field_type,
                                            is_packed>::Extendee::descriptor(),
      extension.number());
}

}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_EXTENSION_SET_H__
