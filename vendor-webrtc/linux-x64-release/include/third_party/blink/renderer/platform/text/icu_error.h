// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_ICU_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_ICU_ERROR_H_

#include <unicode/utypes.h>
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// ICUError provides the unified way to handle ICU errors in Blink.
class PLATFORM_EXPORT ICUError {
  STACK_ALLOCATED();

 public:
  ~ICUError() { CrashIfCritical(); }

  UErrorCode* operator&() { return &error_; }
  operator UErrorCode() const { return error_; }
  operator UErrorCode&() { return error_; }

  void operator=(UErrorCode error) { error_ = error; }

  // Crash the renderer in the appropriate way if critical failure occurred.
  void CrashIfCritical();

 private:
  UErrorCode error_ = U_ZERO_ERROR;

  void HandleFailure();
};

inline void ICUError::CrashIfCritical() {
  if (U_FAILURE(error_))
    HandleFailure();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_ICU_ERROR_H_
