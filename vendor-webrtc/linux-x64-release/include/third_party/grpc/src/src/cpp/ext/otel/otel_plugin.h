//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CPP_EXT_OTEL_OTEL_PLUGIN_H
#define GRPC_SRC_CPP_EXT_OTEL_OTEL_PLUGIN_H

#include <grpc/support/port_platform.h>
#include <grpcpp/ext/otel_plugin.h>
#include <grpcpp/impl/server_builder_option.h>
#include <stddef.h>
#include <stdint.h>

#include <bitset>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "opentelemetry/metrics/async_instruments.h"
#include "opentelemetry/metrics/meter_provider.h"
#include "opentelemetry/metrics/observer_result.h"
#include "opentelemetry/metrics/sync_instruments.h"
#include "opentelemetry/nostd/shared_ptr.h"
#include "opentelemetry/trace/tracer.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/telemetry/metrics.h"
#include "src/core/util/down_cast.h"

namespace grpc {
namespace internal {

// An iterable container interface that can be used as a return type for the
// OpenTelemetry plugin's label injector.
class LabelsIterable {
 public:
  virtual ~LabelsIterable() = default;

  // Returns the key-value label at the current position or std::nullopt if the
  // iterator has reached the end.
  virtual std::optional<std::pair<absl::string_view, absl::string_view>>
  Next() = 0;

  virtual size_t Size() const = 0;

  // Resets position of iterator to the start.
  virtual void ResetIteratorPosition() = 0;
};

// An interface that allows you to add additional labels on the calls traced
// through the OpenTelemetry plugin.
class LabelsInjector {
 public:
  virtual ~LabelsInjector() {}
  // Read the incoming initial metadata to get the set of labels to be added to
  // metrics.
  virtual std::unique_ptr<LabelsIterable> GetLabels(
      grpc_metadata_batch* incoming_initial_metadata) const = 0;

  // Modify the outgoing initial metadata with metadata information to be sent
  // to the peer. On the server side, \a labels_from_incoming_metadata returned
  // from `GetLabels` should be provided as input here. On the client side, this
  // should be nullptr.
  virtual void AddLabels(
      grpc_metadata_batch* outgoing_initial_metadata,
      LabelsIterable* labels_from_incoming_metadata) const = 0;

  // Adds optional labels to the traced calls. Each entry in the span
  // corresponds to the CallAttemptTracer::OptionalLabelComponent enum. Returns
  // false when callback returns false.
  virtual bool AddOptionalLabels(
      bool is_client,
      absl::Span<const grpc_core::RefCountedStringValue> optional_labels,
      opentelemetry::nostd::function_ref<
          bool(opentelemetry::nostd::string_view,
               opentelemetry::common::AttributeValue)>
          callback) const = 0;

