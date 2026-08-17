// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_PAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_PAGE_H_

#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Page;
class VisualViewport;

class SimPage final {
 public:
  SimPage();
  ~SimPage();

  void SetPage(Page*);

  void SetFocused(bool);
  bool IsFocused() const;

  void SetActive(bool);
  bool IsActive() const;

  const VisualViewport& GetVisualViewport() const;

 private:
  Persistent<Page> page_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_PAGE_H_
