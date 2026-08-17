// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_MAP_FIELD_H__
#define GOOGLE_PROTOBUF_MAP_FIELD_H__

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <functional>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/base/config.h"
#include "absl/hash/hash.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/strings/string_view.h"
#include "absl/synchronization/mutex.h"
#include "absl/types/variant.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/generated_message_reflection.h"
#include "google/protobuf/generated_message_util.h"
#include "google/protobuf/internal_visibility.h"
#include "google/protobuf/map.h"
#include "google/protobuf/map_field_lite.h"
#include "google/protobuf/map_type_handler.h"
#include "google/protobuf/message.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/port.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/unknown_field_set.h"


// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {
class DynamicMessage;
class MapIterator;

// Microsoft compiler complains about non-virtual destructor,
// even when the destructor is private.
#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4265)
#endif  // _MSC_VER

#define TYPE_CHECK(EXPECTEDTYPE, METHOD)                                  \
  if (type() != EXPECTEDTYPE) {                                           \
    ABSL_LOG(FATAL) << "Protocol Buffer map usage error:\n"               \
                    << METHOD << " type does not match\n"                 \
                    << "  Expected : "                                    \
                    << FieldDescriptor::CppTypeName(EXPECTEDTYPE) << "\n" \
                    << "  Actual   : "                                    \
                    << FieldDescriptor::CppTypeName(type());              \
  }

// MapKey is an union type for representing any possible map key. For strings,
// map key does not own the underlying data. It is up to the caller to ensure
// any supplied strings outlive any instance of this class.
class PROTOBUF_EXPORT MapKey {
 public:
  MapKey() = default;
  MapKey(const MapKey&) = default;
  MapKey& operator=(const MapKey&) = default;

  FieldDescriptor::CppType type() const {
    if (type_ == FieldDescriptor::CppType()) {
      ABSL_LOG(FATAL) << "Protocol Buffer map usage error:\n"
                      << "MapKey::type MapKey is not initialized. "
                      << "Call set methods to initialize MapKey.";
    }
    return type_;
  }

  void SetInt64Value(int64_t value) {
    SetType(FieldDescriptor::CPPTYPE_INT64);
    val_.int64_value = value;
  }
  void SetUInt64Value(uint64_t value) {
    SetType(FieldDescriptor::CPPTYPE_UINT64);
    val_.uint64_value = value;
  }
  void SetInt32Value(int32_t value) {
    SetType(FieldDescriptor::CPPTYPE_INT32);
    val_.int32_value = value;
  }
  void SetUInt32Value(uint32_t value) {
    SetType(FieldDescriptor::CPPTYPE_UINT32);
    val_.uint32_value = value;
  }
  void SetBoolValue(bool value) {
    SetType(FieldDescriptor::CPPTYPE_BOOL);
    val_.bool_value = value;
  }
  void SetStringValue(absl::string_view val) {
    SetType(FieldDescriptor::CPPTYPE_STRING);
    val_.string_value = val;
  }

