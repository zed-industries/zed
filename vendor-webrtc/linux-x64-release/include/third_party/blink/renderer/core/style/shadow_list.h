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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHADOW_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHADOW_LIST_H_

#include "third_party/blink/renderer/core/style/shadow_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace gfx {
class OutsetsF;
class RectF;
}  // namespace gfx

namespace blink {

typedef HeapVector<ShadowData, 1> ShadowDataVector;

// These are used to store shadows in specified order, but we usually want to
// iterate over them backwards as the first-specified shadow is painted on top.
class ShadowList : public GarbageCollected<ShadowList> {
 public:
  explicit ShadowList(ShadowDataVector&& shadows) : shadows_(shadows) {
    // If we have no shadows, we use a null ShadowList
    DCHECK(!shadows.empty());
  }

  void Trace(Visitor* visitor) const { visitor->Trace(shadows_); }

  const ShadowDataVector& Shadows() const { return shadows_; }
  bool operator==(const ShadowList& o) const { return shadows_ == o.shadows_; }
  bool operator!=(const ShadowList& o) const { return !(*this == o); }

  // Outsets needed to include all shadows in this list, as well as the
  // source (i.e. no outsets will be negative).
  gfx::OutsetsF RectOutsetsIncludingOriginal() const;

  void AdjustRectForShadow(gfx::RectF&) const;

 private:
  ShadowDataVector shadows_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHADOW_LIST_H_
