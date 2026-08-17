// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SYNC_ITERATOR_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SYNC_ITERATOR_BASE_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class ExceptionState;

namespace bindings {

// SyncIteratorBase is the common base class of all sync iterator classes.
// Most importantly this class provides a way of type dispatching (e.g.
// overload resolutions, SFINAE technique, etc.) so that it's possible to
// distinguish sync iterators from anything else. Also it provides common
// implementation of sync iterators.
class PLATFORM_EXPORT SyncIteratorBase : public ScriptWrappable {
 public:
  // https://webidl.spec.whatwg.org/#default-iterator-object-kind
  enum class Kind {
    kKey,
    kValue,
    kKeyValue,
  };

  class PLATFORM_EXPORT IterationSourceBase
      : public GarbageCollected<IterationSourceBase> {
   public:
    IterationSourceBase() = default;
    virtual ~IterationSourceBase() = default;
    IterationSourceBase(const IterationSourceBase&) = delete;
    IterationSourceBase& operator=(const IterationSourceBase&) = delete;

    virtual v8::Local<v8::Object> Next(ScriptState* script_state,
                                       Kind kind,
                                       ExceptionState& exception_state) = 0;

    virtual void Trace(Visitor* visitor) const {}
  };

  ~SyncIteratorBase() override = default;

  v8::Local<v8::Object> next(ScriptState* script_state,
                             ExceptionState& exception_state);

  void Trace(Visitor* visitor) const override;

 protected:
  explicit SyncIteratorBase(IterationSourceBase* iteration_source, Kind kind)
      : iteration_source_(iteration_source), kind_(kind) {}

  const Member<IterationSourceBase> iteration_source_;
  const Kind kind_;
};

}  // namespace bindings

// This class template is specialized by the bindings code generator for each
// class T which implements an IDL interface declared to be iterable, maplike,
// or setlike.
template <typename T>
class SyncIterator;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SYNC_ITERATOR_BASE_H_
