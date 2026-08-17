// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Declares CalculatorGraph, which links Calculators into a directed acyclic
// graph, and allows its evaluation.

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_GRAPH_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_GRAPH_H_

#include <atomic>
#include <functional>
#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_base.h"
#include "mediapipe/framework/calculator_node.h"
#include "mediapipe/framework/counter_factory.h"
#include "mediapipe/framework/executor.h"
#include "mediapipe/framework/graph_output_stream.h"
#include "mediapipe/framework/graph_runtime_info.pb.h"
#include "mediapipe/framework/graph_service.h"
#include "mediapipe/framework/graph_service_manager.h"
#include "mediapipe/framework/mediapipe_profiling.h"
#include "mediapipe/framework/output_side_packet_impl.h"
#include "mediapipe/framework/output_stream_manager.h"
#include "mediapipe/framework/output_stream_poller.h"
#include "mediapipe/framework/output_stream_shard.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_generator_graph.h"
#include "mediapipe/framework/resources_service.h"
#include "mediapipe/framework/scheduler.h"
#include "mediapipe/framework/scheduler_shared.h"
#include "mediapipe/framework/subgraph.h"
#include "mediapipe/framework/thread_pool_executor.pb.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/framework/validated_graph_config.h"

#if !defined(__EMSCRIPTEN__)
#include "mediapipe/framework/tool/graph_runtime_info_logger.h"
#endif  // !defined(__EMSCRIPTEN__)

namespace mediapipe {

#if !MEDIAPIPE_DISABLE_GPU
class GpuResources;
struct GpuSharedData;
#endif  // !MEDIAPIPE_DISABLE_GPU

typedef absl::StatusOr<OutputStreamPoller> StatusOrPoller;

// The class representing a DAG of calculator nodes.
//
// CalculatorGraph is the primary API for the MediaPipe Framework.
// In general, CalculatorGraph should be used if the only thing you need
// to do is run the graph (without pushing data in or extracting it as
// the graph runs).
//
// Example:
//   // Build dependency "//mediapipe/framework:calculator_framework".
//
//   #include "mediapipe/framework/calculator_framework.h"
//
//   mediapipe::CalculatorGraphConfig config;
//   MP_RETURN_IF_ERROR(mediapipe::tool::ParseGraphFromString(kGraphStr,
//   &config)); mediapipe::CalculatorGraph graph;
//   MP_RETURN_IF_ERROR(graph.Initialize(config));
//
//   std::map<std::string, mediapipe::Packet> extra_side_packets;
//   extra_side_packets["video_id"] = mediapipe::MakePacket<std::string>(
//       "3edb9503834e9b42");
//   MP_RETURN_IF_ERROR(graph.Run(extra_side_packets));
//
//   // Run again (demonstrating the asynchronous StartRun call with more
//   // concise initializer list syntax).
//   MP_RETURN_IF_ERROR(graph.StartRun(
//       {{"video_id", mediapipe::MakePacket<std::string>("Ex-uGhDzue4")}}));
//   // See mediapipe/framework/graph_runner.h for an interface
//   // to insert and extract packets from a graph as it runs.
//   // Once it is done using the graph, close its streams and wait till done.
//   MP_RETURN_IF_ERROR(graph->CloseAllInputStreams());
//   MP_RETURN_IF_ERROR(graph->WaitUntilDone());
class CalculatorGraph {
 public:
  // Defines possible modes for adding a packet to a graph input stream.
  // WAIT_TILL_NOT_FULL can be used to control the memory usage of a graph by
  // avoiding adding a new packet until all dependent input streams fall below
  // the maximum queue size specified in the graph configuration.
  // ADD_IF_NOT_FULL could also be used to control the latency if used in a
  // real-time graph (e.g. drop camera frames if the MediaPipe graph queues are
  // full).
  enum class GraphInputStreamAddMode {
    // Blocks and waits until none of the affected streams
    // are full. Note that if max_queue_size is set to -1, the packet will be
    // added regardless of queue size.
    WAIT_TILL_NOT_FULL,
    // Returns and does not add packet if any affected input
    // stream is full.
    ADD_IF_NOT_FULL
  };

