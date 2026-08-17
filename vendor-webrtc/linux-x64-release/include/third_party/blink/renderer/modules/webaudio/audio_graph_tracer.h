// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_GRAPH_TRACER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_GRAPH_TRACER_H_

#include "third_party/blink/renderer/core/page/page.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class AudioListener;
class AudioNode;
class AudioParam;
class BaseAudioContext;
class LocalDOMWindow;
class InspectorWebAudioAgent;
class Page;

class MODULES_EXPORT AudioGraphTracer final
    : public GarbageCollected<AudioGraphTracer>,
      public Supplement<Page> {
 public:
  static const char kSupplementName[];

  static void ProvideAudioGraphTracerTo(Page&);

  AudioGraphTracer(Page& page);

  void Trace(Visitor*) const override;

  void SetInspectorAgent(InspectorWebAudioAgent*);

  // Graph lifecycle events: notifies an associated inspector agent about
  // the object lifecycle of BaseAudioContext, AudioListener, AudioNode, and
  // AudioParam.
  void DidCreateBaseAudioContext(BaseAudioContext*);
  void WillDestroyBaseAudioContext(BaseAudioContext*);
  void DidCreateAudioListener(AudioListener*);
  void WillDestroyAudioListener(AudioListener*);
  void DidCreateAudioNode(AudioNode*);
  void WillDestroyAudioNode(AudioNode*);
  void DidCreateAudioParam(AudioParam*);
  void WillDestroyAudioParam(AudioParam*);

  // Graph connection events: notifies an associated inspector agent about
  // when a connection between graph objects happens.
  void DidConnectNodes(AudioNode* source_node,
                       AudioNode* destination_node,
                       unsigned source_output_index = 0,
                       unsigned destination_input_index = 0);
  void DidDisconnectNodes(AudioNode* source_node,
                          AudioNode* destination_node = nullptr,
                          unsigned source_output_index = 0,
                          unsigned destination_input_index = 0);
  void DidConnectNodeParam(AudioNode* source_node,
                           AudioParam* destination_param,
                           unsigned source_output_index = 0);
  void DidDisconnectNodeParam(AudioNode* source_node,
                              AudioParam* destination_param,
                              unsigned source_output_index = 0);

  // Notifies an associated inspector agent when a BaseAudioContext is changed.
  void DidChangeBaseAudioContext(BaseAudioContext*);

  BaseAudioContext* GetContextById(const String contextId);

  static AudioGraphTracer* FromPage(Page*);
  static AudioGraphTracer* FromWindow(const LocalDOMWindow&);

 private:
  Member<InspectorWebAudioAgent> inspector_agent_;
  HeapHashSet<WeakMember<BaseAudioContext>> contexts_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_GRAPH_TRACER_H_
