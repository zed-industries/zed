// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_AUDITS_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_AUDITS_AGENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/inspector/inspected_frames.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_contrast.h"
#include "third_party/blink/renderer/core/inspector/protocol/audits.h"

namespace blink {

namespace protocol::Audits {
class InspectorIssue;
}  // namespace protocol::Audits

class InspectorIssueStorage;
class WebAutofillClient;

class CORE_EXPORT InspectorAuditsAgent final
    : public InspectorBaseAgent<protocol::Audits::Metainfo> {
 public:
  explicit InspectorAuditsAgent(InspectorNetworkAgent*,
                                InspectorIssueStorage*,
                                InspectedFrames*,
                                WebAutofillClient*);
  InspectorAuditsAgent(const InspectorAuditsAgent&) = delete;
  InspectorAuditsAgent& operator=(const InspectorAuditsAgent&) = delete;
  ~InspectorAuditsAgent() override;

  void Trace(Visitor*) const override;

  void InspectorIssueAdded(protocol::Audits::InspectorIssue*);

  // Protocol methods.
  protocol::Response enable() override;
  protocol::Response disable() override;
  protocol::Response checkContrast(std::optional<bool> report_aaa) override;
  protocol::Response checkFormsIssues(
      std::unique_ptr<protocol::Array<protocol::Audits::GenericIssueDetails>>*
          out_formIssues) override;

  void Restore() override;

  protocol::Response getEncodedResponse(
      const String& request_id,
      const String& encoding,
      std::optional<double> quality,
      std::optional<bool> size_only,
      std::optional<protocol::Binary>* out_body,
      int* out_original_size,
      int* out_encoded_size) override;

 private:
  void InnerEnable();
  void CheckContrastForDocument(Document* document, bool report_aaa);

  InspectorIssueStorage* const inspector_issue_storage_;
  InspectorAgentState::Boolean enabled_;
  Member<InspectorNetworkAgent> network_agent_;
  Member<InspectedFrames> inspected_frames_;
  WebAutofillClient* const web_autofill_client_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_AUDITS_AGENT_H_
