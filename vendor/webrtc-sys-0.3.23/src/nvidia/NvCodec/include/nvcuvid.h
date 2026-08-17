/*
 * This copyright notice applies to this header file only:
 *
 * Copyright (c) 2010-2022 NVIDIA Corporation
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the software, and to permit persons to whom the
 * software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/********************************************************************************************************************/
//! \file nvcuvid.h
//!   NVDECODE API provides video decoding interface to NVIDIA GPU devices.
//! \date 2015-2022
//!  This file contains the interface constants, structure definitions and function prototypes.
/********************************************************************************************************************/

#if !defined(__NVCUVID_H__)
#define __NVCUVID_H__

#include "cuviddec.h"

#if defined(__cplusplus)
extern "C" {
#endif /* __cplusplus */

#define MAX_CLOCK_TS 3

/***********************************************/
//!
//! High-level helper APIs for video sources
//!
/***********************************************/

typedef void *CUvideosource;
typedef void *CUvideoparser;
typedef long long CUvideotimestamp;


/************************************************************************/
//! \enum cudaVideoState
//! Video source state enums
//! Used in cuvidSetVideoSourceState and cuvidGetVideoSourceState APIs
/************************************************************************/
typedef enum {
    cudaVideoState_Error   = -1,    /**< Error state (invalid source)                  */
    cudaVideoState_Stopped = 0,     /**< Source is stopped (or reached end-of-stream)  */
    cudaVideoState_Started = 1      /**< Source is running and delivering data         */
} cudaVideoState;

/************************************************************************/
//! \enum cudaAudioCodec
//! Audio compression enums
//! Used in CUAUDIOFORMAT structure
/************************************************************************/
typedef enum {
    cudaAudioCodec_MPEG1=0,         /**< MPEG-1 Audio               */
    cudaAudioCodec_MPEG2,           /**< MPEG-2 Audio               */
    cudaAudioCodec_MP3,             /**< MPEG-1 Layer III Audio     */
    cudaAudioCodec_AC3,             /**< Dolby Digital (AC3) Audio  */
    cudaAudioCodec_LPCM,            /**< PCM Audio                  */
    cudaAudioCodec_AAC,             /**< AAC Audio                  */
} cudaAudioCodec;

/************************************************************************/
//! \ingroup STRUCTS
//! \struct HEVCTIMECODESET
//! Used to store Time code extracted from Time code SEI in HEVC codec
/************************************************************************/
typedef struct _HEVCTIMECODESET
{
    unsigned int time_offset_value;
    unsigned short n_frames;                 
    unsigned char clock_timestamp_flag;
    unsigned char units_field_based_flag;
    unsigned char counting_type;
    unsigned char full_timestamp_flag;
    unsigned char discontinuity_flag;
    unsigned char cnt_dropped_flag;
    unsigned char seconds_value;
    unsigned char minutes_value;
    unsigned char hours_value;
    unsigned char seconds_flag;
    unsigned char minutes_flag;
    unsigned char hours_flag;
    unsigned char time_offset_length;
    unsigned char reserved;
} HEVCTIMECODESET;

/************************************************************************/
//! \ingroup STRUCTS
//! \struct HEVCSEITIMECODE
//! Used to extract Time code SEI in HEVC codec
/************************************************************************/
typedef struct _HEVCSEITIMECODE
{
    HEVCTIMECODESET time_code_set[MAX_CLOCK_TS];
    unsigned char num_clock_ts;
} HEVCSEITIMECODE;

/**********************************************************************************/
//! \ingroup STRUCTS
//! \struct CUSEIMESSAGE;
//! Used in CUVIDSEIMESSAGEINFO structure
/**********************************************************************************/
typedef struct _CUSEIMESSAGE
{
    unsigned char sei_message_type; /**< OUT: SEI Message Type      */
    unsigned char reserved[3];
    unsigned int sei_message_size;  /**< OUT: SEI Message Size      */
} CUSEIMESSAGE;

/************************************************************************************************/
//! \ingroup STRUCTS
//! \struct CUVIDEOFORMAT
//! Video format
//! Used in cuvidGetSourceVideoFormat API
/************************************************************************************************/
typedef struct
{
    cudaVideoCodec codec;                   /**< OUT: Compression format          */
   /**
    * OUT: frame rate = numerator / denominator (for example: 30000/1001)
    */
    struct {
        /**< OUT: frame rate numerator   (0 = unspecified or variable frame rate) */
        unsigned int numerator;
        /**< OUT: frame rate denominator (0 = unspecified or variable frame rate) */
        unsigned int denominator;
    } frame_rate;
    unsigned char progressive_sequence;     /**< OUT: 0=interlaced, 1=progressive                                      */
    unsigned char bit_depth_luma_minus8;    /**< OUT: high bit depth luma. E.g, 2 for 10-bitdepth, 4 for 12-bitdepth   */
    unsigned char bit_depth_chroma_minus8;  /**< OUT: high bit depth chroma. E.g, 2 for 10-bitdepth, 4 for 12-bitdepth */
    unsigned char min_num_decode_surfaces;  /**< OUT: Minimum number of decode surfaces to be allocated for correct
                                                      decoding. The client can send this value in ulNumDecodeSurfaces
                                                      (in CUVIDDECODECREATEINFO structure).
                                                      This guarantees correct functionality and optimal video memory
                                                      usage but not necessarily the best performance, which depends on
                                                      the design of the overall application. The optimal number of
                                                      decode surfaces (in terms of performance and memory utilization)
                                                      should be decided by experimentation for each application, but it
                                                      cannot go below min_num_decode_surfaces.
                                                      If this value is used for ulNumDecodeSurfaces then it must be
                                                      returned to parser during sequence callback.                     */
    unsigned int coded_width;               /**< OUT: coded frame width in pixels                                      */
    unsigned int coded_height;              /**< OUT: coded frame height in pixels                                     */
   /**
    * area of the frame that should be displayed
    * typical example:
    * coded_width = 1920, coded_height = 1088
    * display_area = { 0,0,1920,1080 }
    */
    struct {
        int left;                           /**< OUT: left position of display rect    */
        int top;                            /**< OUT: top position of display rect     */
        int right;                          /**< OUT: right position of display rect   */
        int bottom;                         /**< OUT: bottom position of display rect  */
    } display_area;
    cudaVideoChromaFormat chroma_format;    /**< OUT:  Chroma format                   */
    unsigned int bitrate;                   /**< OUT: video bitrate (bps, 0=unknown)   */
   /**
    * OUT: Display Aspect Ratio = x:y (4:3, 16:9, etc)
    */
    struct {
        int x;
        int y;
    } display_aspect_ratio;
    /**
    * Video Signal Description
    * Refer section E.2.1 (VUI parameters semantics) of H264 spec file
    */
    struct {
        unsigned char video_format          : 3; /**< OUT: 0-Component, 1-PAL, 2-NTSC, 3-SECAM, 4-MAC, 5-Unspecified     */
        unsigned char video_full_range_flag : 1; /**< OUT: indicates the black level and luma and chroma range           */
        unsigned char reserved_zero_bits    : 4; /**< Reserved bits                                                      */
        unsigned char color_primaries;           /**< OUT: chromaticity coordinates of source primaries                  */
        unsigned char transfer_characteristics;  /**< OUT: opto-electronic transfer characteristic of the source picture */
        unsigned char matrix_coefficients;       /**< OUT: used in deriving luma and chroma signals from RGB primaries   */
    } video_signal_description;
    unsigned int seqhdr_data_length;             /**< OUT: Additional bytes following (CUVIDEOFORMATEX)                  */
} CUVIDEOFORMAT;

/****************************************************************/
//! \ingroup STRUCTS
//! \struct CUVIDOPERATINGPOINTINFO
//! Operating point information of scalable bitstream
/****************************************************************/
typedef struct 
{
    cudaVideoCodec codec;
    union 
    {
        struct
        {
            unsigned char  operating_points_cnt;
            unsigned char  reserved24_bits[3];
            unsigned short operating_points_idc[32];
        } av1;
        unsigned char CodecReserved[1024];
    };
} CUVIDOPERATINGPOINTINFO;

/**********************************************************************************/
//! \ingroup STRUCTS
//! \struct CUVIDSEIMESSAGEINFO
//! Used in cuvidParseVideoData API with PFNVIDSEIMSGCALLBACK pfnGetSEIMsg
/**********************************************************************************/
typedef struct _CUVIDSEIMESSAGEINFO
{
    void *pSEIData;                 /**< OUT: SEI Message Data      */
    CUSEIMESSAGE *pSEIMessage;      /**< OUT: SEI Message Info      */
    unsigned int sei_message_count; /**< OUT: SEI Message Count     */
    unsigned int picIdx;            /**< OUT: SEI Message Pic Index */
} CUVIDSEIMESSAGEINFO;

/****************************************************************/
//! \ingroup STRUCTS
//! \struct CUVIDAV1SEQHDR
//! AV1 specific sequence header information
/****************************************************************/
typedef struct {
    unsigned int max_width;
    unsigned int max_height;
    unsigned char reserved[1016];
} CUVIDAV1SEQHDR;

/****************************************************************/
//! \ingroup STRUCTS
//! \struct CUVIDEOFORMATEX
//! Video format including raw sequence header information
//! Used in cuvidGetSourceVideoFormat API
/****************************************************************/
typedef struct
{
    CUVIDEOFORMAT format;                 /**< OUT: CUVIDEOFORMAT structure */
    union {
        CUVIDAV1SEQHDR av1;
        unsigned char raw_seqhdr_data[1024];  /**< OUT: Sequence header data    */
    };
} CUVIDEOFORMATEX;

/****************************************************************/
//! \ingroup STRUCTS
//! \struct CUAUDIOFORMAT
//! Audio formats
//! Used in cuvidGetSourceAudioFormat API
/****************************************************************/
typedef struct
{
    cudaAudioCodec codec;       /**< OUT: Compression format                                              */
    unsigned int channels;      /**< OUT: number of audio channels                                        */
    unsigned int samplespersec; /**< OUT: sampling frequency                                              */
    unsigned int bitrate;       /**< OUT: For uncompressed, can also be used to determine bits per sample */
    unsigned int reserved1;     /**< Reserved for future use                                              */
    unsigned int reserved2;     /**< Reserved for future use                                              */
} CUAUDIOFORMAT;


/***************************************************************/
//! \enum CUvideopacketflags
//! Data packet flags
//! Used in CUVIDSOURCEDATAPACKET structure
/***************************************************************/
typedef enum {
    CUVID_PKT_ENDOFSTREAM   = 0x01,   /**< Set when this is the last packet for this stream                              */
    CUVID_PKT_TIMESTAMP     = 0x02,   /**< Timestamp is valid                                                            */
    CUVID_PKT_DISCONTINUITY = 0x04,   /**< Set when a discontinuity has to be signalled                                  */
    CUVID_PKT_ENDOFPICTURE  = 0x08,   /**< Set when the packet contains exactly one frame or one field                   */
    CUVID_PKT_NOTIFY_EOS    = 0x10,   /**< If this flag is set along with CUVID_PKT_ENDOFSTREAM, an additional (dummy)
                                           display callback will be invoked with null value of CUVIDPARSERDISPINFO which
                                           should be interpreted as end of the stream.                                   */
} CUvideopacketflags;

/*****************************************************************************/
//! \ingroup STRUCTS
//! \struct CUVIDSOURCEDATAPACKET
//! Data Packet
//! Used in cuvidParseVideoData API
//! IN for cuvidParseVideoData
/*****************************************************************************/
typedef struct _CUVIDSOURCEDATAPACKET
{
    unsigned long flags;            /**< IN: Combination of CUVID_PKT_XXX flags                              */
    unsigned long payload_size;     /**< IN: number of bytes in the payload (may be zero if EOS flag is set) */
    const unsigned char *payload;   /**< IN: Pointer to packet payload data (may be NULL if EOS flag is set) */
    CUvideotimestamp timestamp;     /**< IN: Presentation time stamp (10MHz clock), only valid if
                                             CUVID_PKT_TIMESTAMP flag is set                                 */
} CUVIDSOURCEDATAPACKET;

// Callback for packet delivery
typedef int (CUDAAPI *PFNVIDSOURCECALLBACK)(void *, CUVIDSOURCEDATAPACKET *);

/**************************************************************************************************************************/
//! \ingroup STRUCTS
//! \struct CUVIDSOURCEPARAMS
//! Describes parameters needed in cuvidCreateVideoSource API
//! NVDECODE API is intended for HW accelerated video decoding so CUvideosource doesn't have audio demuxer for all supported
//! containers. It's recommended to clients to use their own or third party demuxer if audio support is needed.
/**************************************************************************************************************************/
typedef struct _CUVIDSOURCEPARAMS
{
    unsigned int ulClockRate;                   /**< IN: Time stamp units in Hz (0=default=10000000Hz)      */
    unsigned int bAnnexb : 1;                   /**< IN: AV1 annexB stream                                  */
    unsigned int uReserved : 31;                /**< Reserved for future use - set to zero                  */
    unsigned int uReserved1[6];                 /**< Reserved for future use - set to zero                  */
    void *pUserData;                            /**< IN: User private data passed in to the data handlers   */
    PFNVIDSOURCECALLBACK pfnVideoDataHandler;   /**< IN: Called to deliver video packets                    */
    PFNVIDSOURCECALLBACK pfnAudioDataHandler;   /**< IN: Called to deliver audio packets.                   */
    void *pvReserved2[8];                       /**< Reserved for future use - set to NULL                  */
} CUVIDSOURCEPARAMS;


/**********************************************/
//! \ingroup ENUMS
//! \enum CUvideosourceformat_flags
//! CUvideosourceformat_flags
//! Used in cuvidGetSourceVideoFormat API
/**********************************************/
typedef enum {
    CUVID_FMT_EXTFORMATINFO = 0x100             /**< Return extended format structure (CUVIDEOFORMATEX) */
} CUvideosourceformat_flags;

#if !defined(__APPLE__)
/***************************************************************************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidCreateVideoSource(CUvideosource *pObj, const char *pszFileName, CUVIDSOURCEPARAMS *pParams)
//! Create CUvideosource object. CUvideosource spawns demultiplexer thread that provides two callbacks: 
//! pfnVideoDataHandler() and pfnAudioDataHandler()
//! NVDECODE API is intended for HW accelerated video decoding so CUvideosource doesn't have audio demuxer for all supported 
//! containers. It's recommended to clients to use their own or third party demuxer if audio support is needed.
/***************************************************************************************************************************/
CUresult CUDAAPI cuvidCreateVideoSource(CUvideosource *pObj, const char *pszFileName, CUVIDSOURCEPARAMS *pParams);

/***************************************************************************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidCreateVideoSourceW(CUvideosource *pObj, const wchar_t *pwszFileName, CUVIDSOURCEPARAMS *pParams)
//! Create video source
/***************************************************************************************************************************/
CUresult CUDAAPI cuvidCreateVideoSourceW(CUvideosource *pObj, const wchar_t *pwszFileName, CUVIDSOURCEPARAMS *pParams);

/********************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidDestroyVideoSource(CUvideosource obj)
//! Destroy video source
/********************************************************************/
CUresult CUDAAPI cuvidDestroyVideoSource(CUvideosource obj);

/******************************************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidSetVideoSourceState(CUvideosource obj, cudaVideoState state)
//! Set video source state to:
//! cudaVideoState_Started - to signal the source to run and deliver data
//! cudaVideoState_Stopped - to stop the source from delivering the data
//! cudaVideoState_Error   - invalid source
/******************************************************************************************/
CUresult CUDAAPI cuvidSetVideoSourceState(CUvideosource obj, cudaVideoState state);

/******************************************************************************************/
//! \ingroup FUNCTS
//! \fn cudaVideoState CUDAAPI cuvidGetVideoSourceState(CUvideosource obj)
//! Get video source state
//! Returns:
//! cudaVideoState_Started - if Source is running and delivering data
//! cudaVideoState_Stopped - if Source is stopped or reached end-of-stream
//! cudaVideoState_Error   - if Source is in error state
/******************************************************************************************/
cudaVideoState CUDAAPI cuvidGetVideoSourceState(CUvideosource obj);

/******************************************************************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidGetSourceVideoFormat(CUvideosource obj, CUVIDEOFORMAT *pvidfmt, unsigned int flags)
//! Gets video source format in pvidfmt, flags is set to combination of CUvideosourceformat_flags as per requirement
/******************************************************************************************************************/
CUresult CUDAAPI cuvidGetSourceVideoFormat(CUvideosource obj, CUVIDEOFORMAT *pvidfmt, unsigned int flags);

/**************************************************************************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidGetSourceAudioFormat(CUvideosource obj, CUAUDIOFORMAT *paudfmt, unsigned int flags)
//! Get audio source format
//! NVDECODE API is intended for HW accelerated video decoding so CUvideosource doesn't have audio demuxer for all supported 
//! containers. It's recommended to clients to use their own or third party demuxer if audio support is needed.
/**************************************************************************************************************************/
CUresult CUDAAPI cuvidGetSourceAudioFormat(CUvideosource obj, CUAUDIOFORMAT *paudfmt, unsigned int flags);

#endif
/**********************************************************************************/
//! \ingroup STRUCTS
//! \struct CUVIDPARSERDISPINFO
//! Used in cuvidParseVideoData API with PFNVIDDISPLAYCALLBACK pfnDisplayPicture
/**********************************************************************************/
typedef struct _CUVIDPARSERDISPINFO
{
    int picture_index;          /**< OUT: Index of the current picture                                                         */
    int progressive_frame;      /**< OUT: 1 if progressive frame; 0 otherwise                                                  */
    int top_field_first;        /**< OUT: 1 if top field is displayed first; 0 otherwise                                       */
    int repeat_first_field;     /**< OUT: Number of additional fields (1=ivtc, 2=frame doubling, 4=frame tripling, 
                                     -1=unpaired field)                                                                        */
    CUvideotimestamp timestamp; /**< OUT: Presentation time stamp                                                              */
} CUVIDPARSERDISPINFO;

/***********************************************************************************************************************/
//! Parser callbacks
//! The parser will call these synchronously from within cuvidParseVideoData(), whenever there is sequence change or a picture
//! is ready to be decoded and/or displayed. First argument in functions is "void *pUserData" member of structure CUVIDSOURCEPARAMS
//! Return values from these callbacks are interpreted as below. If the callbacks return failure, it will be propagated by
//! cuvidParseVideoData() to the application.
//! Parser picks default operating point as 0 and outputAllLayers flag as 0 if PFNVIDOPPOINTCALLBACK is not set or return value is 
//! -1 or invalid operating point.
//! PFNVIDSEQUENCECALLBACK : 0: fail, 1: succeeded, > 1: override dpb size of parser (set by CUVIDPARSERPARAMS::ulMaxNumDecodeSurfaces
//! while creating parser)
//! PFNVIDDECODECALLBACK   : 0: fail, >=1: succeeded
//! PFNVIDDISPLAYCALLBACK  : 0: fail, >=1: succeeded
//! PFNVIDOPPOINTCALLBACK  : <0: fail, >=0: succeeded (bit 0-9: OperatingPoint, bit 10-10: outputAllLayers, bit 11-30: reserved)
//! PFNVIDSEIMSGCALLBACK   : 0: fail, >=1: succeeded
/***********************************************************************************************************************/
typedef int (CUDAAPI *PFNVIDSEQUENCECALLBACK)(void *, CUVIDEOFORMAT *);
typedef int (CUDAAPI *PFNVIDDECODECALLBACK)(void *, CUVIDPICPARAMS *);
typedef int (CUDAAPI *PFNVIDDISPLAYCALLBACK)(void *, CUVIDPARSERDISPINFO *);
typedef int (CUDAAPI *PFNVIDOPPOINTCALLBACK)(void *, CUVIDOPERATINGPOINTINFO*);
typedef int (CUDAAPI *PFNVIDSEIMSGCALLBACK) (void *, CUVIDSEIMESSAGEINFO *);

/**************************************/
//! \ingroup STRUCTS
//! \struct CUVIDPARSERPARAMS
//! Used in cuvidCreateVideoParser API
/**************************************/
typedef struct _CUVIDPARSERPARAMS
{
    cudaVideoCodec CodecType;                   /**< IN: cudaVideoCodec_XXX                                                  */
    unsigned int ulMaxNumDecodeSurfaces;        /**< IN: Max # of decode surfaces (parser will cycle through these)          */
    unsigned int ulClockRate;                   /**< IN: Timestamp units in Hz (0=default=10000000Hz)                        */
    unsigned int ulErrorThreshold;              /**< IN: % Error threshold (0-100) for calling pfnDecodePicture (100=always 
                                                     IN: call pfnDecodePicture even if picture bitstream is fully corrupted) */
    unsigned int ulMaxDisplayDelay;             /**< IN: Max display queue delay (improves pipelining of decode with display)
                                                         0=no delay (recommended values: 2..4)                               */
    unsigned int bAnnexb : 1;                   /**< IN: AV1 annexB stream                                                   */
    unsigned int uReserved : 31;                /**< Reserved for future use - set to zero                                   */
    unsigned int uReserved1[4];                 /**< IN: Reserved for future use - set to 0                                  */
    void *pUserData;                            /**< IN: User data for callbacks                                             */
    PFNVIDSEQUENCECALLBACK pfnSequenceCallback; /**< IN: Called before decoding frames and/or whenever there is a fmt change */
    PFNVIDDECODECALLBACK pfnDecodePicture;      /**< IN: Called when a picture is ready to be decoded (decode order)         */
    PFNVIDDISPLAYCALLBACK pfnDisplayPicture;    /**< IN: Called whenever a picture is ready to be displayed (display order)  */
    PFNVIDOPPOINTCALLBACK pfnGetOperatingPoint; /**< IN: Called from AV1 sequence header to get operating point of a AV1 
                                                         scalable bitstream                                                  */
    PFNVIDSEIMSGCALLBACK pfnGetSEIMsg;          /**< IN: Called when all SEI messages are parsed for particular frame        */
    void *pvReserved2[5];                       /**< Reserved for future use - set to NULL                                   */
    CUVIDEOFORMATEX *pExtVideoInfo;             /**< IN: [Optional] sequence header data from system layer                   */
} CUVIDPARSERPARAMS;

/************************************************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidCreateVideoParser(CUvideoparser *pObj, CUVIDPARSERPARAMS *pParams)
//! Create video parser object and initialize
/************************************************************************************************/
CUresult CUDAAPI cuvidCreateVideoParser(CUvideoparser *pObj, CUVIDPARSERPARAMS *pParams);

/************************************************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidParseVideoData(CUvideoparser obj, CUVIDSOURCEDATAPACKET *pPacket)
//! Parse the video data from source data packet in pPacket 
//! Extracts parameter sets like SPS, PPS, bitstream etc. from pPacket and 
//! calls back pfnDecodePicture with CUVIDPICPARAMS data for kicking of HW decoding
//! calls back pfnSequenceCallback with CUVIDEOFORMAT data for initial sequence header or when
//! the decoder encounters a video format change
//! calls back pfnDisplayPicture with CUVIDPARSERDISPINFO data to display a video frame
/************************************************************************************************/
CUresult CUDAAPI cuvidParseVideoData(CUvideoparser obj, CUVIDSOURCEDATAPACKET *pPacket);

/************************************************************************************************/
//! \ingroup FUNCTS
//! \fn CUresult CUDAAPI cuvidDestroyVideoParser(CUvideoparser obj)
//! Destroy the video parser
/************************************************************************************************/
CUresult CUDAAPI cuvidDestroyVideoParser(CUvideoparser obj);

/**********************************************************************************************/

#if defined(__cplusplus)
}
#endif /* __cplusplus */

#endif // __NVCUVID_H__