  // Creates an uninitialized graph.
  CalculatorGraph();
  CalculatorGraph(const CalculatorGraph&) = delete;
  CalculatorGraph& operator=(const CalculatorGraph&) = delete;

  // Initializes the graph from its proto description (using Initialize())
  // and crashes if something goes wrong.
  explicit CalculatorGraph(CalculatorGraphConfig config);

  // Initializes the graph with shared GraphServices from an existing MP graph
  // environment. It enables to run a CalculatorGraph within a Calculator.
  explicit CalculatorGraph(CalculatorContext* cc);

  virtual ~CalculatorGraph();

  // Initializes the graph from a its proto description.
  // side_packets that are provided at this stage are common across all Run()
  // invocations and could be used to execute PacketGenerators immediately.
  absl::Status Initialize(CalculatorGraphConfig config,
                          const std::map<std::string, Packet>& side_packets);

  // Convenience version which does not take side packets.
  absl::Status Initialize(CalculatorGraphConfig config);

  // Initializes the CalculatorGraph from the specified graph and subgraph
  // configs.  Template graph and subgraph configs can be specified through
  // |input_templates|.  Every subgraph must have its graph type specified in
  // CalclatorGraphConfig.type.  A subgraph can be instantiated directly by
  // specifying its type in |graph_type|.  A template graph can be instantiated
  // directly by specifying its template arguments in |options|.
  absl::Status Initialize(
      const std::vector<CalculatorGraphConfig>& configs,
      const std::vector<CalculatorGraphTemplate>& templates,
      const std::map<std::string, Packet>& side_packets = {},
      const std::string& graph_type = "",
      const Subgraph::SubgraphOptions* options = nullptr);

  // Returns the canonicalized CalculatorGraphConfig for this graph.
  const CalculatorGraphConfig& Config() const {
    return validated_graph_->Config();
  }

  // Observes the named output stream. packet_callback will be invoked on every
  // packet emitted by the output stream. Can only be called before Run() or
  // StartRun(). It is possible for packet_callback to be called until the
  // object is destroyed, even if e.g. Cancel() or WaitUntilDone() have already
  // been called. After this object is destroyed so is packet_callback.
  // TODO: Rename to AddOutputStreamCallback.
  //
  // Note: use `SetErrorCallback` to subscribe for errors when using graph for
  // async use cases.
  absl::Status ObserveOutputStream(
      const std::string& stream_name,
      std::function<absl::Status(const Packet&)> packet_callback,
      bool observe_timestamp_bounds = false);

  // Adds an OutputStreamPoller for a stream. This provides a synchronous,
  // polling API for accessing a stream's output. Should only be called before
  // Run() or StartRun(). For asynchronous output, use ObserveOutputStream. See
  // also the helpers in tool/sink.h.
  StatusOrPoller AddOutputStreamPoller(const std::string& stream_name,
                                       bool observe_timestamp_bounds = false);

  // Gets output side packet by name. The output side packet can be successfully
  // retrevied in one of the following situations:
  //   - The graph is done.
  //   - The output side packet has been generated by a calculator and the graph
  //     is currently idle.
  //   - The side packet is a base packet generated by a PacketGenerator.
  // Returns error if the the output side packet is not found or empty.
  absl::StatusOr<Packet> GetOutputSidePacket(const std::string& packet_name);

  // Runs the graph after adding the given extra input side packets.  All
  // arguments are forgotten after Run() returns.
  // Run() is a blocking call and will return when all calculators are done.
  virtual absl::Status Run(
      const std::map<std::string, Packet>& extra_side_packets);

  // Run the graph without adding any input side packets.
  absl::Status Run() { return Run({}); }

