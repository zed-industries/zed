// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGE_DOWNLOADER_IMAGE_DOWNLOADER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGE_DOWNLOADER_IMAGE_DOWNLOADER_IMPL_H_

#include "third_party/blink/public/mojom/image_downloader/image_downloader.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace gfx {
class Size;
}  // namespace gfx

namespace blink {

class KURL;
class LocalFrame;
class MultiResolutionImageResourceFetcher;
class WebString;

class ImageDownloaderImpl final : public GarbageCollected<ImageDownloaderImpl>,
                                  public Supplement<LocalFrame>,
                                  public ExecutionContextLifecycleObserver,
                                  public mojom::blink::ImageDownloader {
 public:
  static const char kSupplementName[];

  explicit ImageDownloaderImpl(LocalFrame&);

  ImageDownloaderImpl(const ImageDownloaderImpl&) = delete;
  ImageDownloaderImpl& operator=(const ImageDownloaderImpl&) = delete;

  ~ImageDownloaderImpl() override;

  using DownloadCallback =
      base::OnceCallback<void(int32_t, const WTF::Vector<SkBitmap>&)>;

  static ImageDownloaderImpl* From(LocalFrame&);

  static void ProvideTo(LocalFrame&);

  void Trace(Visitor*) const override;

  // OverExecutionContextLifecycleObserver overrides.
  void ContextDestroyed() override;

 private:
  // ImageDownloader implementation. Request to asynchronously download an
  // image. When done, |callback| will be called.
  void DownloadImage(const KURL& url,
                     bool is_favicon,
                     const gfx::Size& preferred_size,
                     uint32_t max_bitmap_size,
                     bool bypass_cache,
                     DownloadImageCallback callback) override;

  // ImageDownloader implementation. Request to asynchronously download an
  // image. When done, |callback| will be called.
  void DownloadImageFromAxNode(int ax_node_id,
                               const gfx::Size& preferred_size,
                               uint32_t max_bitmap_size,
                               bool bypass_cache,
                               DownloadImageCallback callback) override;

  // Called when downloading finishes. All frames in |images| whose size <=
  // |max_image_size| will be returned through |callback|. If all of the frames
  // are larger than |max_image_size|, the smallest frame is resized to
  // |max_image_size| and is the only result. |max_image_size| == 0 is
  // interpreted as no max image size.
  void DidDownloadImage(uint32_t max_bitmap_size,
                        DownloadImageCallback callback,
                        int32_t http_status_code,
                        const WTF::Vector<SkBitmap>& images);

  void CreateMojoService(
      mojo::PendingReceiver<mojom::blink::ImageDownloader> receiver);

  void Dispose();

  // Requests to fetch an image. When done, the image downloader is notified by
  // way of DidFetchImage. If the image is a favicon, cookies will not be sent
  // nor accepted during download. If the image has multiple frames, all frames
  // are returned.
  void FetchImage(const KURL& image_url,
                  bool is_favicon,
                  const gfx::Size& preferred_size,
                  bool bypass_cache,
                  DownloadCallback callback);

  // This callback is triggered when FetchImage completes, either
  // successfully or with a failure. See FetchImage for more
  // details.
  void DidFetchImage(DownloadCallback callback,
                     const gfx::Size& preferred_size,
                     MultiResolutionImageResourceFetcher* fetcher,
                     const std::string& image_data,
                     const WebString& mime_type);

  typedef WTF::Vector<std::unique_ptr<MultiResolutionImageResourceFetcher>>
      ImageResourceFetcherList;

  // ImageResourceFetchers schedule via FetchImage.
  ImageResourceFetcherList image_fetchers_;

  HeapMojoReceiver<mojom::blink::ImageDownloader,
                   ImageDownloaderImpl,
                   HeapMojoWrapperMode::kForceWithoutContextObserver>
      receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGE_DOWNLOADER_IMAGE_DOWNLOADER_IMPL_H_
