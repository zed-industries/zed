// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_NAVIGATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_NAVIGATOR_H_

#include "third_party/blink/renderer/core/execution_context/navigator_base.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class MODULES_EXPORT SharedStorageWorkletNavigator final
    : public NavigatorBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SharedStorageWorkletNavigator(ExecutionContext*);
  ~SharedStorageWorkletNavigator() override;

  // NavigatorLanguage override
  String GetAcceptLanguages() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_NAVIGATOR_H_
