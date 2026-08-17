/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_ADAPTATION_RESOURCE_H_
#define API_ADAPTATION_RESOURCE_H_

#include <string>

#include "api/ref_count.h"
#include "api/scoped_refptr.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class Resource;

enum class ResourceUsageState {
  // Action is needed to minimze the load on this resource.
  kOveruse,
  // Increasing the load on this resource is desired, if possible.
  kUnderuse,
};

RTC_EXPORT const char* ResourceUsageStateToString(
    ResourceUsageState usage_state);

class RTC_EXPORT ResourceListener {
 public:
  virtual ~ResourceListener();

  virtual void OnResourceUsageStateMeasured(scoped_refptr<Resource> resource,
                                            ResourceUsageState usage_state) = 0;
};

// A Resource monitors an implementation-specific resource. It may report
// kOveruse or kUnderuse when resource usage is high or low enough that we
// should perform some sort of mitigation to fulfil the resource's constraints.
//
// The methods on this interface are invoked on the adaptation task queue.
// Resource usage measurements may be performed on an any task queue.
//
// The Resource is reference counted to prevent use-after-free when posting
// between task queues. As such, the implementation MUST NOT make any
// assumptions about which task queue Resource is destructed on.
class RTC_EXPORT Resource : public RefCountInterface {
 public:
  Resource();
  // Destruction may happen on any task queue.
  ~Resource() override;

  virtual std::string Name() const = 0;
  // The `listener` may be informed of resource usage measurements on any task
  // queue, but not after this method is invoked with the null argument.
  virtual void SetResourceListener(ResourceListener* listener) = 0;
};

}  // namespace webrtc

#endif  // API_ADAPTATION_RESOURCE_H_
