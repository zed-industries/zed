// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CREATE_CDM_UMA_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CREATE_CDM_UMA_HELPER_H_

#include "base/time/time.h"
#include "media/base/cdm_config.h"
#include "media/base/cdm_factory.h"

namespace blink {

// Gets UMA prefix for CDM.
std::string GetUMAPrefixForCdm(const media::CdmConfig& cdm_config);

// Reports CreateCdmStatus UMA.
void ReportCreateCdmStatusUMA(const std::string& uma_prefix,
                              bool is_cdm_created,
                              media::CreateCdmStatus status);

// Reports CreateCdm time UMA.
void ReportCreateCdmTimeUMA(const std::string& uma_prefix,
                            base::TimeDelta sample);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CREATE_CDM_UMA_HELPER_H_
