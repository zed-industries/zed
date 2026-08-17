/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#include "benchmark.h"

#include <cassert>
#include <iostream>
#include <sstream>
#include <vector>
#if defined(_WIN32)
#include <windows.h>
#endif

#include "api/video/i420_buffer.h"
#include "common_video/libyuv/include/webrtc_libyuv.h"
#include "fileutils.h"
#include "modules/video_coding/utility/simulcast_rate_allocator.h"
#include "rtc_base/event.h"
#include "video_source.h"

#define SSIM_CALC 0  // by default, don't compute SSIM

using namespace webrtc;

#define EXPECT_EQ (a, b)

FrameQueueTuple::~FrameQueueTuple() {
  if (_codecSpecificInfo != NULL) {
    delete _codecSpecificInfo;
  }
  if (_frame != NULL) {
    delete _frame;
  }
}

void FrameQueue::PushFrame(VideoFrame* frame,
                           webrtc::CodecSpecificInfo* codecSpecificInfo) {
  webrtc::MutexLock cs(&_queueRWLock);
  _frameBufferQueue.push(new FrameQueueTuple(frame, codecSpecificInfo));
}

FrameQueueTuple* FrameQueue::PopFrame() {
  webrtc::MutexLock cs(&_queueRWLock);
  if (_frameBufferQueue.empty()) {
    return NULL;
  }
  FrameQueueTuple* tuple = _frameBufferQueue.front();
  _frameBufferQueue.pop();
  return tuple;
}

bool FrameQueue::Empty() {
  webrtc::MutexLock cs(&_queueRWLock);
  return _frameBufferQueue.empty();
}

uint32_t VideoEncodeCompleteCallback::EncodedBytes() {
  return _encodedBytes;
}

webrtc::EncodedImageCallback::Result
VideoEncodeCompleteCallback::OnEncodedImage(
    const webrtc::EncodedImage& encodedImage,
    const webrtc::CodecSpecificInfo* codecSpecificInfo) {
  _test.UpdateEncodedBytes(encodedImage.GetEncodedData()->size());
  _encodedBytes += encodedImage.GetEncodedData()->size();

  if (_encodedFile != NULL) {
    if (fwrite(encodedImage.GetEncodedData()->data(), 1,
               encodedImage.GetEncodedData()->size(),
               _encodedFile) != encodedImage.GetEncodedData()->size()) {
      fprintf(stderr, "Error writing to encoded file %s\n",
              _test._outname.c_str());
    }
  }

  return webrtc::EncodedImageCallback::Result(
      webrtc::EncodedImageCallback::Result::OK);
}

Benchmark::Benchmark()
    : _resultsFileName(webrtc::test::OutputPath() + "benchmark.txt"),
      _codecName("Default"),
      _env(webrtc::CreateEnvironment()) {}

Benchmark::Benchmark(std::string name, std::string description)
    : _name(name),
      _description(description),
      _resultsFileName(webrtc::test::OutputPath() + "benchmark.txt"),
      _codecName("Default"),
      _env(webrtc::CreateEnvironment()) {}

Benchmark::Benchmark(std::string name,
                     std::string description,
                     std::string resultsFileName,
                     std::string codecName)
    : _name(name),
      _description(description),
      _resultsFileName(resultsFileName),
      _codecName(codecName),
      _cpu(webrtc::CpuWrapper::CreateCpu()),
      _env(webrtc::CreateEnvironment()) {}

