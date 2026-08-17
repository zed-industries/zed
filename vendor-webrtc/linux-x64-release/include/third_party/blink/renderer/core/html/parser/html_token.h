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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_TOKEN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_TOKEN_H_

#include <memory>
#include <utility>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/dom/attribute.h"
#include "third_party/blink/renderer/core/html/parser/html_parser_idioms.h"
#include "third_party/blink/renderer/core/html/parser/literal_buffer.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

struct DoctypeData {
  USING_FAST_MALLOC(DoctypeData);

 public:
  DoctypeData()
      : has_public_identifier_(false),
        has_system_identifier_(false),
        force_quirks_(false) {}
  DoctypeData(const DoctypeData&) = delete;
  DoctypeData& operator=(const DoctypeData&) = delete;

  bool has_public_identifier_;
  bool has_system_identifier_;
  WTF::Vector<UChar> public_identifier_;
  WTF::Vector<UChar> system_identifier_;
  bool force_quirks_;
};

enum class DOMPartTokenType {
  kChildNodePartStart,
  kChildNodePartEnd,
};

struct DOMPartData {
  USING_FAST_MALLOC(DOMPartData);

 public:
  explicit DOMPartData(DOMPartTokenType type) : type_(type) {
    DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled());
  }
  DOMPartData(const DOMPartData&) = delete;
  DOMPartData& operator=(const DOMPartData&) = delete;

  WTF::Vector<String> metadata_;
  DOMPartTokenType type_;
};

struct DOMPartsNeeded {
 public:
  bool needs_node_part{false};
  Vector<AtomicString> needs_attribute_parts{};
  explicit operator bool() const {
    return needs_node_part || !needs_attribute_parts.empty();
  }
};

static inline Attribute* FindAttributeInVector(base::span<Attribute> attributes,
                                               const QualifiedName& name) {
  for (Attribute& attr : attributes) {
    if (attr.GetName().Matches(name))
      return &attr;
  }
  return nullptr;
}

class HTMLToken {
  USING_FAST_MALLOC(HTMLToken);

 public:
  enum TokenType {
    kUninitialized,
    DOCTYPE,
    kStartTag,
    kEndTag,
    kComment,
    kCharacter,
    kEndOfFile,
    kDOMPart,
  };

  class Attribute {
   public:
    AtomicString GetName() const { return name_.AsAtomicString(); }
    AtomicString GetValue() const { return value_.AsAtomicString(); }

    const UCharLiteralBuffer<32>& NameBuffer() const { return name_; }

    String NameAttemptStaticStringCreation() const {
      return AttemptStaticStringCreation(name_);
    }

    bool NameIsEmpty() const { return name_.IsEmpty(); }
    void AppendToName(UChar c) { name_.AddChar(c); }

    String Value() const { return value_.AsString(); }

    void AppendToValue(UChar c) { value_.AddChar(c); }
    void ClearValue() { value_.clear(); }

   private:
    // TODO(chromium:1204030): Do a more rigorous study and select a
    // better-informed inline capacity.
    UCharLiteralBuffer<32> name_;
    UCharLiteralBuffer<32> value_;
  };

  typedef Vector<Attribute, kAttributePrealloc> AttributeList;

  // By using an inline capacity of 256, we avoid spilling over into an malloced
  // buffer approximately 99% of the time based on a non-scientific browse
  // around a number of popular web sites on 23 May 2013.
  // TODO(chromium:1204030): Do a more rigorous study and select a
  // better-informed inline capacity.
  using DataVector = UCharLiteralBuffer<256>;

  HTMLToken() = default;

  HTMLToken(const HTMLToken&) = delete;
  HTMLToken& operator=(const HTMLToken&) = delete;

  std::unique_ptr<HTMLToken> Take() {
    std::unique_ptr<HTMLToken> copy = std::make_unique<HTMLToken>();
    copy->data_ = std::move(data_);
    copy->attributes_ = std::move(attributes_);
    copy->doctype_data_ = std::move(doctype_data_);
    copy->dom_part_data_ = std::move(dom_part_data_);
    copy->type_ = type_;
    copy->self_closing_ = self_closing_;
    copy->dom_parts_needed_ = dom_parts_needed_;
    // Reset to uninitialized.
    Clear();
    return copy;
  }

