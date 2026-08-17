// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_DIRECTIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_DIRECTIVE_H_

#include "third_party/blink/renderer/core/fragment_directive/text_fragment_selector.h"
#include "third_party/blink/renderer/core/frame/selector_directive.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class TextDirectiveOptions;

// Provides the JavaScript-exposed TextDirective which exposes `text=`
// directives in the fragment.
// See: https://github.com/WICG/scroll-to-text-fragment/issues/160
// TODO(bokan): Update link once we have better public documentation.
class TextDirective : public SelectorDirective {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TextDirective* Create(const String& directive_value);
  static Type ClassType() { return kText; }
  explicit TextDirective(const TextFragmentSelector& selector);
  ~TextDirective() override;

  const TextFragmentSelector& GetSelector() const { return selector_; }

  void Trace(Visitor*) const override;

  // Web-exposed TextDirective interface.
  static TextDirective* Create(TextDirectiveOptions* options);
  const String prefix() const;
  const String textStart() const;
  const String textEnd() const;
  const String suffix() const;

 protected:
  String ToStringImpl() const override;

 private:
  TextFragmentSelector selector_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_DIRECTIVE_H_
