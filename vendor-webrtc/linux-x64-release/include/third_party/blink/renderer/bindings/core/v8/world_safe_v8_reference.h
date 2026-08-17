// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WORLD_SAFE_V8_REFERENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WORLD_SAFE_V8_REFERENCE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "v8/include/v8.h"

namespace blink {

class ScriptState;

// This is a namespace to provide utility functions to WorldSafeV8Reference.
class CORE_EXPORT WorldSafeV8ReferenceInternal final {
  STATIC_ONLY(WorldSafeV8ReferenceInternal);

 private:
  // Returns a V8 reference that is safe to access in |target_script_state|.
  // The return value may be a cloned object.
  //
  // TODO(crbug.com/803478): ScriptValue::V8ValueFor should be replaced with
  // this function, or these two functions should be merged at least.
  static v8::Local<v8::Value> ToWorldSafeValue(
      ScriptState* target_script_state,
      const TraceWrapperV8Reference<v8::Value>& v8_reference,
      const DOMWrapperWorld& v8_reference_world);

  // Checks the world of |value|'s creation context if |value| is a v8::Object.
  // The given |world| and |value|'s world must match.  Otherwise, crashes.
  //
  // TODO(yukishiino): Find the best place to put this function.  We might want
  // to share this function among other clients, e.g. access to wrapper objects
  // across worlds.
  static void MaybeCheckCreationContext(
      v8::Isolate*,
      const v8::Local<v8::Context> current_context,
      const DOMWrapperWorld& current_world,
      const v8::Local<v8::Value> value);

  template <typename V8Type>
  friend class WorldSafeV8Reference;
};

// This class provides safe access to v8::Value across worlds.  This class
// provides accessors that check whether the value is accessed in the same world
// or not, also provides an accessor that clones the value when accessed across
// worlds.
//
// TODO(crbug.com/1008765): Allow WorldSafeV8Reference created/set not in
// context.
template <typename V8Type>
class WorldSafeV8Reference final {
  DISALLOW_NEW();

 public:
  WorldSafeV8Reference() = default;

  WorldSafeV8Reference(v8::Isolate* isolate, v8::Local<V8Type> value)
      : v8_reference_(isolate, value) {
    if (value.IsEmpty()) {
      return;
    }

    DCHECK(isolate);
    // Basically, |world_| is a world when this V8 reference is created.
    // However, when this V8 reference isn't created in context and value is
    // object, we set |world_| to a value's creation context's world.
    v8::Local<v8::Context> current_context = isolate->GetCurrentContext();
    if (!current_context.IsEmpty()) [[likely]] {
      const auto& current_world =
          DOMWrapperWorld::World(isolate, current_context);
      world_ = &current_world;
      WorldSafeV8ReferenceInternal::MaybeCheckCreationContext(
          isolate, current_context, current_world, value);
    } else if (value->IsObject()) {
      ScriptState* script_state = ScriptState::ForRelevantRealm(
          isolate, value.template As<v8::Object>());
      world_ = &script_state->World();
    }
  }

  ~WorldSafeV8Reference() = default;

  // Returns the V8 reference.  Crashes if |world_| is set and it is
  // different from |target_script_state|'s world.
  v8::Local<V8Type> Get(ScriptState* target_script_state) const {
    DCHECK(!v8_reference_.IsEmpty());
    if (world_) {
      CHECK_EQ(world_.Get(), &target_script_state->World());
    }
    return v8_reference_.Get(target_script_state->GetIsolate());
  }

  // Returns a V8 reference that is safe to access in |target_script_state|.
  // The return value may be a cloned object.
  v8::Local<V8Type> GetAcrossWorld(ScriptState* target_script_state) const {
    CHECK(world_);
    return WorldSafeV8ReferenceInternal::ToWorldSafeValue(
               target_script_state, v8_reference_, *world_.Get())
        .template As<V8Type>();
  }

