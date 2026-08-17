/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_ACM2_CALL_STATISTICS_H_
#define MODULES_AUDIO_CODING_ACM2_CALL_STATISTICS_H_

#include "api/audio/audio_frame.h"
#include "modules/audio_coding/include/audio_coding_module_typedefs.h"

//
// This class is for book keeping of calls to ACM. It is not useful to log API
// calls which are supposed to be called every 10ms, e.g. PlayoutData10Ms(),
// however, it is useful to know the number of such calls in a given time
// interval. The current implementation covers calls to PlayoutData10Ms() with
// detailed accounting of the decoded speech type.
//
// Thread Safety
// =============
// Please note that this class in not thread safe. The class must be protected
// if different APIs are called from different threads.
//

namespace webrtc {

namespace acm2 {

class CallStatistics {
 public:
  CallStatistics() {}
  ~CallStatistics() {}

  // Call this method to indicate that NetEq engaged in decoding. `speech_type`
  // is the audio-type according to NetEq, and `muted` indicates if the decoded
  // frame was produced in muted state.
  void DecodedByNetEq(AudioFrame::SpeechType speech_type, bool muted);

  // Call this method to indicate that a decoding call resulted in generating
  // silence, i.e. call to NetEq is bypassed and the output audio is zero.
  void DecodedBySilenceGenerator();

  // Get statistics for decoding. The statistics include the number of calls to
  // NetEq and silence generator, as well as the type of speech pulled of off
  // NetEq, c.f. declaration of AudioDecodingCallStats for detailed description.
  const AudioDecodingCallStats& GetDecodingStatistics() const;

 private:
  // Reset the decoding statistics.
  void ResetDecodingStatistics();

  AudioDecodingCallStats decoding_stat_;
};

}  // namespace acm2

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_ACM2_CALL_STATISTICS_H_
