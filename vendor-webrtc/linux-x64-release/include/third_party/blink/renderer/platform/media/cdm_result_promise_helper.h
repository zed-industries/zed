// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_RESULT_PROMISE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_RESULT_PROMISE_HELPER_H_

#include <string>

#include "media/base/cdm_key_information.h"
#include "media/base/cdm_promise.h"
#include "third_party/blink/public/platform/web_content_decryption_module_result.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// A superset of media::ContentDecryptionModule::Exception for UMA reporting.
// These values should never be changed as it will affect existing reporting,
// and must match the values for CdmPromiseResult in
// tools/metrics/histograms/enums.xml. Deprecated values should never be reused.
enum CdmResultForUMA {
  SUCCESS = 0,
  NOT_SUPPORTED_ERROR = 1,
  INVALID_STATE_ERROR = 2,
  TYPE_ERROR = 3,
  QUOTA_EXCEEDED_ERROR = 4,
  // UNKNOWN_ERROR = 5,  // Deprecated.
  // CLIENT_ERROR = 6,   // Deprecated.
  // OUTPUT_ERROR = 7,   // Deprecated.
  SESSION_NOT_FOUND = 8,
  SESSION_ALREADY_EXISTS = 9,
  NUM_RESULT_CODES  // Must be last.
};

PLATFORM_EXPORT CdmResultForUMA
ConvertCdmExceptionToResultForUMA(media::CdmPromise::Exception exception_code);

PLATFORM_EXPORT WebContentDecryptionModuleException
ConvertCdmException(media::CdmPromise::Exception exception_code);

PLATFORM_EXPORT WebEncryptedMediaKeyInformation::KeyStatus ConvertCdmKeyStatus(
    media::CdmKeyInformation::KeyStatus key_status);

PLATFORM_EXPORT void ReportCdmResultUMA(const std::string& uma_name,
                                        uint32_t system_code,
                                        CdmResultForUMA result);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_RESULT_PROMISE_HELPER_H_
