// Copyright 2022 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_PORT_OPENCV_VIDEO_INC_H_
#define MEDIAPIPE_PORT_OPENCV_VIDEO_INC_H_

#include <opencv2/core/version.hpp>

#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/opencv_core_inc.h"

#ifdef CV_VERSION_EPOCH  // for OpenCV 2.x
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/video/video.hpp>
// Copied from "opencv2/videoio.hpp" in OpenCV 4.0.1
namespace cv {
enum VideoCaptureProperties {
  CAP_PROP_POS_MSEC = 0,
  CAP_PROP_POS_FRAMES = 1,
  CAP_PROP_POS_AVI_RATIO = 2,
  CAP_PROP_FRAME_WIDTH = 3,
  CAP_PROP_FRAME_HEIGHT = 4,
  CAP_PROP_FPS = 5,
  CAP_PROP_FOURCC = 6,
  CAP_PROP_FRAME_COUNT = 7,
  CAP_PROP_FORMAT = 8,
  CAP_PROP_MODE = 9,
  CAP_PROP_BRIGHTNESS = 10,
  CAP_PROP_CONTRAST = 11,
  CAP_PROP_SATURATION = 12,
  CAP_PROP_HUE = 13,
  CAP_PROP_GAIN = 14,
  CAP_PROP_EXPOSURE = 15,
  CAP_PROP_CONVERT_RGB = 16,
  CAP_PROP_WHITE_BALANCE_BLUE_U = 17,
  CAP_PROP_RECTIFICATION = 18,
  CAP_PROP_MONOCHROME = 19,
  CAP_PROP_SHARPNESS = 20,
  CAP_PROP_AUTO_EXPOSURE = 21,
  CAP_PROP_GAMMA = 22,
  CAP_PROP_TEMPERATURE = 23,
  CAP_PROP_TRIGGER = 24,
  CAP_PROP_TRIGGER_DELAY = 25,
  CAP_PROP_WHITE_BALANCE_RED_V = 26,
  CAP_PROP_ZOOM = 27,
  CAP_PROP_FOCUS = 28,
  CAP_PROP_GUID = 29,
  CAP_PROP_ISO_SPEED = 30,
  CAP_PROP_BACKLIGHT = 32,
  CAP_PROP_PAN = 33,
  CAP_PROP_TILT = 34,
  CAP_PROP_ROLL = 35,
  CAP_PROP_IRIS = 36,
  CAP_PROP_SETTINGS = 37,
  CAP_PROP_BUFFERSIZE = 38,
  CAP_PROP_AUTOFOCUS = 39,
  CAP_PROP_SAR_NUM = 40,
  CAP_PROP_SAR_DEN = 41,
  CAP_PROP_BACKEND = 42,
  CAP_PROP_CHANNEL = 43,
  CAP_PROP_AUTO_WB = 44,
  CAP_PROP_WB_TEMPERATURE = 45,
};
}  // namespace cv

namespace mediapipe {
inline int fourcc(char c1, char c2, char c3, char c4) {
  return CV_FOURCC(c1, c2, c3, c4);
}
}  // namespace mediapipe

#else
#include <opencv2/video.hpp>
#include <opencv2/videoio.hpp>

#if CV_VERSION_MAJOR == 4 && !defined(MEDIAPIPE_MOBILE)
#include <opencv2/optflow.hpp>

namespace cv {
inline Ptr<DenseOpticalFlow> createOptFlow_DualTVL1() {
  return optflow::createOptFlow_DualTVL1();
}
}  // namespace cv
#endif

namespace mediapipe {
inline int fourcc(char c1, char c2, char c3, char c4) {
  return cv::VideoWriter::fourcc(c1, c2, c3, c4);
}
}  // namespace mediapipe
#endif

#endif  // MEDIAPIPE_PORT_OPENCV_VIDEO_INC_H_