  // Sets a new V8 reference.  Crashes if |world_| is set and it is
  // different from |new_value|'s world.
  void Set(v8::Isolate* isolate, v8::Local<V8Type> new_value) {
    DCHECK(!new_value.IsEmpty());
    CHECK(isolate->InContext());
    v8::Local<v8::Context> current_context = isolate->GetCurrentContext();
    const auto& new_world = DOMWrapperWorld::World(isolate, current_context);
    WorldSafeV8ReferenceInternal::MaybeCheckCreationContext(
        isolate, current_context, new_world, new_value);
    CHECK(v8_reference_.IsEmpty() || world_.Get() == &new_world);
    v8_reference_.Reset(isolate, new_value);
    world_ = new_world;
  }

  // Forcibly sets a new V8 reference even when the worlds are different.  The
  // world of this V8 reference will be |new_value|'s world.
  void SetAcrossWorld(v8::Isolate* isolate, v8::Local<V8Type> new_value) {
    DCHECK(!new_value.IsEmpty());
    CHECK(isolate->InContext());
    const DOMWrapperWorld& new_world = DOMWrapperWorld::Current(isolate);
    v8_reference_.Reset(isolate, new_value);
    world_ = &new_world;
  }

  void Reset() {
    v8_reference_.Reset();
    world_.Clear();
  }

  bool IsEmpty() const { return v8_reference_.IsEmpty(); }

  v8::Isolate* GetIsolate() const {
    DCHECK(!IsEmpty());
    return world_->GetIsolate();
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(v8_reference_);
    visitor->Trace(world_);
  }

  WorldSafeV8Reference& operator=(const WorldSafeV8Reference<V8Type>& other) =
      default;

  bool operator==(const WorldSafeV8Reference<V8Type>& other) const {
    return v8_reference_ == other.v8_reference_;
  }

 private:
  TraceWrapperV8Reference<V8Type> v8_reference_;
  // The world of the current context at the time when |v8_reference_| was set.
  // It's guaranteed that, if |v8_reference_| is a v8::Object, the world of the
  // creation context of |v8_reference_| is the same as |world_|.
  Member<const DOMWrapperWorld> world_;
};

}  // namespace blink

namespace WTF {

template <typename V8Type>
struct VectorTraits<blink::WorldSafeV8Reference<V8Type>>
    : VectorTraitsBase<blink::WorldSafeV8Reference<V8Type>> {
  STATIC_ONLY(VectorTraits);

  static constexpr bool kCanInitializeWithMemset =
      VectorTraits<
          blink::TraceWrapperV8Reference<V8Type>>::kCanInitializeWithMemset &&
      VectorTraits<scoped_refptr<const blink::DOMWrapperWorld>>::
          kCanInitializeWithMemset;
  static constexpr bool kCanClearUnusedSlotsWithMemset =
      VectorTraits<blink::TraceWrapperV8Reference<V8Type>>::
          kCanClearUnusedSlotsWithMemset &&
      VectorTraits<scoped_refptr<const blink::DOMWrapperWorld>>::
          kCanClearUnusedSlotsWithMemset;
  static constexpr bool kCanCopyWithMemcpy =
      VectorTraits<
          blink::TraceWrapperV8Reference<V8Type>>::kCanCopyWithMemcpy &&
      VectorTraits<
          scoped_refptr<const blink::DOMWrapperWorld>>::kCanCopyWithMemcpy;
  static constexpr bool kCanMoveWithMemcpy =
      VectorTraits<
          blink::TraceWrapperV8Reference<V8Type>>::kCanMoveWithMemcpy &&
      VectorTraits<
          scoped_refptr<const blink::DOMWrapperWorld>>::kCanMoveWithMemcpy;

  static constexpr bool kCanTraceConcurrently = VectorTraits<
      blink::TraceWrapperV8Reference<V8Type>>::kCanTraceConcurrently;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WORLD_SAFE_V8_REFERENCE_H_
