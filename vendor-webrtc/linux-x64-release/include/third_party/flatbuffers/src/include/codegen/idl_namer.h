#ifndef FLATBUFFERS_INCLUDE_CODEGEN_IDL_NAMER_H_
#define FLATBUFFERS_INCLUDE_CODEGEN_IDL_NAMER_H_

#include "codegen/namer.h"
#include "flatbuffers/idl.h"

namespace flatbuffers {

// Provides Namer capabilities to types defined in the flatbuffers IDL.
class IdlNamer : public Namer {
 public:
  explicit IdlNamer(Config config, std::set<std::string> keywords)
      : Namer(config, std::move(keywords)) {}

  using Namer::Constant;
  using Namer::Directories;
  using Namer::Field;
  using Namer::File;
  using Namer::Function;
  using Namer::Method;
  using Namer::Namespace;
  using Namer::NamespacedType;
  using Namer::ObjectType;
  using Namer::Type;
  using Namer::Variable;
  using Namer::Variant;

  std::string Constant(const FieldDef &d) const { return Constant(d.name); }

  // Types are always structs or enums so we can only expose these two
  // overloads.
  std::string Type(const StructDef &d) const { return Type(d.name); }
  std::string Type(const EnumDef &d) const { return Type(d.name); }

  std::string Function(const Definition &s) const { return Function(s.name); }
  std::string Function(const std::string& prefix, const Definition &s) const {
    return Function(prefix + s.name);
  }

  std::string Field(const FieldDef &s) const { return Field(s.name); }
  std::string Field(const FieldDef &d, const std::string &s) const {
    return Field(d.name + "_" + s);
  }

  std::string Variable(const FieldDef &s) const { return Variable(s.name); }

  std::string Variable(const StructDef &s) const { return Variable(s.name); }

  std::string Variant(const EnumVal &s) const { return Variant(s.name); }

  std::string EnumVariant(const EnumDef &e, const EnumVal &v) const {
    return Type(e) + config_.enum_variant_seperator + Variant(v);
  }

  std::string ObjectType(const StructDef &d) const {
    return ObjectType(d.name);
  }
  std::string ObjectType(const EnumDef &d) const { return ObjectType(d.name); }

  std::string Method(const FieldDef &d, const std::string &suffix) const {
    return Method(d.name, suffix);
  }
  std::string Method(const std::string &prefix, const StructDef &d) const {
    return Method(prefix, d.name);
  }
  std::string Method(const std::string &prefix, const FieldDef &d) const {
    return Method(prefix, d.name);
  }
  std::string Method(const std::string &prefix, const FieldDef &d,
                     const std::string &suffix) const {
    return Method(prefix, d.name, suffix);
  }

  std::string Namespace(const struct Namespace &ns) const {
    return Namespace(ns.components);
  }

  std::string NamespacedEnumVariant(const EnumDef &e, const EnumVal &v) const {
    return NamespacedString(e.defined_namespace, EnumVariant(e, v));
  }

  std::string NamespacedType(const Definition &def) const {
    return NamespacedString(def.defined_namespace, Type(def.name));
  }

  std::string NamespacedObjectType(const Definition &def) const {
    return NamespacedString(def.defined_namespace, ObjectType(def.name));
  }

  std::string Directories(const struct Namespace &ns,
                          SkipDir skips = SkipDir::None,
                          Case input_case = Case::kUpperCamel) const {
    return Directories(ns.components, skips, input_case);
  }

  // Legacy fields do not really follow the usual config and should be
  // considered for deprecation.

  std::string LegacyRustNativeVariant(const EnumVal &v) const {
    return ConvertCase(EscapeKeyword(v.name), Case::kUpperCamel);
  }

  std::string LegacyRustFieldOffsetName(const FieldDef &field) const {
    return "VT_" + ConvertCase(EscapeKeyword(field.name), Case::kAllUpper);
  }
    std::string LegacyRustUnionTypeOffsetName(const FieldDef &field) const {
    return "VT_" + ConvertCase(EscapeKeyword(field.name + "_type"), Case::kAllUpper);
  }


  std::string LegacySwiftVariant(const EnumVal &ev) const {
    auto name = ev.name;
    if (isupper(name.front())) {
      std::transform(name.begin(), name.end(), name.begin(), CharToLower);
    }
    return EscapeKeyword(ConvertCase(name, Case::kLowerCamel));
  }

  // Also used by Kotlin, lol.
  std::string LegacyJavaMethod2(const std::string &prefix, const StructDef &sd,
                                const std::string &suffix) const {
    return prefix + sd.name + suffix;
  }

  std::string LegacyKotlinVariant(EnumVal &ev) const {
    // Namer assumes the input case is snake case which is wrong...
    return ConvertCase(EscapeKeyword(ev.name), Case::kLowerCamel);
  }
  // Kotlin methods escapes keywords after case conversion but before
  // prefixing and suffixing.
  std::string LegacyKotlinMethod(const std::string &prefix, const FieldDef &d,
                                 const std::string &suffix) const {
    return prefix + ConvertCase(EscapeKeyword(d.name), Case::kUpperCamel) +
           suffix;
  }
  std::string LegacyKotlinMethod(const std::string &prefix, const StructDef &d,
                                 const std::string &suffix) const {
    return prefix + ConvertCase(EscapeKeyword(d.name), Case::kUpperCamel) +
           suffix;
  }

  // This is a mix of snake case and keep casing, when Ts should be using
  // lower camel case.
  std::string LegacyTsMutateMethod(const FieldDef& d) {
    return "mutate_" + d.name;
  }

  std::string LegacyRustUnionTypeMethod(const FieldDef &d) {
    // assert d is a union
    // d should convert case but not escape keywords due to historical reasons
    return ConvertCase(d.name, config_.fields, Case::kLowerCamel) + "_type";
  }

 private:
  std::string NamespacedString(const struct Namespace *ns,
                               const std::string &str) const {
    std::string ret;
    if (ns != nullptr) { ret += Namespace(ns->components); }
    if (!ret.empty()) ret += config_.namespace_seperator;
    return ret + str;
  }
};

// This is a temporary helper function for code generators to call until all
// flag-overriding logic into flatc.cpp
inline Namer::Config WithFlagOptions(const Namer::Config &input,
                                     const IDLOptions &opts,
                                     const std::string &path) {
  Namer::Config result = input;
  result.object_prefix = opts.object_prefix;
  result.object_suffix = opts.object_suffix;
  result.output_path = path;
  result.filename_suffix = opts.filename_suffix;
  return result;
}

}  // namespace flatbuffers

#endif  // FLATBUFFERS_INCLUDE_CODEGEN_IDL_NAMER_H_
