/*!
 * \copy
 *     Copyright (c)  2013, Cisco Systems
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
 */

//wels_const.h
#ifndef WELS_CONST_H__
#define WELS_CONST_H__

#include "as264_common.h" //  to communicate with specific macros there, 3/18/2010
#include "codec_app_def.h"
#include "wels_const_common.h"

/* To control number of spatial, quality and temporal layers constraint by application layer? */
#define NUM_SPATIAL_LAYERS_CONSTRAINT
#define NUM_QUALITY_LAYERS_CONSTRAINT


#define STATISTICS_LOG_INTERVAL_MS (5000) // output statistics log every 5s

#define INTRA_4x4_MODE_NUM              8
#define MB_LUMA_CHROMA_BLOCK4x4_NUM     24

#define MAX_PPS_COUNT_LIMITED           57// limit the max ID of PPS because of known limitation of receiver endpoints
#define MAX_PPS_COUNT                   (MAX_PPS_COUNT_LIMITED)//in Standard is 256     // Count number of PPS

#define PARA_SET_TYPE                   3 // SPS+PPS
#define PARA_SET_TYPE_AVCSPS            0
#define PARA_SET_TYPE_SUBSETSPS         1
#define PARA_SET_TYPE_PPS               2

#define MAX_VERTICAL_MV_RANGE           1024  //TODO, for allocate enough memory for transpose
#define MAX_FRAME_RATE                  60      // maximal frame rate to support
#define MIN_FRAME_RATE                  1       // minimal frame rate need support

#define MAX_BIT_RATE                    INT_MAX // maximal bit rate to support
//TODO {Sijia}: 30fps*MaxCPB in level5.1 = 30*240000*1000bits = 7 200 000 000, larger than INT_MAX which is 2147483647, but this is also very big and abnormal number, should figure out a reasonable number after discussion
#define MIN_BIT_RATE                    1       // minimal bit rate need support

#define SVC_QUALITY_BASE_QP             26
#define MAX_SLICEGROUP_IDS              8       // Count number of SSlice Groups
#define MAX_THREADS_NUM                 4       // assume to support up to 4 logical cores(threads)

#define INTPEL_NEEDED_MARGIN            (3)  // for safe sub-pel MC

#define I420_PLANES                     3

#define COMPRESS_RATIO_THR (1.0f)       //set to size of the original data, which will be large enough considering MinCR

#if !defined(SSEI_BUFFER_SIZE)
#define SSEI_BUFFER_SIZE        128
#endif//SSEI_BUFFER_SIZE

#if !defined(SPS_BUFFER_SIZE)
#define SPS_BUFFER_SIZE         32
#endif//SPS_BUFFER_SIZE

#if !defined(PPS_BUFFER_SIZE)
#define PPS_BUFFER_SIZE         16
#endif//PPS_BUFFER_SIZE

#if !defined(MAX_MACROBLOCK_SIZE_IN_BYTE)
#define MAX_MACROBLOCK_SIZE_IN_BYTE     400 //3200/8, 3200 is from Annex A.3.1.(n)
#endif

#define MAX_MACROBLOCK_SIZE_IN_BYTE_x2 (MAX_MACROBLOCK_SIZE_IN_BYTE<<1)

#if defined(NUM_SPATIAL_LAYERS_CONSTRAINT)
#define MAX_DEPENDENCY_LAYER            MAX_SPATIAL_LAYER_NUM   // Maximal dependency layer
#else
#define MAX_DEPENDENCY_LAYER            8       // Maximal dependency layer
#endif//NUM_SPATIAL_LAYERS_CONSTRAINT

//The max temporal level support is equal or less than MAX_TEMPORAL_LAYER_NUM defined @ codec_app_def.h
#define MAX_TEMPORAL_LEVEL              MAX_TEMPORAL_LAYER_NUM  // Maximal temporal level

