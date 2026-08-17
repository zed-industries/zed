// Copyright 2023 The gRPC Authors.
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

#ifndef GRPC_TEST_CORE_TEST_UTIL_FAKE_STATS_PLUGIN_H
#define GRPC_TEST_CORE_TEST_UTIL_FAKE_STATS_PLUGIN_H

#include <memory>
#include <optional>
#include <string>
#include <type_traits>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/functional/any_invocable.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "gmock/gmock.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/telemetry/metrics.h"
#include "src/core/telemetry/tcp_tracer.h"
#include "src/core/util/ref_counted.h"

namespace grpc_core {

// Registers a FakeStatsClientFilter as a client channel filter if there is a
// FakeClientCallTracerFactory in the channel args. This filter will use the
// FakeClientCallTracerFactory to create and inject a FakeClientCallTracer into
// the call context.
// Example usage:
//   RegisterFakeStatsPlugin();  // before grpc_init()
//
//   // Creates a FakeClientCallTracerFactory and adds it into the channel args.
//   FakeClientCallTracerFactory fake_client_call_tracer_factory;
//   ChannelArguments channel_args;
//   channel_args.SetPointer(GRPC_ARG_INJECT_FAKE_CLIENT_CALL_TRACER_FACTORY,
//                           &fake_client_call_tracer_factory);
//
//   // After the system under test has been executed (e.g. an RPC has been
//   // sent), use the FakeClientCallTracerFactory to verify certain
//   // expectations.
//   EXPECT_THAT(fake_client_call_tracer_factory.GetLastFakeClientCallTracer()
//                   ->GetLastCallAttemptTracer()
//                   ->GetOptionalLabels(),
//               VerifyCsmServiceLabels());
void RegisterFakeStatsPlugin();

class FakeClientCallTracer : public ClientCallTracer {
 public:
  class FakeClientCallAttemptTracer
      : public ClientCallTracer::CallAttemptTracer,
        public RefCounted<FakeClientCallAttemptTracer> {
   public:
    explicit FakeClientCallAttemptTracer(
        std::vector<std::string>* annotation_logger)
        : annotation_logger_(annotation_logger) {}
    void RecordSendInitialMetadata(
        grpc_metadata_batch* /*send_initial_metadata*/) override {}
    void RecordSendTrailingMetadata(
        grpc_metadata_batch* /*send_trailing_metadata*/) override {}
    void RecordSendMessage(const Message& /*send_message*/) override {}
    void RecordSendCompressedMessage(
        const Message& /*send_compressed_message*/) override {}
    void RecordReceivedInitialMetadata(
        grpc_metadata_batch* /*recv_initial_metadata*/) override {}
    void RecordReceivedMessage(const Message& /*recv_message*/) override {}
    void RecordReceivedDecompressedMessage(
        const Message& /*recv_decompressed_message*/) override {}
    void RecordCancel(grpc_error_handle /*cancel_error*/) override {}
    void RecordReceivedTrailingMetadata(
        absl::Status /*status*/,
        grpc_metadata_batch* /*recv_trailing_metadata*/,
        const grpc_transport_stream_stats* /*transport_stream_stats*/)
        override {}
    void RecordEnd() override { Unref(); }
    void RecordIncomingBytes(
        const TransportByteSize& /*transport_byte_size*/) override {}
    void RecordOutgoingBytes(
        const TransportByteSize& /*transport_byte_size*/) override {}
    void RecordAnnotation(absl::string_view annotation) override {
      annotation_logger_->push_back(std::string(annotation));
    }
    void RecordAnnotation(const Annotation& /*annotation*/) override {}
    std::shared_ptr<TcpCallTracer> StartNewTcpTrace() override {
      return nullptr;
    }
    void SetOptionalLabel(OptionalLabelKey key,
                          RefCountedStringValue value) override {
      optional_labels_.emplace(key, std::move(value));
    }
    std::string TraceId() override { return ""; }
    std::string SpanId() override { return ""; }
    bool IsSampled() override { return false; }

    const std::map<OptionalLabelKey, RefCountedStringValue>& GetOptionalLabels()
        const {
      return optional_labels_;
    }

   private:
    std::vector<std::string>* annotation_logger_;
    std::map<OptionalLabelKey, RefCountedStringValue> optional_labels_;
  };

