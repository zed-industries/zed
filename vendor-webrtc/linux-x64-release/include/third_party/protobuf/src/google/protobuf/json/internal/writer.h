// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_JSON_INTERNAL_WRITER_H__
#define GOOGLE_PROTOBUF_JSON_INTERNAL_WRITER_H__

#include <cfloat>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <limits>
#include <ostream>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>

#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/io/strtod.h"
#include "google/protobuf/io/zero_copy_sink.h"
#include "google/protobuf/io/zero_copy_stream.h"
#include "google/protobuf/stubs/status_macros.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace json_internal {
struct WriterOptions {
  // Whether to add spaces, line breaks and indentation to make the JSON output
  // easy to read.
  bool add_whitespace = false;
  // Whether to always print fields which do not support presence if they would
  // otherwise be omitted, namely:
  // - Implicit presence fields set to their 0 value
  // - Empty lists and maps
  bool always_print_fields_with_no_presence = false;
  // Whether to always print enums as ints. By default they are rendered as
  // strings.
  bool always_print_enums_as_ints = false;
  // Whether to preserve proto field names
  bool preserve_proto_field_names = false;
  // If set, int64 values that can be represented exactly as a double are
  // printed without quotes.
  bool unquote_int64_if_possible = false;
  // The original parser used by json_util2 accepted a number of non-standard
  // options. Setting this flag enables them.
  //
  // What those extensions were is explicitly not documented, beyond what exists
  // in the unit tests; we intend to remove this setting eventually. See
  // b/234868512.
  bool allow_legacy_syntax = false;
};

template <typename Tuple, typename F, size_t... i>
void EachInner(const Tuple& value, F f, std::index_sequence<i...>) {
  int ignored[] = {
      (f(std::get<i>(value)), 0)...};  // NOLINT(readability/braces)
  (void)ignored;
}

// Executes f on each element of value.
template <typename Tuple, typename F>
void Each(const Tuple& value, F f) {
  EachInner(value, f,
            std::make_index_sequence<std::tuple_size<Tuple>::value>());
}

// See JsonWriter::Write().
template <typename... T>
struct Quoted {
  std::tuple<T...> value;
};

// Because this is not C++17 yet, we cannot add a deduction guide.
template <typename... T>
static Quoted<T...> MakeQuoted(T... t) {
  return Quoted<T...>{std::make_tuple(t...)};
}

class JsonWriter {
 public:
  JsonWriter(io::ZeroCopyOutputStream* out, WriterOptions options)
      : sink_(out), options_(options) {}

  const WriterOptions& options() const { return options_; }

  void Push() { ++indent_; }
  void Pop() { --indent_; }

  // The many overloads of Write() will write a value to the underlying stream.
  // Some values may want to be quoted; the Quoted<> type will automatically add
  // quotes and escape sequences.
  //
  // Note that Write() is not implemented for 64-bit integers, since they
  // cannot be crisply represented without quotes; use MakeQuoted for that.

  void Write(absl::string_view str) { sink_.Append(str.data(), str.size()); }

  void Write(char c) { sink_.Append(&c, 1); }

  // The precision on this and the following function are completely made-up,
  // in an attempt to match the behavior of the ESF parser.
  void Write(double val) {
    if (!MaybeWriteSpecialFp(val)) {
      Write(io::SimpleDtoa(val));
    }
  }

  void Write(float val) {
    if (!MaybeWriteSpecialFp(val)) {
      Write(io::SimpleFtoa(val));
    }
  }

  void Write(int32_t val) {
    char buf[22];
    int len = absl::SNPrintF(buf, sizeof(buf), "%d", val);
    absl::string_view view(buf, static_cast<size_t>(len));
    Write(view);
  }

  void Write(uint32_t val) {
    char buf[22];
    int len = absl::SNPrintF(buf, sizeof(buf), "%d", val);
    absl::string_view view(buf, static_cast<size_t>(len));
    Write(view);
  }

  void Write(int64_t val) {
    char buf[22];
    int len = absl::SNPrintF(buf, sizeof(buf), "%d", val);
    absl::string_view view(buf, static_cast<size_t>(len));
    Write(view);
  }

  void Write(uint64_t val) {
    char buf[22];
    int len = absl::SNPrintF(buf, sizeof(buf), "%d", val);
    absl::string_view view(buf, static_cast<size_t>(len));
    Write(view);
  }

  template <typename... Ts>
  void Write(Quoted<Ts...> val) {
    Write('"');
    Each(val.value, [this](auto x) { this->WriteQuoted(x); });
    Write('"');
  }

  template <typename... Ts>
  auto Write(Ts... args) ->
      // This bit of SFINAE avoids this function being called with one argument,
      // so the other overloads of Write() can be picked up instead.
      typename std::enable_if<sizeof...(Ts) != 1, void>::type {
    Each(std::make_tuple(args...), [this](auto x) { this->Write(x); });
  }

  void Whitespace(absl::string_view ws) {
    if (options_.add_whitespace) {
      Write(ws);
    }
  }

  void NewLine() {
    Whitespace("\n");
    for (int i = 0; i < indent_; ++i) {
      Whitespace(" ");
    }
  }

  void WriteComma(bool& is_first) {
    if (is_first) {
      is_first = false;
      return;
    }
    Write(",");
  }

  void WriteBase64(absl::string_view str);

  // Returns a buffer that can be re-used throughout a writing session as
  // variable-length scratch space.
  std::string& ScratchBuf() { return scratch_buf_; }

 private:
  template <typename T>
  void WriteQuoted(T val) {
    Write(val);
  }

  void WriteQuoted(absl::string_view val) { WriteEscapedUtf8(val); }

  // Tries to write a non-finite double if necessary; returns false if
  // nothing was written.
  bool MaybeWriteSpecialFp(double val);

  void WriteEscapedUtf8(absl::string_view str);
  void WriteUEscape(uint16_t val);

  io::zc_sink_internal::ZeroCopyStreamByteSink sink_;
  WriterOptions options_;
  int indent_ = 0;

  std::string scratch_buf_;
};
}  // namespace json_internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_JSON_INTERNAL_WRITER_H__
