// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_ATTACHMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_ATTACHMENT_H_

#include <memory>
#include "third_party/blink/public/platform/web_time_range.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fileapi/url_registry.h"
#include "third_party/blink/renderer/core/html/media/media_source_tracer.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

class HTMLMediaElement;
class MediaSourceRegistry;
class TrackBase;
class WebMediaSource;

// Interface for concrete non-oilpan types to coordinate potentially
// cross-context registration, deregistration, lookup of a MediaSource via the
// MediaSourceRegistry, and for proxying operations on an attached MediaSource.
// Upon successful lookup, enables the extension of an HTMLMediaElement by the
// MSE API, aka attachment. This type is not managed by oilpan due to the
// potentially varying context lifetimes. Concrete implementations of this
// handle same-thread (main thread) attachments distinctly from cross-context
// (MSE-in-Worker, HTMLMediaElement in main thread) attachments due to the
// increased complexity for handling the latter. Concrete implementations of
// this interface are reference counted to ensure they are available potentially
// cross-thread and from the registry.
class CORE_EXPORT MediaSourceAttachment
    : public URLRegistrable,
      public WTF::ThreadSafeRefCounted<MediaSourceAttachment> {
 public:
  // Intended to be set by the MediaSourceRegistry during its singleton
  // initialization on the main thread. Caches the pointer in |registry_|.
  static void SetRegistry(MediaSourceRegistry*);

  // Services lookup calls, expected from HTMLMediaElement during its load
  // algorithm. If |url| is not known by MediaSourceRegistry, returns nullptr.
  // Otherwise, returns the MediaSourceAttachment associated with |url|.
  static scoped_refptr<MediaSourceAttachment> LookupMediaSource(
      const String& url);

  MediaSourceAttachment(const MediaSourceAttachment&) = delete;
  MediaSourceAttachment& operator=(const MediaSourceAttachment&) = delete;

  // This is called on the main thread when the URLRegistry unregisters the
  // objectURL for this attachment. Concrete implementation overrides should use
  // this signal to release their strong reference to the MediaSource such that
  // GC might collect it if there is no active attachment represented by other
  // strong references.
  virtual void Unregister() = 0;

  // URLRegistrable
  URLRegistry& Registry() const override { return *registry_; }

  // These two methods are called in sequence when an HTMLMediaElement is
  // attempting to attach to the MediaSource object using this attachment
  // instance. The WebMediaSource is not available to the element initially, so
  // between the two calls, the attachment could be considered partially setup.
  // If attachment start fails (for example, if the underlying MediaSource is
  // already attached, or if this attachment has already been unregistered from
  // the MediaSourceRegistry), StartAttachingToMediaElement() sets |*success|
  // false and returns nullptr. |success| must not be nullptr.
  // Otherwise, the underlying MediaSource must be in 'closed' state, and
  // indicates success by setting |*success| true and optionally returning a
  // tracer object useful in at least same-thread attachments for enabling
  // automatic idle unreferenced same-thread attachment object garbage
  // collection. Note that that tracer could be nullptr even if attachment start
  // was successful, for instance in a cross-thread attachment where there is no
  // tracer.
  // CompleteAttachingToMediaElement() provides the attached MediaSource with
  // the underlying WebMediaSource, enabling parsing of media provided by the
  // application for playback, for example.
  // Once attached, the MediaSource and the HTMLMediaElement use each other via
  // this attachment to accomplish the extended API.
  // The MediaSourceTracer argument to calls in this interface enables at least
  // the same-thread attachment to dynamically retrieve the Oilpan-managed
  // objects without itself being managed by oilpan. Alternatives like requiring
  // the (non-GC'ed) attachment to remember the tracer as a Persistent would
  // break the ability for automatic collection of idle unreferenced same-thread
  // HTMLME+MSE object collections. The tracer argument must be the same as that
  // returned by the most recent call to the attachment's
  // StartAttachingToMediaElement. We cannot have the tracer as a Member, and
  // using Persistent to hold it instead would break the ability for automatic
  // collection of idle unreferenced same-thread attached HTMLMediaElement +
  // MediaSource object groups.
  virtual MediaSourceTracer* StartAttachingToMediaElement(HTMLMediaElement*,
                                                          bool* success) = 0;
  virtual void CompleteAttachingToMediaElement(
      MediaSourceTracer* tracer,
      std::unique_ptr<WebMediaSource>) = 0;

  virtual void Close(MediaSourceTracer* tracer) = 0;

  // 'Internal' in these methods doesn't mean private, it means that they are
  // internal to chromium and are not exposed to JavaScript.
  virtual WebTimeRanges BufferedInternal(MediaSourceTracer* tracer) const = 0;
  virtual WebTimeRanges SeekableInternal(MediaSourceTracer* tracer) const = 0;

  virtual void OnTrackChanged(MediaSourceTracer* tracer, TrackBase*) = 0;

  // Provide state updates to the MediaSource that are necessary for its
  // operation. These are pushed rather than pulled to reduce complexity and
  // latency, especially when the MediaSource is in a Worker context.
  // OnElementTimeUpdate() gives the MediaSource a notion of the recent media
  // element currentTime so that it can more effectively prevent evicting
  // buffered media near to playback and/or seek target time in its heuristic.
  // Alternatives such as pumping this via the media pipeline are insufficient,
  // as the media pipeline may not be aware of overrides to the playback start
  // position.
  virtual void OnElementTimeUpdate(double time) = 0;

  // Needed as a precondition in the Prepare Append algorithm, OnElementError()
  // lets the MediaSource know if the attached media element has transitioned to
  // having an error.
  virtual void OnElementError() = 0;

  // Needed in cross-thread attachments to prevent the attachment from UAF of
  // the media element when the media element's context might be destroyed
  // before a worker-context MSE's context has been destroyed. In such case,
  // neither the media element, nor the underlying main-thread-owned MSE demuxer
  // should be used further.
  virtual void OnElementContextDestroyed() = 0;

 protected:
  friend class WTF::ThreadSafeRefCounted<MediaSourceAttachment>;
  MediaSourceAttachment();
  ~MediaSourceAttachment() override;

 private:
  static URLRegistry* registry_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_ATTACHMENT_H_
