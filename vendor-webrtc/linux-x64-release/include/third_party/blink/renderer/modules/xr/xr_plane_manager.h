// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_PLANE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_PLANE_MANAGER_H_

#include "base/types/pass_key.h"
#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/modules/xr/xr_session.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class XRPlane;
class XRPlaneSet;

// Helper class, used to separate the code related to plane processing out of
// XRSession.
class XRPlaneManager : public GarbageCollected<XRPlaneManager> {
 public:
  explicit XRPlaneManager(base::PassKey<XRSession> pass_key,
                          XRSession* session);

  void ProcessPlaneInformation(
      const device::mojom::blink::XRPlaneDetectionData* detected_planes_data,
      double timestamp);

  XRPlaneSet* GetDetectedPlanes() const;

  void Trace(Visitor* visitor) const;

 private:
  Member<XRSession> session_;

  HeapHashMap<uint64_t, Member<XRPlane>> plane_ids_to_planes_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_PLANE_MANAGER_H_
