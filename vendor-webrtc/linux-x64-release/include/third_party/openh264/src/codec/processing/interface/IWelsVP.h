/*!
 * \copy
 *     Copyright (c)  2004-2013, Cisco Systems
 *     All rights reserved.
 *
 *     Redistribution and use in source and binary forms, with or without
 *     modification, are permitted provided that the following conditions
 *     are met:
 *
 *        * Redistributions of source code must retain the above copyright
 *          notice, this list of conditions and the following disclaimer.
 *
 *        * Redistributions in binary form must reproduce the above copyright
 *          notice, this list of conditions and the following disclaimer in
 *          the documentation and/or other materials provided with the
 *          distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *     "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *     LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *     COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 *     INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *     BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 *     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 *     LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 *     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 *     POSSIBILITY OF SUCH DAMAGE.
 *
 *
 * \file        :  IWelsVP.h
 *
 * \brief       :  Interface of wels video processor class
 *
 * \date        :  2011/01/04
 *
 * \description :  1. should support both C/C++ style interface
 *                 2. should concern with the feature extension requirement
 *                 3. should care the usage of "char"==>
 *                     1) value char  : signed char/unsigned char
 *                     2) string char : char
 *
 *************************************************************************************
 */

#ifndef IWELSVP_H_
#define IWELSVP_H_

#define WELSVP_MAJOR_VERSION   1
#define WELSVP_MINOR_VERSION   1
#define WELSVP_VERSION         ((WELSVP_MAJOR_VERSION << 8) + WELSVP_MINOR_VERSION)

typedef enum {
  RET_SUCCESS          =  0,
  RET_FAILED           = -1,
  RET_INVALIDPARAM     = -2,
  RET_OUTOFMEMORY      = -3,
  RET_NOTSUPPORTED     = -4,
  RET_UNEXPECTED       = -5,
  RET_NEEDREINIT       = -6
} EResult;

typedef enum {
  VIDEO_FORMAT_NULL       = 0,   /* invalid format   */
  /*rgb color formats*/
  VIDEO_FORMAT_RGB        = 1,   /* rgb 24bits       */
  VIDEO_FORMAT_RGBA       = 2,   /* rgba             */
  VIDEO_FORMAT_RGB555     = 3,   /* rgb555           */
  VIDEO_FORMAT_RGB565     = 4,   /* rgb565           */
  VIDEO_FORMAT_BGR        = 5,   /* bgr 24bits       */
  VIDEO_FORMAT_BGRA       = 6,   /* bgr 32bits       */
  VIDEO_FORMAT_ABGR       = 7,   /* abgr             */
  VIDEO_FORMAT_ARGB       = 8,   /* argb             */

  /*yuv color formats*/
  VIDEO_FORMAT_YUY2       = 20,   /* yuy2             */
  VIDEO_FORMAT_YVYU       = 21,   /* yvyu             */
  VIDEO_FORMAT_UYVY       = 22,   /* uyvy             */
  VIDEO_FORMAT_I420       = 23,   /* yuv 4:2:0 planar */
  VIDEO_FORMAT_YV12       = 24,   /* yuv 4:2:0 planar */
  VIDEO_FORMAT_INTERNAL   = 25,   /* Only Used for SVC decoder testbed */
  VIDEO_FORMAT_NV12       = 26,   /* y planar + uv packed */
  VIDEO_FORMAT_I422       = 27,   /* yuv 4:2:2 planar */
  VIDEO_FORMAT_I444       = 28,   /* yuv 4:4:4 planar */
  VIDEO_FORMAT_YUYV       = 20,   /* yuv 4:2:2 packed */

  VIDEO_FORMAT_RGB24      = 1,
  VIDEO_FORMAT_RGB32      = 2,
  VIDEO_FORMAT_RGB24_INV  = 5,
  VIDEO_FORMAT_RGB32_INV  = 6,
  VIDEO_FORMAT_RGB555_INV = 7,
  VIDEO_FORMAT_RGB565_INV = 8,
  VIDEO_FORMAT_YUV2       = 21,
  VIDEO_FORMAT_420        = 23,

  VIDEO_FORMAT_VFlip      = 0x80000000
} EVideoFormat;

typedef enum {
  BUFFER_HOSTMEM  = 0,
  BUFFER_SURFACE
} EPixMapBufferProperty;

typedef struct {
  int iRectTop;
  int iRectLeft;
  int iRectWidth;
  int iRectHeight;
} SRect;

