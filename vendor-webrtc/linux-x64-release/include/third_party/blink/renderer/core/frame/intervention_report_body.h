// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_INTERVENTION_REPORT_BODY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_INTERVENTION_REPORT_BODY_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/location_report_body.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"

namespace blink {

class CORE_EXPORT InterventionReportBody : public LocationReportBody {
  DEFINE_WRAPPERTYPEINFO();

 public:
  InterventionReportBody(const String& id, const String& message)
      : id_(id), message_(message) {}

  ~InterventionReportBody() override = default;

  const String& id() const { return id_; }
  const String& message() const { return message_; }
  void BuildJSONValue(V8ObjectBuilder& builder) const override;

 private:
  const String id_;
  const String message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_INTERVENTION_REPORT_BODY_H_
