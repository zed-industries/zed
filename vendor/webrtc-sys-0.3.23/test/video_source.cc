/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#include "video_source.h"

#include <stdio.h>

#include "fileutils.h"

#define ASSERT_TRUE(condition) \
  do { \
    if (!(condition)) { \
      fprintf(stderr, "Assertion failed: %s\n", #condition); \
      abort(); \
    } \
  } while (0)

VideoSource::VideoSource()
:
_fileName(webrtc::test::ProjectRootPath() + "resources/foreman_cif.yuv"),
_width(352),
_height(288),
_type(webrtc::VideoType::kI420),
_frameRate(30)
{
}

VideoSource::VideoSource(std::string fileName, VideoSize size,
    int frameRate /*= 30*/, webrtc::VideoType type /*=  webrtc::kI420*/)
:
_fileName(fileName),
_type(type),
_frameRate(frameRate)
{
    assert(size != kUndefined && size != kNumberOfVideoSizes);
    assert(type != webrtc::VideoType::kUnknown);
    assert(frameRate > 0);
    if (GetWidthHeight(size, _width, _height) != 0) {
        assert(false);
    }
}

VideoSource::VideoSource(std::string fileName, int width, int height,
    int frameRate /*= 30*/,  webrtc::VideoType type /*=  webrtc::kI420*/)
:
_fileName(fileName),
_width(width),
_height(height),
_type(type),
_frameRate(frameRate)
{
    assert(width > 0);
    assert(height > 0);
    assert(type != webrtc::VideoType::kUnknown);
    assert(frameRate > 0);
}

VideoSize
VideoSource::GetSize() const
{
    return GetSize(_width, _height);
}

VideoSize
VideoSource::GetSize(uint16_t width, uint16_t height)
{
    if(width == 128 && height == 96)
    {
        return kSQCIF;
    }else if(width == 160 && height == 120)
    {
        return kQQVGA;
    }else if(width == 176 && height == 144)
    {
        return kQCIF;
    }else if(width == 320 && height == 240)
    {
        return kQVGA;
    }else if(width == 352 && height == 288)
    {
        return kCIF;
    }else if(width == 640 && height == 480)
    {
        return kVGA;
    }else if(width == 720 && height == 480)
    {
        return kNTSC;
    }else if(width == 704 && height == 576)
    {
        return k4CIF;
    }else if(width == 800 && height == 600)
    {
        return kSVGA;
    }else if(width == 960 && height == 720)
    {
        return kHD;
    }else if(width == 1024 && height == 768)
    {
        return kXGA;
    }else if(width == 1440 && height == 1080)
    {
        return kFullHD;
    }else if(width == 400 && height == 240)
    {
        return kWQVGA;
    }else if(width == 800 && height == 480)
    {
        return kWVGA;
    }else if(width == 1280 && height == 720)
    {
        return kWHD;
    }else if(width == 1920 && height == 1080)
    {
        return kWFullHD;
    }
    return kUndefined;
}

unsigned int
VideoSource::GetFrameLength() const
{
    return webrtc::CalcBufferSize(_type, _width, _height);
}

const char*
VideoSource::GetMySizeString() const
{
    return VideoSource::GetSizeString(GetSize());
}

const char*
VideoSource::GetSizeString(VideoSize size)
{
    switch (size)
    {
        case kSQCIF:
            return "SQCIF";
        case kQQVGA:
            return "QQVGA";
        case kQCIF:
            return "QCIF";
        case kQVGA:
            return "QVGA";
        case kCIF:
            return "CIF";
        case kVGA:
            return "VGA";
        case kNTSC:
            return "NTSC";
        case k4CIF:
            return "4CIF";
        case kSVGA:
            return "SVGA";
        case kHD:
            return "HD";
        case kXGA:
            return "XGA";
        case kFullHD:
            return "Full_HD";
        case kWQVGA:
            return "WQVGA";
        case kWHD:
            return "WHD";
        case kWFullHD:
            return "WFull_HD";
        default:
            return "Undefined";
    }
}

std::string
VideoSource::GetFilePath() const
{
    size_t slashPos = _fileName.find_last_of("/\\");
    if (slashPos == std::string::npos)
    {
        return ".";
    }

    return _fileName.substr(0, slashPos);
}

std::string
VideoSource::GetName() const
{
    // Remove path.
    size_t slashPos = _fileName.find_last_of("/\\");
    if (slashPos == std::string::npos)
    {
        slashPos = 0;
    }
    else
    {
        slashPos++;
    }

    // Remove extension and underscored suffix if it exists.
    return _fileName.substr(slashPos, std::min(_fileName.find_last_of("_"),
        _fileName.find_last_of(".")) - slashPos);
}

void
VideoSource::Convert(const VideoSource &target, bool force /* = false */) const
{
    // Ensure target rate is less than or equal to source
    // (i.e. we are only temporally downsampling).
    ASSERT_TRUE(target.GetFrameRate() <= _frameRate);
    // Only supports YUV420 currently.
    ASSERT_TRUE(_type == webrtc::VideoType::kI420 && target.GetType() == webrtc::VideoType::kI420);
    if (!force && (FileExists(target.GetFileName().c_str()) ||
        (target.GetWidth() == _width && target.GetHeight() == _height && target.GetFrameRate() == _frameRate)))
    {
        // Assume that the filename uniquely defines the content.
        // If the file already exists, it is the correct file.
        return;
    }
    FILE *inFile = NULL;
    FILE *outFile = NULL;

    inFile = fopen(_fileName.c_str(), "rb");
    ASSERT_TRUE(inFile != NULL);

    outFile = fopen(target.GetFileName().c_str(), "wb");
    ASSERT_TRUE(outFile != NULL);

    FrameDropper fd;
    fd.SetFrameRate(target.GetFrameRate(), _frameRate);

    const size_t lengthOutFrame = webrtc::CalcBufferSize(target.GetType(),
        target.GetWidth(), target.GetHeight());
    ASSERT_TRUE(lengthOutFrame > 0);
    unsigned char *outFrame = new unsigned char[lengthOutFrame];

    const size_t lengthInFrame = webrtc::CalcBufferSize(_type, _width, _height);
    ASSERT_TRUE(lengthInFrame > 0);
    unsigned char *inFrame = new unsigned char[lengthInFrame];

    while (fread(inFrame, 1, lengthInFrame, inFile) == lengthInFrame)
    {
        if (!fd.DropFrame())
        {
            ASSERT_TRUE(target.GetWidth() == _width &&
                   target.GetHeight() == _height);
            // Add video interpolator here!
            if (fwrite(outFrame, 1, lengthOutFrame,
                       outFile) !=  lengthOutFrame) {
              return;
            }
        }
    }

    delete inFrame;
    delete outFrame;
    fclose(inFile);
    fclose(outFile);
}

bool VideoSource::FileExists(const char* fileName)
{
    FILE* fp = NULL;
    fp = fopen(fileName, "rb");
    if(fp != NULL)
    {
        fclose(fp);
        return true;
    }
    return false;
}


int
VideoSource::GetWidthHeight( VideoSize size, int & width, int& height)
{
    switch(size)
    {
    case kSQCIF:
        width = 128;
        height = 96;
        return 0;
    case kQQVGA:
        width = 160;
        height = 120;
        return 0;
    case kQCIF:
        width = 176;
        height = 144;
        return 0;
    case kCGA:
        width = 320;
        height = 200;
        return 0;
    case kQVGA:
        width = 320;
        height = 240;
        return 0;
    case kSIF:
        width = 352;
        height = 240;
        return 0;
    case kWQVGA:
        width = 400;
        height = 240;
        return 0;
    case kCIF:
        width = 352;
        height = 288;
        return 0;
    case kW288p:
        width = 512;
        height = 288;
        return 0;
    case k448p:
        width = 576;
        height = 448;
        return 0;
    case kVGA:
        width = 640;
        height = 480;
        return 0;
    case k432p:
        width = 720;
        height = 432;
        return 0;
    case kW432p:
        width = 768;
        height = 432;
        return 0;
    case k4SIF:
        width = 704;
        height = 480;
        return 0;
    case kW448p:
        width = 768;
        height = 448;
        return 0;
    case kNTSC:
        width = 720;
        height = 480;
        return 0;
    case kFW448p:
        width = 800;
        height = 448;
        return 0;
    case kWVGA:
        width = 800;
        height = 480;
        return 0;
    case k4CIF:
        width = 704;
        height = 576;
        return 0;
    case kSVGA:
        width = 800;
        height = 600;
        return 0;
    case kW544p:
        width = 960;
        height = 544;
        return 0;
    case kW576p:
        width = 1024;
        height = 576;
        return 0;
    case kHD:
        width = 960;
        height = 720;
        return 0;
    case kXGA:
        width = 1024;
        height = 768;
        return 0;
    case kFullHD:
        width = 1440;
        height = 1080;
        return 0;
    case kWHD:
        width = 1280;
        height = 720;
        return 0;
    case kWFullHD:
        width = 1920;
        height = 1080;
        return 0;
    default:
        return -1;
    }
}

FrameDropper::FrameDropper()
:
_dropsBetweenRenders(0),
_frameCounter(0)
{
}

bool
FrameDropper::DropFrame()
{
    _frameCounter++;
    if (_frameCounter > _dropsBetweenRenders)
    {
        _frameCounter = 0;
        return false;
    }
    return true;
}

unsigned int
FrameDropper::DropsBetweenRenders()
{
    return _dropsBetweenRenders;
}

void
FrameDropper::SetFrameRate(double frameRate, double maxFrameRate)
{
    if (frameRate >= 1.0)
    {
        _dropsBetweenRenders = static_cast<unsigned int>(maxFrameRate / frameRate + 0.5) - 1;
    }
    else
    {
        _dropsBetweenRenders = 0;
    }
}
