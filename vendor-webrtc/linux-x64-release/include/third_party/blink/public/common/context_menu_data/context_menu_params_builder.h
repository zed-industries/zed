// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_PARAMS_BUILDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_PARAMS_BUILDER_H_

#include "third_party/blink/public/common/common_export.h"

namespace blink {

struct UntrustworthyContextMenuParams;
struct ContextMenuData;

class BLINK_COMMON_EXPORT ContextMenuParamsBuilder {
 public:
  static UntrustworthyContextMenuParams Build(
      const blink::ContextMenuData& data);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_PARAMS_BUILDER_H_
