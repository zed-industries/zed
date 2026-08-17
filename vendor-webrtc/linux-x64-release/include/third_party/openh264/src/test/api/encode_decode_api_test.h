#include <gtest/gtest.h>
#include "codec_def.h"
#include "utils/BufferedData.h"
#include "utils/FileInputStream.h"
#include "BaseDecoderTest.h"
#include "BaseEncoderTest.h"
#include "wels_common_defs.h"
#include <string>
#include <vector>
using namespace WelsCommon;

//TODO: some content here in this file is the same with encode_decode_api_test.cpp
//plan to combine them after some on-going code reviews in that file to avoid merging conflict

#define TRY_TIME_RANGE           (10)
#define ENCODE_FRAME_NUM         (30)
#define LEVEL_ID_RANGE           (18)
#define MAX_WIDTH                (4096)
#define MAX_HEIGHT               (2304)
#define MAX_FRAME_RATE           (30)
#define MIN_FRAME_RATE           (1)
#define FRAME_RATE_RANGE         (2*MAX_FRAME_RATE)
#define RC_MODE_RANGE            (4)
#define BIT_RATE_RANGE           (10000)
#define MAX_QP                   (51)
#define MIN_QP                   (0)
#define QP_RANGE                 (2*MAX_QP)
#define SPATIAL_LAYER_NUM_RANGE  (2*MAX_SPATIAL_LAYER_NUM)
#define TEMPORAL_LAYER_NUM_RANGE (2*MAX_TEMPORAL_LAYER_NUM)
#define SAVED_NALUNIT_NUM        ( (MAX_SPATIAL_LAYER_NUM*MAX_QUALITY_LAYER_NUM) + 1 + MAX_SPATIAL_LAYER_NUM )
#define MAX_SLICES_NUM           ( ( MAX_NAL_UNITS_IN_LAYER - SAVED_NALUNIT_NUM ) / 3 )
#define SLICE_MODE_NUM           (SM_RESERVED)
#define LOOP_FILTER_IDC_NUM      (3)
#define LOOF_FILTER_OFFSET_RANGE (6)
#define MAX_REF_PIC_COUNT        (16)
#define MIN_REF_PIC_COUNT        (1)
#define LONG_TERM_REF_NUM        (2)
#define LONG_TERM_REF_NUM_SCREEN (4)
#define MAX_REFERENCE_PICTURE_COUNT_NUM_CAMERA (6)
#define MAX_REFERENCE_PICTURE_COUNT_NUM_SCREEN (8)
#define VALID_SIZE(iSize) (((iSize)>16)?(iSize):16)
#define GET_MB_WIDTH(x) (((x) + 15)/16)

typedef struct SLost_Sim {
  WelsCommon::EWelsNalUnitType eNalType;
  bool isLost;
} SLostSim;


struct EncodeDecodeFileParamBase {
  int numframes;
  int width;
  int height;
  float frameRate;
  int slicenum;
  bool bLostPara;
  const char* pLossSequence;
};

static void welsStderrTraceOrigin (void* ctx, int level, const char* string) {
  fprintf (stderr, "%s\n", string);
}

typedef struct STrace_Unit {
  int iTarLevel;
} STraceUnit;

class EncodeDecodeTestBase : public BaseEncoderTest, public BaseDecoderTest {
 public:
  uint8_t iRandValue;
 public:
  virtual void SetUp() {
    BaseEncoderTest::SetUp();
    BaseDecoderTest::SetUp();
    pFunc = welsStderrTraceOrigin;
    pTraceInfo = NULL;
    encoder_->SetOption (ENCODER_OPTION_TRACE_CALLBACK, &pFunc);
    encoder_->SetOption (ENCODER_OPTION_TRACE_CALLBACK_CONTEXT, &pTraceInfo);
    decoder_->SetOption (DECODER_OPTION_TRACE_CALLBACK, &pFunc);
    decoder_->SetOption (DECODER_OPTION_TRACE_CALLBACK_CONTEXT, &pTraceInfo);
  }

