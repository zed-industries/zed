/*
 * Copyright (c) 2018-2020 Intel Corporation. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 * IN NO EVENT SHALL INTEL AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/**
 * \file va_protected_content.h
 * \brief Protection content general interface
 *
 * This file contains the \ref api_protected_content "Protected Content
 * Interface".
 */

#ifndef VA_PROTECTED_CONTENT_H
#define VA_PROTECTED_CONTENT_H

#include <stdint.h>
#include <va/va.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_intel Protected Content(PC) API
 *
 * @{
 */

/**\brief CENC status paramter, used for vendor content protection only.
 * The buffer corresponds to #VACencStatusParameters for va/cp*/
#define VACencStatusParameterBufferType ((VABufferType)0x20002)

/** \brief TeeExec Function Codes. */
#define VA_TEE_EXEC_TEE_FUNCID_HW_UPDATE 0x40000002
#define VA_TEE_EXEC_TEE_FUNCID_IS_SESSION_ALIVE 0x40000103

/** \brief values for the encryption return status. */
typedef enum {
  /** \brief Indicate encryption operation is successful.*/
  VA_ENCRYPTION_STATUS_SUCCESSFUL = 0,
  /** \brief Indicate encryption operation is incomplete. */
  VA_ENCRYPTION_STATUS_INCOMPLETE,
  /** \brief Indicate encryption operation is error.*/
  VA_ENCRYPTION_STATUS_ERROR,
  /** \brief Indicate the buf in VACencStatusBuf is full. */
  VA_ENCRYPTION_STATUS_BUFFER_FULL,
  /** \brief Indicate encryption operation is unsupport. */
  VA_ENCRYPTION_STATUS_UNSUPPORT
} VAEncryptionStatus;

/** \brief cenc status parameters, corresponding to
 * #VACencStatusParameterBufferType*/
typedef struct _VACencStatusParameters {
  /** \brief The status report index feedback. */
  uint32_t status_report_index_feedback;

  /** \brief Reserved bytes for future use, must be zero */
  uint32_t va_reserved[VA_PADDING_MEDIUM];
} VACencStatusParameters;
/**
 * \brief Slice parameter for H.264 cenc decode in baseline, main & high
 * profiles.
 *
 * This structure holds information for \c
 * slice_layer_without_partitioning_rbsp() and nal_unit()of the slice
 * as defined by the H.264 specification.
 *
 */
typedef struct _VACencSliceParameterBufferH264 {
  /** \brief Parameters from \c nal_unit() of the slice.*/
  /**@{*/
  /** \brief  Same as the H.264 bitstream syntax element. */
  uint8_t nal_ref_idc;
  /** \brief Indicate if this is coded slice of an IDR picture.
   * Corresponds to IdrPicFlag as the H.264 specification.*/
  uint8_t idr_pic_flag;
  /**@}*/
  /** \brief Same as the H.264 bitstream syntax element. */
  uint8_t slice_type;
  /** \brief Indicate if this is a field or frame picture.
   * \c VA_FRAME_PICTURE, \c VA_TOP_FIELD, \c VA_BOTTOM_FIELD*/
  uint8_t field_frame_flag;
  /** \brief Same as the H.264 bitstream syntax element. */
  uint32_t frame_number;
  /** \brief Same as the H.264 bitstream syntax element. */
  uint32_t idr_pic_id;
  /** \brief Same as the H.264 bitstream syntax element. */
  int32_t pic_order_cnt_lsb;
  /** \brief Same as the H.264 bitstream syntax element. */
  int32_t delta_pic_order_cnt_bottom;
  /** \brief Same as the H.264 bitstream syntax element. */
  int32_t delta_pic_order_cnt[2];
  /**
   * \brief decoded reference picture marking. Information for \c
   * dec_ref_pic_marking() as defined by the H.264 specification.
   */
  /**@{*/
  union {
    struct {
      /** \brief Same as the H.264 bitstream syntax element. */
      uint32_t no_output_of_prior_pics_flag : 1;
      /** \brief Same as the H.264 bitstream syntax element. */
      uint32_t long_term_reference_flag : 1;
      /** \brief Same as the H.264 bitstream syntax element. */
      uint32_t adaptive_ref_pic_marking_mode_flag : 1;
      /** \brief number of decode reference picture marking. */
      uint32_t dec_ref_pic_marking_count : 8;
      /** \brief Reserved for future use, must be zero */
      uint32_t reserved : 21;
    } bits;
    uint32_t value;
  } ref_pic_fields;
  /** \brief Same as the H.264 bitstream syntax element. */
  uint8_t memory_management_control_operation[32];
  /** \brief Same as the H.264 bitstream syntax element. */
  int32_t difference_of_pic_nums_minus1[32];
  /** \brief Same as the H.264 bitstream syntax element. */
  int32_t long_term_pic_num[32];
  /** \brief Same as the H.264 bitstream syntax element. */
  int32_t max_long_term_frame_idx_plus1[32];
  /** \brief Same as the H.264 bitstream syntax element. */
  int32_t long_term_frame_idx[32];
  /**@}*/
  /** \brief Pointer to the next #VACencSliceParameterBufferH264 element,
   * or \c nullptr if there is none.*/
  void* next;

  /** \brief Reserved bytes for future use, must be zero */
  uint32_t va_reserved[VA_PADDING_MEDIUM];
} VACencSliceParameterBufferH264;

/** \brief Cenc Slice Buffer Type*/
typedef enum {
  /** \brief Parsed slice parameters \c VACencSliceParameterBuffer* */
  VaCencSliceBufParamter = 1,
  /** \brief Raw slice header of bitstream*/
  VaCencSliceBufRaw = 2
} VACencSliceBufType;

/** \brief Buffer for CENC status reporting*/
typedef struct _VACencStatusBuf {
  /** \brief Encryption status. VA_ENCRYPTION_STATUS_SUCCESSFUL if
   *  hardware has returned detailed information, others mean the
   *  CENC result is invalid  */
  VAEncryptionStatus status;
  /* \brief feedback of status report index
   * This value is the feedback of status_report_number of
   * \ref VAEncryptionParameters to indicate the CENC workload*/
  uint32_t status_report_index_feedback;
  /** \brief Buf size in bytes.  0 means buf is invalid*/
  uint32_t buf_size;
  /** \brief Buffer formatted as raw data from bitstream for sequence parameter,
   *  picture parameter, SEI parameters. Or \c nullptr means buf is invalid.*/
  void* buf;
  /** \brief Slice buffer type, see \c VACencSliceBufTypex */
  VACencSliceBufType slice_buf_type;
  /** \brief Slice buffer size in bytes. 0 means slice_buf is invalid*/
  uint32_t slice_buf_size;
  /** \brief Slice buffer, parsed slice header information. Or \c nullptr
   *  means slice_buf is invalid.*/
  void* slice_buf;

  /** \brief Reserved bytes for future use, must be zero */
  uint32_t va_reserved[VA_PADDING_MEDIUM];
} VACencStatusBuf;

/**@}*/

#ifdef __cplusplus
}
#endif

#endif  // VA_PROTECTED_CONTENT_H