  int64_t GetInt64Value() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_INT64, "MapKey::GetInt64Value");
    return val_.int64_value;
  }
  uint64_t GetUInt64Value() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_UINT64, "MapKey::GetUInt64Value");
    return val_.uint64_value;
  }
  int32_t GetInt32Value() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_INT32, "MapKey::GetInt32Value");
    return val_.int32_value;
  }
  uint32_t GetUInt32Value() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_UINT32, "MapKey::GetUInt32Value");
    return val_.uint32_value;
  }
  bool GetBoolValue() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_BOOL, "MapKey::GetBoolValue");
    return val_.bool_value;
  }
  absl::string_view GetStringValue() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_STRING, "MapKey::GetStringValue");
    return val_.string_value;
  }

  bool operator<(const MapKey& other) const {
    if (type_ != other.type_) {
      // We could define a total order that handles this case, but
      // there currently no need.  So, for now, fail.
      ABSL_LOG(FATAL) << "Unsupported: type mismatch";
    }
    switch (type()) {
      case FieldDescriptor::CPPTYPE_DOUBLE:
      case FieldDescriptor::CPPTYPE_FLOAT:
      case FieldDescriptor::CPPTYPE_ENUM:
      case FieldDescriptor::CPPTYPE_MESSAGE:
        ABSL_LOG(FATAL) << "Unsupported";
        return false;
      case FieldDescriptor::CPPTYPE_STRING:
        return val_.string_value < other.val_.string_value;
      case FieldDescriptor::CPPTYPE_INT64:
        return val_.int64_value < other.val_.int64_value;
      case FieldDescriptor::CPPTYPE_INT32:
        return val_.int32_value < other.val_.int32_value;
      case FieldDescriptor::CPPTYPE_UINT64:
        return val_.uint64_value < other.val_.uint64_value;
      case FieldDescriptor::CPPTYPE_UINT32:
        return val_.uint32_value < other.val_.uint32_value;
      case FieldDescriptor::CPPTYPE_BOOL:
        return val_.bool_value < other.val_.bool_value;
    }
    return false;
  }

  bool operator==(const MapKey& other) const {
    if (type_ != other.type_) {
      // To be consistent with operator<, we don't allow this either.
      ABSL_LOG(FATAL) << "Unsupported: type mismatch";
    }
    switch (type()) {
      case FieldDescriptor::CPPTYPE_DOUBLE:
      case FieldDescriptor::CPPTYPE_FLOAT:
      case FieldDescriptor::CPPTYPE_ENUM:
      case FieldDescriptor::CPPTYPE_MESSAGE:
        ABSL_LOG(FATAL) << "Unsupported";
        break;
      case FieldDescriptor::CPPTYPE_STRING:
        return val_.string_value == other.val_.string_value;
      case FieldDescriptor::CPPTYPE_INT64:
        return val_.int64_value == other.val_.int64_value;
      case FieldDescriptor::CPPTYPE_INT32:
        return val_.int32_value == other.val_.int32_value;
      case FieldDescriptor::CPPTYPE_UINT64:
        return val_.uint64_value == other.val_.uint64_value;
      case FieldDescriptor::CPPTYPE_UINT32:
        return val_.uint32_value == other.val_.uint32_value;
      case FieldDescriptor::CPPTYPE_BOOL:
        return val_.bool_value == other.val_.bool_value;
    }
    ABSL_LOG(FATAL) << "Can't get here.";
    return false;
  }

  void CopyFrom(const MapKey& other) {
    SetType(other.type());
    switch (type_) {
      case FieldDescriptor::CPPTYPE_DOUBLE:
      case FieldDescriptor::CPPTYPE_FLOAT:
      case FieldDescriptor::CPPTYPE_ENUM:
      case FieldDescriptor::CPPTYPE_MESSAGE:
        ABSL_LOG(FATAL) << "Unsupported";
        break;
      case FieldDescriptor::CPPTYPE_STRING:
        val_.string_value = other.val_.string_value;
        break;
      case FieldDescriptor::CPPTYPE_INT64:
        val_.int64_value = other.val_.int64_value;
        break;
      case FieldDescriptor::CPPTYPE_INT32:
        val_.int32_value = other.val_.int32_value;
        break;
      case FieldDescriptor::CPPTYPE_UINT64:
        val_.uint64_value = other.val_.uint64_value;
        break;
      case FieldDescriptor::CPPTYPE_UINT32:
        val_.uint32_value = other.val_.uint32_value;
        break;
      case FieldDescriptor::CPPTYPE_BOOL:
        val_.bool_value = other.val_.bool_value;
        break;
    }
  }

 private:
  template <typename K, typename V>
  friend class internal::TypeDefinedMapFieldBase;
  friend class internal::MapFieldBase;
  friend class MapIterator;

  template <typename H>
  friend auto AbslHashValue(H state, const MapKey& key) {
    switch (key.type()) {
      case FieldDescriptor::CPPTYPE_STRING:
        return H::combine(std::move(state), key.GetStringValue());
      case FieldDescriptor::CPPTYPE_INT64:
        return H::combine(std::move(state), key.GetInt64Value());
      case FieldDescriptor::CPPTYPE_INT32:
        return H::combine(std::move(state), key.GetInt32Value());
      case FieldDescriptor::CPPTYPE_UINT64:
        return H::combine(std::move(state), key.GetUInt64Value());
      case FieldDescriptor::CPPTYPE_UINT32:
        return H::combine(std::move(state), key.GetUInt32Value());
      case FieldDescriptor::CPPTYPE_BOOL:
        return H::combine(std::move(state), key.GetBoolValue());
      default:
        internal::Unreachable();
    }
  }

  union KeyValue {
    KeyValue() {}
    absl::string_view string_value;
    int64_t int64_value;
    int32_t int32_value;
    uint64_t uint64_value;
    uint32_t uint32_value;
    bool bool_value;
  } val_;

  void SetType(FieldDescriptor::CppType type) { type_ = type; }

  // type_ is 0 or a valid FieldDescriptor::CppType.
  // Use "CppType()" to indicate zero.
  FieldDescriptor::CppType type_ = FieldDescriptor::CppType();
};

