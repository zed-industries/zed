/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_TOKEN_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_TOKEN_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/dom/space_split_string.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Element;
class ExceptionState;

class CORE_EXPORT DOMTokenList : public ScriptWrappable,
                                 public ElementRareDataField {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DOMTokenList(Element& element, const QualifiedName& attr)
      : attribute_name_(attr), element_(element) {}
  DOMTokenList(const DOMTokenList&) = delete;
  DOMTokenList& operator=(const DOMTokenList&) = delete;
  ~DOMTokenList() override = default;
  void Trace(Visitor*) const override;

  unsigned length() const { return token_set_.size(); }
  const AtomicString item(unsigned index) const;
  bool contains(const AtomicString&) const;
  void add(const Vector<String>&, ExceptionState&);
  void remove(const Vector<String>&, ExceptionState&);
  bool toggle(const AtomicString&, ExceptionState&);
  bool toggle(const AtomicString&, bool force, ExceptionState&);
  bool replace(const AtomicString& token,
               const AtomicString& new_token,
               ExceptionState&);
  bool supports(const AtomicString&, ExceptionState&);
  AtomicString value() const;
  void setValue(const AtomicString&);
  AtomicString toString() const { return value(); }

  // This function should be called when the associated attribute value was
  // updated.
  void DidUpdateAttributeValue(const AtomicString& old_value,
                               const AtomicString& new_value);

  const SpaceSplitString& TokenSet() const { return token_set_; }
  // Add() and Remove() have DCHECK for syntax of the specified token.
  void Add(const AtomicString&);
  void Remove(const AtomicString&);

 protected:
  Element& GetElement() const { return *element_; }

  virtual bool ValidateTokenValue(const AtomicString&, ExceptionState&) const;

 private:
  void AddTokens(const Vector<String>&);
  void RemoveTokens(const Vector<String>&);
  void UpdateWithTokenSet(const SpaceSplitString&);

  SpaceSplitString token_set_;
  // Normal DOMTokenList instances is associated to an attribute name.
  // So |attribute_name_| is typically an html_names::kFooAttr.
  // CustomStateTokenList is associated to no attribute name.
  // |attribute_name_| is |g_null_name| in that case.
  const QualifiedName attribute_name_;
  const Member<Element> element_;
  bool is_in_update_step_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_TOKEN_LIST_H_
