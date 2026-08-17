// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This header defines static maps to store the mappings from type hash id and
// name string to MediaPipeTypeData.  It also provides code to inspect types of
// packets and access registered serialize and deserialize functions.
// Calculators can use this to infer types of packets and adjust accordingly.
//
// Register a type:
//    // If the generic serializer can serialize your type use:
//    MEDIAPIPE_REGISTER_GENERIC_TYPE(::namespace::Type);
//
//    // If your type includes commas, you need to use a macro:
//    #define MY_PAIR_TYPE ::std::pair<int, float>
//    MEDIAPIPE_REGISTER_GENERIC_TYPE_WITH_NAME(MY_PAIR_TYPE,
//                                            "::std::pair<int,float>");
//    #undef MY_PAIR_TYPE
//
//    // If you need more control over the serialization functions you can
//    // specify them directly.
//    MEDIAPIPE_REGISTER_TYPE(
//        ::namespace::Type, "::namespace::Type",
//        ::mediapipe::SerializeUsingGenericFn<::namespace::Type>,
//        ::mediapipe::DeserializeUsingGenericFn<::namespace::Type>);
//
//    // If your type is serialized by converting it to an easily serializable
//    // type (such as a proto) use a proxy.
//    // See mediapipe/framework/formats/location.cc for more
//    details. MEDIAPIPE_REGISTER_TYPE_WITH_PROXY(
//        mediapipe::Location, "mediapipe::Location",
//        ::mediapipe::SerializeUsingGenericFn<Location WITH_MEDIAPIPE_PROXY
//        LocationData>,
//        ::mediapipe::DeserializeUsingGenericFn<Location WITH_MEDIAPIPE_PROXY
//        LocationData>, LocationToLocationData, LocationFromLocationData);
//
// Inspect type:
//     const std::string* result = MediaPipeTypeString<CustomStruct>();
//     if (result && *result == "CustomStruct") ...
//
// Compare type hash id's:
//     const size_t* complex_type_id = MediaPipeTypeId("ComplexStruct");
//     if (complex_type_id && *complex_type_id ==
//     tool::GetTypeHash<std::string>())
//       ...
//

#ifndef MEDIAPIPE_FRAMEWORK_TYPE_MAP_H_
#define MEDIAPIPE_FRAMEWORK_TYPE_MAP_H_

#include <functional>
#include <map>
#include <memory>
#include <vector>

#include "absl/base/macros.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/demangle.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/tool/status_util.h"
#include "mediapipe/framework/tool/type_util.h"

namespace mediapipe {

namespace packet_internal {
class HolderBase;
}  // namespace packet_internal

// These functions use HolderBase to hide the type T from the function
// definition.  This allows these functions to be placed into an untyped
// struct in the map of MediaPipeTypeData objects.
using SerializeFn = std::function<absl::Status(
    const packet_internal::HolderBase& holder_base, std::string* output)>;
using DeserializeFn = std::function<absl::Status(
    const std::string& encoding,
    std::unique_ptr<packet_internal::HolderBase>* holder_base)>;

struct MediaPipeTypeData {
  size_t type_id;
  std::string type_string;
  SerializeFn serialize_fn;
  DeserializeFn deserialize_fn;
};

namespace type_map_internal {
// ReflectType allows macros to enclose a type which may contain a comma in
// parentheses so that macros don't misinterpet the type. It's needed
// because a macro calls another macro.
template <typename>
struct ReflectType;
// ReflectType<void(std::map<std::string, int>*)>::Type resolves to type
// std::map<std::string, int> while enclosing the comma using type in
// parentheses for the preprocessor.  Unfortunately, there is no way
// to deal both with parenthesized types and with abstract classes.
// We choose to allow abstract classes (by using T* instead of T).
template <typename T>
struct ReflectType<void(T*)> {
  typedef T Type;
};

// This static map implementation is for mediapipe type registration use only.
// It is a revision based on
// util/registration/static_map.h to support mediapipe type registration
// with/without serialization functions. Note that serialization functions
// should only be defined once within the same macro invocation.
template <typename MapName, typename KeyType>
class StaticMap {
 public:
  typedef std::map<KeyType, std::pair<std::string, MediaPipeTypeData>> MapType;
  StaticMap(const StaticMap&) = delete;
  StaticMap& operator=(const StaticMap&) = delete;
  static const MediaPipeTypeData* GetValue(const KeyType& key) {
    const MapType& internal_map = GetMap()->internal_map_;
    typename MapType::const_iterator value_iter = internal_map.find(key);
    return (value_iter == internal_map.end()) ? nullptr
                                              : &(value_iter->second.second);
  }

