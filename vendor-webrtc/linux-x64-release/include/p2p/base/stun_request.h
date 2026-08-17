/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_STUN_REQUEST_H_
#define P2P_BASE_STUN_REQUEST_H_

#include <stddef.h>
#include <stdint.h>

#include <functional>
#include <map>
#include <memory>
#include <string>

#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/stun.h"
#include "api/units/time_delta.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class StunRequest;

const int kAllRequestsForTest = 0;

// Total max timeouts: 39.75 seconds
// For years, this was 9.5 seconds, but for networks that experience moments of
// high RTT (such as 40s on 2G networks), this doesn't work well.
const int STUN_TOTAL_TIMEOUT = 39750;  // milliseconds

// Manages a set of STUN requests, sending and resending until we receive a
// response or determine that the request has timed out.
class StunRequestManager {
 public:
  StunRequestManager(
      TaskQueueBase* thread,
      std::function<void(const void*, size_t, webrtc::StunRequest*)>
          send_packet);
  ~StunRequestManager();

  // Starts sending the given request (perhaps after a delay).
  void Send(StunRequest* request);
  void SendDelayed(StunRequest* request, int delay);

  // If `msg_type` is kAllRequestsForTest, sends all pending requests right
  // away. Otherwise, sends those that have a matching type right away. Only for
  // testing.
  // TODO(tommi): Remove this method and update tests that use it to simulate
  // production code.
  void FlushForTest(int msg_type);

  // Returns true if at least one request with `msg_type` is scheduled for
  // transmission. For testing only.
  // TODO(tommi): Remove this method and update tests that use it to simulate
  // production code.
  bool HasRequestForTest(int msg_type);

  // Removes all stun requests that were added previously.
  void Clear();

  // Determines whether the given message is a response to one of the
  // outstanding requests, and if so, processes it appropriately.
  bool CheckResponse(StunMessage* msg);
  bool CheckResponse(const char* data, size_t size);

  // Called from a StunRequest when a timeout occurs.
  void OnRequestTimedOut(StunRequest* request);

  bool empty() const;

  TaskQueueBase* network_thread() const { return thread_; }

  void SendPacket(const void* data, size_t size, StunRequest* request);

 private:
  typedef std::map<std::string, std::unique_ptr<StunRequest>> RequestMap;

  TaskQueueBase* const thread_;
  RequestMap requests_ RTC_GUARDED_BY(thread_);
  const std::function<void(const void*, size_t, webrtc::StunRequest*)>
      send_packet_;
};

// Represents an individual request to be sent.  The STUN message can either be
// constructed beforehand or built on demand.
class StunRequest {
 public:
  explicit StunRequest(StunRequestManager& manager);
  StunRequest(StunRequestManager& manager,
              std::unique_ptr<StunMessage> message);
  virtual ~StunRequest();

  // The manager handling this request (if it has been scheduled for sending).
  StunRequestManager* manager() { return &manager_; }

  // Returns the transaction ID of this request.
  const std::string& id() const { return msg_->transaction_id(); }

  // Returns the reduced transaction ID of this request.
  uint32_t reduced_transaction_id() const {
    return msg_->reduced_transaction_id();
  }

  // Returns the STUN type of the request message.
  int type();

  // Returns a const pointer to `msg_`.
  const StunMessage* msg() const;

  // Time elapsed since last send (in ms)
  int Elapsed() const;

  // Add method to explitly allow requests w/o password.
  // - STUN_BINDINGs from StunPort to a stun server
  // - The initial TURN_ALLOCATE_REQUEST
  void SetAuthenticationRequired(bool val) { authentication_required_ = val; }
  bool AuthenticationRequired() const { return authentication_required_; }

 protected:
  friend class StunRequestManager;

  // Called by StunRequestManager.
  void Send(TimeDelta delay);

  // Called from FlushForTest.
  // TODO(tommi): Remove when FlushForTest gets removed.
  void ResetTasksForTest();

  StunMessage* mutable_msg() { return msg_.get(); }

  // Called when the message receives a response or times out.
  virtual void OnResponse(StunMessage* /* response */) {}
  virtual void OnErrorResponse(StunMessage* /* response */) {}
  virtual void OnTimeout() {}
  // Called when the message is sent.
  virtual void OnSent();
  // Returns the next delay for resends in milliseconds.
  virtual int resend_delay();

  TaskQueueBase* network_thread() const { return manager_.network_thread(); }

  void set_timed_out();

 private:
  void SendInternal();
  // Calls `PostDelayedTask` to queue up a call to SendInternal after the
  // specified timeout.
  void SendDelayed(TimeDelta delay);

  StunRequestManager& manager_;
  const std::unique_ptr<StunMessage> msg_;
  int64_t tstamp_ RTC_GUARDED_BY(network_thread());
  int count_ RTC_GUARDED_BY(network_thread());
  bool timeout_ RTC_GUARDED_BY(network_thread());
  ScopedTaskSafety task_safety_{
      PendingTaskSafetyFlag::CreateDetachedInactive()};
  bool authentication_required_ = true;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::kAllRequestsForTest;
using ::webrtc::STUN_TOTAL_TIMEOUT;
using ::webrtc::StunRequest;
using ::webrtc::StunRequestManager;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_STUN_REQUEST_H_
