// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_TYPE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_TYPE_SET_H_

#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_sync_iterator_view_transition_type_set.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

class ViewTransition;
class ExceptionState;
class ViewTransitionTypeIterationSource;

class CORE_EXPORT ViewTransitionTypeSet
    : public ScriptWrappable,
      public ValueSyncIterable<ViewTransitionTypeSet> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ViewTransitionTypeSet(ViewTransition*,
                                 const Vector<String>& initial);
  void Trace(Visitor* visitor) const override;
  void add(const String& value, ExceptionState& exception_state);
  uint32_t size() const { return types_.size(); }
  void clearForBinding(ScriptState*, ExceptionState&);
  // This operation is O(size()).
  bool deleteForBinding(ScriptState*, const String& value, ExceptionState&);
  // This operation is O(size()).
  bool hasForBinding(ScriptState*, const String& value, ExceptionState&) const {
    return Contains(value);
  }

  bool Contains(const String& value) const { return types_.Contains(value); }
  String At(wtf_size_t index) const { return types_.at(index); }
  bool IsEmpty() const { return types_.empty(); }
  static bool IsValidType(const String& value);
  void Append(const Vector<String>& types);

 protected:
  void AddInternal(const String&);
  // blink::ValueSyncIterable override:
  IterationSource* CreateIterationSource(
      ScriptState* script_state,
      ExceptionState& exception_state) override;

  void InvalidateStyle();

 private:
  Vector<String> types_;
  WeakMember<ViewTransition> view_transition_;
  HeapHashSet<WeakMember<ViewTransitionTypeIterationSource>> iterators_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_TYPE_SET_H_
