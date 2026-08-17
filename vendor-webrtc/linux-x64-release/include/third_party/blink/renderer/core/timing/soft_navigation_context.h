// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_SOFT_NAVIGATION_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_SOFT_NAVIGATION_CONTEXT_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CORE_EXPORT SoftNavigationContext
    : public GarbageCollected<SoftNavigationContext> {
 public:
  SoftNavigationContext();

  base::TimeTicks UserInteractionTimestamp() const {
    return user_interaction_timestamp_;
  }
  void SetUserInteractionTimestamp(base::TimeTicks value) {
    user_interaction_timestamp_ = value;
  }

  const String& Url() const { return url_; }
  void SetUrl(const String& url);

  void MarkMainModification() { has_main_modification_ = true; }
  bool HasMainModification() const { return has_main_modification_; }

  void Trace(Visitor*) const {}

  bool IsSoftNavigation() const {
    return has_main_modification_ && !url_.empty();
  }

 private:
  base::TimeTicks user_interaction_timestamp_;
  String url_;
  bool has_main_modification_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_SOFT_NAVIGATION_CONTEXT_H_