  explicit FakeClientCallTracer(std::vector<std::string>* annotation_logger)
      : annotation_logger_(annotation_logger) {}
  ~FakeClientCallTracer() override {}
  CallAttemptTracer* StartNewAttempt(bool /*is_transparent_retry*/) override {
    auto call_attempt_tracer =
        MakeRefCounted<FakeClientCallAttemptTracer>(annotation_logger_);
    call_attempt_tracers_.emplace_back(call_attempt_tracer);
    return call_attempt_tracer.release();  // Released in RecordEnd().
  }

  void RecordAnnotation(absl::string_view annotation) override {
    annotation_logger_->push_back(std::string(annotation));
  }
  void RecordAnnotation(const Annotation& /*annotation*/) override {}
  std::string TraceId() override { return ""; }
  std::string SpanId() override { return ""; }
  bool IsSampled() override { return false; }

  FakeClientCallAttemptTracer* GetLastCallAttemptTracer() const {
    return call_attempt_tracers_.back().get();
  }

 private:
  std::vector<std::string>* annotation_logger_;
  std::vector<RefCountedPtr<FakeClientCallAttemptTracer>> call_attempt_tracers_;
};

#define GRPC_ARG_INJECT_FAKE_CLIENT_CALL_TRACER_FACTORY \
  "grpc.testing.inject_fake_client_call_tracer_factory"

class FakeClientCallTracerFactory {
 public:
  FakeClientCallTracer* CreateFakeClientCallTracer() {
    fake_client_call_tracers_.emplace_back(
        new FakeClientCallTracer(&annotation_logger_));
    return fake_client_call_tracers_.back().get();
  }

  FakeClientCallTracer* GetLastFakeClientCallTracer() {
    return fake_client_call_tracers_.back().get();
  }

 private:
  std::vector<std::string> annotation_logger_;
  std::vector<std::unique_ptr<FakeClientCallTracer>> fake_client_call_tracers_;
};

class FakeServerCallTracer : public ServerCallTracer {
 public:
  explicit FakeServerCallTracer(std::vector<std::string>* annotation_logger)
      : annotation_logger_(annotation_logger) {}
  ~FakeServerCallTracer() override {}
  void RecordSendInitialMetadata(
      grpc_metadata_batch* /*send_initial_metadata*/) override {}
  void RecordSendTrailingMetadata(
      grpc_metadata_batch* /*send_trailing_metadata*/) override {}
  void RecordSendMessage(const Message& /*send_message*/) override {}
  void RecordSendCompressedMessage(
      const Message& /*send_compressed_message*/) override {}
  void RecordReceivedInitialMetadata(
      grpc_metadata_batch* /*recv_initial_metadata*/) override {}
  void RecordReceivedMessage(const Message& /*recv_message*/) override {}
  void RecordReceivedDecompressedMessage(
      const Message& /*recv_decompressed_message*/) override {}
  void RecordCancel(grpc_error_handle /*cancel_error*/) override {}
  void RecordReceivedTrailingMetadata(
      grpc_metadata_batch* /*recv_trailing_metadata*/) override {}
  void RecordEnd(const grpc_call_final_info* /*final_info*/) override {}
  void RecordIncomingBytes(
      const TransportByteSize& /*transport_byte_size*/) override {}
  void RecordOutgoingBytes(
      const TransportByteSize& /*transport_byte_size*/) override {}
  void RecordAnnotation(absl::string_view annotation) override {
    annotation_logger_->push_back(std::string(annotation));
  }
  void RecordAnnotation(const Annotation& /*annotation*/) override {}
  std::shared_ptr<TcpCallTracer> StartNewTcpTrace() override { return nullptr; }
  std::string TraceId() override { return ""; }
  std::string SpanId() override { return ""; }
  bool IsSampled() override { return false; }

