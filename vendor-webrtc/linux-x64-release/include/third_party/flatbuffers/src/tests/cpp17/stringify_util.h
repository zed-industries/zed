/*
 * Copyright 2020 Google Inc. All rights reserved.
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

// This contains some utilities/examples for how to leverage the static reflec-
// tion features of tables and structs in the C++17 code generation to recur-
// sively produce a string representation of any Flatbuffer table or struct use
// compile-time iteration over the fields. Note that this code is completely
// generic in that it makes no reference to any particular Flatbuffer type.

#include <optional>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "flatbuffers/flatbuffers.h"
#include "flatbuffers/util.h"

namespace cpp17 {

// User calls this; need to forward declare it since it is called recursively.
template<typename T>
std::optional<std::string> StringifyFlatbufferValue(
    T &&val, const std::string &indent = "");

namespace detail {

/*******************************************************************************
** Metaprogramming helpers for detecting Flatbuffers Tables, Structs, & Vectors.
*******************************************************************************/
template<typename FBS, typename = void>
struct is_flatbuffers_table_or_struct : std::false_type {};

// We know it's a table or struct when it has a Traits subclass.
template<typename FBS>
struct is_flatbuffers_table_or_struct<FBS, std::void_t<typename FBS::Traits>>
    : std::true_type {};

template<typename FBS>
inline constexpr bool is_flatbuffers_table_or_struct_v =
    is_flatbuffers_table_or_struct<FBS>::value;

template<typename T> struct is_flatbuffers_vector : std::false_type {};

template<typename T>
struct is_flatbuffers_vector<flatbuffers::Vector<T>> : std::true_type {};

template<typename T>
inline constexpr bool is_flatbuffers_vector_v = is_flatbuffers_vector<T>::value;

/*******************************************************************************
** Compile-time Iteration & Recursive Stringification over Flatbuffers types.
*******************************************************************************/
template<size_t Index, typename FBS>
std::string AddStringifiedField(const FBS &fbs, const std::string &indent) {
  auto value_string =
      StringifyFlatbufferValue(fbs.template get_field<Index>(), indent);
  if (!value_string) { return ""; }
  return indent + FBS::Traits::field_names[Index] + " = " + *value_string +
         "\n";
}

template<typename FBS, size_t... Indexes>
std::string StringifyTableOrStructImpl(const FBS &fbs,
                                       const std::string &indent,
                                       std::index_sequence<Indexes...>) {
  // This line is where the compile-time iteration happens!
  return (AddStringifiedField<Indexes>(fbs, indent) + ...);
}

template<typename FBS>
std::string StringifyTableOrStruct(const FBS &fbs, const std::string &indent) {
  (void)fbs;
  (void)indent;
  static constexpr size_t field_count = FBS::Traits::fields_number;
  std::string out;
  if constexpr (field_count > 0) {
    out = std::string(FBS::Traits::fully_qualified_name) + "{\n" +
          StringifyTableOrStructImpl(fbs, indent + "  ",
                                     std::make_index_sequence<field_count>{}) +
          indent + '}';
  }
  return out;
}

template<typename T>
std::string StringifyVector(const flatbuffers::Vector<T> &vec,
                            const std::string &indent) {
  const auto prologue = indent + std::string("  ");
  const auto epilogue = std::string(",\n");
  std::string text;
  text += "[\n";
  for (auto it = vec.cbegin(), end = vec.cend(); it != end; ++it) {
    text += prologue;
    text += StringifyFlatbufferValue(*it).value_or("(field absent)");
    text += epilogue;
  }
  if (vec.cbegin() != vec.cend()) {
    text.resize(text.size() - epilogue.size());
  }
  text += '\n' + indent + ']';
  return text;
}

template<typename T> std::string StringifyArithmeticType(T val) {
  return flatbuffers::NumToString(val);
}

}  // namespace detail

/*******************************************************************************
** Take any flatbuffer type (table, struct, Vector, int...) and stringify it.
*******************************************************************************/
template<typename T>
std::optional<std::string> StringifyFlatbufferValue(T &&val,
                                                    const std::string &indent) {
  (void)indent;
  constexpr bool is_pointer = std::is_pointer_v<std::remove_reference_t<T>>;
  if constexpr (is_pointer) {
    if (val == nullptr) return std::nullopt;  // Field is absent.
  }
  using decayed =
      std::decay_t<std::remove_pointer_t<std::remove_reference_t<T>>>;

  // Is it a Flatbuffers Table or Struct?
  if constexpr (detail::is_flatbuffers_table_or_struct_v<decayed>) {
    // We have a nested table or struct; use recursion!
    if constexpr (is_pointer)
      return detail::StringifyTableOrStruct(*val, indent);
    else
      return detail::StringifyTableOrStruct(val, indent);
  }

  // Is it an 8-bit number?  If so, print it like an int (not char).
  else if constexpr (std::is_same_v<decayed, int8_t> ||
                     std::is_same_v<decayed, uint8_t>) {
    return detail::StringifyArithmeticType(static_cast<int>(val));
  }

  // Is it an enum? If so, print it like an int, since Flatbuffers doesn't yet
  // have type-based reflection for enums, so we can't print the enum's name :(
  else if constexpr (std::is_enum_v<decayed>) {
    return StringifyFlatbufferValue(
        static_cast<std::underlying_type_t<decayed>>(val), indent);
  }

  // Is it an int, double, float, uint32_t, etc.?
  else if constexpr (std::is_arithmetic_v<decayed>) {
    return detail::StringifyArithmeticType(val);
  }

  // Is it a Flatbuffers string?
  else if constexpr (std::is_same_v<decayed, flatbuffers::String>) {
    return '"' + val->str() + '"';
  }

  // Is it a Flatbuffers Vector?
  else if constexpr (detail::is_flatbuffers_vector_v<decayed>) {
    return detail::StringifyVector(*val, indent);
  }

  // Is it a void pointer?
  else if constexpr (std::is_same_v<decayed, void>) {
    // Can't format it.
    return std::nullopt;
  }

  else {
    // Not sure how to format this type, whatever it is.
    static_assert(sizeof(T) != sizeof(T),
                  "Do not know how to format this type T (the compiler error "
                  "should tell you nearby what T is).");
  }
}

}  // namespace cpp17
