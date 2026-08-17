// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HIT_TEST_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HIT_TEST_SOURCE_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

#include "device/vr/public/mojom/vr_service.mojom-blink-forward.h"

namespace blink {

class ExceptionState;
class XRHitTestResult;
class XRSession;

class XRHitTestSource : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRHitTestSource(uint64_t id, XRSession* xr_session);

  uint64_t id() const;

  void cancel(ExceptionState& exception_state);

  // Returns a vector of XRHitTestResults that were obtained during last frame
  // update. This method is not exposed to JavaScript.
  HeapVector<Member<XRHitTestResult>> Results();

  void Update(
      const Vector<device::mojom::blink::XRHitResultPtr>& hit_test_results);

  void Trace(Visitor*) const override;

 private:
  const uint64_t id_;
  Member<XRSession> xr_session_;

  Vector<device::mojom::blink::XRHitResultPtr> last_frame_results_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HIT_TEST_SOURCE_H_
