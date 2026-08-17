//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_TEST_CPP_QPS_CLIENT_H
#define GRPC_TEST_CPP_QPS_CLIENT_H

#include <grpc/support/time.h>
#include <grpcpp/channel.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/channel_arguments.h>
#include <grpcpp/support/slice.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdlib.h>

#include <condition_variable>
#include <mutex>
#include <thread>
#include <unordered_map>
#include <vector>

#include "absl/log/log.h"
#include "absl/memory/memory.h"
#include "absl/strings/match.h"
#include "absl/strings/str_format.h"
#include "src/core/util/crash.h"
#include "src/core/util/env.h"
#include "src/proto/grpc/testing/benchmark_service.grpc.pb.h"
#include "src/proto/grpc/testing/payloads.pb.h"
#include "test/cpp/qps/histogram.h"
#include "test/cpp/qps/interarrival.h"
#include "test/cpp/qps/qps_worker.h"
#include "test/cpp/qps/server.h"
#include "test/cpp/qps/usage_timer.h"
#include "test/cpp/util/create_test_channel.h"
#include "test/cpp/util/test_credentials_provider.h"

#define INPROC_NAME_PREFIX "qpsinproc:"

namespace grpc {
namespace testing {

template <class RequestType>
class ClientRequestCreator {
 public:
  ClientRequestCreator(RequestType* /*req*/, const PayloadConfig&) {
    // this template must be specialized
    // fail with an assertion rather than a compile-time
    // check since these only happen at the beginning anyway
    grpc_core::Crash("unreachable");
  }
};

template <>
class ClientRequestCreator<SimpleRequest> {
 public:
  ClientRequestCreator(SimpleRequest* req,
                       const PayloadConfig& payload_config) {
    if (payload_config.has_bytebuf_params()) {
      grpc_core::Crash(absl::StrFormat(
          "Invalid PayloadConfig, config cannot have bytebuf_params: %s",
          payload_config.DebugString()
              .c_str()));  // not appropriate for this specialization
    } else if (payload_config.has_simple_params()) {
      req->set_response_type(grpc::testing::PayloadType::COMPRESSABLE);
      req->set_response_size(payload_config.simple_params().resp_size());
      req->mutable_payload()->set_type(
          grpc::testing::PayloadType::COMPRESSABLE);
      int size = payload_config.simple_params().req_size();
      std::unique_ptr<char[]> body(new char[size]);
      req->mutable_payload()->set_body(body.get(), size);
    } else if (payload_config.has_complex_params()) {
      grpc_core::Crash(absl::StrFormat(
          "Invalid PayloadConfig, cannot have complex_params: %s",
          payload_config.DebugString()
              .c_str()));  // not appropriate for this specialization
    } else {
      // default should be simple proto without payloads
      req->set_response_type(grpc::testing::PayloadType::COMPRESSABLE);
      req->set_response_size(0);
      req->mutable_payload()->set_type(
          grpc::testing::PayloadType::COMPRESSABLE);
    }
  }
};

template <>
class ClientRequestCreator<ByteBuffer> {
 public:
  ClientRequestCreator(ByteBuffer* req, const PayloadConfig& payload_config) {
    if (payload_config.has_bytebuf_params()) {
      size_t req_sz =
          static_cast<size_t>(payload_config.bytebuf_params().req_size());
      std::unique_ptr<char[]> buf(new char[req_sz]);
      memset(buf.get(), 0, req_sz);
      Slice slice(buf.get(), req_sz);
      *req = ByteBuffer(&slice, 1);
    } else {
      grpc_core::Crash(absl::StrFormat(
          "Invalid PayloadConfig, missing bytebug_params: %s",
          payload_config.DebugString()
              .c_str()));  // not appropriate for this specialization
    }
  }
};

class HistogramEntry final {
 public:
  HistogramEntry() : value_used_(false), status_used_(false) {}
  bool value_used() const { return value_used_; }
  double value() const { return value_; }
  void set_value(double v) {
    value_used_ = true;
    value_ = v;
  }
  bool status_used() const { return status_used_; }
  int status() const { return status_; }
  void set_status(int status) {
    status_used_ = true;
    status_ = status;
  }

