#ifndef GOOGLE_PROTOBUF_REFLECTION_VISIT_FIELD_INFO_H__
#define GOOGLE_PROTOBUF_REFLECTION_VISIT_FIELD_INFO_H__

#include <cstddef>
#include <cstdint>
#include <iterator>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/log/absl_check.h"
#include "absl/strings/cord.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/arenastring.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/extension_set.h"
#include "google/protobuf/inlined_string_field.h"
#include "google/protobuf/map_field.h"
#include "google/protobuf/message.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/port.h"
#include "google/protobuf/wire_format_lite.h"


// clang-format off
#include "google/protobuf/port_def.inc"
// clang-format on

namespace google {
namespace protobuf {
namespace internal {

// A range adaptor for a pair of iterators.
//
// This just wraps two iterators into a range-compatible interface. Nothing
// fancy at all.
template <typename IteratorT>
class iterator_range {
 public:
  using iterator = IteratorT;
  using const_iterator = IteratorT;
  using value_type = typename std::iterator_traits<IteratorT>::value_type;

  iterator_range() : begin_iterator_(), end_iterator_() {}
  iterator_range(IteratorT begin_iterator, IteratorT end_iterator)
      : begin_iterator_(std::move(begin_iterator)),
        end_iterator_(std::move(end_iterator)) {}

  IteratorT begin() const { return begin_iterator_; }
  IteratorT end() const { return end_iterator_; }

  // Returns the size of the wrapped range.  Does not participate in overload
  // resolution for non-random-access iterators, since in those cases this is a
  // slow operation (it must walk the entire range and maintain a count).
  //
  // Users who need to know the "size" of a non-random-access iterator_range
  // should pass the range to `absl::c_distance()` instead.
  template <class It = IteratorT>
  typename std::enable_if<std::is_base_of<std::random_access_iterator_tag,
                                          typename std::iterator_traits<
                                              It>::iterator_category>::value,
                          size_t>::type
  size() const {
    return std::distance(begin_iterator_, end_iterator_);
  }
  // Returns true if this iterator range refers to an empty sequence, and false
  // otherwise.
  bool empty() const { return begin_iterator_ == end_iterator_; }

 private:
  IteratorT begin_iterator_, end_iterator_;
};

#ifdef __cpp_if_constexpr


template <bool is_oneof>
struct DynamicFieldInfoHelper {
  template <typename T>
  static T Get(const Reflection* reflection, const Message& message,
               const FieldDescriptor* field) {
    if constexpr (is_oneof) {
      return reflection->GetRaw<T>(message, field);
    } else {
      return reflection->GetRawNonOneof<T>(message, field);
    }
  }
  template <typename T>
  static T& GetRef(const Reflection* reflection, const Message& message,
                   const FieldDescriptor* field) {
    if constexpr (is_oneof) {
      return reflection->GetRaw<T>(message, field);
    } else {
      return reflection->GetRawNonOneof<T>(message, field);
    }
  }
  template <typename T>
  static T& Mutable(const Reflection* reflection, Message& message,
                    const FieldDescriptor* field) {
    if constexpr (is_oneof) {
      return *reflection->MutableRaw<T>(&message, field);
    } else {
      return *reflection->MutableRawNonOneof<T>(&message, field);
    }
  }

  static void ClearField(const Reflection* reflection, Message& message,
                         const FieldDescriptor* field) {
    if constexpr (is_oneof) {
      reflection->ClearOneofField(&message, field);
    } else {
      reflection->ClearField(&message, field);
    }
  }

  static absl::string_view GetStringView(const Reflection* reflection,
                                         const Message& message,
                                         const FieldDescriptor* field) {
    auto string_type = field->cpp_string_type();
    ABSL_DCHECK(string_type != FieldDescriptor::CppStringType::kCord);
    ABSL_DCHECK(!is_oneof || reflection->HasOneofField(message, field));
    auto str = Get<ArenaStringPtr>(reflection, message, field);
    ABSL_DCHECK(!str.IsDefault());
    return str.Get();
  }
};

struct DynamicExtensionInfoHelper {
  using Extension = ExtensionSet::Extension;

#define PROTOBUF_SINGULAR_PRIMITIVE_METHOD(NAME, FIELD_TYPE, VAR) \
  static FIELD_TYPE Get##NAME(const Extension& ext) {             \
    return ext.VAR##_value;                                       \
  }                                                               \
  static void Set##NAME(Extension& ext, FIELD_TYPE value) {       \
    ext.VAR##_value = value;                                      \
  }                                                               \
  static void Clear##NAME(Extension& ext) { ext.is_cleared = true; }

  PROTOBUF_SINGULAR_PRIMITIVE_METHOD(Int32, int32_t, int32_t);
  PROTOBUF_SINGULAR_PRIMITIVE_METHOD(Int64, int64_t, int64_t);
  PROTOBUF_SINGULAR_PRIMITIVE_METHOD(UInt32, uint32_t, uint32_t);
  PROTOBUF_SINGULAR_PRIMITIVE_METHOD(UInt64, uint64_t, uint64_t);
  PROTOBUF_SINGULAR_PRIMITIVE_METHOD(Float, float, float);
  PROTOBUF_SINGULAR_PRIMITIVE_METHOD(Double, double, double);
  PROTOBUF_SINGULAR_PRIMITIVE_METHOD(Bool, bool, bool);
  PROTOBUF_SINGULAR_PRIMITIVE_METHOD(Enum, int, enum);

#undef PROTOBUF_SINGULAR_PRIMITIVE_METHOD

#define PROTOBUF_REPEATED_FIELD_METHODS(NAME, FIELD_TYPE, VAR)              \
  static const RepeatedField<FIELD_TYPE>* GetRepeated##NAME(                \
      const Extension& ext) {                                               \
    return ext.ptr.repeated_##VAR##_value;                                  \
  }                                                                         \
  static RepeatedField<FIELD_TYPE>* MutableRepeated##NAME(Extension& ext) { \
    return ext.ptr.repeated_##VAR##_value;                                  \
  }                                                                         \
  static void ClearRepeated##NAME(Extension& ext) {                         \
    return ext.ptr.repeated_##VAR##_value->Clear();                         \
  }

  PROTOBUF_REPEATED_FIELD_METHODS(Int32, int32_t, int32_t);
  PROTOBUF_REPEATED_FIELD_METHODS(Int64, int64_t, int64_t);
  PROTOBUF_REPEATED_FIELD_METHODS(UInt32, uint32_t, uint32_t);
  PROTOBUF_REPEATED_FIELD_METHODS(UInt64, uint64_t, uint64_t);
  PROTOBUF_REPEATED_FIELD_METHODS(Float, float, float);
  PROTOBUF_REPEATED_FIELD_METHODS(Double, double, double);
  PROTOBUF_REPEATED_FIELD_METHODS(Bool, bool, bool);
  PROTOBUF_REPEATED_FIELD_METHODS(Enum, int, enum);

#undef PROTOBUF_REPEATED_FIELD_METHODS

#define PROTOBUF_REPEATED_PTR_FIELD_METHODS(FIELD_TYPE, NAME, VAR)             \
  static const RepeatedPtrField<FIELD_TYPE>* GetRepeated##NAME(                \
      const Extension& ext) {                                                  \
    return ext.ptr.repeated_##VAR##_value;                                     \
  }                                                                            \
  static RepeatedPtrField<FIELD_TYPE>* MutableRepeated##NAME(Extension& ext) { \
    return ext.ptr.repeated_##VAR##_value;                                     \
  }                                                                            \
  static void ClearRepeated##NAME(Extension& ext) {                            \
    return ext.ptr.repeated_##VAR##_value->Clear();                            \
  }

  PROTOBUF_REPEATED_PTR_FIELD_METHODS(std::string, String, string);
  PROTOBUF_REPEATED_PTR_FIELD_METHODS(MessageLite, Message, message);

