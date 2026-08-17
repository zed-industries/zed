// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/report_body.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ScriptObject;

// The constants are implemented as static members of a class to have an unique
// address and not violate ODR.
struct CORE_EXPORT ReportType {
  static constexpr const char kCSPViolation[] = "csp-violation";
  static constexpr const char kCSPHash[] = "csp-hash";
  static constexpr const char kCoopAccessViolation[] = "coop-access-violation";
  static constexpr const char kDeprecation[] = "deprecation";
  static constexpr const char kDocumentPolicyViolation[] =
      "document-policy-violation";
  static constexpr const char kPermissionsPolicyViolation[] =
      "permissions-policy-violation";
  static constexpr const char kPotentialPermissionsPolicyViolation[] =
      "potential-permissions-policy-violation";
  static constexpr const char kIntervention[] = "intervention";
};

class CORE_EXPORT Report : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Report(const String& type, const String& url, ReportBody* body)
      : type_(type), url_(url), body_(body) {
    DCHECK(!type.IsNull());
  }

  ~Report() override = default;

  String type() const { return type_; }
  String url() const { return url_; }
  ReportBody* body() const { return body_.Get(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(body_);
    ScriptWrappable::Trace(visitor);
  }

  ScriptObject toJSON(ScriptState* script_state) const;

  // Provides a hash-like value for identifying reports with same content.
  // Collision of match id is possible.
  unsigned MatchId() const;

  // Determines whether this report is allowed to be sent to observers or the
  // reporting endpoints. This should return false if the report should not be
  // sent, for example, if the body of the report would reveal private
  // information, such as extension URLs.
  bool ShouldSendReport() const;

 private:
  const String type_;
  const String url_;
  Member<ReportBody> body_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORT_H_
