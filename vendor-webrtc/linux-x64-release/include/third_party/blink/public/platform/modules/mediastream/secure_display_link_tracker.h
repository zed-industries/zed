// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_SECURE_DISPLAY_LINK_TRACKER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_SECURE_DISPLAY_LINK_TRACKER_H_

#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Tracks all connected links (video sinks / tracks), and reports if they are
// all secure for video capturing.
template <typename T>
class SecureDisplayLinkTracker {
 public:
  SecureDisplayLinkTracker() = default;
  SecureDisplayLinkTracker(const SecureDisplayLinkTracker&) = delete;
  SecureDisplayLinkTracker& operator=(const SecureDisplayLinkTracker&) = delete;
  ~SecureDisplayLinkTracker() = default;

  void Add(T* link, bool is_link_secure);
  void Remove(T* link);
  void Update(T* link, bool is_link_secure);
  bool is_capturing_secure() const { return insecure_links_.empty(); }

 private:
  // Record every insecure links.
  Vector<T*> insecure_links_;
};

template <typename T>
void SecureDisplayLinkTracker<T>::Add(T* link, bool is_link_secure) {
  DCHECK(!insecure_links_.Contains(link));

  if (!is_link_secure)
    insecure_links_.push_back(link);
}

template <typename T>
void SecureDisplayLinkTracker<T>::Remove(T* link) {
  auto it = insecure_links_.Find(link);
  if (it != kNotFound)
    insecure_links_.EraseAt(it);
}

template <typename T>
void SecureDisplayLinkTracker<T>::Update(T* link, bool is_link_secure) {
  auto it = insecure_links_.Find(link);
  if (it != kNotFound) {
    if (is_link_secure)
      insecure_links_.EraseAt(it);
    return;
  }
  Add(link, is_link_secure);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_SECURE_DISPLAY_LINK_TRACKER_H_
