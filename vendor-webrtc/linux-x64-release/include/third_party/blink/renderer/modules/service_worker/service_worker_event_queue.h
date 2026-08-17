// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_EVENT_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_EVENT_QUEUE_H_

#include <optional>
#include <set>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"
#include "base/timer/timer.h"
#include "third_party/blink/public/mojom/service_worker/service_worker.mojom-blink.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_event_status.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace base {

class TickClock;

}  // namespace base

namespace blink {

// ServiceWorkerEventQueue manages events dispatched on ServiceWorkerGlobalScope
// and their timeouts.
//
// There are two types of timeouts: the long standing event timeout
// and the idle timeout.
//
// 1) Event timeout: when an event starts, StartEvent() records the expiration
// time of the event (kEventTimeout). If EndEvent() has not been called within
// the timeout time, |abort_callback| passed to StartEvent() is called with
// status TIMEOUT. Additionally, the |idle_delay_| is set to zero to shut down
// the worker as soon as possible since the worker may have gone into bad state.
// 2) Idle timeout: when a certain time has passed (|idle_delay_|) since all of
// events have ended, ServiceWorkerEventQueue calls the |idle_callback_|.
// Idle callbacks are called on the |task_runner| passed by the constructor.
// Note that the implementation assumes the task runner is provided by
// ServiceWorkerGlobalScope, and the lifetime of the task runner is shorter than
// |this|.
//
// The lifetime of ServiceWorkerEventQueue is the same with the worker
// thread. If ServiceWorkerEventQueue is destructed while there are inflight
// events, all |abort_callback|s will be immediately called with status ABORTED.
class MODULES_EXPORT ServiceWorkerEventQueue {
 public:
  // A token to keep the event queue from going into the idle state if any of
  // them are alive.
  class MODULES_EXPORT StayAwakeToken {
   public:
    explicit StayAwakeToken(base::WeakPtr<ServiceWorkerEventQueue> event_queue);
    ~StayAwakeToken();

   private:
    base::WeakPtr<ServiceWorkerEventQueue> event_queue_;
  };

  using AbortCallback =
      base::OnceCallback<void(int /* event_id */,
                              mojom::blink::ServiceWorkerEventStatus)>;

  ServiceWorkerEventQueue(base::RepeatingClosure idle_callback,
                          scoped_refptr<base::SequencedTaskRunner> task_runner);
  // For testing.
  ServiceWorkerEventQueue(base::RepeatingClosure idle_callback,
                          scoped_refptr<base::SequencedTaskRunner> task_runner,
                          const base::TickClock* tick_clock);
  ~ServiceWorkerEventQueue();

  // Starts the event_queue. This may also update |idle_time_| if there was no
  // activities (i.e., StartEvent()/EndEvent() or StayAwakeToken creation)
  // on the event_queue before.
  void Start();

  using StartCallback = base::OnceCallback<void(int /* event_id */)>;

  // EndEvent() must be called when an event finishes, with |event_id|
  // which StartCallback received.
  void EndEvent(int event_id);

  // Enqueues a Normal event. See ServiceWorkerEventQueue::Event to know the
  // meaning of each parameter.
  void EnqueueNormal(int event_id,
                     StartCallback start_callback,
                     AbortCallback abort_callback,
                     std::optional<base::TimeDelta> custom_timeout);

  // Similar to EnqueueNormal(), but enqueues a Pending event.
  void EnqueuePending(int event_id,
                      StartCallback start_callback,
                      AbortCallback abort_callback,
                      std::optional<base::TimeDelta> custom_timeout);

  // Returns true if |event_id| was enqueued and hasn't ended.
  bool HasEvent(int event_id) const;

  // Returns true if |event_id| was enqueued and hasn't started.
  bool HasEventInQueue(int event_id) const;

  // Creates a StayAwakeToken to ensure that the idle callback won't be
  // triggered while any of these are alive.
  std::unique_ptr<StayAwakeToken> CreateStayAwakeToken();

  // Sets the |idle_delay_| to the new value, and re-schedule the idle callback
  // based on the new delay if it's now pending.
  // This method must be called after the event queue is started.
  void SetIdleDelay(base::TimeDelta idle_delay);

  // Resets the members for idle timeouts to cancel the idle callback.
  void ResetIdleTimeout();

  // Checks if the event queue is empty.
  // Schedules idle timeout callback if there is no ongoing event.
  void CheckEventQueue();

  // Returns true if the event queue thinks no events ran for a while, and has
  // triggered the |idle_callback_| passed to the constructor. It'll be reset to
  // false again when StartEvent() is called.
  bool did_idle_timeout() const { return did_idle_timeout_; }

  // Returns the next event id, which is a monotonically increasing number.
  int NextEventId();

  // Duration of the long standing event timeout since StartEvent() has been
  // called.
  static constexpr base::TimeDelta kEventTimeout = base::Minutes(5);
  // ServiceWorkerEventQueue periodically updates the timeout state by
  // kUpdateInterval.
  static constexpr base::TimeDelta kUpdateInterval = base::Seconds(10);

