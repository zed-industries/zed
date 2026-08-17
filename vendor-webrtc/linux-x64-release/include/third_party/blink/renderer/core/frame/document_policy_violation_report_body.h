// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOCUMENT_POLICY_VIOLATION_REPORT_BODY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOCUMENT_POLICY_VIOLATION_REPORT_BODY_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/location_report_body.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"

namespace blink {

class CORE_EXPORT DocumentPolicyViolationReportBody
    : public LocationReportBody {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DocumentPolicyViolationReportBody(
      const String& feature_id,
      const String& message,
      const String& disposition,
      // URL of the resource that violated the document policy.
      const String& resource_url);

  const String& featureId() const { return feature_id_; }
  const String& disposition() const { return disposition_; }
  const String& message() const { return message_; }

  void BuildJSONValue(V8ObjectBuilder& builder) const override;

  ~DocumentPolicyViolationReportBody() override = default;

  unsigned MatchId() const override;

 private:
  const String feature_id_;
  const String message_;
  const String disposition_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOCUMENT_POLICY_VIOLATION_REPORT_BODY_H_