  static void GetKeys(std::vector<KeyType>* keys) {
    ABSL_CHECK(keys);
    keys->clear();
    const MapType& internal_map = GetMap()->internal_map_;
    for (typename MapType::const_iterator i = internal_map.begin();
         i != internal_map.end(); ++i) {
      keys->push_back(i->first);
    }
  }

  static std::vector<KeyType> Keys() {
    std::vector<KeyType> keys;
    GetKeys(&keys);
    return keys;
  }

  class ValueInserter {
   public:
    ValueInserter(const char* file_and_line, const KeyType& key,
                  const MediaPipeTypeData& value) {
      StaticMap* static_map = GetMap();
      absl::MutexLock l(&(static_map->map_lock_));

      typename MapType::iterator it = static_map->internal_map_.find(key);
      if (it == static_map->internal_map_.end()) {
        static_map->internal_map_.emplace(key,
                                          std::make_pair(file_and_line, value));
        return;
      }

      // Type has been already registered.
      const MediaPipeTypeData& existing_data = it->second.second;
      ABSL_CHECK_EQ(existing_data.type_id, value.type_id)
          << "Found inconsistent type ids (" << existing_data.type_id << " vs "
          << value.type_id
          << ") during mediapipe type registration. Previous definition at "
          << it->second.first << " and current definition at " << file_and_line;
      ABSL_CHECK_EQ(existing_data.type_string, value.type_string)
          << "Found inconsistent type strings (" << existing_data.type_string
          << " vs " << value.type_string
          << ") during mediapipe type registration. Previous registration at "
          << it->second.first << " and current registration at "
          << file_and_line;
      if (value.serialize_fn && value.deserialize_fn) {
        // Doesn't allow to redefine the existing type serialization functions.
        ABSL_CHECK(!existing_data.serialize_fn && !existing_data.deserialize_fn)
            << "Attempting to redefine serialization functions of type "
            << value.type_string << ", that have been defined at "
            << it->second.first << ", at " << file_and_line;
        const std::string previous_file_and_line = it->second.first;
        it->second.first = file_and_line;
        it->second.second = value;
        ABSL_LOG(WARNING) << "Redo mediapipe type registration of type "
                          << value.type_string
                          << " with serialization function at " << file_and_line
                          << ". It was registered at "
                          << previous_file_and_line;
      } else if (!value.serialize_fn && !value.deserialize_fn) {
        // Prefers type registration with serialization functions. If type has
        // been registered with some serialization functions, the
        // non-serialization version will be ignored.
        ABSL_LOG(WARNING)
            << "Ignore mediapipe type registration of type "
            << value.type_string << " at " << file_and_line
            << ", since type has been registered with serialization "
               "functions at "
            << it->second.first;
      } else {
        // Doesn't allow to only have one of serialize_fn and deserialize_fn.
        ABSL_LOG(FATAL)
            << "Invalid mediapipe type registration at " << file_and_line
            << ". Serialization functions should be provided at the same time.";
      }
    }
  };

 protected:
  StaticMap() {}

 private:
  friend class StaticMap::ValueInserter;

  // Returns a pointer to the one true instance of MapName class.
  static MapName* GetMap() {
    // TODO: Uses gtl::NoDestructor for the thread-safe one-time
    // initialization if gtl::NoDestructor will be open sourced by ABSL.
    static MapName* instance = new MapName();
    return instance;
  }

