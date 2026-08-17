// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_RESOURCE_COORDINATOR_DOCUMENT_RESOURCE_COORDINATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_RESOURCE_COORDINATOR_DOCUMENT_RESOURCE_COORDINATOR_H_

#include <memory>

#include "components/performance_manager/public/mojom/coordination_unit.mojom-blink.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class BrowserInterfaceBrokerProxy;

class PLATFORM_EXPORT DocumentResourceCoordinator final {
  USING_FAST_MALLOC(DocumentResourceCoordinator);

 public:
  using WebMemoryMeasurementMode =
      ::performance_manager::mojom::blink::WebMemoryMeasurement::Mode;
  using OnWebMemoryMeasurementRequestedCallback = ::performance_manager::mojom::
      blink::DocumentCoordinationUnit::OnWebMemoryMeasurementRequestedCallback;

  // Returns nullptr if instrumentation is not enabled.
  static std::unique_ptr<DocumentResourceCoordinator> MaybeCreate(
      const BrowserInterfaceBrokerProxy&);
  DocumentResourceCoordinator(const DocumentResourceCoordinator&) = delete;
  DocumentResourceCoordinator& operator=(const DocumentResourceCoordinator&) =
      delete;
  ~DocumentResourceCoordinator();

  void SetNetworkAlmostIdle();
  void SetLifecycleState(performance_manager::mojom::LifecycleState);
  void SetHasNonEmptyBeforeUnload(bool has_nonempty_beforeunload);
  void SetIsAdFrame(bool is_ad_frame);
  void OnNonPersistentNotificationCreated();
  void SetHadFormInteraction();
  void SetHadUserEdits();
  void OnStartedUsingWebRTC();
  void OnStoppedUsingWebRTC();
  void OnFirstContentfulPaint(base::TimeDelta time_since_navigation_start);
  void OnWebMemoryMeasurementRequested(
      WebMemoryMeasurementMode mode,
      OnWebMemoryMeasurementRequestedCallback callback);
  void OnFreezingOriginTrialOptOut();

 private:
  explicit DocumentResourceCoordinator(const BrowserInterfaceBrokerProxy&);

  mojo::Remote<performance_manager::mojom::blink::DocumentCoordinationUnit>
      service_;

  bool had_form_interaction_ = false;
  bool had_user_edits_ = false;
  int num_web_rtc_usage_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_RESOURCE_COORDINATOR_DOCUMENT_RESOURCE_COORDINATOR_H_
