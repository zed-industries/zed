// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_OBSERVABLE_ARRAY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_OBSERVABLE_ARRAY_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/heap_traits.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "v8/include/v8-forward.h"

// Overview of Blink implementation of Web IDL observable arrays
//
// Observable arrays are implemented with two objects:
// 1) backing list object
// https://webidl.spec.whatwg.org/#observable-array-attribute-backing-list
// and 2) exotic object.
// https://webidl.spec.whatwg.org/#backing-observable-array-exotic-object
// As DOM objects are implemented with a ScriptWrappable and its V8 wrappers,
// there are a ScriptWrappable of a backing list object and its V8 wrappers,
// also a ScriptWrappable of a exotic object and its (pseudo) V8 wrappers.
//
// By definition, observable array exotic object is a JS Proxy in ECMAScript
// binding.
//
//   let exotic_object = new Proxy(backing_list_object, handler_object);
//
// For web developers, the JS Proxy object is the only visible object.  Web
// developers cannot access the backing list object directly.
//
// For Blink developers, the backing list object looks the primary object.
// However, when exposing an observable array to web developers, the exotic
// object must be exposed instead of the backing list object.  This is done in
// auto-generated bindings (generated bindings automatically call
// GetExoticObject()).
//
//   class MyIdlInterface : public ScriptWrappable {
//    public:
//     V8ObservableArrayNode* myAttr() const {
//       return my_observable_array_;
//     }
//    private:
//     // my_observable_array_ is a backing list object.
//     Member<V8ObservableArrayNode> my_observable_array_;
//   };
//
// Class hierarchy and relationship:
//   bindings::ObservableArrayBase -- the base class of backing list objects
//   +-- bindings::ObservableArrayImplHelper<T> -- just a helper
//       +-- V8ObservableArrayNode -- generated implementation of IDL
//               ObservableArray<Node>.  Bindings code generator produces this
//               class from *.idl files.
//   ObservableArrayExoticObject -- the implementation of exotic objects
//
//   v8_exotic_object (= JS Proxy)
//       --(proxy target)--> v8_array (= JS Array)
//       --(private property)--> v8_backing_list_object
//       --(internal field)--> blink_backing_list_object
//       --(data member)--> blink_exotic_object
//       --(ToV8Traits)--> v8_exotic_object

namespace blink {

class ObservableArrayExoticObject;

namespace bindings {

// ObservableArrayBase is the common base class of all the observable array
// classes, and represents the backing list for an IDL attribute of an
// observable array type (but the actual implementation lives in
// bindings/core/v8/).
// https://webidl.spec.whatwg.org/#observable-array-attribute-backing-list
class PLATFORM_EXPORT ObservableArrayBase : public ScriptWrappable {
 public:
  ObservableArrayBase(
      GarbageCollectedMixin* platform_object,
      ObservableArrayExoticObject* observable_array_exotic_object);
  ~ObservableArrayBase() override = default;

  // Returns the observable array exotic object, which is the value to be
  // returned as the IDL attribute value.  Do not use `this` object (= the
  // observable array backing list object) as the IDL attribute value.
  ObservableArrayExoticObject* GetExoticObject() const {
    return observable_array_exotic_object_.Get();
  }

  v8::Local<v8::Object> GetProxyHandlerObject(ScriptState* script_state);

  void Trace(Visitor* visitor) const override;

 protected:
  GarbageCollectedMixin* GetPlatformObject() { return platform_object_.Get(); }

  virtual v8::Local<v8::FunctionTemplate> GetProxyHandlerFunctionTemplate(
      ScriptState* script_state) = 0;

 private:
  Member<GarbageCollectedMixin> platform_object_;  // IDL attribute owner
  Member<ObservableArrayExoticObject> observable_array_exotic_object_;
};

template <typename ElementType>
class ObservableArrayImplHelper : public bindings::ObservableArrayBase {
 public:
  using BackingListType = VectorOf<ElementType>;
  using size_type = typename BackingListType::size_type;
  using value_type = typename BackingListType::value_type;
  using reference = typename BackingListType::reference;
  using const_reference = typename BackingListType::const_reference;
  using pointer = typename BackingListType::pointer;
  using const_pointer = typename BackingListType::const_pointer;
  using iterator = typename BackingListType::iterator;
  using const_iterator = typename BackingListType::const_iterator;
  using reverse_iterator = typename BackingListType::reverse_iterator;
  using const_reverse_iterator =
      typename BackingListType::const_reverse_iterator;