 private:
  bool value_used_;
  double value_;
  bool status_used_;
  int status_;
};

typedef std::unordered_map<int, int64_t> StatusHistogram;

inline void MergeStatusHistogram(const StatusHistogram& from,
                                 StatusHistogram* to) {
  for (StatusHistogram::const_iterator it = from.begin(); it != from.end();
       ++it) {
    (*to)[it->first] += it->second;
  }
}

class Client {
 public:
  Client()
      : timer_(new UsageTimer),
        interarrival_timer_(),
        started_requests_(false),
        last_reset_poll_count_(0) {
    gpr_event_init(&start_requests_);
  }
  virtual ~Client() {}

  ClientStats Mark(bool reset) {
    Histogram latencies;
    StatusHistogram statuses;
    UsageTimer::Result timer_result;

    MaybeStartRequests();

    int cur_poll_count = GetPollCount();
    int poll_count = cur_poll_count - last_reset_poll_count_;
    if (reset) {
      std::vector<Histogram> to_merge(threads_.size());
      std::vector<StatusHistogram> to_merge_status(threads_.size());

      for (size_t i = 0; i < threads_.size(); i++) {
        threads_[i]->BeginSwap(&to_merge[i], &to_merge_status[i]);
      }
      std::unique_ptr<UsageTimer> timer(new UsageTimer);
      timer_.swap(timer);
      for (size_t i = 0; i < threads_.size(); i++) {
        latencies.Merge(to_merge[i]);
        MergeStatusHistogram(to_merge_status[i], &statuses);
      }
      timer_result = timer->Mark();
      last_reset_poll_count_ = cur_poll_count;
    } else {
      // merge snapshots of each thread histogram
      for (size_t i = 0; i < threads_.size(); i++) {
        threads_[i]->MergeStatsInto(&latencies, &statuses);
      }
      timer_result = timer_->Mark();
    }

    // Print the median latency per interval for one thread.
    // If the number of warmup seconds is x, then the first x + 1 numbers in the
    // vector are from the warmup period and should be discarded.
    if (median_latency_collection_interval_seconds_ > 0) {
      std::vector<double> medians_per_interval =
          threads_[0]->GetMedianPerIntervalList();
      LOG(INFO) << "Num threads: " << threads_.size();
      LOG(INFO) << "Number of medians: " << medians_per_interval.size();
      for (size_t j = 0; j < medians_per_interval.size(); j++) {
        LOG(INFO) << medians_per_interval[j];
      }
    }

    ClientStats stats;
    latencies.FillProto(stats.mutable_latencies());
    for (StatusHistogram::const_iterator it = statuses.begin();
         it != statuses.end(); ++it) {
      RequestResultCount* rrc = stats.add_request_results();
      rrc->set_status_code(it->first);
      rrc->set_count(it->second);
    }
    stats.set_time_elapsed(timer_result.wall);
    stats.set_time_system(timer_result.system);
    stats.set_time_user(timer_result.user);
    stats.set_cq_poll_count(poll_count);
    return stats;
  }

  // Must call AwaitThreadsCompletion before destructor to avoid a race
  // between destructor and invocation of virtual ThreadFunc
  void AwaitThreadsCompletion() {
    gpr_atm_rel_store(&thread_pool_done_, static_cast<gpr_atm>(true));
    DestroyMultithreading();
    std::unique_lock<std::mutex> g(thread_completion_mu_);
    while (threads_remaining_ != 0) {
      threads_complete_.wait(g);
    }
  }

  // Returns the interval (in seconds) between collecting latency medians. If 0,
  // no periodic median latencies will be collected.
  double GetLatencyCollectionIntervalInSeconds() {
    return median_latency_collection_interval_seconds_;
  }