  // Gets the actual size of the optional labels that the Plugin is going to
  // produce through the AddOptionalLabels method.
  virtual size_t GetOptionalLabelsSize(
      bool is_client,
      absl::Span<const grpc_core::RefCountedStringValue> optional_labels)
      const = 0;
};

class InternalOpenTelemetryPluginOption
    : public grpc::OpenTelemetryPluginOption {
 public:
  ~InternalOpenTelemetryPluginOption() override = default;
  // Determines whether a plugin option is active on a given channel target
  virtual bool IsActiveOnClientChannel(absl::string_view target) const = 0;
  // Determines whether a plugin option is active on a given server
  virtual bool IsActiveOnServer(const grpc_core::ChannelArgs& args) const = 0;
  // Returns the LabelsInjector used by this plugin option, nullptr if none.
  virtual const grpc::internal::LabelsInjector* labels_injector() const = 0;
};

// Tags
absl::string_view OpenTelemetryMethodKey();
absl::string_view OpenTelemetryStatusKey();
absl::string_view OpenTelemetryTargetKey();

class OpenTelemetryPluginBuilderImpl {
 public:
  OpenTelemetryPluginBuilderImpl();
  ~OpenTelemetryPluginBuilderImpl();
  // If `SetMeterProvider()` is not called, no metrics are collected.
  OpenTelemetryPluginBuilderImpl& SetMeterProvider(
      std::shared_ptr<opentelemetry::metrics::MeterProvider> meter_provider);
  // Methods to manipulate which instruments are enabled in the OpenTelemetry
  // Stats Plugin. The default set of instruments are -
  // grpc.client.attempt.started
  // grpc.client.attempt.duration
  // grpc.client.attempt.sent_total_compressed_message_size
  // grpc.client.attempt.rcvd_total_compressed_message_size
  // grpc.server.call.started
  // grpc.server.call.duration
  // grpc.server.call.sent_total_compressed_message_size
  // grpc.server.call.rcvd_total_compressed_message_size
  OpenTelemetryPluginBuilderImpl& EnableMetrics(
      absl::Span<const absl::string_view> metric_names);
  OpenTelemetryPluginBuilderImpl& DisableMetrics(
      absl::Span<const absl::string_view> metric_names);
  OpenTelemetryPluginBuilderImpl& DisableAllMetrics();
  // If set, \a server_selector is called per incoming call on the server
  // to decide whether to collect metrics on that call or not.
  // TODO(yashkt): We should only need to do this per server connection or even
  // per server. Change this when we have a ServerTracer.
  OpenTelemetryPluginBuilderImpl& SetServerSelector(
      absl::AnyInvocable<bool(const grpc_core::ChannelArgs& /*args*/) const>
          server_selector);
  // If set, \a target_attribute_filter is called per channel to decide whether
  // to record the target attribute on client or to replace it with "other".
  // This helps reduce the cardinality on metrics in cases where many channels
  // are created with different targets in the same binary (which might happen
  // for example, if the channel target string uses IP addresses directly).
  OpenTelemetryPluginBuilderImpl& SetTargetAttributeFilter(
      absl::AnyInvocable<bool(absl::string_view /*target*/) const>
          target_attribute_filter);
  // If set, \a generic_method_attribute_filter is called per call with a
  // generic method type to decide whether to record the method name or to
  // replace it with "other". Non-generic or pre-registered methods remain
  // unaffected. If not set, by default, generic method names are replaced with
  // "other" when recording metrics.
  OpenTelemetryPluginBuilderImpl& SetGenericMethodAttributeFilter(
      absl::AnyInvocable<bool(absl::string_view /*generic_method*/) const>
          generic_method_attribute_filter);
  OpenTelemetryPluginBuilderImpl& AddPluginOption(
      std::unique_ptr<InternalOpenTelemetryPluginOption> option);
  // Records \a optional_label_key on all metrics that provide it.
  OpenTelemetryPluginBuilderImpl& AddOptionalLabel(
      absl::string_view optional_label_key);
  // If `SetTracerProvider()` is not called, no traces are collected.
  OpenTelemetryPluginBuilderImpl& SetTracerProvider(
      std::shared_ptr<opentelemetry::trace::TracerProvider> tracer_provider);
  // Set one or multiple text map propagators for span context propagation,
  // e.g. the community standard ones like W3C, etc.
  OpenTelemetryPluginBuilderImpl& SetTextMapPropagator(
      std::unique_ptr<opentelemetry::context::propagation::TextMapPropagator>
          text_map_propagator);
  // Set scope filter to choose which channels are recorded by this plugin.
  // Server-side recording remains unaffected.
  OpenTelemetryPluginBuilderImpl& SetChannelScopeFilter(
      absl::AnyInvocable<
          bool(const OpenTelemetryPluginBuilder::ChannelScope& /*scope*/) const>
          channel_scope_filter);
  absl::Status BuildAndRegisterGlobal();
  absl::StatusOr<std::shared_ptr<grpc::experimental::OpenTelemetryPlugin>>
  Build();

  const absl::flat_hash_set<std::string>& TestOnlyEnabledMetrics() {
    return metrics_;
  }

 private:
  std::shared_ptr<opentelemetry::metrics::MeterProvider> meter_provider_;
  std::unique_ptr<LabelsInjector> labels_injector_;
  absl::AnyInvocable<bool(absl::string_view /*target*/) const>
      target_attribute_filter_;
  absl::flat_hash_set<std::string> metrics_;
  absl::AnyInvocable<bool(absl::string_view /*generic_method*/) const>
      generic_method_attribute_filter_;
  absl::AnyInvocable<bool(const grpc_core::ChannelArgs& /*args*/) const>
      server_selector_;
  std::vector<std::unique_ptr<InternalOpenTelemetryPluginOption>>
      plugin_options_;
  std::set<absl::string_view> optional_label_keys_;
  std::shared_ptr<opentelemetry::trace::TracerProvider> tracer_provider_;