  // Start a run of the graph.  StartRun, WaitUntilDone, Cancel, HasError,
  // AddPacketToInputStream, and CloseInputStream allow more control over
  // the execution of the graph run.  You can insert packets directly into
  // a stream while the graph is running. Once StartRun has been called,
  // the graph will continue to run until all work is either done or canceled,
  // meaning that either WaitUntilDone() or Cancel() has been called and has
  // completed. If StartRun returns an error, then the graph is not started and
  // a subsequent call to StartRun can be attempted.
  //
  // Example:
  //   MP_RETURN_IF_ERROR(graph.StartRun(...));
  //   while (true) {
  //     if (graph.HasError() || want_to_stop) break;
  //     MP_RETURN_IF_ERROR(graph.AddPacketToInputStream(...));
  //   }
  //   for (const std::string& stream : streams) {
  //     MP_RETURN_IF_ERROR(graph.CloseInputStream(stream));
  //   }
  //   MP_RETURN_IF_ERROR(graph.WaitUntilDone());
  absl::Status StartRun(
      const std::map<std::string, Packet>& extra_side_packets) {
    return StartRun(extra_side_packets, {});
  }

  // In addition to the above StartRun, add additional parameter to set the
  // stream header before running.
  // Note: We highly discourage the use of stream headers, this is added for the
  // compatibility of existing calculators that use headers during Open().
  absl::Status StartRun(const std::map<std::string, Packet>& extra_side_packets,
                        const std::map<std::string, Packet>& stream_headers);

  // Wait for the current run to finish (block the current thread
  // until all source calculators have returned StatusStop(), all
  // graph_input_streams_ have been closed, and no more calculators can
  // be run). This function can be called only after StartRun(). If you want to
  // stop the run quickly, without waiting for all the work in progress to
  // finish, see Cancel(). The graph cannot be destroyed until all work is
  // either done or canceled, meaning that either WaitUntilDone() or Cancel()
  // has been called and completed.
  absl::Status WaitUntilDone();

  // Wait until the running graph is in the idle mode, which is when nothing can
  // be scheduled and nothing is running in the worker threads. This function
  // can be called only after StartRun().
  //
  // NOTE: The graph must not have any source nodes because source nodes prevent
  // the running graph from becoming idle until the source nodes are done.
  // Currently, `WaitUntilIdle` cannot be used reliably on graphs with any
  // source nodes.
  absl::Status WaitUntilIdle();

  // Wait until a packet is emitted on one of the observed output streams.
  // Returns immediately if a packet has already been emitted since the last
  // call to this function.
  // Returns OutOfRangeError if the graph terminated while waiting.
  absl::Status WaitForObservedOutput();

  // Quick non-locking means of checking if the graph has encountered an error.
  bool HasError() const { return has_error_; }

  // Returns debugging information about the graph transient state, including
  // information about all input streams and their timestamp bounds. This method
  // is thread safe and can be called from any thread.
  absl::StatusOr<GraphRuntimeInfo> GetGraphRuntimeInfo();

  // Add a Packet to a graph input stream based on the graph input stream add
  // mode. If the mode is ADD_IF_NOT_FULL, the packet will not be added if any
  // queue exceeds max_queue_size specified by the graph config and will return
  // StatusUnavailable. The WAIT_TILL_NOT_FULL mode (default) will block until
  // the queues fall below the max_queue_size before adding the packet. If the
  // mode is max_queue_size is -1, then the packet is added regardless of the
  // sizes of the queues in the graph. The input stream must have been specified
  // in the configuration as a graph level input_stream. On error, nothing is
  // added.
  absl::Status AddPacketToInputStream(absl::string_view stream_name,
                                      const Packet& packet);

  // Same as the l-value version of this function by the same name, but moves
  // the r-value referenced packet into the stream instead of copying it over.
  // This allows the graph to take exclusive ownership of the packet, which may
  // allow more memory optimizations. Note that, if an error is returned, the
  // packet may remain valid.  In particular, when using the ADD_IF_NOT_FULL
  // mode with a full queue, this will return StatusUnavailable and the caller
  // may try adding the packet again later.
  absl::Status AddPacketToInputStream(absl::string_view stream_name,
                                      Packet&& packet);

