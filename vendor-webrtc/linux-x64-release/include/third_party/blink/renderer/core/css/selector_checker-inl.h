// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_CHECKER_INL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_CHECKER_INL_H_

#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/selector_checker.h"
#include "third_party/blink/renderer/core/html/html_document.h"

namespace blink {

bool EasySelectorChecker::IsEasy(const CSSSelector* selector) {
  bool has_descendant_selector = false;
  for (; selector != nullptr; selector = selector->NextSimpleSelector()) {
    if (!selector->IsLastInComplexSelector() &&
        selector->Relation() != CSSSelector::kSubSelector &&
        selector->Relation() != CSSSelector::kDescendant &&
        selector->Relation() != CSSSelector::kChild) {
      // We don't support anything that requires us to recurse.
      return false;
    }
    if (selector->Relation() == CSSSelector::kDescendant) {
      has_descendant_selector = true;
    } else if (selector->Relation() == CSSSelector::kChild &&
               has_descendant_selector) {
      // Having a child selector after a descendant selector requires
      // more complicated backtracking (we need to backtrack in both
      // selector and element), which we don't support yet.
      return false;
    }
    if (selector->IsCoveredByBucketing()) {
      // No matter what this selector is, we won't need to check it,
      // so it's fine.
      continue;
    }
    switch (selector->Match()) {
      case CSSSelector::kTag:
      case CSSSelector::kUniversalTag:
      case CSSSelector::kId:
      case CSSSelector::kClass:
        break;
      case CSSSelector::kAttributeExact:
      case CSSSelector::kAttributeSet:
        if (selector->Attribute().Prefix() == g_star_atom) {
          // We don't support attribute matches with wildcard namespaces
          // (e.g. [*|attr]), since those prevent short-circuiting in
          // Match() once we've found the attribute; there might be more
          // than one, so we would have to keep looking, and we don't
          // want to support that.
          return false;
        }
        break;
      default:
        // Unsupported selector.
        return false;
    }
  }
  return true;
}

bool EasySelectorChecker::Match(const CSSSelector* selector,
                                const Element* element) {
  DCHECK(IsEasy(selector));

  // Since we only support subselector, child and descendant combinators
  // (and not all combinations of the latter two), we can do with a
  // nonrecursive algorithm. The idea is fairly simple: We can match
  // greedily and never need to backtrack. E.g. if we have .a.b .c.d .e.f {}
  // and see an element matching .e.f and then later some parent matching .c.d,
  // we never need to look for .c.d again.
  //
  // Apart from that, it's a simple matter of just matching the simple selectors
  // against the current element, one by one. If we have a mismatch
  // in the subject (.e.f in the example above), the match fails immediately.
  // If we have a mismatch when looking for a parent (either .a.b or .c.d
  // in the example above), we rewind to the start of the compound and move on
  // to the parent element. (rewind_on_failure then points to the start of the
  // compound; it's nullptr if we're matching the subject.) Child combinators
  // are implemented by simply skipping to the parent element and keeping
  // matching.
  //
  // If all subselectors in a compound have matched, we move on to the next
  // compound (setting rewind_on_failure to the start of it) and go to the
  // parent element to check the next descendant.
  const CSSSelector* rewind_on_failure = nullptr;

  while (selector != nullptr) {
    if (selector->IsCoveredByBucketing() || MatchOne(selector, element)) {
      if (selector->Relation() == CSSSelector::kDescendant) {
        // We matched the entire compound, but there are more.
        // Move to the next one.
        DCHECK(!selector->IsLastInComplexSelector());
        rewind_on_failure = selector->NextSimpleSelector();

        element = element->parentElement();
        if (element == nullptr) {
          return false;
        }
      } else if (selector->Relation() == CSSSelector::kChild) {
        // Similar, but we need this to match the exact parent,
        // so on failure, we should not rewind but fail the match
        // (do not set rewind_on_failure).
        DCHECK(!selector->IsLastInComplexSelector());
        DCHECK(!rewind_on_failure);

        element = element->parentElement();
        if (element == nullptr) {
          return false;
        }
      }
      selector = selector->NextSimpleSelector();
    } else if (rewind_on_failure) {
      // We failed to match this compound, but we are looking for descendants,
      // so rewind to start of the compound and try the parent element.
      selector = rewind_on_failure;

      element = element->parentElement();
      if (element == nullptr) {
        return false;
      }
    } else {
      // We failed to match this compound, and we're in the subject,
      // so fail immediately.
      return false;
    }
  }

  return true;
}

bool EasySelectorChecker::MatchOne(const CSSSelector* selector,
                                   const Element* element) {
  switch (selector->Match()) {
    case CSSSelector::kTag: {
      const QualifiedName& tag_q_name = selector->TagQName();
      if (element->namespaceURI() != tag_q_name.NamespaceURI() &&
          tag_q_name.NamespaceURI() != g_star_atom) {
        // Namespace mismatch.
        return false;
      }
      if (element->localName() == tag_q_name.LocalName()) {
        return true;
      }
      if (!element->IsHTMLElement() &&
          IsA<HTMLDocument>(element->GetDocument())) {
        // If we have a non-HTML element in a HTML document, we need to
        // also check case-insensitively (see MatchesTagName()). Ideally,
        // we'd like to not have to handle this case in easy selector matching,
        // but it turns out to be hard to reliably check that a tag in a
        // descendant selector doesn't hit this issue (the subject element
        // could be checked once, outside EasySelectorChecker).
        return element->TagQName().LocalNameUpper() ==
               tag_q_name.LocalNameUpper();
      } else {
        return false;
      }
    }
    case CSSSelector::kUniversalTag: {
      const QualifiedName& tag_q_name = selector->TagQName();
      return element->namespaceURI() == tag_q_name.NamespaceURI() ||
             tag_q_name.NamespaceURI() == g_star_atom;
    }
    case CSSSelector::kClass:
      return element->HasClass() &&
             element->ClassNames().Contains(selector->Value());
    case CSSSelector::kId:
      return element->HasID() &&
             element->IdForStyleResolution() == selector->Value();
    case CSSSelector::kAttributeSet:
      return AttributeIsSet(*element, selector->Attribute());
    case CSSSelector::kAttributeExact: {
      bool case_insensitive =
          selector->AttributeMatch() ==
              CSSSelector::AttributeMatchType::kCaseInsensitive ||
          (selector->LegacyCaseInsensitiveMatch() &&
           IsA<HTMLDocument>(element->GetDocument()));
      return AttributeMatches(*element, selector->Attribute(),
                              selector->Value(), case_insensitive);
    }
    default:
      NOTREACHED();
  }
}

bool EasySelectorChecker::AttributeIsSet(const Element& element,
                                         const QualifiedName& attr) {
  element.SynchronizeAttribute(attr.LocalName());
  AttributeCollection attributes = element.AttributesWithoutUpdate();
  for (const auto& attribute_item : attributes) {
    if (AttributeItemHasName(attribute_item, element, attr)) {
      return true;
    }
  }
  return false;
}

bool EasySelectorChecker::AttributeMatches(const Element& element,
                                           const QualifiedName& attr,
                                           const AtomicString& value,
                                           bool case_insensitive) {
  element.SynchronizeAttribute(attr.LocalName());

#if !DCHECK_IS_ON()
  // In non-debug builds, we test the Bloom filter here and exit early
  // if the attribute could not exist on the element. For non-debug builds,
  // we go through the entire normal operation but verify that the Bloom
  // filter would not erroneously reject a match.
  if (!element.CouldHaveAttribute(attr)) {
    return false;
  }
#endif

  AttributeCollection attributes = element.AttributesWithoutUpdate();
  for (const auto& attribute_item : attributes) {
    if (AttributeItemHasName(attribute_item, element, attr)) {
#if DCHECK_IS_ON()
      // NOTE: Even if the value doesn't match, we want to check that the
      // attribute name was properly found.
      DCHECK(element.CouldHaveAttribute(attr))
          << element << " should have contained attribute " << attr
          << ", Bloom bits on element are "
          << element.AttributeBloomFilterForDebug();
#endif
      return attribute_item.Value() == value ||
             (case_insensitive &&
              EqualIgnoringASCIICase(attribute_item.Value(), value));
    }
  }
  return false;
}

bool EasySelectorChecker::AttributeItemHasName(const Attribute& attribute_item,
                                               const Element& element,
                                               const QualifiedName& name) {
  // See MatchesTagName() and the comment in MatchOne() for information
  // on the extra check on IsHTMLElement() etc..
  return attribute_item.Matches(name) ||
         (!element.IsHTMLElement() &&
          IsA<HTMLDocument>(element.GetDocument()) &&
          attribute_item.MatchesCaseInsensitive(name));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_CHECKER_INL_H_