  virtual void TearDown() {
    BaseEncoderTest::TearDown();
    BaseDecoderTest::TearDown();
  }

  virtual void prepareParam (int iLayers, int iSlices, int width, int height, float framerate, SEncParamExt* pParam);

  virtual bool prepareEncDecParam (const EncodeDecodeFileParamBase EncDecFileParam);

  virtual void encToDecData (const SFrameBSInfo& info, int& len);

  virtual void encToDecSliceData (const int iLayerNum, const int iSliceNum, const SFrameBSInfo& info, int& len);

  virtual int GetRandWidth() {
    return WelsClip3 ((((rand() % MAX_WIDTH) >> 1) + 1) << 1, 16, MAX_WIDTH);
  }

  virtual int GetRandHeight() {
    return WelsClip3 ((((rand() % MAX_HEIGHT) >> 1) + 1) << 1, 16, MAX_HEIGHT);
  }

 protected:
  SEncParamExt   param_;
  BufferedData   buf_;
  SSourcePicture EncPic;
  SFrameBSInfo   info;
  SBufferInfo    dstBufInfo_;
  std::vector<SLostSim> m_SLostSim;
  WelsTraceCallback pFunc;
  STraceUnit sTrace;
  STraceUnit* pTraceInfo;
};

class EncodeDecodeTestAPIBase : public EncodeDecodeTestBase {
 public:
  uint8_t iRandValue;
 public:
  void SetUp() {
    EncodeDecodeTestBase::SetUp();
  }

  void TearDown() {
    EncodeDecodeTestBase::TearDown();
  }

  void prepareParam0 (int iLayers, int iSlices, int width, int height, float framerate, SEncParamExt* pParam);

  void prepareParamDefault (int iLayers, int iSlices, int width, int height, float framerate, SEncParamExt* pParam);

  bool InitialEncDec (int iWidth, int iHeight);
  void RandomParamExtCombination();
  void ValidateParamExtCombination();
  void SliceParamValidationForMode2 (int iSpatialIdx);
  void SliceParamValidationForMode3 (int iSpatialIdx);
  void SliceParamValidationForMode4();

  void EncodeOneFrame (int iCheckTypeIndex);
  bool EncDecOneFrame (const int iWidth, const int iHeight, const int iFrame, FILE* pfEnc);
  bool TestOneSimulcastAVC (SEncParamExt* pParam, ISVCDecoder** decoder, unsigned char** pBsBuf, int iSpatialLayerNum,
                            int iEncFrameNum,
                            int iCallTimes);
};

class EncodeDecodeTestAPI : public ::testing::TestWithParam<EncodeDecodeFileParamBase>, public EncodeDecodeTestAPIBase {
  void SetUp() {
    EncodeDecodeTestAPIBase::SetUp();
  }

  void TearDown() {
    EncodeDecodeTestAPIBase::TearDown();
  }
};


bool ToRemainDidNal (const unsigned char* pSrc, EWelsNalUnitType eNalType, int iTarDid);
void ExtractDidNal (SFrameBSInfo* pBsInfo, int& iSrcLen, std::vector<SLostSim>* p_SLostSim, int iTarDid);
int SimulateNALLoss (const unsigned char* pSrc,  int& iSrcLen, std::vector<SLostSim>* p_SLostSim,
                     const char* pLossChars, bool bLossPara, int& iLossIdx, bool& bVCLLoss);

long IsKeyFrameLost (ISVCDecoder* pDecoder, SLTRRecoverRequest* p_LTR_Recover_Request, long hr);
bool IsLTRMarking (ISVCDecoder* pDecoder);
void LTRRecoveryRequest (ISVCDecoder* pDecoder, ISVCEncoder* pEncoder, SLTRRecoverRequest* p_LTR_Recover_Request,
                         long hr, bool m_P2PmodeFlag);
void LTRMarkFeedback (ISVCDecoder* pDecoder, ISVCEncoder* pEncoder, SLTRMarkingFeedback* p_LTR_Marking_Feedback,
                      long hr);
