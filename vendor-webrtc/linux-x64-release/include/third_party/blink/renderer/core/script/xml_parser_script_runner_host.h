// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_XML_PARSER_SCRIPT_RUNNER_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_XML_PARSER_SCRIPT_RUNNER_HOST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class CORE_EXPORT XMLParserScriptRunnerHost : public GarbageCollectedMixin {
 public:
  virtual ~XMLParserScriptRunnerHost() = default;
  void Trace(Visitor*) const override {}

  virtual void NotifyScriptExecuted() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_XML_PARSER_SCRIPT_RUNNER_HOST_H_