  virtual int GetPollCount() {
    // For sync client.
    return 0;
  }

  bool IsClosedLoop() { return closed_loop_; }

  gpr_timespec NextIssueTime(int thread_idx) {
    const gpr_timespec result = next_time_[thread_idx];
    next_time_[thread_idx] =
        gpr_time_add(next_time_[thread_idx],
                     gpr_time_from_nanos(interarrival_timer_.next(thread_idx),
                                         GPR_TIMESPAN));
    return result;
  }

  bool ThreadCompleted() {
    return static_cast<bool>(gpr_atm_acq_load(&thread_pool_done_));
  }

  class Thread {
   public:
    Thread(Client* client, size_t idx)
        : client_(client), idx_(idx), impl_(&Thread::ThreadFunc, this) {}

    ~Thread() { impl_.join(); }

    void BeginSwap(Histogram* n, StatusHistogram* s) {
      std::lock_guard<std::mutex> g(mu_);
      n->Swap(&histogram_);
      s->swap(statuses_);
    }

    void MergeStatsInto(Histogram* hist, StatusHistogram* s) {
      std::unique_lock<std::mutex> g(mu_);
      hist->Merge(histogram_);
      MergeStatusHistogram(statuses_, s);
    }

    std::vector<double> GetMedianPerIntervalList() {
      return medians_each_interval_list_;
    }

    void UpdateHistogram(HistogramEntry* entry) {
      std::lock_guard<std::mutex> g(mu_);
      if (entry->value_used()) {
        histogram_.Add(entry->value());
        if (client_->GetLatencyCollectionIntervalInSeconds() > 0) {
          histogram_per_interval_.Add(entry->value());
          double now = UsageTimer::Now();
          if ((now - interval_start_time_) >=
              client_->GetLatencyCollectionIntervalInSeconds()) {
            // Record the median latency of requests from the last interval.
            // Divide by 1e3 to get microseconds.
            medians_each_interval_list_.push_back(
                histogram_per_interval_.Percentile(50) / 1e3);
            histogram_per_interval_.Reset();
            interval_start_time_ = now;
          }
        }
      }
      if (entry->status_used()) {
        statuses_[entry->status()]++;
      }
    }

   private:
    Thread(const Thread&);
    Thread& operator=(const Thread&);

    void ThreadFunc() {
      int wait_loop = 0;
      while (!gpr_event_wait(
          &client_->start_requests_,
          gpr_time_add(gpr_now(GPR_CLOCK_REALTIME),
                       gpr_time_from_seconds(20, GPR_TIMESPAN)))) {
        LOG(INFO) << idx_ << ": Waiting for benchmark to start (" << wait_loop
                  << ")";
        wait_loop++;
      }

      client_->ThreadFunc(idx_, this);
      client_->CompleteThread();
    }

    std::mutex mu_;
    Histogram histogram_;
    StatusHistogram statuses_;
    Client* client_;
    const size_t idx_;
    std::thread impl_;
    // The following are used only if
    // median_latency_collection_interval_seconds_ is greater than 0
    Histogram histogram_per_interval_;
    std::vector<double> medians_each_interval_list_;
    double interval_start_time_;
  };

 protected:
  bool closed_loop_;
  gpr_atm thread_pool_done_;
  double median_latency_collection_interval_seconds_;  // In seconds

  void StartThreads(size_t num_threads) {
    gpr_atm_rel_store(&thread_pool_done_, static_cast<gpr_atm>(false));
    threads_remaining_ = num_threads;
    for (size_t i = 0; i < num_threads; i++) {
      threads_.emplace_back(new Thread(this, i));
    }
  }

  void EndThreads() {
    MaybeStartRequests();
    threads_.clear();
  }

  virtual void DestroyMultithreading() = 0;