namespace internal {

template <>
struct is_internal_map_key_type<MapKey> : std::true_type {};

}  // namespace internal

namespace internal {

class ContendedMapCleanTest;
class GeneratedMessageReflection;
class MapFieldAccessor;

template <typename MessageT>
struct MapDynamicFieldInfo;
struct MapFieldTestPeer;

// Return the prototype message for a Map entry.
// REQUIRES: `default_entry` is a map entry message.
// REQUIRES: mapped_type is of type message.
inline const Message& GetMapEntryValuePrototype(const Message& default_entry) {
  return default_entry.GetReflection()->GetMessage(
      default_entry, default_entry.GetDescriptor()->map_value());
}

// This class provides access to map field using reflection, which is the same
// as those provided for RepeatedPtrField<Message>. It is used for internal
// reflection implementation only. Users should never use this directly.
class PROTOBUF_EXPORT MapFieldBase : public MapFieldBaseForParse {
 public:
  explicit constexpr MapFieldBase(const void* prototype_as_void)
      : MapFieldBaseForParse(prototype_as_void) {}
  explicit MapFieldBase(const Message* prototype, Arena* arena)
      : MapFieldBaseForParse(prototype, ToTaggedPtr(arena)) {}
  MapFieldBase(const MapFieldBase&) = delete;
  MapFieldBase& operator=(const MapFieldBase&) = delete;

 protected:
  // "protected" stops users from deleting a `MapFieldBase *`
  ~MapFieldBase();

 public:
  // Returns reference to internal repeated field. Data written using
  // Map's api prior to calling this function is guarantted to be
  // included in repeated field.
  const RepeatedPtrFieldBase& GetRepeatedField() const;

  // Like above. Returns mutable pointer to the internal repeated field.
  RepeatedPtrFieldBase* MutableRepeatedField();

  bool ContainsMapKey(const MapKey& map_key) const {
    return LookupMapValue(map_key, static_cast<MapValueConstRef*>(nullptr));
  }
  bool LookupMapValue(const MapKey& map_key, MapValueConstRef* val) const {
    SyncMapWithRepeatedField();
    return LookupMapValueNoSync(map_key, val);
  }
  bool LookupMapValue(const MapKey&, MapValueRef*) const = delete;

  bool InsertOrLookupMapValue(const MapKey& map_key, MapValueRef* val);

