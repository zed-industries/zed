// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_HANDLE_ATTACHMENT_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_HANDLE_ATTACHMENT_PROVIDER_H_

#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/core/html/media/media_source_attachment.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

// Enables enforcement that a handle instance, originally retrieved from a
// MediaSource instance via MediaSource::handle(), and any new clones of it that
// may be produced during postMessage serialization of it (which can
// surprisingly cause broadcast semantics) all drop their hard reference to the
// underlying MediaSourceAttachment once one of those clones is used to start
// attaching to an HTMLMediaElement. This prevents leakage of the attached media
// element and MSE collection of objects in multiple GC heaps, as the refcounted
// attachment object has persistent references to the element and the
// mediasource. Once attached, only the media element and the mediasource should
// have references to the attachment object, and when they close the attachment
// and drop their references, there should be no other references remaining,
// enabling GC. This object serves as a provider of either a never-yet attached
// CrossThreadMediaSourceAttachment for a handle instance (and its potential
// descendants due to serialization), or a nullptr once that attachment was
// started. Locking is used to prevent read/write collision for this scenario.
class HandleAttachmentProvider final
    : public WTF::ThreadSafeRefCounted<HandleAttachmentProvider> {
 public:
  explicit HandleAttachmentProvider(
      scoped_refptr<MediaSourceAttachment> attachment);
  HandleAttachmentProvider(const HandleAttachmentProvider&) = delete;
  HandleAttachmentProvider& operator=(const HandleAttachmentProvider&) = delete;
  ~HandleAttachmentProvider();

  // Returns |attachment_| and drops our reference to it. Will return nullptr if
  // TakeAttachment() has already occurred.
  scoped_refptr<MediaSourceAttachment> TakeAttachment()
      LOCKS_EXCLUDED(attachment_lock_);

 private:
  base::Lock attachment_lock_;
  scoped_refptr<MediaSourceAttachment> attachment_ GUARDED_BY(attachment_lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_HANDLE_ATTACHMENT_PROVIDER_H_
