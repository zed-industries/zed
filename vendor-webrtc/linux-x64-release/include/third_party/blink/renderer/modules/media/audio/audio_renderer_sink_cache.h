// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_SINK_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_SINK_CACHE_H_

#include <memory>
#include <string>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "media/audio/audio_sink_parameters.h"
#include "media/base/output_device_info.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace media {
class AudioRendererSink;
}

namespace blink {

class LocalDOMWindow;

// Creates temporary audio sinks in order to acquire OutputDeviceInfo from them.
// These sinks live for a total time of |delete_timeout| to allow for multiple
// queries without reconstructing the temporary sink, and are then deleted.
// Must live on the main render thread. Thread safe.
class MODULES_EXPORT AudioRendererSinkCache {
 public:
  class WindowObserver;

  // Callback to be used for AudioRendererSink creation
  using CreateSinkCallback =
      base::RepeatingCallback<scoped_refptr<media::AudioRendererSink>(
          const LocalFrameToken& frame_token,
          const std::string& device_id)>;

  // If called, the cache will drop sinks belonging to the specified window on
  // navigation.
  static void InstallWindowObserver(LocalDOMWindow&);

  // |cleanup_task_runner| will be used to delete sinks.
  // AudioRendererSinkCache must outlive any tasks posted to it. Since
  // the sink cache is normally a process-wide singleton, this isn't a problem.
  AudioRendererSinkCache(
      scoped_refptr<base::SequencedTaskRunner> cleanup_task_runner,
      CreateSinkCallback create_sink_callback,
      base::TimeDelta delete_timeout);

  AudioRendererSinkCache(const AudioRendererSinkCache&) = delete;
  AudioRendererSinkCache& operator=(const AudioRendererSinkCache&) = delete;

  ~AudioRendererSinkCache();

  media::OutputDeviceInfo GetSinkInfo(const LocalFrameToken& source_frame_token,
                                      const std::string& device_id);

 private:
  friend class AudioRendererSinkCacheTest;
  friend class CacheEntryFinder;
  friend class AudioRendererSinkCache::WindowObserver;

  struct CacheEntry;
  using CacheContainer = Vector<CacheEntry>;

  // Schedules a sink for deletion. Deletion will be performed on the same
  // thread the cache is created on.
  void DeleteLater(scoped_refptr<media::AudioRendererSink> sink);

  // Deletes a sink from the cache.
  void DeleteSink(const media::AudioRendererSink* sink_ptr);

  CacheContainer::iterator FindCacheEntry_Locked(
      const LocalFrameToken& source_frame_token,
      const std::string& device_id);

  void MaybeCacheSink(const LocalFrameToken& source_frame_token,
                      const std::string& device_id,
                      scoped_refptr<media::AudioRendererSink> sink);

  void DropSinksForFrame(const LocalFrameToken& source_frame_token);

  // To avoid publishing CacheEntry structure in the header.
  wtf_size_t GetCacheSizeForTesting();

  // Global instance, set in constructor and unset in destructor.
  static AudioRendererSinkCache* instance_;

  // Renderer main task runner.
  const scoped_refptr<base::SequencedTaskRunner> cleanup_task_runner_;

  // Callback used for sink creation.
  const CreateSinkCallback create_sink_cb_;

  // Cached sink deletion timeout.
  const base::TimeDelta delete_timeout_;

  // Cached sinks, protected by lock.
  base::Lock cache_lock_;
  CacheContainer cache_ GUARDED_BY(cache_lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_SINK_CACHE_H_