  ALWAYS_INLINE void Clear() {
    if (type_ == kUninitialized)
      return;

    type_ = kUninitialized;
    data_.clear();
    if (current_attribute_) {
      current_attribute_ = nullptr;
      attributes_.clear();
    }
  }

  bool IsUninitialized() { return type_ == kUninitialized; }
  TokenType GetType() const { return type_; }

  void MakeEndOfFile() {
    DCHECK_EQ(type_, kUninitialized);
    type_ = kEndOfFile;
  }

  const DataVector& Data() const {
    DCHECK(type_ == kCharacter || type_ == kComment || type_ == kStartTag ||
           type_ == kEndTag);
    return data_;
  }

  const DataVector& GetName() const {
    DCHECK(type_ == kStartTag || type_ == kEndTag || type_ == DOCTYPE);
    return data_;
  }

  ALWAYS_INLINE void AppendToName(UChar character) {
    DCHECK(type_ == kStartTag || type_ == kEndTag || type_ == DOCTYPE);
    DCHECK(character);
    data_.AddChar(character);
  }

  /* DOCTYPE Tokens */

  bool ForceQuirks() const {
    DCHECK_EQ(type_, DOCTYPE);
    return doctype_data_->force_quirks_;
  }

  void SetForceQuirks() {
    DCHECK_EQ(type_, DOCTYPE);
    doctype_data_->force_quirks_ = true;
  }

  void BeginDOCTYPE() {
    DCHECK_EQ(type_, kUninitialized);
    type_ = DOCTYPE;
    doctype_data_ = std::make_unique<DoctypeData>();
  }

  void BeginDOCTYPE(UChar character) {
    DCHECK(character);
    BeginDOCTYPE();
    data_.AddChar(character);
  }

  // FIXME: Distinguish between a missing public identifer and an empty one.
  const WTF::Vector<UChar>& PublicIdentifier() const {
    DCHECK_EQ(type_, DOCTYPE);
    return doctype_data_->public_identifier_;
  }

  // FIXME: Distinguish between a missing system identifer and an empty one.
  const WTF::Vector<UChar>& SystemIdentifier() const {
    DCHECK_EQ(type_, DOCTYPE);
    return doctype_data_->system_identifier_;
  }

  void SetPublicIdentifierToEmptyString() {
    DCHECK_EQ(type_, DOCTYPE);
    doctype_data_->has_public_identifier_ = true;
    doctype_data_->public_identifier_.clear();
  }

  void SetSystemIdentifierToEmptyString() {
    DCHECK_EQ(type_, DOCTYPE);
    doctype_data_->has_system_identifier_ = true;
    doctype_data_->system_identifier_.clear();
  }

  void AppendToPublicIdentifier(UChar character) {
    DCHECK(character);
    DCHECK_EQ(type_, DOCTYPE);
    DCHECK(doctype_data_->has_public_identifier_);
    doctype_data_->public_identifier_.push_back(character);
  }

  void AppendToSystemIdentifier(UChar character) {
    DCHECK(character);
    DCHECK_EQ(type_, DOCTYPE);
    DCHECK(doctype_data_->has_system_identifier_);
    doctype_data_->system_identifier_.push_back(character);
  }

  std::unique_ptr<DoctypeData> ReleaseDoctypeData() {
    return std::move(doctype_data_);
  }

  /* Start/End Tag Tokens */

  bool SelfClosing() const {
    DCHECK(type_ == kStartTag || type_ == kEndTag);
    return self_closing_;
  }

  void SetSelfClosing() {
    DCHECK(type_ == kStartTag || type_ == kEndTag);
    self_closing_ = true;
  }

  ALWAYS_INLINE void BeginStartTag(LChar character) {
    DCHECK(character);
    DCHECK_EQ(type_, kUninitialized);
    type_ = kStartTag;
    self_closing_ = false;
    dom_parts_needed_ = {};
    DCHECK(!current_attribute_);
    DCHECK(attributes_.empty());

    data_.AddChar(character);
  }

  ALWAYS_INLINE void BeginEndTag(LChar character) {
    DCHECK_EQ(type_, kUninitialized);
    type_ = kEndTag;
    self_closing_ = false;
    DCHECK(!current_attribute_);
    DCHECK(attributes_.empty());

    data_.AddChar(character);
  }

  ALWAYS_INLINE void BeginEndTag(const LCharLiteralBuffer<32>& characters) {
    DCHECK_EQ(type_, kUninitialized);
    type_ = kEndTag;
    self_closing_ = false;
    DCHECK(!current_attribute_);
    DCHECK(attributes_.empty());

    data_.AppendLiteral(characters);
  }

