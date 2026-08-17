/*
 * common functions for Indeo Video Interactive codecs (Indeo4 and Indeo5)
 *
 * Copyright (c) 2009 Maxim Poliakovski
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

/**
 * @file
 * This file contains structures and macros shared by both Indeo4 and
 * Indeo5 decoders.
 */

#ifndef AVCODEC_IVI_H
#define AVCODEC_IVI_H

#include "avcodec.h"
#include "get_bits.h"
#include <stdint.h>

/**
 *  Indeo 4 frame types.
 */
enum {
    IVI4_FRAMETYPE_INTRA       = 0,
    IVI4_FRAMETYPE_INTRA1      = 1,  ///< intra frame with slightly different bitstream coding
    IVI4_FRAMETYPE_INTER       = 2,  ///< non-droppable P-frame
    IVI4_FRAMETYPE_BIDIR       = 3,  ///< bidirectional frame
    IVI4_FRAMETYPE_INTER_NOREF = 4,  ///< droppable P-frame
    IVI4_FRAMETYPE_NULL_FIRST  = 5,  ///< empty frame with no data
    IVI4_FRAMETYPE_NULL_LAST   = 6   ///< empty frame with no data
};

#define IVI_VLC_BITS 13 ///< max number of bits of the ivi's huffman codes
#define IVI5_IS_PROTECTED       0x20

/**
 *  huffman codebook descriptor
 */
typedef struct IVIHuffDesc {
    int32_t     num_rows;
    uint8_t     xbits[16];
} IVIHuffDesc;

/**
 *  macroblock/block huffman table descriptor
 */
typedef struct IVIHuffTab {
    int32_t     tab_sel;    ///< index of one of the predefined tables, or "7" for custom one
    VLC         *tab;       ///< pointer to the table associated with tab_sel

    // the following are used only when tab_sel == 7
    IVIHuffDesc cust_desc;  ///< custom Huffman codebook descriptor
    VLC         cust_tab;   ///< vlc table for custom codebook
} IVIHuffTab;

enum {
    IVI_MB_HUFF   = 0,      ///< Huffman table is used for coding macroblocks
    IVI_BLK_HUFF  = 1       ///< Huffman table is used for coding blocks
};


/**
 *  Common scan patterns (defined in ivi_common.c)
 */
extern const uint8_t ff_ivi_vertical_scan_8x8[64];
extern const uint8_t ff_ivi_horizontal_scan_8x8[64];
extern const uint8_t ff_ivi_direct_scan_4x4[16];


/**
 *  Declare inverse transform function types
 */
typedef void (InvTransformPtr)(const int32_t *in, int16_t *out, ptrdiff_t pitch, const uint8_t *flags);
typedef void (DCTransformPtr) (const int32_t *in, int16_t *out, ptrdiff_t pitch, int blk_size);


/**
 *  run-value (RLE) table descriptor
 */
typedef struct RVMapDesc {
    uint8_t     eob_sym; ///< end of block symbol
    uint8_t     esc_sym; ///< escape symbol
    uint8_t     runtab[256];
    int8_t      valtab[256];
} RVMapDesc;

extern const RVMapDesc ff_ivi_rvmap_tabs[9];


/**
 *  information for Indeo macroblock (16x16, 8x8 or 4x4)
 */
typedef struct IVIMbInfo {
    int16_t     xpos;
    int16_t     ypos;
    uint32_t    buf_offs; ///< address in the output buffer for this mb
    uint8_t     type;     ///< macroblock type: 0 - INTRA, 1 - INTER
    uint8_t     cbp;      ///< coded block pattern
    int8_t      q_delta;  ///< quant delta
    int8_t      mv_x;     ///< motion vector (x component)
    int8_t      mv_y;     ///< motion vector (y component)
    int8_t      b_mv_x;   ///< second motion vector (x component)
    int8_t      b_mv_y;   ///< second motion vector (y component)
} IVIMbInfo;


/**
 *  information for Indeo tile
 */
typedef struct IVITile {
    int         xpos;
    int         ypos;
    int         width;
    int         height;
    int         mb_size;
    int         is_empty;  ///< = 1 if this tile doesn't contain any data
    int         data_size; ///< size of the data in bytes
    int         num_MBs;   ///< number of macroblocks in this tile
    IVIMbInfo   *mbs;      ///< array of macroblock descriptors
    IVIMbInfo   *ref_mbs;  ///< ptr to the macroblock descriptors of the reference tile
} IVITile;


/**
 *  information for Indeo wavelet band
 */
