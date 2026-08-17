// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEPRECATION_DEPRECATION_REPORT_BODY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEPRECATION_DEPRECATION_REPORT_BODY_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/location_report_body.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"

namespace blink {

class CORE_EXPORT DeprecationReportBody : public LocationReportBody {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DeprecationReportBody(const String& id,
                        std::optional<base::Time> anticipated_removal,
                        const String& message)
      : id_(id), message_(message), anticipated_removal_(anticipated_removal) {}

  ~DeprecationReportBody() override = default;

  const String& id() const { return id_; }
  const String& message() const { return message_; }
  ScriptObject anticipatedRemoval(ScriptState* script_state) const;
  std::optional<base::Time> AnticipatedRemoval() const;

  void BuildJSONValue(V8ObjectBuilder& builder) const override;

 private:
  const String id_;
  const String message_;
  const std::optional<base::Time> anticipated_removal_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEPRECATION_DEPRECATION_REPORT_BODY_H_
