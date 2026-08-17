// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_REPORTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_REPORTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace base {
class UnguessableToken;
}

namespace blink {
class CoreProbeSink;
class DocumentLoader;
class LocalFrame;
class ResourceError;
class InspectorIssueStorage;

// This class is always present on the local frame and can be used to
// subscribe to core probes for issue reporting (which is on independent of
// whether a DevToolsSession is attached).
class CORE_EXPORT InspectorIssueReporter final
    : public GarbageCollected<InspectorIssueReporter> {
 public:
  explicit InspectorIssueReporter(InspectorIssueStorage* storage);
  ~InspectorIssueReporter();
  InspectorIssueReporter(const InspectorIssueReporter&) = delete;
  InspectorIssueReporter& operator=(const InspectorIssueReporter&) = delete;

  // Core Probes.
  void DidFailLoading(CoreProbeSink* sink,
                      uint64_t identifier,
                      DocumentLoader* loader,
                      const ResourceError& error,
                      const base::UnguessableToken& token);
  void DomContentLoadedEventFired(LocalFrame*);

  void Trace(Visitor*) const;

 private:
  InspectorIssueStorage* storage_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_REPORTER_H_
