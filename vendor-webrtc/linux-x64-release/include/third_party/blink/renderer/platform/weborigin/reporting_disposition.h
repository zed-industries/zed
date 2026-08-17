// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_REPORTING_DISPOSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_REPORTING_DISPOSITION_H_

namespace blink {

enum class ReportingDisposition {
  kSuppressReporting,
  kReport,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_REPORTING_DISPOSITION_H_
