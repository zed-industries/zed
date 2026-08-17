/*
 * Copyright (C) 2013 Google, Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_ATOMIC_HTML_TOKEN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_ATOMIC_HTML_TOKEN_H_

#include <memory>
#include <optional>

#include "base/check_op.h"
#include "base/containers/contains.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/attribute.h"
#include "third_party/blink/renderer/core/html/parser/html_token.h"
#include "third_party/blink/renderer/core/html_element_attribute_name_lookup_trie.h"
#include "third_party/blink/renderer/core/html_element_lookup_trie.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/character_visitor.h"

namespace blink {

class AtomicHTMLToken;

// HTMLTokenName represents a parsed token name (the local name of a
// QualifiedName). The token name contains the local name as an AtomicString and
// if the name is a valid html tag name, an HTMLTag. As this class is created
// from tokenized input, it does not know the namespace, the namespace is
// determined later in parsing (see HTMLTreeBuilder and HTMLStackItem).
class CORE_EXPORT HTMLTokenName {
 public:
  explicit HTMLTokenName(html_names::HTMLTag tag) : tag_(tag) {
    if (tag != html_names::HTMLTag::kUnknown)
      local_name_ = html_names::TagToQualifiedName(tag).LocalName();
  }

  // Returns an HTMLTokenName for the specified string. This function looks up
  // the HTMLTag from the supplied string.
  static HTMLTokenName FromLocalName(const AtomicString& local_name) {
    if (local_name.empty())
      return HTMLTokenName(html_names::HTMLTag::kUnknown);

    return WTF::VisitCharacters(local_name, [&local_name](auto chars) {
      return HTMLTokenName(LookupHtmlTag(chars), local_name);
    });
  }

  bool operator==(const HTMLTokenName& other) const {
    return other.local_name_ == local_name_;
  }

  bool IsValidHTMLTag() const { return tag_ != html_names::HTMLTag::kUnknown; }

  html_names::HTMLTag GetHTMLTag() const { return tag_; }

  const AtomicString& GetLocalName() const { return local_name_; }

 private:
  // For access to constructor.
  friend class AtomicHTMLToken;

  explicit HTMLTokenName(html_names::HTMLTag tag, const AtomicString& name)
      : tag_(tag), local_name_(name) {
#if DCHECK_IS_ON()
    if (tag == html_names::HTMLTag::kUnknown) {
      // If the tag is unknown, then `name` must either be empty, or not
      // identify any other HTMLTag.
      if (!name.empty()) {
        WTF::VisitCharacters(name, [](auto chars) {
          DCHECK_EQ(html_names::HTMLTag::kUnknown, LookupHtmlTag(chars));
        });
      }
    }
#endif
  }

  // This constructor is intended for use by AtomicHTMLToken when it is known
  // the string is not a known html tag.
  explicit HTMLTokenName(const AtomicString& name)
      : HTMLTokenName(html_names::HTMLTag::kUnknown, name) {}

  // Store both the tag and the name. The tag is enough to lookup the name, but
  // enough code makes use of the name that's it's worth caching (this performs
  // a bit better than using a variant for the two and looking up on demand).
  html_names::HTMLTag tag_;
  AtomicString local_name_;
};

class CORE_EXPORT AtomicHTMLToken {
  STACK_ALLOCATED();

 public:
  bool ForceQuirks() const {
    DCHECK_EQ(type_, HTMLToken::DOCTYPE);
    return doctype_data_->force_quirks_;
  }

  HTMLToken::TokenType GetType() const { return type_; }

  // TODO(sky): for consistency, rename to GetLocalName().
  const AtomicString& GetName() const {
    DCHECK(UsesName());
    return name_.GetLocalName();
  }

  void SetTokenName(const HTMLTokenName& name) {
    DCHECK(UsesName());
    name_ = name;
  }

  html_names::HTMLTag GetHTMLTag() const { return name_.GetHTMLTag(); }

  bool IsValidHTMLTag() const { return name_.IsValidHTMLTag(); }

  const HTMLTokenName& GetTokenName() const { return name_; }

  bool SelfClosing() const {
    DCHECK(type_ == HTMLToken::kStartTag || type_ == HTMLToken::kEndTag);
    return self_closing_;
  }

  void SetSelfClosingToFalse() {
    DCHECK(self_closing_);
    DCHECK_EQ(type_, HTMLToken::kStartTag);
    DCHECK_EQ(GetHTMLTag(), html_names::HTMLTag::kScript);
    self_closing_ = false;
  }

  bool HasDuplicateAttribute() const { return duplicate_attribute_; }

  Attribute* GetAttributeItem(const QualifiedName& attribute_name) {
    DCHECK(UsesAttributes());
    return FindAttributeInVector(attributes_, attribute_name);
  }

  Vector<Attribute, kAttributePrealloc>& Attributes() {
    DCHECK(UsesAttributes());
    return attributes_;
  }

  const Vector<Attribute, kAttributePrealloc>& Attributes() const {
    DCHECK(UsesAttributes());
    return attributes_;
  }

  const String& Characters() const {
    DCHECK_EQ(type_, HTMLToken::kCharacter);
    return data_;
  }

  const String& Comment() const {
    DCHECK_EQ(type_, HTMLToken::kComment);
    return data_;
  }

  // FIXME: Distinguish between a missing public identifer and an empty one.
  Vector<UChar>& PublicIdentifier() const {
    DCHECK_EQ(type_, HTMLToken::DOCTYPE);
    return doctype_data_->public_identifier_;
  }

  // FIXME: Distinguish between a missing system identifer and an empty one.
  Vector<UChar>& SystemIdentifier() const {
    DCHECK_EQ(type_, HTMLToken::DOCTYPE);
    return doctype_data_->system_identifier_;
  }

  DOMPartTokenType DOMPartType() const {
    DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled());
    DCHECK_EQ(type_, HTMLToken::kDOMPart);
    return dom_part_data_->type_;
  }

  WTF::Vector<String> DOMPartMetadata() const {
    DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled());
    DCHECK_EQ(type_, HTMLToken::kDOMPart);
    return dom_part_data_->metadata_;
  }

  DOMPartsNeeded GetDOMPartsNeeded() {
    DCHECK_EQ(type_, HTMLToken::kStartTag);
    return dom_parts_needed_;
  }

  explicit AtomicHTMLToken(HTMLToken& token)
      : type_(token.GetType()), name_(HTMLTokenNameFromToken(token)) {
    switch (type_) {
      case HTMLToken::kUninitialized:
        NOTREACHED();
      case HTMLToken::DOCTYPE:
        doctype_data_ = token.ReleaseDoctypeData();
        break;
      case HTMLToken::kDOMPart:
        DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled());
        dom_part_data_ = token.ReleaseDOMPartData();
        break;
      case HTMLToken::kEndOfFile:
        break;
      case HTMLToken::kStartTag:
        dom_parts_needed_ = token.GetDOMPartsNeeded();
        [[fallthrough]];
      case HTMLToken::kEndTag: {
        self_closing_ = token.SelfClosing();
        const HTMLToken::AttributeList& attributes = token.Attributes();

        // This limit is set fairly arbitrarily; the main point is to avoid
        // DDoS opportunities or similar with O(n²) behavior by setting lots
        // of attributes.
        const int kMinimumNumAttributesToDedupWithHash = 10;

        if (attributes.size() >= kMinimumNumAttributesToDedupWithHash) {
          InitializeAttributes</*DedupWithHash=*/true>(token.Attributes());
        } else if (attributes.size()) {
          InitializeAttributes</*DedupWithHash=*/false>(token.Attributes());
        }
        break;
      }
      case HTMLToken::kCharacter:
      case HTMLToken::kComment:
        data_ = token.Data().AsString();
        break;
    }
  }

  explicit AtomicHTMLToken(HTMLToken::TokenType type)
      : type_(type), name_(html_names::HTMLTag::kUnknown) {}

  AtomicHTMLToken(HTMLToken::TokenType type,
                  html_names::HTMLTag tag,
                  const Vector<Attribute>& attributes = Vector<Attribute>())
      : type_(type), name_(tag), attributes_(attributes) {
    DCHECK(UsesName());
  }

  AtomicHTMLToken(HTMLToken::TokenType type,
                  const HTMLTokenName& name,
                  const Vector<Attribute>& attributes = Vector<Attribute>())
      : type_(type), name_(name), attributes_(attributes) {
    DCHECK(UsesName());
  }

  AtomicHTMLToken(const AtomicHTMLToken&) = delete;
  AtomicHTMLToken& operator=(const AtomicHTMLToken&) = delete;

