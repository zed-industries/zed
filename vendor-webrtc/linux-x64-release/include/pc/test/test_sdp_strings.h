/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contain SDP strings used for testing.

#ifndef PC_TEST_TEST_SDP_STRINGS_H_
#define PC_TEST_TEST_SDP_STRINGS_H_

namespace webrtc {

// SDP offer string from a Nightly FireFox build.
static const char kFireFoxSdpOffer[] =
    "v=0\r\n"
    "o=Mozilla-SIPUA 23551 0 IN IP4 0.0.0.0\r\n"
    "s=SIP Call\r\n"
    "t=0 0\r\n"
    "a=ice-ufrag:e5785931\r\n"
    "a=ice-pwd:36fb7878390db89481c1d46daa4278d8\r\n"
    "a=fingerprint:sha-256 A7:24:72:CA:6E:02:55:39:BA:66:DF:6E:CC:4C:D8:B0:1A:"
    "BF:1A:56:65:7D:F4:03:AD:7E:77:43:2A:29:EC:93\r\n"
    "m=audio 36993 RTP/SAVPF 109 0 8 101\r\n"
    "c=IN IP4 74.95.2.170\r\n"
    "a=rtpmap:109 opus/48000/2\r\n"
    "a=ptime:20\r\n"
    "a=rtcp-mux\r\n"
    "a=rtpmap:0 PCMU/8000\r\n"
    "a=rtpmap:8 PCMA/8000\r\n"
    "a=rtpmap:101 telephone-event/8000\r\n"
    "a=fmtp:101 0-15\r\n"
    "a=sendrecv\r\n"
    "a=candidate:0 1 UDP 2112946431 172.16.191.1 61725 typ host\r\n"
    "a=candidate:2 1 UDP 2112487679 172.16.131.1 58798 typ host\r\n"
    "a=candidate:4 1 UDP 2113667327 10.0.254.2 58122 typ host\r\n"
    "a=candidate:5 1 UDP 1694302207 74.95.2.170 36993 typ srflx raddr "
    "10.0.254.2 rport 58122\r\n"
    "a=candidate:0 2 UDP 2112946430 172.16.191.1 55025 typ host\r\n"
    "a=candidate:2 2 UDP 2112487678 172.16.131.1 63576 typ host\r\n"
    "a=candidate:4 2 UDP 2113667326 10.0.254.2 50962 typ host\r\n"
    "a=candidate:5 2 UDP 1694302206 74.95.2.170 41028 typ srflx raddr"
    " 10.0.254.2 rport 50962\r\n"
    "m=video 38826 RTP/SAVPF 120\r\n"
    "c=IN IP4 74.95.2.170\r\n"
    "a=rtcp-mux\r\n"
    "a=rtpmap:120 VP8/90000\r\n"
    "a=sendrecv\r\n"
    "a=candidate:0 1 UDP 2112946431 172.16.191.1 62017 typ host\r\n"
    "a=candidate:2 1 UDP 2112487679 172.16.131.1 59741 typ host\r\n"
    "a=candidate:4 1 UDP 2113667327 10.0.254.2 62652 typ host\r\n"
    "a=candidate:5 1 UDP 1694302207 74.95.2.170 38826 typ srflx raddr"
    " 10.0.254.2 rport 62652\r\n"
    "a=candidate:0 2 UDP 2112946430 172.16.191.1 63440 typ host\r\n"
    "a=candidate:2 2 UDP 2112487678 172.16.131.1 51847 typ host\r\n"
    "a=candidate:4 2 UDP 2113667326 10.0.254.2 58890 typ host\r\n"
    "a=candidate:5 2 UDP 1694302206 74.95.2.170 33611 typ srflx raddr"
    " 10.0.254.2 rport 58890\r\n"
#ifdef WEBRTC_HAVE_SCTP
    "m=application 45536 DTLS/SCTP 5000\r\n"
    "c=IN IP4 74.95.2.170\r\n"
    "a=fmtp:5000 protocol=webrtc-datachannel;streams=16\r\n"
    "a=sendrecv\r\n"
    "a=candidate:0 1 UDP 2112946431 172.16.191.1 60248 typ host\r\n"
    "a=candidate:2 1 UDP 2112487679 172.16.131.1 55925 typ host\r\n"
    "a=candidate:4 1 UDP 2113667327 10.0.254.2 65268 typ host\r\n"
    "a=candidate:5 1 UDP 1694302207 74.95.2.170 45536 typ srflx raddr"
    " 10.0.254.2 rport 65268\r\n"
    "a=candidate:0 2 UDP 2112946430 172.16.191.1 49162 typ host\r\n"
    "a=candidate:2 2 UDP 2112487678 172.16.131.1 59635 typ host\r\n"
    "a=candidate:4 2 UDP 2113667326 10.0.254.2 61232 typ host\r\n"
    "a=candidate:5 2 UDP 1694302206 74.95.2.170 45468 typ srflx raddr"
    " 10.0.254.2 rport 61232\r\n"
#endif
    ;  // NOLINT(whitespace/semicolon)

// Audio SDP with a limited set of audio codecs.
static const char kAudioSdpPlanB[] =
    "v=0\r\n"
    "o=- 7859371131 2 IN IP4 192.168.30.208\r\n"
    "s=-\r\n"
    "c=IN IP4 192.168.30.208\r\n"
    "t=0 0\r\n"
    "m=audio 16000 RTP/SAVPF 0 8 126\r\n"
    "a=rtpmap:0 PCMU/8000\r\n"
    "a=rtpmap:8 PCMA/8000\r\n"
    "a=rtpmap:126 telephone-event/8000\r\n"
    "a=sendrecv\r\n"
    "a=rtcp:16000 IN IP4 192.168.30.208\r\n"
    "a=rtcp-mux\r\n"
    "a=crypto:0 AES_CM_128_HMAC_SHA1_80 "
    "inline:tvKIFjbMQ7W0/C2RzhwN0oQglj/7GJg+frdsNRxt\r\n"
    "a=ice-ufrag:AI2sRT3r\r\n"
    "a=ice-pwd:lByS9z2RSQlSE9XurlvjYmEm\r\n"
    "a=ssrc:4227871655 cname:GeAAgb6XCPNLVMX5\r\n"
    "a=ssrc:4227871655 msid:1NFAV3iD08ioO2339rQS9pfOI9mDf6GeG9F4 a0\r\n"
    "a=ssrc:4227871655 mslabel:1NFAV3iD08ioO2339rQS9pfOI9mDf6GeG9F4\r\n"
    "a=ssrc:4227871655 label:1NFAV3iD08ioO2339rQS9pfOI9mDf6GeG9F4a0\r\n"
    "a=mid:audio\r\n";
// Same string as above but with the MID changed to the Unified Plan default.
// This is needed so that this SDP can be used as an answer for a Unified Plan
// offer.
static const char kAudioSdpUnifiedPlan[] =
    "v=0\r\n"
    "o=- 7859371131 2 IN IP4 192.168.30.208\r\n"
    "s=-\r\n"
    "c=IN IP4 192.168.30.208\r\n"
    "t=0 0\r\n"
    "m=audio 16000 RTP/SAVPF 0 8 126\r\n"
    "a=rtpmap:0 PCMU/8000\r\n"
    "a=rtpmap:8 PCMA/8000\r\n"
    "a=rtpmap:126 telephone-event/8000\r\n"
    "a=sendrecv\r\n"
    "a=rtcp:16000 IN IP4 192.168.30.208\r\n"
    "a=rtcp-mux\r\n"
    "a=crypto:0 AES_CM_128_HMAC_SHA1_80 "
    "inline:tvKIFjbMQ7W0/C2RzhwN0oQglj/7GJg+frdsNRxt\r\n"
    "a=ice-ufrag:AI2sRT3r\r\n"
    "a=ice-pwd:lByS9z2RSQlSE9XurlvjYmEm\r\n"
    "a=ssrc:4227871655 cname:GeAAgb6XCPNLVMX5\r\n"
    "a=ssrc:4227871655 msid:1NFAV3iD08ioO2339rQS9pfOI9mDf6GeG9F4 a0\r\n"
    "a=ssrc:4227871655 mslabel:1NFAV3iD08ioO2339rQS9pfOI9mDf6GeG9F4\r\n"
    "a=ssrc:4227871655 label:1NFAV3iD08ioO2339rQS9pfOI9mDf6GeG9F4a0\r\n"
    "a=mid:0\r\n";

static const char kAudioSdpWithUnsupportedCodecsPlanB[] =
    "v=0\r\n"
    "o=- 6858750541 2 IN IP4 192.168.30.208\r\n"
    "s=-\r\n"
    "c=IN IP4 192.168.30.208\r\n"
    "t=0 0\r\n"
    "m=audio 16000 RTP/SAVPF 0 8 109 110 126\r\n"
    "a=rtpmap:0 PCMU/8000\r\n"
    "a=rtpmap:8 PCMA/8000\r\n"
    "a=rtpmap:109 WeirdCodec1/8000\r\n"
    "a=rtpmap:110 WeirdCodec2/8000\r\n"
    "a=rtpmap:126 telephone-event/8000\r\n"
    "a=sendonly\r\n"
    "a=rtcp:16000 IN IP4 192.168.30.208\r\n"
    "a=rtcp-mux\r\n"
    "a=crypto:0 AES_CM_128_HMAC_SHA1_80 "
    "inline:tvKIFjbMQ7W0/C2RzhwN0oQglj/7GJg+frdsNRxt\r\n"
    "a=ice-ufrag:AI2sRT3r\r\n"
    "a=ice-pwd:lByS9z2RSQlSE9XurlvjYmEm\r\n"
    "a=ssrc:4227871655 cname:TsmD02HRfhkJBm4m\r\n"
    "a=ssrc:4227871655 msid:7nU0TApbB-n4dfPlCplWT9QTEsbBDS1IlpW3 a0\r\n"
    "a=ssrc:4227871655 mslabel:7nU0TApbB-n4dfPlCplWT9QTEsbBDS1IlpW3\r\n"
    "a=ssrc:4227871655 label:7nU0TApbB-n4dfPlCplWT9QTEsbBDS1IlpW3a0\r\n"
    "a=mid:audio\r\n";
// Same string as above but with the MID changed to the Unified Plan default.
// This is needed so that this SDP can be used as an answer for a Unified Plan
// offer.
static const char kAudioSdpWithUnsupportedCodecsUnifiedPlan[] =
    "v=0\r\n"
    "o=- 6858750541 2 IN IP4 192.168.30.208\r\n"
    "s=-\r\n"
    "c=IN IP4 192.168.30.208\r\n"
    "t=0 0\r\n"
    "m=audio 16000 RTP/SAVPF 0 8 109 110 126\r\n"
    "a=rtpmap:0 PCMU/8000\r\n"
    "a=rtpmap:8 PCMA/8000\r\n"
    "a=rtpmap:109 WeirdCodec1/8000\r\n"
    "a=rtpmap:110 WeirdCodec2/8000\r\n"
    "a=rtpmap:126 telephone-event/8000\r\n"
    "a=sendonly\r\n"
    "a=rtcp:16000 IN IP4 192.168.30.208\r\n"
    "a=rtcp-mux\r\n"
    "a=crypto:0 AES_CM_128_HMAC_SHA1_80 "
    "inline:tvKIFjbMQ7W0/C2RzhwN0oQglj/7GJg+frdsNRxt\r\n"
    "a=ice-ufrag:AI2sRT3r\r\n"
    "a=ice-pwd:lByS9z2RSQlSE9XurlvjYmEm\r\n"
    "a=ssrc:4227871655 cname:TsmD02HRfhkJBm4m\r\n"
    "a=ssrc:4227871655 msid:7nU0TApbB-n4dfPlCplWT9QTEsbBDS1IlpW3 a0\r\n"
    "a=ssrc:4227871655 mslabel:7nU0TApbB-n4dfPlCplWT9QTEsbBDS1IlpW3\r\n"
    "a=ssrc:4227871655 label:7nU0TApbB-n4dfPlCplWT9QTEsbBDS1IlpW3a0\r\n"
    "a=mid:0\r\n";

}  // namespace webrtc

#endif  // PC_TEST_TEST_SDP_STRINGS_H_
