// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_DOCUMENT_PAINT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_DOCUMENT_PAINT_DEFINITION_H_

#include "third_party/blink/renderer/core/css/css_syntax_definition.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/modules/csspaint/css_paint_definition.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

// A document paint definition is a struct which describes the information
// needed by the document about the author defined image function (which can be
// referenced by the paint function). It consists of:
//   * A input properties which is a list of DOMStrings.
//   * A input argument syntaxes which is a list of parsed CSS Properties and
//     Values.
//   * A context alpha flag.
//
// The document has a map of document paint definitions. Initially the map is
// empty; it is populated when registerPaint(name, paintCtor) is called.
class MODULES_EXPORT DocumentPaintDefinition {
 public:
  explicit DocumentPaintDefinition(
      const Vector<CSSPropertyID>& native_invalidation_properties,
      const Vector<AtomicString>& custom_invalidation_properties,
      const Vector<CSSSyntaxDefinition>& input_argument_types,
      bool alpha);
  virtual ~DocumentPaintDefinition();

  const Vector<CSSPropertyID>& NativeInvalidationProperties() const {
    return native_invalidation_properties_;
  }
  const Vector<AtomicString>& CustomInvalidationProperties() const {
    return custom_invalidation_properties_;
  }
  const Vector<CSSSyntaxDefinition>& InputArgumentTypes() const {
    return input_argument_types_;
  }
  bool alpha() const { return alpha_; }

  bool RegisterAdditionalPaintDefinition(const CSSPaintDefinition&);
  bool RegisterAdditionalPaintDefinition(const Vector<CSSPropertyID>&,
                                         const Vector<String>&,
                                         const Vector<CSSSyntaxDefinition>&,
                                         bool alpha);

  unsigned GetRegisteredDefinitionCount() const {
    return registered_definitions_count_;
  }

 private:
  Vector<CSSPropertyID> native_invalidation_properties_;
  Vector<AtomicString> custom_invalidation_properties_;
  Vector<CSSSyntaxDefinition> input_argument_types_;
  bool alpha_;
  unsigned registered_definitions_count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_DOCUMENT_PAINT_DEFINITION_H_
