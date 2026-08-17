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

#ifndef MEDIAPIPE_FRAMEWORK_GRAPH_OUTPUT_STREAM_H_
#define MEDIAPIPE_FRAMEWORK_GRAPH_OUTPUT_STREAM_H_

#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/thread_annotations.h"
#include "absl/log/absl_log.h"
#include "absl/strings/substitute.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/input_stream_handler.h"
#include "mediapipe/framework/input_stream_manager.h"
#include "mediapipe/framework/output_stream_manager.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {

class InputStreamHandler;

namespace internal {

// A base class of OutputStreamObserver and OutputStreamPollerImpl.
class GraphOutputStream {
 public:
  virtual ~GraphOutputStream() {}

  // Initializes an input stream handler that only manages one
  // input stream and attaches the input stream to an output stream as
  // the mirror for observation/polling.  Ownership of output_stream_manager
  // is not transferred to the graph output stream object.
  absl::Status Initialize(const std::string& stream_name,
                          const PacketType* packet_type,
                          OutputStreamManager* output_stream_manager,
                          bool observe_timestamp_bounds = false);

  // Installs callbacks into its GraphOutputStreamHandler.
  virtual void PrepareForRun(std::function<void()> notification_callback,
                             std::function<void(absl::Status)> error_callback);

  // Notifies the graph output stream of new packets emitted by the output
  // stream.
  virtual absl::Status Notify() = 0;

  // Notifies the graph output stream of the errors in the calculator graph.
  virtual void NotifyError() = 0;

  InputStreamManager* input_stream() { return input_stream_.get(); }

 protected:
  // A simple input stream handler that manages one input stream. The input
  // stream is only for observation/polling purpose and should never be used
  // for any further process. Any call to GetNodeReadiness and Preprocess is
  // illegal.
  // TODO: Simplify this. We are forced to use an ISH just to
  // receive a packet, even though we do not need to do any of the things an ISH
  // normally does. The fact that we have to disable required overrides with
  // ABSL_LOG(FATAL) shows that this is the wrong interface.
  class GraphOutputStreamHandler : public InputStreamHandler {
   public:
    GraphOutputStreamHandler(std::shared_ptr<tool::TagMap> tag_map,
                             CalculatorContextManager* cc_manager,
                             const MediaPipeOptions& options,
                             bool calculator_run_in_parallel)
        : InputStreamHandler(std::move(tag_map), cc_manager, options,
                             calculator_run_in_parallel) {}

   protected:
    NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp) override {
      ABSL_LOG(FATAL) << "GraphOutputStreamHandler::GetNodeReadiness should "
                         "never be invoked.";
      return NodeReadiness::kNotReady;
    }

    void FillInputSet(Timestamp input_timestamp,
                      InputStreamShardSet* input_set) override {
      ABSL_LOG(FATAL) << "GraphOutputStreamHandler::FillInputSet should "
                         "never be invoked.";
    }
  };

  bool observe_timestamp_bounds_;
  absl::Mutex mutex_;
  bool notifying_ ABSL_GUARDED_BY(mutex_) = false;
  Timestamp last_processed_ts_ = Timestamp::Unstarted();
  std::unique_ptr<InputStreamHandler> input_stream_handler_;
  std::unique_ptr<InputStreamManager> input_stream_;
};

// OutputStreamObserver that observes the output stream and passes packets to
// the caller via packet_callback.
class OutputStreamObserver : public GraphOutputStream {
 public:
  virtual ~OutputStreamObserver() {}

  absl::Status Initialize(
      const std::string& stream_name, const PacketType* packet_type,
      std::function<absl::Status(const Packet&)> packet_callback,
      OutputStreamManager* output_stream_manager,
      bool observe_timestamp_bounds = false);

  // Notifies the observer of new packets emitted by the observed
  // output stream.
  absl::Status Notify() override;

  // Notifies the observer of the errors in the calculator graph.
  void NotifyError() override {}

 private:
  // Invoked on every packet emitted by the observed output stream.
  std::function<absl::Status(const Packet&)> packet_callback_;
};

// OutputStreamPollerImpl that returns packets to the caller via
// Next()/NextBatch().
// TODO: Support observe_timestamp_bounds.
class OutputStreamPollerImpl : public GraphOutputStream {
 public:
  virtual ~OutputStreamPollerImpl() {}

  // Initializes an OutputStreamPollerImpl.
  absl::Status Initialize(
      const std::string& stream_name, const PacketType* packet_type,
      std::function<void(InputStreamManager*, bool*)> queue_size_callback,
      OutputStreamManager* output_stream_manager,
      bool observe_timestamp_bounds = false);

  void PrepareForRun(std::function<void()> notification_callback,
                     std::function<void(absl::Status)> error_callback) override;

  // Resets graph_has_error_ and cleans the internal packet queue.
  void Reset();

  void SetMaxQueueSize(int queue_size);

  // Returns the number of packets in the queue.
  int QueueSize();

  // Notifies the poller of new packets emitted by the output stream.
  absl::Status Notify() override;

  // Notifies the poller of the errors in the calculator graph.
  void NotifyError() override;

  // Gets the next packet (block until it is available or the stream is
  // done).  Returns true if successful.
  ABSL_MUST_USE_RESULT bool Next(Packet* packet);

 private:
  absl::Mutex mutex_;
  absl::CondVar handler_condvar_ ABSL_GUARDED_BY(mutex_);
  bool graph_has_error_ ABSL_GUARDED_BY(mutex_);
  Timestamp output_timestamp_ ABSL_GUARDED_BY(mutex_) = Timestamp::Min();
};

}  // namespace internal
}  // namespace mediapipe
#endif  // MEDIAPIPE_FRAMEWORK_GRAPH_OUTPUT_STREAM_H_
