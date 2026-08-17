// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOM_WINDOW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOM_WINDOW_H_

#include "base/memory/scoped_refptr.h"
#include "base/rand_util.h"
#include "services/network/public/mojom/cross_origin_opener_policy.mojom-blink.h"
#include "third_party/blink/public/common/messaging/message_port_channel.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/messaging/delegated_capability.mojom-blink.h"
#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/transferables.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/frame.h"
#include "third_party/blink/renderer/core/frame/window_properties.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/assertions.h"

namespace blink {

class ContextLifecycleNotifier;
class DOMWrapperWorld;
class InputDeviceCapabilitiesConstants;
class LocalDOMWindow;
class Location;
class ScriptObject;
class ScriptValue;
class SecurityOrigin;
class SerializedScriptValue;
class UserActivation;
class WindowPostMessageOptions;
class WindowProxyManager;
struct WrapperTypeInfo;

struct BlinkTransferableMessage;

// DOMWindow is an abstract class of Window interface implementations.
// We have two derived implementation classes;  LocalDOMWindow and
// RemoteDOMWindow.
//
// TODO(tkent): Rename DOMWindow to Window. The class was named as 'DOMWindow'
// because WebKit already had KJS::Window.  We have no reasons to avoid
// blink::Window now.
class CORE_EXPORT DOMWindow : public WindowProperties {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class CrossDocumentAccessPolicy { kAllowed, kDisallowed };

  ~DOMWindow() override;

  Frame* GetFrame() const {
    // A Frame is typically reused for navigations. If |frame_| is not null,
    // two conditions must always be true:
    // - |frame_->domWindow()| must point back to this DOMWindow. If it does
    //   not, it is easy to introduce a bug where script execution uses the
    //   wrong DOMWindow (which may be cross-origin).
    // - |frame_| must be attached, i.e. |frame_->page()| must not be null.
    //   If |frame_->page()| is null, this indicates a bug where the frame was
    //   detached but |frame_| was not set to null. This bug can lead to
    //   issues where executing script incorrectly schedules work on a detached
    //   frame.
    SECURITY_DCHECK(!frame_ ||
                    (frame_->DomWindow() == this && frame_->GetPage()));
    return frame_.Get();
  }

  // GarbageCollected overrides:
  void Trace(Visitor*) const override;

  virtual bool IsLocalDOMWindow() const = 0;

  // ScriptWrappable overrides:
  v8::Local<v8::Value> Wrap(ScriptState*) final;
  v8::Local<v8::Object> AssociateWithWrapper(
      v8::Isolate*,
      const WrapperTypeInfo*,
      v8::Local<v8::Object> wrapper) final;
  v8::Local<v8::Object> AssociateWithWrapper(
      v8::Isolate* isolate,
      DOMWrapperWorld* world,
      const WrapperTypeInfo* wrapper_type_info,
      v8::Local<v8::Object> wrapper);

  // EventTarget overrides:
  const AtomicString& InterfaceName() const override;
  const DOMWindow* ToDOMWindow() const override;
  bool IsWindowOrWorkerGlobalScope() const final;

  // Cross-origin DOM Level 0
  Location* location() const;
  bool closed() const;
  unsigned length() const;

  // Self-referential attributes
  DOMWindow* self() const;
  DOMWindow* window() const;
  DOMWindow* frames() const;

  DOMWindow* opener() const;
  DOMWindow* parent() const;
  DOMWindow* top() const;

  void focus(v8::Isolate*);
  void blur();
  void close(v8::Isolate*);
  void Close(LocalDOMWindow* incumbent_window);

  void postMessage(v8::Isolate*,
                   const ScriptValue& message,
                   const String& target_origin,
                   HeapVector<ScriptObject> transfer,
                   ExceptionState&);

  void postMessage(v8::Isolate*,
                   const ScriptValue& message,
                   const WindowPostMessageOptions* options,
                   ExceptionState&);

  // Indexed properties
  DOMWindow* AnonymousIndexedGetter(uint32_t index);

  // Returns the opener and collects cross-origin access metrics.
  ScriptValue openerForBindings(v8::Isolate*) const;
  void setOpenerForBindings(v8::Isolate*, ScriptValue, ExceptionState&);

  String SanitizedCrossDomainAccessErrorMessage(
      const LocalDOMWindow* accessing_window,
      CrossDocumentAccessPolicy cross_document_access) const;
  String CrossDomainAccessErrorMessage(
      const LocalDOMWindow* accessing_window,
      CrossDocumentAccessPolicy cross_document_access) const;

  // FIXME: When this DOMWindow is no longer the active DOMWindow (i.e.,
  // when its document is no longer the document that is displayed in its
  // frame), we would like to zero out |frame_| to avoid being confused
  // by the document that is currently active in |frame_|.
  // See https://bugs.webkit.org/show_bug.cgi?id=62054
  bool IsCurrentlyDisplayedInFrame() const;

