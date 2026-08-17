// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_TASK_QUEUE_FACTORY_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_TASK_QUEUE_FACTORY_H_

#include <memory>

#include "third_party/webrtc/api/task_queue/task_queue_base.h"
#include "third_party/webrtc/api/task_queue/task_queue_factory.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

// Creates a factory for webrtc::TaskQueueBase that is backed by a
// blink::MetronomeSource. Tested by
// /third_party/blink/renderer/platform/peerconnection/task_queue_factory_test.cc
RTC_EXPORT std::unique_ptr<webrtc::TaskQueueFactory>
CreateWebRtcTaskQueueFactory();

RTC_EXPORT std::unique_ptr<webrtc::TaskQueueBase, webrtc::TaskQueueDeleter>
CreateWebRtcTaskQueue(webrtc::TaskQueueFactory::Priority priority);

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_TASK_QUEUE_FACTORY_H_
