/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_OS_SPECIFIC_INLINE_H_
#define MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_OS_SPECIFIC_INLINE_H_

#include <math.h>

#include "rtc_base/system/arch.h"

#if defined(WEBRTC_POSIX)
#define WebRtcIsac_lrint lrint
#elif (defined(WEBRTC_ARCH_X86) && defined(WIN32))
static __inline long int WebRtcIsac_lrint(double x_dbl) {
  long int x_int;

  __asm {
    fld x_dbl
    fistp x_int
  }
  ;

  return x_int;
}
#else  // Do a slow but correct implementation of lrint

static __inline long int WebRtcIsac_lrint(double x_dbl) {
  long int x_int;
  x_int = (long int)floor(x_dbl + 0.499999999999);
  return x_int;
}

#endif

#endif  // MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_OS_SPECIFIC_INLINE_H_