#ifndef NDEBUG
  void Show() const;
#endif

 private:
  static HTMLTokenName HTMLTokenNameFromToken(const HTMLToken& token) {
    switch (token.GetType()) {
      case HTMLToken::DOCTYPE:
        // Doctype name may be empty, but not start/end tags.
        if (token.GetName().IsEmpty())
          return HTMLTokenName(html_names::HTMLTag::kUnknown);
        [[fallthrough]];
      case HTMLToken::kStartTag:
      case HTMLToken::kEndTag: {
        const html_names::HTMLTag html_tag = LookupHtmlTag(token.GetName());
        if (html_tag != html_names::HTMLTag::kUnknown)
          return HTMLTokenName(html_tag);
        return HTMLTokenName(token.GetName().AsAtomicString());
      }
      default:
        return HTMLTokenName(html_names::HTMLTag::kUnknown);
    }
  }

  // Sets up and deduplicates attributes.
  //
  // We can deduplicate attributes in two ways; using a hash table
  // (DedupWithHash=true) or by simple linear scanning (DedupWithHash=false).
  // If we don't have many attributes, the linear scan is cheaper than
  // setting up and searching in a hash table, even though the big-O
  // complexity is higher. Thus, we use the hash table only if the caller
  // expects a lot of attributes.
  template <bool DedupWithHash>
  ALWAYS_INLINE void InitializeAttributes(
      const HTMLToken::AttributeList& attributes);

  bool UsesName() const;

  bool UsesAttributes() const;

  HTMLToken::TokenType type_;

  // "name" for DOCTYPE, StartTag, and EndTag
  HTMLTokenName name_;

  // "data" for Comment, "characters" for Character
  String data_;

  // For DOCTYPE
  std::unique_ptr<DoctypeData> doctype_data_;

  // For DOM Parts
  std::unique_ptr<DOMPartData> dom_part_data_;
  DOMPartsNeeded dom_parts_needed_;

  // For StartTag and EndTag
  bool self_closing_ = false;

  bool duplicate_attribute_ = false;

  Vector<Attribute, kAttributePrealloc> attributes_;
};