  void SetupLoadTest(const ClientConfig& config, size_t num_threads) {
    // Set up the load distribution based on the number of threads
    const auto& load = config.load_params();

    std::unique_ptr<RandomDistInterface> random_dist;
    switch (load.load_case()) {
      case LoadParams::kClosedLoop:
        // Closed-loop doesn't use random dist at all
        break;
      case LoadParams::kPoisson:
        random_dist = std::make_unique<ExpDist>(load.poisson().offered_load() /
                                                num_threads);
        break;
      default:
        grpc_core::Crash("unreachable");
    }

    // Set closed_loop_ based on whether or not random_dist is set
    if (!random_dist) {
      closed_loop_ = true;
    } else {
      closed_loop_ = false;
      // set up interarrival timer according to random dist
      interarrival_timer_.init(*random_dist, num_threads);
      const auto now = gpr_now(GPR_CLOCK_MONOTONIC);
      for (size_t i = 0; i < num_threads; i++) {
        next_time_.push_back(gpr_time_add(
            now,
            gpr_time_from_nanos(interarrival_timer_.next(i), GPR_TIMESPAN)));
      }
    }
  }

  std::function<gpr_timespec()> NextIssuer(int thread_idx) {
    return closed_loop_ ? std::function<gpr_timespec()>()
                        : std::bind(&Client::NextIssueTime, this, thread_idx);
  }

  virtual void ThreadFunc(size_t thread_idx, Client::Thread* t) = 0;

  std::vector<std::unique_ptr<Thread>> threads_;
  std::unique_ptr<UsageTimer> timer_;

  InterarrivalTimer interarrival_timer_;
  std::vector<gpr_timespec> next_time_;

  std::mutex thread_completion_mu_;
  size_t threads_remaining_;
  std::condition_variable threads_complete_;

  gpr_event start_requests_;
  bool started_requests_;

  int last_reset_poll_count_;

  void MaybeStartRequests() {
    if (!started_requests_) {
      started_requests_ = true;
      gpr_event_set(&start_requests_, reinterpret_cast<void*>(1));
    }
  }

  void CompleteThread() {
    std::lock_guard<std::mutex> g(thread_completion_mu_);
    threads_remaining_--;
    if (threads_remaining_ == 0) {
      threads_complete_.notify_all();
    }
  }
};

template <class StubType, class RequestType>
class ClientImpl : public Client {
 public:
  ClientImpl(const ClientConfig& config,
             std::function<std::unique_ptr<StubType>(std::shared_ptr<Channel>)>
                 create_stub)
      : cores_(gpr_cpu_num_cores()), create_stub_(create_stub) {
    for (int i = 0; i < config.client_channels(); i++) {
      channels_.emplace_back(
          config.server_targets(i % config.server_targets_size()), config,
          create_stub_, i);
    }
    WaitForChannelsToConnect();
    median_latency_collection_interval_seconds_ =
        config.median_latency_collection_interval_millis() / 1e3;
    ClientRequestCreator<RequestType> create_req(&request_,
                                                 config.payload_config());
  }
  ~ClientImpl() override {}
  const RequestType* request() { return &request_; }

