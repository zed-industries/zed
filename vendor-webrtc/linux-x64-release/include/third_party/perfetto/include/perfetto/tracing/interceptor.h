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

#ifndef INCLUDE_PERFETTO_TRACING_INTERCEPTOR_H_
#define INCLUDE_PERFETTO_TRACING_INTERCEPTOR_H_

// An interceptor is used to redirect trace packets written by a data source
// into a custom backend instead of the normal Perfetto tracing service. For
// example, the console interceptor prints all trace packets to the console as
// they are generated. Another potential use is exporting trace data to another
// tracing service such as Android ATrace or Windows ETW.
//
// An interceptor is defined by subclassing the perfetto::Interceptor template:
//
// class MyInterceptor : public perfetto::Interceptor<MyInterceptor> {
//  public:
//   ~MyInterceptor() override = default;
//
//   // This function is called for each intercepted trace packet. |context|
//   // contains information about the trace packet as well as other state
//   // tracked by the interceptor (e.g., see ThreadLocalState).
//   //
//   // Intercepted trace data is provided in the form of serialized protobuf
//   // bytes, accessed through the |context.packet_data| field.
//   //
//   // Warning: this function can be called on any thread at any time. See
//   // below for how to safely access shared interceptor data from here.
//   static void OnTracePacket(InterceptorContext context) {
//     perfetto::protos::pbzero::TracePacket::Decoder packet(
//         context.packet_data.data, context.packet_data.size);
//     // ... Write |packet| to the desired destination ...
//   }
// };
//
// An interceptor should be registered before any tracing sessions are started.
// Note that the interceptor also needs to be activated through the trace config
// as shown below.
//
//   perfetto::InterceptorDescriptor desc;
//   desc.set_name("my_interceptor");
//   MyInterceptor::Register(desc);
//
// Finally, an interceptor is enabled through the trace config like this:
//
//   perfetto::TraceConfig cfg;
//   auto* ds_cfg = cfg.add_data_sources()->mutable_config();
//   ds_cfg->set_name("data_source_to_intercept");   // e.g. "track_event"
//   ds_cfg->mutable_interceptor_config()->set_name("my_interceptor");
//
// Once an interceptor is enabled, all data from the affected data sources is
// sent to the interceptor instead of the main tracing buffer.
//
// Interceptor state
// =================
//
// Besides the serialized trace packet data, the |OnTracePacket| interceptor
// function can access three other types of state:
//
// 1. Global state: this is no different from a normal static function, but care
//    must be taken because |OnTracePacket| can be called concurrently on any
//    thread at any time.
//
// 2. Per-data source instance state: since the interceptor class is
//    automatically instantiated for each intercepted data source, its fields
//    can be used to store per-instance data such as the trace config. This data
//    can be maintained through the OnSetup/OnStart/OnStop callbacks:
//
//    class MyInterceptor : public perfetto::Interceptor<MyInterceptor> {
//     public:
//      void OnSetup(const SetupArgs& args) override {
//        enable_foo_ = args.config.interceptor_config().enable_foo();
//      }
//
//      bool enable_foo_{};
//    };
//
//    In the interceptor function this data must be accessed through a scoped
//    lock for safety:
//
//    class MyInterceptor : public perfetto::Interceptor<MyInterceptor> {
//      ...
//      static void OnTracePacket(InterceptorContext context) {
//        auto my_interceptor = context.GetInterceptorLocked();
//        if (my_interceptor) {
//           // Access fields of MyInterceptor here.
//           if (my_interceptor->enable_foo_) { ... }
//        }
//        ...
//      }
//    };
//
//    Since accessing this data involves holding a lock, it should be done
//    sparingly.
//
// 3. Per-thread/TraceWriter state: many data sources use interning to avoid
//    repeating common data in the trace. Since the interning dictionaries are
//    typically kept individually for each TraceWriter sequence (i.e., per
//    thread), an interceptor can declare a data structure with lifetime
//    matching the TraceWriter:
//
//    class MyInterceptor : public perfetto::Interceptor<MyInterceptor> {
//     public:
//      struct ThreadLocalState
//          : public perfetto::InterceptorBase::ThreadLocalState {
//        ThreadLocalState(ThreadLocalStateArgs&) override = default;
//        ~ThreadLocalState() override = default;
//
//        std::map<size_t, std::string> event_names;
//      };
//    };
//
//    This per-thread state can then be accessed and maintained in
//    |OnTracePacket| like this:
//
//    class MyInterceptor : public perfetto::Interceptor<MyInterceptor> {
//      ...
//      static void OnTracePacket(InterceptorContext context) {
//        // Updating interned data.
//        auto& tls = context.GetThreadLocalState();
//        if (parsed_packet.sequence_flags() & perfetto::protos::pbzero::
//                TracePacket::SEQ_INCREMENTAL_STATE_CLEARED) {
//          tls.event_names.clear();
//        }
//        for (const auto& entry : parsed_packet.interned_data().event_names())
//          tls.event_names[entry.iid()] = entry.name();
//
//        // Looking up interned data.
//        if (parsed_packet.has_track_event()) {
//          size_t name_iid = parsed_packet.track_event().name_iid();
//          const std::string& event_name = tls.event_names[name_iid];
//        }
//        ...
//      }
//    };
//

