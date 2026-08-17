/*
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

#ifndef AVCODEC_VLC_H
#define AVCODEC_VLC_H

#include <stddef.h>
#include <stdint.h>

#include "libavutil/macros.h"

#define VLC_MULTI_MAX_SYMBOLS 6

// When changing this, be sure to also update tableprint_vlc.h accordingly.
typedef int16_t VLCBaseType;

typedef struct VLCElem {
    union {
        /// The struct is for use as ordinary VLC (with get_vlc2())
        struct {
            VLCBaseType sym;
            VLCBaseType len;
        };
        /// This struct is for use as run-length VLC (with GET_RL_VLC)
        struct {
            int16_t level;
            int8_t   len8;
            uint8_t   run;
        };
    };
} VLCElem;

typedef VLCElem RL_VLC_ELEM;

typedef struct VLC {
    int bits;
    VLCElem *table;
    int table_size, table_allocated;
} VLC;

typedef struct VLC_MULTI_ELEM {
    union {
        uint8_t   val8[VLC_MULTI_MAX_SYMBOLS];
        uint16_t val16[VLC_MULTI_MAX_SYMBOLS / 2];
    };
    int8_t len; // -31,32
    uint8_t num;
} VLC_MULTI_ELEM;

typedef struct VLC_MULTI {
    VLC_MULTI_ELEM *table;
    int table_size, table_allocated;
} VLC_MULTI;

#define vlc_init(vlc, nb_bits, nb_codes,                \
                 bits, bits_wrap, bits_size,            \
                 codes, codes_wrap, codes_size,         \
                 flags)                                 \
    ff_vlc_init_sparse(vlc, nb_bits, nb_codes,          \
                       bits, bits_wrap, bits_size,      \
                       codes, codes_wrap, codes_size,   \
                       NULL, 0, 0, flags)

/**
 * Build VLC decoding tables suitable for use with get_vlc2().
 *
 * @param[in,out] vlc      The VLC to be initialized; table and table_allocated
 *                         must have been set when initializing a static VLC,
 *                         otherwise this will be treated as uninitialized.
 * @param[in] nb_bits      The number of bits to use for the VLC table;
 *                         higher values take up more memory and cache, but
 *                         allow to read codes with fewer reads.
 *                         Corresponds to the `bits` parameter of get_vlc2().
 * @param[in] nb_codes     The number of provided bits, codes and (if supplied)
 *                         symbol entries.
 * @param[in] bits         The lengths (in bits) of the codes. Entries > 0
 *                         correspond to valid codes; entries == 0 will be skipped.
 * @param[in] bits_wrap    Stride (in bytes) of the bits table.
 * @param[in] codes_size   Size of the bits. 1, 2 and 4 are supported.
 * @param[in] codes        Table which gives the bit pattern of of each vlc code.
 * @param[in] codes_wrap   Stride (in bytes) of the codes table.
 * @param[in] codes_size   Size of the codes. 1, 2 and 4 are supported.
 * @param[in] symbols      The symbols, i.e. what is returned from get_vlc2()
 *                         when the corresponding code is encountered.
 *                         May be NULL, then 0, 1, 2, 3, 4,... will be used.
 * @param[in] symbols_wrap Stride (in bytes) of the symbols table.
 * @param[in] symbols_size Size of the symbols. 1 and 2 are supported.
 * @param[in] flags        A combination of the VLC_INIT_* flags.
 *
 * 'wrap' and 'size' make it possible to use any memory configuration and types
 * (byte/word/int) to store the 'bits', 'codes', and 'symbols' tables.
 */
int ff_vlc_init_sparse(VLC *vlc, int nb_bits, int nb_codes,
                       const void *bits, int bits_wrap, int bits_size,
                       const void *codes, int codes_wrap, int codes_size,
                       const void *symbols, int symbols_wrap, int symbols_size,
                       int flags);

