// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_PRINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_PRINTER_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

#if DCHECK_IS_ON()

namespace blink {

class LocalFrameView;
class LayoutObject;
class ObjectPaintProperties;
class VisualViewport;

namespace paint_property_tree_printer {

void UpdateDebugNames(const VisualViewport&);
void UpdateDebugNames(const LayoutObject&, ObjectPaintProperties&);

}  // namespace paint_property_tree_printer

}  // namespace blink

// Outside the blink namespace for ease of invocation from gdb.
CORE_EXPORT void ShowAllPropertyTrees(const blink::LocalFrameView& rootFrame);
CORE_EXPORT void ShowTransformPropertyTree(
    const blink::LocalFrameView& rootFrame);
CORE_EXPORT void ShowClipPropertyTree(const blink::LocalFrameView& rootFrame);
CORE_EXPORT void ShowEffectPropertyTree(const blink::LocalFrameView& rootFrame);
CORE_EXPORT void ShowScrollPropertyTree(const blink::LocalFrameView& rootFrame);
CORE_EXPORT String
TransformPropertyTreeAsString(const blink::LocalFrameView& rootFrame);
CORE_EXPORT String
ClipPropertyTreeAsString(const blink::LocalFrameView& rootFrame);
CORE_EXPORT String
EffectPropertyTreeAsString(const blink::LocalFrameView& rootFrame);
CORE_EXPORT String
ScrollPropertyTreeAsString(const blink::LocalFrameView& rootFrame);

#endif  // if DCHECK_IS_ON()

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_PRINTER_H_