#undef PROTOBUF_REPEATED_PTR_FIELD_METHODS

  static absl::string_view GetStringView(const Extension& ext) {
    return *ext.ptr.string_value;
  }
  static void SetStringView(Extension& ext, absl::string_view value) {
    ext.ptr.string_value->assign(value.data(), value.size());
  }
  static void ClearStringView(Extension& ext) {
    ext.is_cleared = true;
    ext.ptr.string_value->clear();
  }

  static const Message& GetMessage(const Extension& ext) {
    return DownCastMessage<Message>(*ext.ptr.message_value);
  }
  static Message& MutableMessage(Extension& ext) {
    return DownCastMessage<Message>(*ext.ptr.message_value);
  }
  static void ClearMessage(Extension& ext) {
    ext.is_cleared = true;
    ext.ptr.message_value->Clear();
  }

  static const Message& GetLazyMessage(const Extension& ext,
                                       const Message& prototype, Arena* arena) {
    return DownCastMessage<Message>(
        ext.ptr.lazymessage_value->GetMessage(prototype, arena));
  }
  static const Message& GetLazyMessageIgnoreUnparsed(const Extension& ext,
                                                     const Message& prototype,
                                                     Arena* arena) {
    return DownCastMessage<Message>(
        ext.ptr.lazymessage_value->GetMessageIgnoreUnparsed(prototype, arena));
  }
  static Message& MutableLazyMessage(Extension& ext, const Message& prototype,
                                     Arena* arena) {
    return DownCastMessage<Message>(
        *ext.ptr.lazymessage_value->MutableMessage(prototype, arena));
  }
  static void ClearLazyMessage(Extension& ext) {
    ext.is_cleared = true;
    return ext.ptr.lazymessage_value->Clear();
  }
  static size_t ByteSizeLongLazyMessage(const Extension& ext) {
    return ext.ptr.lazymessage_value->ByteSizeLong();
  }
};

////////////////////////////////////////////////////////////////////////
// Primitive fields
////////////////////////////////////////////////////////////////////////
template <typename MessageT, typename FieldT,
          FieldDescriptor::CppType cpp_type_parameter, bool is_oneof_parameter>
struct SingularPrimitive {
  constexpr SingularPrimitive(const Reflection* r, MessageT& m,
                              const FieldDescriptor* f)
      : reflection(r), message(m), field(f) {}

  int number() const { return field->number(); }
  FieldDescriptor::Type type() const { return field->type(); }

  FieldT Get() const {
    return DynamicFieldInfoHelper<is_oneof>::template Get<FieldT>(
        reflection, message, field);
  }
  void Set(FieldT value) {
    DynamicFieldInfoHelper<is_oneof>::template Mutable<FieldT>(
        reflection, message, field) = value;
  }
  void Clear() {
    DynamicFieldInfoHelper<is_oneof>::ClearField(reflection, message, field);
  }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      cpp_type_parameter;
  static constexpr bool is_repeated = false;            // NOLINT
  static constexpr bool is_map = false;                 // NOLINT
  static constexpr bool is_extension = false;           // NOLINT
  static constexpr bool is_oneof = is_oneof_parameter;  // NOLINT

  const Reflection* reflection;
  MessageT& message;
  const FieldDescriptor* field;
};

#define PROTOBUF_DYN_FIELD_INFO_VARINT(NAME, CPPTYPE, FIELD_TYPE)         \
  template <typename MessageT, bool is_oneof>                             \
  struct NAME##DynamicFieldInfo                                           \
      : SingularPrimitive<MessageT, FIELD_TYPE,                           \
                          FieldDescriptor::CPPTYPE_##CPPTYPE, is_oneof> { \
    using BaseT =                                                         \
        SingularPrimitive<MessageT, FIELD_TYPE,                           \
                          FieldDescriptor::CPPTYPE_##CPPTYPE, is_oneof>;  \
                                                                          \
    constexpr NAME##DynamicFieldInfo(const Reflection* r, MessageT& m,    \
                                     const FieldDescriptor* f)            \
        : BaseT(r, m, f) {}                                               \
    size_t FieldByteSize() const {                                        \
      return WireFormatLite::NAME##Size(BaseT::Get());                    \
    }                                                                     \
  };

#define PROTOBUF_DYN_FIELD_INFO_FIXED(NAME, CPPTYPE, FIELD_TYPE)          \
  template <typename MessageT, bool is_oneof>                             \
  struct NAME##DynamicFieldInfo                                           \
      : SingularPrimitive<MessageT, FIELD_TYPE,                           \
                          FieldDescriptor::CPPTYPE_##CPPTYPE, is_oneof> { \
    using BaseT =                                                         \
        SingularPrimitive<MessageT, FIELD_TYPE,                           \
                          FieldDescriptor::CPPTYPE_##CPPTYPE, is_oneof>;  \
                                                                          \
    constexpr NAME##DynamicFieldInfo(const Reflection* r, MessageT& m,    \
                                     const FieldDescriptor* f)            \
        : BaseT(r, m, f) {}                                               \
    constexpr size_t FieldByteSize() const {                              \
      return WireFormatLite::k##NAME##Size;                               \
    }                                                                     \
  };

PROTOBUF_DYN_FIELD_INFO_VARINT(Int32, INT32, int32_t);
PROTOBUF_DYN_FIELD_INFO_VARINT(Int64, INT64, int64_t);
PROTOBUF_DYN_FIELD_INFO_VARINT(UInt32, UINT32, uint32_t);
PROTOBUF_DYN_FIELD_INFO_VARINT(UInt64, UINT64, uint64_t);
PROTOBUF_DYN_FIELD_INFO_VARINT(SInt32, INT32, int32_t);
PROTOBUF_DYN_FIELD_INFO_VARINT(SInt64, INT64, int64_t);

PROTOBUF_DYN_FIELD_INFO_FIXED(Fixed32, UINT32, uint32_t);
PROTOBUF_DYN_FIELD_INFO_FIXED(Fixed64, UINT64, uint64_t);
PROTOBUF_DYN_FIELD_INFO_FIXED(SFixed32, INT32, int32_t);
PROTOBUF_DYN_FIELD_INFO_FIXED(SFixed64, INT64, int64_t);
PROTOBUF_DYN_FIELD_INFO_FIXED(Double, DOUBLE, double);
PROTOBUF_DYN_FIELD_INFO_FIXED(Float, FLOAT, float);
PROTOBUF_DYN_FIELD_INFO_FIXED(Bool, BOOL, bool);

#undef PROTOBUF_DYN_FIELD_INFO_VARINT
#undef PROTOBUF_DYN_FIELD_INFO_FIXED

