// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_MESSAGING_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_MESSAGING_PROXY_H_

#include <memory>
#include <optional>

#include "base/time/time.h"
#include "third_party/blink/renderer/core/workers/threaded_worklet_messaging_proxy.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class AudioWorklet;
class AudioWorkletHandler;
class CrossThreadAudioParamInfo;
class CrossThreadAudioWorkletProcessorInfo;
class ExecutionContext;
class MessagePortChannel;
class SerializedScriptValue;
class WorkerThread;

// AudioWorkletMessagingProxy is a main thread interface for
// AudioWorkletGlobalScope. The proxy communicates with the associated global
// scope via AudioWorkletObjectProxy.
class MODULES_EXPORT AudioWorkletMessagingProxy final
    : public ThreadedWorkletMessagingProxy {
 public:
  AudioWorkletMessagingProxy(ExecutionContext*, AudioWorklet*);

  // Since the creation of AudioWorkletProcessor needs to be done in the
  // different thread, this method is a wrapper for cross-thread task posting.
  void CreateProcessor(scoped_refptr<AudioWorkletHandler>,
                       MessagePortChannel,
                       scoped_refptr<SerializedScriptValue> node_options);

  // Invokes AudioWorkletGlobalScope to create an instance of
  // AudioWorkletProcessor.
  void CreateProcessorOnRenderingThread(
      WorkerThread*,
      scoped_refptr<AudioWorkletHandler>,
      const String& name,
      MessagePortChannel,
      scoped_refptr<SerializedScriptValue> node_options);

  // Invoked by AudioWorkletObjectProxy on AudioWorkletThread to fetch the
  // information from AudioWorkletGlobalScope to AudioWorkletMessagingProxy
  // after the script code evaluation. It copies the information about newly
  // added AudioWorkletProcessor since the previous synchronization. (e.g.
  // processor name and AudioParam list)
  void SynchronizeWorkletProcessorInfoList(
      std::unique_ptr<Vector<CrossThreadAudioWorkletProcessorInfo>>);

  // Returns true if the processor with given name is registered in
  // AudioWorkletGlobalScope.
  bool IsProcessorRegistered(const String& name) const;

  Vector<CrossThreadAudioParamInfo> GetParamInfoListForProcessor(
      const String& name) const;

  // Returns a WorkerThread object backs the AudioWorkletThread instance.
  WorkerThread* GetBackingWorkerThread();

  // Create a Worklet backing thread based on constraints:
  // If realtime_buffer_duration is not provided:
  // 1. OfflineAudioContext: BACKGROUND priority thread.
  // Otherwise:
  // 2. AudioContext && outermost main frame: RT priority thread;
  // 3. AudioContext && sub frame: DISPLAY priority thread.
  static std::unique_ptr<WorkerThread> CreateWorkletThreadWithConstraints(
      WorkerReportingProxy&,
      std::optional<base::TimeDelta> realtime_buffer_duration,
      const bool is_outermost_main_frame);

  void Trace(Visitor*) const override;

 private:
  // Implements ThreadedWorkletMessagingProxy.
  std::unique_ptr<ThreadedWorkletObjectProxy> CreateObjectProxy(
      ThreadedWorkletMessagingProxy*,
      ParentExecutionContextTaskRunners*,
      scoped_refptr<base::SingleThreadTaskRunner>
          parent_agent_group_task_runner) override;

  std::unique_ptr<WorkerThread> CreateWorkerThread() override;

  // Each entry consists of processor name and associated AudioParam list.
  HashMap<String, Vector<CrossThreadAudioParamInfo>> processor_info_map_;

  Member<AudioWorklet> worklet_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_MESSAGING_PROXY_H_