 private:
  std::vector<std::string>* annotation_logger_;
};

std::string MakeLabelString(
    absl::Span<const absl::string_view> label_keys,
    absl::Span<const absl::string_view> label_values,
    absl::Span<const absl::string_view> optional_label_keys,
    absl::Span<const absl::string_view> optional_values);

class FakeStatsPlugin : public StatsPlugin {
 public:
  explicit FakeStatsPlugin(
      absl::AnyInvocable<
          bool(const experimental::StatsPluginChannelScope& /*scope*/) const>
          channel_filter = nullptr,
      bool use_disabled_by_default_metrics = false)
      : channel_filter_(std::move(channel_filter)),
        use_disabled_by_default_metrics_(use_disabled_by_default_metrics) {
    GlobalInstrumentsRegistry::ForEach(
        [&](const GlobalInstrumentsRegistry::GlobalInstrumentDescriptor&
                descriptor) {
          if (!use_disabled_by_default_metrics &&
              !descriptor.enable_by_default) {
            return;
          }
          switch (descriptor.instrument_type) {
            case GlobalInstrumentsRegistry::InstrumentType::kCounter: {
              MutexLock lock(&mu_);
              if (descriptor.value_type ==
                  GlobalInstrumentsRegistry::ValueType::kUInt64) {
                uint64_counters_.emplace(descriptor.index, descriptor);
              } else {
                double_counters_.emplace(descriptor.index, descriptor);
              }
              break;
            }
            case GlobalInstrumentsRegistry::InstrumentType::kHistogram: {
              MutexLock lock(&mu_);
              if (descriptor.value_type ==
                  GlobalInstrumentsRegistry::ValueType::kUInt64) {
                uint64_histograms_.emplace(descriptor.index, descriptor);
              } else {
                double_histograms_.emplace(descriptor.index, descriptor);
              }
              break;
            }
            case GlobalInstrumentsRegistry::InstrumentType::kCallbackGauge: {
              MutexLock lock(&callback_mu_);
              if (descriptor.value_type ==
                  GlobalInstrumentsRegistry::ValueType::kInt64) {
                int64_callback_gauges_.emplace(descriptor.index, descriptor);
              } else {
                double_callback_gauges_.emplace(descriptor.index, descriptor);
              }
              break;
            }
            default:
              Crash("unknown instrument type");
          }
        });
  }

  std::pair<bool, std::shared_ptr<StatsPlugin::ScopeConfig>>
  IsEnabledForChannel(
      const experimental::StatsPluginChannelScope& scope) const override {
    if (channel_filter_ == nullptr || channel_filter_(scope)) {
      return {true, nullptr};
    }
    return {false, nullptr};
  }
  std::pair<bool, std::shared_ptr<StatsPlugin::ScopeConfig>> IsEnabledForServer(
      const ChannelArgs& /*args*/) const override {
    return {true, nullptr};
  }
  std::shared_ptr<StatsPlugin::ScopeConfig> GetChannelScopeConfig(
      const experimental::StatsPluginChannelScope& /*scope*/) const override {
    return nullptr;
  }
  std::shared_ptr<StatsPlugin::ScopeConfig> GetServerScopeConfig(
      const ChannelArgs& /*args*/) const override {
    return nullptr;
  }

