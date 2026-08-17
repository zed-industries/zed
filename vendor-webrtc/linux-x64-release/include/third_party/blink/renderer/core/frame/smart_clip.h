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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SMART_CLIP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SMART_CLIP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class LocalFrame;

class CORE_EXPORT SmartClipData {
  STACK_ALLOCATED();

 public:
  SmartClipData() = default;

  SmartClipData(const gfx::Rect& rect, const String& string)
      : rect_in_viewport_(rect), string_(string) {}

  const gfx::Rect& RectInViewport() const { return rect_in_viewport_; }
  const String& ClipData() const { return string_; }

 private:
  gfx::Rect rect_in_viewport_;
  String string_;
};

// SmartClip implements support for the copy operation
// with an S-Pen on Samsung devices. The behavior of this
// class is quirky and poorly tested. It's approximately
// trying to do an implementation of columnar selection
// followed by a copy operation.
class CORE_EXPORT SmartClip {
  STACK_ALLOCATED();

 public:
  explicit SmartClip(LocalFrame*);

  SmartClipData DataForRect(const gfx::Rect&);

 private:
  float PageScaleFactor();

  Node* MinNodeContainsNodes(Node* min_node, Node* new_node);
  Node* FindBestOverlappingNode(Node*, const gfx::Rect& crop_rect_in_viewport);
  bool ShouldSkipBackgroundImage(Node*);
  void CollectOverlappingChildNodes(
      Node* parent_node,
      const gfx::Rect& crop_rect_in_viewport,
      HeapVector<Member<Node>>& overlapping_node_info_table);
  String ExtractTextFromNode(Node*);

  LocalFrame* frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SMART_CLIP_H_