#include <functional>

#include "perfetto/protozero/field.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "perfetto/tracing/internal/basic_types.h"
#include "perfetto/tracing/internal/data_source_internal.h"
#include "perfetto/tracing/locked_handle.h"

namespace {
class MockTracingMuxer;
}

namespace perfetto {
namespace protos {
namespace gen {
class DataSourceConfig;
class InterceptorDescriptor;
}  // namespace gen
}  // namespace protos

using protos::gen::InterceptorDescriptor;

namespace internal {
class InterceptorTraceWriter;
class InterceptorTraceWriterTest;
class TracingMuxer;
class TracingMuxerFake;
class TracingMuxerImpl;
}  // namespace internal

// A virtual base class for interceptors. Users should derive from the templated
// subclass below instead of this one.
class PERFETTO_EXPORT_COMPONENT InterceptorBase {
 public:
  virtual ~InterceptorBase();

  // A virtual base class for thread-local state needed by the interceptor.
  // To define your own state, subclass this with the same name in the
  // interceptor class. A reference to the state can then be looked up through
  // context.GetThreadLocalState() in the trace packet interceptor function.
  class PERFETTO_EXPORT_COMPONENT ThreadLocalState {
   public:
    virtual ~ThreadLocalState();
  };

  struct SetupArgs {
    const DataSourceConfig& config;
  };
  struct StartArgs {};
  struct StopArgs {};

  // Called when an intercepted data source is set up. Both the interceptor's
  // and the data source's configuration is available in
  // |SetupArgs|. Called on an internal Perfetto service thread, but not
  // concurrently.
  virtual void OnSetup(const SetupArgs&) {}

  // Called when an intercepted data source starts. Called on an internal
  // Perfetto service thread, but not concurrently.
  virtual void OnStart(const StartArgs&) {}

  // Called when an intercepted data source stops. Called on an internal
  // Perfetto service thread, but not concurrently.
  virtual void OnStop(const StopArgs&) {}

 private:
  friend class internal::InterceptorTraceWriter;
  friend class internal::InterceptorTraceWriterTest;
  friend class internal::TracingMuxer;
  friend class internal::TracingMuxerFake;
  friend class internal::TracingMuxerImpl;
  friend MockTracingMuxer;
  template <class T>
  friend class Interceptor;

  // Data passed from DataSource::Trace() into the interceptor.
  struct TracePacketCallbackArgs {
    internal::DataSourceStaticState* static_state;
    uint32_t instance_index;
    protozero::ConstBytes packet_data;
    ThreadLocalState* tls;
  };

  // These callback functions are defined as stateless to avoid accidentally
  // introducing cross-thread data races.
  using TLSFactory = std::unique_ptr<ThreadLocalState> (*)(
      internal::DataSourceStaticState*,
      uint32_t data_source_instance_index);
  using TracePacketCallback = void (*)(TracePacketCallbackArgs);

  static void RegisterImpl(
      const InterceptorDescriptor& descriptor,
      std::function<std::unique_ptr<InterceptorBase>()> factory,
      InterceptorBase::TLSFactory tls_factory,
      InterceptorBase::TracePacketCallback on_trace_packet);
};

// Templated interceptor instantiation. See above for usage.
template <class InterceptorType>
class PERFETTO_EXPORT_COMPONENT Interceptor : public InterceptorBase {
 public:
  // A context object provided to the ThreadLocalState constructor. Provides
  // access to the per-instance interceptor object.
  class ThreadLocalStateArgs {
   public:
    ~ThreadLocalStateArgs() = default;