  void AddCounter(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, uint64_t value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) override {
    // The problem with this approach is that we initialize uint64_counters_ in
    // BuildAndRegister by querying the GlobalInstrumentsRegistry at the time.
    // If the GlobalInstrumentsRegistry has changed since then (which we
    // currently don't allow), we might not have seen that descriptor nor have
    // we created an instrument for it. We probably could copy the existing
    // instruments at build time and for the handle that we haven't seen we will
    // just ignore it here. This would also prevent us from having to lock the
    // GlobalInstrumentsRegistry everytime a metric is recorded. But this is not
    // a concern for now.
    VLOG(2) << "FakeStatsPlugin[" << this
            << "]::AddCounter(index=" << handle.index << ", value=(uint64)"
            << value << ", label_values={" << absl::StrJoin(label_values, ", ")
            << "}, optional_label_values={"
            << absl::StrJoin(optional_values, ", ") << "}";
    MutexLock lock(&mu_);
    auto iter = uint64_counters_.find(handle.index);
    if (iter == uint64_counters_.end()) return;
    iter->second.Add(value, label_values, optional_values);
  }
  void AddCounter(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, double value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) override {
    VLOG(2) << "FakeStatsPlugin[" << this
            << "]::AddCounter(index=" << handle.index
            << ", value(double)=" << value << ", label_values={"
            << absl::StrJoin(label_values, ", ") << "}, optional_label_values={"
            << absl::StrJoin(optional_values, ", ") << "}";
    MutexLock lock(&mu_);
    auto iter = double_counters_.find(handle.index);
    if (iter == double_counters_.end()) return;
    iter->second.Add(value, label_values, optional_values);
  }
  void RecordHistogram(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, uint64_t value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) override {
    VLOG(2) << "FakeStatsPlugin[" << this
            << "]::RecordHistogram(index=" << handle.index << ", value=(uint64)"
            << value << ", label_values={" << absl::StrJoin(label_values, ", ")
            << "}, optional_label_values={"
            << absl::StrJoin(optional_values, ", ") << "}";
    MutexLock lock(&mu_);
    auto iter = uint64_histograms_.find(handle.index);
    if (iter == uint64_histograms_.end()) return;
    iter->second.Record(value, label_values, optional_values);
  }
  void RecordHistogram(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, double value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) override {
    VLOG(2) << "FakeStatsPlugin[" << this
            << "]::RecordHistogram(index=" << handle.index << ", value=(double)"
            << value << ", label_values={" << absl::StrJoin(label_values, ", ")
            << "}, optional_label_values={"
            << absl::StrJoin(optional_values, ", ") << "}";
    MutexLock lock(&mu_);
    auto iter = double_histograms_.find(handle.index);
    if (iter == double_histograms_.end()) return;
    iter->second.Record(value, label_values, optional_values);
  }
  void AddCallback(RegisteredMetricCallback* callback) override {
    VLOG(2) << "FakeStatsPlugin[" << this << "]::AddCallback(" << callback
            << ")";
    MutexLock lock(&callback_mu_);
    callbacks_.insert(callback);
  }
  void RemoveCallback(RegisteredMetricCallback* callback) override {
    VLOG(2) << "FakeStatsPlugin[" << this << "]::RemoveCallback(" << callback
            << ")";
    MutexLock lock(&callback_mu_);
    callbacks_.erase(callback);
  }

  ClientCallTracer* GetClientCallTracer(
      const Slice& /*path*/, bool /*registered_method*/,
      std::shared_ptr<StatsPlugin::ScopeConfig> /*scope_config*/) override {
    return nullptr;
  }
  ServerCallTracer* GetServerCallTracer(
      std::shared_ptr<StatsPlugin::ScopeConfig> /*scope_config*/) override {
    return nullptr;
  }
  bool IsInstrumentEnabled(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle) const override {
    const auto& descriptor =
        GlobalInstrumentsRegistry::GetInstrumentDescriptor(handle);
    return use_disabled_by_default_metrics_ || descriptor.enable_by_default;
  }

