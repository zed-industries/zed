// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_RESULT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_fetch_later_result.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

// FetchLaterResult represents the state of a fetchLater API call.
// https://whatpr.org/fetch/1647/53e4c3d...71fd383.html#fetch-later-method
//
// The state in this class is read-only and may be updated by C++ world.
class CORE_EXPORT FetchLaterResult final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  FetchLaterResult();

  void SetActivated(bool activated);

  // From fetch_later.idl:
  bool activated() const;

 private:
  bool activated_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_RESULT_H_
