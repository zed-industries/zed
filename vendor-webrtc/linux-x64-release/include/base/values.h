// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_VALUES_H_
#define BASE_VALUES_H_

#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <array>
#include <compare>
#include <concepts>
#include <initializer_list>
#include <iosfwd>
#include <iterator>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <variant>
#include <vector>

#include "base/base_export.h"
#include "base/bit_cast.h"
#include "base/compiler_specific.h"
#include "base/containers/checked_iterators.h"
#include "base/containers/flat_map.h"
#include "base/containers/span.h"
#include "base/trace_event/base_tracing_forward.h"
#include "base/value_iterators.h"

namespace base {

class DictValue;
class Value;

using BlobStorage = std::vector<uint8_t>;

// Represents a list of Values.
class BASE_EXPORT GSL_OWNER ListValue {
 public:
  using iterator = CheckedContiguousIterator<Value>;
  using const_iterator = CheckedContiguousConstIterator<Value>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;
  using value_type = Value;

  // Creates a list with the given capacity reserved.
  // Correctly using this will greatly reduce the code size and improve
  // performance when creating a list whose size is known up front.
  static ListValue with_capacity(size_t capacity);

  ListValue();

  ListValue(ListValue&&) noexcept;
  ListValue& operator=(ListValue&&) noexcept;

  // Deleted to prevent accidental copying.
  ListValue(const ListValue&) = delete;
  ListValue& operator=(const ListValue&) = delete;

  ~ListValue();

  // Returns true if there are no values in this list and false otherwise.
  bool empty() const;

  // Returns the number of values in this list.
  size_t size() const;

  // Returns an iterator to the first value in this list.
  iterator begin();
  const_iterator begin() const;
  const_iterator cbegin() const;

  // Returns an iterator following the last value in this list. May not be
  // dereferenced.
  iterator end();
  const_iterator end() const;
  const_iterator cend() const;

  // Returns a reverse iterator preceding the first value in this list. May
  // not be dereferenced.
  reverse_iterator rend();
  const_reverse_iterator rend() const;

  // Returns a reverse iterator to the last value in this list.
  reverse_iterator rbegin();
  const_reverse_iterator rbegin() const;

  // Returns a reference to the first value in the container. Fails with
  // `CHECK()` if the list is empty.
  const Value& front() const LIFETIME_BOUND;
  Value& front() LIFETIME_BOUND;

  // Returns a reference to the last value in the container. Fails with
  // `CHECK()` if the list is empty.
  const Value& back() const LIFETIME_BOUND;
  Value& back() LIFETIME_BOUND;

  // Increase the capacity of the backing container, but does not change
  // the size. Assume all existing iterators will be invalidated.
  void reserve(size_t capacity);

  // Resizes the list.
  // If `new_size` is greater than current size, the extra elements in the
  // back will be destroyed.
  // If `new_size` is less than current size, new default-initialized elements
  // will be added to the back.
  // Assume all existing iterators will be invalidated.
  void resize(size_t new_size);

  // Returns a reference to the value at `index` in this list. Fails with a
  // `CHECK()` if `index >= size()`.
  const Value& operator[](size_t index) const;
  Value& operator[](size_t index);

  // Returns true if the specified `val` is present in the list.
  bool contains(bool val) const;
  bool contains(int val) const;
  bool contains(double val) const;
  // Note: std::u16string_view overload intentionally omitted: Value
  // internally stores strings as UTF-8.
  bool contains(std::string_view val) const;
  bool contains(const char* val) const;
  bool contains(const BlobStorage& val) const;
  bool contains(const DictValue& val) const;
  bool contains(const ListValue& val) const;

  // Removes all value from this list.
  REINITIALIZES_AFTER_MOVE void clear();

  // Removes the value referenced by `pos` in this list and returns an
  // iterator to the value following the removed value.
  iterator erase(iterator pos);
  const_iterator erase(const_iterator pos);

  // Remove the values in the range [`first`, `last`). Returns iterator to the
  // first value following the removed range, which is `last`. If `first` ==
  // `last`, removes nothing and returns `last`.
  iterator erase(iterator first, iterator last);
  const_iterator erase(const_iterator first, const_iterator last);

  // Creates a deep copy of this dictionary.
  ListValue Clone() const;

  // Appends `value` to the end of this list.
  void Append(Value&& value) &;
  void Append(bool value) &;
  template <typename T>
  void Append(const T*) & = delete;
  void Append(int value) &;
  void Append(double value) &;
  void Append(std::string_view value) &;
  void Append(std::u16string_view value) &;
  void Append(const char* value) &;
  void Append(const char16_t* value) &;
  void Append(std::string&& value) &;
  void Append(BlobStorage&& value) &;
  void Append(DictValue&& value) &;
  void Append(ListValue&& value) &;

  // Rvalue overrides of the `Append` methods, which allow you to construct
  // a `Value::List` builder-style:
  //
  // Value::List result =
  //   Value::List().Append("first value").Append(2).Append(true);
  //
  // Each method returns a rvalue reference to `this`, so this is as efficient
  // as stand-alone calls to `Append`, while at the same time making it harder
  // to accidentally append to the wrong list.
  //
  // The equivalent code without using these builder-style methods:
  //
  // Value::List no_builder_example;
  // no_builder_example.Append("first value");
  // no_builder_example.Append(2);
  // no_builder_example.Append(true);
  //
  ListValue&& Append(Value&& value) &&;
  ListValue&& Append(bool value) &&;
  template <typename T>
  ListValue&& Append(const T*) && = delete;
  ListValue&& Append(int value) &&;
  ListValue&& Append(double value) &&;
  ListValue&& Append(std::string_view value) &&;
  ListValue&& Append(std::u16string_view value) &&;
  ListValue&& Append(const char* value) &&;
  ListValue&& Append(const char16_t* value) &&;
  ListValue&& Append(std::string&& value) &&;
  ListValue&& Append(BlobStorage&& value) &&;
  ListValue&& Append(DictValue&& value) &&;
  ListValue&& Append(ListValue&& value) &&;

