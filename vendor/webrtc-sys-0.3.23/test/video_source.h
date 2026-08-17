/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef WEBRTC_MODULES_VIDEO_CODING_CODECS_TEST_FRAMEWORK_VIDEO_SOURCE_H_
#define WEBRTC_MODULES_VIDEO_CODING_CODECS_TEST_FRAMEWORK_VIDEO_SOURCE_H_

#include <string>
#include "common_video/libyuv/include/webrtc_libyuv.h"

enum VideoSize
    {
        kUndefined,
        kSQCIF,     // 128*96       = 12 288
        kQQVGA,     // 160*120      = 19 200
        kQCIF,      // 176*144      = 25 344
        kCGA,       // 320*200      = 64 000
        kQVGA,      // 320*240      = 76 800
        kSIF,       // 352*240      = 84 480
        kWQVGA,     // 400*240      = 96 000
        kCIF,       // 352*288      = 101 376
        kW288p,     // 512*288      = 147 456 (WCIF)
        k448p,      // 576*448      = 281 088
        kVGA,       // 640*480      = 307 200
        k432p,      // 720*432      = 311 040
        kW432p,     // 768*432      = 331 776
        k4SIF,      // 704*480      = 337 920
        kW448p,     // 768*448      = 344 064
        kNTSC,		// 720*480      = 345 600
        kFW448p,    // 800*448      = 358 400
        kWVGA,      // 800*480      = 384 000
        k4CIF,      // 704�576      = 405 504
        kSVGA,      // 800*600      = 480 000
        kW544p,     // 960*544      = 522 240
        kW576p,     // 1024*576     = 589 824 (W4CIF)
        kHD,        // 960*720      = 691 200
        kXGA,       // 1024*768     = 786 432
        kWHD,       // 1280*720     = 921 600
        kFullHD,    // 1440*1080    = 1 555 200
        kWFullHD,   // 1920*1080    = 2 073 600

        kNumberOfVideoSizes
    };

class VideoSource
{
public:
    VideoSource();
    VideoSource(std::string fileName, VideoSize size, int frameRate = 30,
        webrtc::VideoType type = webrtc::VideoType::kI420);
    VideoSource(std::string fileName, int width, int height, int frameRate = 30,
                webrtc::VideoType type = webrtc::VideoType::kI420);

    std::string GetFileName() const { return _fileName; }
    int GetWidth() const { return _width; }
    int GetHeight() const { return _height; }
    webrtc::VideoType GetType() const { return _type; }
    int GetFrameRate() const { return _frameRate; }

    // Returns the file path without a trailing slash.
    std::string GetFilePath() const;

    // Returns the filename with the path (including the leading slash) removed.
    std::string GetName() const;

    VideoSize GetSize() const;
    static VideoSize GetSize(uint16_t width, uint16_t height);
    unsigned int GetFrameLength() const;

    // Returns a human-readable size string.
    static const char* GetSizeString(VideoSize size);
    const char* GetMySizeString() const;

    // Opens the video source, converting and writing to the specified target.
    // If force is true, the conversion will be done even if the target file
    // already exists.
    void Convert(const VideoSource& target, bool force = false) const;
    static bool FileExists(const char* fileName);
private:
    static int GetWidthHeight( VideoSize size, int& width, int& height);
    std::string _fileName;
    int _width;
    int _height;
    webrtc::VideoType _type;
    int _frameRate;
};

class FrameDropper
{
public:
    FrameDropper();
    bool DropFrame();
    unsigned int DropsBetweenRenders();
    void SetFrameRate(double frameRate, double maxFrameRate);

private:
    unsigned int _dropsBetweenRenders;
    unsigned int _frameCounter;
};


#endif // WEBRTC_MODULES_VIDEO_CODING_CODECS_TEST_FRAMEWORK_VIDEO_SOURCE_H_

