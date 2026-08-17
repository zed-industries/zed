/*
 * Copyright (C) 2004, 2005, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2010 Rob Buis <buis@kde.org>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TESTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TESTS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class QualifiedName;
class SVGElement;
class SVGStaticStringList;
class SVGStringListTearOff;
class SVGAnimatedPropertyBase;

class CORE_EXPORT SVGTests : public GarbageCollectedMixin {
 public:
  // JS API
  SVGStringListTearOff* requiredExtensions();
  SVGStringListTearOff* systemLanguage();

  bool IsValid() const;

  void Trace(Visitor*) const override;

 protected:
  explicit SVGTests(SVGElement* context_element);

  static bool IsKnownAttribute(const QualifiedName&);

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const;
  void SynchronizeAllSVGAttributes() const;

 private:
  Member<SVGStaticStringList> required_extensions_;
  Member<SVGStaticStringList> system_language_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TESTS_H_