  // Returns whether changes to the map are reflected in the repeated field.
  bool IsRepeatedFieldValid() const;
  // Insures operations after won't get executed before calling this.
  bool IsMapValid() const;
  bool DeleteMapValue(const MapKey& map_key);
  void MergeFrom(const MapFieldBase& other);
  void Swap(MapFieldBase* other);
  void InternalSwap(MapFieldBase* other);
  // Sync Map with repeated field and returns the size of map.
  int size() const;
  void Clear();
  void SetMapIteratorValue(MapIterator* map_iter) const;

  void MapBegin(MapIterator* map_iter) const;
  void MapEnd(MapIterator* map_iter) const;
  bool EqualIterator(const MapIterator& a, const MapIterator& b) const;

  // Returns the number of bytes used by the repeated field, excluding
  // sizeof(*this)
  size_t SpaceUsedExcludingSelfLong() const;

  int SpaceUsedExcludingSelf() const {
    return internal::ToIntSize(SpaceUsedExcludingSelfLong());
  }

  static constexpr size_t InternalGetArenaOffset(internal::InternalVisibility) {
    return PROTOBUF_FIELD_OFFSET(MapFieldBase, payload_);
  }

 protected:
  const Message* GetPrototype() const {
    return reinterpret_cast<const Message*>(prototype_as_void_);
  }
  void ClearMapNoSync();

  // Synchronizes the content in Map to RepeatedPtrField if there is any change
  // to Map after last synchronization.
  const RepeatedPtrFieldBase& SyncRepeatedFieldWithMap(bool for_mutation) const;
  void SyncRepeatedFieldWithMapNoLock();

  // Synchronizes the content in RepeatedPtrField to Map if there is any change
  // to RepeatedPtrField after last synchronization.
  void SyncMapWithRepeatedField() const;
  void SyncMapWithRepeatedFieldNoLock();

  static void SwapPayload(MapFieldBase& lhs, MapFieldBase& rhs);

  // Tells MapFieldBase that there is new change to Map.
  void SetMapDirty() {
    MutableAccess();
    // These are called by (non-const) mutator functions. So by our API it's the
    // callers responsibility to have these calls properly ordered.
    if (auto* p = maybe_payload()) {
      // If we don't have a payload, it is already assumed `STATE_MODIFIED_MAP`.
      p->state.store(STATE_MODIFIED_MAP, std::memory_order_relaxed);
    }
  }

  // Tells MapFieldBase that there is new change to RepeatedPtrField.
  void SetRepeatedDirty();

  // Provides derived class the access to repeated field.
  void* MutableRepeatedPtrField() const;

  bool InsertOrLookupMapValueNoSync(const MapKey& map_key, MapValueRef* val);

  // Support thread sanitizer (tsan) by making const / mutable races
  // more apparent.  If one thread calls MutableAccess() while another
  // thread calls either ConstAccess() or MutableAccess(), on the same
  // MapFieldBase-derived object, and there is no synchronization going
  // on between them, tsan will alert.
#if defined(ABSL_HAVE_THREAD_SANITIZER)
  // Using prototype_as_void_ as an arbitrary member that we can read/write.
  void ConstAccess() const {
    auto* p = prototype_as_void_;
    asm volatile("" : "+r"(p));
  }
  void MutableAccess() {
    auto* p = prototype_as_void_;
    asm volatile("" : "+r"(p));
    prototype_as_void_ = p;
  }
#else
  void ConstAccess() const {}
  void MutableAccess() {}
#endif
  enum State {
    STATE_MODIFIED_MAP = 0,       // map has newly added data that has not been
                                  // synchronized to repeated field
    STATE_MODIFIED_REPEATED = 1,  // repeated field has newly added data that
                                  // has not been synchronized to map
    CLEAN = 2,                    // data in map and repeated field are same
  };

  struct ReflectionPayload {
    explicit ReflectionPayload(Arena* arena) : repeated_field(arena) {}
    RepeatedPtrField<Message> repeated_field;

    absl::Mutex mutex;  // The thread to synchronize map and repeated
                        // field needs to get lock first;
    std::atomic<State> state{STATE_MODIFIED_MAP};
  };

