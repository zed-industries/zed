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

#import "RTCDesktopMediaList.h"

namespace webrtc {
    class ObjCDesktopMediaList;
    class MediaSource;
}

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE(RTCDesktopMediaList) ()

@property(nonatomic, readonly)std::shared_ptr<webrtc::ObjCDesktopMediaList> nativeMediaList;

-(void)mediaSourceAdded:(webrtc::MediaSource *) source;

-(void)mediaSourceRemoved:(webrtc::MediaSource *) source;

-(void)mediaSourceNameChanged:(webrtc::MediaSource *) source;

-(void)mediaSourceThumbnailChanged:(webrtc::MediaSource *) source;

@end

NS_ASSUME_NONNULL_END