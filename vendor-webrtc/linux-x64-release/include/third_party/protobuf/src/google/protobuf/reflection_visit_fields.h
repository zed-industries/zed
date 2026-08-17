#ifndef GOOGLE_PROTOBUF_REFLECTION_VISIT_FIELDS_H__
#define GOOGLE_PROTOBUF_REFLECTION_VISIT_FIELDS_H__

#include <cstdint>
#include <string>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/base/optimization.h"
#include "absl/log/absl_check.h"
#include "absl/strings/cord.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/descriptor_lite.h"
#include "google/protobuf/extension_set.h"
#include "google/protobuf/generated_message_reflection.h"
#include "google/protobuf/message.h"
#include "google/protobuf/port.h"
#include "google/protobuf/reflection.h"
#include "google/protobuf/reflection_visit_field_info.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"


// Must be the last include.
#include "google/protobuf/port_def.inc"  // NOLINT

namespace google {
namespace protobuf {
namespace internal {

enum class FieldMask : uint32_t {
  kInt32 = 1 << FieldDescriptor::CPPTYPE_INT32,
  kInt64 = 1 << FieldDescriptor::CPPTYPE_INT64,
  kUInt32 = 1 << FieldDescriptor::CPPTYPE_UINT32,
  kUInt64 = 1 << FieldDescriptor::CPPTYPE_UINT64,
  kDouble = 1 << FieldDescriptor::CPPTYPE_DOUBLE,
  kFloat = 1 << FieldDescriptor::CPPTYPE_FLOAT,
  kBool = 1 << FieldDescriptor::CPPTYPE_BOOL,
  kEnum = 1 << FieldDescriptor::CPPTYPE_ENUM,
  kString = 1 << FieldDescriptor::CPPTYPE_STRING,
  kMessage = 1 << FieldDescriptor::CPPTYPE_MESSAGE,
  kPrimitive =
      kInt32 | kInt64 | kUInt32 | kUInt64 | kDouble | kFloat | kBool | kEnum,
  kAll = 0xFFFFFFFFu,
};

inline FieldMask operator|(FieldMask lhs, FieldMask rhs) {
  return static_cast<FieldMask>(static_cast<uint32_t>(lhs) |
                                static_cast<uint32_t>(rhs));
}

#ifdef __cpp_if_constexpr

template <typename MessageT, typename CallbackFn>
void VisitFields(MessageT& message, CallbackFn&& func,
                 FieldMask mask = FieldMask::kAll);

class ReflectionVisit final {
 public:
  template <typename MessageT, typename CallbackFn>
  static void VisitFields(MessageT& message, CallbackFn&& func, FieldMask mask);

  template <typename CallbackFn>
  static void VisitMessageFields(const Message& message, CallbackFn&& func);

  template <typename CallbackFn>
  static void VisitMessageFields(Message& message, CallbackFn&& func);

