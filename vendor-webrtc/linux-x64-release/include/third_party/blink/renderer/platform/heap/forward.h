// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_FORWARD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_FORWARD_H_

namespace cppgc {
class LivenessBroker;
class Visitor;
}

namespace blink {

using LivenessBroker = cppgc::LivenessBroker;
using Visitor = cppgc::Visitor;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_FORWARD_H_
