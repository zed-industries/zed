// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_BRIDGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_BRIDGE_H_

#include "third_party/blink/public/mojom/background_fetch/background_fetch.mojom-blink.h"
#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BackgroundFetchRegistration;

// The bridge is responsible for establishing and maintaining the Mojo
// connection to the BackgroundFetchService. It's keyed on an active Service
// Worker Registration.
class BackgroundFetchBridge final
    : public GarbageCollected<BackgroundFetchBridge>,
      public Supplement<ServiceWorkerRegistration> {
 public:
  static const char kSupplementName[];

  using GetDeveloperIdsCallback =
      base::OnceCallback<void(mojom::blink::BackgroundFetchError,
                              const Vector<String>&)>;
  using RegistrationCallback =
      base::OnceCallback<void(mojom::blink::BackgroundFetchError,
                              BackgroundFetchRegistration*)>;
  using GetIconDisplaySizeCallback = base::OnceCallback<void(const gfx::Size&)>;

  static BackgroundFetchBridge* From(ServiceWorkerRegistration* registration);

  explicit BackgroundFetchBridge(ServiceWorkerRegistration& registration);

  BackgroundFetchBridge(const BackgroundFetchBridge&) = delete;
  BackgroundFetchBridge& operator=(const BackgroundFetchBridge&) = delete;

  ~BackgroundFetchBridge();
  void Trace(Visitor* visitor) const override;

  // Creates a new Background Fetch registration identified by |developer_id|
  // for the sequence of |requests|. The |callback| will be invoked when the
  // registration has been created.
  void Fetch(const String& developer_id,
             Vector<mojom::blink::FetchAPIRequestPtr> requests,
             mojom::blink::BackgroundFetchOptionsPtr options,
             const SkBitmap& icon,
             mojom::blink::BackgroundFetchUkmDataPtr ukm_data,
             RegistrationCallback callback);

  // Gets the size of the icon to be displayed in Background Fetch UI.
  void GetIconDisplaySize(GetIconDisplaySizeCallback callback);

  // Gets the Background Fetch registration for the given |developer_id|. Will
  // invoke the |callback| with the Background Fetch registration, which may be
  // a nullptr if the |developer_id| does not exist, when the Mojo call has
  // completed.
  void GetRegistration(const String& developer_id,
                       RegistrationCallback callback);

  // Gets the sequence of ids for active Background Fetch registrations. Will
  // invoke the |callback| with the |developers_id|s when the Mojo call has
  // completed.
  void GetDeveloperIds(GetDeveloperIdsCallback callback);

 private:
  // Returns an initialized BackgroundFetchService*. A connection will be
  // established after the first call to this method.
  mojom::blink::BackgroundFetchService* GetService();

  void DidGetRegistration(
      RegistrationCallback callback,
      mojom::blink::BackgroundFetchError error,
      mojom::blink::BackgroundFetchRegistrationPtr registration_ptr);

  HeapMojoRemote<mojom::blink::BackgroundFetchService>
      background_fetch_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_BRIDGE_H_
