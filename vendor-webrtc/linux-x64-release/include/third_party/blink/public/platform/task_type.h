// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_TASK_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_TASK_TYPE_H_

namespace blink {

// A list of task sources known to Blink according to the spec.
// This enum is used for a histogram and it should not be re-numbered.
//
// For the task type usage guideline, see https://bit.ly/2vMAsQ4
//
// When a new task type is created:
// * Set the new task type's value to "Next value"
// * Update kMaxValue to point to the new task type
// * Increment "Next value"
// * Update TaskTypes.md
//
// Next value: 88
enum class TaskType : unsigned char {
  ///////////////////////////////////////
  // Speced tasks should use one of the following task types
  ///////////////////////////////////////

  // Speced tasks and related internal tasks should be posted to one of
  // the following task runners. These task runners may be throttled.

  // This value is used as a default value in cases where TaskType
  // isn't supported yet. Don't use outside platform/scheduler code.
  kDeprecatedNone = 0,

  // https://html.spec.whatwg.org/multipage/webappapis.html#generic-task-sources
  //
  // This task source is used for features that react to DOM manipulations, such
  // as things that happen in a non-blocking fashion when an element is inserted
  // into the document.
  kDOMManipulation = 1,
  // This task source is used for features that react to user interaction, for
  // example keyboard or mouse input. Events sent in response to user input
  // (e.g. click events) must be fired using tasks queued with the user
  // interaction task source.
  kUserInteraction = 2,
  // TODO(altimin) Fix the networking task source related namings once it is
  // clear how
  // all loading tasks are annotated.
  // This task source is used for features that trigger in response to network
  // activity.
  kNetworking = 3,
  // This is a part of Networking task that should not be frozen when a page is
  // frozen.
  kNetworkingUnfreezable = 75,
  // Tasks associated with loading that should also block rendering. Split off
  // from kNetworkingUnfreezable so tasks such as image loading can be
  // prioritized above rendering. Note that not all render-blocking resources
  // use this queue (e.g., script load tasks are not put here).
  kNetworkingUnfreezableRenderBlockingLoading = 83,
  // This task source is used for control messages between kNetworking tasks.
  kNetworkingControl = 4,
  // Tasks used to run low priority scripts.
  kLowPriorityScriptExecution = 81,
  // This task source is used to queue calls to history.back() and similar APIs.
  kHistoryTraversal = 5,

  // https://html.spec.whatwg.org/multipage/embedded-content.html#the-embed-element
  // This task source is used for the embed element setup steps.
  kEmbed = 6,

  // https://html.spec.whatwg.org/multipage/embedded-content.html#media-elements
  // This task source is used for all tasks queued in the [Media elements]
  // section and subsections of the spec unless explicitly specified otherwise.
  kMediaElementEvent = 7,

  // https://html.spec.whatwg.org/multipage/scripting.html#the-canvas-element
  // This task source is used to invoke the result callback of
  // HTMLCanvasElement.toBlob().
  kCanvasBlobSerialization = 8,

  // https://html.spec.whatwg.org/multipage/webappapis.html#event-loop-processing-model
  // This task source is used when an algorithm requires a microtask to be
  // queued.
  kMicrotask = 9,

  // https://html.spec.whatwg.org/multipage/webappapis.html#timers
  // For tasks queued by setTimeout() or setInterval().
  //
  // Task nesting level is < 5 and timeout is zero.
  kJavascriptTimerImmediate = 72,
  // Task nesting level is < 5 and timeout is > 0.
  kJavascriptTimerDelayedLowNesting = 73,
  // Task nesting level is >= 5.
  kJavascriptTimerDelayedHighNesting = 10,
  // Note: The timeout is increased to be at least 4ms when the task nesting
  // level is >= 5. Therefore, the timeout is necessarily > 0 for
  // kJavascriptTimerDelayedHighNesting.

  // https://html.spec.whatwg.org/multipage/comms.html#sse-processing-model
  // This task source is used for any tasks that are queued by EventSource
  // objects.
  kRemoteEvent = 11,

  // https://html.spec.whatwg.org/multipage/comms.html#feedback-from-the-protocol
  // The task source for all tasks queued in the [WebSocket] section of the
  // spec.
  kWebSocket = 12,

