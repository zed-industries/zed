/*
 * Copyright (C) 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SVG_SMIL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SVG_SMIL_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/animation/smil_repeat_count.h"
#include "third_party/blink/renderer/core/svg/animation/smil_time.h"
#include "third_party/blink/renderer/core/svg/svg_element.h"
#include "third_party/blink/renderer/core/svg/svg_tests.h"
#include "third_party/blink/renderer/core/svg_names.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ConditionEventListener;
class SMILTimeContainer;
class IdTargetObserver;
class SVGSMILElement;

class CORE_EXPORT SMILInstanceTimeList {
 public:
  void Append(SMILTime, SMILTimeOrigin);
  void InsertSortedAndUnique(SMILTime, SMILTimeOrigin);
  void RemoveWithOrigin(SMILTimeOrigin);
  void Sort();
  SMILTime NextAfter(SMILTime) const;

  wtf_size_t size() const { return instance_times_.size(); }
  bool IsEmpty() const { return instance_times_.empty(); }

  using const_iterator = typename Vector<SMILTimeWithOrigin>::const_iterator;
  const_iterator begin() const { return instance_times_.begin(); }
  const_iterator end() const { return instance_times_.end(); }

 private:
  Vector<SMILTimeWithOrigin> instance_times_;
  SMILTimeOriginSet time_origins_;
};

// This class implements SMIL interval timing model as needed for SVG animation.
class CORE_EXPORT SVGSMILElement : public SVGElement, public SVGTests {
 public:
  SVGSMILElement(const QualifiedName&, Document&);
  ~SVGSMILElement() override;

  void ParseAttribute(const AttributeModificationParams&) override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  SMILTimeContainer* TimeContainer() const { return time_container_.Get(); }

  bool HasValidTarget() const;
  SVGElement* targetElement() const { return target_element_.Get(); }

  void BeginByLinkActivation();

  enum Restart { kRestartAlways, kRestartWhenNotActive, kRestartNever };

  Restart GetRestart() const { return static_cast<Restart>(restart_); }

  enum FillMode { kFillRemove, kFillFreeze };

  FillMode Fill() const { return static_cast<FillMode>(fill_); }

  SMILTime Dur() const;
  SMILTime RepeatDur() const;
  SMILRepeatCount RepeatCount() const;
  SMILTime MaxValue() const;
  SMILTime MinValue() const;

  SMILTime Elapsed() const;

  SMILTime IntervalBegin() const { return interval_.begin; }
  SMILTime SimpleDuration() const;

  void UpdateInterval(SMILTime presentation_time);
  enum EventDispatchMask {
    kDispatchNoEvent = 0,
    kDispatchBeginEvent = 1u << 0,
    kDispatchRepeatEvent = 1u << 1,
    kDispatchEndEvent = 1u << 2,
  };
  EventDispatchMask UpdateActiveState(SMILTime presentation_time,
                                      bool skip_repeat);
  EventDispatchMask ComputeSeekEvents(
      const SMILInterval& starting_interval) const;
  void DispatchEvents(EventDispatchMask);
  void UpdateProgressState(SMILTime presentation_time);
  bool IsHigherPriorityThan(const SVGSMILElement* other,
                            SMILTime presentation_time) const;

  enum IncludeRepeats { kIncludeRepeats, kExcludeRepeats };
  SMILTime ComputeNextIntervalTime(SMILTime presentation_time,
                                   IncludeRepeats) const;
  SMILTime NextProgressTime(SMILTime elapsed) const;

  void Reset();

  static SMILTime ParseClockValue(const String&);
  static SMILTime ParseOffsetValue(const String&);

  bool IsContributing(SMILTime elapsed) const;
  const SMILInterval& GetActiveInterval(SMILTime presentation_time) const;

  unsigned DocumentOrderIndex() const { return document_order_index_; }
  void SetDocumentOrderIndex(unsigned index) { document_order_index_ = index; }

  wtf_size_t& PriorityQueueHandle() { return queue_handle_; }

  void Trace(Visitor*) const override;

 protected:
  enum BeginOrEnd { kBegin, kEnd };

  void AddInstanceTimeAndUpdate(BeginOrEnd, SMILTime, SMILTimeOrigin);

  void SetTargetElement(SVGElement*);

  // Sub-classes may need to take action when the target is changed.
  virtual void WillChangeAnimationTarget();
  virtual void DidChangeAnimationTarget();

  struct ProgressState {
    float progress;
    unsigned repeat;
  };
  const ProgressState& GetProgressState() const { return last_progress_; }

 private:
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;
  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;

  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) final;

  void BuildPendingResource() override;
  void ClearResourceAndEventBaseReferences();
  void ClearConditions();

  void StartedActiveInterval();
  void EndedActiveInterval();

  bool LayoutObjectIsNeeded(const DisplayStyle&) const override {
    return false;
  }

  SMILTime NextAfter(BeginOrEnd, SMILTime time) const;

  SMILTime BeginTimeForPrioritization(SMILTime presentation_time) const;

  SMILInterval ResolveInterval(SMILTime begin_after, SMILTime end_after);
  // Check if the current interval is still current, and apply restart
  // semantics. Returns true if a new interval should be resolved.
  bool HandleIntervalRestart(SMILTime presentation_time);
  void DiscardOrRevalidateCurrentInterval(SMILTime presentation_time);
  SMILTime ResolveActiveEnd(SMILTime resolved_begin) const;
  SMILTime RepeatingDuration() const;
  void SetNewInterval(const SMILInterval&);
  void SetNewIntervalEnd(SMILTime new_end);

  void AddInstanceTime(BeginOrEnd begin_or_end,
                       SMILTime time,
                       SMILTimeOrigin origin);
  void InstanceListChanged();
  void IntervalStateChanged();

  // This represents conditions on elements begin or end list that need to be
  // resolved on runtime, for example
  // <animate begin="otherElement.begin + 8s; button.click" ... />
  class Condition final : public GarbageCollected<Condition> {
   public:
    enum Type { kEventBase, kSyncBase, kAccessKey };

    Condition(Type,
              BeginOrEnd,
              const AtomicString& base_id,
              const AtomicString& name,
              SMILTime offset,
              unsigned repeat);

    ~Condition();
    void Trace(Visitor*) const;

    Type GetType() const { return type_; }
    BeginOrEnd GetBeginOrEnd() const { return begin_or_end_; }
    const AtomicString& GetName() const { return name_; }
    SMILTime Offset() const { return offset_; }
    unsigned Repeat() const { return repeat_; }

    void ConnectSyncBase(SVGSMILElement&);
    void DisconnectSyncBase(SVGSMILElement&);
    bool IsSyncBaseFor(SVGSMILElement* timed_element) const {
      return GetType() == kSyncBase && base_element_ == timed_element;
    }

    void ConnectEventBase(SVGSMILElement&);
    void DisconnectEventBase(SVGSMILElement&);

   private:
    Type type_;
    BeginOrEnd begin_or_end_;
    AtomicString base_id_;
    AtomicString name_;
    SMILTime offset_;
    unsigned repeat_;
    Member<Element> base_element_;
    Member<IdTargetObserver> base_id_observer_;
    Member<ConditionEventListener> event_listener_;
  };
  bool ParseCondition(const String&, BeginOrEnd begin_or_end);
  void ParseBeginOrEnd(const String&, BeginOrEnd begin_or_end);

  void ConnectConditions();
  void DisconnectConditions();

  void AddedToTimeContainer();
  void RemovedFromTimeContainer();

  void NotifyDependentsOnNewInterval(const SMILInterval& interval);
  void NotifyDependentsOnRepeat(unsigned repeat_nr, SMILTime repeat_time);

  struct NotifyDependentsInfo;
  void NotifyDependents(const NotifyDependentsInfo& info);
  void CreateInstanceTimesFromSyncBase(SVGSMILElement* timed_element,
                                       const NotifyDependentsInfo& info);
  void AddSyncBaseDependent(SVGSMILElement&);
  void RemoveSyncBaseDependent(SVGSMILElement&);

  enum ActiveState { kInactive, kActive, kFrozen };

  ActiveState GetActiveState() const {
    return static_cast<ActiveState>(active_state_);
  }
  ActiveState DetermineActiveState(const SMILInterval& interval,
                                   SMILTime elapsed) const;

  ProgressState CalculateProgressState(SMILTime presentation_time) const;

  SMILTime LastIntervalEndTime() const;

  Member<SVGElement> target_element_;
  Member<IdTargetObserver> target_id_observer_;

  HeapVector<Member<Condition>> conditions_;
  bool conditions_connected_;
  bool has_end_event_conditions_;

  bool is_waiting_for_first_interval_;
  bool is_scheduled_;

  using TimeDependentSet = HeapHashSet<Member<SVGSMILElement>>;
  TimeDependentSet sync_base_dependents_;

  // Instance time lists
  SMILInstanceTimeList begin_times_;
  SMILInstanceTimeList end_times_;

  // This is the upcoming or current interval
  SMILInterval interval_;
  // This is the previous interval. It should always be non-overlapping and
  // "before" |interval_|.
  SMILInterval previous_interval_;

  unsigned active_state_ : 2;
  unsigned restart_ : 2;
  unsigned fill_ : 1;
  ProgressState last_progress_;

  Member<SMILTimeContainer> time_container_;
  unsigned document_order_index_;
  wtf_size_t queue_handle_;

  mutable SMILTime cached_dur_;
  mutable SMILTime cached_repeat_dur_;
  mutable SMILRepeatCount cached_repeat_count_;
  mutable SMILTime cached_min_;
  mutable SMILTime cached_max_;

  bool interval_has_changed_;
  bool instance_lists_have_changed_;
  bool interval_needs_revalidation_;
  bool is_notifying_dependents_;

  friend class ConditionEventListener;
};

template <>
struct DowncastTraits<SVGSMILElement> {
  static bool AllowFrom(const Node& node) {
    auto* svg_element = DynamicTo<SVGElement>(node);
    return svg_element && AllowFrom(*svg_element);
  }
  static bool AllowFrom(const SVGElement& svg_element) {
    return svg_element.HasTagName(svg_names::kSetTag) ||
           svg_element.HasTagName(svg_names::kAnimateTag) ||
           svg_element.HasTagName(svg_names::kAnimateMotionTag) ||
           svg_element.HasTagName(svg_names::kAnimateTransformTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SVG_SMIL_ELEMENT_H_
