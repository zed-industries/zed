// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_JSON_INTERNAL_DESCRIPTOR_TRAITS_H__
#define GOOGLE_PROTOBUF_JSON_INTERNAL_DESCRIPTOR_TRAITS_H__

#include <array>
#include <cfloat>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <limits>
#include <string>
#include <utility>

#include "google/protobuf/type.pb.h"
#include "absl/algorithm/container.h"
#include "absl/log/absl_log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/match.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/types/optional.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/dynamic_message.h"
#include "google/protobuf/json/internal/lexer.h"
#include "google/protobuf/json/internal/untyped_message.h"
#include "google/protobuf/message.h"
#include "google/protobuf/stubs/status_macros.h"


// Must be included last.
#include "google/protobuf/port_def.inc"

// Traits for working with descriptor.proto and type.proto generically.

namespace google {
namespace protobuf {
namespace json_internal {
enum class MessageType {
  kNotWellKnown,
  kAny,
  kWrapper,
  kStruct,
  kList,
  kValue,
  kNull,
  kTimestamp,
  kDuration,
  kFieldMask,
};

inline MessageType ClassifyMessage(absl::string_view name) {
  constexpr absl::string_view kWellKnownPkg = "google.protobuf.";
  if (!absl::StartsWith(name, kWellKnownPkg)) {
    return MessageType::kNotWellKnown;
  }
  name = name.substr(kWellKnownPkg.size());

  switch (name.size()) {
    case 3:
      if (name == "Any") {
        return MessageType::kAny;
      }
      break;
    case 5:
      if (name == "Value") {
        return MessageType::kValue;
      }
      break;
    case 6:
      if (name == "Struct") {
        return MessageType::kStruct;
      }
      break;
    case 8:
      if (name == "Duration") {
        return MessageType::kDuration;
      }
      break;
    case 9:
      if (name == "BoolValue") {
        return MessageType::kWrapper;
      }
      if (name == "NullValue") {
        return MessageType::kNull;
      }
      if (name == "ListValue") {
        return MessageType::kList;
      }
      if (name == "Timestamp") {
        return MessageType::kTimestamp;
      }
      if (name == "FieldMask") {
        return MessageType::kFieldMask;
      }
      break;
    case 10:
      if (name == "BytesValue" || name == "FloatValue" ||
          name == "Int32Value" || name == "Int64Value") {
        return MessageType::kWrapper;
      }
      break;
    case 11:
      if (name == "DoubleValue" || name == "StringValue" ||
          name == "UInt32Value" || name == "UInt64Value") {
        return MessageType::kWrapper;
      }
      break;
    default:
      break;
  }

  return MessageType::kNotWellKnown;
}

// Helper alias templates to avoid needing to write `typename` in function
// signatures.
template <typename Traits>
using Field = typename Traits::Field;
template <typename Traits>
using Desc = typename Traits::Desc;

// Traits for proto2-ish descriptors.
struct Proto2Descriptor {
  // A descriptor for introspecting the fields of a message type.
  //
  // Desc<Traits> needs to be handled through a const Desc& in most (but not
  // all, in the case of ResolverTraits) cases, so we do not include the const*
  // annotation on this type.
  using Desc = Descriptor;

  // A field descriptor for introspecting a single field.
  //
  // Field<Traits> is always copyable, so this can be a pointer directly.
  using Field = const FieldDescriptor*;

  /// Functions for working with descriptors. ///

  static absl::string_view TypeName(const Desc& d) { return d.full_name(); }

  static absl::optional<Field> FieldByNumber(const Desc& d, int32_t number) {
    if (const auto* field = d.FindFieldByNumber(number)) {
      return field;
    }
    return absl::nullopt;
  }

  static Field MustHaveField(const Desc& d, int32_t number,
                             JsonLocation::SourceLocation loc =
                                 JsonLocation::SourceLocation::current()) {
    auto f = FieldByNumber(d, number);
    if (!f.has_value()) {
      ABSL_LOG(FATAL)
          << absl::StrFormat(
                 "%s has, by definition, a field numbered %d, but it could not "
                 "be "
                 "looked up; this is a bug",
                 TypeName(d), number);
    }
    return *f;
  }

