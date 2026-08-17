/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_PERFETTO_CMD_PERFETTO_CMD_H_
#define SRC_PERFETTO_CMD_PERFETTO_CMD_H_

#include <cstdint>
#include <functional>
#include <list>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/base/build_config.h"
#include "perfetto/ext/base/event_fd.h"
#include "perfetto/ext/base/pipe.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/thread_task_runner.h"
#include "perfetto/ext/base/unix_task_runner.h"
#include "perfetto/ext/base/uuid.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/consumer.h"
#include "perfetto/ext/tracing/ipc/consumer_ipc_client.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "src/android_stats/perfetto_atoms.h"
#include "src/perfetto_cmd/packet_writer.h"

namespace perfetto {

// Directory for local state and temporary files. This is automatically
// created by the system by setting setprop persist.traced.enable=1.
extern const char* kStateDir;

class PerfettoCmd : public Consumer {
 public:
  PerfettoCmd();
  ~PerfettoCmd() override;

  // The main() is split in two stages: cmdline parsing and actual interaction
  // with traced. This is to allow tools like tracebox to avoid spawning the
  // service for no reason if the cmdline parsing fails.
  // Return value:
  //   std::nullopt: no error, the caller should call
  //   ConnectToServiceRunAndMaybeNotify.
  //   0-N: the caller should exit() with the given exit code.
  std::optional<int> ParseCmdlineAndMaybeDaemonize(int argc, char** argv);
  int ConnectToServiceRunAndMaybeNotify();

  // perfetto::Consumer implementation.
  void OnConnect() override;
  void OnDisconnect() override;
  void OnTracingDisabled(const std::string& error) override;
  void OnTraceData(std::vector<TracePacket>, bool has_more) override;
  void OnDetach(bool) override;
  void OnAttach(bool, const TraceConfig&) override;
  void OnTraceStats(bool, const TraceStats&) override;
  void OnObservableEvents(const ObservableEvents&) override;
  void OnSessionCloned(const OnSessionClonedArgs&) override;

  void SignalCtrlC() { ctrl_c_evt_.Notify(); }

 private:
  struct SnapshotTriggerInfo;

  enum CloneThreadMode { kSingleExtraThread, kNewThreadPerRequest };

  bool OpenOutputFile();
  void SetupCtrlCSignalHandler();
  void FinalizeTraceAndExit();
  void PrintUsage(const char* argv0);
  void PrintServiceState(bool success, const TracingServiceState&);
  void CloneAllBugreportTraces(bool success, const TracingServiceState&);

  void CloneSessionOnThread(TracingSessionID,
                            const std::string& cmdline,  // \0 separated.
                            CloneThreadMode,
                            const std::optional<SnapshotTriggerInfo>& trigger,
                            std::function<void()> on_clone_callback);
  void OnTimeout();
  bool is_detach() const { return !detach_key_.empty(); }
  bool is_attach() const { return !attach_key_.empty(); }
  bool is_clone() const {
    return clone_tsid_.has_value() || !clone_name_.empty();
  }

  // Once we call ReadBuffers we expect one or more calls to OnTraceData
  // with the last call having |has_more| set to false. However we should
  // gracefully handle the service failing to ever call OnTraceData or
  // setting |has_more| incorrectly. To do this we maintain a timeout
  // which finalizes and exits the client if we don't receive OnTraceData
  // within OnTraceDataTimeoutMs of when we expected to.
  void CheckTraceDataTimeout();

  int ConnectToServiceAndRun();

  void ReadbackTraceDataAndQuit(const std::string& error);

  enum BgProcessStatus : char {
    kBackgroundOk = 0,
    kBackgroundOtherError = 1,
    kBackgroundTimeout = 2,
  };

  // Used to implement the --background-wait flag.
  //
  // Waits (up to 30s) for the child process to signal (success or an error).
  //
  // Returns the status received from the child process or kTimeout, in case of
  // timeout.
  BgProcessStatus WaitOnBgProcessPipe();

  // Used to implement the --background-wait flag.
  //
  // Signals the parent process (if there is one) that it can exit (successfully
  // or with an error).
  //
  // Only the first time this function is called is significant. Further calls
  // will have no effect.
  void NotifyBgProcessPipe(BgProcessStatus status);

  void OnCloneSnapshotTriggerReceived(TracingSessionID,
                                      const SnapshotTriggerInfo& trigger);

#if PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID)
  static base::ScopedFile CreateUnlinkedTmpFile();
  void SaveTraceIntoIncidentOrCrash();
  void SaveOutputToIncidentTraceOrCrash();
  void ReportTraceToAndroidFrameworkOrCrash();
#endif
  void LogUploadEvent(PerfettoStatsdAtom atom);
  void LogUploadEvent(PerfettoStatsdAtom atom, const std::string& trigger_name);
  void LogTriggerEvents(PerfettoTriggerAtom atom,
                        const std::vector<std::string>& trigger_names);

  base::UnixTaskRunner task_runner_;

  std::unique_ptr<perfetto::TracingService::ConsumerEndpoint>
      consumer_endpoint_;
  std::unique_ptr<TraceConfig> trace_config_;
  std::optional<PacketWriter> packet_writer_;
  base::ScopedFstream trace_out_stream_;
  std::vector<std::string> triggers_to_activate_;
  std::string trace_out_path_;
  base::EventFd ctrl_c_evt_;
  bool ctrl_c_handler_installed_ = false;
  base::Pipe background_wait_pipe_;
  bool save_to_incidentd_ = false;
  bool report_to_android_framework_ = false;
  bool statsd_logging_ = false;
  bool tracing_succeeded_ = false;
  uint64_t bytes_written_ = 0;
  std::string detach_key_;
  std::string attach_key_;
  bool stop_trace_once_attached_ = false;
  bool redetach_once_attached_ = false;
  bool query_service_ = false;
  bool query_service_output_raw_ = false;
  bool query_service_long_ = false;
  bool clone_all_bugreport_traces_ = false;
  bool bugreport_ = false;
  bool background_ = false;
  bool background_wait_ = false;
  bool ignore_guardrails_ = false;
  bool upload_flag_ = false;
  bool connected_ = false;
  std::string uuid_;
  std::optional<TracingSessionID> clone_tsid_{};
  std::string clone_name_;
  bool clone_for_bugreport_ = false;
  std::function<void()> on_session_cloned_;

  // How long we expect to trace for or 0 if the trace is indefinite.
  uint32_t expected_duration_ms_ = 0;
  bool trace_data_timeout_armed_ = false;

  // The aux threads used to invoke secondary instances of PerfettoCmd to create
  // snapshots. This is used only when the trace config involves a
  // CLONE_SNAPSHOT trigger or when using --save-all-for-bugreport.
  std::list<base::ThreadTaskRunner> snapshot_threads_;
  int snapshot_count_ = 0;
  std::string snapshot_config_;
  // If the trigger caused the clone operation, we want to save the information
  // about that trigger to the trace We may get multiple triggers with the same
  // name, so we pass the entire structure to uniquely identify the trigger
  // later. This structure is identical to the
  // `TracingServiceImpl::TriggerInfo`.
  struct SnapshotTriggerInfo {
    uint64_t boot_time_ns = 0;
    std::string trigger_name;
    std::string producer_name;
    uid_t producer_uid = 0;
    uint64_t trigger_delay_ms = 0;
  };
  std::optional<SnapshotTriggerInfo> snapshot_trigger_info_;

  base::WeakPtrFactory<PerfettoCmd> weak_factory_{this};
};

}  // namespace perfetto

#endif  // SRC_PERFETTO_CMD_PERFETTO_CMD_H_