  absl::Mutex map_lock_;
  MapType internal_map_;
};
}  // namespace type_map_internal

// Helper macros used to concatenate three strings.
#define MEDIAPIPE_STRING_CONCAT_HELPER(a, b, c) a##b##_##c
#define MEDIAPIPE_STRING_CONCAT(a, b, c) MEDIAPIPE_STRING_CONCAT_HELPER(a, b, c)
// MEDIAPIPE_STRINGIFY is a helper macro to effectively apply # operator
// to an arbitrary value.
#define MEDIAPIPE_STRINGIFY_HELPER(x) #x
#define MEDIAPIPE_STRINGIFY(x) MEDIAPIPE_STRINGIFY_HELPER(x)

// Unique object name used for temp instances of ValueInserter. Including a
// counter is important to allow use of this macro nested inside multiple levels
// of macros.
#define TYPE_MAP_TEMP_OBJECT_NAME \
  MEDIAPIPE_STRING_CONCAT(obj_, __LINE__, __COUNTER__)
#define FILE_LINE __FILE__ ":line" MEDIAPIPE_STRINGIFY(__LINE__)
#define SET_MEDIAPIPE_TYPE_MAP_VALUE(map_name, key, value)                 \
  static map_name::ValueInserter TYPE_MAP_TEMP_OBJECT_NAME(FILE_LINE, key, \
                                                           value);

// Defines a static mediapipe type map.
#define DEFINE_MEDIAPIPE_TYPE_MAP(MapName, KeyType) \
  class MapName : public type_map_internal::StaticMap<MapName, KeyType> {};
// Defines a map from unique typeid number to MediaPipeTypeData.
DEFINE_MEDIAPIPE_TYPE_MAP(PacketTypeIdToMediaPipeTypeData, size_t)
// Defines a map from unique type string to MediaPipeTypeData.
DEFINE_MEDIAPIPE_TYPE_MAP(PacketTypeStringToMediaPipeTypeData, std::string)

// MEDIAPIPE_REGISTER_TYPE can be used to register a type.
// Convention:
//     Use the C++ reference of the type as the type_string with
//     leading double colon. Don't use whitespace in type_string.
//     Even std types should have their names start with "::std".
//     Only basic types such as "int" can be left bare.  Remember to
//     include full namespaces for template arguments.  For example
//     "::map<std::string,mediapipe::Packet>".
//
// Examples:
//   Prefers an additional macro to define a type that contains comma(s) in
//   advance to make macro expression work correctly. STL containers and
//   template classes with multiple template arguments should apply this
//   trick.
//
//   #define MY_MAP_TYPE ::std::map<std::string, int>
//   MEDIAPIPE_REGISTER_TYPE(MY_MAP_TYPE, "::std::map<std::string,int>",
//                         ::mediapipe::SerializeUsingGenericFn<MY_MAP_TYPE>,
//                         ::mediapipe::DeserializeUsingGenericFn<MY_MAP_TYPE>);
//   #undef MY_MAP_TYPE
//
//   MEDIAPIPE_REGISTER_TYPE(
//       std::string, "std::string", StringSerializeFn, StringDeserializeFn);
//
#define MEDIAPIPE_REGISTER_TYPE(type, type_name, serialize_fn, deserialize_fn) \
  SET_MEDIAPIPE_TYPE_MAP_VALUE(                                                \
      mediapipe::PacketTypeIdToMediaPipeTypeData,                              \
      mediapipe::TypeId::Of<                                                   \
          mediapipe::type_map_internal::ReflectType<void(type*)>::Type>()      \
          .hash_code(),                                                        \
      (mediapipe::MediaPipeTypeData{                                           \
          mediapipe::TypeId::Of<                                               \
              mediapipe::type_map_internal::ReflectType<void(type*)>::Type>()  \
              .hash_code(),                                                    \
          type_name, serialize_fn, deserialize_fn}));                          \
  SET_MEDIAPIPE_TYPE_MAP_VALUE(                                                \
      mediapipe::PacketTypeStringToMediaPipeTypeData, type_name,               \
      (mediapipe::MediaPipeTypeData{                                           \
          mediapipe::TypeId::Of<                                               \
              mediapipe::type_map_internal::ReflectType<void(type*)>::Type>()  \
              .hash_code(),                                                    \
          type_name, serialize_fn, deserialize_fn}));
// End define MEDIAPIPE_REGISTER_TYPE.

// MEDIAPIPE_REGISTER_TYPE_WITH_PROXY can be used to register a type with its
// serialization proxy.
// Convention: use the C++ reference of the type as the type_string with leading
//     double colon if possible. Don't use whitespace in type_string. If a
//     typedef is used, the name should be prefixed with the namespace(s),
//     seperated by double colons.
//
// Example 1: register type with non-string proxy.
//   absl::Status ToProxyFn(
//       const ClassType& obj, ProxyType* proxy)
//   {
//     ...
//     return absl::OkStatus();
//   }
//
//   absl::Status FromProxyFn(
//       const ProxyType& proxy, ClassType* obj)
//   {
//     ...
//     return absl::OkStatus();
//   }
//
//   MEDIAPIPE_REGISTER_TYPE_WITH_PROXY(
//      ClassType, "ClassTypeName",
//      ::mediapipe::SerializeUsingGenericFn<ClassType WITH_MEDIAPIPE_PROXY
//      ProxyType>,
//      ::mediapipe::DeserializeUsingGenericFn<ClassType WITH_MEDIAPIPE_PROXY
//      ProxyType>, ToProxyFn, FromProxyFn);
//
// Example 2: register type with string proxy.
//   absl::Status ToProxyFn(const ClassType& obj, string* encoding)
//   {
//     ...
//     return absl::OkStatus();
//   }
//
//   absl::Status FromProxyFn(
//       const ProxyType& proxy, string* encoding) {
//     ...
//     return absl::OkStatus();
//   }
//
//   MEDIAPIPE_REGISTER_TYPE_WITH_PROXY(
//      ClassType, "ClassTypeName",
//      SerializeToString<ClassType>, DeserializeFromString<ClassType>,
//      ToProxyFn, FromProxyFn);
//
#define WITH_MEDIAPIPE_PROXY ,
#define MEDIAPIPE_REGISTER_TYPE_WITH_PROXY(                                    \
    type, type_name, serialize_fn, deserialize_fn, to_proxy_fn, from_proxy_fn) \
  SET_MEDIAPIPE_TYPE_MAP_VALUE(                                                \
      mediapipe::PacketTypeIdToMediaPipeTypeData,                              \
      mediapipe::tool::GetTypeHash<                                            \
          mediapipe::type_map_internal::ReflectType<void(type*)>::Type>(),     \
      (mediapipe::MediaPipeTypeData{                                           \
          mediapipe::tool::GetTypeHash<                                        \
              mediapipe::type_map_internal::ReflectType<void(type*)>::Type>(), \
          type_name,                                                           \
          std::bind(&serialize_fn, to_proxy_fn, std::placeholders::_1,         \
                    std::placeholders::_2),                                    \
          std::bind(&deserialize_fn, from_proxy_fn, std::placeholders::_1,     \
                    std::placeholders::_2)}));                                 \
  SET_MEDIAPIPE_TYPE_MAP_VALUE(                                                \
      mediapipe::PacketTypeStringToMediaPipeTypeData, type_name,               \
      (mediapipe::MediaPipeTypeData{                                           \
          mediapipe::tool::GetTypeHash<                                        \
              mediapipe::type_map_internal::ReflectType<void(type*)>::Type>(), \
          type_name,                                                           \
          std::bind(&serialize_fn, to_proxy_fn, std::placeholders::_1,         \
                    std::placeholders::_2),                                    \
          std::bind(&deserialize_fn, from_proxy_fn, std::placeholders::_1,     \
                    std::placeholders::_2)}));
