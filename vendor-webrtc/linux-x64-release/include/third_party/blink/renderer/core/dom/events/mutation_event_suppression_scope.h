// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_MUTATION_EVENT_SUPPRESSION_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_MUTATION_EVENT_SUPPRESSION_SCOPE_H_

#include "third_party/blink/renderer/core/dom/document.h"

namespace blink {

class MutationEventSuppressionScope {
  STACK_ALLOCATED();

 public:
  explicit MutationEventSuppressionScope(Document& document)
      : document_(document) {
    // Save the current suppression value, so we can restore it from our
    // destructor when `this` is destroyed. This is important because it is
    // possible for these scopes to be nested, which means
    // `Document::ShouldSuppressMutationEvents()` could already be true here.
    old_should_suppress_mutation_events_ =
        document.ShouldSuppressMutationEvents();

    document.SetSuppressMutationEvents(true);
  }
  ~MutationEventSuppressionScope() {
    document_.SetSuppressMutationEvents(old_should_suppress_mutation_events_);
  }

 private:
  bool old_should_suppress_mutation_events_;
  Document& document_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_MUTATION_EVENT_SUPPRESSION_SCOPE_H_