  // Indicates that input will arrive no earlier than a certain timestamp.
  absl::Status SetInputStreamTimestampBound(const std::string& stream_name,
                                            Timestamp timestamp);

  // Sets the queue size of a graph input stream, overriding the graph default.
  absl::Status SetInputStreamMaxQueueSize(const std::string& stream_name,
                                          int max_queue_size);

  // Check if an input stream exists in the graph
  bool HasInputStream(const std::string& name);

  // Close a graph input stream.  If the graph has any graph input streams
  // then Run() will not return until all the graph input streams have
  // been closed (and all packets propagate through the graph).
  // Note that multiple threads cannot call CloseInputStream() on the same
  // stream_name at the same time.
  absl::Status CloseInputStream(const std::string& stream_name);

  // Closes all the graph input streams.
  absl::Status CloseAllInputStreams();

  // Closes all the graph input streams and source calculator nodes.
  absl::Status CloseAllPacketSources();

  // Returns the pointer to the stream with the given name, or dies if none
  // exists. The result remains owned by the CalculatorGraph.
  ABSL_DEPRECATED(
      "Prefer using a Calculator to get information of all sorts out of the "
      "graph.")
  const OutputStreamManager* FindOutputStreamManager(const std::string& name);

  // Returns the ProfilingContext assocoaited with the CalculatorGraph.
  ProfilingContext* profiler() { return profiler_.get(); }
  // Collects the runtime profile for Open(), Process(), and Close() of each
  // calculator in the graph. May be called at any time after the graph has been
  // initialized.
  ABSL_DEPRECATED("Use profiler()->GetCalculatorProfiles() instead")
  absl::Status GetCalculatorProfiles(std::vector<CalculatorProfile>*) const;

  // Set the type of counter used in this graph.
  void SetCounterFactory(CounterFactory* factory) {
    counter_factory_.reset(factory);
  }
  CounterFactory* GetCounterFactory() { return counter_factory_.get(); }

  // Sets the error callback to receive graph execution errors when blocking
  // calls like `WaitUntilIdle()`, `WaitUntilDone()` cannot be used.
  //
  // Useful for async graph use cases: e.g. user entering words and each
  // word is sent to the graph while graph outputs are received and rendered
  // asynchronously.
  //
  // NOTE:
  // - Must be called before graph is initialized.
  // - May be executed from multiple threads.
  // - Errors are first processed by the graph, then the graph transitions into
  //   the error state, and then finally the callback is invoked.
  absl::Status SetErrorCallback(
      std::function<void(const absl::Status&)> error_callback);

  // Callback when an error is encountered.
  // Adds the error to the vector of errors.
  //
  // Use `SetErrorCallback` to subscribe for errors when using graph for async
  // use cases.
  void RecordError(const absl::Status& error) ABSL_LOCKS_EXCLUDED(error_mutex_);

  // Combines errors into a status. Returns true if the vector of errors is
  // non-empty.
  bool GetCombinedErrors(const std::string& error_prefix,
                         absl::Status* error_status);
  // Convenience overload which specifies a default error prefix.
  bool GetCombinedErrors(absl::Status* error_status);

  // Returns the maximum input stream queue size.
  int GetMaxInputStreamQueueSize();

  // Get the mode for adding packets to an input stream.
  GraphInputStreamAddMode GetGraphInputStreamAddMode() const;

  // Set the mode for adding packets to an input stream.
  void SetGraphInputStreamAddMode(GraphInputStreamAddMode mode);

  // Aborts the scheduler if the graph is not terminated; no-op otherwise. Does
  // not wait for all work in progress to finish. To stop the run and wait for
  // work in progress to finish, see CloseAllInputStreams() and WaitUntilDone().
  // The graph cannot be destroyed until all work is either done or canceled,
  // meaning that either WaitUntilDone() or Cancel() has been called and
  // completed.
  void Cancel();

  // Pauses the scheduler. Only used by calculator graph testing.
  ABSL_DEPRECATED(
      "CalculatorGraph will not allow external callers to explictly pause and "
      "resume a graph.")
  void Pause();

