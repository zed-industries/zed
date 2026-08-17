/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_PROPERTY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_PROPERTY_H_

#include "third_party/blink/renderer/core/svg/properties/svg_property_info.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class SVGElement;
struct SMILAnimationEffectParameters;

class SVGPropertyBase : public GarbageCollected<SVGPropertyBase> {
 public:
  // Properties do not have a primitive type by default
  typedef void PrimitiveType;

  SVGPropertyBase(const SVGPropertyBase&) = delete;
  SVGPropertyBase& operator=(const SVGPropertyBase&) = delete;

  virtual ~SVGPropertyBase() = default;

  virtual WTF::String ValueAsString() const = 0;

  // Set the initial value based on a per-type defined (encoded) value. Overload
  // this in the specific subclass to handle initial values, and set
  // kInitialValueBits appropriately.
  static constexpr int kInitialValueBits = 0;
  void SetInitial(unsigned) {}

  // FIXME: remove below and just have this inherit AnimatableValue in
  // WebAnimations transition.
  virtual void Add(const SVGPropertyBase*, const SVGElement*) = 0;
  virtual void CalculateAnimatedValue(
      const SMILAnimationEffectParameters&,
      float percentage,
      unsigned repeat_count,
      const SVGPropertyBase* from,
      const SVGPropertyBase* to,
      const SVGPropertyBase* to_at_end_of_duration_value,
      const SVGElement*) = 0;
  virtual float CalculateDistance(const SVGPropertyBase* to,
                                  const SVGElement*) const = 0;

  virtual AnimatedPropertyType GetType() const = 0;

  virtual void Trace(Visitor* visitor) const {}

 protected:
  SVGPropertyBase() = default;
};

template <typename T>
struct ThreadingTrait<T,
                      std::enable_if_t<std::is_base_of_v<SVGPropertyBase, T>>> {
  static constexpr ThreadAffinity kAffinity = kMainThreadOnly;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_PROPERTY_H_
