// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_V8_FEATURES_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_V8_FEATURES_H_

#include "base/process/process.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_common.h"
#include "v8/include/v8-forward.h"

namespace blink {

// WebV8Features is used in conjunction with IDL interface features which
// specify a [ContextEnabled] extended attribute. Such features may be enabled
// for arbitrary main-world V8 contexts by using these methods during
// WebLocalFrameClient::DidCreateScriptContext. Enabling a given feature causes
// its binding(s) to be installed on the appropriate global object(s) during
// context initialization.
//
// See src/third_party/blink/renderer/bindings/IDLExtendedAttributes.md for more
// information.
class BLINK_EXPORT WebV8Features {
 public:
  static void EnableMojoJS(v8::Local<v8::Context>, bool);

  static void EnableMojoJSAndUseBroker(
      v8::Local<v8::Context> context,
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase>
          broker_remote);

  static void EnableMojoJSFileSystemAccessHelper(v8::Local<v8::Context>, bool);

  // Protected memory values require initialization before they can be used.
  // This method is used to perform that initialization of the static protected
  // memory bool is used to track if MojoJS has been properly enabled for a
  // render frame in the current process.
  static void InitializeMojoJSAllowedProtectedMemory();

  // A static protected memory bool is used to track if MojoJS has been properly
  // enabled for a render frame in the current process. This method is used to
  // update that bool, indicating that MojoJS is allowed to be enabled for any
  // render frame in the process.
  static void AllowMojoJSForProcess();

  // Method use to validate the value of isMojoJSEnabled() for the context in
  // tests.
  static bool IsMojoJSEnabledForTesting(v8::Local<v8::Context>);

  // Testing method that enables mojo JS on the ContextFeatureSettings while
  // bypassing the protected memory bool check. This is used to validate later
  // stage protected memory check code paths.
  static void EnableMojoJSWithoutSecurityChecksForTesting(
      v8::Local<v8::Context>);

  // Send isolate priority change notification to worker thread isolates.
  static void SetIsolatePriority(base::Process::Priority priority);

 private:
  WebV8Features() = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_V8_FEATURES_H_