  std::optional<uint64_t> GetUInt64CounterValue(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) {
    MutexLock lock(&mu_);
    auto iter = uint64_counters_.find(handle.index);
    if (iter == uint64_counters_.end()) {
      return std::nullopt;
    }
    return iter->second.GetValue(label_values, optional_values);
  }
  std::optional<double> GetDoubleCounterValue(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) {
    MutexLock lock(&mu_);
    auto iter = double_counters_.find(handle.index);
    if (iter == double_counters_.end()) {
      return std::nullopt;
    }
    return iter->second.GetValue(label_values, optional_values);
  }
  std::optional<std::vector<uint64_t>> GetUInt64HistogramValue(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) {
    MutexLock lock(&mu_);
    auto iter = uint64_histograms_.find(handle.index);
    if (iter == uint64_histograms_.end()) {
      return std::nullopt;
    }
    return iter->second.GetValues(label_values, optional_values);
  }
  std::optional<std::vector<double>> GetDoubleHistogramValue(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) {
    MutexLock lock(&mu_);
    auto iter = double_histograms_.find(handle.index);
    if (iter == double_histograms_.end()) {
      return std::nullopt;
    }
    return iter->second.GetValues(label_values, optional_values);
  }
  void TriggerCallbacks() {
    VLOG(2) << "FakeStatsPlugin[" << this << "]::TriggerCallbacks(): START";
    Reporter reporter(*this);
    MutexLock lock(&callback_mu_);
    for (auto* callback : callbacks_) {
      callback->Run(reporter);
    }
    VLOG(2) << "FakeStatsPlugin[" << this << "]::TriggerCallbacks(): END";
  }
  std::optional<int64_t> GetInt64CallbackGaugeValue(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) {
    MutexLock lock(&callback_mu_);
    auto iter = int64_callback_gauges_.find(handle.index);
    if (iter == int64_callback_gauges_.end()) {
      return std::nullopt;
    }
    return iter->second.GetValue(label_values, optional_values);
  }
  std::optional<double> GetDoubleCallbackGaugeValue(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) {
    MutexLock lock(&callback_mu_);
    auto iter = double_callback_gauges_.find(handle.index);
    if (iter == double_callback_gauges_.end()) {
      return std::nullopt;
    }
    return iter->second.GetValue(label_values, optional_values);
  }

 private:
  class Reporter : public CallbackMetricReporter {
   public:
    explicit Reporter(FakeStatsPlugin& plugin) : plugin_(plugin) {}

    void ReportInt64(
        GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, int64_t value,
        absl::Span<const absl::string_view> label_values,
        absl::Span<const absl::string_view> optional_values) override
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(plugin_.callback_mu_) {
      VLOG(2) << "FakeStatsPlugin[" << this
              << "]::Reporter::Report(index=" << handle.index
              << ", value=(int64_t)" << value << ", label_values={"
              << absl::StrJoin(label_values, ", ")
              << "}, optional_label_values={"
              << absl::StrJoin(optional_values, ", ") << "}";
      auto iter = plugin_.int64_callback_gauges_.find(handle.index);
      if (iter == plugin_.int64_callback_gauges_.end()) return;
      iter->second.Set(value, label_values, optional_values);
    }

    void ReportDouble(
        GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, double value,
        absl::Span<const absl::string_view> label_values,
        absl::Span<const absl::string_view> optional_values) override
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(plugin_.callback_mu_) {
      VLOG(2) << "FakeStatsPlugin[" << this
              << "]::Reporter::Report(index=" << handle.index
              << ", value=(double)" << value << ", label_values={"
              << absl::StrJoin(label_values, ", ")
              << "}, optional_label_values={"
              << absl::StrJoin(optional_values, ", ") << "}";
      auto iter = plugin_.double_callback_gauges_.find(handle.index);
      if (iter == plugin_.double_callback_gauges_.end()) return;
      iter->second.Set(value, label_values, optional_values);
    }

   private:
    FakeStatsPlugin& plugin_;
  };

  template <class T>
  class Counter {
   public:
    explicit Counter(GlobalInstrumentsRegistry::GlobalInstrumentDescriptor u)
        : name_(u.name),
          description_(u.description),
          unit_(u.unit),
          label_keys_(std::move(u.label_keys)),
          optional_label_keys_(std::move(u.optional_label_keys)) {}