  // Inserts `value` before `pos` in this list. Returns an iterator to the
  // inserted value.
  // TODO(dcheng): Should this provide the same set of overloads that Append()
  // does?
  iterator Insert(const_iterator pos, Value&& value);

  // Erases all values equal to `value` from this list.
  size_t EraseValue(const Value& value);

  // Erases all values for which `predicate` evaluates to true from this list.
  template <typename Predicate>
  size_t EraseIf(Predicate predicate) {
    return std::erase_if(storage_, predicate);
  }

  // Estimates dynamic memory usage. Requires tracing support
  // (enable_base_tracing gn flag), otherwise always returns 0. See
  // base/trace_event/memory_usage_estimator.h for more info.
  size_t EstimateMemoryUsage() const;

  // Serializes to a string for logging and debug purposes.
  std::string DebugString() const;

#if BUILDFLAG(ENABLE_BASE_TRACING)
  // Write this object into a trace.
  void WriteIntoTrace(perfetto::TracedValue) const;
#endif  // BUILDFLAG(ENABLE_BASE_TRACING)

 private:
  using ListStorage = std::vector<Value>;

  friend bool operator==(const ListValue& lhs, const ListValue& rhs) = default;
  BASE_EXPORT friend std::partial_ordering operator<=>(const ListValue& lhs,
                                                       const ListValue& rhs);

  explicit ListValue(const std::vector<Value>& storage);

  // Shared implementation of public `contains()` methods.
  template <typename T, typename R>
    requires std::equality_comparable_with<T, R>
  bool contains(const T& val,
                bool (Value::*test)() const,
                R (Value::*get)() const) const;

  std::vector<Value> storage_;
};

// Represents a dictionary of string keys to Values.
class BASE_EXPORT GSL_OWNER DictValue {
 public:
  using iterator = detail::dict_iterator;
  using const_iterator = detail::const_dict_iterator;
  using value_type = detail::const_dict_iterator::value_type;

  DictValue();

  DictValue(DictValue&&) noexcept;
  DictValue& operator=(DictValue&&) noexcept;

  // Deleted to prevent accidental copying.
  DictValue(const DictValue&) = delete;
  DictValue& operator=(const DictValue&) = delete;

  // Takes move_iterators iterators that return std::pair<std::string, Value>,
  // and moves their values into a new Dict. Adding all entries at once
  // results in a faster initial sort operation. Takes move iterators to avoid
  // having to clone the input.
  template <class IteratorType>
  explicit DictValue(std::move_iterator<IteratorType> first,
                     std::move_iterator<IteratorType> last) {
    // Need to move into a vector first, since `storage_` currently uses
    // unique_ptrs.
    std::vector<std::pair<std::string, std::unique_ptr<Value>>> values;
    for (auto current = first; current != last; ++current) {
      // With move iterators, no need to call Clone(), but do need to move
      // to a temporary first, as accessing either field individually will
      // directly from the iterator will delete the other field.
      auto value = *current;
      values.emplace_back(std::move(value.first),
                          std::make_unique<Value>(std::move(value.second)));
    }
    storage_ = flat_map<std::string, std::unique_ptr<Value>>(std::move(values));
  }

  ~DictValue();

  // Returns true if there are no entries in this dictionary and false
  // otherwise.
  bool empty() const;

  // Returns the number of entries in this dictionary.
  size_t size() const;

  // Returns an iterator to the first entry in this dictionary.
  iterator begin();
  const_iterator begin() const;
  const_iterator cbegin() const;

  // Returns an iterator following the last entry in this dictionary. May not
  // be dereferenced.
  iterator end();
  const_iterator end() const;
  const_iterator cend() const;

  // Returns true if `key` is an entry in this dictionary.
  bool contains(std::string_view key) const;

  // Removes all entries from this dictionary.
  REINITIALIZES_AFTER_MOVE void clear();

  // Removes the entry referenced by `pos` in this dictionary and returns an
  // iterator to the entry following the removed entry.
  iterator erase(iterator pos);
  iterator erase(const_iterator pos);

  // Creates a deep copy of this dictionary.
  DictValue Clone() const;

  // Merges the entries from `dict` into this dictionary. If an entry with the
  // same key exists in this dictionary and `dict`:
  // - if both entries are dictionaries, they will be recursively merged
  // - otherwise, the already-existing entry in this dictionary will be
  //   overwritten with the entry from `dict`.
  void Merge(DictValue dict);

  // Finds the entry corresponding to `key` in this dictionary. Returns
  // nullptr if there is no such entry.
  const Value* Find(std::string_view key) const;
  Value* Find(std::string_view key);

