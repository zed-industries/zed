// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_DTMF_SENDER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_DTMF_SENDER_HANDLER_H_

#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/webrtc/api/dtmf_sender_interface.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace webrtc {
class DtmfSenderInterface;
}

namespace blink {

// RtcDtmfSenderHandler is a delegate for the RTC DTMF Sender API messages
// going between WebKit and native DTMF Sender in libjingle.
// Instances of this class are owned by WebKit.
// WebKit call all of these methods on the main render thread.
// Callbacks to the webrtc::DtmfSenderObserverInterface implementation also
// occur on the main render thread.
class PLATFORM_EXPORT RtcDtmfSenderHandler final {
 public:
  class PLATFORM_EXPORT Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() = default;
    virtual void DidPlayTone(const String& tone) = 0;

    void Trace(Visitor* visitor) const override {}
  };

  RtcDtmfSenderHandler(scoped_refptr<base::SingleThreadTaskRunner> main_thread,
                       webrtc::DtmfSenderInterface* dtmf_sender);
  RtcDtmfSenderHandler(const RtcDtmfSenderHandler&) = delete;
  RtcDtmfSenderHandler& operator=(const RtcDtmfSenderHandler&) = delete;
  ~RtcDtmfSenderHandler();

  void SetClient(RtcDtmfSenderHandler::Client* client);
  String CurrentToneBuffer();
  bool CanInsertDTMF();
  bool InsertDTMF(const String& tones, int duration, int inter_tone_gap);

  void OnToneChange(const String& tone);

 private:
  scoped_refptr<webrtc::DtmfSenderInterface> dtmf_sender_;
  WeakPersistent<RtcDtmfSenderHandler::Client> webkit_client_;
  class Observer;
  scoped_refptr<Observer> observer_;

  SEQUENCE_CHECKER(sequence_checker_);

  // |weak_factory_| must be the last member.
  base::WeakPtrFactory<RtcDtmfSenderHandler> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_DTMF_SENDER_HANDLER_H_
