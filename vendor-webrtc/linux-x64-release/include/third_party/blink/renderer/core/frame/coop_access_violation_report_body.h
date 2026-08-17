// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_COOP_ACCESS_VIOLATION_REPORT_BODY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_COOP_ACCESS_VIOLATION_REPORT_BODY_H_

#include "services/network/public/mojom/cross_origin_opener_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/location_report_body.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"

namespace blink {

class CORE_EXPORT CoopAccessViolationReportBody final
    : public LocationReportBody {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CoopAccessViolationReportBody(
      std::unique_ptr<SourceLocation> source_location,
      network::mojom::blink::CoopAccessReportType type,
      const String& property,
      const String& reported_url);
  ~CoopAccessViolationReportBody() final = default;
  String type() const;
  String openeeURL() const;
  String openerURL() const;
  String otherDocumentURL() const;
  const String& property() const { return property_; }
  void BuildJSONValue(V8ObjectBuilder& builder) const final;

 private:
  network::mojom::blink::CoopAccessReportType type_;
  const String property_;
  const String reported_url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_COOP_ACCESS_VIOLATION_REPORT_BODY_H_
