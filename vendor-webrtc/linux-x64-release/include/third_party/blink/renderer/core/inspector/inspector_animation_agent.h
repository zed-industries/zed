// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ANIMATION_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ANIMATION_AGENT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_animation_play_state.h"
#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"
#include "third_party/blink/renderer/core/animation/scroll_snapshot_timeline.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_keyframes_rule.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/animation.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/weak_cell.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-inspector.h"

namespace blink {

class DocumentTimeline;
class InspectedFrames;
class InspectorCSSAgent;

class CORE_EXPORT InspectorAnimationAgent final
    : public InspectorBaseAgent<protocol::Animation::Metainfo> {
 public:
  InspectorAnimationAgent(InspectedFrames*,
                          InspectorCSSAgent*,
                          v8_inspector::V8InspectorSession*);
  InspectorAnimationAgent(const InspectorAnimationAgent&) = delete;
  InspectorAnimationAgent& operator=(const InspectorAnimationAgent&) = delete;

  // Base agent methods.
  void Restore() override;
  void DidCommitLoadForLocalFrame(LocalFrame*) override;

  // Protocol method implementations
  protocol::Response enable() override;
  protocol::Response disable() override;
  protocol::Response getPlaybackRate(double* playback_rate) override;
  protocol::Response setPlaybackRate(double) override;
  protocol::Response getCurrentTime(const String& id,
                                    double* current_time) override;
  protocol::Response setPaused(
      std::unique_ptr<protocol::Array<String>> animations,
      bool paused) override;
  protocol::Response setTiming(const String& animation_id,
                               double duration,
                               double delay) override;
  protocol::Response seekAnimations(
      std::unique_ptr<protocol::Array<String>> animations,
      double current_time) override;
  protocol::Response releaseAnimations(
      std::unique_ptr<protocol::Array<String>> animations) override;
  protocol::Response resolveAnimation(
      const String& animation_id,
      std::unique_ptr<v8_inspector::protocol::Runtime::API::RemoteObject>*)
      override;

  // API for InspectorInstrumentation
  void DidCreateAnimation(unsigned);
  void AnimationUpdated(blink::Animation*);
  void DidClearDocumentOfWindowObject(LocalFrame*);

  // Methods for other agents to use.
  protocol::Response AssertAnimation(const String& id,
                                     blink::Animation*& result);

  static String AnimationDisplayName(const Animation& animation);
  void Trace(Visitor*) const override;

 private:
  struct AnimationKeyframeSnapshot
      : public GarbageCollected<AnimationKeyframeSnapshot> {
    double computed_offset;
    String easing;

    void Trace(Visitor* visitor) const {}
  };
  struct AnimationSnapshot : public GarbageCollected<AnimationSnapshot> {
    double start_time;
    double duration;
    double delay;
    double end_delay;
    double iterations;
    String timing_function;
    HeapVector<Member<AnimationKeyframeSnapshot>> keyframes;
    std::optional<double> start_offset;
    std::optional<double> end_offset;
    V8AnimationPlayState::Enum play_state;

    void Trace(Visitor* visitor) const { visitor->Trace(keyframes); }
  };
  using AnimationType = protocol::Animation::Animation::TypeEnum;

  std::unique_ptr<protocol::Animation::Animation> BuildObjectForAnimation(
      blink::Animation&);
  double NormalizedStartTime(blink::Animation&);
  DocumentTimeline& ReferenceTimeline();
  String CreateCSSId(blink::Animation&);
  void InvalidateInternalState();
  // Updates the given animation snapshot and
  // returns whether any value of the snapshot is updated or not.
  bool CompareAndUpdateInternalSnapshot(blink::Animation& animation,
                                        AnimationSnapshot* snapshot);
  bool CompareAndUpdateKeyframesSnapshot(
      KeyframeEffect* keyframe_effect,
      HeapVector<Member<AnimationKeyframeSnapshot>>* snapshot_keyframes);
  void NotifyAnimationUpdated(const String& animation_id);

  Member<InspectedFrames> inspected_frames_;
  Member<InspectorCSSAgent> css_agent_;
  v8_inspector::V8InspectorSession* v8_session_;
  // Keeps track of the snapshot of animations that are sent to the frontend.
  // The snapshots are used to check whether to send an `animationUpdated` event
  // when a blink::Animation instance is updated.
  HeapHashMap<String, Member<AnimationSnapshot>> id_to_animation_snapshot_;
  // Keeps track of the blink::Animation instances by their ids.
  HeapHashMap<String, WeakMember<blink::Animation>> id_to_animation_;
  bool is_cloning_;
  HashSet<String> cleared_animations_;
  InspectorAgentState::Boolean enabled_;
  InspectorAgentState::Double playback_rate_;
  // Keeps track of the animation ids that has an active notifyAnimationUpdate
  // task.
  HashSet<String> notify_animation_updated_tasks_;
  WeakCellFactory<InspectorAnimationAgent> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ANIMATION_AGENT_H_
