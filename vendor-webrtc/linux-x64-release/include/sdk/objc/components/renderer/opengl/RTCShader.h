/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "base/RTCVideoFrame.h"

RTC_EXTERN const char RTC_CONSTANT_TYPE(RTCVertexShaderSource)[];

RTC_EXTERN GLuint RTC_OBJC_TYPE(RTCCreateShader)(GLenum type, const GLchar* source);
RTC_EXTERN GLuint RTC_OBJC_TYPE(RTCCreateProgram)(GLuint vertexShader, GLuint fragmentShader);
RTC_EXTERN GLuint
RTC_OBJC_TYPE(RTCCreateProgramFromFragmentSource)(const char fragmentShaderSource[]);
RTC_EXTERN BOOL RTC_OBJC_TYPE(RTCCreateVertexBuffer)(GLuint* vertexBuffer,
                                      GLuint* vertexArray);
RTC_EXTERN void RTC_OBJC_TYPE(RTCSetVertexData)(RTCVideoRotation rotation);
