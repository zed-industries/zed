// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_UKM_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_UKM_RECORDER_H_

#include "components/ukm/test_ukm_recorder.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Document;

class InternalsUkmRecorder final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit InternalsUkmRecorder(Document*);
  ~InternalsUkmRecorder() override = default;

  HeapVector<ScriptObject> getMetrics(ScriptState* script_state,
                                      const String& entry_name,
                                      const Vector<String>& metric_names);

 private:
  ukm::TestAutoSetUkmRecorder recorder_;
  ukm::SourceId source_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_UKM_RECORDER_H_
