// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_PRESET_OPTIONS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_PRESET_OPTIONS_H_

#include <optional>

#include "printing/mojom/print.mojom-shared.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

struct WebPrintPresetOptions {
  // Specifies whether scaling is disabled.
  bool is_scaling_disabled = false;

  // Specifies the number of copies to be printed.
  int copies = 0;

  // Specifies duplex mode to be used for printing.
  printing::mojom::DuplexMode duplex_mode =
      printing::mojom::DuplexMode::kUnknownDuplexMode;

  // Only valid if the page sizes are uniform. The page size in points.
  std::optional<gfx::Size> uniform_page_size;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_PRESET_OPTIONS_H_
