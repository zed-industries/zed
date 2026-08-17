// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_DATA_SOURCE_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_DATA_SOURCE_FACTORY_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/tick_clock.h"
#include "base/types/pass_key.h"
#include "media/base/data_source.h"
#include "media/base/media_log.h"
#include "media/filters/hls_data_source_provider_impl.h"
#include "third_party/blink/renderer/platform/media/url_index.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class BufferedDataSourceHostImpl;

class PLATFORM_EXPORT MultiBufferDataSourceFactory
    : public media::HlsDataSourceProviderImpl::DataSourceFactory {
 public:
  using UrlDataCb = base::RepeatingCallback<void(
      const GURL& url,
      bool ignore_cache,
      base::OnceCallback<void(scoped_refptr<UrlData>)>)>;

  ~MultiBufferDataSourceFactory() override;
  MultiBufferDataSourceFactory(
      media::MediaLog* media_log,
      UrlDataCb get_url_data,
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      const base::TickClock* tick_clock);

  void CreateDataSource(GURL uri, bool ignore_cache, DataSourceCb cb) override;

 private:
  void OnUrlData(DataSourceCb cb,
                 base::RepeatingCallback<void(bool)> download_cb,
                 scoped_refptr<UrlData> data);

  std::unique_ptr<media::MediaLog> media_log_;
  UrlDataCb get_url_data_;
  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;

  std::unique_ptr<BufferedDataSourceHostImpl> buffered_data_source_host_;
  base::WeakPtrFactory<MultiBufferDataSourceFactory> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_DATA_SOURCE_FACTORY_H_
