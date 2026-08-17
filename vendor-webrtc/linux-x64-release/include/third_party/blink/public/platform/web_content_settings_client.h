// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_SETTINGS_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_SETTINGS_CLIENT_H_

#include <memory>
#include <utility>

#include "base/functional/callback.h"

namespace blink {

class WebURL;

// This class provides the content settings information which tells
// whether each feature is allowed or not.
class WebContentSettingsClient {
 public:
  // Only used if this is a WebContentSettingsClient on a worker thread. Clones
  // this WebContentSettingsClient so it can be used by another worker thread.
  virtual std::unique_ptr<WebContentSettingsClient> Clone() { return nullptr; }

  enum class StorageType {
    kCacheStorage,
    kIndexedDB,
    kFileSystem,
    kWebLocks,
    kLocalStorage,
    kSessionStorage
  };

  // Controls whether access to the given StorageType is allowed for this frame.
  // Runs asynchronously.
  virtual void AllowStorageAccess(StorageType storage_type,
                                  base::OnceCallback<void(bool)> callback) {
    std::move(callback).Run(true);
  }

  // Controls whether access to the given StorageType is allowed for this frame.
  // Blocks until done.
  virtual bool AllowStorageAccessSync(StorageType storage_type) { return true; }

  // Controls whether insecure scripts are allowed to execute for this frame.
  virtual bool AllowRunningInsecureContent(bool enabled_per_settings,
                                           const WebURL&) {
    return enabled_per_settings;
  }

  // Controls whether access to read the clipboard is allowed for this frame.
  virtual bool AllowReadFromClipboard() { return false; }

  // Controls whether access to write the clipboard is allowed for this frame.
  virtual bool AllowWriteToClipboard() { return false; }

  // Controls whether to enable MutationEvents for this frame.
  // The common use case of this method is actually to selectively disable
  // MutationEvents, but it's been named for consistency with the rest of the
  // interface.
  virtual bool AllowMutationEvents(bool default_value) { return default_value; }

  // Reports that passive mixed content was found at the provided URL.
  virtual void PassiveInsecureContentFound(const WebURL&) {}

  // Notifies the client that the frame would have executed script if script
  // were enabled.
  virtual void DidNotAllowScript() {}

  // Notifies the client that the frame would have loaded an image if image were
  // enabled.
  virtual void DidNotAllowImage() {}

  // Controls whether mixed content autoupgrades should be allowed in this
  // frame.
  virtual bool ShouldAutoupgradeMixedContent() { return true; }

  virtual ~WebContentSettingsClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_SETTINGS_CLIENT_H_
