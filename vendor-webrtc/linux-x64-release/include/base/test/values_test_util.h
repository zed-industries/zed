// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_VALUES_TEST_UTIL_H_
#define BASE_TEST_VALUES_TEST_UTIL_H_

#include <iosfwd>
#include <memory>
#include <string>
#include <string_view>

#include "base/files/file_path.h"
#include "base/json/json_reader.h"
#include "base/types/expected.h"
#include "base/values.h"
#include "testing/gmock/include/gmock/gmock-matchers.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base::test {
namespace internal {

// Default parsing options for the json util functions. By default, the content
// will be parsed with the default set of Chromium-specific behaviours
// implemented in `JSONReader`, and additionally allowing trailing commas.
inline constexpr int kDefaultJsonParseOptions =
    JSON_PARSE_CHROMIUM_EXTENSIONS | JSON_ALLOW_TRAILING_COMMAS;

class DictionaryHasValueMatcher {
 public:
  DictionaryHasValueMatcher(std::string key, const Value& expected_value);
  DictionaryHasValueMatcher(std::string key, Value&& expected_value);

  DictionaryHasValueMatcher(const DictionaryHasValueMatcher&);
  DictionaryHasValueMatcher& operator=(const DictionaryHasValueMatcher&);
  DictionaryHasValueMatcher(DictionaryHasValueMatcher&&) = default;
  DictionaryHasValueMatcher& operator=(DictionaryHasValueMatcher&&) = default;

  ~DictionaryHasValueMatcher();

  bool MatchAndExplain(const Value::Dict& value,
                       testing::MatchResultListener* listener) const;
  bool MatchAndExplain(const Value& dict,
                       testing::MatchResultListener* listener) const;

  void DescribeTo(std::ostream* os) const;

  void DescribeNegationTo(std::ostream* os) const;

 private:
  std::string key_;
  Value expected_value_;
};

class DictionaryHasValuesMatcher {
 public:
  explicit DictionaryHasValuesMatcher(const Value::Dict& template_value);
  explicit DictionaryHasValuesMatcher(Value::Dict&& template_value);

  DictionaryHasValuesMatcher(const DictionaryHasValuesMatcher&);
  DictionaryHasValuesMatcher& operator=(const DictionaryHasValuesMatcher&);
  DictionaryHasValuesMatcher(DictionaryHasValuesMatcher&&) = default;
  DictionaryHasValuesMatcher& operator=(DictionaryHasValuesMatcher&&) = default;

  ~DictionaryHasValuesMatcher();

  bool MatchAndExplain(const Value::Dict& dict,
                       testing::MatchResultListener* listener) const;
  bool MatchAndExplain(const Value& dict,
                       testing::MatchResultListener* listener) const;

  void DescribeTo(std::ostream* os) const;

  void DescribeNegationTo(std::ostream* os) const;

 private:
  Value::Dict template_value_;
};

class IsSupersetOfValueMatcher {
 public:
  explicit IsSupersetOfValueMatcher(const Value& template_value);
  explicit IsSupersetOfValueMatcher(const Value::Dict& template_value);
  explicit IsSupersetOfValueMatcher(const Value::List& template_value);
  explicit IsSupersetOfValueMatcher(Value&& template_value);
  explicit IsSupersetOfValueMatcher(Value::Dict&& template_value);
  explicit IsSupersetOfValueMatcher(Value::List&& template_value);

  IsSupersetOfValueMatcher(const IsSupersetOfValueMatcher&);
  IsSupersetOfValueMatcher& operator=(const IsSupersetOfValueMatcher&);
  IsSupersetOfValueMatcher(IsSupersetOfValueMatcher&&) = default;
  IsSupersetOfValueMatcher& operator=(IsSupersetOfValueMatcher&&) = default;

  ~IsSupersetOfValueMatcher();

  bool MatchAndExplain(const Value& value,
                       testing::MatchResultListener* listener) const;
  bool MatchAndExplain(const Value::Dict& value,
                       testing::MatchResultListener* listener) const;
  bool MatchAndExplain(const Value::List& value,
                       testing::MatchResultListener* listener) const;

  void DescribeTo(std::ostream* os) const;

  void DescribeNegationTo(std::ostream* os) const;

 private:
  Value template_value_;
};

// A custom GMock matcher.  For details, see
// https://github.com/google/googletest/blob/644319b9f06f6ca9bf69fe791be399061044bc3d/googlemock/docs/CookBook.md#writing-new-polymorphic-matchers
class IsJsonMatcher {
 public:
  explicit IsJsonMatcher(std::string_view json);
  explicit IsJsonMatcher(const Value& value);
  explicit IsJsonMatcher(const Value::Dict& value);
  explicit IsJsonMatcher(const Value::List& value);
  explicit IsJsonMatcher(Value&& value);
  explicit IsJsonMatcher(Value::Dict&& value);
  explicit IsJsonMatcher(Value::List&& value);