  // Similar to `Find()` above, but returns `std::nullopt`/`nullptr` if the
  // type of the entry does not match. `bool`, `int`, and `double` are
  // returned in a wrapped `std::optional`; blobs, `Value::Dict`, and
  // `Value::List` are returned by pointer.
  std::optional<bool> FindBool(std::string_view key) const;
  std::optional<int> FindInt(std::string_view key) const;
  // Returns a non-null value for both `Value::Type::DOUBLE` and
  // `Value::Type::INT`, converting the latter to a double.
  std::optional<double> FindDouble(std::string_view key) const;
  const std::string* FindString(std::string_view key) const;
  std::string* FindString(std::string_view key);
  const BlobStorage* FindBlob(std::string_view key) const;
  BlobStorage* FindBlob(std::string_view key);
  const DictValue* FindDict(std::string_view key) const;
  DictValue* FindDict(std::string_view key);
  const ListValue* FindList(std::string_view key) const;
  ListValue* FindList(std::string_view key);

  // If there's a value of the specified type at `key` in this dictionary,
  // returns it. Otherwise, creates an empty container of the specified type,
  // inserts it at `key`, and returns it. If there's a value of some other
  // type at `key`, will overwrite that entry.
  DictValue* EnsureDict(std::string_view key);
  ListValue* EnsureList(std::string_view key);

  // Sets an entry with `key` and `value` in this dictionary, overwriting any
  // existing entry with the same `key`. Returns a pointer to the set `value`.
  Value* Set(std::string_view key, Value&& value) &;
  Value* Set(std::string_view key, bool value) &;
  template <typename T>
  Value* Set(std::string_view, const T*) & = delete;
  Value* Set(std::string_view key, int value) &;
  Value* Set(std::string_view key, double value) &;
  Value* Set(std::string_view key, std::string_view value) &;
  Value* Set(std::string_view key, std::u16string_view value) &;
  Value* Set(std::string_view key, const char* value) &;
  Value* Set(std::string_view key, const char16_t* value) &;
  Value* Set(std::string_view key, std::string&& value) &;
  Value* Set(std::string_view key, BlobStorage&& value) &;
  Value* Set(std::string_view key, DictValue&& value) &;
  Value* Set(std::string_view key, ListValue&& value) &;

  // Rvalue overrides of the `Set` methods, which allow you to construct
  // a `Value::Dict` builder-style:
  //
  // Value::Dict result =
  //     Value::Dict()
  //         .Set("key-1", "first value")
  //         .Set("key-2", 2)
  //         .Set("key-3", true)
  //         .Set("nested-dictionary", Value::Dict()
  //                                       .Set("nested-key-1", "value")
  //                                       .Set("nested-key-2", true))
  //         .Set("nested-list", Value::List()
  //                                 .Append("nested-list-value")
  //                                 .Append(5)
  //                                 .Append(true));
  //
  // Each method returns a rvalue reference to `this`, so this is as efficient
  // as stand-alone calls to `Set`, while also making it harder to
  // accidentally insert items in the wrong dictionary.
  //
  // The equivalent code without using these builder-style methods:
  //
  // Value::Dict no_builder_example;
  // no_builder_example.Set("key-1", "first value")
  // no_builder_example.Set("key-2", 2)
  // no_builder_example.Set("key-3", true)
  // Value::Dict nested_dictionary;
  // nested_dictionary.Set("nested-key-1", "value");
  // nested_dictionary.Set("nested-key-2", true);
  // no_builder_example.Set("nested_dictionary",
  //                        std::move(nested_dictionary));
  // Value::List nested_list;
  // nested_list.Append("nested-list-value");
  // nested_list.Append(5);
  // nested_list.Append(true);
  // no_builder_example.Set("nested-list", std::move(nested_list));
  //
  // Sometimes `git cl format` does a less than perfect job formatting these
  // chained `Set` calls. In these cases you can use a trailing empty comment
  // to influence the code formatting:
  //
  // Value::Dict result = Value::Dict().Set(
  //     "nested",
  //     base::Value::Dict().Set("key", "value").Set("other key", "other"));
  //
  // Value::Dict result = Value::Dict().Set("nested",
  //                                        base::Value::Dict() //
  //                                           .Set("key", "value")
  //                                           .Set("other key", "value"));
  //
  DictValue&& Set(std::string_view key, Value&& value) &&;
  DictValue&& Set(std::string_view key, bool value) &&;
  template <typename T>
  DictValue&& Set(std::string_view, const T*) && = delete;
  DictValue&& Set(std::string_view key, int value) &&;
  DictValue&& Set(std::string_view key, double value) &&;
  DictValue&& Set(std::string_view key, std::string_view value) &&;
  DictValue&& Set(std::string_view key, std::u16string_view value) &&;
  DictValue&& Set(std::string_view key, const char* value) &&;
  DictValue&& Set(std::string_view key, const char16_t* value) &&;
  DictValue&& Set(std::string_view key, std::string&& value) &&;
  DictValue&& Set(std::string_view key, BlobStorage&& value) &&;
  DictValue&& Set(std::string_view key, DictValue&& value) &&;
  DictValue&& Set(std::string_view key, ListValue&& value) &&;

  // Removes the entry corresponding to `key` from this dictionary. Returns
  // true if an entry was removed or false otherwise.
  bool Remove(std::string_view key);

  // Similar to `Remove()`, but returns the value corresponding to the removed
  // entry or `std::nullopt` otherwise.
  std::optional<Value> Extract(std::string_view key);