 private:
  static const internal::ReflectionSchema& GetSchema(
      const Reflection* reflection) {
    return reflection->schema_;
  }
  static const Descriptor* GetDescriptor(const Reflection* reflection) {
    return reflection->descriptor_;
  }
  static const internal::ExtensionSet& ExtensionSet(
      const Reflection* reflection, const Message& message) {
    return reflection->GetExtensionSet(message);
  }
  static internal::ExtensionSet& ExtensionSet(const Reflection* reflection,
                                              Message& message) {
    return *reflection->MutableExtensionSet(&message);
  }
};

inline bool ShouldVisit(FieldMask mask, FieldDescriptor::CppType cpptype) {
  if (ABSL_PREDICT_TRUE(mask == FieldMask::kAll)) return true;
  return (static_cast<uint32_t>(mask) & (1 << cpptype)) != 0;
}

template <typename MessageT, typename CallbackFn>
void ReflectionVisit::VisitFields(MessageT& message, CallbackFn&& func,
                                  FieldMask mask) {
  const Reflection* reflection = message.GetReflection();
  const auto& schema = GetSchema(reflection);

  ABSL_CHECK(!schema.HasWeakFields()) << "weak fields are not supported";

  // See Reflection::ListFields for the optimization.
  const uint32_t* const has_bits =
      schema.HasHasbits() ? reflection->GetHasBits(message) : nullptr;
  const uint32_t* const has_bits_indices = schema.has_bit_indices_;
  const Descriptor* descriptor = GetDescriptor(reflection);
  const int field_count = descriptor->field_count();

  for (int i = 0; i < field_count; i++) {
    const FieldDescriptor* field = descriptor->field(i);
    ABSL_DCHECK(!field->options().weak()) << "weak fields are not supported";

    if (!ShouldVisit(mask, field->cpp_type())) continue;

    if (field->is_repeated()) {
      switch (field->type()) {
#define PROTOBUF_HANDLE_REPEATED_CASE(TYPE, CPPTYPE, NAME)                  \
  case FieldDescriptor::TYPE_##TYPE: {                                      \
    ABSL_DCHECK(!field->is_map());                                          \
    const auto& rep =                                                       \
        reflection->GetRawNonOneof<RepeatedField<CPPTYPE>>(message, field); \
    if (rep.size() == 0) continue;                                          \
    func(internal::Repeated##NAME##DynamicFieldInfo<MessageT>{              \
        reflection, message, field, rep});                                  \
    break;                                                                  \
  }

        PROTOBUF_HANDLE_REPEATED_CASE(DOUBLE, double, Double);
        PROTOBUF_HANDLE_REPEATED_CASE(FLOAT, float, Float);
        PROTOBUF_HANDLE_REPEATED_CASE(INT64, int64_t, Int64);
        PROTOBUF_HANDLE_REPEATED_CASE(UINT64, uint64_t, UInt64);
        PROTOBUF_HANDLE_REPEATED_CASE(INT32, int32_t, Int32);
        PROTOBUF_HANDLE_REPEATED_CASE(FIXED64, uint64_t, Fixed64);
        PROTOBUF_HANDLE_REPEATED_CASE(FIXED32, uint32_t, Fixed32);
        PROTOBUF_HANDLE_REPEATED_CASE(BOOL, bool, Bool);
        PROTOBUF_HANDLE_REPEATED_CASE(UINT32, uint32_t, UInt32);
        PROTOBUF_HANDLE_REPEATED_CASE(ENUM, int, Enum);
        PROTOBUF_HANDLE_REPEATED_CASE(SFIXED32, int32_t, SFixed32);
        PROTOBUF_HANDLE_REPEATED_CASE(SFIXED64, int64_t, SFixed64);
        PROTOBUF_HANDLE_REPEATED_CASE(SINT32, int32_t, SInt32);
        PROTOBUF_HANDLE_REPEATED_CASE(SINT64, int64_t, SInt64);

#define PROTOBUF_HANDLE_REPEATED_PTR_CASE(TYPE, CPPTYPE, NAME)                 \
  case FieldDescriptor::TYPE_##TYPE: {                                         \
    if (ABSL_PREDICT_TRUE(!field->is_map())) {                                 \
      /* Handle repeated fields. */                                            \
      const auto& rep = reflection->GetRawNonOneof<RepeatedPtrField<CPPTYPE>>( \
          message, field);                                                     \
      if (rep.size() == 0) continue;                                           \
      func(internal::Repeated##NAME##DynamicFieldInfo<MessageT>{               \
          reflection, message, field, rep});                                   \
    } else {                                                                   \
      /* Handle map fields. */                                                 \
      const auto& map =                                                        \
          reflection->GetRawNonOneof<MapFieldBase>(message, field);            \
      if (map.size() == 0) continue; /* NOLINT */                              \
      const Descriptor* desc = field->message_type();                          \
      func(internal::MapDynamicFieldInfo<MessageT>{reflection, message, field, \
                                                   desc->map_key(),            \
                                                   desc->map_value(), map});   \
    }                                                                          \
    break;                                                                     \
  }

        PROTOBUF_HANDLE_REPEATED_PTR_CASE(MESSAGE, Message, Message);
        PROTOBUF_HANDLE_REPEATED_PTR_CASE(GROUP, Message, Group);

        case FieldDescriptor::TYPE_BYTES:
        case FieldDescriptor::TYPE_STRING:
#define PROTOBUF_IMPL_STRING_CASE(CPPTYPE, NAME)                               \
  {                                                                            \
    const auto& rep =                                                          \
        reflection->GetRawNonOneof<RepeatedPtrField<CPPTYPE>>(message, field); \
    if (rep.size() == 0) continue;                                             \
    func(internal::Repeated##NAME##DynamicFieldInfo<MessageT>{                 \
        reflection, message, field, rep});                                     \
  }