typedef struct {
  void*        pPixel[3];
  int          iSizeInBits;
  int          iStride[3];
  SRect        sRect;
  EVideoFormat eFormat;
  EPixMapBufferProperty eProperty;//not use? to remove? but how about the size of SPixMap?
} SPixMap;

typedef enum {
  METHOD_NULL              = 0,
  METHOD_COLORSPACE_CONVERT    ,//not support yet
  METHOD_DENOISE              ,
  METHOD_SCENE_CHANGE_DETECTION_VIDEO ,
  METHOD_SCENE_CHANGE_DETECTION_SCREEN ,
  METHOD_DOWNSAMPLE            ,
  METHOD_VAA_STATISTICS        ,
  METHOD_BACKGROUND_DETECTION  ,
  METHOD_ADAPTIVE_QUANT ,
  METHOD_COMPLEXITY_ANALYSIS   ,
  METHOD_COMPLEXITY_ANALYSIS_SCREEN,
  METHOD_IMAGE_ROTATE          ,
  METHOD_SCROLL_DETECTION,
  METHOD_MASK
} EMethods;

//-----------------------------------------------------------------//
//  Algorithm parameters define
//-----------------------------------------------------------------//

typedef enum {
  SIMILAR_SCENE,   //similar scene
  MEDIUM_CHANGED_SCENE,   //medium changed scene
  LARGE_CHANGED_SCENE     //large changed scene
} ESceneChangeIdc;

typedef enum {
  NO_STATIC,  // motion block
  COLLOCATED_STATIC, // collocated static block
  SCROLLED_STATIC,  // scrolled static block
  BLOCK_STATIC_IDC_ALL
} EStaticBlockIdc;

typedef struct {
  SRect sMaskRect;
  bool bMaskInfoAvailable;
  int iScrollMvX;
  int iScrollMvY;
  bool bScrollDetectFlag; // 0:false ; 1:ltr; 2: scene change
} SScrollDetectionParam;

typedef struct {
  ESceneChangeIdc eSceneChangeIdc; // SIMILAR_SCENE, MEDIUM_CHANGED_SCENE, LARGE_CHANGED_SCENE
  int             iMotionBlockNum; // Number of motion blocks
  long long       iFrameComplexity; // frame complexity
  unsigned char* pStaticBlockIdc;   // static block idc
  SScrollDetectionParam sScrollResult; //results from scroll detection
} SSceneChangeResult;

typedef struct {
  unsigned char* pCurY;             // Y data of current frame
  unsigned char* pRefY;             // Y data of pRef frame for diff calc
  int (*pSad8x8)[4];                // sad of 8x8, every 4 in the same 16x16 get together
  int* pSsd16x16;                   // sum of square difference of 16x16
  int* pSum16x16;                   // sum of 16x16
  int* pSumOfSquare16x16;           // sum of square of 16x16
  int   (*pSumOfDiff8x8)[4];
  unsigned char (*pMad8x8)[4];
  int iFrameSad;                    // sad of frame
} SVAACalcResult;

typedef struct {
  int iCalcVar;
  int iCalcBgd;
  int iCalcSsd;
  int iReserved;
  SVAACalcResult*  pCalcResult;
} SVAACalcParam;

typedef struct {
  signed char*     pBackgroundMbFlag;
  SVAACalcResult*  pCalcRes;
} SBGDInterface;

typedef enum {
  AQ_QUALITY_MODE,   //Quality mode
  AQ_BITRATE_MODE    //Bitrate mode
} EAQModes;

typedef struct {
  unsigned short    uiMotionIndex;
  unsigned short    uiTextureIndex;
} SMotionTextureUnit;

typedef struct {
  int                  iAdaptiveQuantMode; // 0:quality mode, 1:bitrates mode
  SVAACalcResult*      pCalcResult;
  SMotionTextureUnit*  pMotionTextureUnit;

  signed char*      pMotionTextureIndexToDeltaQp;
  int               iAverMotionTextureIndexToDeltaQp; // *AQ_STEP_INT_MULTIPLY
} SAdaptiveQuantizationParam;

typedef enum {
  FRAME_SAD     =  0,
  GOM_SAD       = -1,
  GOM_VAR       = -2
} EComplexityAnalysisMode;

