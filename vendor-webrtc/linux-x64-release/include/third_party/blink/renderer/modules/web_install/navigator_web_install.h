// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEB_INSTALL_NAVIGATOR_WEB_INSTALL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEB_INSTALL_NAVIGATOR_WEB_INSTALL_H_

#include "third_party/blink/public/mojom/web_install/web_install.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_web_install_result.h"
#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ScriptState;

// It is owned by Navigator, and an instance is created lazily by calling
// NavigatorWebInstall::From() via install().
class MODULES_EXPORT NavigatorWebInstall final
    : public GarbageCollected<NavigatorWebInstall>,
      public Supplement<Navigator> {
 public:
  static const char kSupplementName[];

  explicit NavigatorWebInstall(Navigator& navigator);
  ~NavigatorWebInstall() = default;

  static ScriptPromise<WebInstallResult> install(
      ScriptState* script_state,
      Navigator& navigator,
      ExceptionState& exception_state);

  static ScriptPromise<WebInstallResult> install(
      ScriptState* script_state,
      Navigator& navigator,
      const String& install_url,
      ExceptionState& exception_state);

  static ScriptPromise<WebInstallResult> install(
      ScriptState* script_state,
      Navigator& navigator,
      const String& install_url,
      const String& manifest_id,
      ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 private:
  static NavigatorWebInstall& From(Navigator&);

  // `install_url` and/or `manifest_id` may be empty depending on which of the 3
  // versions of `install()` was called.
  ScriptPromise<WebInstallResult> InstallImpl(
      ScriptState* script_state,
      const std::optional<String>& install_url,
      const std::optional<String>& manifest_id,
      ExceptionState& exception_state);
  HeapMojoRemote<mojom::blink::WebInstallService>& GetService();
  void OnConnectionError();
  bool CheckPreconditionsMaybeThrow(ScriptState*, ExceptionState&);
  bool IsInstallUrlValid(const String& install_url);
  KURL ValidateAndResolveManifestId(const String& manifest_id);

  HeapMojoRemote<mojom::blink::WebInstallService> service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEB_INSTALL_NAVIGATOR_WEB_INSTALL_H_
