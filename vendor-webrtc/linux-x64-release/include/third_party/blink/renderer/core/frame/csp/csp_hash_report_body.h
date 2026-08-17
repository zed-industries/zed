// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_HASH_REPORT_BODY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_HASH_REPORT_BODY_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/report_body.h"

namespace blink {

class CORE_EXPORT CSPHashReportBody : public ReportBody {
 public:
  CSPHashReportBody(const String& url,
                    const String& hash,
                    const String& type,
                    const String& destination)
      : subresource_url_(url),
        hash_(hash),
        type_(type),
        destination_(destination) {}
  ~CSPHashReportBody() override = default;
  const String& subresourceURL() const { return subresource_url_; }
  const String& hash() const { return hash_; }
  const String& type() const { return type_; }
  const String& destination() const { return destination_; }
  void BuildJSONValue(V8ObjectBuilder& builder) const override;

 private:
  const String subresource_url_;
  const String hash_;
  const String type_;
  const String destination_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_HASH_REPORT_BODY_H_
