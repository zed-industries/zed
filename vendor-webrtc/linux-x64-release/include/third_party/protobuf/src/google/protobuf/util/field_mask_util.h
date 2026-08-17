// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Defines utilities for the FieldMask well known type.

#ifndef GOOGLE_PROTOBUF_UTIL_FIELD_MASK_UTIL_H__
#define GOOGLE_PROTOBUF_UTIL_FIELD_MASK_UTIL_H__

#include <cstdint>
#include <string>
#include <vector>

#include "google/protobuf/field_mask.pb.h"
#include "absl/log/absl_check.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/descriptor.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace util {

class PROTOBUF_EXPORT FieldMaskUtil {
  typedef google::protobuf::FieldMask FieldMask;

 public:
  // Converts FieldMask to/from string, formatted by separating each path
  // with a comma (e.g., "foo_bar,baz.quz").
  static std::string ToString(const FieldMask& mask);
  static void FromString(absl::string_view str, FieldMask* out);

  // Populates the FieldMask with the paths corresponding to the fields with the
  // given numbers, after checking that all field numbers are valid.
  template <typename T>
  static void FromFieldNumbers(const std::vector<int64_t>& field_numbers,
                               FieldMask* out) {
    for (const auto field_number : field_numbers) {
      const FieldDescriptor* field_desc =
          T::descriptor()->FindFieldByNumber(field_number);
      ABSL_CHECK(field_desc != nullptr)
          << "Invalid field number for " << T::descriptor()->full_name() << ": "
          << field_number;
      AddPathToFieldMask<T>(field_desc->lowercase_name(), out);
    }
  }

  // Converts FieldMask to/from string, formatted according to proto3 JSON
  // spec for FieldMask (e.g., "fooBar,baz.quz"). If the field name is not
  // style conforming (i.e., not snake_case when converted to string, or not
  // camelCase when converted from string), the conversion will fail.
  static bool ToJsonString(const FieldMask& mask, std::string* out);
  static bool FromJsonString(absl::string_view str, FieldMask* out);

  // Get the descriptors of the fields which the given path from the message
  // descriptor traverses, if field_descriptors is not null.
  // Return false if the path is not valid, and the content of field_descriptors
  // is unspecified.
  static bool GetFieldDescriptors(
      const Descriptor* descriptor, absl::string_view path,
      std::vector<const FieldDescriptor*>* field_descriptors);

  // Checks whether the given path is valid for type T.
  template <typename T>
  static bool IsValidPath(absl::string_view path) {
    return GetFieldDescriptors(T::descriptor(), path, nullptr);
  }

  // Checks whether the given FieldMask is valid for type T.
  template <typename T>
  static bool IsValidFieldMask(const FieldMask& mask) {
    for (int i = 0; i < mask.paths_size(); ++i) {
      if (!GetFieldDescriptors(T::descriptor(), mask.paths(i), nullptr)) {
        return false;
      }
    }
    return true;
  }

  // Adds a path to FieldMask after checking whether the given path is valid.
  // This method check-fails if the path is not a valid path for type T.
  template <typename T>
  static void AddPathToFieldMask(absl::string_view path, FieldMask* mask) {
    ABSL_CHECK(IsValidPath<T>(path)) << path;
    mask->add_paths(std::string(path));
  }

  // Creates a FieldMask with all fields of type T. This FieldMask only
  // contains fields of T but not any sub-message fields.
  template <typename T>
  static FieldMask GetFieldMaskForAllFields() {
    FieldMask out;
    GetFieldMaskForAllFields(T::descriptor(), &out);
    return out;
  }
  template <typename T>
  [[deprecated("Use *out = GetFieldMaskForAllFields() instead")]] static void
  GetFieldMaskForAllFields(FieldMask* out) {
    GetFieldMaskForAllFields(T::descriptor(), out);
  }
  // This flavor takes the protobuf type descriptor as an argument.
  // Useful when the type is not known at compile time.
  static void GetFieldMaskForAllFields(const Descriptor* descriptor,
                                       FieldMask* out);

  // Converts a FieldMask to the canonical form. It will:
  //   1. Remove paths that are covered by another path. For example,
  //      "foo.bar" is covered by "foo" and will be removed if "foo"
  //      is also in the FieldMask.
  //   2. Sort all paths in alphabetical order.
  static void ToCanonicalForm(const FieldMask& mask, FieldMask* out);

  // Creates an union of two FieldMasks.
  static void Union(const FieldMask& mask1, const FieldMask& mask2,
                    FieldMask* out);

  // Creates an intersection of two FieldMasks.
  static void Intersect(const FieldMask& mask1, const FieldMask& mask2,
                        FieldMask* out);

