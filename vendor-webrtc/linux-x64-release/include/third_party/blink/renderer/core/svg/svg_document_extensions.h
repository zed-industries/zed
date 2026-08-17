/*
 * Copyright (C) 2006 Apple Inc. All rights reserved.
 * Copyright (C) 2006, 2008 Nikolas Zimmermann <zimmermann@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_DOCUMENT_EXTENSIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_DOCUMENT_EXTENSIONS_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

class Document;
class SVGSVGElement;

class CORE_EXPORT SVGDocumentExtensions final
    : public GarbageCollected<SVGDocumentExtensions> {
 public:
  explicit SVGDocumentExtensions(Document*);
  SVGDocumentExtensions(const SVGDocumentExtensions&) = delete;
  SVGDocumentExtensions& operator=(const SVGDocumentExtensions&) = delete;
  ~SVGDocumentExtensions();

  void AddTimeContainer(SVGSVGElement*);
  void RemoveTimeContainer(SVGSVGElement*);

  // True if a SMIL animation frame is successfully scheduled.
  static bool ServiceSmilOnAnimationFrame(Document&);

  void StartAnimations();
  void PauseAnimations();
  bool HasSmilAnimations() const;
  // True if a SMIL animation frame is successfully scheduled.
  bool ServiceSmilAnimations();

  void DispatchSVGLoadEventToOutermostSVGElements();

  bool ZoomAndPanEnabled() const;

  void StartPan(const gfx::PointF& start);
  void UpdatePan(const gfx::PointF& pos) const;

  static SVGSVGElement* rootElement(const Document&);

  void Trace(Visitor*) const;

 private:
  Member<Document> document_;
  HeapHashSet<Member<SVGSVGElement>> time_containers_;
  gfx::Vector2dF translate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_DOCUMENT_EXTENSIONS_H_