  // https://html.spec.whatwg.org/multipage/comms.html#web-messaging
  // This task source is used for the tasks in cross-document messaging.
  kPostedMessage = 13,

  // https://html.spec.whatwg.org/multipage/comms.html#message-ports
  kUnshippedPortMessage = 14,

  // https://www.w3.org/TR/FileAPI/#blobreader-task-source
  // This task source is used for all tasks queued in the FileAPI spec to read
  // byte sequences associated with Blob and File objects.
  kFileReading = 15,

  // https://www.w3.org/TR/IndexedDB/#request-api
  kDatabaseAccess = 16,

  // https://w3c.github.io/presentation-api/#common-idioms
  // This task source is used for all tasks in the Presentation API spec.
  kPresentation = 17,

  // https://www.w3.org/TR/2016/WD-generic-sensor-20160830/#sensor-task-source
  // This task source is used for all tasks in the Sensor API spec.
  kSensor = 18,

  // https://w3c.github.io/performance-timeline/#performance-timeline
  kPerformanceTimeline = 19,

  // https://www.khronos.org/registry/webgl/specs/latest/1.0/#5.15
  // This task source is used for all tasks in the WebGL spec.
  kWebGL = 20,

  // https://www.w3.org/TR/requestidlecallback/#start-an-event-loop-s-idle-period
  kIdleTask = 21,

  // Use MiscPlatformAPI for a task that is defined in the spec but is not yet
  // associated with any specific task runner in the spec. MiscPlatformAPI is
  // not encouraged for stable and matured APIs. The spec should define the task
  // runner explicitly.
  // The task runner may be throttled.
  kMiscPlatformAPI = 22,

  // Tasks used for DedicatedWorker's requestAnimationFrame.
  kWorkerAnimation = 51,

  // Obsolete.
  // kNetworkingWithURLLoaderAnnotation = 50, (see crbug.com/860545)
  // kExperimentalWebSchedulingUserInteraction = 53,
  // kExperimentalWebSchedulingBestEffort = 54,

  // https://drafts.csswg.org/css-font-loading/#task-source
  kFontLoading = 56,

  // https://w3c.github.io/manifest/#dfn-application-life-cycle-task-source
  kApplicationLifeCycle = 57,

  // https://wicg.github.io/background-fetch/#infrastructure
  kBackgroundFetch = 58,

  // https://www.w3.org/TR/permissions/
  kPermission = 59,

  // https://w3c.github.io/ServiceWorker/#dfn-client-message-queue
  kServiceWorkerClientMessage = 60,

  // https://w3c.github.io/web-locks/#web-locks-tasks-source
  kWebLocks = 66,

  // Task type used for the Prioritized Task Scheduling API
  // (https://wicg.github.io/scheduling-apis/#the-posted-task-task-source).
  // This task type should not be passed directly to
  // FrameScheduler::GetTaskRunner(); it is used indirectly by
  // WebSchedulingTaskQueues.
  kWebSchedulingPostedTask = 67,

  // https://w3c.github.io/screen-wake-lock/#dfn-screen-wake-lock-task-source
  kWakeLock = 76,

  // https://storage.spec.whatwg.org/#storage-task-source
  kStorage = 82,

  // https://www.w3.org/TR/clipboard-apis/#clipboard-task-source
  kClipboard = 85,

  // https://www.w3.org/TR/webnn/#ml-task-source
  kMachineLearning = 86,

  ///////////////////////////////////////
  // Not-speced tasks should use one of the following task types
  ///////////////////////////////////////

  // The default task type. The task may be throttled or paused.
  kInternalDefault = 23,

  // Tasks used for all tasks associated with loading page content.
  kInternalLoading = 24,

  // Tasks for tests or mock objects.
  kInternalTest = 26,

  // Tasks that are posting back the result from the WebCrypto task runner to
  // the Blink thread that initiated the call and holds the Promise. Tasks with
  // this type are posted by:
  // * //components/webcrypto
  kInternalWebCrypto = 27,

  // Tasks to execute media-related things like logging or playback. Tasks with
  // this type are mainly posted by:
  // * //content/renderer/media
  // * //media
  kInternalMedia = 29,