  Arena* arena() const {
    auto p = payload_.load(std::memory_order_acquire);
    if (IsPayload(p)) return ToPayload(p)->repeated_field.GetArena();
    return ToArena(p);
  }

  // Returns the reflection payload. Returns null if it does not exist yet.
  ReflectionPayload* maybe_payload() const {
    auto p = payload_.load(std::memory_order_acquire);
    return IsPayload(p) ? ToPayload(p) : nullptr;
  }
  // Returns the reflection payload, and constructs one if does not exist yet.
  ReflectionPayload& payload() const {
    auto* p = maybe_payload();
    return p != nullptr ? *p : PayloadSlow();
  }
  ReflectionPayload& PayloadSlow() const;

  State state() const {
    auto* p = maybe_payload();
    return p != nullptr ? p->state.load(std::memory_order_acquire)
                        // The default
                        : STATE_MODIFIED_MAP;
  }

 private:
  friend class ContendedMapCleanTest;
  friend class GeneratedMessageReflection;
  friend class MapFieldAccessor;
  friend class google::protobuf::Reflection;
  friend class google::protobuf::DynamicMessage;

  template <typename T, typename... U>
  void InitializeKeyValue(T* v, const U&... init) {
    ::new (static_cast<void*>(v)) T(init...);
    if constexpr (std::is_same_v<std::string, T>) {
      if (arena() != nullptr) {
        arena()->OwnDestructor(v);
      }
    }
  }

  void InitializeKeyValue(MessageLite* msg) {
    GetClassData(GetMapEntryValuePrototype(*GetPrototype()))
        ->PlacementNew(msg, arena());
  }

  // Virtual helper methods for MapIterator. MapIterator doesn't have the
  // type helper for key and value. Call these help methods to deal with
  // different types. Real helper methods are implemented in
  // TypeDefinedMapFieldBase.
  friend class google::protobuf::MapIterator;

  // Copy the map<...>::iterator from other_iterator to
  // this_iterator.
  void CopyIterator(MapIterator* this_iter, const MapIterator& that_iter) const;

  // IncreaseIterator() is called by operator++() of MapIterator only.
  // It implements the ++ operator of MapIterator.
  void IncreaseIterator(MapIterator* map_iter) const;

  bool LookupMapValueNoSync(const MapKey& map_key, MapValueConstRef* val) const;
  static ReflectionPayload* ToPayload(TaggedPtr p) {
    ABSL_DCHECK(IsPayload(p));
    auto* res = reinterpret_cast<ReflectionPayload*>(static_cast<uintptr_t>(p) -
                                                     kHasPayloadBit);
    PROTOBUF_ASSUME(res != nullptr);
    return res;
  }
  static Arena* ToArena(TaggedPtr p) {
    ABSL_DCHECK(!IsPayload(p));
    return reinterpret_cast<Arena*>(p);
  }
  static TaggedPtr ToTaggedPtr(ReflectionPayload* p) {
    return static_cast<TaggedPtr>(reinterpret_cast<uintptr_t>(p) +
                                  kHasPayloadBit);
  }
  static TaggedPtr ToTaggedPtr(Arena* p) {
    return static_cast<TaggedPtr>(reinterpret_cast<uintptr_t>(p));
  }
};

// This class provides common Map Reflection implementations for generated
// message and dynamic message.
template <typename Key, typename T>
class TypeDefinedMapFieldBase : public MapFieldBase {
 public:
  explicit constexpr TypeDefinedMapFieldBase(const void* prototype_as_void)
      : MapFieldBase(prototype_as_void), map_() {
    // This invariant is required by `GetMapRaw` to easily access the map
    // member without paying for dynamic dispatch.
    static_assert(MapFieldBaseForParse::MapOffset() ==
                  PROTOBUF_FIELD_OFFSET(TypeDefinedMapFieldBase, map_));
  }
  TypeDefinedMapFieldBase(const TypeDefinedMapFieldBase&) = delete;
  TypeDefinedMapFieldBase& operator=(const TypeDefinedMapFieldBase&) = delete;