  InputDeviceCapabilitiesConstants* GetInputDeviceCapabilities();

  void PostMessageForTesting(scoped_refptr<SerializedScriptValue> message,
                             const MessagePortArray&,
                             const String& target_origin,
                             LocalDOMWindow* source,
                             ExceptionState&);

  // Cross-Origin-Opener-Policy (COOP):
  // Check accesses from |accessing_frame| and every same-origin iframe toward
  // this window. A report is sent to |reporter| when this happens.
  void InstallCoopAccessMonitor(
      LocalFrame* accessing_frame,
      network::mojom::blink::CrossOriginOpenerPolicyReporterParamsPtr
          coop_reporter_params,
      bool is_in_same_virtual_coop_related_group);
  // Whenever we detect that the enforcement of a report-only COOP policy would
  // have resulted in preventing access to this window, a report is potentially
  // sent when calling this function.
  //
  // This function must be called when accessing any attributes and methods
  // marked as "CrossOrigin" in the window.idl.
  void ReportCoopAccess(const char* property_name);

  // Records metrics for access to the cross-origin WindowProxy properties.
  void RecordWindowProxyAccessMetrics(
      mojom::blink::WindowProxyAccessType access_type) const;

  // We need to check proxy access to see if it's blocked, and if so whether
  // it's for COOP-RP issues or Partitioned Popin issues.
  enum class ProxyAccessBlockedReason { kCoopRp, kPartitionedPopins };
  std::optional<ProxyAccessBlockedReason> GetProxyAccessBlockedReason(
      v8::Isolate* isolate) const;
  static String GetProxyAccessBlockedExceptionMessage(
      ProxyAccessBlockedReason reason);

 protected:
  explicit DOMWindow(Frame&);

  struct PostedMessage final : GarbageCollected<PostedMessage> {
    void Trace(Visitor* visitor) const;
    BlinkTransferableMessage ToBlinkTransferableMessage() &&;

    scoped_refptr<const SecurityOrigin> source_origin;
    scoped_refptr<const SecurityOrigin> target_origin;
    scoped_refptr<SerializedScriptValue> data;
    Vector<MessagePortChannel> channels;
    Member<LocalDOMWindow> source;
    Member<UserActivation> user_activation;
    mojom::blink::DelegatedCapability delegated_capability =
        mojom::blink::DelegatedCapability::kNone;
  };
  virtual void SchedulePostMessage(PostedMessage* message) = 0;

  void DisconnectFromFrame() { frame_ = nullptr; }

 private:
  void DoPostMessage(scoped_refptr<SerializedScriptValue> message,
                     const MessagePortArray&,
                     const WindowPostMessageOptions* options,
                     LocalDOMWindow* source,
                     ExceptionState&);

  // Removed the CoopAccessMonitor with the given |accessing_main_frame| from
  // the |coop_access_monitor| list. This is called when the COOP reporter is
  // gone or a more recent CoopAccessMonitor is being added.
  void DisconnectCoopAccessMonitor(const LocalFrameToken& accessing_main_frame);

  Member<Frame> frame_;
  // Unlike |frame_|, |window_proxy_manager_| is available even after the
  // window's frame gets detached from the DOM, until the end of the lifetime
  // of this object.
  const Member<WindowProxyManager> window_proxy_manager_;
  Member<InputDeviceCapabilitiesConstants> input_capabilities_;
  mutable Member<Location> location_;

  // Set to true when close() has been called. Needed for
  // |window.closed| determinism; having it return 'true'
  // only after the layout widget's deferred window close
  // operation has been performed, exposes (confusing)
  // implementation details to scripts.
  bool window_is_closing_;

  // Cross-Origin-Opener-Policy (COOP):
  // Check accesses made toward this window from |accessing_main_frame|. If this
  // happens a report will sent to |reporter|.
  struct CoopAccessMonitor : public GarbageCollected<CoopAccessMonitor> {
    explicit CoopAccessMonitor(ContextLifecycleNotifier* context)
        : reporter(context) {}
    void Trace(Visitor* visitor) const { visitor->Trace(reporter); }

    network::mojom::blink::CoopAccessReportType report_type;
    blink::LocalFrameToken accessing_main_frame;
    HeapMojoRemote<network::mojom::blink::CrossOriginOpenerPolicyReporter>
        reporter;
    bool endpoint_defined;
    WTF::String reported_window_url;
    bool is_in_same_virtual_coop_related_group = false;
  };
  HeapVector<Member<CoopAccessMonitor>> coop_access_monitor_;
  // Mutable: only used to downsample metrics, no change to observable state.
  mutable base::MetricsSubSampler metrics_sub_sampler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DOM_WINDOW_H_
