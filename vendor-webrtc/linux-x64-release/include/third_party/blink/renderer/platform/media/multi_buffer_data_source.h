// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_DATA_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_DATA_SOURCE_H_

#include <stdint.h>

#include <memory>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "media/base/cross_origin_data_source.h"
#include "media/base/data_source.h"
#include "media/base/ranges.h"
#include "media/base/tuneable.h"
#include "third_party/blink/renderer/platform/media/url_index.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "url/gurl.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace media {
class MediaLog;
}

namespace blink {
class BufferedDataSourceHost;
class MultiBufferReader;

// A data source capable of loading URLs and buffering the data using an
// in-memory sliding window.
//
// MultiBufferDataSource must be created and destroyed on the thread associated
// with the |task_runner| passed in the constructor.
class PLATFORM_EXPORT MultiBufferDataSource
    : public media::CrossOriginDataSource {
 public:
  using DownloadingCB = base::RepeatingCallback<void(bool)>;
  using RedirectCB = base::RepeatingCallback<void()>;

  // |url| and |cors_mode| are passed to the object. Buffered byte range changes
  // will be reported to |host|. |downloading_cb| will be called whenever the
  // downloading/paused state of the source changes.
  MultiBufferDataSource(
      const scoped_refptr<base::SingleThreadTaskRunner>& task_runner,
      scoped_refptr<UrlData> url_data,
      media::MediaLog* media_log,
      BufferedDataSourceHost* host,
      DownloadingCB downloading_cb);
  MultiBufferDataSource(const MultiBufferDataSource&) = delete;
  MultiBufferDataSource& operator=(const MultiBufferDataSource&) = delete;
  ~MultiBufferDataSource() override;

  // CrossOriginDataSource overrides.
  bool IsCorsCrossOrigin() const override;
  bool HasAccessControl() const override;
  const std::string& GetMimeType() const override {
    return url_data_->mime_type();
  }

  // Method called on the render thread.
  using InitializeCB = base::OnceCallback<void(bool)>;
  void Initialize(InitializeCB init_cb) override;

  // Adjusts the buffering algorithm based on the given preload value.
  void SetPreload(media::DataSource::Preload preload) override;

  // Returns true if the media resource has a single origin, false otherwise.
  // Only valid to call after Initialize() has completed.
  //
  // Method called on the render thread.
  bool HasSingleOrigin();

  // Provides a callback to be run when the underlying url is redirected.
  void OnRedirect(RedirectCB callback);

  bool PassedTimingAllowOriginCheck() override;

  bool WouldTaintOrigin() override;

  // Returns the CorsMode of the underlying UrlData.
  UrlData::CorsMode cors_mode() const;

  // Notifies changes in playback state for controlling media buffering
  // behavior.
  void OnMediaPlaybackRateChanged(double playback_rate) override;
  void OnMediaIsPlaying() override;
  bool media_has_played() const;

  // Returns true if the resource is local.
  bool AssumeFullyBuffered() const override;

  void StopPreloading() override;

  int64_t GetMemoryUsage() override;

  GURL GetUrlAfterRedirects() const override;

  // media::DataSource implementation.
  // Called from demuxer thread.
  void Stop() override;
  void Abort() override;

  void Read(int64_t position,
            int size,
            uint8_t* data,
            media::DataSource::ReadCB read_cb) override;
  [[nodiscard]] bool GetSize(int64_t* size_out) override;
  bool IsStreaming() override;
  void SetBitrate(int bitrate) override;
  void SetIsClientAudioElement(bool is_client_audio_element) {
    is_client_audio_element_ = is_client_audio_element;
  }

  CrossOriginDataSource* GetAsCrossOriginDataSource() override { return this; }

  bool cancel_on_defer_for_testing() const { return cancel_on_defer_; }

 protected:
  void OnRedirected(const scoped_refptr<UrlData>& new_destination);

  // A factory method to create a BufferedResourceLoader based on the read
  // parameters.
  void CreateResourceLoader(int64_t first_byte_position,
                            int64_t last_byte_position);

  // Same as above, but called with |lock_| held.
  void CreateResourceLoader_Locked(int64_t first_byte_position,
                                   int64_t last_byte_position);

  // Set reader_ while asserting proper locking.
  void SetReader(std::unique_ptr<MultiBufferReader> reader);

  friend class MultiBufferDataSourceTest;

  // Task posted to perform actual reading on the render thread.
  void ReadTask();

  // After a read, this function updates the read position.
  // It's in a separate function because the read itself can either happen
  // in ReadTask() or in Read(), both of which call this function afterwards.
  void SeekTask_Locked();

  // Lock |lock_| lock and call SeekTask_Locked().
  // Called with PostTask when read() complets on the demuxer thread.
  void SeekTask();

  // Cancels oustanding callbacks and sets |stop_signal_received_|. Safe to call
  // from any thread.
  void StopInternal_Locked();

  // Stops |reader_| if present. Used by Abort() and Stop().
  void StopLoader();

  // Tells |reader_| the bitrate of the media.
  void SetBitrateTask(int bitrate);

  // BufferedResourceLoader::Start() callback for initial load.
  void StartCallback();

  // Check if we've moved to a new url and update has_signgle_origin_.
  void UpdateSingleOrigin();

  // MultiBufferReader progress callback.
  void ProgressCallback(int64_t begin, int64_t end);

  // Update progress based on current reader state.
  void UpdateProgress();

  // call downloading_cb_ if needed.
  // If |force_loading| is true, we call downloading_cb_ and tell it that
  // we are currently loading, regardless of what reader_->IsLoading() says.
  // Caller must hold |lock_|.
  void UpdateLoadingState_Locked(bool force_loading);

  // Update |reader_|'s preload and buffer settings.
  void UpdateBufferSizes();

  // The total size of the resource. Set during StartCallback() if the size is
  // known, otherwise it will remain kPositionNotSpecified until the size is
  // determined by reaching EOF.
  int64_t total_bytes_ = kPositionNotSpecified;

  // Bytes we've read but not reported to the url_data yet.
  // SeekTask handles the reporting.
  int64_t bytes_read_ = 0;

  // Places we might want to seek to. After each read we add another
  // location here, and when SeekTask() is called, it picks the best
  // position and then clears it out.
  Vector<int64_t> seek_positions_;

  // This value will be true if this data source can only support streaming.
  // i.e. range request is not supported.
  bool streaming_ = false;

  // This is the loading state that we last reported to our owner through
  // |downloading_cb_|.
  bool loading_ = false;

  // True if a failure has occured.
  bool failed_ = false;

  // The task runner of the render thread.
  const scoped_refptr<base::SingleThreadTaskRunner> render_task_runner_;

  // URL of the resource requested.
  scoped_refptr<UrlData> url_data_;

  // A resource reader for the media resource.
  std::unique_ptr<MultiBufferReader> reader_;

  // Callback method from the pipeline for initialization.
  InitializeCB init_cb_;

  // Read parameters received from the Read() method call. Must be accessed
  // under |lock_|.
  class ReadOperation;
  std::unique_ptr<ReadOperation> read_op_;

  // Protects |stop_signal_received_|, |read_op_|, |reader_| and |total_bytes_|.
  base::Lock lock_;

  // Whether we've been told to stop via Abort() or Stop().
  bool stop_signal_received_ = false;

  // This variable is true when the user has requested the video to play at
  // least once.
  bool media_has_played_ = false;

  // As we follow redirects, we set this variable to false if redirects
  // go between different origins.
  bool single_origin_ = true;

  // Callback used when a redirect occurs.
  RedirectCB redirect_cb_;

  // Stops preloading and closes the connection when we have enough data.
  bool cancel_on_defer_ = false;

  // This variable holds the value of the preload attribute for the video
  // element.
  media::DataSource::Preload preload_ = AUTO;

  // Bitrate of the content, 0 if unknown.
  int bitrate_ = 0;

  // Current playback rate.
  double playback_rate_ = 0;

  std::unique_ptr<media::MediaLog> media_log_;

  bool is_client_audio_element_ = false;

  int buffer_size_update_counter_ = 0;

  // Host object to report buffered byte range changes to.
  raw_ptr<BufferedDataSourceHost, DanglingUntriaged> host_;

  DownloadingCB downloading_cb_;

  // Preload this many seconds of data by default.
  media::Tuneable<int> preload_seconds_ = {"SrcMediaMultiBufferPreloadSeconds",
                                           0, 10, 60};

  // Keep this many seconds of data for going back by default.
  media::Tuneable<int> keep_after_playback_seconds_ = {
      "SrcMediaMultiBufferKeepAfterPlaybackSeconds", 0, 2, 60};

  // Disallow rebinding WeakReference ownership to a different thread by keeping
  // a persistent reference. This avoids problems with the thread-safety of
  // reaching into this class from multiple threads to attain a WeakPtr.
  base::WeakPtr<MultiBufferDataSource> weak_ptr_;
  base::WeakPtrFactory<MultiBufferDataSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_DATA_SOURCE_H_
