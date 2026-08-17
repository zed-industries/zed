/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_H_

#include "third_party/blink/public/mojom/notifications/notification_service.mojom-blink.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/permissions/permission_status.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_notification_permission.h"
#include "third_party/blink/renderer/core/dom/dom_time_stamp.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/vibration/vibration_controller.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class ExecutionContext;
class NotificationOptions;
class NotificationResourcesLoader;
class ScriptState;
class TimestampTrigger;
class V8NotificationPermissionCallback;

class MODULES_EXPORT Notification final
    : public EventTarget,
      public ActiveScriptWrappable<Notification>,
      public ExecutionContextLifecycleObserver,
      public mojom::blink::NonPersistentNotificationListener {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Used for JavaScript instantiations of non-persistent notifications. Will
  // automatically schedule for the notification to be displayed to the user
  // when the developer-provided data is valid.
  static Notification* Create(ExecutionContext* context,
                              const String& title,
                              const NotificationOptions* options,
                              ExceptionState& state);

  // Used for embedder-created persistent notifications. Initializes the state
  // of the notification as either Showing or Closed based on |showing|.
  static Notification* Create(ExecutionContext* context,
                              const String& notification_id,
                              mojom::blink::NotificationDataPtr data,
                              bool showing);

  // The type of notification this instance represents. Non-persistent
  // notifications will have events delivered to their instance, whereas
  // persistent notification will be using a Service Worker.
  enum class Type { kNonPersistent, kPersistent };

  Notification(ExecutionContext* context,
               Type type,
               mojom::blink::NotificationDataPtr data);
  ~Notification() override;

  void close();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(click, kClick)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(show, kShow)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(close, kClose)

  // NonPersistentNotificationListener interface.
  void OnShow() override;
  void OnClick(OnClickCallback completed_closure) override;
  void OnClose(OnCloseCallback completed_closure) override;

  String title() const;
  String dir() const;
  String lang() const;
  String body() const;
  String tag() const;
  String image() const;
  String icon() const;
  String badge() const;
  VibrationController::VibrationPattern vibrate() const;
  DOMTimeStamp timestamp() const;
  bool renotify() const;
  bool silent() const;
  bool requireInteraction() const;
  ScriptValue data(ScriptState* script_state);
  v8::LocalVector<v8::Value> actions(ScriptState* script_state) const;
  TimestampTrigger* showTrigger() const { return show_trigger_.Get(); }
  String scenario() const;

  static V8NotificationPermission::Enum PermissionToV8Enum(
      mojom::blink::PermissionStatus permission);
  static V8NotificationPermission permission(ExecutionContext* context);
  static ScriptPromise<V8NotificationPermission> requestPermission(
      ScriptState* script_state,
      V8NotificationPermissionCallback* deprecated_callback = nullptr);

  static uint32_t maxActions();

  // EventTarget interface.
  ExecutionContext* GetExecutionContext() const final {
    return ExecutionContextLifecycleObserver::GetExecutionContext();
  }
  const AtomicString& InterfaceName() const override;

  // ExecutionContextLifecycleObserver interface.
  void ContextDestroyed() override;

  // ScriptWrappable interface.
  bool HasPendingActivity() const final;

  void Trace(Visitor* visitor) const override;

 protected:
  // EventTarget interface.
  DispatchEventResult DispatchEventInternal(Event& event) final;

 private:
  // The current phase of the notification in its lifecycle.
  enum class State { kLoading, kShowing, kClosing, kClosed };

  // Sets the state of the notification in its lifecycle.
  void SetState(State state) { state_ = state; }

  // Sets the notification ID to |notificationId|. This should be done once
  // the notification has shown for non-persistent notifications, and at
  // object initialisation time for persistent notifications.
  void SetNotificationId(const String& notification_id) {
    notification_id_ = notification_id;
  }

  // Sets the token which will be used to both show and close the notification.
  // Should be equal to tag_ if a tag is present, else should be unique.
  void SetToken(const String& token) { token_ = token; }

  // Schedules an asynchronous call to |prepareShow|, allowing the constructor
  // to return so that events can be fired on the notification object.
  void SchedulePrepareShow();

  // Verifies that permission has been granted, then asynchronously starts
  // loading the resources associated with this notification.
  void PrepareShow(TimerBase* timer);

  // Shows the notification through the embedder using the loaded resources.
  void DidLoadResources(NotificationResourcesLoader* loader);

  void DispatchErrorEvent();

  Type type_;
  State state_;

  mojom::blink::NotificationDataPtr data_;

  Member<TimestampTrigger> show_trigger_;

  String notification_id_;

  String token_;

  HeapTaskRunnerTimer<Notification> prepare_show_timer_;

  Member<NotificationResourcesLoader> loader_;

  HeapMojoReceiver<mojom::blink::NonPersistentNotificationListener,
                   Notification>
      listener_receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_H_
