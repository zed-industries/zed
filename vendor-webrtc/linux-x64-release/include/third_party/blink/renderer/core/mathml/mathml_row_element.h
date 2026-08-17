// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_ROW_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_ROW_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/mathml/mathml_element.h"

namespace blink {

class ComputedStyle;
class Document;

class CORE_EXPORT MathMLRowElement : public MathMLElement {
 public:
  explicit MathMLRowElement(const QualifiedName&, Document&);

  void ChildrenChanged(const ChildrenChange&) override;

 private:
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) final;

  bool IsGroupingElement() const override { return true; }
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_ROW_ELEMENT_H_