  static absl::optional<Field> FieldByName(const Desc& d,
                                           absl::string_view name) {
    if (const auto* field = d.FindFieldByCamelcaseName(name)) {
      return field;
    }

    if (const auto* field = d.FindFieldByName(name)) {
      return field;
    }

    for (int i = 0; i < d.field_count(); ++i) {
      const auto* field = d.field(i);
      if (field->has_json_name() && field->json_name() == name) {
        return field;
      }
    }

    return absl::nullopt;
  }

  static Field KeyField(const Desc& d) { return d.map_key(); }

  static Field ValueField(const Desc& d) { return d.map_value(); }

  static size_t FieldCount(const Desc& d) { return d.field_count(); }

  static Field FieldByIndex(const Desc& d, size_t idx) { return d.field(idx); }

  static absl::optional<Field> ExtensionByName(const Desc& d,
                                               absl::string_view name) {
    auto* field = d.file()->pool()->FindExtensionByName(name);
    if (field == nullptr) {
      return absl::nullopt;
    }
    return field;
  }

  /// Functions for introspecting fields. ///

  static absl::string_view FieldName(Field f) { return f->name(); }
  static absl::string_view FieldJsonName(Field f) {
    return f->has_json_name() ? f->json_name() : f->camelcase_name();
  }
  static absl::string_view FieldFullName(Field f) { return f->full_name(); }

  static absl::string_view FieldTypeName(Field f) {
    if (f->type() == FieldDescriptor::TYPE_MESSAGE) {
      return f->message_type()->full_name();
    }
    if (f->type() == FieldDescriptor::TYPE_ENUM) {
      return f->enum_type()->full_name();
    }
    return "";
  }

  static FieldDescriptor::Type FieldType(Field f) { return f->type(); }

  static int32_t FieldNumber(Field f) { return f->number(); }

  static bool Is32Bit(Field f) {
    switch (f->cpp_type()) {
      case FieldDescriptor::CPPTYPE_UINT32:
      case FieldDescriptor::CPPTYPE_INT32:
      case FieldDescriptor::CPPTYPE_ENUM:
      case FieldDescriptor::CPPTYPE_FLOAT:
        return true;
      default:
        return false;
    }
  }

  static const Desc& ContainingType(Field f) { return *f->containing_type(); }

  static bool IsMap(Field f) { return f->is_map(); }

  static bool IsRepeated(Field f) { return f->is_repeated(); }

  static bool IsExplicitPresence(Field f) { return f->has_presence(); }

  static bool IsImplicitPresence(Field f) {
    return !f->is_repeated() && !f->has_presence();
  }

  static bool IsExtension(Field f) { return f->is_extension(); }

  static bool IsOneof(Field f) { return f->containing_oneof() != nullptr; }

  static absl::StatusOr<int32_t> EnumNumberByName(Field f,
                                                  absl::string_view name,
                                                  bool case_insensitive) {
    if (case_insensitive) {
      for (int i = 0; i < f->enum_type()->value_count(); ++i) {
        const auto* ev = f->enum_type()->value(i);
        if (absl::EqualsIgnoreCase(name, ev->name())) {
          return ev->number();
        }
      }
      return absl::InvalidArgumentError(
          absl::StrFormat("unknown enum value: '%s'", name));
    }

    if (const auto* ev = f->enum_type()->FindValueByName(name)) {
      return ev->number();
    }

    return absl::InvalidArgumentError(
        absl::StrFormat("unknown enum value: '%s'", name));
  }

  static absl::StatusOr<std::string> EnumNameByNumber(Field f, int32_t number) {
    if (const auto* ev = f->enum_type()->FindValueByNumber(number)) {
      return std::string(ev->name());
    }
    return absl::InvalidArgumentError(
        absl::StrFormat("unknown enum number: '%d'", number));
  }

