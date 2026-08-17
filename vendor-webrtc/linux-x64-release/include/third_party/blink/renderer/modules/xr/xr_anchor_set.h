// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ANCHOR_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ANCHOR_SET_H_

#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_sync_iterator_xr_anchor_set.h"
#include "third_party/blink/renderer/modules/xr/xr_anchor.h"
#include "third_party/blink/renderer/modules/xr/xr_setlike.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"

namespace blink {

class XRAnchorSet : public ScriptWrappable,
                    public XRSetlike<XRAnchorSet, XRAnchor> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRAnchorSet(HeapHashSet<Member<XRAnchor>> anchors);

  void Trace(Visitor* visitor) const override;

 protected:
  const HeapHashSet<Member<XRAnchor>>& elements() const override;

 private:
  HeapHashSet<Member<XRAnchor>> anchors_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ANCHOR_SET_H_
