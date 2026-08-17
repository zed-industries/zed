// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_DOCUMENT_LAYOUT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_DOCUMENT_LAYOUT_DEFINITION_H_

#include "third_party/blink/renderer/core/layout/custom/css_layout_definition.h"

namespace blink {

// A document layout definition is a struct which describes the information
// needed by the document about the author defined layout.
// https://drafts.css-houdini.org/css-layout-api/#document-layout-definition
class DocumentLayoutDefinition final
    : public GarbageCollected<DocumentLayoutDefinition> {
 public:
  explicit DocumentLayoutDefinition(CSSLayoutDefinition*);
  ~DocumentLayoutDefinition();

  const Vector<CSSPropertyID>& NativeInvalidationProperties() const {
    return layout_definition_->NativeInvalidationProperties();
  }
  const Vector<AtomicString>& CustomInvalidationProperties() const {
    return layout_definition_->CustomInvalidationProperties();
  }
  const Vector<CSSPropertyID>& ChildNativeInvalidationProperties() const {
    return layout_definition_->ChildNativeInvalidationProperties();
  }
  const Vector<AtomicString>& ChildCustomInvalidationProperties() const {
    return layout_definition_->ChildCustomInvalidationProperties();
  }

  bool RegisterAdditionalLayoutDefinition(const CSSLayoutDefinition&);

  unsigned GetRegisteredDefinitionCount() const {
    return registered_definitions_count_;
  }

  void Trace(Visitor*) const;

 private:
  bool IsEqual(const CSSLayoutDefinition&);

  Member<CSSLayoutDefinition> layout_definition_;
  unsigned registered_definitions_count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_DOCUMENT_LAYOUT_DEFINITION_H_
