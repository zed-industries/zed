// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_FUNCTION_SIGNATURE_H_
#define TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_FUNCTION_SIGNATURE_H_

#include <deque>
#include <string>
#include <string_view>
#include <tuple>
#include <vector>

namespace caspian {
std::vector<std::string_view> SplitBy(std::string_view str, char delim);

// Breaks Java |full_name| into parts.
// If needed, new strings are allocated into |owned_strings|.
// Returns: A tuple of (full_name, template_name, name), where:
//   * full_name = "class_with_package#member(args): type"
//   * template_name = "class_with_package#member"
//   * name = "class_without_package#member"
std::tuple<std::string_view, std::string_view, std::string_view> ParseJava(
    std::string_view full_name,
    std::deque<std::string>* owned_strings);

// Strips return type and breaks function signature into parts.
// See unit tests for example signatures.
// Returns:
//  A tuple of:
//   * name without return type (symbol.full_name),
//   * full_name without params (symbol.template_name),
//   * full_name without params and template args (symbol.name)
std::tuple<std::string_view, std::string_view, std::string_view> ParseCpp(
    std::string_view name,
    std::deque<std::string>* owned_strings);

// Returns the last index of |target_char| that is not within ()s nor <>s.
size_t FindLastCharOutsideOfBrackets(std::string_view name,
                                     char target_char,
                                     size_t prev_idx = std::string::npos);

// Finds index of the "(" that denotes the start of a parameter list.
size_t FindParameterListParen(std::string_view name);

// Returns the index of the space that comes after the return type.
size_t FindReturnValueSpace(std::string_view name, size_t paren_idx);

// Different compilers produce different lambda symbols. These utility
// functions standardize the two, so we can compare between compilers.
std::string NormalizeTopLevelGccLambda(std::string_view name,
                                       size_t left_paren_idx);
std::string NormalizeTopLevelClangLambda(std::string_view name,
                                         size_t left_paren_idx);

// Strips the contents of <>, leaving empty <>s to denote that it's a template.
std::string StripTemplateArgs(std::string_view name);
}  // namespace caspian

#endif  // TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_FUNCTION_SIGNATURE_H_