    ThreadLocalStateArgs(const ThreadLocalStateArgs&) = delete;
    ThreadLocalStateArgs& operator=(const ThreadLocalStateArgs&) = delete;

    ThreadLocalStateArgs(ThreadLocalStateArgs&&) noexcept = default;
    ThreadLocalStateArgs& operator=(ThreadLocalStateArgs&&) noexcept = default;

    // Return a locked reference to the interceptor session. The session object
    // will remain valid as long as the returned handle is in scope.
    LockedHandle<InterceptorType> GetInterceptorLocked() {
      auto* internal_state = static_state_->TryGet(data_source_instance_index_);
      if (!internal_state)
        return LockedHandle<InterceptorType>();
      std::unique_lock<std::recursive_mutex> lock(internal_state->lock);
      return LockedHandle<InterceptorType>(
          std::move(lock),
          static_cast<InterceptorType*>(internal_state->interceptor.get()));
    }

   private:
    friend class Interceptor<InterceptorType>;
    friend class InterceptorContext;
    friend class TracingMuxerImpl;

    ThreadLocalStateArgs(internal::DataSourceStaticState* static_state,
                         uint32_t data_source_instance_index)
        : static_state_(static_state),
          data_source_instance_index_(data_source_instance_index) {}

    internal::DataSourceStaticState* const static_state_;
    const uint32_t data_source_instance_index_;
  };

  // A context object provided to each call into |OnTracePacket|. Contains the
  // intercepted serialized trace packet data.
  class InterceptorContext {
   public:
    InterceptorContext(InterceptorContext&&) noexcept = default;
    ~InterceptorContext() = default;

    // Return a locked reference to the interceptor session. The session object
    // will remain valid as long as the returned handle is in scope.
    LockedHandle<InterceptorType> GetInterceptorLocked() {
      return tls_args_.GetInterceptorLocked();
    }

    // Return the thread-local state for this interceptor. See
    // InterceptorBase::ThreadLocalState.
    typename InterceptorType::ThreadLocalState& GetThreadLocalState() {
      return static_cast<typename InterceptorType::ThreadLocalState&>(*tls_);
    }

    // A buffer containing the serialized TracePacket protocol buffer message.
    // This memory is only valid during the call to OnTracePacket.
    protozero::ConstBytes packet_data;

   private:
    friend class Interceptor<InterceptorType>;
    InterceptorContext(TracePacketCallbackArgs args)
        : packet_data(args.packet_data),
          tls_args_(args.static_state, args.instance_index),
          tls_(args.tls) {}
    InterceptorContext(const InterceptorContext&) = delete;
    InterceptorContext& operator=(const InterceptorContext&) = delete;

    ThreadLocalStateArgs tls_args_;
    InterceptorBase::ThreadLocalState* const tls_;
  };

  // Register the interceptor for use in tracing sessions.
  // The optional |constructor_args| will be passed to the interceptor when it
  // is constructed.
  template <class... Args>
  static void Register(const InterceptorDescriptor& descriptor,
                       const Args&... constructor_args) {
    auto factory = [constructor_args...]() {
      return std::unique_ptr<InterceptorBase>(
          new InterceptorType(constructor_args...));
    };
    auto tls_factory = [](internal::DataSourceStaticState* static_state,
                          uint32_t data_source_instance_index) {
      // Don't bother allocating TLS state unless the interceptor is actually
      // using it.
      if (std::is_same<typename InterceptorType::ThreadLocalState,
                       InterceptorBase::ThreadLocalState>::value) {
        return std::unique_ptr<InterceptorBase::ThreadLocalState>(nullptr);
      }
      ThreadLocalStateArgs args(static_state, data_source_instance_index);
      return std::unique_ptr<InterceptorBase::ThreadLocalState>(
          new typename InterceptorType::ThreadLocalState(args));
    };
    auto on_trace_packet = [](TracePacketCallbackArgs args) {
      InterceptorType::OnTracePacket(InterceptorContext(std::move(args)));
    };
    RegisterImpl(descriptor, std::move(factory), std::move(tls_factory),
                 std::move(on_trace_packet));
  }
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERCEPTOR_H_
