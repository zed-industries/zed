/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_NUMBER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_NUMBER_H_

#include "third_party/blink/renderer/core/svg/properties/svg_animated_property.h"
#include "third_party/blink/renderer/core/svg/svg_number_tear_off.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGAnimatedNumberOptionalNumber;

// SVG Spec: http://www.w3.org/TR/SVG11/types.html#InterfaceSVGAnimatedNumber
class SVGAnimatedNumber : public ScriptWrappable,
                          public SVGAnimatedProperty<SVGNumber> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SVGAnimatedNumber(SVGElement* context_element,
                    const QualifiedName& attribute_name,
                    float initial_number)
      : SVGAnimatedNumber(context_element,
                          attribute_name,
                          MakeGarbageCollected<SVGNumber>(initial_number)) {}

  SVGAnimatedNumber(SVGElement* context_element,
                    const QualifiedName& attribute_name,
                    SVGNumber* initial_value)
      : SVGAnimatedProperty<SVGNumber>(
            context_element,
            attribute_name,
            initial_value,
            CSSPropertyID::kInvalid,
            static_cast<unsigned>(initial_value->Value())),
        parent_number_optional_number_(nullptr) {}

  void SynchronizeAttribute() override;

  void SetParentOptionalNumber(
      SVGAnimatedNumberOptionalNumber* number_optional_number) {
    parent_number_optional_number_ = number_optional_number;
  }

  void Trace(Visitor*) const override;

 protected:
  Member<SVGAnimatedNumberOptionalNumber> parent_number_optional_number_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_NUMBER_H_
