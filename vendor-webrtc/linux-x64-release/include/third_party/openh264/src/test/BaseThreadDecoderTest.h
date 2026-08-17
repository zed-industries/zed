#ifndef __BASETHREADDECODERTEST_H__
#define __BASETHREADDECODERTEST_H__

#include "test_stdint.h"
#include <limits.h>
#include <fstream>
#include "codec_api.h"

#include "utils/BufferedData.h"

class BaseThreadDecoderTest {
 public:
  struct Plane {
    const uint8_t* data;
    int width;
    int height;
    int stride;
  };

  struct Frame {
    Plane y;
    Plane u;
    Plane v;
  };

  typedef enum tagDecodeStatus {
    OpenFile,
    Decoding,
    EndOfStream,
    End
  } eDecodeStatus;

  struct Callback {
    virtual void onDecodeFrame (const Frame& frame) = 0;
  };

  BaseThreadDecoderTest();
  int32_t SetUp();
  void TearDown();
  bool ThreadDecodeFile (const char* fileName, Callback* cbk);

  bool Open (const char* fileName);
  ISVCDecoder* decoder_;

 private:
  void DecodeFrame (const uint8_t* src, size_t sliceSize, Callback* cbk);
  void FlushFrame (Callback* cbk);

  std::ifstream file_;
  SBufferInfo sBufInfo;
  uint8_t* pData[3];
  uint64_t uiTimeStamp;
  FILE* pYuvFile;
  bool bEnableYuvDumpTest;
  eDecodeStatus decodeStatus_;
};

#endif //__BASETHREADDECODERTEST_H__