  // Equivalent to the above methods but operating on paths instead of keys.
  // A path is shorthand syntax for referring to a key nested inside
  // intermediate dictionaries, with components delimited by ".". Paths may
  // not be empty.
  //
  // Prefer the non-path methods above when possible. Paths that have only one
  // component (i.e. no dots in the path) should never use the path-based
  // methods.
  //
  // Originally, the path-based APIs were the only way of specifying a key, so
  // there are likely to be many legacy (and unnecessary) uses of the path
  // APIs that do not actually require traversing nested dictionaries.
  const Value* FindByDottedPath(std::string_view path) const;
  Value* FindByDottedPath(std::string_view path);

  std::optional<bool> FindBoolByDottedPath(std::string_view path) const;
  std::optional<int> FindIntByDottedPath(std::string_view path) const;
  // Returns a non-null value for both `Value::Type::DOUBLE` and
  // `Value::Type::INT`, converting the latter to a double.
  std::optional<double> FindDoubleByDottedPath(std::string_view path) const;
  const std::string* FindStringByDottedPath(std::string_view path) const;
  std::string* FindStringByDottedPath(std::string_view path);
  const BlobStorage* FindBlobByDottedPath(std::string_view path) const;
  BlobStorage* FindBlobByDottedPath(std::string_view path);
  const DictValue* FindDictByDottedPath(std::string_view path) const;
  DictValue* FindDictByDottedPath(std::string_view path);
  const ListValue* FindListByDottedPath(std::string_view path) const;
  ListValue* FindListByDottedPath(std::string_view path);

  // Creates a new entry with a dictionary for any non-last component that is
  // missing an entry while performing the path traversal. Will fail if any
  // non-last component of the path refers to an already-existing entry that
  // is not a dictionary. Returns `nullptr` on failure.
  //
  // Warning: repeatedly using this API to enter entries in the same nested
  // dictionary is inefficient, so please do not write the following:
  //
  // bad_example.SetByDottedPath("a.nested.dictionary.field_1", 1);
  // bad_example.SetByDottedPath("a.nested.dictionary.field_2", "value");
  // bad_example.SetByDottedPath("a.nested.dictionary.field_3", 1);
  //
  Value* SetByDottedPath(std::string_view path, Value&& value) &;
  Value* SetByDottedPath(std::string_view path, bool value) &;
  template <typename T>
  Value* SetByDottedPath(std::string_view, const T*) & = delete;
  Value* SetByDottedPath(std::string_view path, int value) &;
  Value* SetByDottedPath(std::string_view path, double value) &;
  Value* SetByDottedPath(std::string_view path, std::string_view value) &;
  Value* SetByDottedPath(std::string_view path, std::u16string_view value) &;
  Value* SetByDottedPath(std::string_view path, const char* value) &;
  Value* SetByDottedPath(std::string_view path, const char16_t* value) &;
  Value* SetByDottedPath(std::string_view path, std::string&& value) &;
  Value* SetByDottedPath(std::string_view path, BlobStorage&& value) &;
  Value* SetByDottedPath(std::string_view path, DictValue&& value) &;
  Value* SetByDottedPath(std::string_view path, ListValue&& value) &;

  // Rvalue overrides of the `SetByDottedPath` methods, which allow you to
  // construct a `Value::Dict` builder-style:
  //
  // Value::Dict result =
  //     Value::Dict()
  //         .SetByDottedPath("a.nested.dictionary.with.key-1", "first value")
  //         .Set("local-key-1", 2));
  //
  // Each method returns a rvalue reference to `this`, so this is as efficient
  // as (and less mistake-prone than) stand-alone calls to `Set`.
  //
  // Warning: repeatedly using this API to enter entries in the same nested
  // dictionary is inefficient, so do not write this:
  //
  // Value::Dict bad_example =
  //   Value::Dict()
  //     .SetByDottedPath("nested.dictionary.key-1", "first value")
  //     .SetByDottedPath("nested.dictionary.key-2", "second value")
  //     .SetByDottedPath("nested.dictionary.key-3", "third value");
  //
  // Instead, simply write this
  //
  // Value::Dict good_example =
  //   Value::Dict()
  //     .Set("nested",
  //          base::Value::Dict()
  //            .Set("dictionary",
  //                 base::Value::Dict()
  //                   .Set(key-1", "first value")
  //                   .Set(key-2", "second value")
  //                   .Set(key-3", "third value")));
  //
  //
  DictValue&& SetByDottedPath(std::string_view path, Value&& value) &&;
  DictValue&& SetByDottedPath(std::string_view path, bool value) &&;
  template <typename T>
  DictValue&& SetByDottedPath(std::string_view, const T*) && = delete;
  DictValue&& SetByDottedPath(std::string_view path, int value) &&;
  DictValue&& SetByDottedPath(std::string_view path, double value) &&;
  DictValue&& SetByDottedPath(std::string_view path, std::string_view value) &&;
  DictValue&& SetByDottedPath(std::string_view path,
                              std::u16string_view value) &&;
  DictValue&& SetByDottedPath(std::string_view path, const char* value) &&;
  DictValue&& SetByDottedPath(std::string_view path, const char16_t* value) &&;
  DictValue&& SetByDottedPath(std::string_view path, std::string&& value) &&;
  DictValue&& SetByDottedPath(std::string_view path, BlobStorage&& value) &&;
  DictValue&& SetByDottedPath(std::string_view path, DictValue&& value) &&;
  DictValue&& SetByDottedPath(std::string_view path, ListValue&& value) &&;

  bool RemoveByDottedPath(std::string_view path);

  std::optional<Value> ExtractByDottedPath(std::string_view path);