          switch (field->cpp_string_type()) {
            case FieldDescriptor::CppStringType::kCord:
            case FieldDescriptor::CppStringType::kView:
            case FieldDescriptor::CppStringType::kString:
              PROTOBUF_IMPL_STRING_CASE(std::string, String);
              break;
          }
          break;
        default:
          internal::Unreachable();
          break;
      }
#undef PROTOBUF_HANDLE_REPEATED_CASE
#undef PROTOBUF_HANDLE_REPEATED_PTR_CASE
#undef PROTOBUF_IMPL_STRING_CASE
    } else if (schema.InRealOneof(field)) {
      const OneofDescriptor* containing_oneof = field->containing_oneof();
      const uint32_t* const oneof_case_array =
          internal::GetConstPointerAtOffset<uint32_t>(
              &message, schema.oneof_case_offset_);
      // Equivalent to: !HasOneofField(message, field)
      if (static_cast<int64_t>(oneof_case_array[containing_oneof->index()]) !=
          field->number()) {
        continue;
      }
      switch (field->type()) {
#define PROTOBUF_HANDLE_CASE(TYPE, NAME)                                       \
  case FieldDescriptor::TYPE_##TYPE:                                           \
    func(internal::NAME##DynamicFieldInfo<MessageT, true>{reflection, message, \
                                                          field});             \
    break;
        PROTOBUF_HANDLE_CASE(DOUBLE, Double);
        PROTOBUF_HANDLE_CASE(FLOAT, Float);
        PROTOBUF_HANDLE_CASE(INT64, Int64);
        PROTOBUF_HANDLE_CASE(UINT64, UInt64);
        PROTOBUF_HANDLE_CASE(INT32, Int32);
        PROTOBUF_HANDLE_CASE(FIXED64, Fixed64);
        PROTOBUF_HANDLE_CASE(FIXED32, Fixed32);
        PROTOBUF_HANDLE_CASE(BOOL, Bool);
        PROTOBUF_HANDLE_CASE(UINT32, UInt32);
        PROTOBUF_HANDLE_CASE(ENUM, Enum);
        PROTOBUF_HANDLE_CASE(SFIXED32, SFixed32);
        PROTOBUF_HANDLE_CASE(SFIXED64, SFixed64);
        PROTOBUF_HANDLE_CASE(SINT32, SInt32);
        PROTOBUF_HANDLE_CASE(SINT64, SInt64);

        case FieldDescriptor::TYPE_MESSAGE:
        case FieldDescriptor::TYPE_GROUP:
          func(internal::MessageDynamicFieldInfo<MessageT, true>{
              reflection, message, field});
          break;

        case FieldDescriptor::TYPE_BYTES:
        case FieldDescriptor::TYPE_STRING: {
          switch (field->cpp_string_type()) {
            case FieldDescriptor::CppStringType::kCord:
              func(CordDynamicFieldInfo<MessageT, true>{reflection, message,
                                                        field});
              break;
            case FieldDescriptor::CppStringType::kString:
            case FieldDescriptor::CppStringType::kView:
              func(StringDynamicFieldInfo<MessageT, true>{reflection, message,
                                                          field});
              break;
          }
          break;
        }
        default:
          internal::Unreachable();
          break;
#undef PROTOBUF_HANDLE_CASE
      }
    } else {
      auto index = has_bits_indices[i];
      bool check_hasbits = has_bits && index != static_cast<uint32_t>(-1);
      if (ABSL_PREDICT_TRUE(check_hasbits)) {
        if ((has_bits[index / 32] & (1u << (index % 32))) == 0) continue;
      } else {
        // Skip if it has default values.
        if (!reflection->HasFieldSingular(message, field)) continue;
      }
      switch (field->type()) {
#define PROTOBUF_HANDLE_CASE(TYPE, NAME)                                     \
  case FieldDescriptor::TYPE_##TYPE:                                         \
    func(internal::NAME##DynamicFieldInfo<MessageT, false>{reflection,       \
                                                           message, field}); \
    break;
        PROTOBUF_HANDLE_CASE(DOUBLE, Double);
        PROTOBUF_HANDLE_CASE(FLOAT, Float);
        PROTOBUF_HANDLE_CASE(INT64, Int64);
        PROTOBUF_HANDLE_CASE(UINT64, UInt64);
        PROTOBUF_HANDLE_CASE(INT32, Int32);
        PROTOBUF_HANDLE_CASE(FIXED64, Fixed64);
        PROTOBUF_HANDLE_CASE(FIXED32, Fixed32);
        PROTOBUF_HANDLE_CASE(BOOL, Bool);
        PROTOBUF_HANDLE_CASE(UINT32, UInt32);
        PROTOBUF_HANDLE_CASE(ENUM, Enum);
        PROTOBUF_HANDLE_CASE(SFIXED32, SFixed32);
        PROTOBUF_HANDLE_CASE(SFIXED64, SFixed64);
        PROTOBUF_HANDLE_CASE(SINT32, SInt32);
        PROTOBUF_HANDLE_CASE(SINT64, SInt64);

        case FieldDescriptor::TYPE_MESSAGE:
        case FieldDescriptor::TYPE_GROUP:
          func(internal::MessageDynamicFieldInfo<MessageT, false>{
              reflection, message, field});
          break;
        case FieldDescriptor::TYPE_BYTES:
        case FieldDescriptor::TYPE_STRING: {
          switch (field->cpp_string_type()) {
            case FieldDescriptor::CppStringType::kCord:
              func(CordDynamicFieldInfo<MessageT, false>{reflection, message,
                                                         field});
              break;
            case FieldDescriptor::CppStringType::kString:
            case FieldDescriptor::CppStringType::kView:
              func(StringDynamicFieldInfo<MessageT, false>{reflection, message,
                                                           field});
              break;
          }
          break;
        }
        default:
          internal::Unreachable();
          break;
#undef PROTOBUF_HANDLE_CASE
      }
    }
  }

  if (!schema.HasExtensionSet()) return;

  auto& set = ExtensionSet(reflection, message);
  auto* extendee = reflection->descriptor_;
  auto* pool = reflection->descriptor_pool_;

  set.ForEach(
      [&](int number, auto& ext) {
        ABSL_DCHECK_GT(ext.type, 0);
        ABSL_DCHECK_LE(ext.type, FieldDescriptor::MAX_TYPE);

        if (!ShouldVisit(mask,
                         FieldDescriptor::TypeToCppType(
                             static_cast<FieldDescriptor::Type>(ext.type)))) {
          return;
        }

        if (ext.is_repeated) {
          if (ext.GetSize() == 0) return;

          switch (ext.type) {
#define PROTOBUF_HANDLE_CASE(TYPE, NAME)                                \
  case FieldDescriptor::TYPE_##TYPE:                                    \
    func(internal::Repeated##NAME##DynamicExtensionInfo<decltype(ext)>{ \
        ext, number});                                                  \
    break;
            PROTOBUF_HANDLE_CASE(DOUBLE, Double);
            PROTOBUF_HANDLE_CASE(FLOAT, Float);
            PROTOBUF_HANDLE_CASE(INT64, Int64);
            PROTOBUF_HANDLE_CASE(UINT64, UInt64);
            PROTOBUF_HANDLE_CASE(INT32, Int32);
            PROTOBUF_HANDLE_CASE(FIXED64, Fixed64);
            PROTOBUF_HANDLE_CASE(FIXED32, Fixed32);
            PROTOBUF_HANDLE_CASE(BOOL, Bool);
            PROTOBUF_HANDLE_CASE(UINT32, UInt32);
            PROTOBUF_HANDLE_CASE(ENUM, Enum);
            PROTOBUF_HANDLE_CASE(SFIXED32, SFixed32);
            PROTOBUF_HANDLE_CASE(SFIXED64, SFixed64);
            PROTOBUF_HANDLE_CASE(SINT32, SInt32);
            PROTOBUF_HANDLE_CASE(SINT64, SInt64);

            PROTOBUF_HANDLE_CASE(MESSAGE, Message);
            PROTOBUF_HANDLE_CASE(GROUP, Group);

            case FieldDescriptor::TYPE_BYTES:
            case FieldDescriptor::TYPE_STRING:
              func(internal::RepeatedStringDynamicExtensionInfo<decltype(ext)>{
                  ext, number});
              break;
            default:
              internal::Unreachable();
              break;
#undef PROTOBUF_HANDLE_CASE
          }
        } else {
          if (ext.is_cleared) return;

          switch (ext.type) {
#define PROTOBUF_HANDLE_CASE(TYPE, NAME)                                    \
  case FieldDescriptor::TYPE_##TYPE:                                        \
    func(internal::NAME##DynamicExtensionInfo<decltype(ext)>{ext, number}); \
    break;
            PROTOBUF_HANDLE_CASE(DOUBLE, Double);
            PROTOBUF_HANDLE_CASE(FLOAT, Float);
            PROTOBUF_HANDLE_CASE(INT64, Int64);
            PROTOBUF_HANDLE_CASE(UINT64, UInt64);
            PROTOBUF_HANDLE_CASE(INT32, Int32);
            PROTOBUF_HANDLE_CASE(FIXED64, Fixed64);
            PROTOBUF_HANDLE_CASE(FIXED32, Fixed32);
            PROTOBUF_HANDLE_CASE(BOOL, Bool);
            PROTOBUF_HANDLE_CASE(UINT32, UInt32);
            PROTOBUF_HANDLE_CASE(ENUM, Enum);
            PROTOBUF_HANDLE_CASE(SFIXED32, SFixed32);
            PROTOBUF_HANDLE_CASE(SFIXED64, SFixed64);
            PROTOBUF_HANDLE_CASE(SINT32, SInt32);
            PROTOBUF_HANDLE_CASE(SINT64, SInt64);

            PROTOBUF_HANDLE_CASE(GROUP, Group);
            case FieldDescriptor::TYPE_MESSAGE: {
              const FieldDescriptor* field =
                  ext.descriptor != nullptr
                      ? ext.descriptor
                      : pool->FindExtensionByNumber(extendee, number);
              ABSL_DCHECK_EQ(field->number(), number);
              bool is_mset =
                  field->containing_type()->options().message_set_wire_format();
              func(internal::MessageDynamicExtensionInfo<decltype(ext)>{
                  ext, number, is_mset});
              break;
            }

            case FieldDescriptor::TYPE_BYTES:
            case FieldDescriptor::TYPE_STRING:
              func(internal::StringDynamicExtensionInfo<decltype(ext)>{ext,
                                                                       number});
              break;

            default:
              internal::Unreachable();
              break;
#undef PROTOBUF_HANDLE_CASE
          }
        }
      },
      ExtensionSet::Prefetch{});
}

