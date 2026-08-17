// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_TRANSIENT_INPUT_HIT_TEST_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_TRANSIENT_INPUT_HIT_TEST_RESULT_H_

#include "device/vr/public/mojom/vr_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class XRInputSource;
class XRHitTestResult;

template <typename IDLType>
class FrozenArray;

class XRTransientInputHitTestResult : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRTransientInputHitTestResult(
      XRInputSource* input_source,
      const Vector<device::mojom::blink::XRHitResultPtr>& results);

  XRInputSource* inputSource();

  const FrozenArray<XRHitTestResult>& results() const;

  void Trace(Visitor* visitor) const override;

 private:
  Member<XRInputSource> input_source_;
  Member<FrozenArray<XRHitTestResult>> results_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_TRANSIENT_INPUT_HIT_TEST_RESULT_H_
