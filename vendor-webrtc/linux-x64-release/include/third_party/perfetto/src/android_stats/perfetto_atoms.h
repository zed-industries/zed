/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_ANDROID_STATS_PERFETTO_ATOMS_H_
#define SRC_ANDROID_STATS_PERFETTO_ATOMS_H_

namespace perfetto {

// This must match the values of the PerfettoUploadEvent enum in:
// frameworks/proto_logging/stats/atoms.proto
enum class PerfettoStatsdAtom {
  kUndefined = 0,

  // Checkpoints inside perfetto_cmd before tracing is finished.
  kTraceBegin = 1,
  kBackgroundTraceBegin = 2,
  kCmdCloneTraceBegin = 55,
  kCmdCloneTriggerTraceBegin = 56,
  kOnConnect = 3,
  kCmdOnSessionClone = 58,
  kCmdOnTriggerSessionClone = 59,

  // Guardrails inside perfetto_cmd before tracing is finished.
  kOnTimeout = 16,

  // Checkpoints inside traced.
  kTracedEnableTracing = 37,
  kTracedStartTracing = 38,
  kTracedDisableTracing = 39,
  kTracedNotifyTracingDisabled = 40,

  // Trigger checkpoints inside traced.
  // These atoms are special because, along with the UUID,
  // they log the trigger name.
  kTracedTriggerStartTracing = 41,
  kTracedTriggerStopTracing = 42,
  kTracedTriggerCloneSnapshot = 53,

  // Guardrails inside traced.
  kTracedEnableTracingExistingTraceSession = 18,
  kTracedEnableTracingTooLongTrace = 19,
  kTracedEnableTracingInvalidTriggerTimeout = 20,
  kTracedEnableTracingDurationWithTrigger = 21,
  kTracedEnableTracingStopTracingWriteIntoFile = 22,
  kTracedEnableTracingDuplicateTriggerName = 23,
  kTracedEnableTracingInvalidDeferredStart = 24,
  kTracedEnableTracingInvalidBufferSize = 25,
  kTracedEnableTracingBufferSizeTooLarge = 26,
  kTracedEnableTracingTooManyBuffers = 27,
  kTracedEnableTracingDuplicateSessionName = 28,
  kTracedEnableTracingSessionNameTooRecent = 29,
  kTracedEnableTracingTooManySessionsForUid = 30,
  kTracedEnableTracingTooManyConcurrentSessions = 31,
  kTracedEnableTracingInvalidFdOutputFile = 32,
  kTracedEnableTracingFailedToCreateFile = 33,
  kTracedEnableTracingOom = 34,
  kTracedEnableTracingUnknown = 35,
  kTracedStartTracingInvalidSessionState = 36,
  kTracedEnableTracingInvalidFilter = 47,
  kTracedEnableTracingOobTargetBuffer = 48,
  kTracedEnableTracingInvalidTriggerMode = 52,
  kTracedEnableTracingInvalidBrFilename = 54,
  kTracedEnableTracingFailedSessionSemaphoreCheck = 57,

  // Checkpoints inside perfetto_cmd after tracing has finished.
  kOnTracingDisabled = 4,
  kFinalizeTraceAndExit = 11,
  kCmdFwReportBegin = 49,
  // Will be removed once incidentd is no longer used.
  kUploadIncidentBegin = 8,
  kNotUploadingEmptyTrace = 17,

  // Guardrails inside perfetto_cmd after tracing has finished.
  kCmdFwReportEmptyTrace = 50,
  // Will be removed once incidentd is no longer used.
  kUploadIncidentFailure = 10,

  // "Successful" terminal states inside perfetto_cmd.
  kCmdFwReportHandoff = 51,

  // Deprecated as "success" is misleading; it simply means we were
  // able to communicate with incidentd. Will be removed once
  // incidentd is no longer used.
  kUploadIncidentSuccess = 9,

  // Contained trigger begin/success/failure. Replaced by
  // |PerfettoTriggerAtom| to allow aggregation using a count metric
  // and reduce spam.
  // reserved 12, 13, 14;

  // Contained that a guardrail in perfetto_cmd was hit. Replaced with
  // kCmd* guardrails.
  // reserved 15;

  // Contained status of Dropbox uploads. Removed as Perfetto no
  // longer supports uploading traces using Dropbox.
  // reserved 5, 6, 7;

  // Contained status of guardrail state initialization and upload limit in
  // perfetto_cmd. Removed as perfetto no longer manages stateful guardrails
  // reserved 44, 45, 46;

  // Contained the guardrail for user build tracing. Removed as this guardrail
  // causes more problem than it solves these days.
  // reserved 43;
};

// This must match the values of the PerfettoTrigger::TriggerType enum in:
// frameworks/proto_logging/stats/atoms.proto
enum PerfettoTriggerAtom {
  kUndefined = 0,

  kTracedLimitProbability = 5,
  kTracedLimitMaxPer24h = 6,

  kTracedTrigger = 9,

  // Contained events of logging triggers through perfetto_cmd, probes and
  // trigger_perfetto.
  // Removed in W (Oct 2024) and replaced by |kTracedTrigger|.
  // reserved 1, 2, 3, 4, 7, 8
};

}  // namespace perfetto

#endif  // SRC_ANDROID_STATS_PERFETTO_ATOMS_H_