  // Tasks to execute things for real-time media processing like recording. If a
  // task touches MediaStreamTracks, associated sources/sinks, and Web Audio,
  // this task type should be used.
  // Tasks with this type are mainly posted by:
  // * //content/renderer/media
  // * //media
  // * blink/renderer/modules/webaudio
  // * blink/public/platform/audio
  kInternalMediaRealTime = 30,

  // Tasks related to user interaction like clicking or inputting texts.
  kInternalUserInteraction = 32,

  // Tasks related to the inspector.
  kInternalInspector = 33,

  // Obsolete.
  // kInternalWorker = 36,

  // Translation task that freezes when the frame is not visible.
  kInternalTranslation = 55,

  // Tasks used at IntersectionObserver.
  kInternalIntersectionObserver = 44,

  // Task used for ContentCapture.
  kInternalContentCapture = 61,

  // Navigation tasks and tasks which have to run in order with them, including
  // legacy IPCs and channel associated interfaces.
  // Note that the ordering between tasks related to different frames is not
  // always guaranteed - tasks belonging to different frames can be reordered
  // when one of the frames is frozen.
  // Note: all AssociatedRemotes/AssociatedReceivers should use this task type.
  kInternalNavigationAssociated = 63,

  // Tasks which should run when the frame is frozen, but otherwise should run
  // in order with other legacy IPC and channel-associated interfaces.
  // Only tasks related to unfreezing itself should run here, the majority of
  // the tasks
  // should use kInternalNavigationAssociated instead.
  kInternalNavigationAssociatedUnfreezable = 64,

  // Task used to split a script loading task for cooperative scheduling
  kInternalContinueScriptLoading = 65,

  // Tasks used to control frame lifecycle - they should run even when the frame
  // is frozen.
  kInternalFrameLifecycleControl = 68,

  // Tasks used for find-in-page.
  kInternalFindInPage = 70,

  // Tasks that come in on the HighPriorityLocalFrame interface.
  kInternalHighPriorityLocalFrame = 71,

  // Tasks that are should use input priority task queue/runner.
  kInternalInputBlocking = 77,

  // Tasks related to the WebGPU API
  kWebGPU = 78,

  // Cross-process PostMessage IPCs that are deferred in the current task.
  kInternalPostMessageForwarding = 79,

  // Tasks related to renderer-initiated navigation cancellation.
  kInternalNavigationCancellation = 80,

  // Tasks related to autofill.
  //
  // TODO(crbug.com/382342234): This was added to distinguish autofill tasks
  // from kInternalUserInteraction tasks to exclude them from task deferral
  // policies, but tasks with this type need to be synchronized with synchronous
  // submission. Remove this if the autofill tasks become synchronous.
  kInternalAutofill = 88,

  ///////////////////////////////////////
  // The following task types are only for thread-local queues.
  ///////////////////////////////////////

  // The following task types are internal-use only, escpecially for annotations
  // like UMA of per-thread task queues. Do not specify these task types when to
  // get a task queue/runner.

  kMainThreadTaskQueueV8 = 37,
  kMainThreadTaskQueueV8UserVisible = 84,
  kMainThreadTaskQueueV8BestEffort = 87,
  kMainThreadTaskQueueCompositor = 38,
  kMainThreadTaskQueueDefault = 39,
  kMainThreadTaskQueueInput = 40,
  kMainThreadTaskQueueIdle = 41,
  // Removed:
  // kMainThreadTaskQueueIPC = 42,
  kMainThreadTaskQueueControl = 43,
  // Removed:
  // kMainThreadTaskQueueCleanup = 52,
  kMainThreadTaskQueueMemoryPurge = 62,
  kMainThreadTaskQueueNonWaking = 69,
  kMainThreadTaskQueueIPCTracking = 74,
  kCompositorThreadTaskQueueDefault = 45,
  kCompositorThreadTaskQueueInput = 49,
  kWorkerThreadTaskQueueDefault = 46,
  kWorkerThreadTaskQueueV8 = 47,
  kWorkerThreadTaskQueueCompositor = 48,

  kMaxValue = kInternalAutofill,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_TASK_TYPE_H_