  TypeDefinedMapFieldBase(const Message* prototype, Arena* arena)
      : MapFieldBase(prototype, arena), map_(arena) {}

  TypeDefinedMapFieldBase(const Message* prototype, Arena* arena,
                          const TypeDefinedMapFieldBase& from)
      : MapFieldBase(prototype, arena), map_(arena, from.GetMap()) {}

 protected:
  ~TypeDefinedMapFieldBase() { map_.~Map(); }

 public:
  const Map<Key, T>& GetMap() const {
    SyncMapWithRepeatedField();
    return map_;
  }

  Map<Key, T>* MutableMap() {
    SyncMapWithRepeatedField();
    SetMapDirty();
    return &map_;
  }

  // This overload is called from codegen, so we use templates for speed.
  // If there is no codegen (eg optimize_for=CODE_SIZE), then only the
  // reflection based one above will be used.
  void MergeFrom(const TypeDefinedMapFieldBase& other) {
    internal::MapMergeFrom(*MutableMap(), other.GetMap());
  }

  static constexpr size_t InternalGetArenaOffsetAlt(
      internal::InternalVisibility access) {
    return PROTOBUF_FIELD_OFFSET(TypeDefinedMapFieldBase, map_) +
           decltype(map_)::InternalGetArenaOffset(access);
  }

 protected:
  friend struct MapFieldTestPeer;

  using Iter = typename Map<Key, T>::const_iterator;

  // map_ is inside an anonymous union so we can explicitly control its
  // destruction
  union {
    Map<Key, T> map_;
  };
};

// This class provides access to map field using generated api. It is used for
// internal generated message implementation only. Users should never use this
// directly.
template <typename Derived, typename Key, typename T,
          WireFormatLite::FieldType kKeyFieldType_,
          WireFormatLite::FieldType kValueFieldType_>
class MapField final : public TypeDefinedMapFieldBase<Key, T> {
  // Provide utilities to parse/serialize key/value.  Provide utilities to
  // manipulate internal stored type.
  typedef MapTypeHandler<kKeyFieldType_, Key> KeyTypeHandler;
  typedef MapTypeHandler<kValueFieldType_, T> ValueTypeHandler;

 public:
  typedef Map<Key, T> MapType;
  static constexpr WireFormatLite::FieldType kKeyFieldType = kKeyFieldType_;
  static constexpr WireFormatLite::FieldType kValueFieldType = kValueFieldType_;

  constexpr MapField()
      : MapField::TypeDefinedMapFieldBase(
            Derived::internal_default_instance()) {}
  MapField(const MapField&) = delete;
  MapField& operator=(const MapField&) = delete;
  ~MapField() = default;

  explicit MapField(Arena* arena)
      : TypeDefinedMapFieldBase<Key, T>(
            static_cast<const Message*>(Derived::internal_default_instance()),
            arena) {}
  MapField(ArenaInitialized, Arena* arena) : MapField(arena) {}
  MapField(InternalVisibility, Arena* arena) : MapField(arena) {}
  MapField(InternalVisibility, Arena* arena, const MapField& from)
      : TypeDefinedMapFieldBase<Key, T>(
            static_cast<const Message*>(Derived::internal_default_instance()),
            arena, from) {}

 private:
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;

  friend class google::protobuf::Arena;
  friend class MapFieldBase;
  friend class MapFieldStateTest;  // For testing, it needs raw access to impl_
};

template <typename Key, typename T>
bool AllAreInitialized(const TypeDefinedMapFieldBase<Key, T>& field) {
  for (const auto& p : field.GetMap()) {
    if (!p.second.IsInitialized()) return false;
  }
  return true;
}

}  // namespace internal

// MapValueConstRef points to a map value. Users can NOT modify
// the map value.
class PROTOBUF_EXPORT MapValueConstRef {
 public:
  MapValueConstRef() : data_(nullptr), type_() {}

