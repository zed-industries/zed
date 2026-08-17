// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_WEAK_IDENTIFIER_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_WEAK_IDENTIFIER_MAP_H_

#include <limits>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// TODO(sof): WeakIdentifierMap<> belongs (out) in wtf/, but
// cannot until GarbageCollected<> can be used from WTF.

template <typename T, typename IdentifierType = int>
class WeakIdentifierMap final
    : public GarbageCollected<WeakIdentifierMap<T, IdentifierType>> {
 public:
  WeakIdentifierMap() = default;

  void Trace(Visitor* visitor) const {
    visitor->Trace(object_to_identifier_);
    visitor->Trace(identifier_to_object_);
  }

  static IdentifierType Identifier(T* object) {
    IdentifierType result;

    auto it = Instance().object_to_identifier_.find(object);
    if (it == Instance().object_to_identifier_.end()) {
      result = Next();
      while (!Instance().Put(object, result)) [[unlikely]] {
        result = Next();
      }
    } else {
      result = it->value;
    }
    return result;
  }

  // If the object is not found, returns 0 which is not a valid identifier.
  static IdentifierType ExistingIdentifier(T* object) {
    auto it_result = Instance().object_to_identifier_.find(object);
    return it_result != Instance().object_to_identifier_.end()
               ? it_result->value
               : 0;
  }

  static T* Lookup(IdentifierType identifier) {
    auto it = Instance().identifier_to_object_.find(identifier);
    return it != Instance().identifier_to_object_.end() ? it->value : nullptr;
  }

  static void NotifyObjectDestroyed(T* object) {
    Instance().ObjectDestroyed(object);
  }

  static void SetLastIdForTesting(IdentifierType i) { LastIdRef() = i; }

  static size_t GetSizeForTesting() {
    return Instance().object_to_identifier_.size();
  }

 private:
  static WeakIdentifierMap<T, IdentifierType>& Instance();

  static IdentifierType Next() {
    // On overflow, skip negative values for signed IdentifierType, and 0 which
    // is not a valid key in HashMap by default.
    if (LastIdRef() == std::numeric_limits<IdentifierType>::max())
        [[unlikely]] {
      LastIdRef() = 0;
    }
    return ++LastIdRef();
  }

  static IdentifierType& LastIdRef() {
    static IdentifierType last_id = 0;
    return last_id;
  }

  bool Put(T* object, IdentifierType identifier) {
    if (!identifier_to_object_.insert(identifier, object).is_new_entry)
        [[unlikely]] {
      return false;
    }
    DCHECK(object && !object_to_identifier_.Contains(object));
    object_to_identifier_.Set(object, identifier);
    DCHECK_EQ(object_to_identifier_.size(), identifier_to_object_.size());
    return true;
  }

  void ObjectDestroyed(T* object) {
    IdentifierType identifier = object_to_identifier_.Take(object);
    if (!WTF::IsHashTraitsEmptyValue<HashTraits<IdentifierType>>(identifier))
      identifier_to_object_.erase(identifier);
    DCHECK_EQ(object_to_identifier_.size(), identifier_to_object_.size());
  }

  using ObjectToIdentifier = HeapHashMap<WeakMember<T>, IdentifierType>;
  using IdentifierToObject = HeapHashMap<IdentifierType, WeakMember<T>>;

  ObjectToIdentifier object_to_identifier_;
  IdentifierToObject identifier_to_object_;
};

#define DECLARE_WEAK_IDENTIFIER_MAP(T, ...)        \
  template <>                                      \
  WeakIdentifierMap<T, ##__VA_ARGS__>&             \
  WeakIdentifierMap<T, ##__VA_ARGS__>::Instance(); \
  extern template class WeakIdentifierMap<T, ##__VA_ARGS__>

#define DEFINE_WEAK_IDENTIFIER_MAP(T, ...)                              \
  template class WeakIdentifierMap<T, ##__VA_ARGS__>;                   \
  template <>                                                           \
  WeakIdentifierMap<T, ##__VA_ARGS__>&                                  \
  WeakIdentifierMap<T, ##__VA_ARGS__>::Instance() {                     \
    using RefType = WeakIdentifierMap<T, ##__VA_ARGS__>;                \
    DEFINE_STATIC_LOCAL(                                                \
        Persistent<RefType>, map_instance,                              \
        (MakeGarbageCollected<WeakIdentifierMap<T, ##__VA_ARGS__>>())); \
    return *map_instance;                                               \
  }

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_WEAK_IDENTIFIER_MAP_H_