typedef struct IVIBandDesc {
    int             plane;          ///< plane number this band belongs to
    int             band_num;       ///< band number
    int             width;
    int             height;
    int             aheight;        ///< aligned band height
    const uint8_t   *data_ptr;      ///< ptr to the first byte of the band data
    int             data_size;      ///< size of the band data
    int16_t         *buf;           ///< pointer to the output buffer for this band
    int16_t         *ref_buf;       ///< pointer to the reference frame buffer (for motion compensation)
    int16_t         *b_ref_buf;     ///< pointer to the second reference frame buffer (for motion compensation)
    int16_t         *bufs[4];       ///< array of pointers to the band buffers
    ptrdiff_t       pitch;          ///< pitch associated with the buffers above
    int             is_empty;       ///< = 1 if this band doesn't contain any data
    int             mb_size;        ///< macroblock size
    int             blk_size;       ///< block size
    int             is_halfpel;     ///< precision of the motion compensation: 0 - fullpel, 1 - halfpel
    int             inherit_mv;     ///< tells if motion vector is inherited from reference macroblock
    int             inherit_qdelta; ///< tells if quantiser delta is inherited from reference macroblock
    int             qdelta_present; ///< tells if Qdelta signal is present in the bitstream (Indeo5 only)
    int             quant_mat;      ///< dequant matrix index
    int             glob_quant;     ///< quant base for this band
    const uint8_t   *scan;          ///< ptr to the scan pattern
    int             scan_size;      ///< size of the scantable

    IVIHuffTab      blk_vlc;        ///< vlc table for decoding block data

    int             num_corr;       ///< number of correction entries
    uint8_t         corr[61*2];     ///< rvmap correction pairs
    int             rvmap_sel;      ///< rvmap table selector
    RVMapDesc       *rv_map;        ///< ptr to the RLE table for this band
    int             num_tiles;      ///< number of tiles in this band
    IVITile         *tiles;         ///< array of tile descriptors
    InvTransformPtr *inv_transform;
    int             transform_size;
    DCTransformPtr  *dc_transform;
    int             is_2d_trans;    ///< 1 indicates that the two-dimensional inverse transform is used
    int32_t         checksum;       ///< for debug purposes
    int             checksum_present;
    int             bufsize;        ///< band buffer size in bytes
    const uint16_t  *intra_base;    ///< quantization matrix for intra blocks
    const uint16_t  *inter_base;    ///< quantization matrix for inter blocks
    const uint8_t   *intra_scale;   ///< quantization coefficient for intra blocks
    const uint8_t   *inter_scale;   ///< quantization coefficient for inter blocks
} IVIBandDesc;


/**
 *  color plane (luma or chroma) information
 */
typedef struct IVIPlaneDesc {
    uint16_t    width;
    uint16_t    height;
    uint8_t     num_bands;  ///< number of bands this plane subdivided into
    IVIBandDesc *bands;     ///< array of band descriptors
} IVIPlaneDesc;


typedef struct IVIPicConfig {
    uint16_t    pic_width;
    uint16_t    pic_height;
    uint16_t    chroma_width;
    uint16_t    chroma_height;
    uint16_t    tile_width;
    uint16_t    tile_height;
    uint8_t     luma_bands;
    uint8_t     chroma_bands;
} IVIPicConfig;

typedef struct IVI45DecContext {
    GetBitContext   gb;
    RVMapDesc       rvmap_tabs[9];   ///< local corrected copy of the static rvmap tables

    uint32_t        frame_num;
    int             frame_type;
    int             prev_frame_type; ///< frame type of the previous frame
    uint32_t        data_size;       ///< size of the frame data in bytes from picture header
    int             is_scalable;
    const uint8_t   *frame_data;     ///< input frame data pointer
    int             inter_scal;      ///< signals a sequence of scalable inter frames
    uint32_t        frame_size;      ///< frame size in bytes
    uint32_t        pic_hdr_size;    ///< picture header size in bytes
    uint8_t         frame_flags;
    uint16_t        checksum;        ///< frame checksum

    IVIPicConfig    pic_conf;
    IVIPlaneDesc    planes[3];       ///< color planes

    int             buf_switch;      ///< used to switch between three buffers
    int             dst_buf;         ///< buffer index for the currently decoded frame
    int             ref_buf;         ///< inter frame reference buffer index
    int             ref2_buf;        ///< temporal storage for switching buffers
    int             b_ref_buf;       ///< second reference frame buffer index

    IVIHuffTab      mb_vlc;          ///< current macroblock table descriptor
    IVIHuffTab      blk_vlc;         ///< current block table descriptor

    uint8_t         rvmap_sel;
    uint8_t         in_imf;
    uint8_t         in_q;            ///< flag for explicitly stored quantiser delta
    uint8_t         pic_glob_quant;
    uint8_t         unknown1;

    uint16_t        gop_hdr_size;
    uint8_t         gop_flags;
    uint32_t        lock_word;

    int             show_indeo4_info;
    uint8_t         has_b_frames;
    uint8_t         has_transp;      ///< transparency mode status: 1 - enabled
    uint8_t         uses_tiling;
    uint8_t         uses_haar;
    uint8_t         uses_fullpel;

    int             (*decode_pic_hdr)  (struct IVI45DecContext *ctx, AVCodecContext *avctx);
    int             (*decode_band_hdr) (struct IVI45DecContext *ctx, IVIBandDesc *band, AVCodecContext *avctx);
    int             (*decode_mb_info)  (struct IVI45DecContext *ctx, IVIBandDesc *band, IVITile *tile, AVCodecContext *avctx);
    void            (*switch_buffers)  (struct IVI45DecContext *ctx);
    int             (*is_nonnull_frame)(struct IVI45DecContext *ctx);

    int gop_invalid;
    int buf_invalid[4];

    int is_indeo4;

    AVFrame         *p_frame;
    int             got_p_frame;
} IVI45DecContext;

