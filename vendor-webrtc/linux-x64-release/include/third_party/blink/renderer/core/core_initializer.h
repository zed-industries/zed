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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CORE_INITIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CORE_INITIALIZER_H_

#include <memory>

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/common/dom_storage/session_storage_namespace_id.h"
#include "third_party/blink/public/mojom/dom_storage/storage_area.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/filesystem/file_system.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace display {
struct ScreenInfos;
}

namespace mojo {
class BinderMap;
}

namespace blink {

class DevToolsSession;
class Document;
class ExecutionContext;
class HTMLMediaElement;
class InspectedFrames;
class InspectorDOMAgent;
class LocalFrame;
class MediaControls;
class Page;
class PictureInPictureController;
class RemotePlaybackClient;
class ServiceWorkerGlobalScope;
class Settings;
class ShadowRoot;
class WebLocalFrameClient;
class WebMediaPlayer;
class WebMediaPlayerClient;
class WebMediaPlayerSource;

class CORE_EXPORT CoreInitializer {
  USING_FAST_MALLOC(CoreInitializer);

 public:
  // Initialize must be called before GetInstance.
  static CoreInitializer& GetInstance() {
    DCHECK(instance_);
    return *instance_;
  }

  CoreInitializer(const CoreInitializer&) = delete;
  CoreInitializer& operator=(const CoreInitializer&) = delete;
  virtual ~CoreInitializer() = default;

  // Should be called by clients before trying to create Frames.
  virtual void Initialize();

  // Called on startup to register Mojo interfaces that for control messages,
  // e.g. messages that are not routed to a specific frame.
  virtual void RegisterInterfaces(mojo::BinderMap&) = 0;
  // Methods defined in CoreInitializer and implemented by ModulesInitializer to
  // bypass the inverted dependency from core/ to modules/.
  // Mojo Interfaces registered with LocalFrame
  virtual void InitLocalFrame(LocalFrame&) const = 0;
  // Mojo Interfaces registered with ServiceWorkerGlobalScope.
  virtual void InitServiceWorkerGlobalScope(
      ServiceWorkerGlobalScope&) const = 0;
  // Supplements installed on a frame using ChromeClient
  virtual void InstallSupplements(LocalFrame&) const = 0;
  virtual MediaControls* CreateMediaControls(HTMLMediaElement&,
                                             ShadowRoot&) const = 0;
  virtual PictureInPictureController* CreatePictureInPictureController(
      Document&) const = 0;
  // Session Initializers for Inspector Agents in modules/
  // These methods typically create agents and append them to a session.
  // TODO(nverne): remove this and restore to WebDevToolsAgentImpl once that
  // class is a controller/ crbug:731490
  virtual void InitInspectorAgentSession(DevToolsSession*,
                                         InspectorDOMAgent*,
                                         InspectedFrames*,
                                         Page*) const = 0;

  virtual void OnClearWindowObjectInMainWorld(Document&,
                                              const Settings&) const = 0;

  virtual std::unique_ptr<WebMediaPlayer> CreateWebMediaPlayer(
      WebLocalFrameClient*,
      HTMLMediaElement&,
      const WebMediaPlayerSource&,
      WebMediaPlayerClient*) const = 0;

  virtual RemotePlaybackClient* CreateRemotePlaybackClient(
      HTMLMediaElement&) const = 0;

  virtual void ProvideModulesToPage(Page&,
                                    const SessionStorageNamespaceId&) const = 0;
  virtual void ForceNextWebGLContextCreationToFail() const = 0;

  virtual void CollectAllGarbageForAnimationAndPaintWorkletForTesting()
      const = 0;

  virtual void CloneSessionStorage(
      Page* clone_from_page,
      const SessionStorageNamespaceId& clone_to_namespace) = 0;

  // Evicts the cached data of Session Storage. Called after dispatching a
  // document unload or freeze event to avoid reusing old data in the cache in
  // case the same renderer process is reused after the session storage has been
  // modified by another renderer process. (Eg: Back navigation from a
  // prerendered page.)
  virtual void EvictSessionStorageCachedData(Page*) = 0;

  virtual void DidChangeManifest(LocalFrame&) = 0;
  virtual void NotifyOrientationChanged(LocalFrame&) = 0;
  // Called with an updated set of ScreenInfos for a local root frame
  // during a visual property update.
  virtual void DidUpdateScreens(LocalFrame& frame,
                                const display::ScreenInfos&) = 0;

  virtual void SetLocalStorageArea(
      LocalFrame& frame,
      mojo::PendingRemote<mojom::blink::StorageArea> local_storage_area) = 0;
  virtual void SetSessionStorageArea(
      LocalFrame& frame,
      mojo::PendingRemote<mojom::blink::StorageArea> session_storage_area) = 0;

  virtual mojom::blink::FileSystemManager& GetFileSystemManager(
      ExecutionContext* context) = 0;

 protected:
  // CoreInitializer is only instantiated by subclass ModulesInitializer.
  CoreInitializer() = default;

 private:
  static CoreInitializer* instance_;
  void RegisterEventFactory();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CORE_INITIALIZER_H_