  // Resumes the scheduler. Only used by calculator graph testing.
  ABSL_DEPRECATED(
      "CalculatorGraph will not allow external callers to explictly pause and "
      "resume a graph.")
  void Resume();

  // Sets the executor that will run the nodes assigned to the executor
  // named |name|. If |name| is empty, this sets the default executor. Must
  // be called before the graph is initialized.
  absl::Status SetExecutor(const std::string& name,
                           std::shared_ptr<Executor> executor);

  // WARNING: the following public methods are exposed to Scheduler only.

  // Return true if all the graph input streams have been closed.
  bool GraphInputStreamsClosed() {
    return num_closed_graph_input_streams_ == graph_input_streams_.size();
  }

  // Returns true if this node or graph input stream is connected to
  // any input stream whose queue has hit maximum capacity.
  bool IsNodeThrottled(int node_id)
      ABSL_LOCKS_EXCLUDED(full_input_streams_mutex_);

  // If any active source node or graph input stream is throttled and not yet
  // closed, increases the max_queue_size for each full input stream in the
  // graph.
  // Returns true if at least one max_queue_size has been grown.
  bool UnthrottleSources() ABSL_LOCKS_EXCLUDED(full_input_streams_mutex_);

  // Returns the scheduler's runtime measures for overhead measurement.
  // Only meant for test purposes.
  internal::SchedulerTimes GetSchedulerTimes() {
    return scheduler_.GetSchedulerTimes();
  }

#if !MEDIAPIPE_DISABLE_GPU
  // Returns a pointer to the GpuResources in use, if any.
  // Only meant for internal use.
  std::shared_ptr<GpuResources> GetGpuResources() const;

  absl::Status SetGpuResources(std::shared_ptr<GpuResources> resources);
#endif  // !MEDIAPIPE_DISABLE_GPU

  // Sets a service object, essentially a graph-level singleton, which can be
  // accessed by calculators and subgraphs without requiring an explicit
  // connection.
  //
  // NOTE: must be called before `Initialize`, so subgraphs can access services
  // as well, as graph expansion happens during initialization.
  template <typename T>
  absl::Status SetServiceObject(const GraphService<T>& service,
                                std::shared_ptr<T> object) {
    if (initialized_) {
      // TODO: check that the graph has not been initialized for
      // all services!
      if (service.key == kResourcesService.key) {
        return absl::InternalError(
            "Service objects must be set before graph is initialized.");
      }
    }

    return service_manager_.SetServiceObject(service, object);
  }

  template <typename T>
  std::shared_ptr<T> GetServiceObject(const GraphService<T>& service) {
    return service_manager_.GetServiceObject(service);
  }

  // Disallows/disables default initialization of MediaPipe graph services.
  //
  // IMPORTANT: MediaPipe graph serices, essentially a graph-level singletons,
  // are designed in the way, so they may provide default initialization. For
  // example, this allows to run OpenGL processing wihtin the graph without
  // provinging a praticular OpenGL context as it can be provided by
  // default-initializable `kGpuService`. (One caveat here, you may still need
  // to initialize it manually to share graph context with external context.)
  //
  // Even if calculators require some service optionally
  // (`calculator_contract->UseService(kSomeService).Optional()`), it will be
  // still initialized if it allows default initialization.
  //
  // So far, in rare cases, this may be unwanted and strict control of what
  // services are allowed in the graph can be achieved by calling this method,
  // following `SetServiceObject` call for services which are allowed in the
  // graph.
  //
  // Recommendation: do not use unless you have to (for example, default
  // initialization has side effects)
  //
  // NOTE: must be called before `StartRun`/`Run`, where services are checked
  // and can be default-initialized.
  absl::Status DisallowServiceDefaultInitialization() {
    allow_service_default_initialization_ = false;
    return absl::OkStatus();
  }

