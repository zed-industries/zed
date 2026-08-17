// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Helper functions for generating ObjectiveC code.

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_NAMES_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_NAMES_H__

#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "google/protobuf/descriptor.h"

// Must be included last
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

// Get/Set the path to a file to load for objc class prefix lookups.
PROTOC_EXPORT absl::string_view GetPackageToPrefixMappingsPath();
PROTOC_EXPORT void SetPackageToPrefixMappingsPath(absl::string_view file_path);
// Get/Set if the proto package should be used to make the default prefix for
// symbols. This will then impact most of the type naming apis below. It is done
// as a global to not break any other generator reusing the methods since they
// are exported.
PROTOC_EXPORT bool UseProtoPackageAsDefaultPrefix();
PROTOC_EXPORT void SetUseProtoPackageAsDefaultPrefix(bool on_or_off);
// Get/Set the path to a file to load as exceptions when
// `UseProtoPackageAsDefaultPrefix()` is `true`. An empty string means there
// should be no exceptions.
PROTOC_EXPORT absl::string_view GetProtoPackagePrefixExceptionList();
PROTOC_EXPORT void SetProtoPackagePrefixExceptionList(
    absl::string_view file_path);
// Get/Set a prefix to add before the prefix generated from the package name.
// This is only used when UseProtoPackageAsDefaultPrefix() is True.
PROTOC_EXPORT absl::string_view GetForcedPackagePrefix();
PROTOC_EXPORT void SetForcedPackagePrefix(absl::string_view prefix);

// Returns true if the name requires a ns_returns_not_retained attribute applied
// to it.
PROTOC_EXPORT bool IsRetainedName(absl::string_view name);

// Returns true if the name starts with "init" and will need to have special
// handling under ARC.
PROTOC_EXPORT bool IsInitName(absl::string_view name);

// Returns true if the name requires a cf_returns_not_retained attribute applied
// to it.
PROTOC_EXPORT bool IsCreateName(absl::string_view name);

// Gets the objc_class_prefix or the prefix made from the proto package.
PROTOC_EXPORT std::string FileClassPrefix(const FileDescriptor* file);

// Gets the path of the file we're going to generate (sans the .pb.h
// extension).  The path will be dependent on the objectivec package
// declared in the proto package.
PROTOC_EXPORT std::string FilePath(const FileDescriptor* file);

// Just like FilePath(), but without the directory part.
PROTOC_EXPORT std::string FilePathBasename(const FileDescriptor* file);

// Gets the name of the root class we'll generate in the file.  This class
// is not meant for external consumption, but instead contains helpers that
// the rest of the classes need
PROTOC_EXPORT std::string FileClassName(const FileDescriptor* file);

// These return the fully-qualified class name corresponding to the given
// descriptor.
PROTOC_EXPORT std::string ClassName(const Descriptor* descriptor);
PROTOC_EXPORT std::string ClassName(const Descriptor* descriptor,
                                    std::string* out_suffix_added);
PROTOC_EXPORT std::string EnumName(const EnumDescriptor* descriptor);

// Returns the fully-qualified name of the enum value corresponding to the
// the descriptor.
PROTOC_EXPORT std::string EnumValueName(const EnumValueDescriptor* descriptor);

// Returns the name of the enum value corresponding to the descriptor.
PROTOC_EXPORT std::string EnumValueShortName(
    const EnumValueDescriptor* descriptor);

// Reverse what an enum does.
PROTOC_EXPORT std::string UnCamelCaseEnumShortName(absl::string_view name);

// Returns the name to use for the extension (used as the method off the file's
// Root class).
PROTOC_EXPORT std::string ExtensionMethodName(
    const FieldDescriptor* descriptor);

// Returns the transformed field name.
PROTOC_EXPORT std::string FieldName(const FieldDescriptor* field);
PROTOC_EXPORT std::string FieldNameCapitalized(const FieldDescriptor* field);

// The formatting options for FieldObjCType
enum FieldObjCTypeOptions : unsigned int {
  // No options.
  kFieldObjCTypeOptions_None = 0,
  // Leave off the lightweight generics from any collection classes.
  kFieldObjCTypeOptions_OmitLightweightGenerics = 1 << 0,
  // For things that are pointers, include a space before the `*`.
  kFieldObjCTypeOptions_IncludeSpaceBeforeStar = 1 << 1,
  // For things that are basic types (int, float, etc.), include a space after
  // the type, needed for some usage cases. This is mainly needed when using the
  // type to declare variables. Pointers don't need the space to generate valid
  // code.
  kFieldObjCTypeOptions_IncludeSpaceAfterBasicTypes = 1 << 2,
};

// This returns the ObjC type for the field. `options` allows some controls on
// how the string is created for different usages.
PROTOC_EXPORT std::string FieldObjCType(
    const FieldDescriptor* field,
    FieldObjCTypeOptions options = kFieldObjCTypeOptions_None);

// Returns the transformed oneof name.
PROTOC_EXPORT std::string OneofEnumName(const OneofDescriptor* descriptor);
PROTOC_EXPORT std::string OneofName(const OneofDescriptor* descriptor);
PROTOC_EXPORT std::string OneofNameCapitalized(
    const OneofDescriptor* descriptor);

// Reverse of the above.
PROTOC_EXPORT std::string UnCamelCaseFieldName(absl::string_view name,
                                               const FieldDescriptor* field);

// The name the commonly used by the library when built as a framework.
// This lines up to the name used in the CocoaPod.
extern PROTOC_EXPORT const char* const ProtobufLibraryFrameworkName;
// Returns the CPP symbol name to use as the gate for framework style imports
// for the given framework name to use.
PROTOC_EXPORT std::string ProtobufFrameworkImportSymbol(
    absl::string_view framework_name);

// ---------------------------------------------------------------------------

// These aren't really "naming" related, but can be useful for something
// building on top of ObjC Protos to be able to share the knowledge/enforcement.

// Checks if the file is one of the proto's bundled with the library.
PROTOC_EXPORT bool IsProtobufLibraryBundledProtoFile(
    const FileDescriptor* file);

// Generator Prefix Validation Options (see generator.cc for a
// description of each):
struct Options {
  Options();
  std::string expected_prefixes_path;
  std::vector<std::string> expected_prefixes_suppressions;
  bool prefixes_must_be_registered;
  bool require_prefixes;
};

// Checks the prefix for the given files and outputs any warnings as needed. If
// there are flat out errors, then out_error is filled in with the first error
// and the result is false.
PROTOC_EXPORT bool ValidateObjCClassPrefixes(
    const std::vector<const FileDescriptor*>& files,
    const Options& validation_options, std::string* out_error);
// Same was the other ValidateObjCClassPrefixes() calls, but the options all
// come from the environment variables.
PROTOC_EXPORT bool ValidateObjCClassPrefixes(
    const std::vector<const FileDescriptor*>& files, std::string* out_error);

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_NAMES_H__