  // Estimates dynamic memory usage. Requires tracing support
  // (enable_base_tracing gn flag), otherwise always returns 0. See
  // base/trace_event/memory_usage_estimator.h for more info.
  size_t EstimateMemoryUsage() const;

  // Serializes to a string for logging and debug purposes.
  std::string DebugString() const;

#if BUILDFLAG(ENABLE_BASE_TRACING)
  // Write this object into a trace.
  void WriteIntoTrace(perfetto::TracedValue) const;
#endif  // BUILDFLAG(ENABLE_BASE_TRACING)

 private:
  BASE_EXPORT friend bool operator==(const DictValue& lhs,
                                     const DictValue& rhs);
  BASE_EXPORT friend std::partial_ordering operator<=>(const DictValue& lhs,
                                                       const DictValue& rhs);

  // TODO(dcheng): Replace with `flat_map<std::string, Value>` once no caller
  // relies on stability of pointers anymore.
  flat_map<std::string, std::unique_ptr<Value>> storage_;
};

// The `Value` class is a variant type can hold one of the following types:
// - null
// - bool
// - int
// - double
// - string (internally UTF8-encoded)
// - binary data (i.e. a blob)
// - dictionary of string keys to `Value`s
// - list of `Value`s
//
// With the exception of binary blobs, `Value` is intended to be the C++ version
// of data types that can be represented in JSON.
//
// Warning: blob support may be removed in the future.
//
// ## Usage
//
// Do not use `Value` if a more specific type would be more appropriate.  For
// example, a function that only accepts dictionary values should have a
// `base::Value::Dict` parameter, not a `base::Value` parameter.
//
// Construction:
//
// `Value` is directly constructible from `bool`, `int`, `double`, binary blobs
// (`std::vector<uint8_t>`), `std::string_view`, `std::u16string_view`,
// `Value::Dict`, and `Value::List`.
//
// Copying:
//
// `Value` does not support C++ copy semantics to make it harder to accidentally
// copy large values. Instead, use `Clone()` to manually create a deep copy.
//
// Reading:
//
// `GetBool()`, GetInt()`, et cetera `CHECK()` that the `Value` has the correct
// subtype before returning the contained value. `bool`, `int`, `double` are
// returned by value. Binary blobs, `std::string`, `Value::Dict`, `Value::List`
// are returned by reference.
//
// `GetIfBool()`, `GetIfInt()`, et cetera return `std::nullopt`/`nullptr` if
// the `Value` does not have the correct subtype; otherwise, returns the value
// wrapped in an `std::optional` (for `bool`, `int`, `double`) or by pointer
// (for binary blobs, `std::string`, `Value::Dict`, `Value::List`).
//
// Note: both `GetDouble()` and `GetIfDouble()` still return a non-null result
// when the subtype is `Value::Type::INT`. In that case, the stored value is
// coerced to a double before being returned.
//
// Assignment:
//
// It is not possible to directly assign `bool`, `int`, et cetera to a `Value`.
// Instead, wrap the underlying type in `Value` before assigning.
//
// ## Dictionaries and Lists
//
// `Value` provides the `Value::Dict` and `Value::List` container types for
// working with dictionaries and lists of values respectively, rather than
// exposing the underlying container types directly. This allows the types to
// provide convenient helpers for dictionaries and lists, as well as giving
// greater flexibility for changing implementation details in the future.
//
// Both container types support enough STL-isms to be usable in range-based for
// loops and generic operations such as those from <algorithm>.
//
// Dictionaries support:
// - `empty()`, `size()`, `begin()`, `end()`, `cbegin()`, `cend()`,
//       `contains()`, `clear()`, `erase()`: Identical to the STL container
//       equivalents, with additional safety checks, e.g. iterators will
//       `CHECK()` if `end()` is dereferenced.
//
// - `Clone()`: Create a deep copy.
// - `Merge()`: Merge another dictionary into this dictionary.
// - `Find()`: Find a value by `std::string_view` key, returning nullptr if the
//       key is not present.
// - `FindBool()`, `FindInt()`, ...: Similar to `Find()`, but ensures that the
//       `Value` also has the correct subtype. Same return semantics as
//       `GetIfBool()`, `GetIfInt()`, et cetera, returning `std::nullopt` or
//       `nullptr` if the key is not present or the value has the wrong subtype.
// - `Set()`: Associate a value with a `std::string_view` key. Accepts `Value`
//       or any of the subtypes that `Value` can hold.
// - `Remove()`: Remove the key from this dictionary, if present.
// - `Extract()`: If the key is present in the dictionary, removes the key from
//       the dictionary and transfers ownership of `Value` to the caller.
//       Otherwise, returns `std::nullopt`.
//
// Dictionaries also support an additional set of helper methods that operate on
// "paths": `FindByDottedPath()`, `SetByDottedPath()`, `RemoveByDottedPath()`,
// and `ExtractByDottedPath()`. Dotted paths are a convenience method of naming
// intermediate nested dictionaries, separating the components of the path using
// '.' characters. For example, finding a string path on a `Value::Dict` using
// the dotted path:
//
//   "aaa.bbb.ccc"
//
// Will first look for a `Value::Type::DICT` associated with the key "aaa", then
// another `Value::Type::DICT` under the "aaa" dict associated with the
// key "bbb", and then a `Value::Type::STRING` under the "bbb" dict associated
// with the key "ccc".
//
// If a path only has one component (i.e. has no dots), please use the regular,
// non-path APIs.
//
// Lists support:
// - `empty()`, `size()`, `begin()`, `end()`, `cbegin()`, `cend()`,
//       `rbegin()`, `rend()`, `front()`, `back()`, `reserve()`, `operator[]`,
//       `contains()`, `clear()`, `erase()`: Identical to the STL container
//       equivalents, with additional safety checks, e.g. `operator[]` will
//       `CHECK()` if the index is out of range.
// - `Clone()`: Create a deep copy.
// - `Append()`: Append a value to the end of the list. Accepts `Value` or any
//       of the subtypes that `Value` can hold.
// - `Insert()`: Insert a `Value` at a specified point in the list.
// - `EraseValue()`: Erases all matching `Value`s from the list.
// - `EraseIf()`: Erase all `Value`s matching an arbitrary predicate from the
//       list.
class BASE_EXPORT GSL_OWNER Value {
 public:
  using BlobStorage = BlobStorage;

