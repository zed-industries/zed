/*
 * Copyright 2014 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_IDL_H_
#define FLATBUFFERS_IDL_H_

#include <algorithm>
#include <functional>
#include <map>
#include <memory>
#include <stack>
#include <vector>

#include "flatbuffers/base.h"
#include "flatbuffers/flatbuffers.h"
#include "flatbuffers/flexbuffers.h"
#include "flatbuffers/hash.h"
#include "flatbuffers/reflection.h"

// This file defines the data types representing a parsed IDL (Interface
// Definition Language) / schema file.

// Limits maximum depth of nested objects.
// Prevents stack overflow while parse scheme, or json, or flexbuffer.
#if !defined(FLATBUFFERS_MAX_PARSING_DEPTH)
#  define FLATBUFFERS_MAX_PARSING_DEPTH 64
#endif

namespace flatbuffers {

// The order of these matters for Is*() functions below.
// Additionally, Parser::ParseType assumes bool..string is a contiguous range
// of type tokens.
// clang-format off
#define FLATBUFFERS_GEN_TYPES_SCALAR(TD) \
  TD(NONE,     "",       uint8_t,  byte,   byte,    byte,   uint8,   u8,   UByte, UInt8, 0) \
  TD(UTYPE,    "",       uint8_t,  byte,   byte,    byte,   uint8,   u8,   UByte, UInt8, 1) /* begin scalar/int */ \
  TD(BOOL,     "bool",   uint8_t,  boolean,bool,    bool,   bool,    bool, Boolean, Bool, 2) \
  TD(CHAR,     "byte",   int8_t,   byte,   int8,    sbyte,  int8,    i8,   Byte, Int8, 3) \
  TD(UCHAR,    "ubyte",  uint8_t,  byte,   byte,    byte,   uint8,   u8,   UByte, UInt8, 4) \
  TD(SHORT,    "short",  int16_t,  short,  int16,   short,  int16,   i16,  Short, Int16, 5) \
  TD(USHORT,   "ushort", uint16_t, short,  uint16,  ushort, uint16,  u16,  UShort, UInt16, 6) \
  TD(INT,      "int",    int32_t,  int,    int32,   int,    int32,   i32,  Int, Int32, 7) \
  TD(UINT,     "uint",   uint32_t, int,    uint32,  uint,   uint32,  u32,  UInt, UInt32, 8) \
  TD(LONG,     "long",   int64_t,  long,   int64,   long,   int64,   i64,  Long, Int64, 9) \
  TD(ULONG,    "ulong",  uint64_t, long,   uint64,  ulong,  uint64,  u64,  ULong, UInt64, 10) /* end int */ \
  TD(FLOAT,    "float",  float,    float,  float32, float,  float32, f32,  Float, Float32, 11) /* begin float */ \
  TD(DOUBLE,   "double", double,   double, float64, double, float64, f64,  Double, Double, 12) /* end float/scalar */
#define FLATBUFFERS_GEN_TYPES_POINTER(TD) \
  TD(STRING,   "string", Offset<void>,   int, int, StringOffset, int, unused, Int, Offset<String>, 13) \
  TD(VECTOR,   "",       Offset<void>,   int, int, VectorOffset, int, unused, Int, Offset<UOffset>, 14) \
  TD(VECTOR64, "",       Offset64<void>, int, int, VectorOffset, int, unused, Int, Offset<UOffset>, 18) \
  TD(STRUCT,   "",       Offset<void>,   int, int, int,          int, unused, Int, Offset<UOffset>, 15) \
  TD(UNION,    "",       Offset<void>,   int, int, int,          int, unused, Int, Offset<UOffset>, 16)
#define FLATBUFFERS_GEN_TYPE_ARRAY(TD) \
  TD(ARRAY,    "",       int,            int, int, int,          int, unused, Int, Offset<UOffset>, 17)
// The fields are:
// - enum
// - FlatBuffers schema type.
// - C++ type.
// - Java type.
// - Go type.
// - C# / .Net type.
// - Python type.
// - Kotlin type.
// - Rust type.
// - Swift type.
// - enum value (matches the reflected values)

// using these macros, we can now write code dealing with types just once, e.g.

/*
switch (type) {
  #define FLATBUFFERS_TD(ENUM, IDLTYPE, CTYPE, JTYPE, GTYPE, NTYPE, PTYPE, \
                         RTYPE, KTYPE, STYPE, ...) \
    case BASE_TYPE_ ## ENUM: \
      // do something specific to CTYPE here
    FLATBUFFERS_GEN_TYPES(FLATBUFFERS_TD)
  #undef FLATBUFFERS_TD
}
*/

// If not all FLATBUFFERS_GEN_() arguments are necessary for implementation
// of FLATBUFFERS_TD, you can use a variadic macro (with __VA_ARGS__ if needed).
// In the above example, only CTYPE is used to generate the code, it can be rewritten:

/*
switch (type) {
  #define FLATBUFFERS_TD(ENUM, IDLTYPE, CTYPE, ...) \
    case BASE_TYPE_ ## ENUM: \
      // do something specific to CTYPE here
    FLATBUFFERS_GEN_TYPES(FLATBUFFERS_TD)
  #undef FLATBUFFERS_TD
}
*/

#define FLATBUFFERS_GEN_TYPES(TD) \
        FLATBUFFERS_GEN_TYPES_SCALAR(TD) \
        FLATBUFFERS_GEN_TYPES_POINTER(TD) \
        FLATBUFFERS_GEN_TYPE_ARRAY(TD)

// Create an enum for all the types above.
#ifdef __GNUC__
__extension__  // Stop GCC complaining about trailing comma with -Wpendantic.
#endif
enum BaseType {
  #define FLATBUFFERS_TD(ENUM, IDLTYPE, \
              CTYPE, JTYPE, GTYPE, NTYPE, PTYPE, RTYPE, KTYPE, STYPE, ENUM_VALUE) \
    BASE_TYPE_ ## ENUM = ENUM_VALUE,
    FLATBUFFERS_GEN_TYPES(FLATBUFFERS_TD)
  #undef FLATBUFFERS_TD
};

