// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_AGENT_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_AGENT_GENERATOR_H_

#include "third_party/blink/public/mojom/annotation/annotation.mojom-blink.h"
#include "third_party/blink/renderer/core/annotation/annotation_agent_container_impl.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_selector_generator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

// This class is a helper class that is used by AnnotationAgentContainerImpl and
// responsible for kicking off link generation and retrieving link generation
// results. This class enables both preemptive link generations and post request
// link generations.
class CORE_EXPORT AnnotationAgentGenerator final
    : public GarbageCollected<AnnotationAgentGenerator> {
 public:
  explicit AnnotationAgentGenerator(LocalFrame* frame);
  AnnotationAgentGenerator(const AnnotationAgentGenerator&) = delete;
  AnnotationAgentGenerator& operator=(const AnnotationAgentGenerator&) = delete;

  using SelectorGenerationCallback =
      base::OnceCallback<void(mojom::blink::AnnotationType,
                              shared_highlighting::LinkGenerationReadyStatus,
                              const String&,
                              const TextFragmentSelector&,
                              shared_highlighting::LinkGenerationError)>;

  void Trace(Visitor* visitor) const;

  // This starts running the generator over the current selection.
  void PreemptivelyGenerateForCurrentSelection();

  // This requests the result of |PreemptivelyGenerateForCurrentSelection| if
  // the link generation was preemptive over the current selection. Otherwise,
  // it starts a new generation for the current selection and returns the result
  // by the callback.
  void GetForCurrentSelection(mojom::blink::AnnotationType type,
                              SelectorGenerationCallback callback);

 private:
  void GenerateSelector();
  void DidFinishGeneration(const TextFragmentSelector& selector,
                           shared_highlighting::LinkGenerationError error);
  void InvokeCompletionCallbackIfNeeded(
      shared_highlighting::LinkGenerationReadyStatus ready_status);

  SelectorGenerationCallback callback_;
  mojom::blink::AnnotationType type_;
  shared_highlighting::LinkGenerationError selector_error_;
  std::optional<TextFragmentSelector> generation_result_;
  Member<TextFragmentSelectorGenerator> generator_;
  Member<LocalFrame> frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_AGENT_GENERATOR_H_
