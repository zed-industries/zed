// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_GLOBAL_VALUE_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_GLOBAL_VALUE_MAP_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "v8/include/v8-util.h"
#include "v8/include/v8.h"

namespace blink {

/**
 * A Traits class for v8::GlobalValueMap that uses wtf/HashMap as a
 * backing store.
 *
 * The parameter is_weak will determine whether the references are 'weak'.
 * If so, entries will be removed from the map as the weak references are
 * collected.
 */
template <typename KeyType,
          typename ValueType,
          v8::PersistentContainerCallbackType type>
class V8GlobalValueMapTraits {
  STATIC_ONLY(V8GlobalValueMapTraits);

 public:
  // Map traits:
  typedef HashMap<KeyType, v8::PersistentContainerValue> Impl;
  typedef typename Impl::iterator Iterator;
  static size_t Size(const Impl* impl) { return impl->size(); }
  static bool Empty(Impl* impl) { return impl->empty(); }
  static void Swap(Impl& impl, Impl& other) { impl.swap(other); }
  static Iterator Begin(Impl* impl) { return impl->begin(); }
  static Iterator End(Impl* impl) { return impl->end(); }
  static v8::PersistentContainerValue Value(Iterator& iter) {
    return iter->value;
  }
  static KeyType Key(Iterator& iter) { return iter->key; }
  static v8::PersistentContainerValue Set(Impl* impl,
                                          KeyType key,
                                          v8::PersistentContainerValue value) {
    v8::PersistentContainerValue old_value = Get(impl, key);
    impl->Set(key, value);
    return old_value;
  }
  static v8::PersistentContainerValue Get(const Impl* impl, KeyType key) {
    auto it = impl->find(key);
    return it != impl->end() ? it->value : 0;
  }

  static v8::PersistentContainerValue Remove(Impl* impl, KeyType key) {
    return impl->Take(key);
  }

  // Weak traits:
  static const v8::PersistentContainerCallbackType kCallbackType = type;
  typedef v8::GlobalValueMap<KeyType,
                             ValueType,
                             V8GlobalValueMapTraits<KeyType, ValueType, type>>
      MapType;

  typedef void WeakCallbackDataType;

  static WeakCallbackDataType* WeakCallbackParameter(
      MapType* map,
      KeyType key,
      const v8::Local<ValueType>& value) {
    return nullptr;
  }

  static void DisposeCallbackData(WeakCallbackDataType* callback_data) {}

  static MapType* MapFromWeakCallbackInfo(
      const v8::WeakCallbackInfo<WeakCallbackDataType>& data) {
    return nullptr;
  }

  static KeyType KeyFromWeakCallbackInfo(
      const v8::WeakCallbackInfo<WeakCallbackDataType>& data) {
    return KeyType();
  }

  static void OnWeakCallback(
      const v8::WeakCallbackInfo<WeakCallbackDataType>& data) {}

  // Dispose traits:
  static void Dispose(v8::Isolate* isolate,
                      v8::Global<ValueType> value,
                      KeyType key) {}
  static void DisposeWeak(
      const v8::WeakCallbackInfo<WeakCallbackDataType>& data) {}
};

/**
 * A map for safely storing persistent V8 values, based on
 * v8::GlobalValueMap.
 */
template <typename KeyType, typename ValueType>
class V8GlobalValueMap
    : public v8::GlobalValueMap<
          KeyType,
          ValueType,
          V8GlobalValueMapTraits<KeyType, ValueType, v8::kNotWeak>> {
  DISALLOW_NEW();

 public:
  typedef V8GlobalValueMapTraits<KeyType, ValueType, v8::kNotWeak> Traits;
  explicit V8GlobalValueMap(v8::Isolate* isolate)
      : v8::GlobalValueMap<KeyType, ValueType, Traits>(isolate) {}
  V8GlobalValueMap(v8::Isolate* isolate, const char* label)
      : v8::GlobalValueMap<KeyType, ValueType, Traits>(isolate, label) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_GLOBAL_VALUE_MAP_H_