template <bool DedupWithHash>
void AtomicHTMLToken::InitializeAttributes(
    const HTMLToken::AttributeList& attributes) {
  wtf_size_t size = attributes.size();

  // Track which attributes have already been inserted to avoid N^2
  // behavior with repeated linear searches when populating `attributes_`.
  std::conditional_t<DedupWithHash, HashSet<AtomicString>, int>
      added_attributes;
  if constexpr (DedupWithHash) {
    added_attributes.ReserveCapacityForSize(size);
  }

  // This is only called once, so `attributes_` should be empty.
  DCHECK(attributes_.empty());
  attributes_.ReserveInitialCapacity(size);
  for (const auto& attribute : attributes) {
    if (attribute.NameIsEmpty())
      continue;

    QualifiedName name = LookupHTMLAttributeName(attribute.NameBuffer().data(),
                                                 attribute.NameBuffer().size());
    if (name == g_null_name) {
      name = QualifiedName(attribute.GetName());
    }

    if constexpr (DedupWithHash) {
      if (!added_attributes.insert(name.LocalName()).is_new_entry) {
        duplicate_attribute_ = true;
        continue;
      }
    } else {
      if (base::Contains(attributes_, name.LocalName(),
                         &Attribute::LocalName)) {
        duplicate_attribute_ = true;
        continue;
      }
    }

    // The string pointer in |value| is null for attributes with no values, but
    // the null atom is used to represent absence of attributes; attributes with
    // no values have the value set to an empty atom instead.
    AtomicString value(attribute.GetValue());
    if (value.IsNull()) {
      value = g_empty_atom;
    }
    attributes_.UncheckedAppend(Attribute(std::move(name), std::move(value)));
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_ATOMIC_HTML_TOKEN_H_
