/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_ICE_TRANSPORT_H_
#define PC_ICE_TRANSPORT_H_

#include "api/ice_transport_interface.h"
#include "rtc_base/checks.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Implementation of IceTransportInterface that does not take ownership
// of its underlying IceTransport. It depends on its creator class to
// ensure that Clear() is called before the underlying IceTransport
// is deallocated.
class IceTransportWithPointer : public IceTransportInterface {
 public:
  explicit IceTransportWithPointer(IceTransportInternal* internal)
      : creator_thread_(Thread::Current()), internal_(internal) {
    RTC_DCHECK(internal_);
  }

  IceTransportWithPointer() = delete;
  IceTransportWithPointer(const IceTransportWithPointer&) = delete;
  IceTransportWithPointer& operator=(const IceTransportWithPointer&) = delete;

  IceTransportInternal* internal() override;
  // This call will ensure that the pointer passed at construction is
  // no longer in use by this object. Later calls to internal() will return
  // null.
  void Clear();

 protected:
  ~IceTransportWithPointer() override;

 private:
  const Thread* creator_thread_;
  IceTransportInternal* internal_ RTC_GUARDED_BY(creator_thread_);
};

}  // namespace webrtc

#endif  // PC_ICE_TRANSPORT_H_
