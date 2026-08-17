// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_TYPE_CONVERTERS_H_

#include "third_party/blink/public/mojom/background_fetch/background_fetch.mojom-blink.h"

namespace blink {
class BackgroundFetchOptions;
}

namespace mojo {

template <>
struct TypeConverter<blink::mojom::blink::BackgroundFetchOptionsPtr,
                     const blink::BackgroundFetchOptions*> {
  static blink::mojom::blink::BackgroundFetchOptionsPtr Convert(
      const blink::BackgroundFetchOptions* options);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_TYPE_CONVERTERS_H_