 private:
  // Represents an event dispatch, which can be queued into |queue_|.
  struct Event {
    enum class Type {
      // A normal event is enqueued to the end of the event queue and
      // triggers processing of the queue.
      Normal,
      // A pending event which does not start until a normal event is
      // pushed. A pending event should be used if the idle timeout
      // occurred (did_idle_timeout() returns true). In practice, the
      // caller enqueues pending events after the service worker
      // requested the browser to terminate it due to idleness. These
      // events run when a new event comes from the browser,
      // signalling that the browser decided not to terminate the
      // worker.
      Pending,
    };

    Event(int event_id,
          Type type,
          StartCallback start_callback,
          AbortCallback abort_callback,
          std::optional<base::TimeDelta> custom_timeout);
    ~Event();
    const int event_id;
    Type type;
    // Callback which is run when the event queue starts this event. The
    // callback receives |event_id|. When an event finishes,
    // EndEvent() should be called with the given |event_id|.
    StartCallback start_callback;
    // Callback which is run when a started event is aborted.
    AbortCallback abort_callback;
    // The custom timeout value.
    std::optional<base::TimeDelta> custom_timeout;
  };

  // Enqueues the event to |queue_|, and run events in the queue or sometimes
  // later synchronously, depending on the type of events.
  void EnqueueEvent(std::unique_ptr<Event> event);

  // Starts a single event.
  void StartEvent(int event_id, std::unique_ptr<Event> event);

  // Updates the internal states and fires the event timeout callbacks if any.
  // TODO(shimazu): re-implement it by delayed tasks and cancelable callbacks.
  void UpdateStatus();

  // Schedules the idle callback in |delay|.
  void ScheduleIdleCallback(base::TimeDelta delay);

  // Runs the idle callback and updates the state accordingly.
  void TriggerIdleCallback();

  // Sets the |idle_time_| and maybe calls |idle_callback_| immediately if the
  // timeout delay is set to zero.
  void OnNoInflightEvent();

  // Returns true if there are running events.
  bool HasInflightEvent() const;

  // Processes all events in |queue_|.
  void ProcessEvents();

  // True if the idle callback is scheduled to run.
  bool HasScheduledIdleCallback() const;

  struct EventInfo {
    EventInfo(base::TimeTicks expiration_time,
              base::OnceCallback<void(mojom::blink::ServiceWorkerEventStatus)>
                  abort_callback);
    ~EventInfo();
    const base::TimeTicks expiration_time;
    base::OnceCallback<void(mojom::blink::ServiceWorkerEventStatus)>
        abort_callback;
  };

  scoped_refptr<base::SequencedTaskRunner> task_runner_;

  // For long standing event timeouts. This is used to look up an EventInfo
  // by event id.
  HashMap<int /* event_id */, std::unique_ptr<EventInfo>> all_events_;

  // For idle timeouts. When there's no inflight event, this is scheduled to run
  // in |idle_delay_|.
  base::RepeatingClosure idle_callback_;

  // For idle timeouts. The handle of the scheduled |idle_callback_|. This can
  // be canceled when a new event happens before the scheduled callback runs.
  TaskHandle idle_callback_handle_;

  // For idle timeouts. The time when there's no inflight event. Set to null if
  // there are inflight events.
  base::TimeTicks last_no_inflight_event_time_;

  // For idle timeouts. The delay until the worker is identified as idle after
  // all inflight events are completed.
  base::TimeDelta idle_delay_ =
      base::Seconds(mojom::blink::kServiceWorkerDefaultIdleDelayInSeconds);

  // Set to true once |idle_callback_| has been invoked. Set to false when
  // StartEvent() is called.
  bool did_idle_timeout_ = false;

  // Event queue to where all online events (normal and pending events) are
  // enqueued. We use std::map as a task queue because it's ordered by the
  // `event_id` and the entries can be effectively erased in random order.
  std::map<int /* event_id */, std::unique_ptr<Event>> queued_online_events_
      ALLOW_DISCOURAGED_TYPE("Needs to be ordered by event_id");

  // Set to true during running ProcessEvents(). This is used for avoiding to
  // invoke |idle_callback_| or to re-enter ProcessEvents() when calling
  // ProcessEvents().
  bool processing_events_ = false;

  // The number of the living StayAwakeToken. See also class comments.
  int num_of_stay_awake_tokens_ = 0;

  // |timer_| invokes UpdateEventStatus() periodically.
  base::RepeatingTimer timer_;

  // If true, this event queue is ready for processing events.
  bool is_ready_for_processing_events_ = false;

  // |tick_clock_| outlives |this|.
  const raw_ptr<const base::TickClock> tick_clock_;

  // Monotonically increasing number. Event id should not start from zero since
  // HashMap in Blink requires non-zero keys.
  int next_event_id_ = 1;

  base::WeakPtrFactory<ServiceWorkerEventQueue> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_EVENT_QUEUE_H_
