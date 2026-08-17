/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_CREATE_VIDEO_QUALITY_TEST_FIXTURE_H_
#define API_TEST_CREATE_VIDEO_QUALITY_TEST_FIXTURE_H_

#include <memory>

#include "api/fec_controller.h"
#include "api/test/video_quality_test_fixture.h"

namespace webrtc {

std::unique_ptr<VideoQualityTestFixtureInterface>
CreateVideoQualityTestFixture();

std::unique_ptr<VideoQualityTestFixtureInterface> CreateVideoQualityTestFixture(
    std::unique_ptr<FecControllerFactoryInterface> fec_controller_factory);

std::unique_ptr<VideoQualityTestFixtureInterface> CreateVideoQualityTestFixture(
    std::unique_ptr<VideoQualityTestFixtureInterface::InjectionComponents>
        components);
}  // namespace webrtc

#endif  // API_TEST_CREATE_VIDEO_QUALITY_TEST_FIXTURE_H_
