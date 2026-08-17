; /*
   *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
   *
   *  Use of this source code is governed by a BSD-style license
   *  that can be found in the LICENSE file in the root of the source
   *  tree. An additional intellectual property rights grant can be found
   *  in the file PATENTS.  All contributing project authors may
   *  be found in the AUTHORS file in the root of the source tree.
   */

#ifndef WEBRTC_MODULES_VIDEO_CODING_CODECS_TEST_FRAWEWORK_BENCHMARK_H_
#define WEBRTC_MODULES_VIDEO_CODING_CODECS_TEST_FRAWEWORK_BENCHMARK_H_

#include <cstdlib>
#include <fstream>
#include <list>
#include <queue>
#include <string>

#include "cpu/cpu_linux.h"
#include "api/environment/environment_factory.h"
#include "modules/include/module_common_types.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "rtc_base/synchronization/mutex.h"
#include "system_wrappers/include/clock.h"


class VideoSource;
class Benchmark;

// feedback signal to encoder
struct fbSignal {
  fbSignal(int d, uint8_t pid) : delay(d), id(pid) {};
  int delay;
  uint8_t id;
};

class FrameQueueTuple {
 public:
  FrameQueueTuple(webrtc::VideoFrame* frame,
                  const webrtc::CodecSpecificInfo* codecSpecificInfo = NULL)
      : _frame(frame), _codecSpecificInfo(codecSpecificInfo) {};
  ~FrameQueueTuple();
  webrtc::VideoFrame* _frame;
  const webrtc::CodecSpecificInfo* _codecSpecificInfo;
};

class FrameQueue {
 public:
  FrameQueue() {}

  ~FrameQueue() {}

  void PushFrame(webrtc::VideoFrame* frame,
                 webrtc::CodecSpecificInfo* codecSpecificInfo = NULL);
  FrameQueueTuple* PopFrame();
  bool Empty();

 private:
  webrtc::Mutex _queueRWLock;
  std::queue<FrameQueueTuple*> _frameBufferQueue;
};

class VideoEncodeCompleteCallback : public webrtc::EncodedImageCallback {
 public:
  VideoEncodeCompleteCallback(FILE* encodedFile,
                              FrameQueue* frameQueue,
                              Benchmark& test)
      : _encodedFile(encodedFile),
        _frameQueue(frameQueue),
        _test(test),
        _encodedBytes(0) {}

  webrtc::EncodedImageCallback::Result OnEncodedImage(
      const webrtc::EncodedImage& encoded_image,
      const webrtc::CodecSpecificInfo* codec_specific_info) override;

  uint32_t EncodedBytes();

 private:
  FILE* _encodedFile;
  FrameQueue* _frameQueue;
  Benchmark& _test;
  uint32_t _encodedBytes;
};

class Benchmark {
 public:
  friend class VideoEncodeCompleteCallback;

 public:
  Benchmark();
  virtual void Perform();
  virtual bool IsSupported() = 0;

 protected:
  Benchmark(std::string name, std::string description);
  Benchmark(std::string name,
            std::string description,
            std::string resultsFileName,
            std::string codecName);
  virtual webrtc::VideoEncoder* GetNewEncoder(webrtc::Environment &env) = 0;
  virtual void PerformNormalTest();
  virtual void CodecSpecific_InitBitrate();
  static const char* GetMagicStr() { return "#!benchmark1.0"; }

  double ActualBitRate(int nFrames) {
    return 8.0 * _sumEncBytes / (nFrames / _inst.maxFramerate);
  }

  webrtc::CodecSpecificInfo* CopyCodecSpecificInfo(
      const webrtc::CodecSpecificInfo* codecSpecificInfo) const;

  bool Encode();

  void Setup();

  void Teardown();

  void CodecSettings(int width,
                     int height,
                     uint32_t frameRate /*=30*/,
                     uint32_t bitRate /*=0*/) {
    if (bitRate > 0) {
      _bitRate = bitRate;
    } else if (_bitRate == 0) {
      _bitRate = 600;
    }
    _inst.codecType = webrtc::kVideoCodecH264;
    _inst.maxFramerate = (unsigned char)frameRate;
    _inst.minBitrate = (unsigned char)frameRate;
    _inst.startBitrate = (int)_bitRate;
    _inst.maxBitrate = 8000;
    _inst.width = width;
    _inst.height = height;
    _inst.numberOfSimulcastStreams = 1;
    _inst.simulcastStream[0].width = width;
    _inst.simulcastStream[0].height = height;
    _inst.simulcastStream[0].maxBitrate = 8000;
    _inst.simulcastStream[0].minBitrate = _bitRate;
    _inst.simulcastStream[0].targetBitrate = _bitRate;
    _inst.simulcastStream[0].maxFramerate = frameRate;
    _inst.simulcastStream[0].active = true;
    _inst.SetScalabilityMode(webrtc::ScalabilityMode::kL1T1);
    _inst.mode = webrtc::VideoCodecMode::kRealtimeVideo;
    _inst.qpMax = 56;
    _inst.SetFrameDropEnabled(true);
  }

  double tGetTime() {
    // return time in sec
    return ((double)(webrtc::Clock::GetRealTimeClock()->TimeInMilliseconds()) /
            1000);
  }

  virtual webrtc::CodecSpecificInfo* CreateEncoderSpecificInfo() const {
    return NULL;
  };

  void UpdateEncodedBytes(int encodedBytes) { _sumEncBytes += encodedBytes; }

  const VideoSource* _target;
  std::string _resultsFileName;
  std::ofstream _results;
  std::string _name;
  std::string _description;
  std::string _codecName;
  std::string _inname;
  std::string _outname;
  webrtc::VideoEncoder* _encoder;
  //webrtc::VideoDecoder* _decoder;
  uint32_t _bitRate;
  bool _appendNext = false;
  int _framecnt;
  int _encFrameCnt;
  double _totalEncodeTime;
  double _totalDecodeTime;
  double _decodeCompleteTime;
  double _encodeCompleteTime;
  double _totalEncodePipeTime;
  double _totalDecodePipeTime;
  webrtc::VideoCodec _inst;
  int _sumEncBytes;

  unsigned int _lengthSourceFrame = 0;
  unsigned char* _sourceBuffer = nullptr;

  FILE* _encodedFile = nullptr;
  unsigned int _lengthEncFrame = 0;
  FrameQueueTuple* _frameToDecode = nullptr;

  FILE* _sourceFile = nullptr;
  FILE* _decodedFile = nullptr;

  bool _hasReceivedPLI = false;
  bool _waitForKey = false;
  std::map<uint32_t, double> _encodeTimes;
  std::map<uint32_t, double> _decodeTimes;

  bool _missingFrames = false;
  std::list<fbSignal> _signalSLI;
  int _rttFrames = 0;
  mutable bool _hasReceivedSLI = false;
  mutable bool _hasReceivedRPSI = false;
  uint8_t _pictureIdSLI = 0;
  uint16_t _pictureIdRPSI = 0;
  uint64_t _lastDecRefPictureId = 0;
  uint64_t _lastDecPictureId = 0;
  std::list<fbSignal> _signalPLI;
  webrtc::CpuWrapper* _cpu;
  webrtc::Environment _env;
};

#endif  // WEBRTC_MODULES_VIDEO_CODING_CODECS_TEST_FRAWEWORK_BENCHMARK_H_
