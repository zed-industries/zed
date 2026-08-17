// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_SCOPE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/layout/custom/custom_layout_work_task.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// The work queue is a list of work tasks which will either produce fragment(s)
// or intrinsic-size(s) for the custom-layout class.
typedef HeapVector<Member<CustomLayoutWorkTask>, 4> CustomLayoutWorkQueue;

// This heap allocated class is used to indicate which custom-layout (heap)
// objects are still valid.
//
// Any objects which have a pointer to this object with |is_detached_| being
// true, or not matching the current scope token should be considered invalid.
class CustomLayoutToken : public GarbageCollected<CustomLayoutToken> {
 public:
  CustomLayoutToken() : is_detached_(false) {}
  void Trace(Visitor* visitor) const {}
  bool IsValid() const;

 private:
  friend class CustomLayoutScope;
  bool is_detached_;
};

// This scope object is used to track which custom-layout (heap) objects are
// valid.
//
// Also maintains the current work queue. Work should only be pushed onto this
// queue if this is the current scope.
class CustomLayoutScope {
  STACK_ALLOCATED();

 public:
  static CustomLayoutScope* Current() { return current_scope_; }

  CustomLayoutScope()
      : prev_scope_(current_scope_),
        token_(MakeGarbageCollected<CustomLayoutToken>()) {
    current_scope_ = this;
  }

  // Marks the scope-token as "detached" for any classes with a pointer to it.
  ~CustomLayoutScope() {
    token_->is_detached_ = true;
    current_scope_ = current_scope_->prev_scope_;
  }

  CustomLayoutWorkQueue* Queue() {
    DCHECK_EQ(this, current_scope_);
    return &queue_;
  }
  CustomLayoutToken* Token() { return token_; }

 private:
  static CustomLayoutScope* current_scope_;

  CustomLayoutScope* prev_scope_;
  CustomLayoutWorkQueue queue_;
  CustomLayoutToken* token_;
};

inline bool CustomLayoutToken::IsValid() const {
  return !is_detached_ && this == CustomLayoutScope::Current()->Token();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_SCOPE_H_
