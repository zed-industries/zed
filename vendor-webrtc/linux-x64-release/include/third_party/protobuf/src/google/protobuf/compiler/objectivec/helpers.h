// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Helper functions for generating ObjectiveC code.

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_HELPERS_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_HELPERS_H__

#include <cstddef>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/log/absl_log.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/objectivec/options.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

// Escape C++ trigraphs by escaping question marks to "\?".
std::string EscapeTrigraphs(absl::string_view to_escape);

// Returns true if the extension field is a custom option.
// https://protobuf.dev/programming-guides/proto2/#customoptions
bool ExtensionIsCustomOption(const FieldDescriptor* extension_field);

enum ObjectiveCType {
  OBJECTIVECTYPE_INT32,
  OBJECTIVECTYPE_UINT32,
  OBJECTIVECTYPE_INT64,
  OBJECTIVECTYPE_UINT64,
  OBJECTIVECTYPE_FLOAT,
  OBJECTIVECTYPE_DOUBLE,
  OBJECTIVECTYPE_BOOLEAN,
  OBJECTIVECTYPE_STRING,
  OBJECTIVECTYPE_DATA,
  OBJECTIVECTYPE_ENUM,
  OBJECTIVECTYPE_MESSAGE
};

enum FlagType {
  FLAGTYPE_DESCRIPTOR_INITIALIZATION,
  FLAGTYPE_EXTENSION,
  FLAGTYPE_FIELD
};

std::string GetCapitalizedType(const FieldDescriptor* field);

ObjectiveCType GetObjectiveCType(FieldDescriptor::Type field_type);

inline ObjectiveCType GetObjectiveCType(const FieldDescriptor* field) {
  return GetObjectiveCType(field->type());
}

inline bool IsPrimitiveType(const FieldDescriptor* field) {
  ObjectiveCType type = GetObjectiveCType(field);
  switch (type) {
    case OBJECTIVECTYPE_INT32:
    case OBJECTIVECTYPE_UINT32:
    case OBJECTIVECTYPE_INT64:
    case OBJECTIVECTYPE_UINT64:
    case OBJECTIVECTYPE_FLOAT:
    case OBJECTIVECTYPE_DOUBLE:
    case OBJECTIVECTYPE_BOOLEAN:
    case OBJECTIVECTYPE_ENUM:
      return true;
      break;
    default:
      return false;
  }
}

inline bool IsReferenceType(const FieldDescriptor* field) {
  return !IsPrimitiveType(field);
}

std::string GPBGenericValueFieldName(const FieldDescriptor* field);
std::string DefaultValue(const FieldDescriptor* field);

std::string BuildFlagsString(FlagType type,
                             const std::vector<std::string>& strings);

// Returns a symbol that can be used in C code to refer to an Objective-C
// class without initializing the class.
std::string ObjCClass(absl::string_view class_name);

// Declares an Objective-C class without initializing the class so that it can
// be referred to by ObjCClass.
std::string ObjCClassDeclaration(absl::string_view class_name);

// Flag to control the behavior of `EmitCommentsString`.
enum CommentStringFlags : unsigned int {
  kCommentStringFlags_None = 0,
  // Add a newline before the comment.
  kCommentStringFlags_AddLeadingNewline = 1 << 1,
  // Force a multiline comment even if only 1 line.
  kCommentStringFlags_ForceMultiline = 1 << 2,
};

// Emits HeaderDoc/appledoc style comments out of the comments in the .proto
// file.
void EmitCommentsString(io::Printer* printer, const GenerationOptions& opts,
                        const SourceLocation& location,
                        CommentStringFlags flags = kCommentStringFlags_None);

// Emits HeaderDoc/appledoc style comments out of the comments in the .proto
// file.
template <class TDescriptor>
void EmitCommentsString(io::Printer* printer, const GenerationOptions& opts,
                        const TDescriptor* descriptor,
                        CommentStringFlags flags = kCommentStringFlags_None) {
  SourceLocation location;
  if (descriptor->GetSourceLocation(&location)) {
    EmitCommentsString(printer, opts, location, flags);
  }
}

template <class TDescriptor>
std::string GetOptionalDeprecatedAttribute(
    const TDescriptor* descriptor, const FileDescriptor* file = nullptr) {
  bool isDeprecated = descriptor->options().deprecated();
  // The file is only passed when checking Messages & Enums, so those types
  // get tagged. At the moment, it doesn't seem to make sense to tag every
  // field or enum value with when the file is deprecated.
  bool isFileLevelDeprecation = false;
  if (!isDeprecated && file) {
    isFileLevelDeprecation = file->options().deprecated();
    isDeprecated = isFileLevelDeprecation;
  }
  if (isDeprecated) {
    std::string message;
    const FileDescriptor* sourceFile = descriptor->file();
    if (isFileLevelDeprecation) {
      message = absl::StrCat(sourceFile->name(), " is deprecated.");
    } else {
      message = absl::StrCat(descriptor->full_name(), " is deprecated (see ",
                             sourceFile->name(), ").");
    }

    return absl::StrCat("GPB_DEPRECATED_MSG(\"", message, "\")");
  } else {
    return "";
  }
}

// Helpers to identify the WellKnownType files/messages that get an Objective-C
// category within the runtime to add helpers.
bool HasWKTWithObjCCategory(const FileDescriptor* file);
bool IsWKTWithObjCCategory(const Descriptor* descriptor);

// A map of `io::Printer::Sub`s, where entries can be overwritten.
//
// This exists because `io::Printer::WithVars` only accepts a flat list of
// substitutions, and will break if there are any duplicated entries. At the
// same time, a lot of code in this generator depends on modifying, overwriting,
// and looking up variables in the list of substitutions.
class SubstitutionMap {
 public:
  SubstitutionMap() = default;
  SubstitutionMap(const SubstitutionMap&) = delete;
  SubstitutionMap& operator=(const SubstitutionMap&) = delete;

  auto Install(io::Printer* printer) const { return printer->WithVars(subs_); }

  std::string Value(absl::string_view key) const {
    if (auto it = subs_map_.find(key); it != subs_map_.end()) {
      return std::string(subs_.at(it->second).value());
    }
    ABSL_LOG(FATAL) << " Unknown variable: " << key;
  }

  // Sets or replaces a variable in the map.
  // All arguments are forwarded to `io::Printer::Sub`.
  template <typename... Args>
  void Set(std::string key, Args&&... args);
  // Same as above, but takes a `io::Printer::Sub` directly.
  //
  // This is necessary to use advanced features of `io::Printer::Sub` like
  // annotations.
  void Set(io::Printer::Sub&& sub);

 private:
  std::vector<io::Printer::Sub> subs_;
  absl::flat_hash_map<std::string, size_t> subs_map_;
};

template <typename... Args>
void SubstitutionMap::Set(std::string key, Args&&... args) {
  if (auto [it, inserted] = subs_map_.try_emplace(key, subs_.size());
      !inserted) {
    subs_[it->second] =
        io::Printer::Sub(std::move(key), std::forward<Args>(args)...);
  } else {
    subs_.emplace_back(std::move(key), std::forward<Args>(args)...);
  }
}

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_HELPERS_H__
