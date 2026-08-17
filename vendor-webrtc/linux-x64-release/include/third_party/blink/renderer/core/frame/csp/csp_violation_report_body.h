// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_VIOLATION_REPORT_BODY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_VIOLATION_REPORT_BODY_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_security_policy_violation_event_disposition.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_security_policy_violation_event_init.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/location_report_body.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"

namespace blink {

class CORE_EXPORT CSPViolationReportBody : public LocationReportBody {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit CSPViolationReportBody(
      const SecurityPolicyViolationEventInit& violation_data)
      : LocationReportBody(violation_data.sourceFile(),
                           violation_data.lineNumber(),
                           violation_data.columnNumber()),
        document_url_(violation_data.documentURI()),
        referrer_(violation_data.referrer()),
        blocked_url_(violation_data.blockedURI()),
        effective_directive_(violation_data.effectiveDirective()),
        original_policy_(violation_data.originalPolicy()),
        sample_(violation_data.sample()),
        disposition_(violation_data.disposition()),
        status_code_(violation_data.statusCode()) {}

  ~CSPViolationReportBody() override = default;

  const String& documentURL() const { return document_url_; }
  const String& referrer() const { return referrer_; }
  const String& blockedURL() const { return blocked_url_; }
  const String& effectiveDirective() const { return effective_directive_; }
  const String& originalPolicy() const { return original_policy_; }
  const String& sample() const { return sample_; }
  const V8SecurityPolicyViolationEventDisposition& disposition() const {
    return disposition_;
  }
  uint16_t statusCode() const { return status_code_; }

  void BuildJSONValue(V8ObjectBuilder& builder) const override;

 private:
  const String document_url_;
  const String referrer_;
  const String blocked_url_;
  const String effective_directive_;
  const String original_policy_;
  const String sample_;
  const V8SecurityPolicyViolationEventDisposition disposition_;
  const uint16_t status_code_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_VIOLATION_REPORT_BODY_H_