void Benchmark::Perform() {
  std::vector<const VideoSource*> sources;
  std::vector<const VideoSource*>::iterator it;

  // Configuration --------------------------
  sources.push_back(new const VideoSource(
      webrtc::test::ProjectRootPath() + "resources/FourPeople_1280x720_30.yuv",
      kWHD));
  //sources.push_back(
  //    new const VideoSource(webrtc::test::ProjectRootPath() +
  //                              "resources/Big_Buck_Bunny_1920x1080_30.yuv",
  //                          kWFullHD));

  const VideoSize size[] = {kWHD};
  const int frameRate[] = {30};
  // Specifies the framerates for which to perform a speed test.
  const bool speedTestMask[] = {true};
  const int bitRate[] = {500, 1000, 2000, 3000, 4000};
  // Determines the number of iterations to perform to arrive at the speed
  // result.
  enum { kSpeedTestIterations = 8 };
  // ----------------------------------------

  const int nFrameRates = sizeof(frameRate) / sizeof(*frameRate);
  assert(sizeof(speedTestMask) / sizeof(*speedTestMask) == nFrameRates);
  const int nBitrates = sizeof(bitRate) / sizeof(*bitRate);
  int testIterations = 1;

  double fps[nBitrates];
  uint32_t cpuUsage[nBitrates];
  double totalEncodeTime[nBitrates];
  double totalDecodeTime[nBitrates];

  _results.open(_resultsFileName.c_str(), std::fstream::out);
  _results << GetMagicStr() << std::endl;
  _results << _codecName << std::endl;

  for (it = sources.begin(); it < sources.end(); it++) {
    int i = 0;
    for (int j = 0; j < nFrameRates; j++) {
      _target = *it;
      _inname = (*it)->GetFileName();
      std::cout << (*it)->GetName() << ", "
                << VideoSource::GetSizeString(size[i]) << ", " << frameRate[j]
                << " fps" << ", " << _name << std::endl;
      _results << (*it)->GetName() << "," << VideoSource::GetSizeString(size[i])
               << "," << frameRate[j] << " fps" << ", " << _name << std::endl
               << "Bitrate [kbps]";
      
      if (speedTestMask[j]) {
        testIterations = kSpeedTestIterations;
      } else {
        testIterations = 1;
      }

      for (int k = 0; k < nBitrates; k++) {
        _bitRate = (bitRate[k]);
        double avgFps = 0.0;
        uint32_t currCpuUsage = 0;
        totalEncodeTime[k] = 0;

        std::cout << "TargetBitrate [kbps]:" << " " << _bitRate << std::endl;

        for (int l = 0; l < testIterations; l++) {
          PerformNormalTest();
          uint32_t cpuUsage = _cpu->CpuUsage();
          if (cpuUsage > 0) {
            currCpuUsage += cpuUsage;
            int coreCount = _cpu->GetNumCores();
            std::string str = "CPU Usage[%]: cores ";
            str += std::to_string(coreCount);
            str += ", usage " + std::to_string(cpuUsage) + "%" +
                   ", Test Iteration: " + std::to_string(l + 1) + "/" +
                   std::to_string(testIterations);
            std::cout << str << std::flush;
            for (int i = 0; i < str.length(); ++i) {
              std::cout << "\b";
            }
          }
          _appendNext = false;
          avgFps += _framecnt / (_totalEncodeTime);
          totalEncodeTime[k] += _totalEncodeTime;
        }
        avgFps /= testIterations;
        totalEncodeTime[k] /= testIterations;
        currCpuUsage /= testIterations;

        double actualBitRate = ActualBitRate(_framecnt) / 1000.0;
        std::cout << "ActualBitRate [kbps]:" << " " << actualBitRate
                  << std::endl;
        _results << "," << actualBitRate;
        fps[k] = avgFps;
        cpuUsage[k] = currCpuUsage;
      }

      std::cout << std::endl << "CpuUsage [%]:";
      _results << std::endl << "CpuUsage [%]";
      for (int k = 0; k < nBitrates; k++) {
        std::cout << " " << cpuUsage[k] << "%";
        _results << "," << cpuUsage[k] << "%";
      }
      std::cout << std::endl << "Encode Time[ms]:";
      _results << std::endl << "Encode Time[ms]";
      for (int k = 0; k < nBitrates; k++) {
        std::cout << " " << totalEncodeTime[k];
        _results << "," << totalEncodeTime[k];
      }

      if (speedTestMask[j]) {
        std::cout << std::endl << "Speed [fps]:";
        _results << std::endl << "Speed [fps]";
        for (int k = 0; k < nBitrates; k++) {
          std::cout << " " << static_cast<int>(fps[k] + 0.5);
          _results << "," << static_cast<int>(fps[k] + 0.5);
        }
      }
      std::cout << std::endl << std::endl;
      _results << std::endl << std::endl;
    }
    i++;
    delete *it;
  }
  _results.close();
}

