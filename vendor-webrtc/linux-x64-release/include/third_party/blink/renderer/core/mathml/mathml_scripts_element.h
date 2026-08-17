// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_SCRIPTS_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_SCRIPTS_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/mathml/mathml_element.h"

namespace blink {

class Document;

enum class MathScriptType {
  kSub,
  kSuper,
  kSubSup,
  kMultiscripts,
  kUnder,
  kOver,
  kUnderOver
};

class CORE_EXPORT MathMLScriptsElement : public MathMLElement {
 public:
  MathScriptType GetScriptType() const { return script_type_; }

  MathMLScriptsElement(const QualifiedName& tagName, Document& document);

 private:
  const MathScriptType script_type_;
};

template <>
struct DowncastTraits<MathMLScriptsElement> {
  static bool AllowFrom(const Node& node) {
    auto* mathml_element = DynamicTo<MathMLElement>(node);
    return mathml_element && AllowFrom(*mathml_element);
  }
  static bool AllowFrom(const MathMLElement& mathml_element) {
    return mathml_element.HasTagName(mathml_names::kMunderTag) ||
           mathml_element.HasTagName(mathml_names::kMoverTag) ||
           mathml_element.HasTagName(mathml_names::kMunderoverTag) ||
           mathml_element.HasTagName(mathml_names::kMsubTag) ||
           mathml_element.HasTagName(mathml_names::kMsupTag) ||
           mathml_element.HasTagName(mathml_names::kMsubsupTag) ||
           mathml_element.HasTagName(mathml_names::kMmultiscriptsTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_SCRIPTS_ELEMENT_H_