  // Looks up the corresponding Desc for `f`'s type, if there is one, and
  // calls `body` with it.
  //
  // This needs to have this funny callback API since whether or not the
  // Descriptor equivalent is an owning type depends on the trait.
  template <typename F>
  static absl::Status WithFieldType(Field f, F body) {
    return body(*f->message_type());
  }

  // Like WithFieldType, but using dynamic lookup by type URL.
  template <typename F>
  static absl::Status WithDynamicType(const Desc& desc,
                                      const std::string& type_url, F body) {
    size_t slash = type_url.rfind('/');
    if (slash == absl::string_view::npos || slash == 0) {
      return absl::InvalidArgumentError(absl::StrCat(
          "@type must contain at least one / and a nonempty host; got: ",
          type_url));
    }
    absl::string_view type_name(type_url);
    type_name = type_name.substr(slash + 1);

    const Descriptor* dyn_desc =
        desc.file()->pool()->FindMessageTypeByName(type_name);
    if (dyn_desc == nullptr) {
      return absl::InvalidArgumentError(
          absl::StrFormat("could not find @type '%s'", type_url));
    }

    return body(*dyn_desc);
  }
};

// Traits for proto3-ish deserialization.
//
// See Proto2Descriptor for API docs.
struct Proto3Type {
  using Desc = ResolverPool::Message;
  using Field = const ResolverPool::Field*;

  /// Functions for working with descriptors. ///
  static absl::string_view TypeName(const Desc& d) { return d.proto().name(); }

  static absl::optional<Field> FieldByNumber(const Desc& d, int32_t number) {
    const auto* f = d.FindField(number);
    return f == nullptr ? absl::nullopt : absl::make_optional(f);
  }

  static Field MustHaveField(const Desc& d, int32_t number,
                             JsonLocation::SourceLocation loc =
                                 JsonLocation::SourceLocation::current()) {
    auto f = FieldByNumber(d, number);
    if (!f.has_value()) {
      ABSL_LOG(FATAL)
          << absl::StrFormat(
                 "%s has, by definition, a field numbered %d, but it could not "
                 "be "
                 "looked up; this is a bug",
                 TypeName(d), number);
    }
    return *f;
  }

  static absl::optional<Field> FieldByName(const Desc& d,
                                           absl::string_view name) {
    const auto* f = d.FindField(name);
    return f == nullptr ? absl::nullopt : absl::make_optional(f);
  }

  static Field KeyField(const Desc& d) { return &d.FieldsByIndex()[0]; }

  static Field ValueField(const Desc& d) { return &d.FieldsByIndex()[1]; }

  static size_t FieldCount(const Desc& d) { return d.proto().fields_size(); }

  static Field FieldByIndex(const Desc& d, size_t idx) {
    return &d.FieldsByIndex()[idx];
  }

  static absl::optional<Field> ExtensionByName(const Desc& d,
                                               absl::string_view name) {
    // type.proto cannot represent extensions, so this function always
    // fails.
    return absl::nullopt;
  }

  /// Functions for introspecting fields. ///

  static absl::string_view FieldName(Field f) { return f->proto().name(); }
  static absl::string_view FieldJsonName(Field f) {
    return f->proto().json_name();
  }
  static absl::string_view FieldFullName(Field f) { return f->proto().name(); }

  static absl::string_view FieldTypeName(Field f) {
    absl::string_view url = f->proto().type_url();

    // If there is no slash, `slash` is string_view::npos, which is guaranteed
    // to be -1.
    size_t slash = url.rfind('/');
    return url.substr(slash + 1);
  }

  static FieldDescriptor::Type FieldType(Field f) {
    // The descriptor.proto and type.proto field type enums are required to be
    // the same, so we leverage this.
    return static_cast<FieldDescriptor::Type>(f->proto().kind());
  }

  static int32_t FieldNumber(Field f) { return f->proto().number(); }

