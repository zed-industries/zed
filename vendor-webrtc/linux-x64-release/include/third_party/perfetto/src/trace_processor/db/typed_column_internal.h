/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_DB_TYPED_COLUMN_INTERNAL_H_
#define SRC_TRACE_PROCESSOR_DB_TYPED_COLUMN_INTERNAL_H_

#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/db/base_id.h"
#include "src/trace_processor/db/column_storage.h"

namespace perfetto {
namespace trace_processor {
namespace tc_internal {

// Serializer converts between the "public" type used by the rest of trace
// processor and the type we store in the ColumnStorage.
template <typename T, typename Enabled = void>
struct Serializer {
  using serialized_type = T;

  static serialized_type Serialize(T value) { return value; }
  static T Deserialize(serialized_type value) { return value; }

  static std::optional<serialized_type> Serialize(std::optional<T> value) {
    return value;
  }
  static std::optional<T> Deserialize(std::optional<serialized_type> value) {
    return value;
  }
};

template <typename T>
using is_id = std::is_base_of<BaseId, T>;

// Specialization of Serializer for id types.
template <typename T>
struct Serializer<T, typename std::enable_if<is_id<T>::value>::type> {
  using serialized_type = uint32_t;

  static serialized_type Serialize(T value) { return value.value; }
  static T Deserialize(serialized_type value) { return T{value}; }

  static std::optional<serialized_type> Serialize(std::optional<T> value) {
    return value ? std::make_optional(Serialize(*value)) : std::nullopt;
  }
  static std::optional<T> Deserialize(std::optional<serialized_type> value) {
    return value ? std::make_optional(Deserialize(*value)) : std::nullopt;
  }
};

// Specialization of Serializer for StringPool types.
template <>
struct Serializer<StringPool::Id> {
  using serialized_type = StringPool::Id;

  static serialized_type Serialize(StringPool::Id value) { return value; }
  static StringPool::Id Deserialize(serialized_type value) { return value; }

  static serialized_type Serialize(std::optional<StringPool::Id> value) {
    // Since StringPool::Id == 0 is always treated as null, rewrite
    // std::nullopt -> 0 to remove an extra check at filter time for
    // std::nullopt. Instead, that code can assume that the ColumnStorage
    // layer always returns a valid id and can handle the nullability at the
    // stringpool level.
    // TODO(lalitm): remove this special casing if we migrate all tables over
    // to macro tables and find that we can remove support for null stringids
    // in the stringpool.
    return value ? Serialize(*value) : StringPool::Id::Null();
  }
  static std::optional<serialized_type> Deserialize(
      std::optional<StringPool::Id> value) {
    return value;
  }
};

// TypeHandler (and it's specializations) allow for specialied handling of
// functions of a TypedColumn based on what is being stored inside.
// Default implementation of TypeHandler.
template <typename T>
struct TypeHandler {
  using non_optional_type = T;
  using sql_value_type =
      typename Serializer<non_optional_type>::serialized_type;
  using stored_type = typename Serializer<non_optional_type>::serialized_type;

  static constexpr bool is_optional = false;
  static constexpr bool is_string = false;

  static stored_type Get(const ColumnStorage<stored_type>& nv, uint32_t idx) {
    return nv.Get(idx);
  }

  static bool Equals(T a, T b) {
    // We need to use equal_to here as it could be T == double and because we
    // enable all compile time warnings, we will get complaints if we just use
    // a == b.
    return std::equal_to<T>()(a, b);
  }
};

// Specialization for Optional types.
template <typename T>
struct TypeHandler<std::optional<T>> {
  using non_optional_type = T;
  using sql_value_type =
      typename Serializer<non_optional_type>::serialized_type;
  using stored_type =
      std::optional<typename Serializer<non_optional_type>::serialized_type>;

  static constexpr bool is_optional = true;
  static constexpr bool is_string = false;

  static stored_type Get(const ColumnStorage<stored_type>& nv, uint32_t idx) {
    return nv.Get(idx);
  }

  static bool Equals(std::optional<T> a, std::optional<T> b) {
    // We need to use equal_to here as it could be T == double and because we
    // enable all compile time warnings, we will get complaints if we just use
    // a == b. This is the same reason why we can't also just use equal_to using
    // a and b directly because the optional implementation of equality uses
    // == which again causes complaints.
    return a.has_value() == b.has_value() &&
           (!a.has_value() || std::equal_to<T>()(*a, *b));
  }
};

// Specialization for std::optional<StringId> types.
template <>
struct TypeHandler<StringPool::Id> {
  using non_optional_type = StringPool::Id;
  using sql_value_type = NullTermStringView;
  using stored_type = StringPool::Id;

  static constexpr bool is_optional = false;
  static constexpr bool is_string = true;

  static StringPool::Id Get(const ColumnStorage<stored_type>& nv,
                            uint32_t idx) {
    return nv.Get(idx);
  }

  static bool Equals(StringPool::Id a, StringPool::Id b) { return a == b; }
};

// Specialization for std::optional<StringId> types.
template <>
struct TypeHandler<std::optional<StringPool::Id>> {
  // get_type removes the base::Optional since we convert std::nullopt ->
  // StringPool::Id::Null (see Serializer<StringPool> above).
  using non_optional_type = StringPool::Id;
  using sql_value_type = NullTermStringView;
  using stored_type = StringPool::Id;

  // is_optional is false again because we always unwrap
  // std::optional<StringPool::Id> into StringPool::Id.
  static constexpr bool is_optional = false;
  static constexpr bool is_string = true;

  static std::optional<StringPool::Id> Get(const ColumnStorage<stored_type>& nv,
                                           uint32_t idx) {
    StringPool::Id id = nv.Get(idx);
    return id.is_null() ? std::nullopt : std::make_optional(id);
  }

  static bool Equals(std::optional<StringPool::Id> a,
                     std::optional<StringPool::Id> b) {
    // To match our handling of treating std::nullopt ==
    // StringPool::Id::Null(), ensure that they both compare equal to each
    // other.
    return a == b || (!a && b->is_null()) || (!b && a->is_null());
  }
};

}  // namespace tc_internal
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_DB_TYPED_COLUMN_INTERNAL_H_