  ALWAYS_INLINE void AddNewAttribute(UChar character) {
    DCHECK(type_ == kStartTag || type_ == kEndTag);
    attributes_.Grow(attributes_.size() + 1);
    current_attribute_ = &attributes_.back();
    current_attribute_->AppendToName(character);
  }

  ALWAYS_INLINE void AppendToAttributeName(UChar character) {
    DCHECK(character);
    DCHECK(type_ == kStartTag || type_ == kEndTag);
    current_attribute_->AppendToName(character);
  }

  ALWAYS_INLINE void AppendToAttributeValue(UChar character) {
    DCHECK(character);
    DCHECK(type_ == kStartTag || type_ == kEndTag);
    current_attribute_->AppendToValue(character);
  }

  const AttributeList& Attributes() const {
    DCHECK(type_ == kStartTag || type_ == kEndTag);
    return attributes_;
  }

  const Attribute* GetAttributeItem(const QualifiedName& name) const {
    for (unsigned i = 0; i < attributes_.size(); ++i) {
      if (attributes_.at(i).GetName() == name.LocalName())
        return &attributes_.at(i);
    }
    return nullptr;
  }

  /* Character Tokens */

  // Starting a character token works slightly differently than starting
  // other types of tokens because we want to save a per-character branch.
  ALWAYS_INLINE void EnsureIsCharacterToken() {
    DCHECK(type_ == kUninitialized || type_ == kCharacter);
    type_ = kCharacter;
  }

  const DataVector& Characters() const {
    DCHECK_EQ(type_, kCharacter);
    return data_;
  }

  ALWAYS_INLINE void AppendToCharacter(char character) {
    DCHECK_EQ(type_, kCharacter);
    data_.AddChar(character);
  }

  ALWAYS_INLINE void AppendToCharacter(UChar character) {
    DCHECK_EQ(type_, kCharacter);
    data_.AddChar(character);
  }

  ALWAYS_INLINE void AppendToCharacter(
      const LCharLiteralBuffer<32>& characters) {
    DCHECK_EQ(type_, kCharacter);
    data_.AppendLiteral(characters);
  }

  /* Comment Tokens */

  const DataVector& Comment() const {
    DCHECK_EQ(type_, kComment);
    return data_;
  }

  ALWAYS_INLINE void BeginComment() {
    DCHECK_EQ(type_, kUninitialized);
    type_ = kComment;
  }

  ALWAYS_INLINE void AppendToComment(UChar character) {
    DCHECK(character);
    DCHECK_EQ(type_, kComment);
    data_.AddChar(character);
  }

  /* DOM Part Tokens */

  ALWAYS_INLINE void BeginDOMPart(DOMPartTokenType type) {
    DCHECK_EQ(type_, kUninitialized);
    DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled());
    type_ = kDOMPart;
    dom_part_data_ = std::make_unique<DOMPartData>(type);
  }

  std::unique_ptr<DOMPartData> ReleaseDOMPartData() {
    DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled());
    return std::move(dom_part_data_);
  }

  DOMPartsNeeded GetDOMPartsNeeded() {
    DCHECK_EQ(type_, kStartTag);
    return dom_parts_needed_;
  }

  void SetNeedsNodePart() {
    DCHECK_EQ(type_, kStartTag);
    dom_parts_needed_.needs_node_part = true;
  }

  void SetNeedsAttributePart() {
    DCHECK_EQ(type_, kStartTag);
    DCHECK(!current_attribute_->NameIsEmpty());
    dom_parts_needed_.needs_attribute_parts.push_back(
        current_attribute_->GetName());
  }

 private:
  DataVector data_;

  AttributeList attributes_;

  // A pointer into attributes_ used during lexing.
  Attribute* current_attribute_ = nullptr;

  // For DOCTYPE
  std::unique_ptr<DoctypeData> doctype_data_;

  // For DOM Parts API
  std::unique_ptr<DOMPartData> dom_part_data_;
  DOMPartsNeeded dom_parts_needed_;

  TokenType type_ = kUninitialized;

  // For StartTag and EndTag
  bool self_closing_;
};

#ifndef NDEBUG
const char* ToString(HTMLToken::TokenType);
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_TOKEN_H_