////////////////////////////////////////////////////////////////////////
// Extension primitive fields
////////////////////////////////////////////////////////////////////////
#define PROTOBUF_DYN_EXTENSION_INFO_VARINT(NAME, CPPNAME, CPPTYPE, FIELD_TYPE) \
  template <typename ExtensionT>                                               \
  struct NAME##DynamicExtensionInfo {                                          \
    constexpr NAME##DynamicExtensionInfo(ExtensionT& e, int n)                 \
        : ext(e), ext_number(n) {}                                             \
    int number() const { return ext_number; }                                  \
    FieldDescriptor::Type type() const {                                       \
      return static_cast<FieldDescriptor::Type>(ext.type);                     \
    }                                                                          \
    FIELD_TYPE Get() const {                                                   \
      return DynamicExtensionInfoHelper::Get##CPPNAME(ext);                    \
    }                                                                          \
    void Set(FIELD_TYPE value) {                                               \
      DynamicExtensionInfoHelper::Set##CPPNAME(ext, value);                    \
    }                                                                          \
    void Clear() { DynamicExtensionInfoHelper::Clear##CPPNAME(ext); }          \
    size_t FieldByteSize() const { return WireFormatLite::NAME##Size(Get()); } \
                                                                               \
    static constexpr FieldDescriptor::CppType cpp_type =                       \
        FieldDescriptor::CPPTYPE_##CPPTYPE;                                    \
    static constexpr bool is_repeated = false;                                 \
    static constexpr bool is_map = false;                                      \
    static constexpr bool is_extension = true;                                 \
    static constexpr bool is_oneof = false;                                    \
                                                                               \
    ExtensionT& ext;                                                           \
    int ext_number;                                                            \
  };

#define PROTOBUF_DYN_EXTENSION_INFO_FIXED(NAME, CPPNAME, CPPTYPE, FIELD_TYPE) \
  template <typename ExtensionT>                                              \
  struct NAME##DynamicExtensionInfo {                                         \
    constexpr NAME##DynamicExtensionInfo(ExtensionT& e, int n)                \
        : ext(e), ext_number(n) {}                                            \
    int number() const { return ext_number; }                                 \
    FieldDescriptor::Type type() const {                                      \
      return static_cast<FieldDescriptor::Type>(ext.type);                    \
    }                                                                         \
    FIELD_TYPE Get() const {                                                  \
      return DynamicExtensionInfoHelper::Get##CPPNAME(ext);                   \
    }                                                                         \
    void Set(FIELD_TYPE value) {                                              \
      DynamicExtensionInfoHelper::Set##CPPNAME(ext, value);                   \
    }                                                                         \
    void Clear() { DynamicExtensionInfoHelper::Clear##CPPNAME(ext); }         \
    constexpr size_t FieldByteSize() const {                                  \
      return WireFormatLite::k##NAME##Size;                                   \
    }                                                                         \
                                                                              \
    static constexpr FieldDescriptor::CppType cpp_type =                      \
        FieldDescriptor::CPPTYPE_##CPPTYPE;                                   \
    static constexpr bool is_repeated = false;                                \
    static constexpr bool is_map = false;                                     \
    static constexpr bool is_extension = true;                                \
    static constexpr bool is_oneof = false;                                   \
                                                                              \
    ExtensionT& ext;                                                          \
    int ext_number;                                                           \
  };

PROTOBUF_DYN_EXTENSION_INFO_VARINT(Int32, Int32, INT32, int32_t);
PROTOBUF_DYN_EXTENSION_INFO_VARINT(Int64, Int64, INT64, int64_t);
PROTOBUF_DYN_EXTENSION_INFO_VARINT(UInt32, UInt32, UINT32, uint32_t);
PROTOBUF_DYN_EXTENSION_INFO_VARINT(UInt64, UInt64, UINT64, uint64_t);
PROTOBUF_DYN_EXTENSION_INFO_VARINT(SInt32, Int32, INT32, int32_t);
PROTOBUF_DYN_EXTENSION_INFO_VARINT(SInt64, Int64, INT64, int64_t);
PROTOBUF_DYN_EXTENSION_INFO_VARINT(Enum, Enum, ENUM, int);

PROTOBUF_DYN_EXTENSION_INFO_FIXED(Fixed32, UInt32, UINT32, uint32_t);
PROTOBUF_DYN_EXTENSION_INFO_FIXED(Fixed64, UInt64, UINT64, uint64_t);
PROTOBUF_DYN_EXTENSION_INFO_FIXED(SFixed32, Int32, INT32, int32_t);
PROTOBUF_DYN_EXTENSION_INFO_FIXED(SFixed64, Int64, INT64, int64_t);
PROTOBUF_DYN_EXTENSION_INFO_FIXED(Double, Double, DOUBLE, double);
PROTOBUF_DYN_EXTENSION_INFO_FIXED(Float, Float, FLOAT, float);
PROTOBUF_DYN_EXTENSION_INFO_FIXED(Bool, Bool, BOOL, bool);

#undef PROTOBUF_DYN_EXTENSION_INFO_VARINT
#undef PROTOBUF_DYN_EXTENSION_INFO_FIXED

////////////////////////////////////////////////////////////////////////
// Enum fields (to handle closed enums).
////////////////////////////////////////////////////////////////////////

template <typename MessageT, bool is_oneof_parameter>
struct EnumDynamicFieldInfo {
  constexpr EnumDynamicFieldInfo(const Reflection* r, MessageT& m,
                                 const FieldDescriptor* f)
      : reflection(r), message(m), field(f) {}

  int number() const { return field->number(); }
  FieldDescriptor::Type type() const { return field->type(); }

  int Get() const {
    if constexpr (is_oneof) {
      return reflection->GetEnumValue(message, field);
    } else {
      return DynamicFieldInfoHelper<false>::Get<int>(reflection, message,
                                                     field);
    }
  }
  void Set(int value) { reflection->SetEnumValue(&message, field, value); }
  void Clear() {
    DynamicFieldInfoHelper<is_oneof>::ClearField(reflection, message, field);
  }
  size_t FieldByteSize() const { return WireFormatLite::EnumSize(Get()); }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_ENUM;
  static constexpr bool is_repeated = false;            // NOLINT
  static constexpr bool is_map = false;                 // NOLINT
  static constexpr bool is_extension = false;           // NOLINT
  static constexpr bool is_oneof = is_oneof_parameter;  // NOLINT

  const Reflection* reflection;
  MessageT& message;
  const FieldDescriptor* field;
};

////////////////////////////////////////////////////////////////////////
// String fields
////////////////////////////////////////////////////////////////////////
template <typename MessageT, bool is_oneof_parameter>
struct StringDynamicFieldInfo {
  constexpr StringDynamicFieldInfo(const Reflection* r, MessageT& m,
                                   const FieldDescriptor* f)
      : reflection(r), message(m), field(f) {}

  int number() const { return field->number(); }
  FieldDescriptor::Type type() const { return field->type(); }

  absl::string_view Get() const {
    return DynamicFieldInfoHelper<is_oneof>::GetStringView(reflection, message,
                                                           field);
  }
  void Set(std::string value) {
    reflection->SetString(&message, field, std::move(value));
  }
  void Clear() {
    DynamicFieldInfoHelper<is_oneof>::ClearField(reflection, message, field);
  }
  size_t FieldByteSize() const { return WireFormatLite::StringSize(Get()); }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_STRING;
  static constexpr bool is_repeated = false;            // NOLINT
  static constexpr bool is_map = false;                 // NOLINT
  static constexpr bool is_extension = false;           // NOLINT
  static constexpr bool is_oneof = is_oneof_parameter;  // NOLINT
  static constexpr bool is_cord = false;                // NOLINT

  const Reflection* reflection;
  MessageT& message;
  const FieldDescriptor* field;
};

////////////////////////////////////////////////////////////////////////
// Extension string fields
////////////////////////////////////////////////////////////////////////
template <typename ExtensionT>
struct StringDynamicExtensionInfo {
  constexpr StringDynamicExtensionInfo(ExtensionT& e, int n)
      : ext(e), ext_number(n) {}

  int number() const { return ext_number; }
  FieldDescriptor::Type type() const {
    return static_cast<FieldDescriptor::Type>(ext.type);
  }

  absl::string_view Get() const {
    return DynamicExtensionInfoHelper::GetStringView(ext);
  }
  void Set(absl::string_view value) {
    DynamicExtensionInfoHelper::SetStringView(ext, value);
  }
  void Clear() { DynamicExtensionInfoHelper::ClearStringView(ext); }
  size_t FieldByteSize() const { return WireFormatLite::StringSize(Get()); }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_STRING;
  static constexpr bool is_repeated = false;  // NOLINT
  static constexpr bool is_map = false;       // NOLINT
  static constexpr bool is_extension = true;  // NOLINT
  static constexpr bool is_oneof = false;     // NOLINT
  static constexpr bool is_cord = false;      // NOLINT

  ExtensionT& ext;
  int ext_number;
};

////////////////////////////////////////////////////////////////////////
// Cord fields
////////////////////////////////////////////////////////////////////////
template <typename MessageT, bool is_oneof_parameter>
struct CordDynamicFieldInfo {
  constexpr CordDynamicFieldInfo(const Reflection* r, MessageT& m,
                                 const FieldDescriptor* f)
      : reflection(r), message(m), field(f) {}

  int number() const { return field->number(); }
  FieldDescriptor::Type type() const { return field->type(); }

  ::absl::Cord Get() const { return reflection->GetCord(message, field); }
  void Set(const ::absl::Cord& value) {
    reflection->SetString(&message, field, value);
  }
  void Clear() {
    DynamicFieldInfoHelper<is_oneof>::ClearField(reflection, message, field);
  }
  size_t FieldByteSize() const { return WireFormatLite::StringSize(Get()); }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_STRING;
  static constexpr bool is_repeated = false;            // NOLINT
  static constexpr bool is_map = false;                 // NOLINT
  static constexpr bool is_extension = false;           // NOLINT
  static constexpr bool is_oneof = is_oneof_parameter;  // NOLINT
  static constexpr bool is_cord = true;                 // NOLINT

  const Reflection* reflection;
  MessageT& message;
  const FieldDescriptor* field;
};

////////////////////////////////////////////////////////////////////////
// Message fields
////////////////////////////////////////////////////////////////////////
template <typename MessageT, bool is_oneof_parameter>
struct MessageDynamicFieldInfo {
  constexpr MessageDynamicFieldInfo(const Reflection* r, MessageT& m,
                                    const FieldDescriptor* f)
      : reflection(r), message(m), field(f) {}

  int number() const { return field->number(); }
  FieldDescriptor::Type type() const { return field->type(); }
  const Message& Get(MessageFactory* factory = nullptr) {
    return reflection->GetMessage(message, field, factory);
  }
  Message& Mutable(MessageFactory* factory = nullptr) {
    return *reflection->MutableMessage(&message, field, factory);
  }
  void Clear() { reflection->ClearField(&message, field); }
  size_t FieldByteSize(MessageFactory* factory = nullptr) {
    return Get(factory).ByteSizeLong();
  }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_MESSAGE;
  static constexpr bool is_repeated = false;            // NOLINT
  static constexpr bool is_map = false;                 // NOLINT
  static constexpr bool is_extension = false;           // NOLINT
  static constexpr bool is_oneof = is_oneof_parameter;  // NOLINT
  static constexpr bool is_lazy = false;                // NOLINT

  const Reflection* reflection;
  MessageT& message;
  const FieldDescriptor* field;
};


////////////////////////////////////////////////////////////////////////
// Extension message fields
////////////////////////////////////////////////////////////////////////
struct SingularMessageDynamicExtensionInfo {
  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_MESSAGE;
  static constexpr bool is_repeated = false;  // NOLINT
  static constexpr bool is_map = false;       // NOLINT
  static constexpr bool is_extension = true;  // NOLINT
  static constexpr bool is_oneof = false;     // NOLINT
  static constexpr bool is_lazy = false;      // NOLINT
};

template <typename ExtensionT>
struct MessageDynamicExtensionInfo : SingularMessageDynamicExtensionInfo {
  constexpr MessageDynamicExtensionInfo(ExtensionT& e, int n, bool mset)
      : ext(e), ext_number(n), is_message_set(mset) {}

  int number() const { return ext_number; }
  FieldDescriptor::Type type() const {
    return static_cast<FieldDescriptor::Type>(ext.type);
  }
  const Message& Get() const {
    return DynamicExtensionInfoHelper::GetMessage(ext);
  }
  Message& Mutable() { return DynamicExtensionInfoHelper::MutableMessage(ext); }
  void Clear() { DynamicExtensionInfoHelper::ClearMessage(ext); }
  size_t FieldByteSize() const {
    return DynamicExtensionInfoHelper::GetMessage(ext).ByteSizeLong();
  }

  ExtensionT& ext;
  int ext_number;
  bool is_message_set;
};

template <typename ExtensionT>
struct GroupDynamicExtensionInfo : SingularMessageDynamicExtensionInfo {
  constexpr GroupDynamicExtensionInfo(ExtensionT& e, int n)
      : ext(e), ext_number(n), is_message_set(false) {}

  int number() const { return ext_number; }
  FieldDescriptor::Type type() const {
    return static_cast<FieldDescriptor::Type>(ext.type);
  }
  const Message& Get() const {
    return DynamicExtensionInfoHelper::GetMessage(ext);
  }
  Message& Mutable() { return DynamicExtensionInfoHelper::MutableMessage(ext); }
  void Clear() { DynamicExtensionInfoHelper::ClearMessage(ext); }
  size_t FieldByteSize() const {
    return DynamicExtensionInfoHelper::GetMessage(ext).ByteSizeLong();
  }

  ExtensionT& ext;
  int ext_number;
  bool is_message_set;
};

struct SingularLazyMessageDynamicExtensionInfo {
  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_MESSAGE;
  static constexpr bool is_repeated = false;  // NOLINT
  static constexpr bool is_map = false;       // NOLINT
  static constexpr bool is_extension = true;  // NOLINT
  static constexpr bool is_oneof = false;     // NOLINT
  static constexpr bool is_lazy = true;       // NOLINT
};

template <typename ExtensionT>
struct LazyMessageDynamicExtensionInfo
    : SingularLazyMessageDynamicExtensionInfo {
  LazyMessageDynamicExtensionInfo(ExtensionT& e, int n, bool mset,
                                  const Message& p, Arena* a)
      : ext(e), ext_number(n), is_message_set(mset), prototype(p), arena(a) {}

  int number() const { return ext_number; }
  FieldDescriptor::Type type() const {
    return static_cast<FieldDescriptor::Type>(ext.type);
  }
  const Message& Get() const {
    return DynamicExtensionInfoHelper::GetLazyMessage(ext, prototype, arena);
  }
  const Message& GetIgnoreUnparsed() const {
    return DynamicExtensionInfoHelper::GetLazyMessageIgnoreUnparsed(
        ext, prototype, arena);
  }
  Message& Mutable() {
    return DynamicExtensionInfoHelper::MutableLazyMessage(ext, prototype,
                                                          arena);
  }
  void Clear() { DynamicExtensionInfoHelper::ClearLazyMessage(ext); }
  size_t FieldByteSize() const {
    return DynamicExtensionInfoHelper::ByteSizeLongLazyMessage(ext);
  }

  ExtensionT& ext;
  int ext_number;
  bool is_message_set;
  const Message& prototype;
  Arena* arena;
};

////////////////////////////////////////////////////////////////////////
// Repeated fields
////////////////////////////////////////////////////////////////////////
template <typename MessageT, typename FieldT>
struct RepeatedEntityDynamicFieldInfoBase {
  constexpr RepeatedEntityDynamicFieldInfoBase(const Reflection* r, MessageT& m,
                                               const FieldDescriptor* f,
                                               const RepeatedField<FieldT>& rep)
      : reflection(r), message(m), field(f), const_repeated(rep) {}

  int number() const { return field->number(); }
  FieldDescriptor::Type type() const { return field->type(); }
  bool is_packed() const { return field->is_packed(); }

  int size() const { return const_repeated.size(); }
  iterator_range<typename RepeatedField<FieldT>::const_iterator> Get() const {
    return {const_repeated.cbegin(), const_repeated.cend()};
  }
  iterator_range<typename RepeatedField<FieldT>::iterator> Mutable() {
    auto& rep =
        *reflection->MutableRepeatedFieldInternal<FieldT>(&message, field);
    return {rep.begin(), rep.end()};
  }
  void Clear() {
    reflection->MutableRepeatedFieldInternal<FieldT>(&message, field)->Clear();
  }

  const Reflection* reflection;
  MessageT& message;
  const FieldDescriptor* field;
  const RepeatedField<FieldT>& const_repeated;
};

#define PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT(NAME, CPPTYPE, FIELD_TYPE)  \
  template <typename MessageT>                                              \
  struct Repeated##NAME##DynamicFieldInfo                                   \
      : RepeatedEntityDynamicFieldInfoBase<MessageT, FIELD_TYPE> {          \
    using BaseT = RepeatedEntityDynamicFieldInfoBase<MessageT, FIELD_TYPE>; \
                                                                            \
    constexpr Repeated##NAME##DynamicFieldInfo(                             \
        const Reflection* r, MessageT& m, const FieldDescriptor* f,         \
        const RepeatedField<FIELD_TYPE>& rep)                               \
        : BaseT(r, m, f, rep) {}                                            \
                                                                            \
    size_t FieldByteSize() const {                                          \
      size_t byte_size = 0;                                                 \
      for (auto it : BaseT::const_repeated) {                               \
        byte_size += WireFormatLite::NAME##Size(it);                        \
      }                                                                     \
      return byte_size;                                                     \
    }                                                                       \
                                                                            \
    static constexpr FieldDescriptor::CppType cpp_type =                    \
        FieldDescriptor::CPPTYPE_##CPPTYPE;                                 \
    static constexpr bool is_repeated = true;                               \
    static constexpr bool is_map = false;                                   \
    static constexpr bool is_extension = false;                             \
    static constexpr bool is_oneof = false;                                 \
  };

