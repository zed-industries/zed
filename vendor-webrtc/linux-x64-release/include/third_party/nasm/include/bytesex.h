/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2017 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

/*
 * bytesex.h - byte order helper functions
 *
 * In this function, be careful about getting X86_MEMORY versus
 * LITTLE_ENDIAN correct: X86_MEMORY also means we are allowed to
 * do unaligned memory references, and is probabilistic.
 */

#ifndef NASM_BYTEORD_H
#define NASM_BYTEORD_H

#include "compiler.h"

/*
 * Some handy macros that will probably be of use in more than one
 * output format: convert integers into little-endian byte packed
 * format in memory.
 */

#define WRITECHAR(p,v)                          	\
    do {                                        	\
        uint8_t *_wc_p = (uint8_t *)(p);		\
        *_wc_p++ = (v);                                 \
        (p) = (void *)_wc_p;                            \
    } while (0)

#if X86_MEMORY

#define WRITESHORT(p,v)                         	\
    do {                                        	\
        uint16_t *_ws_p = (uint16_t *)(p);      	\
        *_ws_p++ = (v);                           	\
        (p) = (void *)_ws_p;              		\
    } while (0)

#define WRITELONG(p,v)                          	\
    do {                                        	\
        uint32_t *_wl_p = (uint32_t *)(p);		\
        *_wl_p++ = (v);                           	\
        (p) = (void *)_wl_p;              		\
    } while (0)

#define WRITEDLONG(p,v)                         	\
    do {                                        	\
        uint64_t *_wq_p = (uint64_t *)(p);      	\
        *_wq_p++ = (v);                           	\
        (p) = (void *)_wq_p;              		\
    } while (0)

#else /* !X86_MEMORY */

#define WRITESHORT(p,v)                         	\
    do {                                        	\
        uint8_t *_ws_p = (uint8_t *)(p);        	\
        const uint16_t _ws_v = (v);                     \
        WRITECHAR(_ws_p, _ws_v);                        \
        WRITECHAR(_ws_p, _ws_v >> 8);                   \
        (p) = (void *)_ws_p;                            \
    } while (0)

#define WRITELONG(p,v)                         		\
    do {                                        	\
        uint8_t *_wl_p = (uint8_t *)(p);        	\
        const uint32_t _wl_v = (v);                     \
        WRITESHORT(_wl_p, _wl_v);                       \
        WRITESHORT(_wl_p, _wl_v >> 16);                 \
        (p) = (void *)_wl_p;                            \
    } while (0)

#define WRITEDLONG(p,v)                         	\
    do {                                        	\
        uint8_t *_wq_p = (uint8_t *)(p);        	\
        const uint64_t _wq_v = (v);                     \
        WRITELONG(_wq_p, _wq_v);                        \
        WRITELONG(_wq_p, _wq_v >> 32);                  \
        (p) = (void *)_wq_p;                            \
    } while (0)

#endif /* X86_MEMORY */

/*
 * Endian control functions which work on a single integer
 */
#ifdef WORDS_LITTLEENDIAN

#ifndef HAVE_CPU_TO_LE16
# define cpu_to_le16(v) ((uint16_t)(v))
#endif
#ifndef HAVE_CPU_TO_LE32
# define cpu_to_le32(v) ((uint32_t)(v))
#endif
#ifndef HAVE_CPU_TO_LE64
# define cpu_to_le64(v) ((uint64_t)(v))
#endif

#elif defined(WORDS_BIGENDIAN)

#ifndef HAVE_CPU_TO_LE16
static inline uint16_t cpu_to_le16(uint16_t v)
{
# ifdef HAVE___CPU_TO_LE16
    return __cpu_to_le16(v);
# elif defined(HAVE_HTOLE16)
    return htole16(v);
# elif defined(HAVE___BSWAP_16)
    return __bswap_16(v);
# elif defined(HAVE___BUILTIN_BSWAP16)
    return __builtin_bswap16(v);
# elif defined(HAVE__BYTESWAP_USHORT) && (USHRT_MAX == 0xffffU)
    return _byteswap_ushort(v);
# else
    return (v << 8) | (v >> 8);
# endif
}
#endif

#ifndef HAVE_CPU_TO_LE32
static inline uint32_t cpu_to_le32(uint32_t v)
{
# ifdef HAVE___CPU_TO_LE32
    return __cpu_to_le32(v);
# elif defined(HAVE_HTOLE32)
    return htole32(v);
# elif defined(HAVE___BSWAP_32)
    return __bswap_32(v);
# elif defined(HAVE___BUILTIN_BSWAP32)
    return __builtin_bswap32(v);
# elif defined(HAVE__BYTESWAP_ULONG) && (ULONG_MAX == 0xffffffffUL)
    return _byteswap_ulong(v);
# else
    v = ((v << 8) & 0xff00ff00 ) |
        ((v >> 8) & 0x00ff00ff);
    return (v << 16) | (v >> 16);
# endif
}
#endif

#ifndef HAVE_CPU_TO_LE64
static inline uint64_t cpu_to_le64(uint64_t v)
{
# ifdef HAVE___CPU_TO_LE64
    return __cpu_to_le64(v);
# elif defined(HAVE_HTOLE64)
    return htole64(v);
# elif defined(HAVE___BSWAP_64)
    return __bswap_64(v);
# elif defined(HAVE___BUILTIN_BSWAP64)
    return __builtin_bswap64(v);
# elif defined(HAVE__BYTESWAP_UINT64)
    return _byteswap_uint64(v);
# else
    v = ((v << 8) & 0xff00ff00ff00ff00ull) |
        ((v >> 8) & 0x00ff00ff00ff00ffull);
    v = ((v << 16) & 0xffff0000ffff0000ull) |
        ((v >> 16) & 0x0000ffff0000ffffull);
    return (v << 32) | (v >> 32);
# endif
}
#endif

#else /* not WORDS_LITTLEENDIAN or WORDS_BIGENDIAN */

static inline uint16_t cpu_to_le16(uint16_t v)
{
    union u16 {
        uint16_t v;
        uint8_t c[2];
    } x;
    uint8_t *cp = &x.c;

    WRITESHORT(cp, v);
    return x.v;
}

static inline uint32_t cpu_to_le32(uint32_t v)
{
    union u32 {
        uint32_t v;
        uint8_t c[4];
    } x;
    uint8_t *cp = &x.c;

    WRITELONG(cp, v);
    return x.v;
}

static inline uint64_t cpu_to_le64(uint64_t v)
{
    union u64 {
        uint64_t v;
        uint8_t c[8];
    } x;
    uint8_t *cp = &x.c;

    WRITEDLONG(cp, v);
    return x.v;
}

#endif

#define WRITEADDR(p,v,s)                        		\
    do {                                                        \
	switch (is_constant(s) ? (s) : 0) {                     \
        case 1:                                                 \
            WRITECHAR(p,v);                                     \
            break;                                              \
        case 2:                                                 \
            WRITESHORT(p,v);                                    \
            break;                                              \
        case 4:                                                 \
            WRITELONG(p,v);                                     \
            break;                                              \
	case 8:                                                 \
            WRITEDLONG(p,v);                                    \
            break;                                              \
        default:                                                \
        {                                                       \
            const uint64_t _wa_v = cpu_to_le64(v);              \
            const size_t _wa_s = (s);                           \
            uint8_t * const _wa_p = (uint8_t *)(p);             \
            memcpy(_wa_p, &_wa_v, _wa_s);                       \
            (p) = (void *)(_wa_p + _wa_s);                      \
        }                                                       \
        break;                                                  \
        }                                                       \
    } while (0)

#endif /* NASM_BYTESEX_H */
