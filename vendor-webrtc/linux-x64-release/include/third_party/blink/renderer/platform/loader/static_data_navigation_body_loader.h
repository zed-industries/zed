// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_STATIC_DATA_NAVIGATION_BODY_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_STATIC_DATA_NAVIGATION_BODY_LOADER_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/public/platform/web_navigation_body_loader.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"

namespace blink {

// This class allows to write navigation body from outside,
// and adheres to the contract of WebNavigationBodyLoader.
// Used for tests and static (as in "not loaded over network") response body.
class PLATFORM_EXPORT StaticDataNavigationBodyLoader
    : public WebNavigationBodyLoader {
 public:
  static std::unique_ptr<StaticDataNavigationBodyLoader> CreateWithData(
      scoped_refptr<SharedBuffer> data);

  StaticDataNavigationBodyLoader();
  ~StaticDataNavigationBodyLoader() override;

  void Write(base::span<const char> data);

  void Finish();

  void SetDefersLoading(LoaderFreezeMode) override;
  void StartLoadingBody(WebNavigationBodyLoader::Client*) override;
  BodyLoaderType GetType() const override { return BodyLoaderType::kStatic; }

 private:
  void Continue();

  scoped_refptr<SharedBuffer> data_;
  raw_ptr<WebNavigationBodyLoader::Client> client_ = nullptr;
  LoaderFreezeMode freeze_mode_ = LoaderFreezeMode::kNone;
  bool sent_all_data_ = false;
  bool received_all_data_ = false;
  bool is_in_continue_ = false;
  int64_t total_encoded_data_length_ = 0;
  base::WeakPtrFactory<StaticDataNavigationBodyLoader> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_STATIC_DATA_NAVIGATION_BODY_LOADER_H_