  using Dict = DictValue;
  using List = ListValue;

  enum class Type : unsigned char {
    NONE = 0,
    BOOLEAN,
    INTEGER,
    DOUBLE,
    STRING,
    BINARY,
    DICT,
    LIST,
    // Note: Do not add more types. See the file-level comment above for why.
  };

  // Adaptors for converting from the old way to the new way and vice versa.
  static Value FromUniquePtrValue(std::unique_ptr<Value> val);
  static std::unique_ptr<Value> ToUniquePtrValue(Value val);

  Value() noexcept;

  Value(Value&&) noexcept;
  Value& operator=(Value&&) noexcept;

  // Deleted to prevent accidental copying.
  Value(const Value&) = delete;
  Value& operator=(const Value&) = delete;

  // Creates a deep copy of this value.
  Value Clone() const;

  // Creates a `Value` of `type`. The data of the corresponding type will be
  // default constructed.
  explicit Value(Type type);

  // Constructor for `Value::Type::BOOLEAN`.
  explicit Value(bool value);

  // Prevent pointers from implicitly converting to bool. Another way to write
  // this would be to template the bool constructor and use SFINAE to only allow
  // use if `std::is_same_v<T, bool>` is true, but this has surprising behavior
  // with range-based for loops over a `std::vector<bool>` (which will
  // unintuitively match the int overload instead).
  //
  // The `const` is load-bearing; otherwise, a `char*` argument would prefer the
  // deleted overload due to requiring a qualification conversion.
  template <typename T>
  explicit Value(const T*) = delete;

  // Constructor for `Value::Type::INT`.
  explicit Value(int value);

  // Constructor for `Value::Type::DOUBLE`.
  explicit Value(double value);

  // Constructors for `Value::Type::STRING`.
  explicit Value(std::string_view value);
  explicit Value(std::u16string_view value);
  // `char*` and `char16_t*` are needed to provide a more specific overload than
  // the deleted `const T*` overload above.
  explicit Value(const char* value);
  explicit Value(const char16_t* value);
  // `std::string&&` allows for efficient move construction.
  explicit Value(std::string&& value) noexcept;

  // Constructors for `Value::Type::BINARY`.
  explicit Value(const std::vector<char>& value);
  explicit Value(base::span<const uint8_t> value);
  explicit Value(BlobStorage&& value) noexcept;

  // Constructor for `Value::Type::DICT`.
  explicit Value(Dict&& value) noexcept;

  // Constructor for `Value::Type::LIST`.
  explicit Value(List&& value) noexcept;

  ~Value();

  // Returns the name for a given `type`.
  static const char* GetTypeName(Type type);

  // Returns the type of the value stored by the current Value object.
  Type type() const { return static_cast<Type>(data_.index()); }

  // Returns true if the current object represents a given type.
  bool is_none() const { return type() == Type::NONE; }
  bool is_bool() const { return type() == Type::BOOLEAN; }
  bool is_int() const { return type() == Type::INTEGER; }
  bool is_double() const { return type() == Type::DOUBLE; }
  bool is_string() const { return type() == Type::STRING; }
  bool is_blob() const { return type() == Type::BINARY; }
  bool is_dict() const { return type() == Type::DICT; }
  bool is_list() const { return type() == Type::LIST; }

  // Returns the stored data if the type matches, or `std::nullopt`/`nullptr`
  // otherwise. `bool`, `int`, and `double` are returned in a wrapped
  // `std::optional`; blobs, `Value::Dict`, and `Value::List` are returned by
  // pointer.
  std::optional<bool> GetIfBool() const;
  std::optional<int> GetIfInt() const;
  // Returns a non-null value for both `Value::Type::DOUBLE` and
  // `Value::Type::INT`, converting the latter to a double.
  std::optional<double> GetIfDouble() const;
  const std::string* GetIfString() const;
  std::string* GetIfString();
  const BlobStorage* GetIfBlob() const;
  BlobStorage* GetIfBlob();
  const Dict* GetIfDict() const;
  Dict* GetIfDict();
  const List* GetIfList() const;
  List* GetIfList();

  // Similar to the `GetIf...()` variants above, but fails with a `CHECK()` on a
  // type mismatch. `bool`, `int`, and `double` are returned by value; blobs,
  // `Value::Dict`, and `Value::List` are returned by reference.
  bool GetBool() const;
  int GetInt() const;
  // Returns a value for both `Value::Type::DOUBLE` and `Value::Type::INT`,
  // converting the latter to a double.
  double GetDouble() const;
  const std::string& GetString() const LIFETIME_BOUND;
  std::string& GetString() LIFETIME_BOUND;
  const BlobStorage& GetBlob() const LIFETIME_BOUND;
  BlobStorage& GetBlob() LIFETIME_BOUND;
  const Dict& GetDict() const LIFETIME_BOUND;
  Dict& GetDict() LIFETIME_BOUND;
  const List& GetList() const LIFETIME_BOUND;
  List& GetList() LIFETIME_BOUND;

