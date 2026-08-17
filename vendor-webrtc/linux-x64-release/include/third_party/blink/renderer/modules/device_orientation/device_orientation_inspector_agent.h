// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_INSPECTOR_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_INSPECTOR_AGENT_H_

#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/device_orientation.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class DeviceOrientationController;
class InspectedFrames;

// This is the Blink part of the implementation of the DeviceOrientation CDP
// domain. Handling of this domain begins in DeviceOrientationHandler in
// content and reaches this class after initial processing.
//
// The code here is responsible for showing DevTools messages and restarting
// DeviceOrientationEventPump as necessary when overrides are enabled or
// disabled so that the Mojo connections are torn down and reestablished if
// required.
class MODULES_EXPORT DeviceOrientationInspectorAgent final
    : public InspectorBaseAgent<protocol::DeviceOrientation::Metainfo> {
 public:
  explicit DeviceOrientationInspectorAgent(InspectedFrames*);

  DeviceOrientationInspectorAgent(const DeviceOrientationInspectorAgent&) =
      delete;
  DeviceOrientationInspectorAgent& operator=(
      const DeviceOrientationInspectorAgent&) = delete;

  ~DeviceOrientationInspectorAgent() override;
  void Trace(Visitor*) const override;

  // Protocol methods.
  protocol::Response setDeviceOrientationOverride(double,
                                                  double,
                                                  double) override;
  protocol::Response clearDeviceOrientationOverride() override;

  protocol::Response disable() override;
  void Restore() override;

 private:
  DeviceOrientationController& Controller();

  Member<InspectedFrames> inspected_frames_;
  InspectorAgentState::Boolean enabled_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_INSPECTOR_AGENT_H_