template <typename CallbackFn>
void ReflectionVisit::VisitMessageFields(const Message& message,
                                         CallbackFn&& func) {
  ReflectionVisit::VisitFields(
      message,
      [&](auto info) {
        if constexpr (info.is_map) {
          auto value_type = info.value_type();
          if (value_type != FieldDescriptor::TYPE_MESSAGE &&
              value_type != FieldDescriptor::TYPE_GROUP) {
            return;
          }
          info.VisitElements([&](auto key, auto val) {
            if constexpr (val.cpp_type == FieldDescriptor::CPPTYPE_MESSAGE) {
              func(val.Get());
            }
          });
        } else if constexpr (info.cpp_type ==
                             FieldDescriptor::CPPTYPE_MESSAGE) {
          if constexpr (info.is_repeated) {
            for (const auto& it : info.Get()) {
              func(DownCastMessage<Message>(it));
            }
          } else {
            func(info.Get());
          }
        }
      },
      FieldMask::kMessage);
}

template <typename CallbackFn>
void ReflectionVisit::VisitMessageFields(Message& message, CallbackFn&& func) {
  ReflectionVisit::VisitFields(
      message,
      [&](auto info) {
        if constexpr (info.is_map) {
          auto value_type = info.value_type();
          if (value_type != FieldDescriptor::TYPE_MESSAGE &&
              value_type != FieldDescriptor::TYPE_GROUP) {
            return;
          }
          info.VisitElements([&](auto key, auto val) {
            if constexpr (val.cpp_type == FieldDescriptor::CPPTYPE_MESSAGE) {
              func(*val.Mutable());
            }
          });
        } else if constexpr (info.cpp_type ==
                             FieldDescriptor::CPPTYPE_MESSAGE) {
          if constexpr (info.is_repeated) {
            for (auto& it : info.Mutable()) {
              func(DownCastMessage<Message>(it));
            }
          } else {
            func(info.Mutable());
          }
        }
      },
      FieldMask::kMessage);
}