#define FLATBUFFERS_TD(ENUM, IDLTYPE, CTYPE, ...) \
  static_assert(sizeof(CTYPE) <= sizeof(largest_scalar_t), \
                "define largest_scalar_t as " #CTYPE);
  FLATBUFFERS_GEN_TYPES(FLATBUFFERS_TD)
#undef FLATBUFFERS_TD

inline bool IsScalar (BaseType t) { return t >= BASE_TYPE_UTYPE &&
                                           t <= BASE_TYPE_DOUBLE; }
inline bool IsInteger(BaseType t) { return t >= BASE_TYPE_UTYPE &&
                                           t <= BASE_TYPE_ULONG; }
inline bool IsFloat  (BaseType t) { return t == BASE_TYPE_FLOAT ||
                                           t == BASE_TYPE_DOUBLE; }
inline bool IsLong   (BaseType t) { return t == BASE_TYPE_LONG ||
                                           t == BASE_TYPE_ULONG; }
inline bool IsBool   (BaseType t) { return t == BASE_TYPE_BOOL; }
inline bool IsOneByte(BaseType t) { return t >= BASE_TYPE_UTYPE &&
                                           t <= BASE_TYPE_UCHAR; }
inline bool IsVector (BaseType t) { return t == BASE_TYPE_VECTOR ||
                                           t == BASE_TYPE_VECTOR64; }

inline bool IsUnsigned(BaseType t) {
  return (t == BASE_TYPE_UTYPE)  || (t == BASE_TYPE_UCHAR) ||
         (t == BASE_TYPE_USHORT) || (t == BASE_TYPE_UINT)  ||
         (t == BASE_TYPE_ULONG);
}

inline size_t SizeOf(const BaseType t) {
  switch (t) {
  #define FLATBUFFERS_TD(ENUM, IDLTYPE, CTYPE, ...) \
    case BASE_TYPE_##ENUM: return sizeof(CTYPE);
      FLATBUFFERS_GEN_TYPES(FLATBUFFERS_TD)
  #undef FLATBUFFERS_TD
    default: FLATBUFFERS_ASSERT(0);
  }
  return 0;
}

inline const char* TypeName(const BaseType t) {
  switch (t) {
  #define FLATBUFFERS_TD(ENUM, IDLTYPE, ...) \
    case BASE_TYPE_##ENUM: return IDLTYPE;
      FLATBUFFERS_GEN_TYPES(FLATBUFFERS_TD)
  #undef FLATBUFFERS_TD
    default: FLATBUFFERS_ASSERT(0);
  }
  return nullptr;
}

inline const char* StringOf(const BaseType t) {
  switch (t) {
  #define FLATBUFFERS_TD(ENUM, IDLTYPE, CTYPE, ...) \
    case BASE_TYPE_##ENUM: return #CTYPE;
      FLATBUFFERS_GEN_TYPES(FLATBUFFERS_TD)
  #undef FLATBUFFERS_TD
    default: FLATBUFFERS_ASSERT(0);
  }
  return "";
}

// clang-format on

struct StructDef;
struct EnumDef;
class Parser;

// Represents any type in the IDL, which is a combination of the BaseType
// and additional information for vectors/structs_.
struct Type {
  explicit Type(BaseType _base_type = BASE_TYPE_NONE, StructDef *_sd = nullptr,
                EnumDef *_ed = nullptr, uint16_t _fixed_length = 0)
      : base_type(_base_type),
        element(BASE_TYPE_NONE),
        struct_def(_sd),
        enum_def(_ed),
        fixed_length(_fixed_length) {}

  bool operator==(const Type &o) const {
    return base_type == o.base_type && element == o.element &&
           struct_def == o.struct_def && enum_def == o.enum_def;
  }

  Type VectorType() const {
    return Type(element, struct_def, enum_def, fixed_length);
  }

  Offset<reflection::Type> Serialize(FlatBufferBuilder *builder) const;

  bool Deserialize(const Parser &parser, const reflection::Type *type);

  BaseType base_type;
  BaseType element;       // only set if t == BASE_TYPE_VECTOR or
                          // BASE_TYPE_VECTOR64
  StructDef *struct_def;  // only set if t or element == BASE_TYPE_STRUCT
  EnumDef *enum_def;      // set if t == BASE_TYPE_UNION / BASE_TYPE_UTYPE,
                          // or for an integral type derived from an enum.
  uint16_t fixed_length;  // only set if t == BASE_TYPE_ARRAY
};

// Represents a parsed scalar value, it's type, and field offset.
struct Value {
  Value()
      : constant("0"),
        offset(static_cast<voffset_t>(~(static_cast<voffset_t>(0U)))) {}
  Type type;
  std::string constant;
  voffset_t offset;
};

// Helper class that retains the original order of a set of identifiers and
// also provides quick lookup.
template<typename T> class SymbolTable {
 public:
  ~SymbolTable() {
    for (auto it = vec.begin(); it != vec.end(); ++it) { delete *it; }
  }

  bool Add(const std::string &name, T *e) {
    vec.emplace_back(e);
    auto it = dict.find(name);
    if (it != dict.end()) return true;
    dict[name] = e;
    return false;
  }

  void Move(const std::string &oldname, const std::string &newname) {
    auto it = dict.find(oldname);
    if (it != dict.end()) {
      auto obj = it->second;
      dict.erase(it);
      dict[newname] = obj;
    } else {
      FLATBUFFERS_ASSERT(false);
    }
  }

  T *Lookup(const std::string &name) const {
    auto it = dict.find(name);
    return it == dict.end() ? nullptr : it->second;
  }

 public:
  std::map<std::string, T *> dict;  // quick lookup
  std::vector<T *> vec;             // Used to iterate in order of insertion
};

// A name space, as set in the schema.
struct Namespace {
  Namespace() : from_table(0) {}

  // Given a (potentially unqualified) name, return the "fully qualified" name
  // which has a full namespaced descriptor.
  // With max_components you can request less than the number of components
  // the current namespace has.
  std::string GetFullyQualifiedName(const std::string &name,
                                    size_t max_components = 1000) const;

  std::vector<std::string> components;
  size_t from_table;  // Part of the namespace corresponds to a message/table.
};

inline bool operator<(const Namespace &a, const Namespace &b) {
  size_t min_size = std::min(a.components.size(), b.components.size());
  for (size_t i = 0; i < min_size; ++i) {
    if (a.components[i] != b.components[i])
      return a.components[i] < b.components[i];
  }
  return a.components.size() < b.components.size();
}

// Base class for all definition types (fields, structs_, enums_).
struct Definition {
  Definition()
      : generated(false),
        defined_namespace(nullptr),
        serialized_location(0),
        index(-1),
        refcount(1),
        declaration_file(nullptr) {}

  flatbuffers::Offset<
      flatbuffers::Vector<flatbuffers::Offset<reflection::KeyValue>>>
  SerializeAttributes(FlatBufferBuilder *builder, const Parser &parser) const;

  bool DeserializeAttributes(Parser &parser,
                             const Vector<Offset<reflection::KeyValue>> *attrs);

  std::string name;
  std::string file;
  std::vector<std::string> doc_comment;
  SymbolTable<Value> attributes;
  bool generated;  // did we already output code for this definition?
  Namespace *defined_namespace;  // Where it was defined.

  // For use with Serialize()
  uoffset_t serialized_location;
  int index;  // Inside the vector it is stored.
  int refcount;
  const std::string *declaration_file;
};

struct FieldDef : public Definition {
  FieldDef()
      : deprecated(false),
        key(false),
        shared(false),
        native_inline(false),
        flexbuffer(false),
        offset64(false),
        presence(kDefault),
        nested_flatbuffer(nullptr),
        padding(0),
        sibling_union_field(nullptr) {}

  Offset<reflection::Field> Serialize(FlatBufferBuilder *builder, uint16_t id,
                                      const Parser &parser) const;

  bool Deserialize(Parser &parser, const reflection::Field *field);

  bool IsScalarOptional() const {
    return IsScalar() && IsOptional();
  }
  bool IsScalar() const {
      return ::flatbuffers::IsScalar(value.type.base_type);
  }
  bool IsOptional() const { return presence == kOptional; }
  bool IsRequired() const { return presence == kRequired; }
  bool IsDefault() const { return presence == kDefault; }

  Value value;
  bool deprecated;  // Field is allowed to be present in old data, but can't be.
                    // written in new data nor accessed in new code.
  bool key;         // Field functions as a key for creating sorted vectors.
  bool shared;  // Field will be using string pooling (i.e. CreateSharedString)
                // as default serialization behavior if field is a string.
  bool native_inline;  // Field will be defined inline (instead of as a pointer)
                       // for native tables if field is a struct.
  bool flexbuffer;     // This field contains FlexBuffer data.
  bool offset64;       // If the field uses 64-bit offsets.

  enum Presence {
    // Field must always be present.
    kRequired,
    // Non-presence should be signalled to and controlled by users.
    kOptional,
    // Non-presence is hidden from users.
    // Implementations may omit writing default values.
    kDefault,
  };
  Presence static MakeFieldPresence(bool optional, bool required) {
    FLATBUFFERS_ASSERT(!(required && optional));
    // clang-format off
    return required ? FieldDef::kRequired
         : optional ? FieldDef::kOptional
                    : FieldDef::kDefault;
    // clang-format on
  }
  Presence presence;

  StructDef *nested_flatbuffer;  // This field contains nested FlatBuffer data.
  size_t padding;                // Bytes to always pad after this field.

  // sibling_union_field is always set to nullptr. The only exception is
  // when FieldDef is a union field or an union type field. Therefore,
  // sibling_union_field on a union field points to the union type field
  // and vice-versa.
  FieldDef *sibling_union_field;
};

struct StructDef : public Definition {
  StructDef()
      : fixed(false),
        predecl(true),
        sortbysize(true),
        has_key(false),
        minalign(1),
        bytesize(0) {}

  void PadLastField(size_t min_align) {
    auto padding = PaddingBytes(bytesize, min_align);
    bytesize += padding;
    if (fields.vec.size()) fields.vec.back()->padding = padding;
  }

  Offset<reflection::Object> Serialize(FlatBufferBuilder *builder,
                                       const Parser &parser) const;

  bool Deserialize(Parser &parser, const reflection::Object *object);

  SymbolTable<FieldDef> fields;

  bool fixed;       // If it's struct, not a table.
  bool predecl;     // If it's used before it was defined.
  bool sortbysize;  // Whether fields come in the declaration or size order.
  bool has_key;     // It has a key field.
  size_t minalign;  // What the whole object needs to be aligned to.
  size_t bytesize;  // Size if fixed.

  flatbuffers::unique_ptr<std::string> original_location;
  std::vector<voffset_t> reserved_ids;
};

struct EnumDef;
struct EnumValBuilder;

struct EnumVal {
  Offset<reflection::EnumVal> Serialize(FlatBufferBuilder *builder,
                                        const Parser &parser) const;

  bool Deserialize(Parser &parser, const reflection::EnumVal *val);

  flatbuffers::Offset<
      flatbuffers::Vector<flatbuffers::Offset<reflection::KeyValue>>>
  SerializeAttributes(FlatBufferBuilder *builder, const Parser &parser) const;

  bool DeserializeAttributes(Parser &parser,
                             const Vector<Offset<reflection::KeyValue>> *attrs);

  uint64_t GetAsUInt64() const { return static_cast<uint64_t>(value); }
  int64_t GetAsInt64() const { return value; }
  bool IsZero() const { return 0 == value; }
  bool IsNonZero() const { return !IsZero(); }

  std::string name;
  std::vector<std::string> doc_comment;
  Type union_type;
  SymbolTable<Value> attributes;

 private:
  friend EnumDef;
  friend EnumValBuilder;
  friend bool operator==(const EnumVal &lhs, const EnumVal &rhs);

  EnumVal(const std::string &_name, int64_t _val) : name(_name), value(_val) {}
  EnumVal() : value(0) {}

  int64_t value;
};

struct EnumDef : public Definition {
  EnumDef() : is_union(false), uses_multiple_type_instances(false) {}

  Offset<reflection::Enum> Serialize(FlatBufferBuilder *builder,
                                     const Parser &parser) const;

  bool Deserialize(Parser &parser, const reflection::Enum *values);

  template<typename T> void ChangeEnumValue(EnumVal *ev, T new_val);
  void SortByValue();
  void RemoveDuplicates();

  std::string AllFlags() const;
  const EnumVal *MinValue() const;
  const EnumVal *MaxValue() const;
  // Returns the number of integer steps from v1 to v2.
  uint64_t Distance(const EnumVal *v1, const EnumVal *v2) const;
  // Returns the number of integer steps from Min to Max.
  uint64_t Distance() const { return Distance(MinValue(), MaxValue()); }

  EnumVal *ReverseLookup(int64_t enum_idx,
                         bool skip_union_default = false) const;
  EnumVal *FindByValue(const std::string &constant) const;

  std::string ToString(const EnumVal &ev) const {
    return IsUInt64() ? NumToString(ev.GetAsUInt64())
                      : NumToString(ev.GetAsInt64());
  }

  size_t size() const { return vals.vec.size(); }

  const std::vector<EnumVal *> &Vals() const { return vals.vec; }

  const EnumVal *Lookup(const std::string &enum_name) const {
    return vals.Lookup(enum_name);
  }

  bool is_union;
  // Type is a union which uses type aliases where at least one type is
  // available under two different names.
  bool uses_multiple_type_instances;
  Type underlying_type;

 private:
  bool IsUInt64() const {
    return (BASE_TYPE_ULONG == underlying_type.base_type);
  }

  friend EnumValBuilder;
  SymbolTable<EnumVal> vals;
};

inline bool IsString(const Type &type) {
  return type.base_type == BASE_TYPE_STRING;
}

inline bool IsStruct(const Type &type) {
  return type.base_type == BASE_TYPE_STRUCT && type.struct_def->fixed;
}

inline bool IsIncompleteStruct(const Type &type) {
  return type.base_type == BASE_TYPE_STRUCT && type.struct_def->predecl;
}

inline bool IsTable(const Type &type) {
  return type.base_type == BASE_TYPE_STRUCT && !type.struct_def->fixed;
}

inline bool IsUnion(const Type &type) {
  return type.enum_def != nullptr && type.enum_def->is_union;
}

inline bool IsUnionType(const Type &type) {
  return IsUnion(type) && IsInteger(type.base_type);
}

inline bool IsVector(const Type &type) { return IsVector(type.base_type); }

inline bool IsVectorOfStruct(const Type &type) {
  return IsVector(type) && IsStruct(type.VectorType());
}

inline bool IsVectorOfTable(const Type &type) {
  return IsVector(type) && IsTable(type.VectorType());
}

inline bool IsArray(const Type &type) {
  return type.base_type == BASE_TYPE_ARRAY;
}

inline bool IsSeries(const Type &type) {
  return IsVector(type) || IsArray(type);
}

inline bool IsEnum(const Type &type) {
  return type.enum_def != nullptr && IsInteger(type.base_type);
}

inline size_t InlineSize(const Type &type) {
  return IsStruct(type)
             ? type.struct_def->bytesize
             : (IsArray(type)
                    ? InlineSize(type.VectorType()) * type.fixed_length
                    : SizeOf(type.base_type));
}

inline size_t InlineAlignment(const Type &type) {
  if (IsStruct(type)) {
    return type.struct_def->minalign;
  } else if (IsArray(type)) {
    return IsStruct(type.VectorType()) ? type.struct_def->minalign
                                       : SizeOf(type.element);
  } else {
    return SizeOf(type.base_type);
  }
}
inline bool operator==(const EnumVal &lhs, const EnumVal &rhs) {
  return lhs.value == rhs.value;
}
inline bool operator!=(const EnumVal &lhs, const EnumVal &rhs) {
  return !(lhs == rhs);
}

inline bool EqualByName(const Type &a, const Type &b) {
  return a.base_type == b.base_type && a.element == b.element &&
         (a.struct_def == b.struct_def ||
          (a.struct_def != nullptr && b.struct_def != nullptr &&
           a.struct_def->name == b.struct_def->name)) &&
         (a.enum_def == b.enum_def ||
          (a.enum_def != nullptr && b.enum_def != nullptr &&
           a.enum_def->name == b.enum_def->name));
}

struct RPCCall : public Definition {
  Offset<reflection::RPCCall> Serialize(FlatBufferBuilder *builder,
                                        const Parser &parser) const;

  bool Deserialize(Parser &parser, const reflection::RPCCall *call);

  StructDef *request, *response;
};

struct ServiceDef : public Definition {
  Offset<reflection::Service> Serialize(FlatBufferBuilder *builder,
                                        const Parser &parser) const;
  bool Deserialize(Parser &parser, const reflection::Service *service);

  SymbolTable<RPCCall> calls;
};

struct IncludedFile {
  // The name of the schema file being included, as defined in the .fbs file.
  // This includes the prefix (e.g., include "foo/bar/baz.fbs" would mean this
  // value is "foo/bar/baz.fbs").
  std::string schema_name;

  // The filename of where the included file was found, after searching the
  // relative paths plus any other paths included with `flatc -I ...`. Note,
  // while this is sometimes the same as schema_name, it is not always, since it
  // can be defined relative to where flatc was invoked.
  std::string filename;
};

// Since IncludedFile is contained within a std::set, need to provide ordering.
inline bool operator<(const IncludedFile &a, const IncludedFile &b) {
  return a.filename < b.filename;
}

// Container of options that may apply to any of the source/text generators.
struct IDLOptions {
  // field case style options for C++
  enum CaseStyle { CaseStyle_Unchanged = 0, CaseStyle_Upper, CaseStyle_Lower };
  enum class ProtoIdGapAction { NO_OP, WARNING, ERROR };
  bool gen_jvmstatic;
  // Use flexbuffers instead for binary and text generation
  bool use_flexbuffers;
  bool strict_json;
  bool output_default_scalars_in_json;
  int indent_step;
  bool cpp_minify_enums;
  bool output_enum_identifiers;
  bool prefixed_enums;
  bool scoped_enums;
  bool emit_min_max_enum_values;
  bool swift_implementation_only;
  bool include_dependence_headers;
  bool mutable_buffer;
  bool one_file;
  bool proto_mode;
  bool proto_oneof_union;
  bool generate_all;
  bool skip_unexpected_fields_in_json;
  bool generate_name_strings;
  bool generate_object_based_api;
  bool gen_compare;
  std::string cpp_object_api_pointer_type;
  std::string cpp_object_api_string_type;
  bool cpp_object_api_string_flexible_constructor;
  CaseStyle cpp_object_api_field_case_style;
  bool cpp_direct_copy;
  bool gen_nullable;
  std::string java_package_prefix;
  bool java_checkerframework;
  bool gen_generated;
  bool gen_json_coders;
  std::string object_prefix;
  std::string object_suffix;
  bool union_value_namespacing;
  bool allow_non_utf8;
  bool natural_utf8;
  std::string include_prefix;
  bool keep_prefix;
  bool binary_schema_comments;
  bool binary_schema_builtins;
  bool binary_schema_gen_embed;
  bool binary_schema_absolute_paths;
  std::string go_import;
  std::string go_namespace;
  std::string go_module_name;
  bool protobuf_ascii_alike;
  bool size_prefixed;
  std::string root_type;
  bool force_defaults;
  bool java_primitive_has_method;
  bool cs_gen_json_serializer;
  std::vector<std::string> cpp_includes;
  std::string cpp_std;
  bool cpp_static_reflection;
  std::string proto_namespace_suffix;
  std::string filename_suffix;
  std::string filename_extension;
  bool no_warnings;
  bool warnings_as_errors;
  std::string project_root;
  bool cs_global_alias;
  bool json_nested_flatbuffers;
  bool json_nested_flexbuffers;
  bool json_nested_legacy_flatbuffers;
  bool ts_flat_files;
  bool ts_entry_points;
  bool ts_no_import_ext;
  bool no_leak_private_annotations;
  bool require_json_eof;
  bool keep_proto_id;

  /********************************** Python **********************************/
  bool python_no_type_prefix_suffix;
  bool python_typing;

  // The target Python version. Can be one of the following:
  // -  "0"
  // -  "2"
  // -  "3"
  // -  "2.<minor>"
  // -  "3.<minor>"
  // -  "2.<minor>.<micro>"
  // -  "3.<minor>.<micro>"
  //
  // https://docs.python.org/3/faq/general.html#how-does-the-python-version-numbering-scheme-work
  std::string python_version;

  // Whether to generate numpy helpers.
  bool python_gen_numpy;

  bool ts_omit_entrypoint;
  ProtoIdGapAction proto_id_gap_action;

  // Possible options for the more general generator below.
  enum Language {
    kJava = 1 << 0,
    kCSharp = 1 << 1,
    kGo = 1 << 2,
    kCpp = 1 << 3,
    kPython = 1 << 5,
    kPhp = 1 << 6,
    kJson = 1 << 7,
    kBinary = 1 << 8,
    kTs = 1 << 9,
    kJsonSchema = 1 << 10,
    kDart = 1 << 11,
    kLua = 1 << 12,
    kLobster = 1 << 13,
    kRust = 1 << 14,
    kKotlin = 1 << 15,
    kSwift = 1 << 16,
    kNim = 1 << 17,
    kProto = 1 << 18,
    kKotlinKmp = 1 << 19,
    kMAX
  };

  enum MiniReflect { kNone, kTypes, kTypesAndNames };

  MiniReflect mini_reflect;

  // If set, require all fields in a table to be explicitly numbered.
  bool require_explicit_ids;

  // If set, implement serde::Serialize for generated Rust types
  bool rust_serialize;

  // If set, generate rust types in individual files with a root module file.
  bool rust_module_root_file;

  // The corresponding language bit will be set if a language is included
  // for code generation.
  unsigned long lang_to_generate;

  // If set (default behavior), empty string fields will be set to nullptr to
  // make the flatbuffer more compact.
  bool set_empty_strings_to_null;

  // If set (default behavior), empty vector fields will be set to nullptr to
  // make the flatbuffer more compact.
  bool set_empty_vectors_to_null;

  /*********************************** gRPC ***********************************/
  std::string grpc_filename_suffix;
  bool grpc_use_system_headers;
  std::string grpc_search_path;
  std::vector<std::string> grpc_additional_headers;

  /******************************* Python gRPC ********************************/
  bool grpc_python_typed_handlers;

  IDLOptions()
      : gen_jvmstatic(false),
        use_flexbuffers(false),
        strict_json(false),
        output_default_scalars_in_json(false),
        indent_step(2),
        cpp_minify_enums(false),
        output_enum_identifiers(true),
        prefixed_enums(true),
        scoped_enums(false),
        emit_min_max_enum_values(true),
        swift_implementation_only(false),
        include_dependence_headers(true),
        mutable_buffer(false),
        one_file(false),
        proto_mode(false),
        proto_oneof_union(false),
        generate_all(false),
        skip_unexpected_fields_in_json(false),
        generate_name_strings(false),
        generate_object_based_api(false),
        gen_compare(false),
        cpp_object_api_pointer_type("std::unique_ptr"),
        cpp_object_api_string_flexible_constructor(false),
        cpp_object_api_field_case_style(CaseStyle_Unchanged),
        cpp_direct_copy(true),
        gen_nullable(false),
        java_checkerframework(false),
        gen_generated(false),
        gen_json_coders(false),
        object_suffix("T"),
        union_value_namespacing(true),
        allow_non_utf8(false),
        natural_utf8(false),
        keep_prefix(false),
        binary_schema_comments(false),
        binary_schema_builtins(false),
        binary_schema_gen_embed(false),
        binary_schema_absolute_paths(false),
        protobuf_ascii_alike(false),
        size_prefixed(false),
        force_defaults(false),
        java_primitive_has_method(false),
        cs_gen_json_serializer(false),
        cpp_static_reflection(false),
        filename_suffix("_generated"),
        filename_extension(),
        no_warnings(false),
        warnings_as_errors(false),
        project_root(""),
        cs_global_alias(false),
        json_nested_flatbuffers(true),
        json_nested_flexbuffers(true),
        json_nested_legacy_flatbuffers(false),
        ts_flat_files(false),
        ts_entry_points(false),
        ts_no_import_ext(false),
        no_leak_private_annotations(false),
        require_json_eof(true),
        keep_proto_id(false),
        python_no_type_prefix_suffix(false),
        python_typing(false),
        python_gen_numpy(true),
        ts_omit_entrypoint(false),
        proto_id_gap_action(ProtoIdGapAction::WARNING),
        mini_reflect(IDLOptions::kNone),
        require_explicit_ids(false),
        rust_serialize(false),
        rust_module_root_file(false),
        lang_to_generate(0),
        set_empty_strings_to_null(true),
        set_empty_vectors_to_null(true),
        grpc_filename_suffix(".fb"),
        grpc_use_system_headers(true),
        grpc_python_typed_handlers(false) {}
};

// This encapsulates where the parser is in the current source file.
struct ParserState {
  ParserState()
      : prev_cursor_(nullptr),
        cursor_(nullptr),
        line_start_(nullptr),
        line_(0),
        token_(-1),
        attr_is_trivial_ascii_string_(true) {}

 protected:
  void ResetState(const char *source) {
    prev_cursor_ = source;
    cursor_ = source;
    line_ = 0;
    MarkNewLine();
  }

  void MarkNewLine() {
    line_start_ = cursor_;
    line_ += 1;
  }

  int64_t CursorPosition() const {
    FLATBUFFERS_ASSERT(cursor_ && line_start_ && cursor_ >= line_start_);
    return static_cast<int64_t>(cursor_ - line_start_);
  }

  const char *prev_cursor_;
  const char *cursor_;
  const char *line_start_;
  int line_;  // the current line being parsed
  int token_;

  // Flag: text in attribute_ is true ASCII string without escape
  // sequences. Only printable ASCII (without [\t\r\n]).
  // Used for number-in-string (and base64 string in future).
  bool attr_is_trivial_ascii_string_;
  std::string attribute_;
  std::vector<std::string> doc_comment_;
};

// A way to make error propagation less error prone by requiring values to be
// checked.
// Once you create a value of this type you must either:
// - Call Check() on it.
// - Copy or assign it to another value.
// Failure to do so leads to an assert.
// This guarantees that this as return value cannot be ignored.
class CheckedError {
 public:
  explicit CheckedError(bool error)
      : is_error_(error), has_been_checked_(false) {}

  CheckedError &operator=(const CheckedError &other) {
    is_error_ = other.is_error_;
    has_been_checked_ = false;
    other.has_been_checked_ = true;
    return *this;
  }

  CheckedError(const CheckedError &other) {
    *this = other;  // Use assignment operator.
  }

  ~CheckedError() { FLATBUFFERS_ASSERT(has_been_checked_); }

  bool Check() {
    has_been_checked_ = true;
    return is_error_;
  }

 private:
  bool is_error_;
  mutable bool has_been_checked_;
};

// Additionally, in GCC we can get these errors statically, for additional
// assurance:
// clang-format off
#ifdef __GNUC__
#define FLATBUFFERS_CHECKED_ERROR CheckedError \
          __attribute__((warn_unused_result))
#else
#define FLATBUFFERS_CHECKED_ERROR CheckedError
#endif
// clang-format on

class Parser : public ParserState {
 public:
  explicit Parser(const IDLOptions &options = IDLOptions())
      : current_namespace_(nullptr),
        empty_namespace_(nullptr),
        flex_builder_(256, flexbuffers::BUILDER_FLAG_SHARE_ALL),
        root_struct_def_(nullptr),
        opts(options),
        uses_flexbuffers_(false),
        has_warning_(false),
        advanced_features_(0),
        source_(nullptr),
        anonymous_counter_(0),
        parse_depth_counter_(0) {
    if (opts.force_defaults) { builder_.ForceDefaults(true); }
    // Start out with the empty namespace being current.
    empty_namespace_ = new Namespace();
    namespaces_.push_back(empty_namespace_);
    current_namespace_ = empty_namespace_;
    known_attributes_["deprecated"] = true;
    known_attributes_["required"] = true;
    known_attributes_["key"] = true;
    known_attributes_["shared"] = true;
    known_attributes_["hash"] = true;
    known_attributes_["id"] = true;
    known_attributes_["force_align"] = true;
    known_attributes_["bit_flags"] = true;
    known_attributes_["original_order"] = true;
    known_attributes_["nested_flatbuffer"] = true;
    known_attributes_["csharp_partial"] = true;
    known_attributes_["streaming"] = true;
    known_attributes_["idempotent"] = true;
    known_attributes_["cpp_type"] = true;
    known_attributes_["cpp_ptr_type"] = true;
    known_attributes_["cpp_ptr_type_get"] = true;
    known_attributes_["cpp_str_type"] = true;
    known_attributes_["cpp_str_flex_ctor"] = true;
    known_attributes_["native_inline"] = true;
    known_attributes_["native_custom_alloc"] = true;
    known_attributes_["native_type"] = true;
    known_attributes_["native_type_pack_name"] = true;
    known_attributes_["native_default"] = true;
    known_attributes_["flexbuffer"] = true;
    known_attributes_["private"] = true;

    // An attribute added to a field to indicate that is uses 64-bit addressing.
    known_attributes_["offset64"] = true;

    // An attribute added to a vector field to indicate that it uses 64-bit
    // addressing and it has a 64-bit length.
    known_attributes_["vector64"] = true;
  }

  // Copying is not allowed
  Parser(const Parser &) = delete;
  Parser &operator=(const Parser &) = delete;

  Parser(Parser &&) = default;
  Parser &operator=(Parser &&) = default;

  ~Parser() {
    for (auto it = namespaces_.begin(); it != namespaces_.end(); ++it) {
      delete *it;
    }
  }

  // Parse the string containing either schema or JSON data, which will
  // populate the SymbolTable's or the FlatBufferBuilder above.
  // include_paths is used to resolve any include statements, and typically
  // should at least include the project path (where you loaded source_ from).
  // include_paths must be nullptr terminated if specified.
  // If include_paths is nullptr, it will attempt to load from the current
  // directory.
  // If the source was loaded from a file and isn't an include file,
  // supply its name in source_filename.
  // All paths specified in this call must be in posix format, if you accept
  // paths from user input, please call PosixPath on them first.
  bool Parse(const char *_source, const char **include_paths = nullptr,
             const char *source_filename = nullptr);

  bool ParseJson(const char *json, const char *json_filename = nullptr);

  // Returns the number of characters were consumed when parsing a JSON string.
  std::ptrdiff_t BytesConsumed() const;

  // Set the root type. May override the one set in the schema.
  bool SetRootType(const char *name);

  // Mark all definitions as already having code generated.
  void MarkGenerated();

  // Get the files recursively included by the given file. The returned
  // container will have at least the given file.
  std::set<std::string> GetIncludedFilesRecursive(
      const std::string &file_name) const;

  // Fills builder_ with a binary version of the schema parsed.
  // See reflection/reflection.fbs
  void Serialize();

  // Deserialize a schema buffer
  bool Deserialize(const uint8_t *buf, const size_t size);

  // Fills internal structure as if the schema passed had been loaded by parsing
  // with Parse except that included filenames will not be populated.
  bool Deserialize(const reflection::Schema *schema);

  Type *DeserializeType(const reflection::Type *type);

  // Checks that the schema represented by this parser is a safe evolution
  // of the schema provided. Returns non-empty error on any problems.
  std::string ConformTo(const Parser &base);

  // Similar to Parse(), but now only accepts JSON to be parsed into a
  // FlexBuffer.
  bool ParseFlexBuffer(const char *source, const char *source_filename,
                       flexbuffers::Builder *builder);

  StructDef *LookupStruct(const std::string &id) const;
  StructDef *LookupStructThruParentNamespaces(const std::string &id) const;

  std::string UnqualifiedName(const std::string &fullQualifiedName);

  FLATBUFFERS_CHECKED_ERROR Error(const std::string &msg);

  // @brief Verify that any of 'opts.lang_to_generate' supports Optional scalars
  // in a schema.
  // @param opts Options used to parce a schema and generate code.
  static bool SupportsOptionalScalars(const flatbuffers::IDLOptions &opts);

  // Get the set of included files that are directly referenced by the file
  // being parsed. This does not include files that are transitively included by
  // others includes.
  std::vector<IncludedFile> GetIncludedFiles() const;

 private:
  class ParseDepthGuard;

  void Message(const std::string &msg);
  void Warning(const std::string &msg);
  FLATBUFFERS_CHECKED_ERROR ParseHexNum(int nibbles, uint64_t *val);
  FLATBUFFERS_CHECKED_ERROR Next();
  FLATBUFFERS_CHECKED_ERROR SkipByteOrderMark();
  bool Is(int t) const;
  bool IsIdent(const char *id) const;
  FLATBUFFERS_CHECKED_ERROR Expect(int t);
  std::string TokenToStringId(int t) const;
  EnumDef *LookupEnum(const std::string &id);
  FLATBUFFERS_CHECKED_ERROR ParseNamespacing(std::string *id,
                                             std::string *last);
  FLATBUFFERS_CHECKED_ERROR ParseTypeIdent(Type &type);
  FLATBUFFERS_CHECKED_ERROR ParseType(Type &type);
  FLATBUFFERS_CHECKED_ERROR AddField(StructDef &struct_def,
                                     const std::string &name, const Type &type,
                                     FieldDef **dest);
  FLATBUFFERS_CHECKED_ERROR ParseField(StructDef &struct_def);
  FLATBUFFERS_CHECKED_ERROR ParseString(Value &val, bool use_string_pooling);
  FLATBUFFERS_CHECKED_ERROR ParseComma();
  FLATBUFFERS_CHECKED_ERROR ParseAnyValue(Value &val, FieldDef *field,
                                          size_t parent_fieldn,
                                          const StructDef *parent_struct_def,
                                          size_t count,
                                          bool inside_vector = false);
  template<typename F>
  FLATBUFFERS_CHECKED_ERROR ParseTableDelimiters(size_t &fieldn,
                                                 const StructDef *struct_def,
                                                 F body);
  FLATBUFFERS_CHECKED_ERROR ParseTable(const StructDef &struct_def,
                                       std::string *value, uoffset_t *ovalue);
  void SerializeStruct(const StructDef &struct_def, const Value &val);
  void SerializeStruct(FlatBufferBuilder &builder, const StructDef &struct_def,
                       const Value &val);
  template<typename F>
  FLATBUFFERS_CHECKED_ERROR ParseVectorDelimiters(size_t &count, F body);
  FLATBUFFERS_CHECKED_ERROR ParseVector(const Type &type, uoffset_t *ovalue,
                                        FieldDef *field, size_t fieldn);
  FLATBUFFERS_CHECKED_ERROR ParseArray(Value &array);
  FLATBUFFERS_CHECKED_ERROR ParseNestedFlatbuffer(
      Value &val, FieldDef *field, size_t fieldn,
      const StructDef *parent_struct_def);
  FLATBUFFERS_CHECKED_ERROR ParseMetaData(SymbolTable<Value> *attributes);
  FLATBUFFERS_CHECKED_ERROR TryTypedValue(const std::string *name, int dtoken,
                                          bool check, Value &e, BaseType req,
                                          bool *destmatch);
  FLATBUFFERS_CHECKED_ERROR ParseHash(Value &e, FieldDef *field);
  FLATBUFFERS_CHECKED_ERROR TokenError();
  FLATBUFFERS_CHECKED_ERROR ParseSingleValue(const std::string *name, Value &e,
                                             bool check_now);
  FLATBUFFERS_CHECKED_ERROR ParseFunction(const std::string *name, Value &e);
  FLATBUFFERS_CHECKED_ERROR ParseEnumFromString(const Type &type,
                                                std::string *result);
  StructDef *LookupCreateStruct(const std::string &name,
                                bool create_if_new = true,
                                bool definition = false);
  FLATBUFFERS_CHECKED_ERROR ParseEnum(bool is_union, EnumDef **dest,
                                      const char *filename);
  FLATBUFFERS_CHECKED_ERROR ParseNamespace();
  FLATBUFFERS_CHECKED_ERROR StartStruct(const std::string &name,
                                        StructDef **dest);
  FLATBUFFERS_CHECKED_ERROR StartEnum(const std::string &name, bool is_union,
                                      EnumDef **dest);
  FLATBUFFERS_CHECKED_ERROR ParseDecl(const char *filename);
  FLATBUFFERS_CHECKED_ERROR ParseService(const char *filename);
  FLATBUFFERS_CHECKED_ERROR ParseProtoFields(StructDef *struct_def,
                                             bool isextend, bool inside_oneof);
  FLATBUFFERS_CHECKED_ERROR ParseProtoMapField(StructDef *struct_def);
  FLATBUFFERS_CHECKED_ERROR ParseProtoOption();
  FLATBUFFERS_CHECKED_ERROR ParseProtoKey();
  FLATBUFFERS_CHECKED_ERROR ParseProtoDecl();
  FLATBUFFERS_CHECKED_ERROR ParseProtoCurliesOrIdent();
  FLATBUFFERS_CHECKED_ERROR ParseTypeFromProtoType(Type *type);
  FLATBUFFERS_CHECKED_ERROR SkipAnyJsonValue();
  FLATBUFFERS_CHECKED_ERROR ParseFlexBufferNumericConstant(
      flexbuffers::Builder *builder);
  FLATBUFFERS_CHECKED_ERROR ParseFlexBufferValue(flexbuffers::Builder *builder);
  FLATBUFFERS_CHECKED_ERROR StartParseFile(const char *source,
                                           const char *source_filename);
  FLATBUFFERS_CHECKED_ERROR ParseRoot(const char *_source,
                                      const char **include_paths,
                                      const char *source_filename);
  FLATBUFFERS_CHECKED_ERROR CheckPrivateLeak();
  FLATBUFFERS_CHECKED_ERROR CheckPrivatelyLeakedFields(
      const Definition &def, const Definition &value_type);
  FLATBUFFERS_CHECKED_ERROR DoParse(const char *_source,
                                    const char **include_paths,
                                    const char *source_filename,
                                    const char *include_filename);
  FLATBUFFERS_CHECKED_ERROR DoParseJson();
  FLATBUFFERS_CHECKED_ERROR CheckClash(std::vector<FieldDef *> &fields,
                                       StructDef *struct_def,
                                       const char *suffix, BaseType baseType);
  FLATBUFFERS_CHECKED_ERROR ParseAlignAttribute(
      const std::string &align_constant, size_t min_align, size_t *align);

  bool SupportsAdvancedUnionFeatures() const;
  bool SupportsAdvancedArrayFeatures() const;
  bool SupportsOptionalScalars() const;
  bool SupportsDefaultVectorsAndStrings() const;
  bool Supports64BitOffsets() const;
  bool SupportsUnionUnderlyingType() const;
  Namespace *UniqueNamespace(Namespace *ns);

  FLATBUFFERS_CHECKED_ERROR RecurseError();
  template<typename F> CheckedError Recurse(F f);

  const std::string &GetPooledString(const std::string &s) const;

 public:
  SymbolTable<Type> types_;
  SymbolTable<StructDef> structs_;
  SymbolTable<EnumDef> enums_;
  SymbolTable<ServiceDef> services_;
  std::vector<Namespace *> namespaces_;
  Namespace *current_namespace_;
  Namespace *empty_namespace_;
  std::string error_;  // User readable error_ if Parse() == false

  FlatBufferBuilder builder_;  // any data contained in the file
  flexbuffers::Builder flex_builder_;
  flexbuffers::Reference flex_root_;
  StructDef *root_struct_def_;
  std::string file_identifier_;
  std::string file_extension_;

  std::map<uint64_t, std::string> included_files_;
  std::map<std::string, std::set<IncludedFile>> files_included_per_file_;
  std::vector<std::string> native_included_files_;

  std::map<std::string, bool> known_attributes_;

  IDLOptions opts;
  bool uses_flexbuffers_;
  bool has_warning_;

  uint64_t advanced_features_;

  std::string file_being_parsed_;

 private:
  const char *source_;

  std::vector<std::pair<Value, FieldDef *>> field_stack_;

  // TODO(cneo): Refactor parser to use string_cache more often to save
  // on memory usage.
  mutable std::set<std::string> string_cache_;

  int anonymous_counter_;
  int parse_depth_counter_;  // stack-overflow guard
};

// Utility functions for multiple generators:

// Generate text (JSON) from a given FlatBuffer, and a given Parser
// object that has been populated with the corresponding schema.
// If ident_step is 0, no indentation will be generated. Additionally,
// if it is less than 0, no linefeeds will be generated either.
// See idl_gen_text.cpp.
// strict_json adds "quotes" around field names if true.
// These functions return nullptr on success, or an error string,
// which may happen if the flatbuffer cannot be encoded in JSON (e.g.,
// it contains non-UTF-8 byte arrays in String values).
extern bool GenerateTextFromTable(const Parser &parser,
                                         const void *table,
                                         const std::string &tablename,
                                         std::string *text);
extern const char *GenerateText(const Parser &parser, const void *flatbuffer,
                                std::string *text);
extern const char *GenerateTextFile(const Parser &parser,
                                    const std::string &path,
                                    const std::string &file_name);

extern const char *GenTextFromTable(const Parser &parser, const void *table,
                                    const std::string &tablename,
                                    std::string *text);
extern const char *GenText(const Parser &parser, const void *flatbuffer,
                           std::string *text);
extern const char *GenTextFile(const Parser &parser, const std::string &path,
                               const std::string &file_name);

// Generate GRPC Cpp interfaces.
// See idl_gen_grpc.cpp.
bool GenerateCppGRPC(const Parser &parser, const std::string &path,
                     const std::string &file_name);

// Generate GRPC Go interfaces.
// See idl_gen_grpc.cpp.
bool GenerateGoGRPC(const Parser &parser, const std::string &path,
                    const std::string &file_name);

// Generate GRPC Java classes.
// See idl_gen_grpc.cpp
bool GenerateJavaGRPC(const Parser &parser, const std::string &path,
                      const std::string &file_name);

// Generate GRPC Python interfaces.
// See idl_gen_grpc.cpp.
bool GeneratePythonGRPC(const Parser &parser, const std::string &path,
                        const std::string &file_name);

// Generate GRPC Swift interfaces.
// See idl_gen_grpc.cpp.
extern bool GenerateSwiftGRPC(const Parser &parser, const std::string &path,
                              const std::string &file_name);

extern bool GenerateTSGRPC(const Parser &parser, const std::string &path,
                           const std::string &file_name);
}  // namespace flatbuffers

#endif  // FLATBUFFERS_IDL_H_
