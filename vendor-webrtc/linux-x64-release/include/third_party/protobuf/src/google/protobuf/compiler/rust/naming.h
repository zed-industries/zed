// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_NAMING_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_NAMING_H__

#include <array>
#include <string>

#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/rust/context.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {
std::string GetCrateName(Context& ctx, const FileDescriptor& dep);

// Gets the file name for the entry point rs file. This path will be in the same
// directory as the provided file. This will be the path provided by command
// line flag, or a default path relative to the provided `file` (which should
// be the first .proto src proto file).
std::string GetEntryPointRsFilePath(Context& ctx, const FileDescriptor& file);

std::string GetRsFile(Context& ctx, const FileDescriptor& file);
std::string GetThunkCcFile(Context& ctx, const FileDescriptor& file);
std::string GetHeaderFile(Context& ctx, const FileDescriptor& file);

std::string ThunkName(Context& ctx, const FieldDescriptor& field,
                      absl::string_view op);
std::string ThunkName(Context& ctx, const OneofDescriptor& field,
                      absl::string_view op);

std::string ThunkName(Context& ctx, const Descriptor& msg,
                      absl::string_view op);
std::string RawMapThunk(Context& ctx, const Descriptor& msg,
                        absl::string_view key_t, absl::string_view op);
std::string RawMapThunk(Context& ctx, const EnumDescriptor& desc,
                        absl::string_view key_t, absl::string_view op);

// Returns a path to the Proxied Rust type of the given field. The path will be
// relative if the type is in the same crate, or absolute if it is in a
// different crate.
std::string RsTypePath(Context& ctx, const FieldDescriptor& field);
std::string RsTypePath(Context& ctx, const Descriptor& message);
std::string RsTypePath(Context& ctx, const EnumDescriptor& descriptor);

// Returns the 'simple spelling' of the Rust View type for the provided field.
// For example, `i32` for int32 fields and `SomeMsgView<'$lifetime$>` for
// message fields, or `SomeMsgView` if an empty lifetime is provided).
//
// The returned type will always be functionally substitutable for the
// corresponding View<'$lifetime$, $sometype$> of the field's Rust type.
std::string RsViewType(Context& ctx, const FieldDescriptor& field,
                       absl::string_view lifetime);

std::string EnumRsName(const EnumDescriptor& desc);
std::string EnumValueRsName(const EnumValueDescriptor& value);

std::string OneofViewEnumRsName(const OneofDescriptor& oneof);
std::string OneofCaseEnumRsName(const OneofDescriptor& oneof);

std::string OneofCaseEnumCppName(const OneofDescriptor& oneof);
std::string OneofCaseRsName(const FieldDescriptor& oneof_field);

std::string FieldInfoComment(Context& ctx, const FieldDescriptor& field);

// Return how to name a field with 'collision avoidance'. This adds a suffix
// of the field number to the field name if it appears that it will collide with
// another field's non-getter accessor.
//
// For example, for the message:
// message M { bool set_x = 1; int32 x = 2; string x_mut = 8; }
// All accessors for the field `set_x` will be constructed as though the field
// was instead named `set_x_1`, and all accessors for `x_mut` will be as though
// the field was instead named `x_mut_8`.
//
// This is a best-effort heuristic to avoid realistic accidental
// collisions. It is still possible to create a message definition that will
// have a collision, and it may rename a field even if there's no collision (as
// in the case of x_mut in the example).
//
// Note the returned name may still be a rust keyword: RsSafeName() should
// additionally be used if there is no prefix/suffix being appended to the name.
std::string FieldNameWithCollisionAvoidance(const FieldDescriptor& field);

// Returns how to 'spell' the provided name in Rust, which is the provided name
// verbatim unless it is a Rust keyword that isn't a legal symbol name.
std::string RsSafeName(absl::string_view name);

// Constructs a string of the Rust modules which will contain the entity.
//
// Example: Given a message 'NestedMessage' which is defined in package 'x.y'
// which is inside 'ParentMessage', the message will be placed in the
// x::y::parent_message Rust module, so this function will return
// "x::y::parent_message::", with the necessary prefix to make it relative to
// the current scope, or absolute if the entity is in a different crate.
std::string RustModule(Context& ctx, const Descriptor& msg);
std::string RustModule(Context& ctx, const EnumDescriptor& enum_);
std::string RustModule(Context& ctx, const OneofDescriptor& oneof);

std::string RustInternalModuleName(const FileDescriptor& file);

template <typename Desc>
std::string GetUnderscoreDelimitedFullName(Context& ctx, const Desc& desc);

std::string UnderscoreDelimitFullName(Context& ctx,
                                      absl::string_view full_name);

// TODO: Unify these with other case-conversion functions.

// Converts an UpperCamel or lowerCamel string to a snake_case string.
std::string CamelToSnakeCase(absl::string_view input);

// Converts a snake_case string to an UpperCamelCase string.
std::string SnakeToUpperCamelCase(absl::string_view input);

// Converts a SCREAMING_SNAKE_CASE string to an UpperCamelCase string.
std::string ScreamingSnakeToUpperCamelCase(absl::string_view input);

// Given a fixed prefix, this will repeatedly strip provided
// string_views if they start with the prefix, the prefix in UpperCamel, or
// the prefix in snake_case.
class MultiCasePrefixStripper final {
 public:
  explicit MultiCasePrefixStripper(absl::string_view prefix);

  // Strip a prefix from the name in UpperCamel or snake_case, if present.
  // If there is an underscore after the prefix, that will also be stripped.
  // The stripping is case-insensitive.
  absl::string_view StripPrefix(absl::string_view name) const;

 private:
  std::array<std::string, 3> prefixes_;
};

// More efficient overload if a stripper is already constructed.
std::string EnumValueRsName(const MultiCasePrefixStripper& stripper,
                            absl::string_view value_name);

// Describes the names and conversions for a supported map key type.
struct MapKeyType {
  // Identifier used in thunk name.
  absl::string_view thunk_ident;

  // Rust key typename (K in Map<K, V>, so e.g. `[u8]` for bytes).
  // This field may have an unexpanded `$pb$` variable.
  absl::string_view rs_key_t;

  // Rust key typename used by thunks for FFI (e.g. `PtrAndLen` for bytes).
  // This field may have an unexpanded `$pbi$` variable.
  absl::string_view rs_ffi_key_t;

  // Rust expression converting `key: rs_key_t` into an `rs_ffi_key_t`.
  absl::string_view rs_to_ffi_key_expr;

  // Rust expression converting `ffi_key: rs_ffi_key_t` into an `rs_key_t`.
  // This field may have an unexpanded `$pb$` variable.
  absl::string_view rs_from_ffi_key_expr;

  // C++ key typename (K in Map<K, V>, so e.g. `std::string` for bytes).
  absl::string_view cc_key_t;

  // C++ key typename used by thunks for FFI (e.g. `PtrAndLen` for bytes).
  absl::string_view cc_ffi_key_t;

  // C++ expression converting `cc_ffi_key_t key` into a `cc_key_t`.
  absl::string_view cc_from_ffi_key_expr;

  // C++ expression converting `cc_key_t cpp_key` into a `cc_ffi_key_t`.
  absl::string_view cc_to_ffi_key_expr;
};

extern const MapKeyType kMapKeyTypes[6];

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_NAMING_H__
