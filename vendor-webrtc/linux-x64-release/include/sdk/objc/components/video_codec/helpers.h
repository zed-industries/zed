/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 */

#ifndef SDK_OBJC_FRAMEWORK_CLASSES_VIDEOTOOLBOX_HELPERS_H_
#define SDK_OBJC_FRAMEWORK_CLASSES_VIDEOTOOLBOX_HELPERS_H_

#include <CoreFoundation/CoreFoundation.h>
#include <VideoToolbox/VideoToolbox.h>

#include <string>

// Convenience function for creating a dictionary.
inline CFDictionaryRef CreateCFTypeDictionary(CFTypeRef* keys,
                                              CFTypeRef* values,
                                              size_t size) {
  return CFDictionaryCreate(kCFAllocatorDefault, keys, values, size,
                            &kCFTypeDictionaryKeyCallBacks,
                            &kCFTypeDictionaryValueCallBacks);
}

// Copies characters from a CFStringRef into a std::string.
std::string CFStringToString(CFStringRef cf_string);

// Convenience function for setting a VT property.
void SetVTSessionProperty(VTSessionRef session, CFStringRef key, int32_t value);

// Convenience function for setting a VT property.
void SetVTSessionProperty(VTSessionRef session,
                          CFStringRef key,
                          uint32_t value);

// Convenience function for setting a VT property.
void SetVTSessionProperty(VTSessionRef session, CFStringRef key, bool value);

// Convenience function for setting a VT property.
void SetVTSessionProperty(VTSessionRef session,
                          CFStringRef key,
                          CFStringRef value);

#endif  // SDK_OBJC_FRAMEWORK_CLASSES_VIDEOTOOLBOX_HELPERS_H_
