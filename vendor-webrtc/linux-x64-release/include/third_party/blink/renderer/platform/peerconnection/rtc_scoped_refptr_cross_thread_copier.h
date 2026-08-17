// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_SCOPED_REFPTR_CROSS_THREAD_COPIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_SCOPED_REFPTR_CROSS_THREAD_COPIER_H_

#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/webrtc/api/scoped_refptr.h"

namespace WTF {

template <typename T>
struct CrossThreadCopier<webrtc::scoped_refptr<T>> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = webrtc::scoped_refptr<T>;
  static Type Copy(Type pointer) { return pointer; }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_SCOPED_REFPTR_CROSS_THREAD_COPIER_H_
