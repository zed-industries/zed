/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_DTLS_TRANSPORT_H_
#define PC_DTLS_TRANSPORT_H_

#include <memory>
#include <utility>

#include "api/dtls_transport_interface.h"
#include "api/ice_transport_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "p2p/dtls/dtls_transport_internal.h"
#include "pc/ice_transport.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class IceTransportWithPointer;

// This implementation wraps a webrtc::DtlsTransportInternalImpl, and takes
// ownership of it.
class DtlsTransport : public DtlsTransportInterface {
 public:
  // This object must be constructed and updated on a consistent thread,
  // the same thread as the one the webrtc::DtlsTransportInternal object
  // lives on.
  // The Information() function can be called from a different thread,
  // such as the signalling thread.
  explicit DtlsTransport(std::unique_ptr<DtlsTransportInternal> internal);

  scoped_refptr<IceTransportInterface> ice_transport() override;

  // Currently called from the signaling thread and potentially Chromium's
  // JS thread.
  DtlsTransportInformation Information() override;

  void RegisterObserver(DtlsTransportObserverInterface* observer) override;
  void UnregisterObserver() override;
  void Clear();

  DtlsTransportInternal* internal() {
    RTC_DCHECK_RUN_ON(owner_thread_);
    return internal_dtls_transport_.get();
  }

  const DtlsTransportInternal* internal() const {
    RTC_DCHECK_RUN_ON(owner_thread_);
    return internal_dtls_transport_.get();
  }

 protected:
  ~DtlsTransport();

 private:
  void OnInternalDtlsState(DtlsTransportInternal* transport,
                           DtlsTransportState state);
  void UpdateInformation();

  // Called when changing `info_`. We only change the values from the
  // `owner_thread_` (a.k.a. the network thread).
  void set_info(DtlsTransportInformation&& info) RTC_RUN_ON(owner_thread_) {
    MutexLock lock(&lock_);
    info_ = std::move(info);
  }

  DtlsTransportObserverInterface* observer_ = nullptr;
  Thread* owner_thread_;
  mutable Mutex lock_;
  DtlsTransportInformation info_ RTC_GUARDED_BY(lock_);
  std::unique_ptr<DtlsTransportInternal> internal_dtls_transport_
      RTC_GUARDED_BY(owner_thread_);
  const scoped_refptr<IceTransportWithPointer> ice_transport_;
};

}  // namespace webrtc
#endif  // PC_DTLS_TRANSPORT_H_