  static bool Is32Bit(Field f) {
    switch (f->proto().kind()) {
      case google::protobuf::Field::TYPE_INT32:
      case google::protobuf::Field::TYPE_SINT32:
      case google::protobuf::Field::TYPE_UINT32:
      case google::protobuf::Field::TYPE_FIXED32:
      case google::protobuf::Field::TYPE_SFIXED32:
      case google::protobuf::Field::TYPE_FLOAT:
        return true;
      default:
        return false;
    }
  }

  static const Desc& ContainingType(Field f) { return f->parent(); }
  static bool IsMap(Field f) {
    if (f->proto().kind() != google::protobuf::Field::TYPE_MESSAGE) {
      return false;
    }

    bool value = false;
    (void)WithFieldType(f, [&value](const Desc& desc) {
      value = absl::c_any_of(desc.proto().options(), [&](auto& option) {
        return option.name() == "map_entry";
      });
      return absl::OkStatus();
    });
    return value;
  }

  static bool IsRepeated(Field f) {
    return f->proto().cardinality() ==
           google::protobuf::Field::CARDINALITY_REPEATED;
  }

  static bool IsExplicitPresence(Field f) {
    // Implicit presence requires this weird check: in proto3 the following
    // cases support presence:
    // 1) Anything contained in a oneof (including things explicitly declared
    //    'optional' which are represented as as synthetic oneof in proto3).
    // 2) Fields that are a message type (but not map fields which are also
    //    TYPE_MESSAGE here).
    if (f->parent().proto().syntax() == google::protobuf::SYNTAX_PROTO3) {
      return f->proto().oneof_index() != 0 ||
             (f->proto().kind() == google::protobuf::Field::TYPE_MESSAGE &&
              !IsRepeated(f));
    }

    return f->proto().cardinality() ==
               google::protobuf::Field::CARDINALITY_OPTIONAL ||
           google::protobuf::Field::CARDINALITY_REQUIRED;
  }

  static bool IsImplicitPresence(Field f) {
    return !IsRepeated(f) && !IsExplicitPresence(f);
  }

  static bool IsExtension(Field f) { return false; }

  static bool IsOneof(Field f) { return f->proto().oneof_index() != 0; }

  static absl::StatusOr<int32_t> EnumNumberByName(Field f,
                                                  absl::string_view name,
                                                  bool case_insensitive) {
    auto e = f->EnumType();
    RETURN_IF_ERROR(e.status());

    for (const auto& ev : (**e).proto().enumvalue()) {
      if (case_insensitive) {
        // Two ifs to avoid doing operator== twice if the names are not equal.
        if (absl::EqualsIgnoreCase(ev.name(), name)) {
          return ev.number();
        }
      } else if (ev.name() == name) {
        return ev.number();
      }
    }
    return absl::InvalidArgumentError(
        absl::StrFormat("unknown enum value: '%s'", name));
  }

  static absl::StatusOr<std::string> EnumNameByNumber(Field f, int32_t number) {
    auto e = f->EnumType();
    RETURN_IF_ERROR(e.status());

    for (const auto& ev : (**e).proto().enumvalue()) {
      if (ev.number() == number) {
        return ev.name();
      }
    }
    return absl::InvalidArgumentError(
        absl::StrFormat("unknown enum number: '%d'", number));
  }

  template <typename F>
  static absl::Status WithFieldType(Field f, F body) {
    auto m = f->MessageType();
    RETURN_IF_ERROR(m.status());
    return body(**m);
  }

  template <typename F>
  static absl::Status WithDynamicType(const Desc& desc,
                                      const std::string& type_url, F body) {
    auto dyn_desc = desc.pool()->FindMessage(type_url);
    RETURN_IF_ERROR(dyn_desc.status());
    return body(**dyn_desc);
  }
};
}  // namespace json_internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_JSON_INTERNAL_DESCRIPTOR_TRAITS_INTERNAL_H__