  void WaitForChannelsToConnect() {
    int connect_deadline_seconds = 10;
    // Allow optionally overriding connect_deadline in order
    // to deal with benchmark environments in which the server
    // can take a long time to become ready.
    auto channel_connect_timeout_str =
        grpc_core::GetEnv("QPS_WORKER_CHANNEL_CONNECT_TIMEOUT");
    if (channel_connect_timeout_str.has_value() &&
        !channel_connect_timeout_str->empty()) {
      connect_deadline_seconds = atoi(channel_connect_timeout_str->c_str());
    }
    LOG(INFO) << "Waiting for up to " << connect_deadline_seconds
              << " seconds for all channels to connect";
    gpr_timespec connect_deadline = gpr_time_add(
        gpr_now(GPR_CLOCK_REALTIME),
        gpr_time_from_seconds(connect_deadline_seconds, GPR_TIMESPAN));
    CompletionQueue cq;
    size_t num_remaining = 0;
    for (auto& c : channels_) {
      if (!c.is_inproc()) {
        Channel* channel = c.get_channel();
        grpc_connectivity_state last_observed = channel->GetState(true);
        if (last_observed == GRPC_CHANNEL_READY) {
          LOG(INFO) << "Channel " << channel << " connected!";
        } else {
          num_remaining++;
          channel->NotifyOnStateChange(last_observed, connect_deadline, &cq,
                                       channel);
        }
      }
    }
    while (num_remaining > 0) {
      bool ok = false;
      void* tag = nullptr;
      cq.Next(&tag, &ok);
      Channel* channel = static_cast<Channel*>(tag);
      if (!ok) {
        grpc_core::Crash(absl::StrFormat(
            "Channel %p failed to connect within the deadline", channel));
      } else {
        grpc_connectivity_state last_observed = channel->GetState(true);
        if (last_observed == GRPC_CHANNEL_READY) {
          LOG(INFO) << "Channel " << channel << " connected!";
          num_remaining--;
        } else {
          channel->NotifyOnStateChange(last_observed, connect_deadline, &cq,
                                       channel);
        }
      }
    }
  }

 protected:
  const int cores_;
  RequestType request_;

  class ClientChannelInfo {
   public:
    ClientChannelInfo(
        const std::string& target, const ClientConfig& config,
        std::function<std::unique_ptr<StubType>(std::shared_ptr<Channel>)>
            create_stub,
        int shard) {
      ChannelArguments args;
      args.SetInt("shard_to_ensure_no_subchannel_merges", shard);
      set_channel_args(config, &args);

      std::string type;
      if (config.has_security_params() &&
          config.security_params().cred_type().empty()) {
        type = kTlsCredentialsType;
      } else {
        type = config.security_params().cred_type();
      }

      std::string inproc_pfx(INPROC_NAME_PREFIX);
      if (!absl::StartsWith(target, inproc_pfx)) {
        channel_ = CreateTestChannel(
            target, type, config.security_params().server_host_override(),
            !config.security_params().use_test_ca(),
            std::shared_ptr<CallCredentials>(), args);
        LOG(INFO) << "Connecting to " << target;
        is_inproc_ = false;
      } else {
        std::string tgt = target;
        tgt.erase(0, inproc_pfx.length());
        int srv_num = std::stoi(tgt);
        channel_ = (*g_inproc_servers)[srv_num]->InProcessChannel(args);
        is_inproc_ = true;
      }
      stub_ = create_stub(channel_);
    }
    Channel* get_channel() { return channel_.get(); }
    StubType* get_stub() { return stub_.get(); }
    bool is_inproc() { return is_inproc_; }

   private:
    void set_channel_args(const ClientConfig& config, ChannelArguments* args) {
      for (const auto& channel_arg : config.channel_args()) {
        if (channel_arg.value_case() == ChannelArg::kStrValue) {
          args->SetString(channel_arg.name(), channel_arg.str_value());
        } else if (channel_arg.value_case() == ChannelArg::kIntValue) {
          args->SetInt(channel_arg.name(), channel_arg.int_value());
        } else {
          LOG(ERROR) << "Empty channel arg value.";
        }
      }
    }

    std::shared_ptr<Channel> channel_;
    std::unique_ptr<StubType> stub_;
    bool is_inproc_;
  };
  std::vector<ClientChannelInfo> channels_;
  std::function<std::unique_ptr<StubType>(const std::shared_ptr<Channel>&)>
      create_stub_;
};

std::unique_ptr<Client> CreateSynchronousClient(const ClientConfig& config);
std::unique_ptr<Client> CreateAsyncClient(const ClientConfig& config);
std::unique_ptr<Client> CreateCallbackClient(const ClientConfig& config);
std::unique_ptr<Client> CreateGenericAsyncStreamingClient(
    const ClientConfig& config);

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_CLIENT_H