  // Sets a service object, essentially a graph-level singleton, which can be
  // accessed by calculators and subgraphs without requiring an explicit
  // connection.
  //
  // NOTE: must be called before `Initialize`, so subgraphs can access services
  // as well, as graph expansion happens during initialization.
  //
  // Only the Java API should call this directly.
  absl::Status SetServicePacket(const GraphServiceBase& service, Packet p) {
    // TODO: check that the graph has not been started!
    return service_manager_.SetServicePacket(service, p);
  }

 private:
  // GraphRunState is used as a parameter in the function CallStatusHandlers.
  enum class GraphRunState {
    // State of the graph before the run; see status_handler.h for details.
    PRE_RUN,
    // State of the graph after after the run; set by CleanUpAfterRun.
    POST_RUN,
  };

  // The graph input streams (which have packets added to them from
  // outside the graph).  Since these will be connected directly to a
  // node's input streams they are implemented as "output" streams.
  // Based on the assumption that all the graph input packets must be added to a
  // graph input stream sequentially, a GraphInputStream object only contains
  // one reusable output stream shard.
  class GraphInputStream {
   public:
    explicit GraphInputStream(OutputStreamManager* manager)
        : manager_(manager) {
      shard_.SetSpec(manager_->Spec());
    }

    void PrepareForRun(std::function<void(absl::Status)> error_callback) {
      manager_->PrepareForRun(std::move(error_callback));
    }

    void SetMaxQueueSize(int max_queue_size) {
      manager_->SetMaxQueueSize(max_queue_size);
    }

    void SetHeader(const Packet& header);

    void AddPacket(const Packet& packet) { shard_.AddPacket(packet); }

    void AddPacket(Packet&& packet) { shard_.AddPacket(std::move(packet)); }

    void SetNextTimestampBound(Timestamp timestamp);

    void PropagateUpdatesToMirrors();

    void Close();

    bool IsClosed() const { return manager_->IsClosed(); }

    OutputStreamManager* GetManager() { return manager_; }

   private:
    OutputStreamManager* manager_ = nullptr;
    OutputStreamShard shard_;
  };

  // Initializes the graph from a ValidatedGraphConfig object.
  absl::Status Initialize(std::unique_ptr<ValidatedGraphConfig> validated_graph,
                          const std::map<std::string, Packet>& side_packets);

  // AddPacketToInputStreamInternal template is called by either
  // AddPacketToInputStream(Packet&& packet) or
  // AddPacketToInputStream(const Packet& packet).
  template <typename T>
  absl::Status AddPacketToInputStreamInternal(absl::string_view stream_name,
                                              T&& packet);

  // Sets the executor that will run the nodes assigned to the executor
  // named |name|.  If |name| is empty, this sets the default executor.
  // Does not check that the graph is uninitialized and |name| is not a
  // reserved executor name.
  absl::Status SetExecutorInternal(const std::string& name,
                                   std::shared_ptr<Executor> executor);

  // If the num_threads field in default_executor_options is not specified,
  // assigns a reasonable value based on system configuration and the graph.
  // Then, creates the default thread pool if appropriate.
  //
  // Only called by InitializeExecutors().
  absl::Status InitializeDefaultExecutor(
      const ThreadPoolExecutorOptions* default_executor_options,
      bool use_application_thread);

  // Creates a thread pool as the default executor. The num_threads argument
  // overrides the num_threads field in default_executor_options.
  absl::Status CreateDefaultThreadPool(
      const ThreadPoolExecutorOptions* default_executor_options,
      int num_threads);

  // Returns true if |name| is a reserved executor name.
  static bool IsReservedExecutorName(const std::string& name);

  // Helper functions for Initialize().
  absl::Status InitializeExecutors();
  absl::Status InitializePacketGeneratorGraph(
      const std::map<std::string, Packet>& side_packets);
  absl::Status InitializeStreams();
  absl::Status InitializeProfiler();
  absl::Status InitializeCalculatorNodes();
  absl::Status InitializePacketGeneratorNodes(
      const std::vector<int>& non_scheduled_generators);

  // Iterates through all nodes and schedules any that can be opened.
  void ScheduleAllOpenableNodes();