  explicit ObservableArrayImplHelper(GarbageCollectedMixin* platform_object)
      : bindings::ObservableArrayBase(
            platform_object,
            MakeGarbageCollected<ObservableArrayExoticObject>(this)) {}
  ~ObservableArrayImplHelper() override = default;

  // Returns the observable array exotic object, which is the value to be
  // returned as the IDL attribute value.  Do not use `this` object (= the
  // observable array backing list object) as the IDL attribute value.
  using bindings::ObservableArrayBase::GetExoticObject;

  // Vector-compatible APIs (accessors)
  size_type size() const { return backing_list_.size(); }
  size_type capacity() const { return backing_list_.capacity(); }
  bool empty() const { return backing_list_.empty(); }
  void reserve(size_type new_capacity) { backing_list_.reserve(new_capacity); }
  void ReserveInitialCapacity(size_type initial_capacity) {
    backing_list_.ReserveInitialCapacity(initial_capacity);
  }
  reference at(size_type index) { return backing_list_.at(index); }
  const_reference at(size_type index) const { return backing_list_.at(index); }
  reference operator[](size_type index) { return backing_list_[index]; }
  const_reference operator[](size_type index) const {
    return backing_list_[index];
  }
  value_type* data() { return backing_list_.data(); }
  const value_type* data() const { return backing_list_.data(); }
  iterator begin() { return backing_list_.begin(); }
  iterator end() { return backing_list_.end(); }
  const_iterator begin() const { return backing_list_.begin(); }
  const_iterator end() const { return backing_list_.end(); }
  reverse_iterator rbegin() { return backing_list_.rbegin(); }
  reverse_iterator rend() { return backing_list_.rend(); }
  const_reverse_iterator rbegin() const { return backing_list_.rbegin(); }
  const_reverse_iterator rend() const { return backing_list_.rend(); }
  reference front() { return backing_list_.front(); }
  reference back() { return backing_list_.back(); }
  const_reference front() const { return backing_list_.front(); }
  const_reference back() const { return backing_list_.back(); }
  // Vector-compatible APIs (modifiers)
  void resize(size_type size) { backing_list_.resize(size); }
  void clear() { backing_list_.clear(); }
  template <typename T>
  void push_back(T&& value) {
    backing_list_.push_back(std::forward<T>(value));
  }
  void pop_back() { backing_list_.pop_back(); }
  template <typename... Args>
  reference emplace_back(Args&&... args) {
    return backing_list_.emplace_back(std::forward<Args>(args)...);
  }
  template <typename T>
  void insert(iterator position, T&& value) {
    backing_list_.InsertAt(position, std::forward<T>(value));
  }
  iterator erase(iterator position) { return backing_list_.erase(position); }
  iterator erase(iterator first, iterator last) {
    return backing_list_.erase(first, last);
  }

  void Trace(Visitor* visitor) const override {
    ObservableArrayBase::Trace(visitor);
    TraceIfNeeded<BackingListType>::Trace(visitor, backing_list_);
  }

 private:
  BackingListType backing_list_;
};

}  // namespace bindings

// Represents a backing observable array exotic object.
// https://webidl.spec.whatwg.org/#backing-observable-array-exotic-object
class PLATFORM_EXPORT ObservableArrayExoticObject final
    : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Returns the backing list object extracted from the proxy target object
  // of type JS Array.
  template <typename T>
  static T* ProxyTargetToObservableArray(v8::Isolate* isolate,
                                         v8::Local<v8::Array> v8_proxy_target) {
    return ToScriptWrappable<T>(
        isolate, GetBackingObjectFromProxyTarget(isolate, v8_proxy_target));
  }

  explicit ObservableArrayExoticObject(
      bindings::ObservableArrayBase* observable_array_backing_list_object);
  ~ObservableArrayExoticObject() final = default;

  bindings::ObservableArrayBase* GetBackingListObject() const {
    return observable_array_backing_list_object_.Get();
  }

  // ScriptWrappable overrides:
  v8::Local<v8::Value> Wrap(ScriptState* script_state) final;
  [[nodiscard]] v8::Local<v8::Object> AssociateWithWrapper(
      v8::Isolate* isolate,
      const WrapperTypeInfo* wrapper_type_info,
      v8::Local<v8::Object> wrapper) final;

  void Trace(Visitor* visitor) const final;

 private:
  static v8::Local<v8::Object> GetBackingObjectFromProxyTarget(
      v8::Isolate*,
      v8::Local<v8::Array> v8_proxy_target);

  Member<bindings::ObservableArrayBase> observable_array_backing_list_object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_OBSERVABLE_ARRAY_H_
