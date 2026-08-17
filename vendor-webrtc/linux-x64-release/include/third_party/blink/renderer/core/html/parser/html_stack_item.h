/*
 * Copyright (C) 2012 Company 100, Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GOOGLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_STACK_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_STACK_ITEM_H_

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/html/parser/atomic_html_token.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/core/mathml_names.h"
#include "third_party/blink/renderer/core/svg_names.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class ContainerNode;

// NOTE: HTMLStackItem stores all of its attributes (if any) just after the end
// of the pointer itself, to reduce on the number of Alloc/Free calls. (This
// also saves a little bit of memory, as a side effect.)
class HTMLStackItem final : public GarbageCollected<HTMLStackItem> {
 public:
  enum ItemType { kItemForContextElement, kItemForDocumentFragmentNode };

  HTMLStackItem(ContainerNode* node, ItemType type)
      : node_(node), token_name_(html_names::HTMLTag::kUnknown) {
    switch (type) {
      case kItemForDocumentFragmentNode:
        is_document_fragment_node_ = true;
        break;
      case kItemForContextElement:
        token_name_ = HTMLTokenName::FromLocalName(GetElement()->localName());
        namespace_uri_ = GetElement()->namespaceURI();
        is_document_fragment_node_ = false;
        break;
    }
  }

  // You cannot call this constructor directly (but it must be public
  // so that MakeGarbageCollected() can); use Create() below instead.
  HTMLStackItem(base::PassKey<HTMLStackItem>,
                ContainerNode* node,
                AtomicHTMLToken* token,
                const AtomicString& namespace_uri)
      : node_(node),
        token_name_(token->GetTokenName()),
        namespace_uri_(namespace_uri),
        num_token_attributes_(token->Attributes().size()),
        is_document_fragment_node_(false) {
    // We rely on Create() allocating extra memory past our end for the
    // attributes.
    for (wtf_size_t i = 0; i < token->Attributes().size(); ++i) {
      new (UNSAFE_TODO(TokenAttributesData() + i))
          Attribute(token->Attributes()[i]);
    }
  }

  ~HTMLStackItem() {
    // We need to clean up the attributes we initialized in the constructor
    // manually, since they are not stored in a regular member.
    if (num_token_attributes_ > 0) {
      for (Attribute& attribute : Attributes()) {
        attribute.~Attribute();
      }
    }
  }

  static HTMLStackItem* Create(
      ContainerNode* node,
      AtomicHTMLToken* token,
      const AtomicString& namespace_uri = html_names::xhtmlNamespaceURI) {
    return MakeGarbageCollected<HTMLStackItem>(
        AdditionalBytes(token->Attributes().size() * sizeof(Attribute)),
        base::PassKey<HTMLStackItem>(), node, token, namespace_uri);
  }

  Element* GetElement() const { return To<Element>(node_.Get()); }
  ContainerNode* GetNode() const { return node_.Get(); }

  bool IsDocumentFragmentNode() const { return is_document_fragment_node_; }
  bool IsElementNode() const { return !is_document_fragment_node_; }

  const AtomicString& NamespaceURI() const { return namespace_uri_; }
  const AtomicString& LocalName() const { return token_name_.GetLocalName(); }

  const HTMLTokenName& GetTokenName() const { return token_name_; }

  const base::span<Attribute> Attributes() {
    DCHECK(LocalName());
    return UNSAFE_TODO({TokenAttributesData(), num_token_attributes_});
  }
  const base::span<const Attribute> Attributes() const {
    DCHECK(LocalName());
    return UNSAFE_TODO({TokenAttributesData(), num_token_attributes_});
  }
  Attribute* GetAttributeItem(const QualifiedName& attribute_name) {
    DCHECK(LocalName());
    return FindAttributeInVector(Attributes(), attribute_name);
  }
  bool HasParsePartsAttribute() {
    if (!LocalName() || !RuntimeEnabledFeatures::DOMPartsAPIEnabled()) {
      return false;
    }
    return GetAttributeItem(html_names::kParsepartsAttr);
  }

  html_names::HTMLTag GetHTMLTag() const { return token_name_.GetHTMLTag(); }

  bool HasLocalName(const AtomicString& name) const {
    return token_name_.GetLocalName() == name;
  }

  bool HasTagName(const QualifiedName& name) const {
    return token_name_.GetLocalName() == name.LocalName() &&
           namespace_uri_ == name.NamespaceURI();
  }

  bool IsHTMLNamespace() const {
    return namespace_uri_ == html_names::xhtmlNamespaceURI;
  }

  bool MatchesHTMLTag(const HTMLTokenName& name) const {
    return name == token_name_ && IsHTMLNamespace();
  }

  bool MatchesHTMLTag(const AtomicString& name) const {
    return HasLocalName(name) && IsHTMLNamespace();
  }

  bool MatchesHTMLTag(html_names::HTMLTag tag) const {
    // Equality of HTMLTag only works if supplied a value other than
    // kUnknownTag.
    DCHECK_NE(tag, html_names::HTMLTag::kUnknown);
    return tag == GetHTMLTag() && IsHTMLNamespace();
  }

  bool CausesFosterParenting() {
    switch (GetHTMLTag()) {
      case html_names::HTMLTag::kTable:
      case html_names::HTMLTag::kTbody:
      case html_names::HTMLTag::kTfoot:
      case html_names::HTMLTag::kThead:
      case html_names::HTMLTag::kTr:
        return namespace_uri_ == html_names::xhtmlNamespaceURI;
      default:
        return false;
    }
  }

  bool IsInHTMLNamespace() const {
    // A DocumentFragment takes the place of the document element when parsing
    // fragments and should be considered in the HTML namespace.
    return NamespaceURI() == html_names::xhtmlNamespaceURI ||
           IsDocumentFragmentNode();  // FIXME: Does this also apply to
                                      // ShadowRoot?
  }

  bool IsNumberedHeaderElement() const {
    switch (GetHTMLTag()) {
      case html_names::HTMLTag::kH1:
      case html_names::HTMLTag::kH2:
      case html_names::HTMLTag::kH3:
      case html_names::HTMLTag::kH4:
      case html_names::HTMLTag::kH5:
      case html_names::HTMLTag::kH6:
        return namespace_uri_ == html_names::xhtmlNamespaceURI;
      default:
        return false;
    }
  }

  bool IsTableBodyContextElement() const {
    switch (GetHTMLTag()) {
      case html_names::HTMLTag::kTbody:
      case html_names::HTMLTag::kTfoot:
      case html_names::HTMLTag::kThead:
        return namespace_uri_ == html_names::xhtmlNamespaceURI;
      default:
        return false;
    }
  }

  // http://www.whatwg.org/specs/web-apps/current-work/multipage/parsing.html#special
  bool IsSpecialNode() const {
    if (IsDocumentFragmentNode())
      return true;
    if (IsInHTMLNamespace()) {
      switch (GetHTMLTag()) {
        case html_names::HTMLTag::kAddress:
        case html_names::HTMLTag::kArea:
        case html_names::HTMLTag::kApplet:
        case html_names::HTMLTag::kArticle:
        case html_names::HTMLTag::kAside:
        case html_names::HTMLTag::kBase:
        case html_names::HTMLTag::kBasefont:
        case html_names::HTMLTag::kBgsound:
        case html_names::HTMLTag::kBlockquote:
        case html_names::HTMLTag::kBody:
        case html_names::HTMLTag::kBr:
        case html_names::HTMLTag::kButton:
        case html_names::HTMLTag::kCaption:
        case html_names::HTMLTag::kCenter:
        case html_names::HTMLTag::kCol:
        case html_names::HTMLTag::kColgroup:
        case html_names::HTMLTag::kCommand:
        case html_names::HTMLTag::kDd:
        case html_names::HTMLTag::kDetails:
        case html_names::HTMLTag::kDir:
        case html_names::HTMLTag::kDiv:
        case html_names::HTMLTag::kDl:
        case html_names::HTMLTag::kDt:
        case html_names::HTMLTag::kEmbed:
        case html_names::HTMLTag::kFieldset:
        case html_names::HTMLTag::kFigcaption:
        case html_names::HTMLTag::kFigure:
        case html_names::HTMLTag::kFooter:
        case html_names::HTMLTag::kForm:
        case html_names::HTMLTag::kFrame:
        case html_names::HTMLTag::kFrameset:
        case html_names::HTMLTag::kH1:
        case html_names::HTMLTag::kH2:
        case html_names::HTMLTag::kH3:
        case html_names::HTMLTag::kH4:
        case html_names::HTMLTag::kH5:
        case html_names::HTMLTag::kH6:
        case html_names::HTMLTag::kHead:
        case html_names::HTMLTag::kHeader:
        case html_names::HTMLTag::kHgroup:
        case html_names::HTMLTag::kHr:
        case html_names::HTMLTag::kHTML:
        case html_names::HTMLTag::kIFrame:
        case html_names::HTMLTag::kImg:
        case html_names::HTMLTag::kInput:
        case html_names::HTMLTag::kLi:
        case html_names::HTMLTag::kLink:
        case html_names::HTMLTag::kListing:
        case html_names::HTMLTag::kMain:
        case html_names::HTMLTag::kMarquee:
        case html_names::HTMLTag::kMenu:
        case html_names::HTMLTag::kMeta:
        case html_names::HTMLTag::kNav:
        case html_names::HTMLTag::kNoembed:
        case html_names::HTMLTag::kNoframes:
        case html_names::HTMLTag::kNoscript:
        case html_names::HTMLTag::kObject:
        case html_names::HTMLTag::kOl:
        case html_names::HTMLTag::kP:
        case html_names::HTMLTag::kParam:
        case html_names::HTMLTag::kPlaintext:
        case html_names::HTMLTag::kPre:
        case html_names::HTMLTag::kScript:
        case html_names::HTMLTag::kSection:
        case html_names::HTMLTag::kSelect:
        case html_names::HTMLTag::kStyle:
        case html_names::HTMLTag::kSummary:
        case html_names::HTMLTag::kTable:
        case html_names::HTMLTag::kTbody:
        case html_names::HTMLTag::kTfoot:
        case html_names::HTMLTag::kThead:
        case html_names::HTMLTag::kTd:
        case html_names::HTMLTag::kTemplate:
        case html_names::HTMLTag::kTextarea:
        case html_names::HTMLTag::kTh:
        case html_names::HTMLTag::kTitle:
        case html_names::HTMLTag::kTr:
        case html_names::HTMLTag::kUl:
        case html_names::HTMLTag::kWbr:
        case html_names::HTMLTag::kXmp:
          return true;
        default:
          return false;
      }
    }
    if (HasTagName(mathml_names::kMiTag) || HasTagName(mathml_names::kMoTag) ||
        HasTagName(mathml_names::kMnTag) || HasTagName(mathml_names::kMsTag) ||
        HasTagName(mathml_names::kMtextTag) ||
        HasTagName(mathml_names::kAnnotationXmlTag) ||
        HasTagName(svg_names::kForeignObjectTag) ||
        HasTagName(svg_names::kDescTag) || HasTagName(svg_names::kTitleTag))
      return true;
    return false;
  }

  HTMLStackItem* NextItemInStack() { return next_item_in_stack_.Get(); }

  bool IsAboveItemInStack(const HTMLStackItem* item) const {
    DCHECK(item);
    HTMLStackItem* below = next_item_in_stack_.Get();
    while (below) {
      if (below == item) {
        return true;
      }
      below = below->NextItemInStack();
    }
    return false;
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(node_);
    visitor->Trace(next_item_in_stack_);
  }

 private:
  void SetNextItemInStack(HTMLStackItem* item) {
    DCHECK(!item || (item && !next_item_in_stack_));
    next_item_in_stack_ = item;
  }

  HTMLStackItem* ReleaseNextItemInStack() {
    return next_item_in_stack_.Release();
  }

  // Needed for stack related functions.
  friend class HTMLElementStack;

  // The attributes are stored directly after the HTMLStackItem in memory
  // (using Oilpan's AdditionalBytes system). Space for this is guaranteed
  // by Create().
  Attribute* TokenAttributesData() {
    static_assert(alignof(HTMLStackItem) >= alignof(Attribute));
    return reinterpret_cast<Attribute*>(UNSAFE_TODO(this + 1));
  }
  const Attribute* TokenAttributesData() const {
    static_assert(alignof(HTMLStackItem) >= alignof(Attribute));
    return reinterpret_cast<const Attribute*>(UNSAFE_TODO(this + 1));
  }

  Member<ContainerNode> node_;

  // This member is maintained by HTMLElementStack.
  Member<HTMLStackItem> next_item_in_stack_{nullptr};

  HTMLTokenName token_name_;
  AtomicString namespace_uri_;
  wtf_size_t num_token_attributes_ = 0;
  bool is_document_fragment_node_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_STACK_ITEM_H_