  IsJsonMatcher(const IsJsonMatcher& other);
  IsJsonMatcher& operator=(const IsJsonMatcher& other);

  ~IsJsonMatcher();

  bool MatchAndExplain(std::string_view json,
                       testing::MatchResultListener* listener) const;
  bool MatchAndExplain(const Value& value,
                       testing::MatchResultListener* listener) const;
  bool MatchAndExplain(const Value::Dict& dict,
                       testing::MatchResultListener* listener) const;
  bool MatchAndExplain(const Value::List& list,
                       testing::MatchResultListener* listener) const;
  void DescribeTo(std::ostream* os) const;
  void DescribeNegationTo(std::ostream* os) const;

 private:
  Value expected_value_;
};

}  // namespace internal

// A custom GMock matcher which matches if a `base::Value` or
// `base::Value::Dict` has a key `key` that is equal to `value`.
template <typename T>
inline testing::PolymorphicMatcher<internal::DictionaryHasValueMatcher>
DictionaryHasValue(std::string key, T&& expected_value) {
  return testing::MakePolymorphicMatcher(internal::DictionaryHasValueMatcher(
      key, std::forward<T>(expected_value)));
}

// A custom GMock matcher which matches if a `base::Value` or
// `base::Value::Dict` contains all key/value pairs from `template_value`.
template <typename T>
inline testing::PolymorphicMatcher<internal::DictionaryHasValuesMatcher>
DictionaryHasValues(T&& template_value) {
  return testing::MakePolymorphicMatcher(
      internal::DictionaryHasValuesMatcher(std::forward<T>(template_value)));
}

// Matches when a `base::Value` or `base::Value::Dict` or `base::Value::List` is
// a superset of `template_value`, ignoring unexpected Dict keys and list items.
// Uses `testing::DoubleEq` when comparing doubles.
template <typename T>
inline testing::PolymorphicMatcher<internal::IsSupersetOfValueMatcher>
IsSupersetOfValue(T&& template_value) {
  return testing::MakePolymorphicMatcher(
      internal::IsSupersetOfValueMatcher(std::forward<T>(template_value)));
}

// Creates a GMock matcher for testing equivalence of JSON values represented as
// either JSON strings or base::Value objects.  Parsing of the expected value
// uses ParseJson(), which allows trailing commas for convenience.  Parsing of
// the actual value follows the JSON spec strictly.
//
// Although it possible to use this matcher when the actual and expected values
// are both base::Value objects, there is no advantage in that case to using
// this matcher in place of GMock's normal equality semantics.
template <typename T>
inline testing::PolymorphicMatcher<internal::IsJsonMatcher> IsJson(T&& value) {
  return testing::MakePolymorphicMatcher(
      internal::IsJsonMatcher(std::forward<T>(value)));
}

// Parses `json` as JSON, using the provided `options`, and returns the
// resulting value. If `json` fails to parse, causes an EXPECT failure and
// returns the Null Value.
Value ParseJson(std::string_view json,
                int options = internal::kDefaultJsonParseOptions);

// Just like ParseJson(), except returns Dicts/Lists. If `json` fails to parse
// or is not of the expected type, causes an EXPECT failure and returns an empty
// container.
Value::Dict ParseJsonDict(std::string_view json,
                          int options = internal::kDefaultJsonParseOptions);
Value::List ParseJsonList(std::string_view json,
                          int options = internal::kDefaultJsonParseOptions);

// Similar to `ParseJsonDict`, however it loads its contents from a file.
// Returns the parsed `Value::Dict` when successful. Otherwise, it causes an
// EXPECT failure, and returns an empty dict.
Value::Dict ParseJsonDictFromFile(const FilePath& json_file_path);

// An enumaration with the possible types of errors when calling
// `WriteJsonFile`.
enum class WriteJsonError {
  // Failed to generate a json string with the value provided.
  kGenerateJsonFailure,

  // Failed to write the json string into a file.
  kWriteFileFailure,
};

// Serialises `root` as a json string to a file. Returns a empty expected when
// successful. Otherwise returns an error.
expected<void, WriteJsonError> WriteJsonFile(const FilePath& json_file_path,
                                             ValueView root);

}  // namespace base::test

#endif  // BASE_TEST_VALUES_TEST_UTIL_H_