void Benchmark::PerformNormalTest() {
  _encoder = GetNewEncoder(_env);
  _lengthSourceFrame = _target->GetFrameLength();
  CodecSettings(_target->GetWidth(), _target->GetHeight(),
                _target->GetFrameRate(), _bitRate);
  Setup();
  std::unique_ptr<webrtc::Event> waitEvent = std::make_unique<webrtc::Event>();
  //_inputVideoBuffer.VerifyAndAllocate(_lengthSourceFrame);
  _encoder->InitEncode(&_inst, 4, 1440);
  CodecSpecific_InitBitrate();
  //_decoder->InitDecode(&_inst,1);

  FrameQueue frameQueue;
  VideoEncodeCompleteCallback encCallback(_encodedFile, &frameQueue, *this);

  _encoder->RegisterEncodeCompleteCallback(&encCallback);

  _totalEncodeTime = _totalDecodeTime = 0;
  _totalEncodePipeTime = _totalDecodePipeTime = 0;
  bool complete = false;
  _framecnt = 0;
  _encFrameCnt = 0;
  _sumEncBytes = 0;
  _lengthEncFrame = 0;
  while (!complete) {
    complete = Encode();
    _framecnt++;
    _encFrameCnt++;
    /*
    if (!frameQueue.Empty() || complete) {
      while (!frameQueue.Empty()) {
        _frameToDecode = static_cast<FrameQueueTuple*>(frameQueue.PopFrame());
        int ret = Decode();
        delete _frameToDecode;
        _frameToDecode = NULL;
        if (ret < 0) {
          fprintf(stderr, "\n\nError in decoder: %d\n\n", ret);
          exit(EXIT_FAILURE);
        } else if (ret == 0) {
          _framecnt++;
        } else {
          fprintf(stderr, "\n\nPositive return value from decode!\n\n");
        }
      }
    }*/
    // waitEvent->Wait(webrtc::TimeDelta::Seconds(5));
  }

  //_inputVideoBuffer.Free();
  //_encodedVideoBuffer.Free();
  //_decodedVideoBuffer.Free();

  Teardown();
}

void Benchmark::Teardown() {
  // Use _sourceFile as a check to prevent multiple Teardown() calls.
  if (_sourceFile == NULL) {
    return;
  }

  _encoder->Release();

  fclose(_sourceFile);
  _sourceFile = NULL;

  delete[] _sourceBuffer;
  _sourceBuffer = NULL;
}

void Benchmark::CodecSpecific_InitBitrate() {
  webrtc::SimulcastRateAllocator init_allocator(_env,_inst);

  if (_bitRate == 0) {
    VideoBitrateAllocation allocation =
        init_allocator.Allocate(VideoBitrateAllocationParameters(
            DataRate::KilobitsPerSec(600), _inst.maxFramerate));
    _encoder->SetRates(webrtc::VideoEncoder::RateControlParameters(
        allocation, _inst.maxFramerate));
  } else {
    VideoBitrateAllocation allocation =
        init_allocator.Allocate(VideoBitrateAllocationParameters(
            DataRate::BitsPerSec(_bitRate), _inst.maxFramerate));
    _encoder->SetRates(webrtc::VideoEncoder::RateControlParameters(
        allocation, _inst.maxFramerate));
  }
}