  // Does the bulk of the work for StartRun but does not start the scheduler.
  absl::Status PrepareForRun(
      const std::map<std::string, Packet>& extra_side_packets,
      const std::map<std::string, Packet>& stream_headers);

  absl::Status PrepareServices();

#if !MEDIAPIPE_DISABLE_GPU
  absl::Status MaybeSetUpGpuServiceFromLegacySidePacket(Packet legacy_sp);
  // Helper for PrepareForRun. If it returns a non-empty map, those packets
  // must be added to the existing side packets, replacing existing values
  // that have the same key.
  std::map<std::string, Packet> MaybeCreateLegacyGpuSidePacket(
      Packet legacy_sp);
  absl::Status PrepareGpu();
#endif  // !MEDIAPIPE_DISABLE_GPU

  // Cleans up any remaining state after the run and returns any errors that may
  // have occurred during the run. Called after the scheduler has terminated.
  absl::Status FinishRun();

  // Cleans up any remaining state after the run. All status handlers run here
  // if their requested input side packets exist.
  // The original |*status| is passed to all the status handlers. If any status
  // handler fails, it appends its error to errors_, and CleanupAfterRun sets
  // |*status| to the new combined errors on return.
  void CleanupAfterRun(absl::Status* status) ABSL_LOCKS_EXCLUDED(error_mutex_);

  // Calls HandlePreRunStatus or HandleStatus on the StatusHandlers. Which one
  // is called depends on the GraphRunState parameter (PRE_RUN or POST_RUN).
  // current_run_side_packets_ must be set before this function is called.
  // On error, has_error_ will be set.
  void CallStatusHandlers(GraphRunState graph_run_state,
                          const absl::Status& status);

  // Callback function to throttle or unthrottle source nodes when a stream
  // becomes full or non-full. A node is throttled (i.e. prevented being
  // scheduled) if it has caused a downstream input queue to become full. Note
  // that all sources (including graph input streams) that affect this stream
  // will be throttled. A node is unthrottled (i.e. added to the scheduler
  // queue) if all downstream input queues have become non-full.
  //
  // This method is invoked from an input stream when its queue becomes full or
  // non-full. However, since streams are not allowed to hold any locks while
  // invoking a callback, this method must re-lock the stream and query its
  // status before taking any action.
  void UpdateThrottledNodes(InputStreamManager* stream, bool* stream_was_full);

  // Returns a comma-separated list of source nodes.
  std::string ListSourceNodes() const;

  // Returns a parent node name for the given input stream.
  std::string GetParentNodeDebugName(InputStreamManager* stream) const;

#if !MEDIAPIPE_DISABLE_GPU
  // Owns the legacy GpuSharedData if we need to create one for backwards
  // compatibility.
  std::unique_ptr<GpuSharedData> legacy_gpu_shared_;
#endif  // !MEDIAPIPE_DISABLE_GPU

  // True if the graph was initialized.
  bool initialized_ = false;

  // A packet type that has SetAny() called on it.
  PacketType any_packet_type_;

  // The ValidatedGraphConfig object defining this CalculatorGraph.
  std::unique_ptr<ValidatedGraphConfig> validated_graph_;

  // The PacketGeneratorGraph to use to generate all the input side packets.
  PacketGeneratorGraph packet_generator_graph_;

  // True if the graph has source nodes.
  bool has_sources_ = false;

  // A flat array of InputStreamManager/OutputStreamManager/
  // OutputSidePacketImpl/CalculatorNode corresponding to the input/output
  // stream indexes, output side packet indexes, and calculator indexes
  // respectively in validated_graph_.
  // Once allocated these structures must not be reallocated since
  // internal structures may point to individual entries in the array.
  std::unique_ptr<InputStreamManager[]> input_stream_managers_;
  std::unique_ptr<OutputStreamManager[]> output_stream_managers_;
  std::unique_ptr<OutputSidePacketImpl[]> output_side_packets_;
  std::vector<std::unique_ptr<CalculatorNode>> nodes_;
  bool packet_generator_nodes_added_ = false;

