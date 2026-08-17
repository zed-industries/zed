// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_HOLDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_HOLDER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class Node;

class CORE_EXPORT ContentHolder : public GarbageCollected<ContentHolder> {
 public:
  ContentHolder();
  ContentHolder(Node* node, const gfx::Rect& rect);
  virtual ~ContentHolder();

  Node* node() const { return node_.Get(); }
  const gfx::Rect& rect() const { return rect_; }

  void Trace(Visitor*) const;

 private:
  WeakMember<Node> node_;
  gfx::Rect rect_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_HOLDER_H_