bool Benchmark::Encode() {
  _lengthEncFrame = 0;
  if (_sourceBuffer == NULL) {
    _sourceBuffer = new unsigned char[_lengthSourceFrame];
  }
  auto size = fread(_sourceBuffer, 1, _lengthSourceFrame, _sourceFile);
  if (size <= 0) {
    return true;
  }
  // TODO: build video frame from buffer ptr.
  webrtc::scoped_refptr<webrtc::I420Buffer> buffer(
      webrtc::I420Buffer::Create(_inst.width, _inst.height));

  buffer->InitializeData();

  memcpy(buffer->MutableDataY(), _sourceBuffer, _lengthSourceFrame);

  webrtc::VideoFrame inputVideoBuffer =
      webrtc::VideoFrame::Builder()
          .set_video_frame_buffer(buffer)
          .set_rtp_timestamp(
              (unsigned int)(_encFrameCnt * 9e4 / _inst.maxFramerate))
          .build();

  if (feof(_sourceFile) != 0) {
    return true;
  }
  _encodeCompleteTime = 0;
  _encodeTimes[inputVideoBuffer.rtp_timestamp()] = tGetTime();
  std::vector<VideoFrameType> frame_types(1, VideoFrameType::kVideoFrameDelta);

  // check SLI queue
  _hasReceivedSLI = false;
  while (!_signalSLI.empty() && _signalSLI.front().delay == 0) {
    // SLI message has arrived at sender side
    _hasReceivedSLI = true;
    _pictureIdSLI = _signalSLI.front().id;
    _signalSLI.pop_front();
  }
  // decrement SLI queue times
  for (std::list<fbSignal>::iterator it = _signalSLI.begin();
       it != _signalSLI.end(); it++) {
    (*it).delay--;
  }

  // check PLI queue
  _hasReceivedPLI = false;
  while (!_signalPLI.empty() && _signalPLI.front().delay == 0) {
    // PLI message has arrived at sender side
    _hasReceivedPLI = true;
    _signalPLI.pop_front();
  }
  // decrement PLI queue times
  for (std::list<fbSignal>::iterator it = _signalPLI.begin();
       it != _signalPLI.end(); it++) {
    (*it).delay--;
  }

  if (_hasReceivedPLI) {
    // respond to PLI by encoding a key frame
    frame_types[0] = VideoFrameType::kVideoFrameKey;
    _hasReceivedPLI = false;
    _hasReceivedSLI = false;  // don't trigger both at once
  }

  int ret = _encoder->Encode(inputVideoBuffer, &frame_types);

  if (_encodeCompleteTime > 0) {
    _totalEncodeTime +=
        _encodeCompleteTime - _encodeTimes[inputVideoBuffer.rtp_timestamp()];
  } else {
    _totalEncodeTime += tGetTime() - _encodeTimes[inputVideoBuffer.rtp_timestamp()];
  }
  assert(ret >= 0);
  return false;
}

webrtc::CodecSpecificInfo* Benchmark::CopyCodecSpecificInfo(
    const webrtc::CodecSpecificInfo* codecSpecificInfo) const {
  webrtc::CodecSpecificInfo* info = new webrtc::CodecSpecificInfo;
  *info = *codecSpecificInfo;
  return info;
}

void Benchmark::Setup() {
  // Use _sourceFile as a check to prevent multiple Setup() calls.
  if (_sourceFile != NULL) {
    return;
  }

  std::stringstream ss;
  std::string strTestNo;
  ss << "0";
  ss >> strTestNo;

  // Check if settings exist. Otherwise use defaults.
  if (_outname == "") {
    _outname =
        webrtc::test::OutputPath() + "out_normaltest" + strTestNo + ".yuv";
  }

  if (_codecName == "") {
    _codecName =
        webrtc::test::OutputPath() + "encoded_normaltest" + strTestNo + ".yuv";
  }

  if ((_sourceFile = fopen(_inname.c_str(), "rb")) == NULL) {
    printf("Cannot read file %s.\n", _inname.c_str());
    exit(1);
  }

  if ((_encodedFile = fopen(_codecName.c_str(), "wb")) == NULL) {
    printf("Cannot write encoded file.\n");
    exit(1);
  }

  char mode[3] = "wb";
  if (_appendNext) {
    strncpy(mode, "ab", 3);
  }

  // if ((_decodedFile = fopen(_outname.c_str(), mode)) == NULL) {
  //   printf("Cannot write file %s.\n", _outname.c_str());
  //   exit(1);
  // }

  _appendNext = true;
}
