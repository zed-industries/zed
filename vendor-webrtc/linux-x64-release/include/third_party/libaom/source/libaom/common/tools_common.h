/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_COMMON_TOOLS_COMMON_H_
#define AOM_COMMON_TOOLS_COMMON_H_

#include <stdbool.h>
#include <stdio.h>

#include "config/aom_config.h"

#include "aom/aom_codec.h"
#include "aom/aom_image.h"
#include "aom/aom_integer.h"
#include "aom_ports/mem.h"

#if CONFIG_AV1_ENCODER
#include "common/y4minput.h"
#endif

#if defined(_MSC_VER)
/* MSVS uses _f{seek,tell}i64. */
#define fseeko _fseeki64
#define ftello _ftelli64
typedef int64_t FileOffset;
#elif defined(_WIN32)
#include <sys/types.h> /* NOLINT*/
/* MinGW uses f{seek,tell}o64 for large files. */
#define fseeko fseeko64
#define ftello ftello64
typedef off64_t FileOffset;
#elif CONFIG_OS_SUPPORT &&                                                  \
    !(defined(__ANDROID__) && __ANDROID_API__ < 24 && !defined(__LP64__) && \
      defined(_FILE_OFFSET_BITS) && _FILE_OFFSET_BITS == 64)
/* POSIX.1 has fseeko and ftello. fseeko and ftello are not available before
 * Android API level 24. See
 * https://android.googlesource.com/platform/bionic/+/main/docs/32-bit-abi.md */
#include <sys/types.h> /* NOLINT */
typedef off_t FileOffset;
/* Use 32-bit file operations in WebM file format when building ARM
 * executables (.axf) with RVCT. */
#else
#define fseeko fseek
#define ftello ftell
typedef long FileOffset; /* NOLINT */
#endif /* CONFIG_OS_SUPPORT */

#if CONFIG_OS_SUPPORT
#if defined(_MSC_VER)
#include <io.h> /* NOLINT */
#define isatty _isatty
#define fileno _fileno
#else
#include <unistd.h> /* NOLINT */
#endif              /* _MSC_VER */
#endif              /* CONFIG_OS_SUPPORT */

#define LITERALU64(hi, lo) ((((uint64_t)hi) << 32) | lo)

#ifndef PATH_MAX
#define PATH_MAX 512
#endif

#define IVF_FRAME_HDR_SZ (4 + 8) /* 4 byte size + 8 byte timestamp */
#define IVF_FILE_HDR_SZ 32

#define RAW_FRAME_HDR_SZ sizeof(uint32_t)
#define OBU_DETECTION_SZ 34  // See common/obudec.c

#define DETECT_BUF_SZ 34  // Max of the above header sizes

#define AV1_FOURCC 0x31305641

enum VideoFileType {
  FILE_TYPE_OBU,
  FILE_TYPE_RAW,
  FILE_TYPE_IVF,
  FILE_TYPE_Y4M,
  FILE_TYPE_WEBM
};

// The fourcc for large_scale_tile encoding is "LSTC".
#define LST_FOURCC 0x4354534c

struct FileTypeDetectionBuffer {
  char buf[DETECT_BUF_SZ];
  size_t buf_read;
  size_t position;
};

struct AvxRational {
  int numerator;
  int denominator;
};

struct AvxInputContext {
  const char *filename;
  FILE *file;
  int64_t length;
  struct FileTypeDetectionBuffer detect;
  enum VideoFileType file_type;
  uint32_t width;
  uint32_t height;
  struct AvxRational pixel_aspect_ratio;
  aom_img_fmt_t fmt;
  aom_bit_depth_t bit_depth;
  int only_i420;
  uint32_t fourcc;
  struct AvxRational framerate;
#if CONFIG_AV1_ENCODER
  y4m_input y4m;
#endif
  aom_color_range_t color_range;
};