  std::unique_ptr<opentelemetry::context::propagation::TextMapPropagator>
      text_map_propagator_;
  absl::AnyInvocable<bool(
      const OpenTelemetryPluginBuilder::ChannelScope& /*scope*/) const>
      channel_scope_filter_;
};

class OpenTelemetryPluginImpl
    : public grpc::experimental::OpenTelemetryPlugin,
      public grpc_core::StatsPlugin,
      public std::enable_shared_from_this<OpenTelemetryPluginImpl> {
 public:
  OpenTelemetryPluginImpl(
      const absl::flat_hash_set<std::string>& metrics,
      opentelemetry::nostd::shared_ptr<opentelemetry::metrics::MeterProvider>
          meter_provider,
      absl::AnyInvocable<bool(absl::string_view /*target*/) const>
          target_attribute_filter,
      absl::AnyInvocable<bool(absl::string_view /*generic_method*/) const>
          generic_method_attribute_filter,
      absl::AnyInvocable<bool(const grpc_core::ChannelArgs& /*args*/) const>
          server_selector,
      std::vector<std::unique_ptr<InternalOpenTelemetryPluginOption>>
          plugin_options,
      const std::set<absl::string_view>& optional_label_keys,
      std::shared_ptr<opentelemetry::trace::TracerProvider> tracer_provider,
      std::unique_ptr<opentelemetry::context::propagation::TextMapPropagator>
          text_map_propagator,
      absl::AnyInvocable<
          bool(const OpenTelemetryPluginBuilder::ChannelScope& /*scope*/) const>
          channel_scope_filter);
  ~OpenTelemetryPluginImpl() override;

 private:
  class ClientCallTracer;
  class KeyValueIterable;
  class NPCMetricsKeyValueIterable;
  class ServerCallTracer;

  // Creates a convenience wrapper to help iterate over only those plugin
  // options that are active over a given channel/server.
  class ActivePluginOptionsView {
   public:
    static ActivePluginOptionsView MakeForClient(
        absl::string_view target, const OpenTelemetryPluginImpl* otel_plugin) {
      return ActivePluginOptionsView(
          [target](const InternalOpenTelemetryPluginOption& plugin_option) {
            return plugin_option.IsActiveOnClientChannel(target);
          },
          otel_plugin);
    }

    static ActivePluginOptionsView MakeForServer(
        const grpc_core::ChannelArgs& args,
        const OpenTelemetryPluginImpl* otel_plugin) {
      return ActivePluginOptionsView(
          [&args](const InternalOpenTelemetryPluginOption& plugin_option) {
            return plugin_option.IsActiveOnServer(args);
          },
          otel_plugin);
    }

    bool ForEach(absl::FunctionRef<
                     bool(const InternalOpenTelemetryPluginOption&, size_t)>
                     func,
                 const OpenTelemetryPluginImpl* otel_plugin) const {
      for (size_t i = 0; i < otel_plugin->plugin_options().size(); ++i) {
        const auto& plugin_option = otel_plugin->plugin_options()[i];
        if (active_mask_[i] && !func(*plugin_option, i)) {
          return false;
        }
      }
      return true;
    }

    int Compare(const ActivePluginOptionsView& other) const {
      return grpc_core::QsortCompare(active_mask_.to_ulong(),
                                     other.active_mask_.to_ulong());
    }

   private:
    explicit ActivePluginOptionsView(
        absl::FunctionRef<bool(const InternalOpenTelemetryPluginOption&)> func,
        const OpenTelemetryPluginImpl* otel_plugin) {
      for (size_t i = 0; i < otel_plugin->plugin_options().size(); ++i) {
        const auto& plugin_option = otel_plugin->plugin_options()[i];
        if (plugin_option != nullptr && func(*plugin_option)) {
          active_mask_.set(i);
        }
      }
    }

    std::bitset<64> active_mask_;
  };

  class ClientScopeConfig : public grpc_core::StatsPlugin::ScopeConfig {
   public:
    ClientScopeConfig(const OpenTelemetryPluginImpl* otel_plugin,
                      const OpenTelemetryPluginBuilder::ChannelScope& scope)
        : active_plugin_options_view_(ActivePluginOptionsView::MakeForClient(
              scope.target(), otel_plugin)),
          filtered_target_(
              // Use the original target string only if a filter on the
              // attribute is not registered or if the filter returns true,
              // otherwise use "other".
              otel_plugin->target_attribute_filter() == nullptr ||
                      otel_plugin->target_attribute_filter()(scope.target())
                  ? scope.target()
                  : "other") {}

    int Compare(const ScopeConfig& other) const override {
      const auto& o = grpc_core::DownCast<const ClientScopeConfig&>(other);
      int r = grpc_core::QsortCompare(filtered_target_, o.filtered_target_);
      if (r != 0) return r;
      return active_plugin_options_view_.Compare(o.active_plugin_options_view_);
    }

    const ActivePluginOptionsView& active_plugin_options_view() const {
      return active_plugin_options_view_;
    }

    absl::string_view filtered_target() const { return filtered_target_; }

   private:
    ActivePluginOptionsView active_plugin_options_view_;
    std::string filtered_target_;
  };

  class ServerScopeConfig : public grpc_core::StatsPlugin::ScopeConfig {
   public:
    ServerScopeConfig(const OpenTelemetryPluginImpl* otel_plugin,
                      const grpc_core::ChannelArgs& args)
        : active_plugin_options_view_(
              ActivePluginOptionsView::MakeForServer(args, otel_plugin)) {}

    int Compare(const ScopeConfig& other) const override {
      const auto& o = grpc_core::DownCast<const ServerScopeConfig&>(other);
      return active_plugin_options_view_.Compare(o.active_plugin_options_view_);
    }

    const ActivePluginOptionsView& active_plugin_options_view() const {
      return active_plugin_options_view_;
    }

   private:
    ActivePluginOptionsView active_plugin_options_view_;
  };

  struct ClientMetrics {
    struct Attempt {
      std::unique_ptr<opentelemetry::metrics::Counter<uint64_t>> started;
      std::unique_ptr<opentelemetry::metrics::Histogram<double>> duration;
      std::unique_ptr<opentelemetry::metrics::Histogram<uint64_t>>
          sent_total_compressed_message_size;
      std::unique_ptr<opentelemetry::metrics::Histogram<uint64_t>>
          rcvd_total_compressed_message_size;
    } attempt;
  };
  struct ServerMetrics {
    struct Call {
      std::unique_ptr<opentelemetry::metrics::Counter<uint64_t>> started;
      std::unique_ptr<opentelemetry::metrics::Histogram<double>> duration;
      std::unique_ptr<opentelemetry::metrics::Histogram<uint64_t>>
          sent_total_compressed_message_size;
      std::unique_ptr<opentelemetry::metrics::Histogram<uint64_t>>
          rcvd_total_compressed_message_size;
    } call;
  };

  // This object should be used inline.
  class CallbackMetricReporter : public grpc_core::CallbackMetricReporter {
   public:
    CallbackMetricReporter(OpenTelemetryPluginImpl* ot_plugin,
                           grpc_core::RegisteredMetricCallback* key)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(ot_plugin->mu_);

   private:
    void ReportInt64(
        grpc_core::GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
        int64_t value, absl::Span<const absl::string_view> label_values,
        absl::Span<const absl::string_view> optional_values)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(
            CallbackGaugeState<int64_t>::ot_plugin->mu_) override;
    void ReportDouble(
        grpc_core::GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
        double value, absl::Span<const absl::string_view> label_values,
        absl::Span<const absl::string_view> optional_values)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(
            CallbackGaugeState<double>::ot_plugin->mu_) override;

    OpenTelemetryPluginImpl* ot_plugin_;
    grpc_core::RegisteredMetricCallback* key_;
  };

  class ServerBuilderOption : public grpc::ServerBuilderOption {
   public:
    explicit ServerBuilderOption(
        std::shared_ptr<OpenTelemetryPluginImpl> plugin)
        : plugin_(std::move(plugin)) {}
    void UpdateArguments(grpc::ChannelArguments* args) override;
    void UpdatePlugins(std::vector<std::unique_ptr<grpc::ServerBuilderPlugin>>*
                       /*plugins*/) override {}

   private:
    std::shared_ptr<OpenTelemetryPluginImpl> plugin_;
  };

  // Returns the string form of \a key
  static absl::string_view OptionalLabelKeyToString(
      grpc_core::ClientCallTracer::CallAttemptTracer::OptionalLabelKey key);

  // Returns the OptionalLabelKey form of \a key if \a key is recognized and
  // is public, std::nullopt otherwise.
  static std::optional<
      grpc_core::ClientCallTracer::CallAttemptTracer::OptionalLabelKey>
  OptionalLabelStringToKey(absl::string_view key);

  static absl::string_view GetMethodFromPath(const grpc_core::Slice& path);

  // grpc::OpenTelemetryPlugin:
  void AddToChannelArguments(grpc::ChannelArguments* args) override;
  void AddToServerBuilder(grpc::ServerBuilder* builder) override;

  // StatsPlugin:
  std::pair<bool, std::shared_ptr<grpc_core::StatsPlugin::ScopeConfig>>
  IsEnabledForChannel(
      const OpenTelemetryPluginBuilder::ChannelScope& scope) const override;
  std::pair<bool, std::shared_ptr<grpc_core::StatsPlugin::ScopeConfig>>
  IsEnabledForServer(const grpc_core::ChannelArgs& args) const override;
  std::shared_ptr<grpc_core::StatsPlugin::ScopeConfig> GetChannelScopeConfig(
      const OpenTelemetryPluginBuilder::ChannelScope& scope) const override;
  std::shared_ptr<grpc_core::StatsPlugin::ScopeConfig> GetServerScopeConfig(
      const grpc_core::ChannelArgs& args) const override;
  void AddCounter(
      grpc_core::GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      uint64_t value, absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) override;
  void AddCounter(
      grpc_core::GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      double value, absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) override;
  void RecordHistogram(
      grpc_core::GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      uint64_t value, absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) override;
  void RecordHistogram(
      grpc_core::GlobalInstrumentsRegistry::GlobalInstrumentHandle handle,
      double value, absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) override;
  void AddCallback(grpc_core::RegisteredMetricCallback* callback)
      ABSL_LOCKS_EXCLUDED(mu_) override;
  void RemoveCallback(grpc_core::RegisteredMetricCallback* callback)
      ABSL_LOCKS_EXCLUDED(mu_) override;
  grpc_core::ClientCallTracer* GetClientCallTracer(
      const grpc_core::Slice& path, bool registered_method,
      std::shared_ptr<grpc_core::StatsPlugin::ScopeConfig> scope_config)
      override;
  grpc_core::ServerCallTracer* GetServerCallTracer(
      std::shared_ptr<grpc_core::StatsPlugin::ScopeConfig> scope_config)
      override;
  bool IsInstrumentEnabled(
      grpc_core::GlobalInstrumentsRegistry::GlobalInstrumentHandle handle)
      const override;

  const absl::AnyInvocable<bool(const grpc_core::ChannelArgs& /*args*/) const>&
  server_selector() const {
    return server_selector_;
  }
  const absl::AnyInvocable<bool(absl::string_view /*target*/) const>&
  target_attribute_filter() const {
    return target_attribute_filter_;
  }
  const absl::AnyInvocable<bool(absl::string_view /*generic_method*/) const>&
  generic_method_attribute_filter() const {
    return generic_method_attribute_filter_;
  }
  const std::vector<std::unique_ptr<InternalOpenTelemetryPluginOption>>&
  plugin_options() const {
    return plugin_options_;
  }

  template <typename ValueType>
  struct CallbackGaugeState {
    // It's possible to set values for multiple sets of labels at the same time
    // in a single callback. Key is a vector of label values and enabled
    // optional label values.
    using Cache = absl::flat_hash_map<std::vector<std::string>, ValueType>;
    grpc_core::GlobalInstrumentsRegistry::InstrumentID id;
    opentelemetry::nostd::shared_ptr<
        opentelemetry::metrics::ObservableInstrument>
        instrument;
    bool ot_callback_registered ABSL_GUARDED_BY(ot_plugin->mu_);
    // instrument1 ----- RegisteredMetricCallback1
    //               x
    // instrument2 ----- RegisteredMetricCallback2
    // One instrument can be registered by multiple callbacks.
    absl::flat_hash_map<grpc_core::RegisteredMetricCallback*, Cache> caches
        ABSL_GUARDED_BY(ot_plugin->mu_);
    OpenTelemetryPluginImpl* ot_plugin;

    static void CallbackGaugeCallback(
        opentelemetry::metrics::ObserverResult result, void* arg)
        ABSL_LOCKS_EXCLUDED(ot_plugin->mu_);

    void Observe(opentelemetry::metrics::ObserverResult& result,
                 const Cache& cache)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(ot_plugin->mu_);
  };

  // Instruments for per-call metrics.
  ClientMetrics client_;
  ServerMetrics server_;
  static constexpr int kOptionalLabelsSizeLimit = 64;
  using OptionalLabelsBitSet = std::bitset<kOptionalLabelsSizeLimit>;
  OptionalLabelsBitSet per_call_optional_label_bits_;
  // Instruments for non-per-call metrics.
  struct Disabled {};
  using Instrument =
      std::variant<Disabled,
                   std::unique_ptr<opentelemetry::metrics::Counter<uint64_t>>,
                   std::unique_ptr<opentelemetry::metrics::Counter<double>>,
                   std::unique_ptr<opentelemetry::metrics::Histogram<uint64_t>>,
                   std::unique_ptr<opentelemetry::metrics::Histogram<double>>,
                   std::unique_ptr<CallbackGaugeState<int64_t>>,
                   std::unique_ptr<CallbackGaugeState<double>>>;
  struct InstrumentData {
    Instrument instrument;
    OptionalLabelsBitSet optional_labels_bits;
  };
  std::vector<InstrumentData> instruments_data_;
  grpc_core::Mutex mu_;
  absl::flat_hash_map<grpc_core::RegisteredMetricCallback*,
                      grpc_core::Timestamp>
      callback_timestamps_ ABSL_GUARDED_BY(mu_);
  opentelemetry::nostd::shared_ptr<opentelemetry::metrics::MeterProvider>
      meter_provider_;
  absl::AnyInvocable<bool(const grpc_core::ChannelArgs& /*args*/) const>
      server_selector_;
  absl::AnyInvocable<bool(absl::string_view /*target*/) const>
      target_attribute_filter_;
  absl::AnyInvocable<bool(absl::string_view /*generic_method*/) const>
      generic_method_attribute_filter_;
  std::vector<std::unique_ptr<InternalOpenTelemetryPluginOption>>
      plugin_options_;
  std::shared_ptr<opentelemetry::trace::TracerProvider> const tracer_provider_;
  opentelemetry::nostd::shared_ptr<opentelemetry::trace::Tracer> const tracer_;
  std::unique_ptr<opentelemetry::context::propagation::TextMapPropagator> const
      text_map_propagator_;
  absl::AnyInvocable<bool(
      const OpenTelemetryPluginBuilder::ChannelScope& /*scope*/) const>
      channel_scope_filter_;
};

class GrpcTextMapCarrier
    : public opentelemetry::context::propagation::TextMapCarrier {
 public:
  explicit GrpcTextMapCarrier(grpc_metadata_batch* metadata)
      : metadata_(metadata) {}

  opentelemetry::nostd::string_view Get(
      opentelemetry::nostd::string_view key) const noexcept override;

  void Set(opentelemetry::nostd::string_view key,
           opentelemetry::nostd::string_view value) noexcept override;

 private:
  grpc_metadata_batch* metadata_;
};

inline absl::string_view NoStdStringViewToAbslStringView(
    opentelemetry::nostd::string_view string) {
  return absl::string_view(string.data(), string.size());
}

inline opentelemetry::nostd::string_view AbslStringViewToNoStdStringView(
    absl::string_view string) {
  return opentelemetry::nostd::string_view(string.data(), string.size());
}

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_OTEL_OTEL_PLUGIN_H
