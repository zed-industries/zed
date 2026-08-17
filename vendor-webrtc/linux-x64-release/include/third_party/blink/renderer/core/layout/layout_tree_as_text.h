/*
 * Copyright (C) 2003, 2006, 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TREE_AS_TEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TREE_AS_TEXT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder_stream.h"

namespace gfx {
class Point;
class PointF;
class RectF;
}  // namespace gfx

namespace blink {

class Color;
class PaintLayer;
class Element;
class LocalFrame;
class LayoutBlockFlow;
class LayoutObject;
struct PhysicalRect;

enum LayoutAsTextBehaviorFlags {
  kLayoutAsTextBehaviorNormal = 0,
  // Annotate the layer lists.
  kLayoutAsTextShowLayerNesting = 1 << 1,
  // Show layer and layoutObject addresses.
  kLayoutAsTextShowAddresses = 1 << 2,
  // Show id and class attributes
  kLayoutAsTextShowIDAndClass = 1 << 3,
  // Dump the tree in printing mode.
  kLayoutAsTextPrintingMode = 1 << 4,
  // Don't update layout, to make it safe to call ShowLayerTree() from the
  // debugger inside layout or painting code.
  kLayoutAsTextDontUpdateLayout = 1 << 5,
  // Print the various 'needs layout' bits on layoutObjects.
  kLayoutAsTextShowLayoutState = 1 << 6,
  // Dump the line trees for each LayoutBlockFlow.
  kLayoutAsTextShowLineTrees = 1 << 7,
  // Print paint properties associated with layers and layout objects.
  kLayoutAsTextShowPaintProperties = 1 << 8,
};
typedef unsigned LayoutAsTextBehavior;

// You don't need pageWidthInPixels if you don't specify
// LayoutAsTextInPrintingMode.
CORE_EXPORT String
ExternalRepresentation(LocalFrame*,
                       LayoutAsTextBehavior = kLayoutAsTextBehaviorNormal,
                       const PaintLayer* marked_layer = nullptr);
CORE_EXPORT String
ExternalRepresentation(Element*,
                       LayoutAsTextBehavior = kLayoutAsTextBehaviorNormal);

// Helper function shared with SVGLayoutTreeAsText (so they are not exported).
void Write(StringBuilder&,
           const LayoutObject&,
           wtf_size_t indent = 0,
           LayoutAsTextBehavior = kLayoutAsTextBehaviorNormal);
String QuoteAndEscapeNonPrintables(const String&);
StringBuilder& operator<<(StringBuilder&, const Color&);
StringBuilder& operator<<(StringBuilder& ts, const PhysicalRect& r);
StringBuilder& operator<<(StringBuilder&, const gfx::Point&);
StringBuilder& operator<<(StringBuilder&, const gfx::PointF&);
StringBuilder& operator<<(StringBuilder&, const gfx::RectF&);

CORE_EXPORT String CounterValueForElement(Element*);

CORE_EXPORT String MarkerTextForListItem(Element*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TREE_AS_TEXT_H_