// End define MEDIAPIPE_REGISTER_TYPE_WITH_PROXY.

// Helper functions's to retrieve registration data.
inline const std::string* MediaPipeTypeStringFromTypeId(TypeId type_id) {
  const MediaPipeTypeData* value =
      PacketTypeIdToMediaPipeTypeData::GetValue(type_id.hash_code());
  return (value) ? &value->type_string : nullptr;
}

// Returns string identifier of type or NULL if not registered.
template <typename T>
inline const std::string* MediaPipeTypeString() {
  return MediaPipeTypeStringFromTypeId(kTypeId<T>);
}

inline std::string MediaPipeTypeStringOrDemangled(TypeId type_id) {
  const std::string* type_string = MediaPipeTypeStringFromTypeId(type_id);
  if (type_string) {
    return *type_string;
  } else {
    return type_id.name();
  }
}

template <typename T>
std::string MediaPipeTypeStringOrDemangled() {
  return MediaPipeTypeStringOrDemangled(kTypeId<T>);
}

// Returns type hash id of type identified by type_string or NULL if not
// registered.
inline const size_t* MediaPipeTypeId(const std::string& type_string) {
  const MediaPipeTypeData* value =
      PacketTypeStringToMediaPipeTypeData::GetValue(type_string);
  return (value) ? &value->type_id : nullptr;
}

// Returns true if serialize and deserialize functions are both registered.
inline bool SerializeFunctionsAreRegistered(const size_t type_id) {
  const MediaPipeTypeData* mediapipe_type_data =
      PacketTypeIdToMediaPipeTypeData::GetValue(type_id);
  return mediapipe_type_data && mediapipe_type_data->serialize_fn &&
         mediapipe_type_data->deserialize_fn;
}

// Returns true if serialize and deserialize functions are both registered.
inline bool SerializeFunctionsAreRegistered(const std::string& type_string) {
  const MediaPipeTypeData* mediapipe_type_data =
      PacketTypeStringToMediaPipeTypeData::GetValue(type_string);
  return mediapipe_type_data && mediapipe_type_data->serialize_fn &&
         mediapipe_type_data->deserialize_fn;
}
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TYPE_MAP_H_