  // Transfers ownership of the underlying value. Similarly to `Get...()`
  // variants above, fails with a `CHECK()` on a type mismatch. After
  // transferring the ownership `*this` is in a valid, but unspecified, state.
  // Prefer over `std::move(value.Get...())` so clang-tidy can warn about
  // potential use-after-move mistakes.
  std::string TakeString() &&;
  BlobStorage TakeBlob() &&;
  Dict TakeDict() &&;
  List TakeList() &&;



  // Note: Do not add more types. See the file-level comment above for why.

  // Comparison operators so that Values can easily be used with standard
  // library algorithms and associative containers.
  friend bool operator==(const Value& lhs, const Value& rhs) = default;
  friend auto operator<=>(const Value& lhs, const Value& rhs) = default;

  bool operator==(bool rhs) const;
  template <typename T>
  bool operator==(const T* rhs) const = delete;
  bool operator==(int rhs) const;
  bool operator==(double rhs) const;
  // Note: std::u16string_view overload intentionally omitted: Value internally
  // stores strings as UTF-8. While it is possible to implement a comparison
  // operator that would not require first creating a new UTF-8 string from the
  // UTF-16 string argument, it is simpler to just not implement it at all for a
  // rare use case.
  bool operator==(std::string_view rhs) const;
  bool operator==(const char* rhs) const {
    return *this == std::string_view(rhs);
  }
  bool operator==(const std::string& rhs) const {
    return *this == std::string_view(rhs);
  }
  // Note: Blob support intentionally omitted as an experiment for potentially
  // wholly removing Blob support from Value itself in the future.
  bool operator==(const DictValue& rhs) const;
  bool operator==(const ListValue& rhs) const;

  // Estimates dynamic memory usage. Requires tracing support
  // (enable_base_tracing gn flag), otherwise always returns 0. See
  // base/trace_event/memory_usage_estimator.h for more info.
  size_t EstimateMemoryUsage() const;

  // Serializes to a string for logging and debug purposes.
  std::string DebugString() const;

#if BUILDFLAG(ENABLE_BASE_TRACING)
  // Write this object into a trace.
  void WriteIntoTrace(perfetto::TracedValue) const;
#endif  // BUILDFLAG(ENABLE_BASE_TRACING)

  template <typename Visitor>
  auto Visit(Visitor&& visitor) const {
    return std::visit(std::forward<Visitor>(visitor), data_);
  }

 private:
  // For access to DoubleStorage.
  friend class ValueView;

  // Special case for doubles, which are aligned to 8 bytes on some
  // 32-bit architectures. In this case, a simple declaration as a
  // double member would make the whole union 8 byte-aligned, which
  // would also force 4 bytes of wasted padding space before it in
  // the Value layout.
  //
  // To override this, store the value as an array of 32-bit integers, and
  // perform the appropriate bit casts when reading / writing to it.
  class BASE_EXPORT DoubleStorage {
   public:
    explicit DoubleStorage(double v);
    DoubleStorage(const DoubleStorage&) = default;
    DoubleStorage& operator=(const DoubleStorage&) = default;

    // Provide an implicit conversion to double to simplify the use of visitors
    // with `Value::Visit()`. Otherwise, visitors would need a branch for
    // handling `DoubleStorage` like:
    //
    //   value.Visit([] (const auto& member) {
    //     using T = std::decay_t<decltype(member)>;
    //     if constexpr (std::is_same_v<T, Value::DoubleStorage>) {
    //       SomeFunction(double{member});
    //     } else {
    //       SomeFunction(member);
    //     }
    //   });
    operator double() const { return base::bit_cast<double>(v_); }

   private:
    friend bool operator==(const DoubleStorage& lhs, const DoubleStorage& rhs) {
      return double{lhs} == double{rhs};
    }

    // doubles are partially ordered because NaN is unordered, so anything that
    // can contain a DoubleStorage (ie. Value, List or Dict) must also use
    // partial_ordering. `auto` will deduce this correctly but manually written
    // operators will get cryptic compiler errors if the wrong ordering is
    // chosen. The return type is specified explicitly here to document where
    // the requirement comes from.
    friend std::partial_ordering operator<=>(const DoubleStorage& lhs,
                                             const DoubleStorage& rhs) {
      return double{lhs} <=> double{rhs};
    }

    alignas(4) std::array<char, sizeof(double)> v_;
  };

  // Internal constructors, allowing the simplify the implementation of Clone().
  explicit Value(std::monostate);
  explicit Value(DoubleStorage storage);

  // A helper for static functions used for cloning a Value or a ValueView.
  class CloningHelper;