    void Add(T t, absl::Span<const absl::string_view> label_values,
             absl::Span<const absl::string_view> optional_values) {
      auto iter = storage_.find(MakeLabelString(
          label_keys_, label_values, optional_label_keys_, optional_values));
      if (iter != storage_.end()) {
        iter->second += t;
      } else {
        storage_[MakeLabelString(label_keys_, label_values,
                                 optional_label_keys_, optional_values)] = t;
      }
    }

    std::optional<T> GetValue(
        absl::Span<const absl::string_view> label_values,
        absl::Span<const absl::string_view> optional_values) {
      auto iter = storage_.find(MakeLabelString(
          label_keys_, label_values, optional_label_keys_, optional_values));
      if (iter == storage_.end()) {
        return std::nullopt;
      }
      return iter->second;
    }

   private:
    absl::string_view name_;
    absl::string_view description_;
    absl::string_view unit_;
    std::vector<absl::string_view> label_keys_;
    std::vector<absl::string_view> optional_label_keys_;
    // Aggregation of the same key attributes.
    absl::flat_hash_map<std::string, T> storage_;
  };

  template <class T>
  class Histogram {
   public:
    explicit Histogram(GlobalInstrumentsRegistry::GlobalInstrumentDescriptor u)
        : name_(u.name),
          description_(u.description),
          unit_(u.unit),
          label_keys_(std::move(u.label_keys)),
          optional_label_keys_(std::move(u.optional_label_keys)) {}

    void Record(T t, absl::Span<const absl::string_view> label_values,
                absl::Span<const absl::string_view> optional_values) {
      std::string key = MakeLabelString(label_keys_, label_values,
                                        optional_label_keys_, optional_values);
      auto iter = storage_.find(key);
      if (iter == storage_.end()) {
        storage_.emplace(key, std::initializer_list<T>{t});
      } else {
        iter->second.push_back(t);
      }
    }

    std::optional<std::vector<T>> GetValues(
        absl::Span<const absl::string_view> label_values,
        absl::Span<const absl::string_view> optional_values) {
      auto iter = storage_.find(MakeLabelString(
          label_keys_, label_values, optional_label_keys_, optional_values));
      if (iter == storage_.end()) {
        return std::nullopt;
      }
      return iter->second;
    }

   private:
    absl::string_view name_;
    absl::string_view description_;
    absl::string_view unit_;
    std::vector<absl::string_view> label_keys_;
    std::vector<absl::string_view> optional_label_keys_;
    absl::flat_hash_map<std::string, std::vector<T>> storage_;
  };

  template <class T>
  class Gauge {
   public:
    explicit Gauge(GlobalInstrumentsRegistry::GlobalInstrumentDescriptor u)
        : name_(u.name),
          description_(u.description),
          unit_(u.unit),
          label_keys_(std::move(u.label_keys)),
          optional_label_keys_(std::move(u.optional_label_keys)) {}

    void Set(T t, absl::Span<const absl::string_view> label_values,
             absl::Span<const absl::string_view> optional_values) {
      storage_[MakeLabelString(label_keys_, label_values, optional_label_keys_,
                               optional_values)] = t;
    }

    std::optional<T> GetValue(
        absl::Span<const absl::string_view> label_values,
        absl::Span<const absl::string_view> optional_values) {
      auto iter = storage_.find(MakeLabelString(
          label_keys_, label_values, optional_label_keys_, optional_values));
      if (iter == storage_.end()) {
        return std::nullopt;
      }
      return iter->second;
    }

   private:
    absl::string_view name_;
    absl::string_view description_;
    absl::string_view unit_;
    std::vector<absl::string_view> label_keys_;
    std::vector<absl::string_view> optional_label_keys_;
    absl::flat_hash_map<std::string, T> storage_;
  };

