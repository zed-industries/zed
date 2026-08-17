/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCDispatcherQueueType)) {
  // Main dispatcher queue.
  RTC_OBJC_TYPE(RTCDispatcherTypeMain),
  // Used for starting/stopping AVCaptureSession, and assigning
  // capture session to AVCaptureVideoPreviewLayer.
  RTC_OBJC_TYPE(RTCDispatcherTypeCaptureSession),
  // Used for operations on AVAudioSession.
  RTC_OBJC_TYPE(RTCDispatcherTypeAudioSession),
  // Used for operations on NWPathMonitor.
  RTC_OBJC_TYPE(RTCDispatcherTypeNetworkMonitor),
};

/** Dispatcher that asynchronously dispatches blocks to a specific
 *  shared dispatch queue.
 */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCDispatcher) : NSObject

- (instancetype)init NS_UNAVAILABLE;

/** Dispatch the block asynchronously on the queue for dispatchType.
 *  @param dispatchType The queue type to dispatch on.
 *  @param block The block to dispatch asynchronously.
 */
+ (void)dispatchAsyncOnType:(RTC_OBJC_TYPE(RTCDispatcherQueueType))dispatchType block:(dispatch_block_t)block;

/** Returns YES if run on queue for the dispatchType otherwise NO.
 *  Useful for asserting that a method is run on a correct queue.
 */
+ (BOOL)isOnQueueForType:(RTC_OBJC_TYPE(RTCDispatcherQueueType))dispatchType;

@end
