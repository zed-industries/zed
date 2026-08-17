// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SELECTOR_DIRECTIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SELECTOR_DIRECTIVE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/frame/directive.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {
class Range;
class RangeInFlatTree;
class ScriptState;

// Provides the JavaScript-exposed SelectorDirective base class. Selector
// directives are those that select a specific part of the page to scroll to.
// This is the base interface for all selector directive types and provides
// functionality to allow authors to extract the Node Range that the selector
// is scrolling to.
// See: https://github.com/WICG/scroll-to-text-fragment/issues/160
// TODO(bokan): Update link once we have better public documentation.
class SelectorDirective : public Directive {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SelectorDirective(Type);
  ~SelectorDirective() override;

  // Called by Blink-internal code once the selector has finished running. This
  // will resolve the promise with the located Range or nullptr if one wasn't
  // found.
  void DidFinishMatching(const RangeInFlatTree*);
  void Trace(Visitor*) const override;

  // Web-exposed SelectorDirective interface.
  ScriptPromise<Range> getMatchingRange(ScriptState*, ExceptionState&) const;

 private:
  void ResolvePromise() const;

  // Mutable since it's only used to resolve the promise returned from
  // getMatchingRange and not part of this object's state.
  mutable Member<ScriptPromiseResolver<Range>> matching_range_resolver_;

  // We'll cache the resulting range so that future calls to getMatchingRange
  // resolve immediately.
  Member<RangeInFlatTree> selected_range_;

  bool matching_finished_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SELECTOR_DIRECTIVE_H_