  absl::AnyInvocable<bool(
      const experimental::StatsPluginChannelScope& /*scope*/) const>
      channel_filter_;
  bool use_disabled_by_default_metrics_;
  // Instruments.
  Mutex mu_;
  absl::flat_hash_map<uint32_t, Counter<uint64_t>> uint64_counters_
      ABSL_GUARDED_BY(&mu_);
  absl::flat_hash_map<uint32_t, Counter<double>> double_counters_
      ABSL_GUARDED_BY(&mu_);
  absl::flat_hash_map<uint32_t, Histogram<uint64_t>> uint64_histograms_
      ABSL_GUARDED_BY(&mu_);
  absl::flat_hash_map<uint32_t, Histogram<double>> double_histograms_
      ABSL_GUARDED_BY(&mu_);
  Mutex callback_mu_;
  absl::flat_hash_map<uint32_t, Gauge<int64_t>> int64_callback_gauges_
      ABSL_GUARDED_BY(&callback_mu_);
  absl::flat_hash_map<uint32_t, Gauge<double>> double_callback_gauges_
      ABSL_GUARDED_BY(&callback_mu_);
  std::set<RegisteredMetricCallback*> callbacks_ ABSL_GUARDED_BY(&callback_mu_);
};

class FakeStatsPluginBuilder {
 public:
  FakeStatsPluginBuilder& SetChannelFilter(
      absl::AnyInvocable<
          bool(const experimental::StatsPluginChannelScope& /*scope*/) const>
          channel_filter) {
    channel_filter_ = std::move(channel_filter);
    return *this;
  }

  FakeStatsPluginBuilder& UseDisabledByDefaultMetrics(bool value) {
    use_disabled_by_default_metrics_ = value;
    return *this;
  }

  std::shared_ptr<FakeStatsPlugin> BuildAndRegister() {
    auto f = std::make_shared<FakeStatsPlugin>(
        std::move(channel_filter_), use_disabled_by_default_metrics_);
    GlobalStatsPluginRegistry::RegisterStatsPlugin(f);
    return f;
  }

 private:
  absl::AnyInvocable<bool(
      const experimental::StatsPluginChannelScope& /*scope*/) const>
      channel_filter_;
  bool use_disabled_by_default_metrics_ = false;
};

std::shared_ptr<FakeStatsPlugin> MakeStatsPluginForTarget(
    absl::string_view target_suffix);

class GlobalInstrumentsRegistryTestPeer {
 public:
  static void ResetGlobalInstrumentsRegistry();

  static std::optional<GlobalInstrumentsRegistry::GlobalInstrumentHandle>
  FindUInt64CounterHandleByName(absl::string_view name);
  static std::optional<GlobalInstrumentsRegistry::GlobalInstrumentHandle>
  FindDoubleCounterHandleByName(absl::string_view name);
  static std::optional<GlobalInstrumentsRegistry::GlobalInstrumentHandle>
  FindUInt64HistogramHandleByName(absl::string_view name);
  static std::optional<GlobalInstrumentsRegistry::GlobalInstrumentHandle>
  FindDoubleHistogramHandleByName(absl::string_view name);
  static std::optional<GlobalInstrumentsRegistry::GlobalInstrumentHandle>
  FindCallbackInt64GaugeHandleByName(absl::string_view name);
  static std::optional<GlobalInstrumentsRegistry::GlobalInstrumentHandle>
  FindCallbackDoubleGaugeHandleByName(absl::string_view name);

  static GlobalInstrumentsRegistry::GlobalInstrumentDescriptor*
  FindMetricDescriptorByName(absl::string_view name);
};

class GlobalStatsPluginRegistryTestPeer {
 public:
  static void ResetGlobalStatsPluginRegistry() {
    GlobalStatsPluginRegistry::GlobalStatsPluginNode* node =
        GlobalStatsPluginRegistry::plugins_.exchange(nullptr,
                                                     std::memory_order_acq_rel);
    while (node != nullptr) {
      GlobalStatsPluginRegistry::GlobalStatsPluginNode* next = node->next;
      delete node;
      node = next;
    }
  }
};

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_FAKE_STATS_PLUGIN_H
