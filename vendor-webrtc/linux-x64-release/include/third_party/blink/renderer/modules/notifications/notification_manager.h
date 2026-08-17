// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_MANAGER_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/notifications/notification_service.mojom-blink.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_notification_permission_callback.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
class Notification;
class ScriptState;
class V8NotificationPermission;

// The notification manager, unique to the execution context, is responsible for
// connecting and communicating with the Mojo notification service.
//
// TODO(peter): Make the NotificationManager responsible for resource loading.
class NotificationManager final : public GarbageCollected<NotificationManager>,
                                  public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];

  static NotificationManager* From(ExecutionContext* context);

  explicit NotificationManager(ExecutionContext& context);

  NotificationManager(const NotificationManager&) = delete;
  NotificationManager& operator=(const NotificationManager&) = delete;

  ~NotificationManager();

  // Returns the notification permission status of the current origin. This
  // method is synchronous to support the Notification.permission getter.
  mojom::blink::PermissionStatus GetPermissionStatus();

  // Async version of the above method, used to gather some metrics where we
  // don't need to pay the cost of a sync IPC.
  void GetPermissionStatusAsync(
      base::OnceCallback<void(mojom::blink::PermissionStatus)> callback);

  ScriptPromise<V8NotificationPermission> RequestPermission(
      ScriptState* script_state,
      V8NotificationPermissionCallback* deprecated_callback);

  // Shows a notification that is not tied to any service worker.
  //
  // Compares |token| against the token of all currently displayed
  // notifications and if there's a match, replaces the older notification;
  // else displays a new notification.
  void DisplayNonPersistentNotification(
      const String& token,
      mojom::blink::NotificationDataPtr notification_data,
      mojom::blink::NotificationResourcesPtr notification_resources,
      mojo::PendingRemote<mojom::blink::NonPersistentNotificationListener>
          event_listener);

  // Closes the notification that was most recently displayed with this token.
  void CloseNonPersistentNotification(const String& token);

  // Shows a notification from a service worker.
  void DisplayPersistentNotification(
      int64_t service_worker_registration_id,
      mojom::blink::NotificationDataPtr notification_data,
      mojom::blink::NotificationResourcesPtr notification_resources,
      ScriptPromiseResolver<IDLUndefined>* resolver);

  // Closes a persistent notification identified by its notification id.
  void ClosePersistentNotification(const WebString& notification_id);

  // Asynchronously gets the persistent notifications belonging to the Service
  // Worker Registration. If |filter_tag| is not an empty string, only the
  // notification with the given tag will be considered. If |include_triggered|
  // is true, this will include scheduled notifications.
  void GetNotifications(
      int64_t service_worker_registration_id,
      const WebString& filter_tag,
      bool include_triggered,
      ScriptPromiseResolver<IDLSequence<Notification>>* resolver);

  void Trace(Visitor* visitor) const override;

 private:
  void DidDisplayPersistentNotification(
      ScriptPromiseResolver<IDLUndefined>* resolver,
      mojom::blink::PersistentNotificationError error);

  void DidGetNotifications(
      ScriptPromiseResolver<IDLSequence<Notification>>* resolver,
      const Vector<String>& notification_ids,
      Vector<mojom::blink::NotificationDataPtr> notification_datas);

  // Returns an initialized NotificationService remote. A connection will be
  // established the first time this method is called.
  mojom::blink::NotificationService* GetNotificationService();

  void OnPermissionRequestComplete(
      ScriptPromiseResolver<V8NotificationPermission>* resolver,
      V8NotificationPermissionCallback* deprecated_callback,
      mojom::blink::PermissionStatus status);

  void OnNotificationServiceConnectionError();
  void OnPermissionServiceConnectionError();

  HeapMojoRemote<mojom::blink::NotificationService> notification_service_;
  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_MANAGER_H_
