// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_THERMAL_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_THERMAL_RESOURCE_H_

#include "base/feature_list.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "third_party/blink/public/mojom/peerconnection/peer_connection_tracker.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/webrtc/api/adaptation/resource.h"

namespace blink {

MODULES_EXPORT BASE_DECLARE_FEATURE(kWebRtcThermalResource);

// The ThermalResource reports kOveruse or kUnderuse every 10 seconds(*) while
// it has a registered listener and the DeviceThermalMeasurement is known.
// Because OnThermalMeasurement() only happens when the thermal state changes,
// repeated kOveruse is needed to adapt multiple steps.
//
// Based on [1] and manual observations, we do not want to adapt if thermals are
// kNominal or kFair so we map these to kUnderuse. But if thermals are kSerious
// or kCritical this is a strong signal from the OS that "corrective action" or
// "immediate corrective action" is needed.
//
// (*) It can easily take a minute before the thermal state changes after load
// distribution has changed, so the effects of ThermalResource is likely to
// result in either maximally adapted or not adapted at all. The repeated
// interval of 10 seconds was somewhat arbitrarily chosen but was chosen as a
// tradeoff between giving the OS time to measure the new load and not making
// the resource too spammy.
//
// [1]
// https://developer.apple.com/library/archive/documentation/Performance/Conceptual/power_efficiency_guidelines_osx/RespondToThermalStateChanges.html
class MODULES_EXPORT ThermalResource : public webrtc::Resource {
 public:
  static scoped_refptr<ThermalResource> Create(
      scoped_refptr<base::SequencedTaskRunner> task_runner);

  explicit ThermalResource(
      scoped_refptr<base::SequencedTaskRunner> task_runner);
  ~ThermalResource() override = default;

  void OnThermalMeasurement(mojom::blink::DeviceThermalState measurement);

  // webrtc::Resource implementation.
  std::string Name() const override;
  void SetResourceListener(webrtc::ResourceListener* listener) override;

 private:
  void ReportMeasurement(size_t measurement_id);
  void ReportMeasurementWhileHoldingLock(size_t measurement_id)
      EXCLUSIVE_LOCKS_REQUIRED(&lock_);

  const scoped_refptr<base::SequencedTaskRunner> task_runner_;
  base::Lock lock_;
  raw_ptr<webrtc::ResourceListener> listener_ GUARDED_BY(&lock_) = nullptr;
  mojom::blink::DeviceThermalState measurement_ GUARDED_BY(&lock_) =
      mojom::blink::DeviceThermalState::kUnknown;
  size_t measurement_id_ GUARDED_BY(&lock_) = 0u;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_THERMAL_RESOURCE_H_
