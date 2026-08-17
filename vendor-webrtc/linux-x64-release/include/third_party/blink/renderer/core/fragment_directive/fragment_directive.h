// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_FRAGMENT_DIRECTIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_FRAGMENT_DIRECTIVE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/frame/directive.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class Document;
class ScriptState;
class SelectorDirective;
class V8UnionRangeOrSelection;

// This class implements the `window.fragmentDirective` web API and serves as a
// home for features based on the fragment directive portion of a URL (the part
// of the URL fragment that comes after ':~:'. See:
// https://github.com/WICG/scroll-to-text-fragment/
class FragmentDirective : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit FragmentDirective(Document& owner_document);
  ~FragmentDirective() override;

  // Gets all parsed directives of the template type.
  template <typename DirectiveType>
  HeapVector<Member<DirectiveType>> GetDirectives() {
    HeapVector<Member<DirectiveType>> to_return;
    for (const Member<Directive>& directive : directives_) {
      if (directive->GetType() == DirectiveType::ClassType()) {
        auto* typed_directive = static_cast<DirectiveType*>(directive.Get());
        to_return.push_back(Member<DirectiveType>(typed_directive));
      }
    }

    return to_return;
  }

  // If fragment directive is present in the given URL, we'll parse it into
  // Directive objects and strip it from the returned URL. This is used when a
  // URL is set on Document to implement the hiding of the fragment directive
  // from the page.
  KURL ConsumeFragmentDirective(const KURL& url);
  bool LastNavigationHadFragmentDirective() const {
    return last_navigation_had_fragment_directive_;
  }

  void Trace(Visitor*) const override;

  // Web-exposed FragmentDirective interface.
  const HeapVector<Member<Directive>>& items() const;
  ScriptPromise<SelectorDirective> createSelectorDirective(
      ScriptState*,
      const V8UnionRangeOrSelection*);

 private:
  void ParseDirectives(const String& fragment_directive);

  HeapVector<Member<Directive>> directives_;
  Member<Document> owner_document_;
  wtf_size_t fragment_directive_string_length_ = 0;
  bool last_navigation_had_fragment_directive_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_FRAGMENT_DIRECTIVE_H_
