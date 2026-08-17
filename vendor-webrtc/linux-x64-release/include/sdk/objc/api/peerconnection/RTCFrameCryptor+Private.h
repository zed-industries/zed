/*
 * Copyright 2022 LiveKit
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#import "RTCFrameCryptor.h"

#include <string>
#include "api/crypto/frame_crypto_transformer.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCFrameCryptor)
()

    @end

namespace webrtc {

class RTCFrameCryptorDelegateAdapter : public FrameCryptorTransformerObserver {
 public:
  RTCFrameCryptorDelegateAdapter(RTC_OBJC_TYPE(RTCFrameCryptor) * frameCryptor);
  ~RTCFrameCryptorDelegateAdapter() override;

  void OnFrameCryptionStateChanged(const std::string participant_id,
                                   FrameCryptionState state) override;

 private:
  __weak RTC_OBJC_TYPE(RTCFrameCryptor) * frame_cryptor_;
};

}  // namespace webrtc

NS_ASSUME_NONNULL_END
