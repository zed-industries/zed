// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SYSTEM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SYSTEM_H_

#include "base/time/time.h"
#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "device/vr/public/mojom/xr_session.mojom-blink.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_xr_dom_overlay_init.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_xr_session_init.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/page/focus_changed_observer.h"
#include "third_party/blink/renderer/modules/xr/xr_enter_fullscreen_observer.h"
#include "third_party/blink/renderer/modules/xr/xr_exit_fullscreen_observer.h"
#include "third_party/blink/renderer/modules/xr/xr_session.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Navigator;
class V8XRSessionMode;
class XRFrameProvider;
class XRSession;
class XRSessionInit;

// Implementation of the XRSystem interface according to
// https://immersive-web.github.io/webxr/#xrsystem-interface . This is created
// lazily on first access to the navigator.xr attrib,
// and disposed when the execution context is destroyed or on mojo communication
// errors with the browser/device process.
//
// When the XRSystem is used for promises, it uses query objects to store state
// including the associated ScriptPromiseResolverBase. These query objects are
// owned by the XRSystem and remain alive until the promise is resolved or
// rejected. (See comments below for PendingSupportsSessionQuery and
// PendingRequestSessionQuery.) These query objects are destroyed and any
// outstanding promises rejected when the XRSystem is disposed.
//
// The XRSystem owns mojo connections with the Browser process through
// VRService, used for capability queries and session lifetime
// management. The XRSystem is also the receiver for the VRServiceClient.
//
// The XRSystem owns mojo connections with the Device process (either a
// separate utility process, or implemented as part of the Browser process,
// depending on the runtime and options) through XRFrameProvider and
// XREnvironmentIntegrationProvider. These are used to transport per-frame data
// such as image data and input poses. These are lazily created when first
// needed for a sensor-backed session (all except sensorless inline sessions),
// and destroyed when the XRSystem is disposed.
//
// The XRSystem keeps weak references to XRSession objects after they were
// returned through a successful requestSession promise, but does not own them.
class XRSystem final : public EventTarget,
                       public Supplement<Navigator>,
                       public ExecutionContextLifecycleObserver,
                       public device::mojom::blink::VRServiceClient,
                       public FocusChangedObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static XRSystem* From(Document&);
  static XRSystem* FromIfExists(Document&);

  static XRSystem* xr(Navigator&);

  // TODO(crbug.com/976796): Fix lint errors.
  explicit XRSystem(Navigator&);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(devicechange, kDevicechange)

  ScriptPromise<IDLBoolean> isSessionSupported(ScriptState*,
                                               const V8XRSessionMode&,
                                               ExceptionState& exception_state);
  ScriptPromise<XRSession> requestSession(ScriptState*,
                                          const V8XRSessionMode&,
                                          XRSessionInit*,
                                          ExceptionState& exception_state);

  XRFrameProvider* frameProvider();

  device::mojom::blink::XREnvironmentIntegrationProvider*
  xrEnvironmentProviderRemote();

  device::mojom::blink::VRService* BrowserService();

  // VRServiceClient overrides.
  void OnDeviceChanged() override;

  // EventTarget overrides.
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  // ExecutionContextLifecycleObserver overrides.
  void ContextDestroyed() override;
  void Trace(Visitor*) const override;

  // FocusChangedObserver overrides.
  void FocusedFrameChanged() override;
  bool IsFrameFocused();

  using EnvironmentProviderErrorCallback = base::OnceCallback<void()>;
  // Registers a callback that'll be invoked when mojo invokes a disconnect
  // handler on the underlying XREnvironmentIntegrationProvider remote.
  void AddEnvironmentProviderErrorHandler(
      EnvironmentProviderErrorCallback callback);

  void ExitPresent(base::OnceClosure on_exited);

  void SetFramesThrottled(const XRSession* session, bool throttled);

  base::TimeTicks NavigationStart() const { return navigation_start_; }

  bool IsContextDestroyed() const { return is_context_destroyed_; }

  void MakeXrCompatibleAsync(
      device::mojom::blink::VRService::MakeXrCompatibleCallback callback);
  void MakeXrCompatibleSync(
      device::mojom::XrCompatibleResult* xr_compatible_result);

  void OnSessionEnded(XRSession* session);

  device::mojom::blink::WebXrInternalsRendererListener*
  GetWebXrInternalsRendererListener();

  void AddWebXrInternalsMessage(const String& message);

 private:
  void DisableBackForwardCache();

  enum SensorRequirement {
    kNone,
    kOptional,
    kRequired,
  };

  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class SessionRequestStatus : int {
    // `requestSession` succeeded.
    kSuccess = 0,
    // `requestSession` failed with other (unknown) error.
    kOtherError = 1,
    kMaxValue = kOtherError,
  };

  struct RequestedXRSessionFeatureSet {
    // Set of requested features which are known and valid for the current mode.
    XRSessionFeatureSet valid_features;

    // Whether any requested features were unknown or invalid
    bool invalid_features = false;
  };

  // Encapsulates blink-side `XRSystem::requestSession()` call. It is a wrapper
  // around ScriptPromiseResolverBase that allows us to add additional logic as
  // certain things related to promise's life cycle happen. Instances are owned
  // by the XRSystem, see outstanding_request_queries_ below.
  class PendingRequestSessionQuery final
      : public GarbageCollected<PendingRequestSessionQuery> {
   public:
    PendingRequestSessionQuery(int64_t ukm_source_id,
                               ScriptPromiseResolver<XRSession>* resolver,
                               device::mojom::blink::XRSessionMode mode,
                               RequestedXRSessionFeatureSet required_features,
                               RequestedXRSessionFeatureSet optional_features);

    PendingRequestSessionQuery(const PendingRequestSessionQuery&) = delete;
    PendingRequestSessionQuery& operator=(const PendingRequestSessionQuery&) =
        delete;

    ~PendingRequestSessionQuery() = default;

    // Resolves underlying promise with passed in XR session.
    // If metrics are to be recorded for this session, an
    // |XRSessionMetricsRecorded| may be passed in as well.
    void Resolve(
        XRSession* session,
        mojo::PendingRemote<device::mojom::blink::XRSessionMetricsRecorder>
            metrics_recorder = mojo::NullRemote());

    // Rejects underlying promise with a DOMException.
    // Do not call this with |DOMExceptionCode::kSecurityError|, use
    // |RejectWithSecurityError| for that. If the exception is thrown
    // synchronously, an ExceptionState must be passed in. Otherwise it may be
    // null. Care must be taken when setting |message| - it will be accessible
    // to the application and should not contain any sensitive data.
    void RejectWithDOMException(DOMExceptionCode exception_code,
                                const String& message,
                                ExceptionState* exception_state);

    // Rejects underlying promise with a SecurityError.
    // If the exception is thrown synchronously, an ExceptionState must
    // be passed in. Otherwise it may be null. Care must be taken when setting
    // |message| - it will be accessible to the application and should not
    // contain any sensitive data.
    void RejectWithSecurityError(const String& message,
                                 ExceptionState* exception_state);

    // Rejects underlying promise with a TypeError.
    // If the exception is thrown synchronously, an ExceptionState must
    // be passed in. Otherwise it may be null. Care must be taken when setting
    // |message| - it will be accessible to the application and should not
    // contain any sensitive data.
    void RejectWithTypeError(const String& message,
                             ExceptionState* exception_state);

    device::mojom::blink::XRSessionMode mode() const;
    const XRSessionFeatureSet& RequiredFeatures() const;
    const XRSessionFeatureSet& OptionalFeatures() const;
    bool InvalidRequiredFeatures() const;
    bool InvalidOptionalFeatures() const;
    bool HasFeature(device::mojom::XRSessionFeature) const;

    SensorRequirement GetSensorRequirement() const {
      return sensor_requirement_;
    }

    // Returns underlying resolver's script state.
    ScriptState* GetScriptState() const;

    void SetDOMOverlayElement(Element* element) {
      dom_overlay_element_ = element;
    }
    Element* DOMOverlayElement() { return dom_overlay_element_.Get(); }

    void SetTrackedImages(
        const Vector<device::mojom::blink::XRTrackedImage>& images) {
      tracked_images_ = images;
    }
    Vector<device::mojom::blink::XRTrackedImage> TrackedImages() const {
      return tracked_images_;
    }

    void SetDepthSensingConfiguration(
        const Vector<device::mojom::XRDepthUsage>& preferred_usage,
        const Vector<device::mojom::XRDepthDataFormat>& preferred_format,
        const Vector<device::mojom::XRDepthType>& type_request) {
      preferred_usage_ = preferred_usage;
      preferred_format_ = preferred_format;
      depth_type_request_ = type_request;
    }

    const Vector<device::mojom::XRDepthUsage>& PreferredUsage() const {
      return preferred_usage_;
    }

    const Vector<device::mojom::XRDepthDataFormat>& PreferredFormat() const {
      return preferred_format_;
    }

    const Vector<device::mojom::XRDepthType>& DepthTypeRequest() const {
      return depth_type_request_;
    }

    uint64_t TraceId() const { return trace_id_; }

    void Trace(Visitor*) const;

   private:
    void ParseSensorRequirement();
    device::mojom::XRSessionFeatureRequestStatus GetFeatureRequestStatus(
        device::mojom::XRSessionFeature feature,
        const XRSession* session) const;
    void ReportRequestSessionResult(
        SessionRequestStatus status,
        XRSession* session = nullptr,
        mojo::PendingRemote<device::mojom::blink::XRSessionMetricsRecorder>
            metrics_recorder = mojo::NullRemote());

    Member<ScriptPromiseResolver<XRSession>> resolver_;
    const device::mojom::blink::XRSessionMode mode_;
    RequestedXRSessionFeatureSet required_features_;
    RequestedXRSessionFeatureSet optional_features_;
    SensorRequirement sensor_requirement_ = SensorRequirement::kNone;

    const int64_t ukm_source_id_;

    // Used for trace calls in order to correlate this request across processes.
    const uint64_t trace_id_;

    Member<Element> dom_overlay_element_;

    Vector<device::mojom::blink::XRTrackedImage> tracked_images_;

    Vector<device::mojom::XRDepthUsage> preferred_usage_;
    Vector<device::mojom::XRDepthDataFormat> preferred_format_;
    Vector<device::mojom::XRDepthType> depth_type_request_;
  };

  static device::mojom::blink::XRSessionOptionsPtr XRSessionOptionsFromQuery(
      const PendingRequestSessionQuery& query);

  // Encapsulates blink-side `XRSystem::isSessionSupported()` call. It is a
  // wrapper around ScriptPromiseResolverBase that allows us to add additional
  // logic as certain things related to promise's life cycle happen. Instances
  // are owned by the XRSystem, see outstanding_support_queries_ below.
  class PendingSupportsSessionQuery final
      : public GarbageCollected<PendingSupportsSessionQuery> {
   public:
    PendingSupportsSessionQuery(ScriptPromiseResolverBase*,
                                device::mojom::blink::XRSessionMode);

    PendingSupportsSessionQuery(const PendingSupportsSessionQuery&) = delete;
    PendingSupportsSessionQuery& operator=(const PendingSupportsSessionQuery&) =
        delete;

    ~PendingSupportsSessionQuery() = default;

    // Resolves underlying promise.
    void Resolve(bool supported, ExceptionState* exception_state = nullptr);

    // Rejects underlying promise with a DOMException.
    // Do not call this with |DOMExceptionCode::kSecurityError|, use
    // |RejectWithSecurityError| for that. If the exception is thrown
    // synchronously, an ExceptionState must be passed in. Otherwise it may be
    // null. Care must be taken when setting |message| - it will be accessible
    // to the application and should not contain any sensitive data.
    void RejectWithDOMException(DOMExceptionCode exception_code,
                                const String& message,
                                ExceptionState* exception_state);

    // Rejects underlying promise with a SecurityError.
    // If the exception is thrown synchronously, an ExceptionState must
    // be passed in. Otherwise it may be null. Care must be taken when setting
    // |message| - it will be accessible to the application and should not
    // contain any sensitive data.
    void RejectWithSecurityError(const String& message,
                                 ExceptionState* exception_state);

    // Rejects underlying promise with a TypeError.
    // If the exception is thrown synchronously, an ExceptionState must
    // be passed in. Otherwise it may be null. Care must be taken when setting
    // |message| - it will be accessible to the application and should not
    // contain any sensitive data.
    void RejectWithTypeError(const String& message,
                             ExceptionState* exception_state);

    device::mojom::blink::XRSessionMode mode() const;

    uint64_t TraceId() const { return trace_id_; }

    void Trace(Visitor*) const;

   private:
    Member<ScriptPromiseResolverBase> resolver_;
    const device::mojom::blink::XRSessionMode mode_;

    // Used for trace calls in order to correlate this request across processes.
    const uint64_t trace_id_;
  };

  // Helper, logs message to the console as well as DVLOGs.
  void AddConsoleMessage(mojom::blink::ConsoleMessageLevel error_level,
                         const String& message);

  const char* CheckInlineSessionRequestAllowed(
      LocalFrame* frame,
      const PendingRequestSessionQuery& query);

  RequestedXRSessionFeatureSet ParseRequestedFeatures(
      const Vector<String>& features,
      const device::mojom::blink::XRSessionMode& session_mode,
      XRSessionInit* session_init,
      mojom::ConsoleMessageLevel error_level);

  void RequestSessionInternal(device::mojom::blink::XRSessionMode session_mode,
                              PendingRequestSessionQuery* query,
                              ExceptionState* exception_state);

  void RequestImmersiveSession(PendingRequestSessionQuery* query,
                               ExceptionState* exception_state);

  void RequestInlineSession(PendingRequestSessionQuery* query,
                            ExceptionState* exception_state);

  void DoRequestSession(
      PendingRequestSessionQuery* query,
      device::mojom::blink::XRSessionOptionsPtr session_options);
  void OnRequestSessionReturned(
      PendingRequestSessionQuery*,
      device::mojom::blink::RequestSessionResultPtr result);
  void OnFullscreenConfigured(
      PendingRequestSessionQuery* query,
      device::mojom::blink::RequestSessionResultPtr result,
      bool fullscreen_succeeded);
  void FinishSessionCreation(
      PendingRequestSessionQuery*,
      device::mojom::blink::RequestSessionResultPtr result);
  void OnSupportsSessionReturned(PendingSupportsSessionQuery*,
                                 bool supports_session);
  void ResolveSessionRequest(
      PendingRequestSessionQuery*,
      device::mojom::blink::RequestSessionResultPtr result);
  void RejectSessionRequest(PendingRequestSessionQuery*);

  void EnsureDevice();

  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

  XRSession* CreateSession(
      device::mojom::blink::XRSessionMode mode,
      device::mojom::blink::XREnvironmentBlendMode blend_mode,
      device::mojom::blink::XRInteractionMode interaction_mode,
      mojo::PendingReceiver<device::mojom::blink::XRSessionClient>
          client_receiver,
      device::mojom::blink::XRSessionDeviceConfigPtr device_config,
      XRSessionFeatureSet enabled_features,
      uint64_t trace_id,
      bool sensorless_session = false);

  XRSession* CreateSensorlessInlineSession();

  enum class DisposeType {
    kContextDestroyed = 0,
    kDisconnected = 1,
  };
  void Dispose(DisposeType);

  void OnEnvironmentProviderDisconnect();

  void TryEnsureService();

  // Helper, returns true if immersive AR session creation is supported.
  // Currently, it checks whether AR is enabled in runtime features, and in web
  // settings (controlled by enterprise policy).
  bool IsImmersiveArAllowed();

  // Indicates whether use of requestDevice has already been logged.
  bool did_log_supports_immersive_ = false;

  // Indicates whether we've already logged a request for an immersive session.
  bool did_log_request_immersive_session_ = false;

  // The XR object owns outstanding pending session queries, these live until
  // the underlying promise is either resolved or rejected.
  HeapHashSet<Member<PendingSupportsSessionQuery>> outstanding_support_queries_;
  HeapHashSet<Member<PendingRequestSessionQuery>> outstanding_request_queries_;
  bool has_outstanding_immersive_request_ = false;

  Vector<EnvironmentProviderErrorCallback>
      environment_provider_error_callbacks_;

  Member<XRFrameProvider> frame_provider_;
  HeapHashSet<WeakMember<XRSession>> sessions_;
  HeapMojoRemote<device::mojom::blink::VRService> service_;
  HeapMojoAssociatedRemote<
      device::mojom::blink::XREnvironmentIntegrationProvider>
      environment_provider_;
  HeapMojoReceiver<device::mojom::blink::VRServiceClient, XRSystem> receiver_;

  // Time at which navigation started. Used as the base for relative timestamps,
  // such as for Gamepad objects.
  base::TimeTicks navigation_start_;

  FrameOrWorkerScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;

  // In DOM overlay mode, use a fullscreen event listener to detect when
  // transition to fullscreen mode completes or fails, and reject/resolve
  // the pending request session promise accordingly.
  Member<XrEnterFullscreenObserver> fullscreen_enter_observer_;
  // DOM overlay mode uses a separate temporary fullscreen event listener
  // if it needs to wait for fullscreen mode to fully exit when ending
  // the session.
  Member<XrExitFullscreenObserver> fullscreen_exit_observer_;

  bool is_context_destroyed_ = false;
  bool did_service_ever_disconnect_ = false;

  HeapMojoRemote<device::mojom::blink::WebXrInternalsRendererListener>
      webxr_internals_renderer_listener_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SYSTEM_H_
