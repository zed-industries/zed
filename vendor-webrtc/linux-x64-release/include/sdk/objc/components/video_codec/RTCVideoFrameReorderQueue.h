/*
 * Copyright (C) 2023 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#import "base/RTCVideoFrame.h"
#include <deque>
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

class RTCVideoFrameReorderQueue {
public:
    RTCVideoFrameReorderQueue() = default;

    struct RTC_OBJC_TYPE(RTCVideoFrameWithOrder) {
        RTC_OBJC_TYPE(RTCVideoFrameWithOrder)(RTC_OBJC_TYPE(RTCVideoFrame) * frame, uint64_t reorderSize)
            : frame((__bridge_retained void*)frame)
            , timeStamp(frame.timeStamp)
            , reorderSize(reorderSize)
        {
        }

        ~RTC_OBJC_TYPE(RTCVideoFrameWithOrder)()
        {
            if (frame)
                take();
        }

        RTC_OBJC_TYPE(RTCVideoFrame) * take()
        {
            auto* rtcFrame = (__bridge_transfer RTC_OBJC_TYPE(RTCVideoFrame) *)frame;
            frame = nullptr;
            return rtcFrame;
        }

        void* frame;
        uint64_t timeStamp;
        uint64_t reorderSize;
    };

    bool isEmpty();
    uint8_t reorderSize() const;
    void setReorderSize(uint8_t);
    void append(RTC_OBJC_TYPE(RTCVideoFrame) *, uint8_t);
    RTC_OBJC_TYPE(RTCVideoFrame) *takeIfAvailable();
    RTC_OBJC_TYPE(RTCVideoFrame) *takeIfAny();

private:
    std::deque<std::unique_ptr<RTC_OBJC_TYPE(RTCVideoFrameWithOrder)>> _reorderQueue;
    uint8_t _reorderSize { 0 };
    mutable webrtc::Mutex _reorderQueueLock;
};

}