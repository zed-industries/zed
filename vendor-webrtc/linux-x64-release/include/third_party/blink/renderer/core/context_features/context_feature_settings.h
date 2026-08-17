// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CONTEXT_FEATURES_CONTEXT_FEATURE_SETTINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CONTEXT_FEATURES_CONTEXT_FEATURE_SETTINGS_H_

#include "base/memory/protected_memory.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExecutionContext;

// ContextFeatureSettings attaches to an ExecutionContext some basic flags
// pertaining to the enabled/disabled state of any platform API features which
// are gated behind a ContextEnabled extended attribute in IDL.
class CORE_EXPORT ContextFeatureSettings final
    : public GarbageCollected<ContextFeatureSettings>,
      public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];

  enum class CreationMode { kCreateIfNotExists, kDontCreateIfNotExists };

  explicit ContextFeatureSettings(ExecutionContext&);

  // Returns the ContextFeatureSettings for an ExecutionContext. If one does not
  // already exist for the given context, one is created.
  static ContextFeatureSettings* From(ExecutionContext*, CreationMode);

  // Protected memory values require initialization before they can be used.
  // This method is used to perform that initialization.
  static void InitializeMojoJSAllowedProtectedMemory();

  // Can be used to update the protected memory bool to indicate that MojoJS is
  // allowed to be enabled for any given context of the process. Should be set
  // in legitimate code pathways that enable MojoJS bindings for a frame.
  static void AllowMojoJSForProcess();

  // Validates that MojoJS is allowed for the process as indicated by the
  // protected memory bool. Crashes the browser via a CHECK call if the
  // protected memory bool is not set to true, and if the feature
  // "EnableMojoJSProtectedMemory" is enabled.
  static void CrashIfMojoJSNotAllowed();

  // ContextEnabled=MojoJS feature
  void EnableMojoJS(bool enable) { enable_mojo_js_ = enable; }
  bool isMojoJSEnabled() const;

  // ContextEnabled=MojoJSFileSystemAccessHelper
  void EnableMojoJSFileSystemAccessHelper(bool enable) {
    DCHECK(enable_mojo_js_);
    enable_mojo_js_file_system_access_helper_ = enable;
  }
  bool isMojoJSFileSystemAccessHelperEnabled() const {
    return enable_mojo_js_file_system_access_helper_;
  }

  // ContextEnabled=PrivateAggregationInSharedStorage
  void EnablePrivateAggregationInSharedStorage(bool enable) {
    enable_private_aggregation_in_shared_storage_ = enable;
  }
  bool isPrivateAggregationInSharedStorageEnabled() const {
    return enable_private_aggregation_in_shared_storage_;
  }

  void Trace(Visitor*) const override;

 private:
  bool enable_mojo_js_ = false;
  bool enable_mojo_js_file_system_access_helper_ = false;
  bool enable_private_aggregation_in_shared_storage_ = false;

  // Protected memory bool that indicates if MojoJS bindings are allowed to be
  // enabled for any given context of the process. Should be set to true by
  // legitimate code pathways that enable MojoJS bindings for a frame. This is
  // to reduce the ease of enabling MojoJS bindings with a data-only attack.
  static DECLARE_PROTECTED_DATA base::ProtectedMemory<bool> mojo_js_allowed_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CONTEXT_FEATURES_CONTEXT_FEATURE_SETTINGS_H_
