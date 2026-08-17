// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_RESOURCE_PRELOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_RESOURCE_PRELOADER_H_

#include <memory>
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/parser/preload_request.h"

namespace blink {

class CORE_EXPORT ResourcePreloader {
 public:
  virtual void TakeAndPreload(PreloadRequestStream&);

 private:
  virtual void Preload(std::unique_ptr<PreloadRequest>) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_RESOURCE_PRELOADER_H_
