// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_ADS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_ADS_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class MODULES_EXPORT Ads final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Ads();
  ~Ads() override;

  Ads(const Ads&) = delete;
  Ads& operator=(const Ads&) = delete;

  // Validates if this is a currently valid Ads object from a successful
  // createAdRequest call.
  bool IsValid() const;

  WTF::String GetGuid() const;

 private:
  bool populated_ = false;
  WTF::String guid_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_ADS_H_
