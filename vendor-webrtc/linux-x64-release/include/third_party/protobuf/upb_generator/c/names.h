// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef THIRD_PARTY_UPB_UPB_GENERATOR_C_NAMES_H_
#define THIRD_PARTY_UPB_UPB_GENERATOR_C_NAMES_H_

#include <string>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"

namespace upb {
namespace generator {

// Note: these names are not currently exported, in hopes that no code
// generators outside of the protobuf repo will ever use the generated C API.

// Maps: foo/bar/baz.proto -> foo/bar/baz.upb.h
std::string CApiHeaderFilename(absl::string_view proto_filename);

// The foo.upb.h file defines far more symbols than we currently enumerate here.
// We do the bare minimum by by defining the type name for messages and enums,
// which also forms the symbol prefix for associated functions.
//
//   typedef struct { /* ... */ } <MessageType>;
//   typedef enum { <EnumValue> = X, ... } <EnumType>;
//
// Oneofs and extensions have a base name that forms the prefix for associated
// functions.

std::string CApiMessageType(absl::string_view full_name);
std::string CApiEnumType(absl::string_view full_name);
std::string CApiEnumValueSymbol(absl::string_view full_name);
std::string CApiExtensionIdentBase(absl::string_view full_name);
std::string CApiOneofIdentBase(absl::string_view full_name);

// Name mangling for individual fields. NameMangler maps each field name to a
// mangled name, which tries to avoid collisions with other field accessors.
//
// For example, a field named "clear_foo" might be renamed to "clear_foo_" if
// there is a field named "foo" in the same message.
//
// This API would be more principled if it generated a full symbol name for each
// generated API function, eg.
//   mangler.GetSetter("clear_foo") -> "mypkg_MyMessage_set_clear_foo_"
//   mangler.GetHazzer("clear_foo") -> "mypkg_MyMessage_has_clear_foo_"
//
// But that would be a larger and more complicated API. In the long run, we
// probably don't want to have other code generators wrapping these APIs, so
// it's probably not worth designing a fully principled API.

enum FieldClass {
  kStringField = 1 << 0,
  kContainerField = 1 << 1,
  kOtherField = 1 << 2,
};

class NameMangler {
 public:
  explicit NameMangler(
      const absl::flat_hash_map<std::string, FieldClass>& fields);

  std::string ResolveFieldName(absl::string_view name) const {
    auto it = names_.find(name);
    return it == names_.end() ? std::string(name) : it->second;
  }

 private:
  // Maps field_name -> mangled_name.  If a field name is not in the map, it
  // is not mangled.
  absl::flat_hash_map<std::string, std::string> names_;
};

// Here we provide functions for building field lists from both C++ and upb
// reflection.  They are templated so as to not actually introduce dependencies
// on either C++ or upb.

template <class T>
absl::flat_hash_map<std::string, FieldClass> GetCppFields(const T* descriptor) {
  absl::flat_hash_map<std::string, FieldClass> fields;
  for (int i = 0; i < descriptor->field_count(); ++i) {
    const auto* field = descriptor->field(i);
    if (field->is_repeated() || field->is_map()) {
      fields.emplace(field->name(), kContainerField);
    } else if (field->cpp_type() == field->CPPTYPE_STRING) {
      fields.emplace(field->name(), kStringField);
    } else {
      fields.emplace(field->name(), kOtherField);
    }
  }
  return fields;
}

template <class T>
absl::flat_hash_map<std::string, FieldClass> GetUpbFields(const T& msg_def) {
  absl::flat_hash_map<std::string, FieldClass> fields;
  for (const auto field : msg_def.fields()) {
    if (field.IsSequence() || field.IsMap()) {
      fields.emplace(field.name(), kContainerField);
    } else if (field.ctype() == decltype(field)::CType::kUpb_CType_String) {
      fields.emplace(field.name(), kStringField);
    } else {
      fields.emplace(field.name(), kOtherField);
    }
  }
  return fields;
}

ABSL_CONST_INIT const absl::string_view kRepeatedFieldArrayGetterPostfix =
    "upb_array";
ABSL_CONST_INIT const absl::string_view
    kRepeatedFieldMutableArrayGetterPostfix = "mutable_upb_array";

ABSL_CONST_INIT const absl::string_view kMapGetterPostfix = "upb_map";
ABSL_CONST_INIT const absl::string_view kMutableMapGetterPostfix =
    "mutable_upb_map";

}  // namespace generator
}  // namespace upb

#endif  // THIRD_PARTY_UPB_UPB_GENERATOR_C_NAMES_H_
