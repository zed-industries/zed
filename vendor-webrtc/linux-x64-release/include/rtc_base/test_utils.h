/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_TEST_UTILS_H_
#define RTC_BASE_TEST_UTILS_H_

// Utilities for testing rtc infrastructure in unittests

#include <map>
#include <utility>

#include "rtc_base/socket.h"
#include "rtc_base/third_party/sigslot/sigslot.h"

namespace webrtc {
namespace testing {

///////////////////////////////////////////////////////////////////////////////
// StreamSink - Monitor asynchronously signalled events from Socket.
///////////////////////////////////////////////////////////////////////////////

// Note: Any event that is an error is treated as SSE_ERROR instead of that
// event.

enum StreamSinkEvent {
  SSE_OPEN = 1,
  SSE_READ = 2,
  SSE_WRITE = 4,
  SSE_CLOSE = 8,
  SSE_ERROR = 16
};

class StreamSink : public sigslot::has_slots<> {
 public:
  StreamSink();
  ~StreamSink() override;

  void Monitor(Socket* socket) {
    socket->SignalConnectEvent.connect(this, &StreamSink::OnConnectEvent);
    socket->SignalReadEvent.connect(this, &StreamSink::OnReadEvent);
    socket->SignalWriteEvent.connect(this, &StreamSink::OnWriteEvent);
    socket->SignalCloseEvent.connect(this, &StreamSink::OnCloseEvent);
    // In case you forgot to unmonitor a previous object with this address
    events_.erase(socket);
  }
  void Unmonitor(Socket* socket) {
    socket->SignalConnectEvent.disconnect(this);
    socket->SignalReadEvent.disconnect(this);
    socket->SignalWriteEvent.disconnect(this);
    socket->SignalCloseEvent.disconnect(this);
    events_.erase(socket);
  }
  bool Check(Socket* socket, StreamSinkEvent event, bool reset = true) {
    return DoCheck(socket, event, reset);
  }

 private:
  typedef std::map<Socket*, int> EventMap;

  void OnConnectEvent(Socket* socket) { AddEvents(socket, SSE_OPEN); }
  void OnReadEvent(Socket* socket) { AddEvents(socket, SSE_READ); }
  void OnWriteEvent(Socket* socket) { AddEvents(socket, SSE_WRITE); }
  void OnCloseEvent(Socket* socket, int error) {
    AddEvents(socket, (0 == error) ? SSE_CLOSE : SSE_ERROR);
  }

  void AddEvents(Socket* obj, int events) {
    EventMap::iterator it = events_.find(obj);
    if (events_.end() == it) {
      events_.insert(EventMap::value_type(obj, events));
    } else {
      it->second |= events;
    }
  }
  bool DoCheck(Socket* obj, StreamSinkEvent event, bool reset) {
    EventMap::iterator it = events_.find(obj);
    if ((events_.end() == it) || (0 == (it->second & event))) {
      return false;
    }
    if (reset) {
      it->second &= ~event;
    }
    return true;
  }

  EventMap events_;
};

}  // namespace testing
}  // namespace webrtc

#endif  // RTC_BASE_TEST_UTILS_H_