typedef struct {
  int  iComplexityAnalysisMode;
  int  iCalcBgd;
  int  iMbNumInGom;
  long long  iFrameComplexity;
  int*  pGomComplexity;
  int*  pGomForegroundBlockNum;
  signed char*  pBackgroundMbFlag;
  unsigned int* uiRefMbType;
  SVAACalcResult*  pCalcResult;
} SComplexityAnalysisParam;

typedef struct {
  int  iMbRowInGom;
  int*  pGomComplexity;
  int  iGomNumInFrame;
  long long  iFrameComplexity; //255*255(MaxMbSAD)*36864(MaxFS) make the highest bit of 32-bit integer 1
  int  iIdrFlag;
  SScrollDetectionParam sScrollResult;
} SComplexityAnalysisScreenParam;
/////////////////////////////////////////////////////////////////////////////////////////////

typedef struct {
  void*    pCtx;
  EResult (*Init) (void* pCtx, int iType, void* pCfg);
  EResult (*Uninit) (void* pCtx, int iType);
  EResult (*Flush) (void* pCtx, int iType);
  EResult (*Process) (void* pCtx, int iType, SPixMap* pSrc, SPixMap* dst);
  EResult (*Get) (void* pCtx, int iType, void* pParam);
  EResult (*Set) (void* pCtx, int iType, void* pParam);
  EResult (*SpecialFeature) (void* pCtx, int iType, void* pIn, void* pOut);
} IWelsVPc;

#if defined(__cplusplus) && !defined(CINTERFACE)  /* C++ style interface */

class IWelsVP {
 public:
  virtual ~IWelsVP() {}

 public:
  virtual EResult Init (int iType, void* pCfg) = 0;
  virtual EResult Uninit (int iType) = 0;
  virtual EResult Flush (int iType) = 0;
  virtual EResult Process (int iType, SPixMap* pSrc, SPixMap* dst) = 0;
  virtual EResult Get (int iType, void* pParam) = 0;
  virtual EResult Set (int iType, void* pParam) = 0;
  virtual EResult SpecialFeature (int iType, void* pIn, void* pOut) = 0;
};

/* Recommend to invoke the interface via the micro for convenient */
#define IWelsVPFunc_Init(p, a, b)                  (p)->Init(a, b)
#define IWelsVPFunc_Uninit(p, a)                   (p)->Uninit(a)
#define IWelsVPFunc_Flush(p, a)                    (p)->Flush(a)
#define IWelsVPFunc_Process(p, a, b, c)            (p)->Process(a, b, c)
#define IWelsVPFunc_Get(p, a, b)                   (p)->Get(a, b)
#define IWelsVPFunc_Set(p, a, b)                   (p)->Set(a, b)
#define IWelsVPFunc_SpecialFeature(p, a, b, c)     (p)->SpecialFeature(a, b, c)

/* C++ interface version */
#define WELSVP_INTERFACE_VERION                    (0x8000 + (WELSVP_VERSION & 0x7fff))
#define WELSVP_EXTERNC_BEGIN                       extern "C" {
#define WELSVP_EXTERNC_END                         }

#else    /* C style interface */

/* Recommend to invoke the interface via the micro for convenient */
#define IWelsVPFunc_Init(p, a, b)                  (p)->Init(p->h, a, b)
#define IWelsVPFunc_Uninit(p, a)                   (p)->Uninit(p->h, a)
#define IWelsVPFunc_Flush(p, a)                    (p)->Flush(p->h, a)
#define IWelsVPFunc_Process(p, a, b, c)            (p)->Process(p->h, a, b, c)
#define IWelsVPFunc_Get(p, a, b)                   (p)->Get(p->h, a, b)
#define IWelsVPFunc_Set(p, a, b)                   (p)->Set(p->h, a, b)
#define IWelsVPFunc_SpecialFeature(p, a, b, c)     (p)->SpecialFeature(p->h, a, b, c)

/* C interface version */
#define WELSVP_INTERFACE_VERION                    (0x0001 + (WELSVP_VERSION & 0x7fff))
#define WELSVP_EXTERNC_BEGIN
#define WELSVP_EXTERNC_END

#endif

WELSVP_EXTERNC_BEGIN
EResult WelsCreateVpInterface (void** ppCtx, int iVersion /*= WELSVP_INTERFACE_VERION*/);
EResult WelsDestroyVpInterface (void* pCtx , int iVersion /*= WELSVP_INTERFACE_VERION*/);
WELSVP_EXTERNC_END

//////////////////////////////////////////////////////////////////////////////////////////////
#endif // IWELSVP_H_


