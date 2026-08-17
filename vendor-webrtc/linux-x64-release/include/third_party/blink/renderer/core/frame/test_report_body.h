// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_TEST_REPORT_BODY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_TEST_REPORT_BODY_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/report_body.h"

namespace blink {

class CORE_EXPORT TestReportBody : public ReportBody {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TestReportBody(const String& message) : message_(message) {}

  ~TestReportBody() override = default;

  String message() const { return message_; }

  void BuildJSONValue(V8ObjectBuilder& builder) const override;

 private:
  const String message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_TEST_REPORT_BODY_H_