#if defined(NUM_QUALITY_LAYERS_CONSTRAINT)
#define MAX_QUALITY_LEVEL               MAX_QUALITY_LAYER_NUM           // Maximal quality level
#else
#define MAX_QUALITY_LEVEL               16      // Maximal quality level
#endif//NUM_QUALITY_LAYERS_CONSTRAINT

#if defined(MAX_GOP_SIZE)
#undef MAX_GOP_SIZE
#endif//MAX_GOP_SIZE
#define MAX_GOP_SIZE    (1<<(MAX_TEMPORAL_LEVEL-1))

#define MAX_SHORT_REF_COUNT             (MAX_GOP_SIZE>>1) // 16 in standard, maximal count number of short reference pictures
#define LONG_TERM_REF_NUM               2
#define LONG_TERM_REF_NUM_SCREEN        4
#define MAX_REF_PIC_COUNT               16 // 32 in standard, maximal Short + Long reference pictures
#define MIN_REF_PIC_COUNT               1               // minimal count number of reference pictures, 1 short + 2 key reference based?
#define MAX_MULTI_REF_PIC_COUNT         1       //maximum multi-reference number
//#define TOTAL_REF_MINUS_HALF_GOP      1       // last t0 in last gop
#define MAX_MMCO_COUNT                  66

// adjusted numbers reference picture functionality related definition
#define MAX_REFERENCE_MMCO_COUNT_NUM            4       // adjusted MAX_MMCO_COUNT(66 in standard) definition per encoder design
#define MAX_REFERENCE_REORDER_COUNT_NUM         2       // adjusted MAX_REF_PIC_COUNT(32 in standard) for reference reordering definition per encoder design
#define MAX_REFERENCE_PICTURE_COUNT_NUM_CAMERA          (MAX_SHORT_REF_COUNT+LONG_TERM_REF_NUM) // <= MAX_REF_PIC_COUNT, memory saved if <
#define MAX_REFERENCE_PICTURE_COUNT_NUM_SCREEN          (MAX_SHORT_REF_COUNT+LONG_TERM_REF_NUM_SCREEN)  // <= MAX_REF_PIC_COUNT, memory saved if <

#define BASE_DEPENDENCY_ID              0
#define MAX_DQ_LAYER_NUM                (MAX_DEPENDENCY_LAYER/**MAX_QUALITY_LEVEL*/)

#define INVALID_ID                      (-1)

#define NAL_HEADER_ADD_0X30BYTES        20

#define SLICE_NUM_EXPAND_COEF           2

enum {
BLOCK_16x16    = 0,
BLOCK_16x8     = 1,
BLOCK_8x16     = 2,
BLOCK_8x8      = 3,
BLOCK_4x4      = 4,
BLOCK_8x4      = 5,
BLOCK_4x8      = 6,
BLOCK_SIZE_ALL = 7
};

typedef enum {
RECIEVE_UNKOWN = 0,
RECIEVE_SUCCESS = 1,
RECIEVE_FAILED = 2
} LTR_MARKING_RECEIVE_STATE;

enum {
  CUR_AU_IDX    = 0,                    // index symbol for current access unit
  SUC_AU_IDX    = 1                     // index symbol for successive access unit
};

enum {
  ENC_RETURN_SUCCESS = 0,
  ENC_RETURN_MEMALLOCERR = 0x01, //will free memory and uninit
  ENC_RETURN_UNSUPPORTED_PARA = 0x02, //unsupported setting
  ENC_RETURN_UNEXPECTED = 0x04, //unexpected value
  ENC_RETURN_CORRECTED = 0x08, //unexpected value but corrected by encoder
  ENC_RETURN_INVALIDINPUT = 0x10, //invalid input
  ENC_RETURN_MEMOVERFLOWFOUND = 0x20,
  ENC_RETURN_VLCOVERFLOWFOUND = 0x40,
  ENC_RETURN_KNOWN_ISSUE = 0x80
};
//TODO: need to complete the return checking in encoder and fill in more types if needed

#endif//WELS_CONST_H__