  int64_t GetInt64Value() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_INT64,
               "MapValueConstRef::GetInt64Value");
    return *reinterpret_cast<int64_t*>(data_);
  }
  uint64_t GetUInt64Value() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_UINT64,
               "MapValueConstRef::GetUInt64Value");
    return *reinterpret_cast<uint64_t*>(data_);
  }
  int32_t GetInt32Value() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_INT32,
               "MapValueConstRef::GetInt32Value");
    return *reinterpret_cast<int32_t*>(data_);
  }
  uint32_t GetUInt32Value() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_UINT32,
               "MapValueConstRef::GetUInt32Value");
    return *reinterpret_cast<uint32_t*>(data_);
  }
  bool GetBoolValue() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_BOOL, "MapValueConstRef::GetBoolValue");
    return *reinterpret_cast<bool*>(data_);
  }
  int GetEnumValue() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_ENUM, "MapValueConstRef::GetEnumValue");
    return *reinterpret_cast<int*>(data_);
  }
  absl::string_view GetStringValue() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_STRING,
               "MapValueConstRef::GetStringValue");
    return absl::string_view(*reinterpret_cast<std::string*>(data_));
  }
  float GetFloatValue() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_FLOAT,
               "MapValueConstRef::GetFloatValue");
    return *reinterpret_cast<float*>(data_);
  }
  double GetDoubleValue() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_DOUBLE,
               "MapValueConstRef::GetDoubleValue");
    return *reinterpret_cast<double*>(data_);
  }

  const Message& GetMessageValue() const {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_MESSAGE,
               "MapValueConstRef::GetMessageValue");
    return *reinterpret_cast<Message*>(data_);
  }

  FieldDescriptor::CppType type() const {
    if (type_ == FieldDescriptor::CppType() || data_ == nullptr) {
      ABSL_LOG(FATAL)
          << "Protocol Buffer map usage error:\n"
          << "MapValueConstRef::type MapValueConstRef is not initialized.";
    }
    return type_;
  }

 protected:
  // data_ point to a map value. MapValueConstRef does not
  // own this value.
  void* data_;
  // type_ is 0 or a valid FieldDescriptor::CppType.
  // Use "CppType()" to indicate zero.
  FieldDescriptor::CppType type_;

 private:
  template <typename Derived, typename K, typename V,
            internal::WireFormatLite::FieldType key_wire_type,
            internal::WireFormatLite::FieldType value_wire_type>
  friend class internal::MapField;
  template <typename K, typename V>
  friend class internal::TypeDefinedMapFieldBase;
  friend class google::protobuf::MapIterator;
  friend class Reflection;
  friend class internal::MapFieldBase;

  void SetValueOrCopy(const void* val) { SetValue(val); }
  void SetValueOrCopy(const MapValueConstRef* val) { CopyFrom(*val); }

  void SetType(FieldDescriptor::CppType type) { type_ = type; }
  void SetValue(const void* val) { data_ = const_cast<void*>(val); }
  void CopyFrom(const MapValueConstRef& other) {
    type_ = other.type_;
    data_ = other.data_;
  }
};

// MapValueRef points to a map value. Users are able to modify
// the map value.
class PROTOBUF_EXPORT MapValueRef final : public MapValueConstRef {
 public:
  MapValueRef() = default;