  // Subtracts mask2 from mask1 base of type T.
  template <typename T>
  static void Subtract(const FieldMask& mask1, const FieldMask& mask2,
                       FieldMask* out) {
    Subtract(T::descriptor(), mask1, mask2, out);
  }
  // This flavor takes the protobuf type descriptor as an argument.
  // Useful when the type is not known at compile time.
  static void Subtract(const Descriptor* descriptor, const FieldMask& mask1,
                       const FieldMask& mask2, FieldMask* out);

  // Returns true if path is covered by the given FieldMask. Note that path
  // "foo.bar" covers all paths like "foo.bar.baz", "foo.bar.quz.x", etc.
  // Also note that parent paths are not covered by explicit child path, i.e.
  // "foo.bar" does NOT cover "foo", even if "bar" is the only child.
  static bool IsPathInFieldMask(absl::string_view path, const FieldMask& mask);

  class MergeOptions;
  // Merges fields specified in a FieldMask into another message.
  static void MergeMessageTo(const Message& source, const FieldMask& mask,
                             const MergeOptions& options, Message* destination);

  class TrimOptions;
  // Removes from 'message' any field that is not represented in the given
  // FieldMask. If the FieldMask is empty, does nothing.
  // Returns true if the message is modified.
  static bool TrimMessage(const FieldMask& mask, Message* message);

  // Removes from 'message' any field that is not represented in the given
  // FieldMask with customized TrimOptions.
  // If the FieldMask is empty, does nothing.
  // Returns true if the message is modified.
  static bool TrimMessage(const FieldMask& mask, Message* message,
                          const TrimOptions& options);

 private:
  friend class SnakeCaseCamelCaseTest;
  // Converts a field name from snake_case to camelCase:
  //   1. Every character after "_" will be converted to uppercase.
  //   2. All "_"s are removed.
  // The conversion will fail if:
  //   1. The field name contains uppercase letters.
  //   2. Any character after a "_" is not a lowercase letter.
  // If the conversion succeeds, it's guaranteed that the resulted
  // camelCase name will yield the original snake_case name when
  // converted using CamelCaseToSnakeCase().
  //
  // Note that the input can contain characters not allowed in C identifiers.
  // For example, "foo_bar,baz_quz" will be converted to "fooBar,bazQuz"
  // successfully.
  static bool SnakeCaseToCamelCase(absl::string_view input,
                                   std::string* output);
  // Converts a field name from camelCase to snake_case:
  //   1. Every uppercase letter is converted to lowercase with an additional
  //      preceding "_".
  // The conversion will fail if:
  //   1. The field name contains "_"s.
  // If the conversion succeeds, it's guaranteed that the resulted
  // snake_case name will yield the original camelCase name when
  // converted using SnakeCaseToCamelCase().
  //
  // Note that the input can contain characters not allowed in C identifiers.
  // For example, "fooBar,bazQuz" will be converted to "foo_bar,baz_quz"
  // successfully.
  static bool CamelCaseToSnakeCase(absl::string_view input,
                                   std::string* output);
};

class PROTOBUF_EXPORT FieldMaskUtil::MergeOptions {
 public:
  MergeOptions()
      : replace_message_fields_(false), replace_repeated_fields_(false) {}
  // When merging message fields, the default behavior is to merge the
  // content of two message fields together. If you instead want to use
  // the field from the source message to replace the corresponding field
  // in the destination message, set this flag to true. When this flag is set,
  // specified submessage fields that are missing in source will be cleared in
  // destination.
  void set_replace_message_fields(bool value) {
    replace_message_fields_ = value;
  }
  bool replace_message_fields() const { return replace_message_fields_; }
  // The default merging behavior will append entries from the source
  // repeated field to the destination repeated field. If you only want
  // to keep the entries from the source repeated field, set this flag
  // to true.
  void set_replace_repeated_fields(bool value) {
    replace_repeated_fields_ = value;
  }
  bool replace_repeated_fields() const { return replace_repeated_fields_; }

 private:
  bool replace_message_fields_;
  bool replace_repeated_fields_;
};

class PROTOBUF_EXPORT FieldMaskUtil::TrimOptions {
 public:
  TrimOptions() : keep_required_fields_(false) {}
  // When trimming message fields, the default behavior is to trim required
  // fields of the present message if they are not specified in the field mask.
  // If you instead want to keep required fields of the present message even
  // when they are not specified in the field mask, set this flag to true.
  void set_keep_required_fields(bool value) { keep_required_fields_ = value; }
  bool keep_required_fields() const { return keep_required_fields_; }

 private:
  bool keep_required_fields_;
};

}  // namespace util
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_UTIL_FIELD_MASK_UTIL_H__
