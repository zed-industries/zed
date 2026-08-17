// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_RESULT_PROMISE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_RESULT_PROMISE_H_

#include <stdint.h>

#include "base/metrics/histogram_functions.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/web_content_decryption_module_result.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/platform/media/cdm_result_promise_helper.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

const char kTimeToResolveUmaPrefix[] = "TimeTo.";
const char kTimeToRejectUmaPrefix[] = "TimeTo.Reject.";

// Used to convert a WebContentDecryptionModuleResult into a CdmPromiseTemplate
// so that it can be passed through Chromium. When resolve(T) is called, the
// appropriate complete...() method on WebContentDecryptionModuleResult will be
// invoked. If reject() is called instead,
// WebContentDecryptionModuleResult::completeWithError() is called.
// If constructed with a |uma_name|, CdmResultPromise will report the promise
// result (success or rejection code) to UMA.
template <typename... T>
class PLATFORM_EXPORT CdmResultPromise
    : public media::CdmPromiseTemplate<T...> {
 public:
  CdmResultPromise(const WebContentDecryptionModuleResult& result,
                   const std::string& key_system_uma_prefix,
                   const std::string& uma_name);
  CdmResultPromise(const CdmResultPromise&) = delete;
  CdmResultPromise& operator=(const CdmResultPromise&) = delete;
  ~CdmResultPromise() override;

  // media::CdmPromiseTemplate<T> implementation.
  void resolve(const T&... result) override;
  void reject(media::CdmPromise::Exception exception_code,
              uint32_t system_code,
              const std::string& error_message) override;

 private:
  using media::CdmPromiseTemplate<T...>::IsPromiseSettled;
  using media::CdmPromiseTemplate<T...>::MarkPromiseSettled;
  using media::CdmPromiseTemplate<T...>::RejectPromiseOnDestruction;

  WebContentDecryptionModuleResult web_cdm_result_;

  // UMA prefix and name to report result and time to.
  std::string key_system_uma_prefix_;
  std::string uma_name_;

  // Time when |this| is created.
  base::TimeTicks creation_time_;
};

template <typename... T>
CdmResultPromise<T...>::CdmResultPromise(
    const WebContentDecryptionModuleResult& result,
    const std::string& key_system_uma_prefix,
    const std::string& uma_name)
    : web_cdm_result_(result),
      key_system_uma_prefix_(key_system_uma_prefix),
      uma_name_(uma_name),
      creation_time_(base::TimeTicks::Now()) {
  DCHECK(!key_system_uma_prefix_.empty());
  DCHECK(!uma_name_.empty());
}

template <typename... T>
CdmResultPromise<T...>::~CdmResultPromise() {
  if (!IsPromiseSettled())
    RejectPromiseOnDestruction();
}

// "inline" is needed to prevent multiple definition error.

template <>
inline void CdmResultPromise<>::resolve() {
  MarkPromiseSettled();
  ReportCdmResultUMA(key_system_uma_prefix_ + uma_name_, 0, SUCCESS);

  base::UmaHistogramTimes(
      key_system_uma_prefix_ + kTimeToResolveUmaPrefix + uma_name_,
      base::TimeTicks::Now() - creation_time_);

  web_cdm_result_.Complete();
}

template <>
inline void CdmResultPromise<media::CdmKeyInformation::KeyStatus>::resolve(
    const media::CdmKeyInformation::KeyStatus& key_status) {
  MarkPromiseSettled();
  ReportCdmResultUMA(key_system_uma_prefix_ + uma_name_, 0, SUCCESS);

  base::UmaHistogramTimes(
      key_system_uma_prefix_ + kTimeToResolveUmaPrefix + uma_name_,
      base::TimeTicks::Now() - creation_time_);

  web_cdm_result_.CompleteWithKeyStatus(ConvertCdmKeyStatus(key_status));
}

template <typename... T>
void CdmResultPromise<T...>::reject(media::CdmPromise::Exception exception_code,
                                    uint32_t system_code,
                                    const std::string& error_message) {
  MarkPromiseSettled();
  ReportCdmResultUMA(key_system_uma_prefix_ + uma_name_, system_code,
                     ConvertCdmExceptionToResultForUMA(exception_code));

  base::UmaHistogramTimes(
      key_system_uma_prefix_ + kTimeToRejectUmaPrefix + uma_name_,
      base::TimeTicks::Now() - creation_time_);

  web_cdm_result_.CompleteWithError(ConvertCdmException(exception_code),
                                    system_code,
                                    WebString::FromUTF8(error_message));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_RESULT_PROMISE_H_
