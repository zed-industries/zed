// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_PROMISE_ALL_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_PROMISE_ALL_H_

#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

template <typename IDLType,
          typename ResolverType =
              std::conditional_t<std::is_same_v<IDLUndefined, IDLType>,
                                 IDLUndefined,
                                 IDLSequence<IDLType>>>
class CORE_EXPORT PromiseAll final
    : public GarbageCollected<PromiseAll<IDLType>> {
 public:
  // PromiseAll behaves similarly to JS Promise.all() - resolving
  // when all of the given promises have fulfilled, or any rejects.
  static ScriptPromise<ResolverType> Create(
      ScriptState* script_state,
      const HeapVector<MemberScriptPromise<IDLType>>& promises) {
    if (promises.empty()) {
      if constexpr (std::is_same_v<IDLUndefined, IDLType>) {
        return ToResolvedUndefinedPromise(script_state);
      } else {
        return ToResolvedPromise<ResolverType>(script_state, VectorType());
      }
    }
    auto* resolver =
        MakeGarbageCollected<ScriptPromiseResolver<ResolverType>>(script_state);
    MakeGarbageCollected<PromiseAll<IDLType>>(script_state, promises, resolver);
    return resolver->Promise();
  }

  PromiseAll(ScriptState* script_state,
             const HeapVector<MemberScriptPromise<IDLType>>& promises,
             ScriptPromiseResolver<ResolverType>* resolver)
      : number_of_pending_promises_(promises.size()),
        values_(promises.size()),
        resolver_(resolver) {
    for (wtf_size_t i = 0; i < number_of_pending_promises_; ++i) {
      promises[i].Unwrap().Then(script_state,
                                MakeGarbageCollected<Resolve>(i, this),
                                MakeGarbageCollected<Reject>(this));
    }
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(resolver_);
    visitor->Trace(values_);
  }

 private:
  using BlinkType =
      std::conditional_t<WTF::IsGarbageCollectedType<IDLType>::value,
                         std::add_pointer_t<IDLType>,
                         typename IDLTypeToBlinkImplType<IDLType>::type>;
  using VectorType = HeapVector<
      AddMemberIfNeeded<typename IDLTypeToBlinkImplType<IDLType>::type>>;

  class Resolve final : public ThenCallable<IDLType, Resolve> {
   public:
    Resolve(wtf_size_t index, PromiseAll* all) : index_(index), all_(all) {}

    void Trace(Visitor* visitor) const override {
      visitor->Trace(all_);
      ThenCallable<IDLType, Resolve>::Trace(visitor);
    }

    template <typename T = IDLType>
      requires(std::is_same_v<T, IDLUndefined>)
    void React(ScriptState*) {
      all_->OnFulfilled(index_);
    }

    template <typename T = IDLType>
      requires(!std::is_same_v<T, IDLUndefined>)
    void React(ScriptState*, BlinkType value) {
      all_->OnFulfilled(index_, value);
    }

   private:
    const wtf_size_t index_;
    Member<PromiseAll> all_;
  };

  class Reject final : public ThenCallable<IDLAny, Reject> {
   public:
    explicit Reject(PromiseAll* all) : all_(all) {}

    void Trace(Visitor* visitor) const override {
      visitor->Trace(all_);
      ThenCallable<IDLAny, Reject>::Trace(visitor);
    }

    void React(ScriptState*, ScriptValue exception) {
      all_->OnRejected(exception);
    }

   private:
    Member<PromiseAll> all_;
  };

  template <typename T = IDLType>
    requires(std::is_same_v<T, IDLUndefined>)
  void OnFulfilled(wtf_size_t index) {
    if (is_settled_) {
      return;
    }
    if (--number_of_pending_promises_ > 0) {
      return;
    }
    is_settled_ = true;
    resolver_->Resolve();
  }

  template <typename T = IDLType>
    requires(!std::is_same_v<T, IDLUndefined>)
  void OnFulfilled(wtf_size_t index, BlinkType value) {
    if (is_settled_) {
      return;
    }
    DCHECK_LT(index, values_.size());
    values_[index] = value;
    if (--number_of_pending_promises_ > 0) {
      return;
    }
    is_settled_ = true;
    resolver_->Resolve(values_);
    values_.clear();
  }

  void OnRejected(const ScriptValue& value) {
    if (is_settled_) {
      return;
    }
    is_settled_ = true;
    resolver_->Reject(value);
  }

  wtf_size_t number_of_pending_promises_;
  bool is_settled_ = false;
  VectorType values_;
  Member<ScriptPromiseResolver<ResolverType>> resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_PROMISE_ALL_H_