#ifdef __cplusplus
extern "C" {
#endif

#if defined(__GNUC__)
#define AOM_NO_RETURN __attribute__((noreturn))
#elif defined(_MSC_VER)
#define AOM_NO_RETURN __declspec(noreturn)
#else
#define AOM_NO_RETURN
#endif

// Tells the compiler to perform `printf` format string checking if the
// compiler supports it; see the 'format' attribute in
// <https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html>.
#define AOM_TOOLS_FORMAT_PRINTF(string_index, first_to_check)
#if defined(__has_attribute)
#if __has_attribute(format)
#undef AOM_TOOLS_FORMAT_PRINTF
#define AOM_TOOLS_FORMAT_PRINTF(string_index, first_to_check) \
  __attribute__((__format__(__printf__, string_index, first_to_check)))
#endif
#endif

/* Sets a stdio stream into binary mode */
FILE *set_binary_mode(FILE *stream);

AOM_NO_RETURN void die(const char *fmt, ...) AOM_TOOLS_FORMAT_PRINTF(1, 2);
AOM_NO_RETURN void fatal(const char *fmt, ...) AOM_TOOLS_FORMAT_PRINTF(1, 2);
void aom_tools_warn(const char *fmt, ...) AOM_TOOLS_FORMAT_PRINTF(1, 2);

AOM_NO_RETURN void die_codec(aom_codec_ctx_t *ctx, const char *s);

/* The tool including this file must define usage_exit() */
AOM_NO_RETURN void usage_exit(void);

#undef AOM_NO_RETURN

// The AOM library can support different encoders / decoders. These
// functions provide different ways to lookup / iterate through them.
// The return result may be NULL to indicate no codec was found.
int get_aom_encoder_count(void);
aom_codec_iface_t *get_aom_encoder_by_index(int i);
aom_codec_iface_t *get_aom_encoder_by_short_name(const char *name);
// If the interface is unknown, returns NULL.
const char *get_short_name_by_aom_encoder(aom_codec_iface_t *encoder);
// If the interface is unknown, returns 0.
uint32_t get_fourcc_by_aom_encoder(aom_codec_iface_t *iface);

int get_aom_decoder_count(void);
aom_codec_iface_t *get_aom_decoder_by_index(int i);
aom_codec_iface_t *get_aom_decoder_by_short_name(const char *name);
aom_codec_iface_t *get_aom_decoder_by_fourcc(uint32_t fourcc);
const char *get_short_name_by_aom_decoder(aom_codec_iface_t *decoder);
// If the interface is unknown, returns 0.
uint32_t get_fourcc_by_aom_decoder(aom_codec_iface_t *iface);

const char *image_format_to_string(aom_img_fmt_t fmt);

int read_yuv_frame(struct AvxInputContext *input_ctx, aom_image_t *yuv_frame);

void aom_img_write(const aom_image_t *img, FILE *file);
// Returns true on success, false on failure.
bool aom_img_read(aom_image_t *img, FILE *file);

double sse_to_psnr(double samples, double peak, double mse);
void aom_img_upshift(aom_image_t *dst, const aom_image_t *src, int input_shift);
void aom_img_downshift(aom_image_t *dst, const aom_image_t *src,
                       int down_shift);
// Returns true on success, false on failure.
bool aom_shift_img(unsigned int output_bit_depth, aom_image_t **img_ptr,
                   aom_image_t **img_shifted_ptr);
void aom_img_truncate_16_to_8(aom_image_t *dst, const aom_image_t *src);

// Output in NV12 format.
void aom_img_write_nv12(const aom_image_t *img, FILE *file);

size_t read_from_input(struct AvxInputContext *input_ctx, size_t n,
                       unsigned char *buf);
size_t input_to_detect_buf(struct AvxInputContext *input_ctx, size_t n);
size_t buffer_input(struct AvxInputContext *input_ctx, size_t n,
                    unsigned char *buf, bool buffered);
void rewind_detect(struct AvxInputContext *input_ctx);
bool input_eof(struct AvxInputContext *input_ctx);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif  // AOM_COMMON_TOOLS_COMMON_H_
