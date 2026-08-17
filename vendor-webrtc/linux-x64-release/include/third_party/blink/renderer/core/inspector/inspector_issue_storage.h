// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_STORAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_STORAGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class AuditsIssue;
class CoreProbeSink;
class ExecutionContext;

namespace protocol {
namespace Audits {
class InspectorIssue;
}  // namespace Audits
}  // namespace protocol

class CORE_EXPORT InspectorIssueStorage {
 public:
  InspectorIssueStorage();
  InspectorIssueStorage(const InspectorIssueStorage&) = delete;
  InspectorIssueStorage& operator=(const InspectorIssueStorage&) = delete;

  void AddInspectorIssue(ExecutionContext*, AuditsIssue);
  void AddInspectorIssue(CoreProbeSink*, AuditsIssue);

  void Clear();
  wtf_size_t size() const;
  protocol::Audits::InspectorIssue* at(wtf_size_t index) const;

  virtual ~InspectorIssueStorage();

 private:
  void AddInspectorIssue(CoreProbeSink*,
                         std::unique_ptr<protocol::Audits::InspectorIssue>);
  Deque<std::unique_ptr<protocol::Audits::InspectorIssue>> issues_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_STORAGE_H_
