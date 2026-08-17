/*
 * Copyright (C) 2004, 2005, 2008, 2009 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_URI_REFERENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_URI_REFERENCE_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class Document;
class Element;
class IdTargetObserver;
class QualifiedName;
class SVGAnimatedHref;
class SVGAnimatedString;
class SVGElement;
class TreeScope;
class SVGAnimatedPropertyBase;

class CORE_EXPORT SVGURIReference : public GarbageCollectedMixin {
 public:
  virtual ~SVGURIReference() = default;

  static bool IsKnownAttribute(const QualifiedName&);

  // Use this for accesses to 'href' or 'xlink:href' (in that order) for
  // elements where both are allowed and don't necessarily inherit from
  // SVGURIReference.
  static const AtomicString& LegacyHrefString(const SVGElement&);

  // Like above, but for elements that inherit from SVGURIReference. Resolves
  // against the base URL of the passed Document.
  KURL LegacyHrefURL(const Document&) const;

  static AtomicString FragmentIdentifierFromIRIString(const String&,
                                                      const TreeScope&);
  static Element* TargetElementFromIRIString(const String&,
                                             const TreeScope&,
                                             AtomicString* = nullptr);

  const String& HrefString() const;

  // Create an 'id' observer for the href associated with this SVGURIReference
  // and its corresponding SVGElement (which should be passed as
  // |contextElement|.) Will call buildPendingResource() on |contextElement|
  // when changes to the 'id' are noticed.
  Element* ObserveTarget(Member<IdTargetObserver>&, SVGElement&);
  // Create an 'id' observer for any id denoted by |hrefString|, calling
  // buildPendingResource() on |contextElement| on changes.
  static Element* ObserveTarget(Member<IdTargetObserver>&,
                                SVGElement&,
                                const String& href_string);
  // Create an 'id' observer for |id| in the specified TreeScope. On changes,
  // the passed Closure will be called.
  static Element* ObserveTarget(Member<IdTargetObserver>&,
                                TreeScope&,
                                const AtomicString& id,
                                base::RepeatingClosure);
  // Unregister and destroy the observer.
  static void UnobserveTarget(Member<IdTargetObserver>&);

  // JS API
  SVGAnimatedString* href() const;

  void Trace(Visitor*) const override;

 protected:
  explicit SVGURIReference(SVGElement*);

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const;
  void SynchronizeAllSVGAttributes() const;

 private:
  Member<SVGAnimatedHref> href_;
};

// Helper class used to resolve fragment references. Handles the 'local url
// flag' per https://drafts.csswg.org/css-values/#local-urls .
class SVGURLReferenceResolver {
  STACK_ALLOCATED();

 public:
  SVGURLReferenceResolver(const String& url_string, const Document&);

  bool IsLocal() const;
  KURL AbsoluteUrl() const;
  AtomicString FragmentIdentifier() const;

 private:
  const String& relative_url_;
  const Document* document_;
  mutable KURL absolute_url_;
  bool is_local_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_URI_REFERENCE_H_
