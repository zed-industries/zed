// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_UKEY2_D2D_CONNECTION_CONTEXT_FACTORY_H_
#define THIRD_PARTY_UKEY2_D2D_CONNECTION_CONTEXT_FACTORY_H_

#include <memory>

#include "third_party/ukey2/src/src/main/cpp/include/securegcm/d2d_connection_context_v1.h"

namespace securegcm {

std::unique_ptr<securegcm::D2DConnectionContextV1> CreateServerContext();

std::unique_ptr<securegcm::D2DConnectionContextV1> CreateClientContext();

}  // namespace securegcm

#endif  // THIRD_PARTY_UKEY2_D2D_CONNECTION_CONTEXT_FACTORY_H_