  std::variant<std::monostate,
               bool,
               int,
               DoubleStorage,
               std::string,
               BlobStorage,
               Dict,
               List>
      data_;
};

// Adapter so `Value::Dict` or `Value::List` can be directly passed to JSON
// serialization methods without having to clone the contents and transfer
// ownership of the clone to a `Value` wrapper object.
//
// Like `std::string_view` and `span<T>`, this adapter does NOT retain
// ownership. Any underlying object that is passed by reference (i.e.
// `std::string`, `Value::BlobStorage`, `Value::Dict`, `Value::List`, or
// `Value`) MUST remain live as long as there is a `ValueView` referencing it.
//
// While it might be nice to just use the `std::variant` type directly, the
// need to use `std::reference_wrapper` makes it clunky. `std::variant` and
// `std::reference_wrapper` both support implicit construction, but C++ only
// allows at most one user-defined conversion in an implicit conversion
// sequence. If this adapter and its implicit constructors did not exist,
// callers would need to use `std::ref` or `std::cref` to pass `Value::Dict` or
// `Value::List` to a function with a `ValueView` parameter.
class BASE_EXPORT GSL_POINTER ValueView {
 public:
  ValueView() = default;
  ValueView(bool value) : data_view_(value) {}
  template <typename T>
  ValueView(const T*) = delete;
  ValueView(int value) : data_view_(value) {}
  ValueView(double value)
      : data_view_(std::in_place_type_t<Value::DoubleStorage>(), value) {}
  ValueView(std::string_view value) : data_view_(value) {}
  ValueView(const char* value) : ValueView(std::string_view(value)) {}
  ValueView(const std::string& value) : ValueView(std::string_view(value)) {}
  // Note: UTF-16 is intentionally not supported. ValueView is intended to be a
  // low-cost view abstraction, but Value internally represents strings as
  // UTF-8, so it would not be possible to implement this without allocating an
  // entirely new UTF-8 string.
  ValueView(const Value::BlobStorage& value) : data_view_(value) {}
  ValueView(const Value::Dict& value) : data_view_(value) {}
  ValueView(const Value::List& value) : data_view_(value) {}
  ValueView(const Value& value);

  // This is the only 'getter' method provided as `ValueView` is not intended
  // to be a general replacement of `Value`.
  template <typename Visitor>
  auto Visit(Visitor&& visitor) const {
    return std::visit(std::forward<Visitor>(visitor), data_view_);
  }

  // Returns a clone of the underlying Value.
  Value ToValue() const;

 private:
  using ViewType =
      std::variant<std::monostate,
                   bool,
                   int,
                   Value::DoubleStorage,
                   std::string_view,
                   std::reference_wrapper<const Value::BlobStorage>,
                   std::reference_wrapper<const Value::Dict>,
                   std::reference_wrapper<const Value::List>>;

 public:
  using DoubleStorageForTest = Value::DoubleStorage;
  const ViewType& data_view_for_test() const { return data_view_; }

 private:
  ViewType data_view_;
};

// This interface is implemented by classes that know how to serialize
// Value objects.
class BASE_EXPORT ValueSerializer {
 public:
  virtual ~ValueSerializer();

  virtual bool Serialize(ValueView root) = 0;
};

// This interface is implemented by classes that know how to deserialize Value
// objects.
class BASE_EXPORT ValueDeserializer {
 public:
  virtual ~ValueDeserializer();

  // This method deserializes the subclass-specific format into a Value object.
  // If the return value is non-NULL, the caller takes ownership of returned
  // Value.
  //
  // If the return value is nullptr, and if `error_code` is non-nullptr,
  // `*error_code` will be set to an integer value representing the underlying
  // error. See "enum ErrorCode" below for more detail about the integer value.
  //
  // If `error_message` is non-nullptr, it will be filled in with a formatted
  // error message including the location of the error if appropriate.
  virtual std::unique_ptr<Value> Deserialize(int* error_code,
                                             std::string* error_message) = 0;

  // The integer-valued error codes form four groups:
  //  - The value 0 means no error.
  //  - Values between 1 and 999 inclusive mean an error in the data (i.e.
  //    content). The bytes being deserialized are not in the right format.
  //  - Values 1000 and above mean an error in the metadata (i.e. context). The
  //    file could not be read, the network is down, etc.
  //  - Negative values are reserved.
  //
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum ErrorCode {
    kErrorCodeNoError = 0,
    // kErrorCodeInvalidFormat is a generic error code for "the data is not in
    // the right format". Subclasses of ValueDeserializer may return other
    // values for more specific errors.
    kErrorCodeInvalidFormat = 1,
    // kErrorCodeFirstMetadataError is the minimum value (inclusive) of the
    // range of metadata errors.
    kErrorCodeFirstMetadataError = 1000,
  };

  // The `error_code` argument can be one of the ErrorCode values, but it is
  // not restricted to only being 0, 1 or 1000. Subclasses of ValueDeserializer
  // can define their own error code values.
  static inline bool ErrorCodeIsDataError(int error_code) {
    return (kErrorCodeInvalidFormat <= error_code) &&
           (error_code < kErrorCodeFirstMetadataError);
  }
};

// Stream operator so Values can be pretty printed by gtest.
BASE_EXPORT std::ostream& operator<<(std::ostream& out, const Value& value);
BASE_EXPORT std::ostream& operator<<(std::ostream& out,
                                     const Value::Dict& dict);
BASE_EXPORT std::ostream& operator<<(std::ostream& out,
                                     const Value::List& list);

// Stream operator so that enum class Types can be used in log statements.
BASE_EXPORT std::ostream& operator<<(std::ostream& out,
                                     const Value::Type& type);

template <typename T, typename R>
  requires std::equality_comparable_with<T, R>
bool ListValue::contains(const T& val,
                         bool (Value::*test)() const,
                         R (Value::*get)() const) const {
  return std::ranges::any_of(storage_, [&](const Value& value) {
    return (value.*test)() && (value.*get)() == val;
  });
}

}  // namespace base

#endif  // BASE_VALUES_H_
