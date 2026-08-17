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

/*****************************************************************************************************/
//! \file cuviddec.h
//! NVDECODE API provides video decoding interface to NVIDIA GPU devices.
//! This file contains constants, structure definitions and function prototypes used for decoding.
/*****************************************************************************************************/

#if !defined(__CUDA_VIDEO_H__)
#define __CUDA_VIDEO_H__

#ifndef __cuda_cuda_h__
#include <cuda.h>
#endif // __cuda_cuda_h__

#if defined(_WIN64) || defined(__LP64__) || defined(__x86_64) || defined(AMD64) || defined(_M_AMD64)
#if (CUDA_VERSION >= 3020) && (!defined(CUDA_FORCE_API_VERSION) || (CUDA_FORCE_API_VERSION >= 3020))
#define __CUVID_DEVPTR64
#endif
#endif

#if defined(__cplusplus)
extern "C" {
#endif /* __cplusplus */

typedef void *CUvideodecoder;
typedef struct _CUcontextlock_st *CUvideoctxlock;

/*********************************************************************************/
//! \enum cudaVideoCodec
//! Video codec enums
//! These enums are used in CUVIDDECODECREATEINFO and CUVIDDECODECAPS structures
/*********************************************************************************/
typedef enum cudaVideoCodec_enum {
    cudaVideoCodec_MPEG1=0,                                         /**<  MPEG1             */
    cudaVideoCodec_MPEG2,                                           /**<  MPEG2             */
    cudaVideoCodec_MPEG4,                                           /**<  MPEG4             */
    cudaVideoCodec_VC1,                                             /**<  VC1               */
    cudaVideoCodec_H264,                                            /**<  H264              */
    cudaVideoCodec_JPEG,                                            /**<  JPEG              */
    cudaVideoCodec_H264_SVC,                                        /**<  H264-SVC          */
    cudaVideoCodec_H264_MVC,                                        /**<  H264-MVC          */
    cudaVideoCodec_HEVC,                                            /**<  HEVC              */
    cudaVideoCodec_VP8,                                             /**<  VP8               */
    cudaVideoCodec_VP9,                                             /**<  VP9               */
    cudaVideoCodec_AV1,                                             /**<  AV1               */
    cudaVideoCodec_NumCodecs,                                       /**<  Max codecs        */
    // Uncompressed YUV
    cudaVideoCodec_YUV420 = (('I'<<24)|('Y'<<16)|('U'<<8)|('V')),   /**< Y,U,V (4:2:0)      */
    cudaVideoCodec_YV12   = (('Y'<<24)|('V'<<16)|('1'<<8)|('2')),   /**< Y,V,U (4:2:0)      */
    cudaVideoCodec_NV12   = (('N'<<24)|('V'<<16)|('1'<<8)|('2')),   /**< Y,UV  (4:2:0)      */
    cudaVideoCodec_YUYV   = (('Y'<<24)|('U'<<16)|('Y'<<8)|('V')),   /**< YUYV/YUY2 (4:2:2)  */
    cudaVideoCodec_UYVY   = (('U'<<24)|('Y'<<16)|('V'<<8)|('Y'))    /**< UYVY (4:2:2)       */
} cudaVideoCodec;

/*********************************************************************************/
//! \enum cudaVideoSurfaceFormat
//! Video surface format enums used for output format of decoded output
//! These enums are used in CUVIDDECODECREATEINFO structure
/*********************************************************************************/
typedef enum cudaVideoSurfaceFormat_enum {
    cudaVideoSurfaceFormat_NV12=0,          /**< Semi-Planar YUV [Y plane followed by interleaved UV plane]     */
    cudaVideoSurfaceFormat_P016=1,          /**< 16 bit Semi-Planar YUV [Y plane followed by interleaved UV plane].
                                                 Can be used for 10 bit(6LSB bits 0), 12 bit (4LSB bits 0)      */
    cudaVideoSurfaceFormat_YUV444=2,        /**< Planar YUV [Y plane followed by U and V planes]                */
    cudaVideoSurfaceFormat_YUV444_16Bit=3,  /**< 16 bit Planar YUV [Y plane followed by U and V planes]. 
                                                 Can be used for 10 bit(6LSB bits 0), 12 bit (4LSB bits 0)      */
} cudaVideoSurfaceFormat;

/******************************************************************************************************************/
//! \enum cudaVideoDeinterlaceMode
//! Deinterlacing mode enums
//! These enums are used in CUVIDDECODECREATEINFO structure
//! Use cudaVideoDeinterlaceMode_Weave for progressive content and for content that doesn't need deinterlacing
//! cudaVideoDeinterlaceMode_Adaptive needs more video memory than other DImodes
/******************************************************************************************************************/
typedef enum cudaVideoDeinterlaceMode_enum {
    cudaVideoDeinterlaceMode_Weave=0,   /**< Weave both fields (no deinterlacing) */
    cudaVideoDeinterlaceMode_Bob,       /**< Drop one field                       */
    cudaVideoDeinterlaceMode_Adaptive   /**< Adaptive deinterlacing               */
} cudaVideoDeinterlaceMode;

/**************************************************************************************************************/
//! \enum cudaVideoChromaFormat
//! Chroma format enums
//! These enums are used in CUVIDDECODECREATEINFO and CUVIDDECODECAPS structures
/**************************************************************************************************************/
typedef enum cudaVideoChromaFormat_enum {
    cudaVideoChromaFormat_Monochrome=0,  /**< MonoChrome */
    cudaVideoChromaFormat_420,           /**< YUV 4:2:0  */
    cudaVideoChromaFormat_422,           /**< YUV 4:2:2  */
    cudaVideoChromaFormat_444            /**< YUV 4:4:4  */
} cudaVideoChromaFormat;

/*************************************************************************************************************/
//! \enum cudaVideoCreateFlags
//! Decoder flag enums to select preferred decode path
//! cudaVideoCreate_Default and cudaVideoCreate_PreferCUVID are most optimized, use these whenever possible
/*************************************************************************************************************/
typedef enum cudaVideoCreateFlags_enum {
    cudaVideoCreate_Default     = 0x00,     /**< Default operation mode: use dedicated video engines                        */
    cudaVideoCreate_PreferCUDA  = 0x01,     /**< Use CUDA-based decoder (requires valid vidLock object for multi-threading) */
    cudaVideoCreate_PreferDXVA  = 0x02,     /**< Go through DXVA internally if possible (requires D3D9 interop)             */
    cudaVideoCreate_PreferCUVID = 0x04      /**< Use dedicated video engines directly                                       */
} cudaVideoCreateFlags;


/*************************************************************************/
//! \enum cuvidDecodeStatus
//! Decode status enums
//! These enums are used in CUVIDGETDECODESTATUS structure
/*************************************************************************/
typedef enum cuvidDecodeStatus_enum
{
    cuvidDecodeStatus_Invalid         = 0,   // Decode status is not valid
    cuvidDecodeStatus_InProgress      = 1,   // Decode is in progress
    cuvidDecodeStatus_Success         = 2,   // Decode is completed without any errors
    // 3 to 7 enums are reserved for future use
    cuvidDecodeStatus_Error           = 8,   // Decode is completed with an error (error is not concealed)
    cuvidDecodeStatus_Error_Concealed = 9,   // Decode is completed with an error and error is concealed 
} cuvidDecodeStatus;

/**************************************************************************************************************/
//! \struct CUVIDDECODECAPS;
//! This structure is used in cuvidGetDecoderCaps API
/**************************************************************************************************************/
typedef struct _CUVIDDECODECAPS
{
    cudaVideoCodec          eCodecType;                 /**< IN: cudaVideoCodec_XXX                                             */
    cudaVideoChromaFormat   eChromaFormat;              /**< IN: cudaVideoChromaFormat_XXX                                      */
    unsigned int            nBitDepthMinus8;            /**< IN: The Value "BitDepth minus 8"                                   */
    unsigned int            reserved1[3];               /**< Reserved for future use - set to zero                              */

    unsigned char           bIsSupported;               /**< OUT: 1 if codec supported, 0 if not supported                      */
    unsigned char           nNumNVDECs;                 /**< OUT: Number of NVDECs that can support IN params                   */
    unsigned short          nOutputFormatMask;          /**< OUT: each bit represents corresponding cudaVideoSurfaceFormat enum */
    unsigned int            nMaxWidth;                  /**< OUT: Max supported coded width in pixels                           */
    unsigned int            nMaxHeight;                 /**< OUT: Max supported coded height in pixels                          */
    unsigned int            nMaxMBCount;                /**< OUT: Max supported macroblock count
                                                                  CodedWidth*CodedHeight/256 must be <= nMaxMBCount             */
    unsigned short          nMinWidth;                  /**< OUT: Min supported coded width in pixels                           */
    unsigned short          nMinHeight;                 /**< OUT: Min supported coded height in pixels                          */
    unsigned char           bIsHistogramSupported;      /**< OUT: 1 if Y component histogram output is supported, 0 if not
                                                                  Note: histogram is computed on original picture data before
                                                                  any post-processing like scaling, cropping, etc. is applied   */
    unsigned char           nCounterBitDepth;           /**< OUT: histogram counter bit depth                                   */
    unsigned short          nMaxHistogramBins;          /**< OUT: Max number of histogram bins                                  */
    unsigned int            reserved3[10];              /**< Reserved for future use - set to zero                              */
} CUVIDDECODECAPS;

/**************************************************************************************************************/
//! \struct CUVIDDECODECREATEINFO
//! This structure is used in cuvidCreateDecoder API
/**************************************************************************************************************/
typedef struct _CUVIDDECODECREATEINFO
{
    unsigned long ulWidth;              /**< IN: Coded sequence width in pixels                                             */
    unsigned long ulHeight;             /**< IN: Coded sequence height in pixels                                            */
    unsigned long ulNumDecodeSurfaces;  /**< IN: Maximum number of internal decode surfaces                                 */
    cudaVideoCodec CodecType;           /**< IN: cudaVideoCodec_XXX                                                         */
    cudaVideoChromaFormat ChromaFormat; /**< IN: cudaVideoChromaFormat_XXX                                                  */
    unsigned long ulCreationFlags;      /**< IN: Decoder creation flags (cudaVideoCreateFlags_XXX)                          */
    unsigned long bitDepthMinus8;       /**< IN: The value "BitDepth minus 8"                                               */
    unsigned long ulIntraDecodeOnly;    /**< IN: Set 1 only if video has all intra frames (default value is 0). This will
                                             optimize video memory for Intra frames only decoding. The support is limited
                                             to specific codecs - H264, HEVC, VP9, the flag will be ignored for codecs which
                                             are not supported. However decoding might fail if the flag is enabled in case
                                             of supported codecs for regular bit streams having P and/or B frames.          */
    unsigned long ulMaxWidth;           /**< IN: Coded sequence max width in pixels used with reconfigure Decoder           */
    unsigned long ulMaxHeight;          /**< IN: Coded sequence max height in pixels used with reconfigure Decoder          */                                           
    unsigned long Reserved1;            /**< Reserved for future use - set to zero                                          */
    /**
    * IN: area of the frame that should be displayed
    */
    struct {
        short left;
        short top;
        short right;
        short bottom;
    } display_area;

    cudaVideoSurfaceFormat OutputFormat;       /**< IN: cudaVideoSurfaceFormat_XXX                                     */
    cudaVideoDeinterlaceMode DeinterlaceMode;  /**< IN: cudaVideoDeinterlaceMode_XXX                                   */
    unsigned long ulTargetWidth;               /**< IN: Post-processed output width (Should be aligned to 2)           */
    unsigned long ulTargetHeight;              /**< IN: Post-processed output height (Should be aligned to 2)          */
    unsigned long ulNumOutputSurfaces;         /**< IN: Maximum number of output surfaces simultaneously mapped        */
    CUvideoctxlock vidLock;                    /**< IN: If non-NULL, context lock used for synchronizing ownership of 
                                                    the cuda context. Needed for cudaVideoCreate_PreferCUDA decode     */
    /**
    * IN: target rectangle in the output frame (for aspect ratio conversion)
    * if a null rectangle is specified, {0,0,ulTargetWidth,ulTargetHeight} will be used
    */
    struct {
        short left;
        short top;
        short right;
        short bottom;
    } target_rect;

    unsigned long enableHistogram;             /**< IN: enable histogram output, if supported */
    unsigned long Reserved2[4];                /**< Reserved for future use - set to zero */
} CUVIDDECODECREATEINFO;

/*********************************************************/
//! \struct CUVIDH264DPBENTRY
//! H.264 DPB entry
//! This structure is used in CUVIDH264PICPARAMS structure
/*********************************************************/
typedef struct _CUVIDH264DPBENTRY
{
    int PicIdx;                 /**< picture index of reference frame                                        */
    int FrameIdx;               /**< frame_num(short-term) or LongTermFrameIdx(long-term)                    */
    int is_long_term;           /**< 0=short term reference, 1=long term reference                           */
    int not_existing;           /**< non-existing reference frame (corresponding PicIdx should be set to -1) */
    int used_for_reference;     /**< 0=unused, 1=top_field, 2=bottom_field, 3=both_fields                    */
    int FieldOrderCnt[2];       /**< field order count of top and bottom fields                              */
} CUVIDH264DPBENTRY;

/************************************************************/
//! \struct CUVIDH264MVCEXT
//! H.264 MVC picture parameters ext
//! This structure is used in CUVIDH264PICPARAMS structure
/************************************************************/
typedef struct _CUVIDH264MVCEXT
{
    int num_views_minus1;                  /**< Max number of coded views minus 1 in video : Range - 0 to 1023              */
    int view_id;                           /**< view identifier                                                             */
    unsigned char inter_view_flag;         /**< 1 if used for inter-view prediction, 0 if not                               */
    unsigned char num_inter_view_refs_l0;  /**< number of inter-view ref pics in RefPicList0                                */
    unsigned char num_inter_view_refs_l1;  /**< number of inter-view ref pics in RefPicList1                                */
    unsigned char MVCReserved8Bits;        /**< Reserved bits                                                               */
    int InterViewRefsL0[16];               /**< view id of the i-th view component for inter-view prediction in RefPicList0 */
    int InterViewRefsL1[16];               /**< view id of the i-th view component for inter-view prediction in RefPicList1 */
} CUVIDH264MVCEXT;

/*********************************************************/
//! \struct CUVIDH264SVCEXT
//! H.264 SVC picture parameters ext
//! This structure is used in CUVIDH264PICPARAMS structure
/*********************************************************/
typedef struct _CUVIDH264SVCEXT
{
    unsigned char profile_idc;
    unsigned char level_idc;
    unsigned char DQId;
    unsigned char DQIdMax;
    unsigned char disable_inter_layer_deblocking_filter_idc;
    unsigned char ref_layer_chroma_phase_y_plus1;
    signed char   inter_layer_slice_alpha_c0_offset_div2;
    signed char   inter_layer_slice_beta_offset_div2;

    unsigned short DPBEntryValidFlag;
    unsigned char inter_layer_deblocking_filter_control_present_flag;
    unsigned char extended_spatial_scalability_idc;
    unsigned char adaptive_tcoeff_level_prediction_flag;
    unsigned char slice_header_restriction_flag;
    unsigned char chroma_phase_x_plus1_flag;
    unsigned char chroma_phase_y_plus1;

    unsigned char tcoeff_level_prediction_flag;
    unsigned char constrained_intra_resampling_flag;
    unsigned char ref_layer_chroma_phase_x_plus1_flag;
    unsigned char store_ref_base_pic_flag;
    unsigned char Reserved8BitsA;
    unsigned char Reserved8BitsB;

    short scaled_ref_layer_left_offset;
    short scaled_ref_layer_top_offset;
    short scaled_ref_layer_right_offset;
    short scaled_ref_layer_bottom_offset;
    unsigned short Reserved16Bits;
    struct _CUVIDPICPARAMS *pNextLayer; /**< Points to the picparams for the next layer to be decoded. 
                                             Linked list ends at the target layer. */
    int bRefBaseLayer;                  /**< whether to store ref base pic */
} CUVIDH264SVCEXT;

/******************************************************/
//! \struct CUVIDH264PICPARAMS
//! H.264 picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/******************************************************/
typedef struct _CUVIDH264PICPARAMS
{
    // SPS
    int log2_max_frame_num_minus4;
    int pic_order_cnt_type;
    int log2_max_pic_order_cnt_lsb_minus4;
    int delta_pic_order_always_zero_flag;
    int frame_mbs_only_flag;
    int direct_8x8_inference_flag;
    int num_ref_frames;             // NOTE: shall meet level 4.1 restrictions
    unsigned char residual_colour_transform_flag;
    unsigned char bit_depth_luma_minus8;    // Must be 0 (only 8-bit supported)
    unsigned char bit_depth_chroma_minus8;  // Must be 0 (only 8-bit supported)
    unsigned char qpprime_y_zero_transform_bypass_flag;
    // PPS
    int entropy_coding_mode_flag;
    int pic_order_present_flag;
    int num_ref_idx_l0_active_minus1;
    int num_ref_idx_l1_active_minus1;
    int weighted_pred_flag;
    int weighted_bipred_idc;
    int pic_init_qp_minus26;
    int deblocking_filter_control_present_flag;
    int redundant_pic_cnt_present_flag;
    int transform_8x8_mode_flag;
    int MbaffFrameFlag;
    int constrained_intra_pred_flag;
    int chroma_qp_index_offset;
    int second_chroma_qp_index_offset;
    int ref_pic_flag;
    int frame_num;
    int CurrFieldOrderCnt[2];
    // DPB
    CUVIDH264DPBENTRY dpb[16];          // List of reference frames within the DPB
    // Quantization Matrices (raster-order)
    unsigned char WeightScale4x4[6][16];
    unsigned char WeightScale8x8[2][64];
    // FMO/ASO
    unsigned char fmo_aso_enable;
    unsigned char num_slice_groups_minus1;
    unsigned char slice_group_map_type;
    signed char pic_init_qs_minus26;
    unsigned int slice_group_change_rate_minus1;
    union
    {
        unsigned long long slice_group_map_addr;
        const unsigned char *pMb2SliceGroupMap;
    } fmo;
    unsigned int  Reserved[12];
    // SVC/MVC
    union
    {
        CUVIDH264MVCEXT mvcext;
        CUVIDH264SVCEXT svcext;
    };
} CUVIDH264PICPARAMS;


/********************************************************/
//! \struct CUVIDMPEG2PICPARAMS
//! MPEG-2 picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/********************************************************/
typedef struct _CUVIDMPEG2PICPARAMS
{
    int ForwardRefIdx;          // Picture index of forward reference (P/B-frames)
    int BackwardRefIdx;         // Picture index of backward reference (B-frames)
    int picture_coding_type;
    int full_pel_forward_vector;
    int full_pel_backward_vector;
    int f_code[2][2];
    int intra_dc_precision;
    int frame_pred_frame_dct;
    int concealment_motion_vectors;
    int q_scale_type;
    int intra_vlc_format;
    int alternate_scan;
    int top_field_first;
    // Quantization matrices (raster order)
    unsigned char QuantMatrixIntra[64];
    unsigned char QuantMatrixInter[64];
} CUVIDMPEG2PICPARAMS;

// MPEG-4 has VOP types instead of Picture types
#define I_VOP 0
#define P_VOP 1
#define B_VOP 2
#define S_VOP 3

/*******************************************************/
//! \struct CUVIDMPEG4PICPARAMS
//! MPEG-4 picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/*******************************************************/
typedef struct _CUVIDMPEG4PICPARAMS
{
    int ForwardRefIdx;          // Picture index of forward reference (P/B-frames)
    int BackwardRefIdx;         // Picture index of backward reference (B-frames)
    // VOL
    int video_object_layer_width;
    int video_object_layer_height;
    int vop_time_increment_bitcount;
    int top_field_first;
    int resync_marker_disable;
    int quant_type;
    int quarter_sample;
    int short_video_header;
    int divx_flags;
    // VOP
    int vop_coding_type;
    int vop_coded;
    int vop_rounding_type;
    int alternate_vertical_scan_flag;
    int interlaced;
    int vop_fcode_forward;
    int vop_fcode_backward;
    int trd[2];
    int trb[2];
    // Quantization matrices (raster order)
    unsigned char QuantMatrixIntra[64];
    unsigned char QuantMatrixInter[64];
    int gmc_enabled;
} CUVIDMPEG4PICPARAMS;

/********************************************************/
//! \struct CUVIDVC1PICPARAMS
//! VC1 picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/********************************************************/
typedef struct _CUVIDVC1PICPARAMS
{
    int ForwardRefIdx;      /**< Picture index of forward reference (P/B-frames) */
    int BackwardRefIdx;     /**< Picture index of backward reference (B-frames)  */
    int FrameWidth;         /**< Actual frame width                              */
    int FrameHeight;        /**< Actual frame height                             */
    // PICTURE
    int intra_pic_flag;     /**< Set to 1 for I,BI frames */
    int ref_pic_flag;       /**< Set to 1 for I,P frames  */
    int progressive_fcm;    /**< Progressive frame        */
    // SEQUENCE
    int profile;
    int postprocflag;
    int pulldown;
    int interlace;
    int tfcntrflag;
    int finterpflag;
    int psf;
    int multires;
    int syncmarker;
    int rangered;
    int maxbframes;
    // ENTRYPOINT
    int panscan_flag;
    int refdist_flag;
    int extended_mv;
    int dquant;
    int vstransform;
    int loopfilter;
    int fastuvmc;
    int overlap;
    int quantizer;
    int extended_dmv;
    int range_mapy_flag;
    int range_mapy;
    int range_mapuv_flag;
    int range_mapuv;
    int rangeredfrm;    // range reduction state
} CUVIDVC1PICPARAMS;

/***********************************************************/
//! \struct CUVIDJPEGPICPARAMS
//! JPEG picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/***********************************************************/
typedef struct _CUVIDJPEGPICPARAMS
{
    int Reserved;
} CUVIDJPEGPICPARAMS;


/*******************************************************/
//! \struct CUVIDHEVCPICPARAMS
//! HEVC picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/*******************************************************/
typedef struct _CUVIDHEVCPICPARAMS
{
    // sps
    int pic_width_in_luma_samples;
    int pic_height_in_luma_samples;
    unsigned char log2_min_luma_coding_block_size_minus3;
    unsigned char log2_diff_max_min_luma_coding_block_size;
    unsigned char log2_min_transform_block_size_minus2;
    unsigned char log2_diff_max_min_transform_block_size;
    unsigned char pcm_enabled_flag;
    unsigned char log2_min_pcm_luma_coding_block_size_minus3;
    unsigned char log2_diff_max_min_pcm_luma_coding_block_size;
    unsigned char pcm_sample_bit_depth_luma_minus1;

    unsigned char pcm_sample_bit_depth_chroma_minus1;
    unsigned char pcm_loop_filter_disabled_flag;
    unsigned char strong_intra_smoothing_enabled_flag;
    unsigned char max_transform_hierarchy_depth_intra;
    unsigned char max_transform_hierarchy_depth_inter;
    unsigned char amp_enabled_flag;
    unsigned char separate_colour_plane_flag;
    unsigned char log2_max_pic_order_cnt_lsb_minus4;

    unsigned char num_short_term_ref_pic_sets;
    unsigned char long_term_ref_pics_present_flag;
    unsigned char num_long_term_ref_pics_sps;
    unsigned char sps_temporal_mvp_enabled_flag;
    unsigned char sample_adaptive_offset_enabled_flag;
    unsigned char scaling_list_enable_flag;
    unsigned char IrapPicFlag;
    unsigned char IdrPicFlag;

    unsigned char bit_depth_luma_minus8;
    unsigned char bit_depth_chroma_minus8;
    //sps/pps extension fields
    unsigned char log2_max_transform_skip_block_size_minus2;
    unsigned char log2_sao_offset_scale_luma;
    unsigned char log2_sao_offset_scale_chroma;
    unsigned char high_precision_offsets_enabled_flag;
    unsigned char reserved1[10];

    // pps
    unsigned char dependent_slice_segments_enabled_flag;
    unsigned char slice_segment_header_extension_present_flag;
    unsigned char sign_data_hiding_enabled_flag;
    unsigned char cu_qp_delta_enabled_flag;
    unsigned char diff_cu_qp_delta_depth;
    signed char init_qp_minus26;
    signed char pps_cb_qp_offset;
    signed char pps_cr_qp_offset;

    unsigned char constrained_intra_pred_flag;
    unsigned char weighted_pred_flag;
    unsigned char weighted_bipred_flag;
    unsigned char transform_skip_enabled_flag;
    unsigned char transquant_bypass_enabled_flag;
    unsigned char entropy_coding_sync_enabled_flag;
    unsigned char log2_parallel_merge_level_minus2;
    unsigned char num_extra_slice_header_bits;

    unsigned char loop_filter_across_tiles_enabled_flag;
    unsigned char loop_filter_across_slices_enabled_flag;
    unsigned char output_flag_present_flag;
    unsigned char num_ref_idx_l0_default_active_minus1;
    unsigned char num_ref_idx_l1_default_active_minus1;
    unsigned char lists_modification_present_flag;
    unsigned char cabac_init_present_flag;
    unsigned char pps_slice_chroma_qp_offsets_present_flag;

    unsigned char deblocking_filter_override_enabled_flag;
    unsigned char pps_deblocking_filter_disabled_flag;
    signed char   pps_beta_offset_div2;
    signed char   pps_tc_offset_div2;
    unsigned char tiles_enabled_flag;
    unsigned char uniform_spacing_flag;
    unsigned char num_tile_columns_minus1;
    unsigned char num_tile_rows_minus1;

    unsigned short column_width_minus1[21];
    unsigned short row_height_minus1[21];

    // sps and pps extension HEVC-main 444
    unsigned char sps_range_extension_flag;
    unsigned char transform_skip_rotation_enabled_flag;
    unsigned char transform_skip_context_enabled_flag;
    unsigned char implicit_rdpcm_enabled_flag;

    unsigned char explicit_rdpcm_enabled_flag;
    unsigned char extended_precision_processing_flag;
    unsigned char intra_smoothing_disabled_flag;
    unsigned char persistent_rice_adaptation_enabled_flag;

    unsigned char cabac_bypass_alignment_enabled_flag;
    unsigned char pps_range_extension_flag;
    unsigned char cross_component_prediction_enabled_flag;
    unsigned char chroma_qp_offset_list_enabled_flag;

    unsigned char diff_cu_chroma_qp_offset_depth;
    unsigned char chroma_qp_offset_list_len_minus1;
    signed char cb_qp_offset_list[6];

    signed char cr_qp_offset_list[6];
    unsigned char reserved2[2];

    unsigned int   reserved3[8];

    // RefPicSets
    int NumBitsForShortTermRPSInSlice;
    int NumDeltaPocsOfRefRpsIdx;
    int NumPocTotalCurr;
    int NumPocStCurrBefore;
    int NumPocStCurrAfter;
    int NumPocLtCurr;
    int CurrPicOrderCntVal;
    int RefPicIdx[16];                      // [refpic] Indices of valid reference pictures (-1 if unused for reference)
    int PicOrderCntVal[16];                 // [refpic]
    unsigned char IsLongTerm[16];           // [refpic] 0=not a long-term reference, 1=long-term reference
    unsigned char RefPicSetStCurrBefore[8]; // [0..NumPocStCurrBefore-1] -> refpic (0..15)
    unsigned char RefPicSetStCurrAfter[8];  // [0..NumPocStCurrAfter-1] -> refpic (0..15)
    unsigned char RefPicSetLtCurr[8];       // [0..NumPocLtCurr-1] -> refpic (0..15)
    unsigned char RefPicSetInterLayer0[8];
    unsigned char RefPicSetInterLayer1[8];
    unsigned int  reserved4[12];

    // scaling lists (diag order)
    unsigned char ScalingList4x4[6][16];       // [matrixId][i]
    unsigned char ScalingList8x8[6][64];       // [matrixId][i]
    unsigned char ScalingList16x16[6][64];     // [matrixId][i]
    unsigned char ScalingList32x32[2][64];     // [matrixId][i]
    unsigned char ScalingListDCCoeff16x16[6];  // [matrixId]
    unsigned char ScalingListDCCoeff32x32[2];  // [matrixId]
} CUVIDHEVCPICPARAMS;


/***********************************************************/
//! \struct CUVIDVP8PICPARAMS
//! VP8 picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/***********************************************************/
typedef struct _CUVIDVP8PICPARAMS
{
    int width;
    int height;
    unsigned int first_partition_size;
    //Frame Indexes
    unsigned char LastRefIdx;
    unsigned char GoldenRefIdx;
    unsigned char AltRefIdx;
    union {
        struct {
            unsigned char frame_type : 1;    /**< 0 = KEYFRAME, 1 = INTERFRAME  */
            unsigned char version : 3;
            unsigned char show_frame : 1;
            unsigned char update_mb_segmentation_data : 1;    /**< Must be 0 if segmentation is not enabled */
            unsigned char Reserved2Bits : 2;
        }vp8_frame_tag;
        unsigned char wFrameTagFlags;
    };
    unsigned char Reserved1[4];
    unsigned int  Reserved2[3];
} CUVIDVP8PICPARAMS;

/***********************************************************/
//! \struct CUVIDVP9PICPARAMS
//! VP9 picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/***********************************************************/
typedef struct _CUVIDVP9PICPARAMS
{
    unsigned int width;
    unsigned int height;

    //Frame Indices
    unsigned char LastRefIdx;
    unsigned char GoldenRefIdx;
    unsigned char AltRefIdx;
    unsigned char colorSpace;

    unsigned short profile : 3;
    unsigned short frameContextIdx : 2;
    unsigned short frameType : 1;
    unsigned short showFrame : 1;
    unsigned short errorResilient : 1;
    unsigned short frameParallelDecoding : 1;
    unsigned short subSamplingX : 1;
    unsigned short subSamplingY : 1;
    unsigned short intraOnly : 1;
    unsigned short allow_high_precision_mv : 1;
    unsigned short refreshEntropyProbs : 1;
    unsigned short reserved2Bits : 2;

    unsigned short reserved16Bits;

    unsigned char  refFrameSignBias[4];

    unsigned char bitDepthMinus8Luma;
    unsigned char bitDepthMinus8Chroma;
    unsigned char loopFilterLevel;
    unsigned char loopFilterSharpness;

    unsigned char modeRefLfEnabled;
    unsigned char log2_tile_columns;
    unsigned char log2_tile_rows;

    unsigned char segmentEnabled : 1;
    unsigned char segmentMapUpdate : 1;
    unsigned char segmentMapTemporalUpdate : 1;
    unsigned char segmentFeatureMode : 1;
    unsigned char reserved4Bits : 4;


    unsigned char segmentFeatureEnable[8][4];
    short         segmentFeatureData[8][4];
    unsigned char mb_segment_tree_probs[7];
    unsigned char segment_pred_probs[3];
    unsigned char reservedSegment16Bits[2];

    int qpYAc;
    int qpYDc;
    int qpChDc;
    int qpChAc;

    unsigned int activeRefIdx[3];
    unsigned int resetFrameContext;
    unsigned int mcomp_filter_type;
    unsigned int mbRefLfDelta[4];
    unsigned int mbModeLfDelta[2];
    unsigned int frameTagSize;
    unsigned int offsetToDctParts;
    unsigned int reserved128Bits[4];

} CUVIDVP9PICPARAMS;

/***********************************************************/
//! \struct CUVIDAV1PICPARAMS
//! AV1 picture parameters
//! This structure is used in CUVIDPICPARAMS structure
/***********************************************************/
typedef struct _CUVIDAV1PICPARAMS
{
    unsigned int   width;                               // coded width, if superres enabled then it is upscaled width
    unsigned int   height;                              // coded height
    unsigned int   frame_offset;                        // defined as order_hint in AV1 specification
    int            decodePicIdx;                        // decoded output pic index, if film grain enabled, it will keep decoded (without film grain) output
                                                        // It can be used as reference frame for future frames

    // sequence header 
    unsigned int   profile : 3;                         // 0 = profile0, 1 = profile1, 2 = profile2
    unsigned int   use_128x128_superblock : 1;          // superblock size 0:64x64, 1: 128x128
    unsigned int   subsampling_x : 1;                   // (subsampling_x, _y) 1,1 = 420, 1,0 = 422, 0,0 = 444
    unsigned int   subsampling_y : 1;
    unsigned int   mono_chrome : 1;                     // for monochrome content, mono_chrome = 1 and (subsampling_x, _y) should be 1,1
    unsigned int   bit_depth_minus8 : 4;                // bit depth minus 8
    unsigned int   enable_filter_intra : 1;             // tool enable in seq level, 0 : disable 1: frame header control
    unsigned int   enable_intra_edge_filter : 1;        // intra edge filtering process, 0 : disable 1: enabled
    unsigned int   enable_interintra_compound : 1;      // interintra, 0 : not present 1: present
    unsigned int   enable_masked_compound : 1;          // 1: mode info for inter blocks may contain the syntax element compound_type.
                                                        // 0: syntax element compound_type will not be present
    unsigned int   enable_dual_filter : 1;              // vertical and horiz filter selection, 1: enable and 0: disable 
    unsigned int   enable_order_hint : 1;               // order hint, and related tools, 1: enable and 0: disable 
    unsigned int   order_hint_bits_minus1 : 3;          // is used to compute OrderHintBits
    unsigned int   enable_jnt_comp : 1;                 // joint compound modes, 1: enable and 0: disable 
    unsigned int   enable_superres : 1;                 // superres in seq level, 0 : disable 1: frame level control
    unsigned int   enable_cdef : 1;                     // cdef filtering in seq level, 0 : disable 1: frame level control
    unsigned int   enable_restoration : 1;              // loop restoration filtering in seq level, 0 : disable 1: frame level control
    unsigned int   enable_fgs : 1;                      // defined as film_grain_params_present in AV1 specification
    unsigned int   reserved0_7bits : 7;                 // reserved bits; must be set to 0

    // frame header
    unsigned int   frame_type : 2 ;                     // 0:Key frame, 1:Inter frame, 2:intra only, 3:s-frame
    unsigned int   show_frame : 1 ;                     // show_frame = 1 implies that frame should be immediately output once decoded
    unsigned int   disable_cdf_update : 1;              // CDF update during symbol decoding, 1: disabled, 0: enabled
    unsigned int   allow_screen_content_tools : 1;      // 1: intra blocks may use palette encoding, 0: palette encoding is never used
    unsigned int   force_integer_mv : 1;                // 1: motion vectors will always be integers, 0: can contain fractional bits
    unsigned int   coded_denom : 3;                     // coded_denom of the superres scale as specified in AV1 specification
    unsigned int   allow_intrabc : 1;                   // 1: intra block copy may be used, 0: intra block copy is not allowed
    unsigned int   allow_high_precision_mv : 1;         // 1/8 precision mv enable
    unsigned int   interp_filter : 3;                   // interpolation filter. Refer to section 6.8.9 of the AV1 specification Version 1.0.0 with Errata 1
    unsigned int   switchable_motion_mode : 1;          // defined as is_motion_mode_switchable in AV1 specification
    unsigned int   use_ref_frame_mvs : 1;               // 1: current frame can use the previous frame mv information, 0: will not use.
    unsigned int   disable_frame_end_update_cdf : 1;    // 1: indicates that the end of frame CDF update is disabled
    unsigned int   delta_q_present : 1;                 // quantizer index delta values are present in the block level
    unsigned int   delta_q_res : 2;                     // left shift which should be applied to decoded quantizer index delta values
    unsigned int   using_qmatrix : 1;                   // 1: quantizer matrix will be used to compute quantizers
    unsigned int   coded_lossless : 1;                  // 1: all segments use lossless coding
    unsigned int   use_superres : 1;                    // 1: superres enabled for frame 
    unsigned int   tx_mode : 2;                         // 0: ONLY4x4,1:LARGEST,2:SELECT
    unsigned int   reference_mode : 1;                  // 0: SINGLE, 1: SELECT
    unsigned int   allow_warped_motion : 1;             // 1: allow_warped_motion may be present, 0: allow_warped_motion will not be present
    unsigned int   reduced_tx_set : 1;                  // 1: frame is restricted to subset of the full set of transform types, 0: no such restriction
    unsigned int   skip_mode : 1;                       // 1: most of the mode info is skipped, 0: mode info is not skipped
    unsigned int   reserved1_3bits : 3;                 // reserved bits; must be set to 0

    // tiling info
    unsigned int   num_tile_cols : 8;                   // number of tiles across the frame., max is 64
    unsigned int   num_tile_rows : 8;                   // number of tiles down the frame., max is 64
    unsigned int   context_update_tile_id : 16;         // specifies which tile to use for the CDF update
    unsigned short tile_widths[64];                     // Width of each column in superblocks
    unsigned short tile_heights[64];                    // height of each row in superblocks

    // CDEF - refer to section 6.10.14 of the AV1 specification Version 1.0.0 with Errata 1
    unsigned char  cdef_damping_minus_3 : 2;            // controls the amount of damping in the deringing filter 
    unsigned char  cdef_bits : 2;                       // the number of bits needed to specify which CDEF filter to apply  
    unsigned char  reserved2_4bits : 4;                 // reserved bits; must be set to 0
    unsigned char  cdef_y_strength[8];                  // 0-3 bits: y_pri_strength, 4-7 bits y_sec_strength
    unsigned char  cdef_uv_strength[8];                 // 0-3 bits: uv_pri_strength, 4-7 bits uv_sec_strength

    // SkipModeFrames
    unsigned char   SkipModeFrame0 : 4;                 // specifies the frames to use for compound prediction when skip_mode is equal to 1.
    unsigned char   SkipModeFrame1 : 4;

    // qp information - refer to section 6.8.11 of the AV1 specification Version 1.0.0 with Errata 1
    unsigned char  base_qindex;                         // indicates the base frame qindex. Defined as base_q_idx in AV1 specification
    char           qp_y_dc_delta_q;                     // indicates the Y DC quantizer relative to base_q_idx. Defined as DeltaQYDc in AV1 specification
    char           qp_u_dc_delta_q;                     // indicates the U DC quantizer relative to base_q_idx. Defined as DeltaQUDc in AV1 specification
    char           qp_v_dc_delta_q;                     // indicates the V DC quantizer relative to base_q_idx. Defined as DeltaQVDc in AV1 specification
    char           qp_u_ac_delta_q;                     // indicates the U AC quantizer relative to base_q_idx. Defined as DeltaQUAc in AV1 specification
    char           qp_v_ac_delta_q;                     // indicates the V AC quantizer relative to base_q_idx. Defined as DeltaQVAc in AV1 specification
    unsigned char  qm_y;                                // specifies the level in the quantizer matrix that should be used for luma plane decoding
    unsigned char  qm_u;                                // specifies the level in the quantizer matrix that should be used for chroma U plane decoding
    unsigned char  qm_v;                                // specifies the level in the quantizer matrix that should be used for chroma V plane decoding

    // segmentation - refer to section 6.8.13 of the AV1 specification Version 1.0.0 with Errata 1
    unsigned char segmentation_enabled : 1;             // 1 indicates that this frame makes use of the segmentation tool
    unsigned char segmentation_update_map : 1;          // 1 indicates that the segmentation map are updated during the decoding of this frame
    unsigned char segmentation_update_data : 1;         // 1 indicates that new parameters are about to be specified for each segment
    unsigned char segmentation_temporal_update : 1;     // 1 indicates that the updates to the segmentation map are coded relative to the existing segmentation map
    unsigned char reserved3_4bits : 4;                  // reserved bits; must be set to 0
    short         segmentation_feature_data[8][8];      // specifies the feature data for a segment feature
    unsigned char segmentation_feature_mask[8];         // indicates that the corresponding feature is unused or feature value is coded

    // loopfilter - refer to section 6.8.10 of the AV1 specification Version 1.0.0 with Errata 1
    unsigned char  loop_filter_level[2];                // contains loop filter strength values
    unsigned char  loop_filter_level_u;                 // loop filter strength value of U plane
    unsigned char  loop_filter_level_v;                 // loop filter strength value of V plane
    unsigned char  loop_filter_sharpness;               // indicates the sharpness level
    char           loop_filter_ref_deltas[8];           // contains the adjustment needed for the filter level based on the chosen reference frame
    char           loop_filter_mode_deltas[2];          // contains the adjustment needed for the filter level based on the chosen mode
    unsigned char  loop_filter_delta_enabled : 1;       // indicates that the filter level depends on the mode and reference frame used to predict a block
    unsigned char  loop_filter_delta_update : 1;        // indicates that additional syntax elements are present that specify which mode and
                                                        // reference frame deltas are to be updated
    unsigned char  delta_lf_present : 1;                // specifies whether loop filter delta values are present in the block level
    unsigned char  delta_lf_res : 2;                    // specifies the left shift to apply to the decoded loop filter values
    unsigned char  delta_lf_multi  : 1;                 // separate loop filter deltas for Hy,Vy,U,V edges
    unsigned char  reserved4_2bits : 2;                 // reserved bits; must be set to 0

    // restoration - refer to section 6.10.15 of the AV1 specification Version 1.0.0 with Errata 1
    unsigned char lr_unit_size[3];                     // specifies the size of loop restoration units: 0: 32, 1: 64, 2: 128, 3: 256
    unsigned char lr_type[3] ;                         // used to compute FrameRestorationType

    // reference frames
    unsigned char primary_ref_frame;                    // specifies which reference frame contains the CDF values and other state that should be 
                                                        // loaded at the start of the frame
    unsigned char ref_frame_map[8];                     // frames in dpb that can be used as reference for current or future frames

    unsigned char temporal_layer_id : 4;                // temporal layer id
    unsigned char spatial_layer_id : 4;                 // spatial layer id

    unsigned char reserved5_32bits[4];                  // reserved bits; must be set to 0

    // ref frame list
    struct
    {
        unsigned int   width;
        unsigned int   height;
        unsigned char  index;
        unsigned char  reserved24Bits[3];               // reserved bits; must be set to 0
    } ref_frame[7];                                     // frames used as reference frame for current frame.
    
    // global motion
    struct {
        unsigned char invalid : 1;
        unsigned char wmtype : 2;                       // defined as GmType in AV1 specification
        unsigned char reserved5Bits : 5;                // reserved bits; must be set to 0
        char          reserved24Bits[3];                // reserved bits; must be set to 0
        int           wmmat[6];                         // defined as gm_params[] in AV1 specification
    } global_motion[7];                                 // global motion params for reference frames
    
    // film grain params - refer to section 6.8.20 of the AV1 specification Version 1.0.0 with Errata 1
    unsigned short apply_grain : 1;
    unsigned short overlap_flag : 1;
    unsigned short scaling_shift_minus8 : 2;
    unsigned short chroma_scaling_from_luma : 1;  
    unsigned short ar_coeff_lag : 2;
    unsigned short ar_coeff_shift_minus6 : 2;
    unsigned short grain_scale_shift : 2;
    unsigned short clip_to_restricted_range : 1;
    unsigned short reserved6_4bits : 4;                 // reserved bits; must be set to 0
    unsigned char  num_y_points;
    unsigned char  scaling_points_y[14][2];
    unsigned char  num_cb_points;
    unsigned char  scaling_points_cb[10][2];
    unsigned char  num_cr_points;
    unsigned char  scaling_points_cr[10][2];
    unsigned char  reserved7_8bits;                     // reserved bits; must be set to 0
    unsigned short random_seed;
    short          ar_coeffs_y[24];
    short          ar_coeffs_cb[25];
    short          ar_coeffs_cr[25];
    unsigned char  cb_mult;
    unsigned char  cb_luma_mult;
    short          cb_offset;
    unsigned char  cr_mult;
    unsigned char  cr_luma_mult;
    short          cr_offset;

    int            reserved[7];                       // reserved bits; must be set to 0
} CUVIDAV1PICPARAMS;

/******************************************************************************************/
//! \struct CUVIDPICPARAMS
//! Picture parameters for decoding
//! This structure is used in cuvidDecodePicture API
//! IN  for cuvidDecodePicture
/******************************************************************************************/
typedef struct _CUVIDPICPARAMS
{
    int PicWidthInMbs;                     /**< IN: Coded frame size in macroblocks                           */
    int FrameHeightInMbs;                  /**< IN: Coded frame height in macroblocks                         */
    int CurrPicIdx;                        /**< IN: Output index of the current picture                       */
    int field_pic_flag;                    /**< IN: 0=frame picture, 1=field picture                          */
    int bottom_field_flag;                 /**< IN: 0=top field, 1=bottom field (ignored if field_pic_flag=0) */
    int second_field;                      /**< IN: Second field of a complementary field pair                */
    // Bitstream data
    unsigned int nBitstreamDataLen;        /**< IN: Number of bytes in bitstream data buffer                  */
    const unsigned char *pBitstreamData;   /**< IN: Ptr to bitstream data for this picture (slice-layer)      */
    unsigned int nNumSlices;               /**< IN: Number of slices in this picture                          */
    const unsigned int *pSliceDataOffsets; /**< IN: nNumSlices entries, contains offset of each slice within 
                                                        the bitstream data buffer                             */
    int ref_pic_flag;                      /**< IN: This picture is a reference picture                       */
    int intra_pic_flag;                    /**< IN: This picture is entirely intra coded                      */
    unsigned int Reserved[30];             /**< Reserved for future use                                       */
    // IN: Codec-specific data
    union {
        CUVIDMPEG2PICPARAMS mpeg2;         /**< Also used for MPEG-1 */
        CUVIDH264PICPARAMS  h264;
        CUVIDVC1PICPARAMS   vc1;
        CUVIDMPEG4PICPARAMS mpeg4;
        CUVIDJPEGPICPARAMS  jpeg;
        CUVIDHEVCPICPARAMS  hevc;
        CUVIDVP8PICPARAMS   vp8;
        CUVIDVP9PICPARAMS   vp9;
        CUVIDAV1PICPARAMS   av1;
        unsigned int CodecReserved[1024];
    } CodecSpecific;
} CUVIDPICPARAMS;


/******************************************************/
//! \struct CUVIDPROCPARAMS
//! Picture parameters for postprocessing
//! This structure is used in cuvidMapVideoFrame API
/******************************************************/
typedef struct _CUVIDPROCPARAMS
{
    int progressive_frame;                        /**< IN: Input is progressive (deinterlace_mode will be ignored)                */
    int second_field;                             /**< IN: Output the second field (ignored if deinterlace mode is Weave)         */
    int top_field_first;                          /**< IN: Input frame is top field first (1st field is top, 2nd field is bottom) */
    int unpaired_field;                           /**< IN: Input only contains one field (2nd field is invalid)                   */
    // The fields below are used for raw YUV input
    unsigned int reserved_flags;                  /**< Reserved for future use (set to zero)                                      */
    unsigned int reserved_zero;                   /**< Reserved (set to zero)                                                     */
    unsigned long long raw_input_dptr;            /**< IN: Input CUdeviceptr for raw YUV extensions                               */
    unsigned int raw_input_pitch;                 /**< IN: pitch in bytes of raw YUV input (should be aligned appropriately)      */
    unsigned int raw_input_format;                /**< IN: Input YUV format (cudaVideoCodec_enum)                                 */
    unsigned long long raw_output_dptr;           /**< IN: Output CUdeviceptr for raw YUV extensions                              */
    unsigned int raw_output_pitch;                /**< IN: pitch in bytes of raw YUV output (should be aligned appropriately)     */
    unsigned int Reserved1;                       /**< Reserved for future use (set to zero)                                      */
    CUstream output_stream;                       /**< IN: stream object used by cuvidMapVideoFrame                               */
    unsigned int Reserved[46];                    /**< Reserved for future use (set to zero)                                      */
    unsigned long long *histogram_dptr;           /**< OUT: Output CUdeviceptr for histogram extensions                           */
    void *Reserved2[1];                           /**< Reserved for future use (set to zero)                                      */
} CUVIDPROCPARAMS;

/*********************************************************************************************************/
//! \struct CUVIDGETDECODESTATUS
//! Struct for reporting decode status.
//! This structure is used in cuvidGetDecodeStatus API.
/*********************************************************************************************************/
typedef struct _CUVIDGETDECODESTATUS
{
    cuvidDecodeStatus decodeStatus;
    unsigned int reserved[31];
    void *pReserved[8];
} CUVIDGETDECODESTATUS;

/****************************************************/
//! \struct CUVIDRECONFIGUREDECODERINFO
//! Struct for decoder reset
//! This structure is used in cuvidReconfigureDecoder() API
/****************************************************/
typedef struct _CUVIDRECONFIGUREDECODERINFO
{
    unsigned int ulWidth;             /**< IN: Coded sequence width in pixels, MUST be < = ulMaxWidth defined at CUVIDDECODECREATEINFO  */
    unsigned int ulHeight;            /**< IN: Coded sequence height in pixels, MUST be < = ulMaxHeight defined at CUVIDDECODECREATEINFO  */
    unsigned int ulTargetWidth;       /**< IN: Post processed output width */
    unsigned int ulTargetHeight;      /**< IN: Post Processed output height */
    unsigned int ulNumDecodeSurfaces; /**< IN: Maximum number of internal decode surfaces */
    unsigned int reserved1[12];       /**< Reserved for future use. Set to Zero */
    /**
    * IN: Area of frame to be displayed. Use-case : Source Cropping
    */
    struct {
        short left;
        short top;
        short right;
        short bottom;
    } display_area;
    /**
    * IN: Target Rectangle in the OutputFrame. Use-case : Aspect ratio Conversion
    */
    struct {
        short left;
        short top;
        short right;
        short bottom;
    } target_rect;
    unsigned int reserved2[11]; /**< Reserved for future use. Set to Zero */
} CUVIDRECONFIGUREDECODERINFO; 


/***********************************************************************************************************/
//! VIDEO_DECODER
//!
//! In order to minimize decode latencies, there should be always at least 2 pictures in the decode
//! queue at any time, in order to make sure that all decode engines are always busy.
//!
//! Overall data flow:
//!  - cuvidGetDecoderCaps(...)
//!  - cuvidCreateDecoder(...)
//!  - For each picture:
//!    + cuvidDecodePicture(N)
//!    + cuvidMapVideoFrame(N-4)
//!    + do some processing in cuda
//!    + cuvidUnmapVideoFrame(N-4)
//!    + cuvidDecodePicture(N+1)
//!    + cuvidMapVideoFrame(N-3)
//!    + ...
//!  - cuvidDestroyDecoder(...)
//!
//! NOTE:
//! - When the cuda context is created from a D3D device, the D3D device must also be created
//!   with the D3DCREATE_MULTITHREADED flag.
//! - There is a limit to how many pictures can be mapped simultaneously (ulNumOutputSurfaces)
//! - cuvidDecodePicture may block the calling thread if there are too many pictures pending
//!   in the decode queue
/***********************************************************************************************************/


/**********************************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidGetDecoderCaps(CUVIDDECODECAPS *pdc)
//! Queries decode capabilities of NVDEC-HW based on CodecType, ChromaFormat and BitDepthMinus8 parameters.
//! 1. Application fills IN parameters CodecType, ChromaFormat and BitDepthMinus8 of CUVIDDECODECAPS structure
//! 2. On calling cuvidGetDecoderCaps, driver fills OUT parameters if the IN parameters are supported
//!    If IN parameters passed to the driver are not supported by NVDEC-HW, then all OUT params are set to 0.
//! E.g. on Geforce GTX 960:
//!   App fills - eCodecType = cudaVideoCodec_H264; eChromaFormat = cudaVideoChromaFormat_420; nBitDepthMinus8 = 0;
//!   Given IN parameters are supported, hence driver fills: bIsSupported = 1; nMinWidth   = 48; nMinHeight  = 16; 
//!   nMaxWidth = 4096; nMaxHeight = 4096; nMaxMBCount = 65536;
//! CodedWidth*CodedHeight/256 must be less than or equal to nMaxMBCount
/**********************************************************************************************************************/
extern CUresult CUDAAPI cuvidGetDecoderCaps(CUVIDDECODECAPS *pdc);

/*****************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidCreateDecoder(CUvideodecoder *phDecoder, CUVIDDECODECREATEINFO *pdci)
//! Create the decoder object based on pdci. A handle to the created decoder is returned
/*****************************************************************************************************/
extern CUresult CUDAAPI cuvidCreateDecoder(CUvideodecoder *phDecoder, CUVIDDECODECREATEINFO *pdci);

/*****************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidDestroyDecoder(CUvideodecoder hDecoder)
//! Destroy the decoder object
/*****************************************************************************************************/
extern CUresult CUDAAPI cuvidDestroyDecoder(CUvideodecoder hDecoder);

/*****************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidDecodePicture(CUvideodecoder hDecoder, CUVIDPICPARAMS *pPicParams)
//! Decode a single picture (field or frame)
//! Kicks off HW decoding 
/*****************************************************************************************************/
extern CUresult CUDAAPI cuvidDecodePicture(CUvideodecoder hDecoder, CUVIDPICPARAMS *pPicParams);

/************************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidGetDecodeStatus(CUvideodecoder hDecoder, int nPicIdx);
//! Get the decode status for frame corresponding to nPicIdx
//! API is supported for Maxwell and above generation GPUs.
//! API is currently supported for HEVC, H264 and JPEG codecs.
//! API returns CUDA_ERROR_NOT_SUPPORTED error code for unsupported GPU or codec.
/************************************************************************************************************/
extern CUresult CUDAAPI cuvidGetDecodeStatus(CUvideodecoder hDecoder, int nPicIdx, CUVIDGETDECODESTATUS* pDecodeStatus);

/*********************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidReconfigureDecoder(CUvideodecoder hDecoder, CUVIDRECONFIGUREDECODERINFO *pDecReconfigParams)
//! Used to reuse single decoder for multiple clips. Currently supports resolution change, resize params, display area 
//! params, target area params change for same codec. Must be called during CUVIDPARSERPARAMS::pfnSequenceCallback 
/*********************************************************************************************************/
extern CUresult CUDAAPI cuvidReconfigureDecoder(CUvideodecoder hDecoder, CUVIDRECONFIGUREDECODERINFO *pDecReconfigParams);


#if !defined(__CUVID_DEVPTR64) || defined(__CUVID_INTERNAL)
/************************************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidMapVideoFrame(CUvideodecoder hDecoder, int nPicIdx, unsigned int *pDevPtr, 
//!                                         unsigned int *pPitch, CUVIDPROCPARAMS *pVPP);
//! Post-process and map video frame corresponding to nPicIdx for use in cuda. Returns cuda device pointer and associated
//! pitch of the video frame
/************************************************************************************************************************/
extern CUresult CUDAAPI cuvidMapVideoFrame(CUvideodecoder hDecoder, int nPicIdx,
                                           unsigned int *pDevPtr, unsigned int *pPitch,
                                           CUVIDPROCPARAMS *pVPP);

/*****************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidUnmapVideoFrame(CUvideodecoder hDecoder, unsigned int DevPtr)
//! Unmap a previously mapped video frame
/*****************************************************************************************************/
extern CUresult CUDAAPI cuvidUnmapVideoFrame(CUvideodecoder hDecoder, unsigned int DevPtr);
#endif

/****************************************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidMapVideoFrame64(CUvideodecoder hDecoder, int nPicIdx, unsigned long long *pDevPtr, 
//!                                           unsigned int * pPitch, CUVIDPROCPARAMS *pVPP);
//! Post-process and map video frame corresponding to nPicIdx for use in cuda. Returns cuda device pointer and associated
//! pitch of the video frame
/****************************************************************************************************************************/
extern CUresult CUDAAPI cuvidMapVideoFrame64(CUvideodecoder hDecoder, int nPicIdx, unsigned long long *pDevPtr,
                                             unsigned int *pPitch, CUVIDPROCPARAMS *pVPP);

/**************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidUnmapVideoFrame64(CUvideodecoder hDecoder, unsigned long long DevPtr);
//! Unmap a previously mapped video frame
/**************************************************************************************************/
extern CUresult CUDAAPI cuvidUnmapVideoFrame64(CUvideodecoder hDecoder, unsigned long long DevPtr);

#if defined(__CUVID_DEVPTR64) && !defined(__CUVID_INTERNAL)
#define cuvidMapVideoFrame      cuvidMapVideoFrame64
#define cuvidUnmapVideoFrame    cuvidUnmapVideoFrame64
#endif



/********************************************************************************************************************/
//!
//! Context-locking: to facilitate multi-threaded implementations, the following 4 functions
//! provide a simple mutex-style host synchronization. If a non-NULL context is specified
//! in CUVIDDECODECREATEINFO, the codec library will acquire the mutex associated with the given
//! context before making any cuda calls.
//! A multi-threaded application could create a lock associated with a context handle so that
//! multiple threads can safely share the same cuda context:
//!  - use cuCtxPopCurrent immediately after context creation in order to create a 'floating' context
//!    that can be passed to cuvidCtxLockCreate.
//!  - When using a floating context, all cuda calls should only be made within a cuvidCtxLock/cuvidCtxUnlock section.
//!
//! NOTE: This is a safer alternative to cuCtxPushCurrent and cuCtxPopCurrent, and is not related to video
//! decoder in any way (implemented as a critical section associated with cuCtx{Push|Pop}Current calls).
/********************************************************************************************************************/

/********************************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidCtxLockCreate(CUvideoctxlock *pLock, CUcontext ctx)
//! This API is used to create CtxLock object
/********************************************************************************************************************/
extern CUresult CUDAAPI cuvidCtxLockCreate(CUvideoctxlock *pLock, CUcontext ctx);

/********************************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidCtxLockDestroy(CUvideoctxlock lck)
//! This API is used to free CtxLock object
/********************************************************************************************************************/
extern CUresult CUDAAPI cuvidCtxLockDestroy(CUvideoctxlock lck);

/********************************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidCtxLock(CUvideoctxlock lck, unsigned int reserved_flags)
//! This API is used to acquire ctxlock
/********************************************************************************************************************/
extern CUresult CUDAAPI cuvidCtxLock(CUvideoctxlock lck, unsigned int reserved_flags);

/********************************************************************************************************************/
//! \fn CUresult CUDAAPI cuvidCtxUnlock(CUvideoctxlock lck, unsigned int reserved_flags)
//! This API is used to release ctxlock
/********************************************************************************************************************/
extern CUresult CUDAAPI cuvidCtxUnlock(CUvideoctxlock lck, unsigned int reserved_flags);

/**********************************************************************************************/


#if defined(__cplusplus)
}
// Auto-lock helper for C++ applications
class CCtxAutoLock
{
private:
    CUvideoctxlock m_ctx;
public:
    CCtxAutoLock(CUvideoctxlock ctx):m_ctx(ctx) { cuvidCtxLock(m_ctx,0); }
    ~CCtxAutoLock() { cuvidCtxUnlock(m_ctx,0); }
};
#endif /* __cplusplus */

#endif // __CUDA_VIDEO_H__

