// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_DOCUMENT_STYLE_ENVIRONMENT_VARIABLES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_DOCUMENT_STYLE_ENVIRONMENT_VARIABLES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_environment_variables.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Document;
class FeatureContext;

// DocumentStyleEnvironmentVariables is bound 1:1 to a document and provides
// document level invalidation logic.
class CORE_EXPORT DocumentStyleEnvironmentVariables
    : public StyleEnvironmentVariables {
 public:
  // Create an instance bound to |parent| that will invalidate |document|'s
  // style when a variable is changed.
  DocumentStyleEnvironmentVariables(StyleEnvironmentVariables& parent,
                                    Document& document);

  void Trace(Visitor* visitor) const override {
    visitor->Trace(document_);
    StyleEnvironmentVariables::Trace(visitor);
  }

  // Resolve the variable |name| and return the data. This will also cause
  // future changes to this variable to invalidate the associated document's
  // style. If |record_metrics| is true we will record UseCounter metrics when
  // this function is called.
  CSSVariableData* ResolveVariable(const AtomicString& name,
                                   WTF::Vector<unsigned> indices,
                                   bool record_metrics);

  // Resolve the variable |name| and return the data. This will also cause
  // future changes to this variable to invalidate the associated document's
  // style. UseCounter metrics will be recorded when this function is used.
  CSSVariableData* ResolveVariable(const AtomicString& name,
                                   WTF::Vector<unsigned> indices) override;

  const FeatureContext* GetFeatureContext() const override;

 protected:
  // Called when variable |name| is changed. This will notify any children that
  // this variable has changed.
  void InvalidateVariable(const AtomicString& name) override;

 private:
  // Record variable usage using |UseCounter|.
  void RecordVariableUsage(const AtomicString& name);

  HashSet<AtomicString> seen_variables_;
  Member<Document> document_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_DOCUMENT_STYLE_ENVIRONMENT_VARIABLES_H_
