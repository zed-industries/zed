// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MODULES_INITIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MODULES_INITIALIZER_H_

#include "third_party/blink/renderer/core/core_initializer.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class MODULES_EXPORT ModulesInitializer : public CoreInitializer {
 public:
  void Initialize() override;

 protected:
  void InitLocalFrame(LocalFrame&) const override;
  void OnClearWindowObjectInMainWorld(Document&,
                                      const Settings&) const override;

 private:
  void InstallSupplements(LocalFrame&) const override;
  MediaControls* CreateMediaControls(HTMLMediaElement&,
                                     ShadowRoot&) const override;
  PictureInPictureController* CreatePictureInPictureController(
      Document&) const override;
  void InitInspectorAgentSession(DevToolsSession*,
                                 InspectorDOMAgent*,
                                 InspectedFrames*,
                                 Page*) const override;
  std::unique_ptr<WebMediaPlayer> CreateWebMediaPlayer(
      WebLocalFrameClient*,
      HTMLMediaElement&,
      const WebMediaPlayerSource&,
      WebMediaPlayerClient*) const override;
  RemotePlaybackClient* CreateRemotePlaybackClient(
      HTMLMediaElement&) const override;

  void ProvideModulesToPage(Page&,
                            const SessionStorageNamespaceId&) const override;
  void ForceNextWebGLContextCreationToFail() const override;

  void CollectAllGarbageForAnimationAndPaintWorkletForTesting() const override;

  void CloneSessionStorage(
      Page* clone_from_page,
      const SessionStorageNamespaceId& clone_to_namespace) override;
  void EvictSessionStorageCachedData(Page*) override;

  void DidChangeManifest(LocalFrame&) override;
  void NotifyOrientationChanged(LocalFrame&) override;
  void DidUpdateScreens(LocalFrame&, const display::ScreenInfos&) override;
  void SetLocalStorageArea(LocalFrame& frame,
                           mojo::PendingRemote<mojom::blink::StorageArea>
                               local_storage_area) override;
  void SetSessionStorageArea(LocalFrame& frame,
                             mojo::PendingRemote<mojom::blink::StorageArea>
                                 session_storage_area) override;
  mojom::blink::FileSystemManager& GetFileSystemManager(
      ExecutionContext* context) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MODULES_INITIALIZER_H_