#define PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED(NAME, CPPTYPE, FIELD_TYPE)   \
  template <typename MessageT>                                              \
  struct Repeated##NAME##DynamicFieldInfo                                   \
      : RepeatedEntityDynamicFieldInfoBase<MessageT, FIELD_TYPE> {          \
    using BaseT = RepeatedEntityDynamicFieldInfoBase<MessageT, FIELD_TYPE>; \
                                                                            \
    constexpr Repeated##NAME##DynamicFieldInfo(                             \
        const Reflection* r, MessageT& m, const FieldDescriptor* f,         \
        const RepeatedField<FIELD_TYPE>& rep)                               \
        : BaseT(r, m, f, rep) {}                                            \
                                                                            \
    size_t FieldByteSize() const {                                          \
      return static_cast<size_t>(BaseT::size()) *                           \
             WireFormatLite::k##NAME##Size;                                 \
    }                                                                       \
                                                                            \
    static constexpr FieldDescriptor::CppType cpp_type =                    \
        FieldDescriptor::CPPTYPE_##CPPTYPE;                                 \
    static constexpr bool is_repeated = true;                               \
    static constexpr bool is_map = false;                                   \
    static constexpr bool is_extension = false;                             \
    static constexpr bool is_oneof = false;                                 \
  };

PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT(Int32, INT32, int32_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT(Int64, INT64, int64_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT(UInt32, UINT32, uint32_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT(UInt64, UINT64, uint64_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT(SInt32, INT32, int32_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT(SInt64, INT64, int64_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT(Enum, ENUM, int);

PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED(Fixed32, UINT32, uint32_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED(Fixed64, UINT64, uint64_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED(SFixed32, INT32, int32_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED(SFixed64, INT64, int64_t);
PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED(Double, DOUBLE, double);
PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED(Float, FLOAT, float);
PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED(Bool, BOOL, bool);

#undef PROTOBUF_DYN_FIELD_INFO_REPEATED_VARINT
#undef PROTOBUF_DYN_FIELD_INFO_REPEATED_FIXED

template <typename MessageT, typename FieldT>
struct RepeatedPtrEntityDynamicFieldInfoBase {
  constexpr RepeatedPtrEntityDynamicFieldInfoBase(
      const Reflection* r, MessageT& m, const FieldDescriptor* f,
      const RepeatedPtrField<FieldT>& rep)
      : reflection(r), message(m), field(f), const_repeated(rep) {}

  int number() const { return field->number(); }
  FieldDescriptor::Type type() const { return field->type(); }
  bool is_packed() const { return field->is_packed(); }

  int size() const { return const_repeated.size(); }
  iterator_range<typename RepeatedPtrField<FieldT>::const_iterator> Get()
      const {
    return {const_repeated.cbegin(), const_repeated.cend()};
  }
  iterator_range<typename RepeatedPtrField<FieldT>::iterator> Mutable() {
    auto& rep =
        *reflection->MutableRepeatedPtrFieldInternal<FieldT>(&message, field);
    return {rep.begin(), rep.end()};
  }
  void Clear() {
    reflection->MutableRepeatedPtrFieldInternal<FieldT>(&message, field)
        ->Clear();
  }

  const Reflection* reflection;
  MessageT& message;
  const FieldDescriptor* field;
  const RepeatedPtrField<FieldT>& const_repeated;
};

template <typename MessageT, typename FieldT>
struct RepeatedStringDynamicFieldInfoBase
    : RepeatedPtrEntityDynamicFieldInfoBase<MessageT, FieldT> {
  using BaseT = RepeatedPtrEntityDynamicFieldInfoBase<MessageT, FieldT>;

  constexpr RepeatedStringDynamicFieldInfoBase(
      const Reflection* r, MessageT& m, const FieldDescriptor* f,
      const RepeatedPtrField<FieldT>& rep)
      : BaseT(r, m, f, rep) {}
  size_t FieldByteSize() const {
    size_t byte_size = 0;
    for (auto& it : BaseT::const_repeated) {
      byte_size += WireFormatLite::LengthDelimitedSize(it.size());
    }
    return byte_size;
  }
};

template <typename MessageT>
struct RepeatedStringDynamicFieldInfo
    : RepeatedStringDynamicFieldInfoBase<MessageT, std::string> {
  constexpr RepeatedStringDynamicFieldInfo(
      const Reflection* r, MessageT& m, const FieldDescriptor* f,
      const RepeatedPtrField<std::string>& rep)
      : RepeatedStringDynamicFieldInfoBase<MessageT, std::string>(r, m, f,
                                                                  rep) {}

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_STRING;
  static constexpr bool is_repeated = true;       // NOLINT
  static constexpr bool is_map = false;           // NOLINT
  static constexpr bool is_extension = false;     // NOLINT
  static constexpr bool is_oneof = false;         // NOLINT
  static constexpr bool is_cord = false;          // NOLINT
  static constexpr bool is_string_piece = false;  // NOLINT
};


struct RepeatedMessageDynamicFieldInfoMeta {
  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_MESSAGE;
  static constexpr bool is_repeated = true;    // NOLINT
  static constexpr bool is_map = false;        // NOLINT
  static constexpr bool is_extension = false;  // NOLINT
  static constexpr bool is_oneof = false;      // NOLINT
};

template <typename MessageT>
struct RepeatedMessageDynamicFieldInfo
    : RepeatedPtrEntityDynamicFieldInfoBase<MessageT, Message>,
      RepeatedMessageDynamicFieldInfoMeta {
  using BaseT = RepeatedPtrEntityDynamicFieldInfoBase<MessageT, Message>;

  constexpr RepeatedMessageDynamicFieldInfo(
      const Reflection* r, MessageT& m, const FieldDescriptor* f,
      const RepeatedPtrField<Message>& rep)
      : BaseT(r, m, f, rep) {}

  size_t FieldByteSize() const {
    size_t byte_size = 0;
    for (auto& it : BaseT::const_repeated) {
      byte_size += WireFormatLite::LengthDelimitedSize(it.ByteSizeLong());
    }
    return byte_size;
  }
};

template <typename MessageT>
struct RepeatedGroupDynamicFieldInfo
    : RepeatedPtrEntityDynamicFieldInfoBase<MessageT, Message>,
      RepeatedMessageDynamicFieldInfoMeta {
  using BaseT = RepeatedPtrEntityDynamicFieldInfoBase<MessageT, Message>;

  constexpr RepeatedGroupDynamicFieldInfo(const Reflection* r, MessageT& m,
                                          const FieldDescriptor* f,
                                          const RepeatedPtrField<Message>& rep)
      : BaseT(r, m, f, rep) {}

  size_t FieldByteSize() const {
    size_t byte_size = 0;
    for (auto& it : BaseT::const_repeated) {
      byte_size += it.ByteSizeLong();
    }
    return byte_size;
  }
};

////////////////////////////////////////////////////////////////////////
// Extension repeated fields
////////////////////////////////////////////////////////////////////////
#define PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT(NAME, CPPNAME, CPPTYPE, \
                                                    FIELD_TYPE)             \
  template <typename ExtensionT>                                            \
  struct Repeated##NAME##DynamicExtensionInfo {                             \
    constexpr Repeated##NAME##DynamicExtensionInfo(ExtensionT& e, int n)    \
        : ext(e), ext_number(n) {}                                          \
                                                                            \
    int number() const { return ext_number; }                               \
    FieldDescriptor::Type type() const {                                    \
      return static_cast<FieldDescriptor::Type>(ext.type);                  \
    }                                                                       \
    bool is_packed() const { return ext.is_packed; }                        \
                                                                            \
    int size() const {                                                      \
      return DynamicExtensionInfoHelper::GetRepeated##CPPNAME(ext)->size(); \
    }                                                                       \
    const RepeatedField<FIELD_TYPE>& Get() const {                          \
      return *DynamicExtensionInfoHelper::GetRepeated##CPPNAME(ext);        \
    }                                                                       \
    RepeatedField<FIELD_TYPE>& Mutable() {                                  \
      return *DynamicExtensionInfoHelper::MutableRepeated##CPPNAME(ext);    \
    }                                                                       \
    void Clear() {                                                          \
      DynamicExtensionInfoHelper::MutableRepeated##CPPNAME(ext)->Clear();   \
    }                                                                       \
    size_t FieldByteSize() const {                                          \
      size_t byte_size = 0;                                                 \
      for (auto it : Get()) {                                               \
        byte_size += WireFormatLite::NAME##Size(it);                        \
      }                                                                     \
      return byte_size;                                                     \
    }                                                                       \
                                                                            \
    static constexpr FieldDescriptor::CppType cpp_type =                    \
        FieldDescriptor::CPPTYPE_##CPPTYPE;                                 \
    static constexpr bool is_repeated = true;                               \
    static constexpr bool is_map = false;                                   \
    static constexpr bool is_extension = true;                              \
    static constexpr bool is_oneof = false;                                 \
                                                                            \
    ExtensionT& ext;                                                        \
    int ext_number;                                                         \
  };

#define PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED(NAME, CPPNAME, CPPTYPE,  \
                                                   FIELD_TYPE)              \
  template <typename ExtensionT>                                            \
  struct Repeated##NAME##DynamicExtensionInfo {                             \
    constexpr Repeated##NAME##DynamicExtensionInfo(ExtensionT& e, int n)    \
        : ext(e), ext_number(n) {}                                          \
                                                                            \
    int number() const { return ext_number; }                               \
    FieldDescriptor::Type type() const {                                    \
      return static_cast<FieldDescriptor::Type>(ext.type);                  \
    }                                                                       \
    bool is_packed() const { return ext.is_packed; }                        \
                                                                            \
    int size() const {                                                      \
      return DynamicExtensionInfoHelper::GetRepeated##CPPNAME(ext)->size(); \
    }                                                                       \
    const RepeatedField<FIELD_TYPE>& Get() const {                          \
      return *DynamicExtensionInfoHelper::GetRepeated##CPPNAME(ext);        \
    }                                                                       \
    RepeatedField<FIELD_TYPE>& Mutable() {                                  \
      return *DynamicExtensionInfoHelper::MutableRepeated##CPPNAME(ext);    \
    }                                                                       \
    void Clear() {                                                          \
      DynamicExtensionInfoHelper::MutableRepeated##CPPNAME(ext)->Clear();   \
    }                                                                       \
    size_t FieldByteSize() const {                                          \
      return static_cast<size_t>(size()) * WireFormatLite::k##NAME##Size;   \
    }                                                                       \
                                                                            \
    static constexpr FieldDescriptor::CppType cpp_type =                    \
        FieldDescriptor::CPPTYPE_##CPPTYPE;                                 \
    static constexpr bool is_repeated = true;                               \
    static constexpr bool is_map = false;                                   \
    static constexpr bool is_extension = true;                              \
    static constexpr bool is_oneof = false;                                 \
                                                                            \
    ExtensionT& ext;                                                        \
    int ext_number;                                                         \
  };

PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT(Int32, Int32, INT32, int32_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT(Int64, Int64, INT64, int64_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT(UInt32, UInt32, UINT32, uint32_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT(UInt64, UInt64, UINT64, uint64_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT(SInt32, Int32, INT32, int32_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT(SInt64, Int64, INT64, int64_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT(Enum, Enum, ENUM, int);

PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED(Fixed32, UInt32, UINT32, uint32_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED(Fixed64, UInt64, UINT64, uint64_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED(SFixed32, Int32, INT32, int32_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED(SFixed64, Int64, INT64, int64_t);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED(Double, Double, DOUBLE, double);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED(Float, Float, FLOAT, float);
PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED(Bool, Bool, BOOL, bool);

#undef PROTOBUF_DYN_EXTENSION_INFO_REPEATED_VARINT
#undef PROTOBUF_DYN_EXTENSION_INFO_REPEATED_FIXED

template <typename ExtensionT>
struct RepeatedStringDynamicExtensionInfo {
  constexpr RepeatedStringDynamicExtensionInfo(ExtensionT& ext, int n)
      : ext(ext), ext_number(n) {}

  int number() const { return ext_number; }
  FieldDescriptor::Type type() const {
    return static_cast<FieldDescriptor::Type>(ext.type);
  }
  constexpr bool is_packed() const { return false; }

  int size() const {
    return DynamicExtensionInfoHelper::GetRepeatedString(ext)->size();
  }
  const RepeatedPtrField<std::string>& Get() const {
    return *DynamicExtensionInfoHelper::GetRepeatedString(ext);
  }
  RepeatedPtrField<std::string>& Mutable() {
    return *DynamicExtensionInfoHelper::MutableRepeatedString(ext);
  }
  void Clear() {
    DynamicExtensionInfoHelper::MutableRepeatedString(ext)->Clear();
  }
  size_t FieldByteSize() const {
    size_t byte_size = 0;
    for (auto& it : Get()) {
      byte_size += WireFormatLite::LengthDelimitedSize(it.size());
    }
    return byte_size;
  }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_STRING;
  static constexpr bool is_repeated = true;       // NOLINT
  static constexpr bool is_map = false;           // NOLINT
  static constexpr bool is_extension = true;      // NOLINT
  static constexpr bool is_oneof = false;         // NOLINT
  static constexpr bool is_cord = false;          // NOLINT
  static constexpr bool is_string_piece = false;  // NOLINT

  ExtensionT& ext;
  int ext_number;
};

template <typename ExtensionT>
struct RepeatedMessageDynamicExtensionInfoBase {
  constexpr RepeatedMessageDynamicExtensionInfoBase(ExtensionT& e, int n)
      : ext(e), ext_number(n) {}

  int number() const { return ext_number; }
  FieldDescriptor::Type type() const {
    return static_cast<FieldDescriptor::Type>(ext.type);
  }
  constexpr bool is_packed() const { return false; }

  int size() const {
    return DynamicExtensionInfoHelper::GetRepeatedMessage(ext)->size();
  }
  const RepeatedPtrField<MessageLite>& Get() const {
    return *DynamicExtensionInfoHelper::GetRepeatedMessage(ext);
  }
  RepeatedPtrField<MessageLite>& Mutable() {
    return *DynamicExtensionInfoHelper::MutableRepeatedMessage(ext);
  }
  void Clear() {
    DynamicExtensionInfoHelper::MutableRepeatedMessage(ext)->Clear();
  }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_MESSAGE;
  static constexpr bool is_repeated = true;   // NOLINT
  static constexpr bool is_map = false;       // NOLINT
  static constexpr bool is_extension = true;  // NOLINT
  static constexpr bool is_oneof = false;     // NOLINT

  ExtensionT& ext;
  int ext_number;
};

template <typename ExtensionT>
struct RepeatedMessageDynamicExtensionInfo
    : RepeatedMessageDynamicExtensionInfoBase<ExtensionT> {
  using BaseT = RepeatedMessageDynamicExtensionInfoBase<ExtensionT>;

  constexpr RepeatedMessageDynamicExtensionInfo(ExtensionT& e, int n)
      : BaseT(e, n) {}

  size_t FieldByteSize() const {
    size_t byte_size = 0;
    for (auto& it : BaseT::Get()) {
      byte_size += WireFormatLite::LengthDelimitedSize(it.ByteSizeLong());
    }
    return byte_size;
  }
};

template <typename ExtensionT>
struct RepeatedGroupDynamicExtensionInfo
    : RepeatedMessageDynamicExtensionInfoBase<ExtensionT> {
  using BaseT = RepeatedMessageDynamicExtensionInfoBase<ExtensionT>;

  constexpr RepeatedGroupDynamicExtensionInfo(ExtensionT& e, int n)
      : BaseT(e, n) {}

  size_t FieldByteSize() const {
    size_t byte_size = 0;
    for (auto& it : BaseT::Get()) {
      byte_size += it.ByteSizeLong();
    }
    return byte_size;
  }
};

////////////////////////////////////////////////////////////////////////
// Map fields
////////////////////////////////////////////////////////////////////////

// Returns the encoded size for (cpp_type, type, value). Some types are fixed
// sized; while others are variable. Dispatch done here at compile time frees
// users from a similar dispatch without creating KeyInfo or ValueInfo per type.
template <FieldDescriptor::CppType cpp_type, typename T>
inline size_t MapPrimitiveFieldByteSize(FieldDescriptor::Type type, T value) {
  // There is a bug in GCC 9.5 where if-constexpr arguments are not understood
  // if encased in a switch statement. A reproduction of the bug can be found
  // at: https://godbolt.org/z/qo51cKe7b
  // This is fixed in GCC 10.1+.
  (void)type;   // Suppress -Wunused-but-set-parameter
  (void)value;  // Suppress -Wunused-but-set-parameter

  if constexpr (cpp_type == FieldDescriptor::CPPTYPE_INT32) {
    static_assert(std::is_same_v<T, int32_t>, "type mismatch");
    switch (type) {
      case FieldDescriptor::TYPE_INT32:
        return WireFormatLite::Int32Size(value);
      case FieldDescriptor::TYPE_SINT32:
        return WireFormatLite::SInt32Size(value);
      case FieldDescriptor::TYPE_SFIXED32:
        return WireFormatLite::kSFixed32Size;
      default:
        Unreachable();
    }
  } else if constexpr (cpp_type == FieldDescriptor::CPPTYPE_INT64) {
    static_assert(std::is_same_v<T, int64_t>, "type mismatch");
    switch (type) {
      case FieldDescriptor::TYPE_INT64:
        return WireFormatLite::Int64Size(value);
      case FieldDescriptor::TYPE_SINT64:
        return WireFormatLite::SInt64Size(value);
      case FieldDescriptor::TYPE_SFIXED64:
        return WireFormatLite::kSFixed64Size;
      default:
        Unreachable();
    }
  } else if constexpr (cpp_type == FieldDescriptor::CPPTYPE_UINT32) {
    static_assert(std::is_same_v<T, uint32_t>, "type mismatch");
    switch (type) {
      case FieldDescriptor::TYPE_UINT32:
        return WireFormatLite::UInt32Size(value);
      case FieldDescriptor::TYPE_FIXED32:
        return WireFormatLite::kSFixed32Size;
      default:
        Unreachable();
    }
  } else if constexpr (cpp_type == FieldDescriptor::CPPTYPE_UINT64) {
    static_assert(std::is_same_v<T, uint64_t>, "type mismatch");
    switch (type) {
      case FieldDescriptor::TYPE_UINT64:
        return WireFormatLite::UInt64Size(value);
      case FieldDescriptor::TYPE_FIXED64:
        return WireFormatLite::kSFixed64Size;
      default:
        Unreachable();
    }
  } else if constexpr (cpp_type == FieldDescriptor::CPPTYPE_ENUM) {
    static_assert(std::is_same_v<T, int>, "type mismatch");
    return WireFormatLite::EnumSize(value);
  } else if constexpr (cpp_type == FieldDescriptor::CPPTYPE_BOOL) {
    static_assert(std::is_same_v<T, bool>, "type mismatch");
    return WireFormatLite::kBoolSize;
  } else if constexpr (cpp_type == FieldDescriptor::CPPTYPE_FLOAT) {
    static_assert(std::is_same_v<T, float>, "type mismatch");
    return WireFormatLite::kFloatSize;
  } else if constexpr (cpp_type == FieldDescriptor::CPPTYPE_DOUBLE) {
    static_assert(std::is_same_v<T, double>, "type mismatch");
    return WireFormatLite::kDoubleSize;
  }
}

#define PROTOBUF_MAP_KEY_INFO(NAME, KEY_TYPE, CPPTYPE)                   \
  struct MapDynamicField##NAME##KeyInfo {                                \
    explicit MapDynamicField##NAME##KeyInfo(const MapKey& k) : key(k) {  \
      ABSL_DCHECK_EQ(cpp_type, key.type());                              \
    }                                                                    \
                                                                         \
    KEY_TYPE Get() const { return key.Get##NAME##Value(); }              \
    /* Set() API doesn't make sense because MapIter always returns const \
     * MapKey&. */                                                       \
                                                                         \
    static constexpr FieldDescriptor::CppType cpp_type =                 \
        FieldDescriptor::CPPTYPE_##CPPTYPE;                              \
                                                                         \
    const MapKey& key;                                                   \
  };

PROTOBUF_MAP_KEY_INFO(Int32, int32_t, INT32);
PROTOBUF_MAP_KEY_INFO(Int64, int64_t, INT64);
PROTOBUF_MAP_KEY_INFO(UInt32, uint32_t, UINT32);
PROTOBUF_MAP_KEY_INFO(UInt64, uint64_t, UINT64);
PROTOBUF_MAP_KEY_INFO(Bool, bool, BOOL);
PROTOBUF_MAP_KEY_INFO(String, absl::string_view, STRING);

#undef PROTOBUF_MAP_KEY_INFO

#define PROTOBUF_MAP_VALUE_INFO(NAME, VALUE_TYPE, CPPTYPE)                  \
  template <typename MapValueRefT>                                          \
  struct MapDynamicField##NAME##ValueInfo {                                 \
    explicit MapDynamicField##NAME##ValueInfo(MapValueRefT& v) : value(v) { \
      ABSL_DCHECK_EQ(cpp_type, value.type());                               \
    }                                                                       \
                                                                            \
    VALUE_TYPE Get() const { return value.Get##NAME##Value(); }             \
    void Set(VALUE_TYPE val) { value.Set##NAME##Value(val); }               \
                                                                            \
    static constexpr FieldDescriptor::CppType cpp_type =                    \
        FieldDescriptor::CPPTYPE_##CPPTYPE;                                 \
                                                                            \
    MapValueRefT& value;                                                    \
  };

PROTOBUF_MAP_VALUE_INFO(Int32, int32_t, INT32);
PROTOBUF_MAP_VALUE_INFO(Int64, int64_t, INT64);
PROTOBUF_MAP_VALUE_INFO(UInt32, uint32_t, UINT32);
PROTOBUF_MAP_VALUE_INFO(UInt64, uint64_t, UINT64);
PROTOBUF_MAP_VALUE_INFO(Bool, bool, BOOL);
PROTOBUF_MAP_VALUE_INFO(Enum, int, ENUM);
PROTOBUF_MAP_VALUE_INFO(Float, float, FLOAT);
PROTOBUF_MAP_VALUE_INFO(Double, double, DOUBLE);
PROTOBUF_MAP_VALUE_INFO(String, absl::string_view, STRING);

#undef PROTOBUF_MAP_VALUE_INFO

template <typename MapValueRefT>
struct MapDynamicFieldMessageValueInfo {
  explicit constexpr MapDynamicFieldMessageValueInfo(MapValueRefT& v)
      : value(v) {}

  const Message& Get() const { return value.GetMessageValue(); }
  Message* Mutable() { return value.MutableMessageValue(); }

  static constexpr FieldDescriptor::CppType cpp_type =  // NOLINT
      FieldDescriptor::CPPTYPE_MESSAGE;

  MapValueRefT& value;
};

// Calls "cb" with the corresponding ValueInfo. Typically called from
// MapDynamicFieldVisitKey.
template <typename MapValueRefT, typename MapValueCallback>
void MapDynamicFieldVisitValue(MapValueRefT& value, MapValueCallback&& cb) {
  switch (value.type()) {
#define PROTOBUF_HANDLE_MAP_VALUE_CASE(NAME, VALUE_TYPE, CPPTYPE) \
  case FieldDescriptor::CPPTYPE_##CPPTYPE:                        \
    cb(MapDynamicField##NAME##ValueInfo<MapValueRefT>{value});    \
    break;

    PROTOBUF_HANDLE_MAP_VALUE_CASE(Int32, int32_t, INT32);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(Int64, int64_t, INT64);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(UInt32, uint32_t, UINT32);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(UInt64, uint64_t, UINT64);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(Bool, bool, BOOL);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(Enum, int, ENUM);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(Float, float, FLOAT);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(Double, double, DOUBLE);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(String, std::string, STRING);
    PROTOBUF_HANDLE_MAP_VALUE_CASE(Message, Message, MESSAGE);

    default:
      internal::Unreachable();

#undef PROTOBUF_HANDLE_MAP_VALUE_CASE
  }
}

// Dispatches based on key type to instantiate a right KeyInfo, then calls
// MapDynamicFieldVisitValue to dispatch on the value type.
template <typename MapValueRefT, typename MapFieldCallback>
void MapDynamicFieldVisitKey(const MapKey& key, MapValueRefT& value,
                             const MapFieldCallback& user_cb) {
  switch (key.type()) {
#define PROTOBUF_HANDLE_MAP_KEY_CASE(NAME, CPPTYPE)                            \
  case FieldDescriptor::CPPTYPE_##CPPTYPE: {                                   \
    auto key_info = MapDynamicField##NAME##KeyInfo{key};                       \
    MapDynamicFieldVisitValue(value, [key_info, &user_cb](auto&& value_info) { \
      user_cb(key_info, value_info);                                           \
    });                                                                        \
    break;                                                                     \
  }

    PROTOBUF_HANDLE_MAP_KEY_CASE(Int32, INT32);
    PROTOBUF_HANDLE_MAP_KEY_CASE(Int64, INT64);
    PROTOBUF_HANDLE_MAP_KEY_CASE(UInt32, UINT32);
    PROTOBUF_HANDLE_MAP_KEY_CASE(UInt64, UINT64);
    PROTOBUF_HANDLE_MAP_KEY_CASE(Bool, BOOL);
    PROTOBUF_HANDLE_MAP_KEY_CASE(String, STRING);

#undef PROTOBUF_HANDLE_MAP_KEY_CASE

    default:
      internal::Unreachable();
      break;
  }
}

template <typename MessageT>
struct MapDynamicFieldInfo {
  constexpr MapDynamicFieldInfo(const Reflection* r, MessageT& m,
                                const FieldDescriptor* f,
                                const FieldDescriptor* key_f,
                                const FieldDescriptor* val_f,
                                const MapFieldBase& map_field)
      : reflection(r),
        message(m),
        field(f),
        key(key_f),
        value(val_f),
        const_map_field(map_field) {
    ABSL_DCHECK(f->is_map());
    ABSL_DCHECK_NE(key_f, nullptr);
    ABSL_DCHECK_NE(val_f, nullptr);
  }

  int number() const { return field->number(); }
  FieldDescriptor::Type key_type() const { return key->type(); }
  FieldDescriptor::Type value_type() const { return value->type(); }
  int size() const { return const_map_field.size(); }

  // go/ranked-overloads for the rationale.
  struct Rank0 {};
  struct Rank1 : Rank0 {};

  // Preferred version when "msg" is non-const.
  template <typename T, typename Callback,
            typename = std::enable_if_t<!std::is_const_v<T>>>
  static void VisitElementsImpl(T& msg, const Reflection* reflection,
                                const FieldDescriptor* field,
                                const MapFieldBase&, Callback&& cb, Rank1) {
    auto& map_field =
        DynamicFieldInfoHelper<false>::template Mutable<MapFieldBase>(
            reflection, msg, field);
    const Descriptor* descriptor = field->message_type();
    MapIterator begin(&map_field, descriptor), end(&map_field, descriptor);
    map_field.MapBegin(&begin);
    map_field.MapEnd(&end);

    for (auto it = begin; it != end; ++it) {
      MapDynamicFieldVisitKey(it.GetKey(), *it.MutableValueRef(), cb);
    }
  }

  // Fallback version otherwise.
  template <typename T, typename Callback>
  static void VisitElementsImpl(T& msg, const Reflection*,
                                const FieldDescriptor* field,
                                const MapFieldBase& const_map_field,
                                Callback&& cb, Rank0) {
    // Unfortunately, we have to const_cast here because MapIterator only takes
    // a mutable MapFieldBase pointer. This is still safe because value iterator
    // is not mutable.
    MapFieldBase* map_field = const_cast<MapFieldBase*>(&const_map_field);
    const Descriptor* descriptor = field->message_type();
    MapIterator begin(map_field, descriptor), end(map_field, descriptor);
    const_map_field.MapBegin(&begin);
    const_map_field.MapEnd(&end);

    for (auto it = begin; it != end; ++it) {
      MapDynamicFieldVisitKey(it.GetKey(), it.GetValueRef(), cb);
    }
  }

  template <typename MapFieldCallback>
  void VisitElements(MapFieldCallback&& cb) const {
    VisitElementsImpl(message, reflection, field, const_map_field,
                      static_cast<MapFieldCallback&&>(cb), Rank1{});
  }

  void Clear() {
    auto& map_field =
        DynamicFieldInfoHelper<false>::template Mutable<MapFieldBase>(
            reflection, message, field);

    map_field.Clear();
  }

  static constexpr bool is_repeated = true;    // NOLINT
  static constexpr bool is_packed = false;     // NOLINT
  static constexpr bool is_map = true;         // NOLINT
  static constexpr bool is_extension = false;  // NOLINT
  static constexpr bool is_oneof = false;      // NOLINT

  const Reflection* reflection;
  MessageT& message;
  const FieldDescriptor* field;
  const FieldDescriptor* key;
  const FieldDescriptor* value;
  const MapFieldBase& const_map_field;
};

#endif  // __cpp_if_constexpr

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_REFLECTION_VISIT_FIELD_INFO_H__