/**
 * Build VLC decoding tables suitable for use with get_vlc2()
 *
 * This function takes lengths and symbols and calculates the codes from them.
 * For this the input lengths and symbols have to be sorted according to "left
 * nodes in the corresponding tree first".
 *
 * @param[in,out] vlc      The VLC to be initialized; table and table_allocated
 *                         must have been set when initializing a static VLC,
 *                         otherwise this will be treated as uninitialized.
 * @param[in] nb_bits      The number of bits to use for the VLC table;
 *                         higher values take up more memory and cache, but
 *                         allow to read codes with fewer reads.
 * @param[in] nb_codes     The number of provided length and (if supplied) symbol
 *                         entries.
 * @param[in] lens         The lengths of the codes. Entries > 0 correspond to
 *                         valid codes; entries == 0 will be skipped and entries
 *                         with len < 0 indicate that the tree is incomplete and
 *                         has an open end of length -len at this position.
 * @param[in] lens_wrap    Stride (in bytes) of the lengths.
 * @param[in] symbols      The symbols, i.e. what is returned from get_vlc2()
 *                         when the corresponding code is encountered.
 *                         May be NULL, then 0, 1, 2, 3, 4,... will be used.
 * @param[in] symbols_wrap Stride (in bytes) of the symbols.
 * @param[in] symbols_size Size of the symbols. 1 and 2 are supported.
 * @param[in] offset       An offset to apply to all the valid symbols.
 * @param[in] flags        A combination of the VLC_INIT_* flags; notice that
 *                         VLC_INIT_INPUT_LE is pointless and ignored.
 */
int ff_vlc_init_from_lengths(VLC *vlc, int nb_bits, int nb_codes,
                             const int8_t *lens, int lens_wrap,
                             const void *symbols, int symbols_wrap, int symbols_size,
                             int offset, int flags, void *logctx);

/**
 * Build VLC decoding tables suitable for use with get_vlc_multi()
 *
 * This function takes lengths and symbols and calculates the codes from them.
 * For this the input lengths and symbols have to be sorted according to "left
 * nodes in the corresponding tree first".
 *
 * @param[in,out] vlc      The VLC to be initialized; table and table_allocated
 *                         must have been set when initializing a static VLC,
 *                         otherwise this will be treated as uninitialized.
 * @param[in,out] multi    The VLC_MULTI to be initialized; table and table_allocated
 *                         must have been set when initializing a static VLC,
 *                         otherwise this will be treated as uninitialized.
 * @param[in] nb_bits      The number of bits to use for the VLC table;
 *                         higher values take up more memory and cache, but
 *                         allow to read codes with fewer reads.
 * @param[in] nb_elems     The max possible number of elements.
 * @param[in] nb_codes     The number of provided length and (if supplied) symbol
 *                         entries.
 * @param[in] lens         The lengths of the codes. Entries > 0 correspond to
 *                         valid codes; entries == 0 will be skipped and entries
 *                         with len < 0 indicate that the tree is incomplete and
 *                         has an open end of length -len at this position.
 * @param[in] lens_wrap    Stride (in bytes) of the lengths.
 * @param[in] symbols      The symbols, i.e. what is returned from get_vlc2()
 *                         when the corresponding code is encountered.
 *                         May be NULL, then 0, 1, 2, 3, 4,... will be used.
 * @param[in] symbols_wrap Stride (in bytes) of the symbols.
 * @param[in] symbols_size Size of the symbols. 1 and 2 are supported.
 * @param[in] offset       An offset to apply to all the valid symbols.
 * @param[in] flags        A combination of the VLC_INIT_* flags; notice that
 *                         VLC_INIT_INPUT_LE is pointless and ignored.
 */
int ff_vlc_init_multi_from_lengths(VLC *vlc, VLC_MULTI *multi, int nb_bits, int nb_elems,
                                   int nb_codes, const int8_t *lens, int lens_wrap,
                                   const void *symbols, int symbols_wrap, int symbols_size,
                                   int offset, int flags, void *logctx);


void ff_vlc_free_multi(VLC_MULTI *vlc);
void ff_vlc_free(VLC *vlc);

#define VLC_INIT_USE_STATIC     1
#define VLC_INIT_STATIC_OVERLONG (2 | VLC_INIT_USE_STATIC)
/* If VLC_INIT_INPUT_LE is set, the LSB bit of the codes used to
 * initialize the VLC table is the first bit to be read. */
#define VLC_INIT_INPUT_LE       4
/* If set the VLC is intended for a little endian bitstream reader. */
#define VLC_INIT_OUTPUT_LE      8
#define VLC_INIT_LE             (VLC_INIT_INPUT_LE | VLC_INIT_OUTPUT_LE)