// Visits present fields of "message" and calls the callback function "func".
// Skips fields whose ctypes are missing in "mask".
template <typename MessageT, typename CallbackFn>
void VisitFields(MessageT& message, CallbackFn&& func, FieldMask mask) {
  ReflectionVisit::VisitFields(message, std::forward<CallbackFn>(func), mask);
}

// Visits message fields of "message" and calls "func". Expects "func" to
// accept const Message&. Note the following divergence from VisitFields.
//
// --Each of N elements of a repeated message field is visited (total N).
// --Each of M elements of a map field whose value type is message are visited
//   (total M).
// --A map field whose value type is not message is ignored.
//
// This is a helper API built on top of VisitFields to hide specifics about
// extensions, repeated fields, etc.
template <typename CallbackFn>
void VisitMessageFields(const Message& message, CallbackFn&& func) {
  ReflectionVisit::VisitMessageFields(message, std::forward<CallbackFn>(func));
}

// Same as VisitMessageFields above but expects "func" to accept Message&. This
// is useful when mutable access is required. As mutable access can be
// expensive, use it only if it's necessary.
template <typename CallbackFn>
void VisitMutableMessageFields(Message& message, CallbackFn&& func) {
  ReflectionVisit::VisitMessageFields(message, std::forward<CallbackFn>(func));
}

#endif  // __cpp_if_constexpr

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_REFLECTION_VISIT_FIELDS_H__