/** compare some properties of two pictures */
static inline int ivi_pic_config_cmp(IVIPicConfig *str1, IVIPicConfig *str2)
{
    return str1->pic_width    != str2->pic_width    || str1->pic_height    != str2->pic_height    ||
           str1->chroma_width != str2->chroma_width || str1->chroma_height != str2->chroma_height ||
           str1->tile_width   != str2->tile_width   || str1->tile_height   != str2->tile_height   ||
           str1->luma_bands   != str2->luma_bands   || str1->chroma_bands  != str2->chroma_bands;
}

/** calculate number of tiles in a stride */
#define IVI_NUM_TILES(stride, tile_size) (((stride) + (tile_size) - 1) / (tile_size))

/** calculate number of macroblocks in a tile */
#define IVI_MBs_PER_TILE(tile_width, tile_height, mb_size) \
    ((((tile_width) + (mb_size) - 1) / (mb_size)) * (((tile_height) + (mb_size) - 1) / (mb_size)))

/** convert unsigned values into signed ones (the sign is in the LSB) */
#define IVI_TOSIGNED(val) (-(((val) >> 1) ^ -((val) & 1)))

/** scale motion vector */
static inline int ivi_scale_mv(int mv, int mv_scale)
{
    return (mv + (mv > 0) + (mv_scale - 1)) >> mv_scale;
}

/**
 * Initialize static codes used for macroblock and block decoding.
 */
void ff_ivi_init_static_vlc(void);

/**
 *  Decode a huffman codebook descriptor from the bitstream
 *  and select specified huffman table.
 *
 *  @param[in,out]  gb          the GetBit context
 *  @param[in]      desc_coded  flag signalling if table descriptor was coded
 *  @param[in]      which_tab   codebook purpose (IVI_MB_HUFF or IVI_BLK_HUFF)
 *  @param[out]     huff_tab    pointer to the descriptor of the selected table
 *  @param[in]      avctx       AVCodecContext pointer
 *  @return             zero on success, negative value otherwise
 */
int  ff_ivi_dec_huff_desc(GetBitContext *gb, int desc_coded, int which_tab,
                          IVIHuffTab *huff_tab, AVCodecContext *avctx);

/**
 *  Initialize planes (prepares descriptors, allocates buffers etc).
 *
 *  @param[in,out]  planes     pointer to the array of the plane descriptors
 *  @param[in]      cfg        pointer to the ivi_pic_config structure describing picture layout
 *  @param[in]      is_indeo4  flag signalling if it is Indeo 4 or not
 *  @return             result code: 0 - OK
 */
int  ff_ivi_init_planes(AVCodecContext *avctx, IVIPlaneDesc *planes,
                        const IVIPicConfig *cfg, int is_indeo4);

/**
 *  Initialize tile and macroblock descriptors.
 *
 *  @param[in,out]  planes       pointer to the array of the plane descriptors
 *  @param[in]      tile_width   tile width
 *  @param[in]      tile_height  tile height
 *  @return             result code: 0 - OK
 */
int  ff_ivi_init_tiles(IVIPlaneDesc *planes, int tile_width, int tile_height);

int ff_ivi_decode_frame(AVCodecContext *avctx, AVFrame *data,
                        int *got_frame, AVPacket *avpkt);
int ff_ivi_decode_close(AVCodecContext *avctx);

#endif /* AVCODEC_IVI_H */
