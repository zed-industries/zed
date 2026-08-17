/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_RESOURCE_ADAPTATION_PROCESSOR_H_
#define CALL_ADAPTATION_RESOURCE_ADAPTATION_PROCESSOR_H_

#include <map>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/adaptation/resource.h"
#include "api/ref_count.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/video/video_adaptation_counters.h"
#include "call/adaptation/resource_adaptation_processor_interface.h"
#include "call/adaptation/video_source_restrictions.h"
#include "call/adaptation/video_stream_adapter.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// The Resource Adaptation Processor is responsible for reacting to resource
// usage measurements (e.g. overusing or underusing CPU). When a resource is
// overused the Processor is responsible for performing mitigations in order to
// consume less resources.
//
// Today we have one Processor per VideoStreamEncoder and the Processor is only
// capable of restricting resolution or frame rate of the encoded stream. In the
// future we should have a single Processor responsible for all encoded streams,
// and it should be capable of reconfiguring other things than just
// VideoSourceRestrictions (e.g. reduce render frame rate).
// See Resource-Adaptation hotlist:
// https://bugs.chromium.org/u/590058293/hotlists/Resource-Adaptation
//
// The ResourceAdaptationProcessor is single-threaded. It may be constructed on
// any thread but MUST subsequently be used and destroyed on a single sequence,
// i.e. the "resource adaptation task queue". Resources can be added and removed
// from any thread.
class ResourceAdaptationProcessor : public ResourceAdaptationProcessorInterface,
                                    public VideoSourceRestrictionsListener,
                                    public ResourceListener {
 public:
  explicit ResourceAdaptationProcessor(
      VideoStreamAdapter* video_stream_adapter);
  ~ResourceAdaptationProcessor() override;

  // ResourceAdaptationProcessorInterface implementation.
  void AddResourceLimitationsListener(
      ResourceLimitationsListener* limitations_listener) override;
  void RemoveResourceLimitationsListener(
      ResourceLimitationsListener* limitations_listener) override;
  void AddResource(scoped_refptr<Resource> resource) override;
  std::vector<scoped_refptr<Resource>> GetResources() const override;
  void RemoveResource(scoped_refptr<Resource> resource) override;

  // ResourceListener implementation.
  // Triggers OnResourceUnderuse() or OnResourceOveruse().
  void OnResourceUsageStateMeasured(scoped_refptr<Resource> resource,
                                    ResourceUsageState usage_state) override;

  // VideoSourceRestrictionsListener implementation.
  void OnVideoSourceRestrictionsUpdated(
      VideoSourceRestrictions restrictions,
      const VideoAdaptationCounters& adaptation_counters,
      scoped_refptr<Resource> reason,
      const VideoSourceRestrictions& unfiltered_restrictions) override;

 private:
  // If resource usage measurements happens off the adaptation task queue, this
  // class takes care of posting the measurement for the processor to handle it
  // on the adaptation task queue.
  class ResourceListenerDelegate : public RefCountInterface,
                                   public ResourceListener {
   public:
    explicit ResourceListenerDelegate(ResourceAdaptationProcessor* processor);

    void OnProcessorDestroyed();

    // ResourceListener implementation.
    void OnResourceUsageStateMeasured(scoped_refptr<Resource> resource,
                                      ResourceUsageState usage_state) override;

   private:
    TaskQueueBase* task_queue_;
    ResourceAdaptationProcessor* processor_ RTC_GUARDED_BY(task_queue_);
  };

  enum class MitigationResult {
    kNotMostLimitedResource,
    kSharedMostLimitedResource,
    kRejectedByAdapter,
    kAdaptationApplied,
  };

  struct MitigationResultAndLogMessage {
    MitigationResultAndLogMessage();
    MitigationResultAndLogMessage(MitigationResult result,
                                  absl::string_view message);
    MitigationResult result;
    std::string message;
  };

  // Performs the adaptation by getting the next target, applying it and
  // informing listeners of the new VideoSourceRestriction and adaptation
  // counters.
  MitigationResultAndLogMessage OnResourceUnderuse(
      scoped_refptr<Resource> reason_resource);
  MitigationResultAndLogMessage OnResourceOveruse(
      scoped_refptr<Resource> reason_resource);

  void UpdateResourceLimitations(scoped_refptr<Resource> reason_resource,
                                 const VideoSourceRestrictions& restrictions,
                                 const VideoAdaptationCounters& counters)
      RTC_RUN_ON(task_queue_);

  // Searches `adaptation_limits_by_resources_` for each resource with the
  // highest total adaptation counts. Adaptation up may only occur if the
  // resource performing the adaptation is the only most limited resource. This
  // function returns the list of all most limited resources as well as the
  // corresponding adaptation of that resource.
  std::pair<std::vector<scoped_refptr<Resource>>,
            VideoStreamAdapter::RestrictionsWithCounters>
  FindMostLimitedResources() const RTC_RUN_ON(task_queue_);

  void RemoveLimitationsImposedByResource(scoped_refptr<Resource> resource);

  TaskQueueBase* task_queue_;
  scoped_refptr<ResourceListenerDelegate> resource_listener_delegate_;
  // Input and output.
  mutable Mutex resources_lock_;
  std::vector<scoped_refptr<Resource>> resources_
      RTC_GUARDED_BY(resources_lock_);
  std::vector<ResourceLimitationsListener*> resource_limitations_listeners_
      RTC_GUARDED_BY(task_queue_);
  // Purely used for statistics, does not ensure mapped resources stay alive.
  std::map<scoped_refptr<Resource>,
           VideoStreamAdapter::RestrictionsWithCounters>
      adaptation_limits_by_resources_ RTC_GUARDED_BY(task_queue_);
  // Responsible for generating and applying possible adaptations.
  VideoStreamAdapter* const stream_adapter_ RTC_GUARDED_BY(task_queue_);
  VideoSourceRestrictions last_reported_source_restrictions_
      RTC_GUARDED_BY(task_queue_);
  // Keeps track of previous mitigation results per resource since the last
  // successful adaptation. Used to avoid RTC_LOG spam.
  std::map<Resource*, MitigationResult> previous_mitigation_results_
      RTC_GUARDED_BY(task_queue_);
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_RESOURCE_ADAPTATION_PROCESSOR_H_