/**
 * For static VLCs, the number of bits can often be hardcoded
 * at each get_vlc2() callsite. Then using a full VLC would be uneconomical,
 * because only VLC.table would ever be accessed after initialization.
 * The following functions provide wrappers around the relevant ff_vlc_init_*
 * functions suitable for said task.
 *
 * The ff_vlc_init_tables_* functions are intended to be used for initializing
 * a series of VLCs. The user initializes a VLCInitState with the details
 * about the underlying array of VLCElem; it is automatically updated by
 * the ff_vlc_init_tables_* functions (i.e. table is incremented and size
 * decremented by the number of elements of the current table).
 * The VLC_INIT_STATIC_OVERLONG flag is also automatically added.
 * These functions return a pointer to the table just initialized,
 * potentially to be used in arrays of pointer to VLC tables.
 *
 * The ff_vlc_init_table_* functions are intended to be used for initializing
 * a single VLC table, given by table and table_size. The VLC_INIT_USE_STATIC
 * flag is automatically added.
 */

typedef struct VLCInitState {
    VLCElem *table;  ///< points to where the next VLC table will be placed
    unsigned size;   ///< remaining number of elements in table
} VLCInitState;

#define VLC_INIT_STATE(_table) { .table = (_table), .size = FF_ARRAY_ELEMS(_table) }

void ff_vlc_init_table_from_lengths(VLCElem table[], int table_size,
                                    int nb_bits, int nb_codes,
                                    const int8_t *lens, int lens_wrap,
                                    const void *symbols, int symbols_wrap, int symbols_size,
                                    int offset, int flags);

const VLCElem *ff_vlc_init_tables_from_lengths(VLCInitState *state,
                                               int nb_bits, int nb_codes,
                                               const int8_t *lens, int lens_wrap,
                                               const void *symbols, int symbols_wrap, int symbols_size,
                                               int offset, int flags);

void ff_vlc_init_table_sparse(VLCElem table[], int table_size,
                              int nb_bits, int nb_codes,
                              const void *bits, int bits_wrap, int bits_size,
                              const void *codes, int codes_wrap, int codes_size,
                              const void *symbols, int symbols_wrap, int symbols_size,
                              int flags);

const VLCElem *ff_vlc_init_tables_sparse(VLCInitState *state,
                                         int nb_bits, int nb_codes,
                                         const void *bits, int bits_wrap, int bits_size,
                                         const void *codes, int codes_wrap, int codes_size,
                                         const void *symbols, int symbols_wrap, int symbols_size,
                                         int flags);

static inline
const VLCElem *ff_vlc_init_tables(VLCInitState *state,
                                  int nb_bits, int nb_codes,
                                  const void *bits, int bits_wrap, int bits_size,
                                  const void *codes, int codes_wrap, int codes_size,
                                  int flags)
{
    return ff_vlc_init_tables_sparse(state, nb_bits, nb_codes,
                                     bits, bits_wrap, bits_size,
                                     codes, codes_wrap, codes_size,
                                     NULL, 0, 0, flags);
}

#define VLC_INIT_STATIC_SPARSE_TABLE(vlc_table, nb_bits, nb_codes,         \
                                     bits, bits_wrap, bits_size,           \
                                     codes, codes_wrap, codes_size,        \
                                     symbols, symbols_wrap, symbols_size,  \
                                     flags)                                \
    ff_vlc_init_table_sparse(vlc_table, FF_ARRAY_ELEMS(vlc_table),         \
                             (nb_bits), (nb_codes),                        \
                             (bits), (bits_wrap), (bits_size),             \
                             (codes), (codes_wrap), (codes_size),          \
                             (symbols), (symbols_wrap), (symbols_size),    \
                             (flags))

#define VLC_INIT_STATIC_TABLE(vlc_table, nb_bits, nb_codes,                \
                              bits, bits_wrap, bits_size,                  \
                              codes, codes_wrap, codes_size,               \
                              flags)                                       \
    ff_vlc_init_table_sparse(vlc_table, FF_ARRAY_ELEMS(vlc_table),         \
                             (nb_bits), (nb_codes),                        \
                             (bits), (bits_wrap), (bits_size),             \
                             (codes), (codes_wrap), (codes_size),          \
                             NULL, 0, 0, (flags))

#define VLC_INIT_STATIC_TABLE_FROM_LENGTHS(vlc_table, nb_bits, nb_codes,   \
                                           lens, lens_wrap,                \
                                           syms, syms_wrap, syms_size,     \
                                           offset, flags)                  \
    ff_vlc_init_table_from_lengths(vlc_table, FF_ARRAY_ELEMS(vlc_table),   \
                                   (nb_bits), (nb_codes),                  \
                                   (lens), (lens_wrap),                    \
                                   (syms), (syms_wrap), (syms_size),       \
                                   (offset), (flags))

#endif /* AVCODEC_VLC_H */