  void SetInt64Value(int64_t value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_INT64, "MapValueRef::SetInt64Value");
    *reinterpret_cast<int64_t*>(data_) = value;
  }
  void SetUInt64Value(uint64_t value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_UINT64, "MapValueRef::SetUInt64Value");
    *reinterpret_cast<uint64_t*>(data_) = value;
  }
  void SetInt32Value(int32_t value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_INT32, "MapValueRef::SetInt32Value");
    *reinterpret_cast<int32_t*>(data_) = value;
  }
  void SetUInt32Value(uint32_t value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_UINT32, "MapValueRef::SetUInt32Value");
    *reinterpret_cast<uint32_t*>(data_) = value;
  }
  void SetBoolValue(bool value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_BOOL, "MapValueRef::SetBoolValue");
    *reinterpret_cast<bool*>(data_) = value;
  }
  // TODO - Checks that enum is member.
  void SetEnumValue(int value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_ENUM, "MapValueRef::SetEnumValue");
    *reinterpret_cast<int*>(data_) = value;
  }
  void SetStringValue(absl::string_view value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_STRING, "MapValueRef::SetStringValue");
    reinterpret_cast<std::string*>(data_)->assign(value.data(), value.size());
  }
  void SetFloatValue(float value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_FLOAT, "MapValueRef::SetFloatValue");
    *reinterpret_cast<float*>(data_) = value;
  }
  void SetDoubleValue(double value) {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_DOUBLE, "MapValueRef::SetDoubleValue");
    *reinterpret_cast<double*>(data_) = value;
  }

  Message* MutableMessageValue() {
    TYPE_CHECK(FieldDescriptor::CPPTYPE_MESSAGE,
               "MapValueRef::MutableMessageValue");
    return reinterpret_cast<Message*>(data_);
  }
};

#undef TYPE_CHECK

class PROTOBUF_EXPORT MapIterator {
 public:
  MapIterator(Message* message, const FieldDescriptor* field) {
    const Reflection* reflection = message->GetReflection();
    map_ = reflection->MutableMapData(message, field);
    key_.SetType(field->message_type()->map_key()->cpp_type());
    value_.SetType(field->message_type()->map_value()->cpp_type());
  }
  MapIterator(const MapIterator& other) { *this = other; }
  MapIterator& operator=(const MapIterator& other) {
    map_ = other.map_;
    map_->CopyIterator(this, other);
    return *this;
  }
  friend bool operator==(const MapIterator& a, const MapIterator& b) {
    return a.map_->EqualIterator(a, b);
  }
  friend bool operator!=(const MapIterator& a, const MapIterator& b) {
    return !a.map_->EqualIterator(a, b);
  }
  MapIterator& operator++() {
    map_->IncreaseIterator(this);
    return *this;
  }
  MapIterator operator++(int) {
    // iter_ is copied from Map<...>::iterator, no need to
    // copy from its self again. Use the same implementation
    // with operator++()
    map_->IncreaseIterator(this);
    return *this;
  }
  const MapKey& GetKey() { return key_; }
  const MapValueRef& GetValueRef() { return value_; }
  MapValueRef* MutableValueRef() {
    map_->SetMapDirty();
    return &value_;
  }

 private:
  template <typename Key, typename T>
  friend class internal::TypeDefinedMapFieldBase;
  template <typename Derived, typename Key, typename T,
            internal::WireFormatLite::FieldType kKeyFieldType,
            internal::WireFormatLite::FieldType kValueFieldType>
  friend class internal::MapField;
  friend class internal::MapFieldBase;
  template <typename MessageT>
  friend struct internal::MapDynamicFieldInfo;

  MapIterator(internal::MapFieldBase* map, const Descriptor* descriptor) {
    map_ = map;
    key_.SetType(descriptor->map_key()->cpp_type());
    value_.SetType(descriptor->map_value()->cpp_type());
  }

  internal::UntypedMapIterator iter_;
  // Point to a MapField to call helper methods implemented in MapField.
  // MapIterator does not own this object.
  internal::MapFieldBase* map_;
  MapKey key_;
  MapValueRef value_;
};

namespace internal {
template <>
struct is_internal_map_value_type<class MapValueConstRef> : std::true_type {};
template <>
struct is_internal_map_value_type<class MapValueRef> : std::true_type {};
}  // namespace internal

}  // namespace protobuf
}  // namespace google

#ifdef _MSC_VER
#pragma warning(pop)  // restore warning C4265
#endif                // _MSC_VER

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_MAP_FIELD_H__