  // The graph output streams.
  std::vector<std::shared_ptr<internal::GraphOutputStream>>
      graph_output_streams_;

  // Maximum queue size for an input stream. This is used by the scheduler to
  // restrict memory usage.
  int max_queue_size_ = -1;

  // Mode for adding packets to a graph input stream. Set to block until all
  // affected input streams are not full by default.
  GraphInputStreamAddMode graph_input_stream_add_mode_
      ABSL_GUARDED_BY(full_input_streams_mutex_);

  // For a source node or graph input stream (specified using id),
  // this stores the set of dependent input streams that have hit their
  // maximum capacity. Graph input streams are also treated as nodes.
  // A node is scheduled only if this set is empty.  Similarly, a packet
  // is added to a graph input stream only if this set is empty.
  // Note that this vector contains an unused entry for each non-source node.
  std::vector<absl::flat_hash_set<InputStreamManager*>> full_input_streams_
      ABSL_GUARDED_BY(full_input_streams_mutex_);

  // Input stream to index within `input_stream_managers_` mapping.
  absl::flat_hash_map<InputStreamManager*, int> input_stream_to_index_;

  // Maps stream names to graph input stream objects.
  absl::flat_hash_map<std::string, std::unique_ptr<GraphInputStream>>
      graph_input_streams_;

  // Maps graph input streams to their virtual node ids.
  absl::flat_hash_map<std::string, int> graph_input_stream_node_ids_;

  // Maps graph input streams to their max queue size.
  absl::flat_hash_map<std::string, int> graph_input_stream_max_queue_size_;

  // The factory for making counters associated with this graph.
  std::unique_ptr<CounterFactory> counter_factory_;

  // Executors for the scheduler, keyed by the executor's name. The default
  // executor's name is the empty string.
  std::map<std::string, std::shared_ptr<Executor>> executors_;

  // The processed input side packet map for this run.
  std::map<std::string, Packet> current_run_side_packets_;

  // Object to manage graph services.
  GraphServiceManager service_manager_;

  // Indicates whether service default initialization is allowed.
  bool allow_service_default_initialization_ = true;

  // Vector of errors encountered while running graph. Always use RecordError()
  // to add an error to this vector.
  std::vector<absl::Status> errors_ ABSL_GUARDED_BY(error_mutex_);

  // Optional error callback set by client.
  std::function<void(const absl::Status&)> error_callback_;

  // True if the default executor uses the application thread.
  bool use_application_thread_ = false;

  // Condition variable that waits until all input streams that depend on a
  // graph input stream are below the maximum queue size.
  absl::CondVar wait_to_add_packet_cond_var_
      ABSL_GUARDED_BY(full_input_streams_mutex_);

  // Mutex for the vector of errors.
  absl::Mutex error_mutex_;

  // Status variable to indicate if the graph has encountered an error.
  std::atomic<bool> has_error_;

  // Mutex for full_input_streams_.
  mutable absl::Mutex full_input_streams_mutex_;

  // Number of closed graph input streams. This is a separate variable because
  // it is not safe to hold a lock on the scheduler while calling Close() on an
  // input stream. Hence, we decouple the closing of the stream and checking its
  // status.
  // TODO: update this comment.
  std::atomic<unsigned int> num_closed_graph_input_streams_;

  // The graph tracing and profiling interface.  It is owned by the
  // CalculatorGraph using a shared_ptr in order to allow threadsafe access
  // to the ProfilingContext from clients that may outlive the CalculatorGraph
  // such as GlContext.  It is declared here before the Scheduler so that it
  // remains available during the Scheduler destructor.
  std::shared_ptr<ProfilingContext> profiler_;

  internal::Scheduler scheduler_;

#if !defined(__EMSCRIPTEN__)
  // Collects runtime information about the graph in the background.
  tool::GraphRuntimeInfoLogger graph_runtime_info_logger_;
#endif  // !defined(__EMSCRIPTEN__)
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_GRAPH_H_
