// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_MOCK_CLIPBOARD_PERMISSION_SERVICE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_MOCK_CLIPBOARD_PERMISSION_SERVICE_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"

namespace blink {
using mojom::blink::PermissionDescriptorPtr;

class MockClipboardPermissionService final
    : public mojom::blink::PermissionService {
 public:
  MockClipboardPermissionService();

  ~MockClipboardPermissionService() override;

  void BindRequest(mojo::ScopedMessagePipeHandle handle);

  // mojom::blink::PermissionService implementation
  MOCK_METHOD(void,
              HasPermission,
              (mojom::blink::PermissionDescriptorPtr permission,
               HasPermissionCallback),
              (override));

  MOCK_METHOD(
      void,
      RegisterPageEmbeddedPermissionControl,
      (Vector<mojom::blink::PermissionDescriptorPtr> permissions,
       mojo::PendingRemote<mojom::blink::EmbeddedPermissionControlClient>
           client),
      (override));

  MOCK_METHOD(void,
              RequestPageEmbeddedPermission,
              (mojom::blink::EmbeddedPermissionRequestDescriptorPtr permissions,
               RequestPageEmbeddedPermissionCallback),
              (override));

  MOCK_METHOD(void,
              RequestPermission,
              (mojom::blink::PermissionDescriptorPtr permission,
               bool user_gesture,
               RequestPermissionCallback),
              (override));

  MOCK_METHOD(void,
              RequestPermissions,
              (Vector<mojom::blink::PermissionDescriptorPtr> permissions,
               bool user_gesture,
               RequestPermissionsCallback),
              (override));

  MOCK_METHOD(void,
              RevokePermission,
              (mojom::blink::PermissionDescriptorPtr permission,
               RevokePermissionCallback),
              (override));

  MOCK_METHOD(void,
              AddPermissionObserver,
              (mojom::blink::PermissionDescriptorPtr permission,
               mojom::blink::PermissionStatus last_known_status,
               mojo::PendingRemote<mojom::blink::PermissionObserver>),
              (override));

  MOCK_METHOD(void,
              AddPageEmbeddedPermissionObserver,
              (mojom::blink::PermissionDescriptorPtr permission,
               mojom::blink::PermissionStatus last_known_status,
               mojo::PendingRemote<mojom::blink::PermissionObserver>),
              (override));

  MOCK_METHOD(void,
              NotifyEventListener,
              (mojom::blink::PermissionDescriptorPtr permission,
               const String& event_type,
               bool is_added),
              (override));

 private:
  void OnConnectionError();

  mojo::Receiver<mojom::blink::PermissionService> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_MOCK_CLIPBOARD_PERMISSION_SERVICE_H_
