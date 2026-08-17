/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_RESOURCE_ADAPTATION_PROCESSOR_INTERFACE_H_
#define CALL_ADAPTATION_RESOURCE_ADAPTATION_PROCESSOR_INTERFACE_H_

#include <map>
#include <vector>

#include "api/adaptation/resource.h"
#include "api/scoped_refptr.h"
#include "api/video/video_adaptation_counters.h"

namespace webrtc {

class ResourceLimitationsListener {
 public:
  virtual ~ResourceLimitationsListener();

  // The limitations on a resource were changed. This does not mean the current
  // video restrictions have changed.
  virtual void OnResourceLimitationChanged(
      scoped_refptr<Resource> resource,
      const std::map<scoped_refptr<Resource>, VideoAdaptationCounters>&
          resource_limitations) = 0;
};

// The Resource Adaptation Processor is responsible for reacting to resource
// usage measurements (e.g. overusing or underusing CPU). When a resource is
// overused the Processor is responsible for performing mitigations in order to
// consume less resources.
class ResourceAdaptationProcessorInterface {
 public:
  virtual ~ResourceAdaptationProcessorInterface();

  virtual void AddResourceLimitationsListener(
      ResourceLimitationsListener* limitations_listener) = 0;
  virtual void RemoveResourceLimitationsListener(
      ResourceLimitationsListener* limitations_listener) = 0;
  // Starts or stops listening to resources, effectively enabling or disabling
  // processing. May be called from anywhere.
  // TODO(https://crbug.com/webrtc/11172): Automatically register and unregister
  // with AddResource() and RemoveResource() instead. When the processor is
  // multi-stream aware, stream-specific resouces will get added and removed
  // over time.
  virtual void AddResource(scoped_refptr<Resource> resource) = 0;
  virtual std::vector<scoped_refptr<Resource>> GetResources() const = 0;
  virtual void RemoveResource(scoped_refptr<Resource> resource) = 0;
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_RESOURCE_ADAPTATION_PROCESSOR_INTERFACE_H_
