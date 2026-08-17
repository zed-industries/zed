// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_UA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_UA_H_

#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/navigator_ua_data.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CORE_EXPORT NavigatorUA {
 public:
  NavigatorUAData* userAgentData();

 protected:
  virtual UserAgentMetadata GetUserAgentMetadata() const = 0;
  virtual ExecutionContext* GetUAExecutionContext() const = 0;

  // Record identifiability study metrics for NavigatorUAData if the user is in
  // the study.
  void MaybeRecordMetrics(const NavigatorUAData& ua_data);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_UA_H_
