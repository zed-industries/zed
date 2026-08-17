/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCRtpReceiver.h"
#import "RTCRtpSender.h"
#import "sdk/objc/base/RTCMacros.h"

@class RTC_OBJC_TYPE(RTCRtpCodecCapability);

NS_ASSUME_NONNULL_BEGIN

extern NSString *const RTC_CONSTANT_TYPE(RTCRtpTransceiverErrorDomain);

/** https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiverdirection */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCRtpTransceiverDirection)) {
  RTC_OBJC_TYPE(RTCRtpTransceiverDirectionSendRecv),
  RTC_OBJC_TYPE(RTCRtpTransceiverDirectionSendOnly),
  RTC_OBJC_TYPE(RTCRtpTransceiverDirectionRecvOnly),
  RTC_OBJC_TYPE(RTCRtpTransceiverDirectionInactive),
  RTC_OBJC_TYPE(RTCRtpTransceiverDirectionStopped)
};

/** Structure for initializing an RTCRtpTransceiver in a call to
 *  RTCPeerConnection.addTransceiver.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiverinit
 */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCRtpTransceiverInit) : NSObject

/** Direction of the RTCRtpTransceiver. See RTCRtpTransceiver.direction. */
@property(nonatomic) RTC_OBJC_TYPE(RTCRtpTransceiverDirection) direction;

/** The added RTCRtpTransceiver will be added to these streams. */
@property(nonatomic) NSArray<NSString *> *streamIds;

/** TODO(bugs.webrtc.org/7600): Not implemented. */
@property(nonatomic)
    NSArray<RTC_OBJC_TYPE(RTCRtpEncodingParameters) *> *sendEncodings;

@end

@class RTC_OBJC_TYPE(RTCRtpTransceiver);
@class RTC_OBJC_TYPE(RTCRtpCodecCapability);
@class RTC_OBJC_TYPE(RTCRtpHeaderExtensionCapability);

/** The RTCRtpTransceiver maps to the RTCRtpTransceiver defined by the
 *  WebRTC specification. A transceiver represents a combination of an
 * RTCRtpSender and an RTCRtpReceiver that share a common mid. As defined in
 * JSEP, an RTCRtpTransceiver is said to be associated with a media description
 * if its mid property is non-nil; otherwise, it is said to be disassociated.
 *  JSEP: https://tools.ietf.org/html/draft-ietf-rtcweb-jsep-24
 *
 *  Note that RTCRtpTransceivers are only supported when using
 *  RTCPeerConnection with Unified Plan SDP.
 *
 *  WebRTC specification for RTCRtpTransceiver, the JavaScript analog:
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver
 */
RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCRtpTransceiver)<NSObject>

    /** Media type of the transceiver. The sender and receiver will also have
     * this type.
     */
    @property(nonatomic, readonly) RTC_OBJC_TYPE(RTCRtpMediaType) mediaType;

/** The mid attribute is the mid negotiated and present in the local and
 *  remote descriptions. Before negotiation is complete, the mid value may be
 *  nil. After rollbacks, the value may change from a non-nil value to nil.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-mid
 */
@property(nonatomic, readonly) NSString *mid;

/** The sender attribute exposes the RTCRtpSender corresponding to the RTP
 *  media that may be sent with the transceiver's mid. The sender is always
 *  present, regardless of the direction of media.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-sender
 */
@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCRtpSender) * sender;

/** The receiver attribute exposes the RTCRtpReceiver corresponding to the RTP
 *  media that may be received with the transceiver's mid. The receiver is
 *  always present, regardless of the direction of media.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-receiver
 */
@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCRtpReceiver) * receiver;

/** The isStopped attribute indicates that the sender of this transceiver will
 *  no longer send, and that the receiver will no longer receive. It is true if
 *  either stop has been called or if setting the local or remote description
 *  has caused the RTCRtpTransceiver to be stopped.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-stopped
 */
@property(nonatomic, readonly) BOOL isStopped;

/** The direction attribute indicates the preferred direction of this
 *  transceiver, which will be used in calls to createOffer and createAnswer.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-direction
 */
@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCRtpTransceiverDirection) direction;

@property(nonatomic, copy) NSArray<RTC_OBJC_TYPE(RTCRtpCodecCapability) *> *codecPreferences;

/** It will contain all the RTP header extensions that are supported.
 *  The direction attribute for all extensions that are mandatory to use MUST be
 * initialized to an appropriate value other than
 * RTCRtpTransceiverDirectionStopped. The direction attribute for extensions
 * that will not be offered by default in an initial offer MUST be initialized
 * to RTCRtpTransceiverDirectionStopped.
 */
@property(nonatomic, readonly, copy)
    NSArray<RTC_OBJC_TYPE(RTCRtpHeaderExtensionCapability) *>
        *headerExtensionsToNegotiate;
@property(nonatomic, readonly, copy)
    NSArray<RTC_OBJC_TYPE(RTCRtpHeaderExtensionCapability) *>
        *negotiatedHeaderExtensions;

/** The currentDirection attribute indicates the current direction negotiated
 *  for this transceiver. If this transceiver has never been represented in an
 *  offer/answer exchange, or if the transceiver is stopped, the value is not
 *  present and this method returns NO.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-currentdirection
 */
- (BOOL)currentDirection:(RTC_OBJC_TYPE(RTCRtpTransceiverDirection) *)currentDirectionOut;

/** The stop method irreversibly stops the RTCRtpTransceiver. The sender of
 *  this transceiver will no longer send, the receiver will no longer receive.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-stop
 */
- (void)stopInternal;

/** The setCodecPreferences method overrides the default codec preferences used
 * by WebRTC for this transceiver.
 * https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-setcodecpreferences
 */
- (BOOL)setCodecPreferences:
            (NSArray<RTC_OBJC_TYPE(RTCRtpCodecCapability) *> *_Nullable)codecs
                      error:(NSError **_Nullable)error;

/** Deprecated version of [RTCRtpTransceiver setCodecPreferences:error:] */
- (void)setCodecPreferences:
    (NSArray<RTC_OBJC_TYPE(RTCRtpCodecCapability) *> *_Nullable)codecs
    RTC_OBJC_DEPRECATED("Use setCodecPreferences:error: instead.");

/** The setHeaderExtensionsToNegotiate method overrides the default header
 * extensions used by WebRTC for this transceiver.
 *  https://w3c.github.io/webrtc-extensions/#ref-for-dom-rtcrtptransceiver-setheaderextensionstonegotiate
 */
- (BOOL)setHeaderExtensionsToNegotiate:
            (NSArray<RTC_OBJC_TYPE(RTCRtpHeaderExtensionCapability) *> *)
                extensions
                                 error:(NSError **)error;

/** An update of directionality does not take effect immediately. Instead,
 *  future calls to createOffer and createAnswer mark the corresponding media
 *  descriptions as sendrecv, sendonly, recvonly, or inactive.
 *  https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-direction
 */
- (void)setDirection:(RTC_OBJC_TYPE(RTCRtpTransceiverDirection))direction error:(NSError **)error;

@end

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCRtpTransceiver) : NSObject <RTC_OBJC_TYPE(RTCRtpTransceiver)>

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
