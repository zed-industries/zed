/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_STREAM_H_
#define RTC_BASE_STREAM_H_

#include <cstddef>
#include <cstdint>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "api/array_view.h"
#include "api/sequence_checker.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/third_party/sigslot/sigslot.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

///////////////////////////////////////////////////////////////////////////////
// StreamInterface is a generic asynchronous stream interface, supporting read,
// write, and close operations, and asynchronous signalling of state changes.
// The interface is designed with file, memory, and socket implementations in
// mind.  Some implementations offer extended operations, such as seeking.
///////////////////////////////////////////////////////////////////////////////

// The following enumerations are declared outside of the StreamInterface
// class for brevity in use.

// The SS_OPENING state indicates that the stream will signal open or closed
// in the future.
enum StreamState { SS_CLOSED, SS_OPENING, SS_OPEN };

// Stream read/write methods return this value to indicate various success
// and failure conditions described below.
enum StreamResult { SR_ERROR, SR_SUCCESS, SR_BLOCK, SR_EOS };

// StreamEvents are used to asynchronously signal state transitionss.  The flags
// may be combined.
//  SE_OPEN: The stream has transitioned to the SS_OPEN state
//  SE_CLOSE: The stream has transitioned to the SS_CLOSED state
//  SE_READ: Data is available, so Read is likely to not return SR_BLOCK
//  SE_WRITE: Data can be written, so Write is likely to not return SR_BLOCK
enum StreamEvent { SE_OPEN = 1, SE_READ = 2, SE_WRITE = 4, SE_CLOSE = 8 };

class RTC_EXPORT StreamInterface {
 public:
  virtual ~StreamInterface() {}

  StreamInterface(const StreamInterface&) = delete;
  StreamInterface& operator=(const StreamInterface&) = delete;

  virtual StreamState GetState() const = 0;

  // Read attempts to fill buffer of size buffer_len.  Write attempts to send
  // data_len bytes stored in data.  The variables read and write are set only
  // on SR_SUCCESS (see below).  Likewise, error is only set on SR_ERROR.
  // Read and Write return a value indicating:
  //  SR_ERROR: an error occurred, which is returned in a non-null error
  //    argument.  Interpretation of the error requires knowledge of the
  //    stream's concrete type, which limits its usefulness.
  //  SR_SUCCESS: some number of bytes were successfully written, which is
  //    returned in a non-null read/write argument.
  //  SR_BLOCK: the stream is in non-blocking mode, and the operation would
  //    block, or the stream is in SS_OPENING state.
  //  SR_EOS: the end-of-stream has been reached, or the stream is in the
  //    SS_CLOSED state.

  virtual StreamResult Read(ArrayView<uint8_t> buffer,
                            size_t& read,
                            int& error) = 0;
  virtual StreamResult Write(ArrayView<const uint8_t> data,
                             size_t& written,
                             int& error) = 0;

  // Attempt to transition to the SS_CLOSED state.  SE_CLOSE will not be
  // signalled as a result of this call.
  virtual void Close() = 0;

  // Streams may issue one or more events to indicate state changes to a
  // provided callback.
  // The first argument is a bit-wise combination of `StreamEvent` flags.
  // If SE_CLOSE is set, then the second argument is the associated error code.
  // Otherwise, the value of the second parameter is undefined and should be
  // set to 0.
  // Note: Not all streams support callbacks.  However, SS_OPENING and
  // SR_BLOCK returned from member functions imply that certain callbacks will
  // be made in the future.
  void SetEventCallback(absl::AnyInvocable<void(int, int)> callback) {
    RTC_DCHECK_RUN_ON(&callback_sequence_);
    RTC_DCHECK(!callback_ || !callback);
    callback_ = std::move(callback);
  }

  // TODO(bugs.webrtc.org/11943): Remove after updating downstream code.
  sigslot::signal3<StreamInterface*, int, int> SignalEvent
      [[deprecated("Use SetEventCallback instead")]];

  // Return true if flush is successful.
  virtual bool Flush();

  //
  // CONVENIENCE METHODS
  //
  // These methods are implemented in terms of other methods, for convenience.
  //

  // WriteAll is a helper function which repeatedly calls Write until all the
  // data is written, or something other than SR_SUCCESS is returned.  Note that
  // unlike Write, the argument 'written' is always set, and may be non-zero
  // on results other than SR_SUCCESS.  The remaining arguments have the
  // same semantics as Write.
  StreamResult WriteAll(ArrayView<const uint8_t> data,
                        size_t& written,
                        int& error);

 protected:
  StreamInterface();

  // Utility function for derived classes.
  void FireEvent(int stream_events, int err) RTC_RUN_ON(&callback_sequence_) {
    if (callback_) {
      callback_(stream_events, err);
    }
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
    // TODO(tommi): This is for backwards compatibility only while `SignalEvent`
    // is being replaced by `SetEventCallback`.
    SignalEvent(this, stream_events, err);
#pragma clang diagnostic pop
  }

  RTC_NO_UNIQUE_ADDRESS SequenceChecker callback_sequence_{
      webrtc::SequenceChecker::kDetached};

 private:
  absl::AnyInvocable<void(int, int)> callback_
      RTC_GUARDED_BY(&callback_sequence_) = nullptr;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::SE_CLOSE;
using ::webrtc::SE_OPEN;
using ::webrtc::SE_READ;
using ::webrtc::SE_WRITE;
using ::webrtc::SR_BLOCK;
using ::webrtc::SR_EOS;
using ::webrtc::SR_ERROR;
using ::webrtc::SR_SUCCESS;
using ::webrtc::SS_CLOSED;
using ::webrtc::SS_OPEN;
using ::webrtc::SS_OPENING;
using ::webrtc::StreamEvent;
using ::webrtc::StreamInterface;
using ::webrtc::StreamResult;
using ::webrtc::StreamState;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_STREAM_H_
