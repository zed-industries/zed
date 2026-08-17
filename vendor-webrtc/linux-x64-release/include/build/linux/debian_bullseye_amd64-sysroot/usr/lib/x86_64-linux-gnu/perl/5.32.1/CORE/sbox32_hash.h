#ifndef DEBUG_SBOX32_HASH
#define DEBUG_SBOX32_HASH 0

#include "zaphod32_hash.h"

#if DEBUG_SBOX32_HASH == 1
#include <stdio.h>
#define SBOX32_WARN6(pat,v0,v1,v2,v3,v4,v5)    printf(pat, v0, v1, v2, v3, v4, v5)
#define SBOX32_WARN5(pat,v0,v1,v2,v3,v4)       printf(pat, v0, v1, v2, v3, v4)
#define SBOX32_WARN4(pat,v0,v1,v2,v3)          printf(pat, v0, v1, v2, v3)
#define SBOX32_WARN3(pat,v0,v1,v2)             printf(pat, v0, v1, v2)
#define SBOX32_WARN2(pat,v0,v1)                printf(pat, v0, v1)
#define NOTE3(pat,v0,v1,v2)             printf(pat, v0, v1, v2)
#elif DEBUG_SBOX32_HASH == 2
#define SBOX32_WARN6(pat,v0,v1,v2,v3,v4,v5)
#define SBOX32_WARN5(pat,v0,v1,v2,v3,v4)
#define SBOX32_WARN4(pat,v0,v1,v2,v3)
#define SBOX32_WARN3(pat,v0,v1,v2)
#define SBOX32_WARN2(pat,v0,v1)
#define NOTE3(pat,v0,v1,v2)             printf(pat, v0, v1, v2)
#else
#define SBOX32_WARN6(pat,v0,v1,v2,v3,v4,v5)
#define SBOX32_WARN5(pat,v0,v1,v2,v3,v4)
#define SBOX32_WARN4(pat,v0,v1,v2,v3)
#define SBOX32_WARN3(pat,v0,v1,v2)
#define NOTE3(pat,v0,v1,v2)
#define SBOX32_WARN2(pat,v0,v1)
#endif

#ifndef PERL_SEEN_HV_FUNC_H
#if !defined(U32) 
#include <stdint.h>
#define U32 uint32_t
#endif

#if !defined(U8)
#define U8 unsigned char
#endif

#if !defined(U16)
#define U16 uint16_t
#endif

#ifndef STRLEN
#define STRLEN int
#endif
#endif

#ifndef SBOX32_STATIC_INLINE
#ifdef PERL_STATIC_INLINE
#define SBOX32_STATIC_INLINE PERL_STATIC_INLINE
#else
#define SBOX32_STATIC_INLINE static inline
#endif
#endif

#ifndef STMT_START
#define STMT_START do
#define STMT_END while(0)
#endif

/* Find best way to ROTL32/ROTL64 */
#ifndef ROTL32
#if defined(_MSC_VER)
#include <stdlib.h>  /* Microsoft put _rotl declaration in here */
#define ROTL32(x,r)  _rotl(x,r)
#define ROTR32(x,r)  _rotr(x,r)
#else
/* gcc recognises this code and generates a rotate instruction for CPUs with one */
#define ROTL32(x,r)  (((U32)(x) << (r)) | ((U32)(x) >> (32 - (r))))
#define ROTR32(x,r)  (((U32)(x) << (32 - (r))) | ((U32)(x) >> (r)))
#endif
#endif

#ifndef SBOX32_MAX_LEN
#define SBOX32_MAX_LEN 256
#endif

#ifndef SBOX32_STATE_WORDS
#define SBOX32_STATE_WORDS (1 + (SBOX32_MAX_LEN * 256))
#define SBOX32_STATE_BYTES (SBOX32_STATE_WORDS * sizeof(U32))
#define SBOX32_STATE_BITS (SBOX32_STATE_BYTES * 8)
#endif

#define SBOX32_MIX4(v0,v1,v2,v3,text) STMT_START { \
        SBOX32_WARN5("v0=%08x v1=%08x v2=%08x v3=%08x - SBOX32_MIX4 %s\n", \
                            (unsigned int)v0, (unsigned int)v1,    \
                            (unsigned int)v2, (unsigned int)v3, text);   \
        v0 = ROTL32(v0,13) - v3;    \
        v1 ^= v2;                   \
        v3 = ROTL32(v3, 9) + v1;    \
        v2 ^= v0;                   \
        v0 = ROTL32(v0,14) ^ v3;    \
        v1 = ROTL32(v1,25) - v2;    \
        v3 ^= v1;                   \
        v2 = ROTL32(v2, 4) - v0;    \
} STMT_END

#define SBOX32_MIX3(v0,v1,v2,text) STMT_START {                               \
    SBOX32_WARN4("v0=%08x v1=%08x v2=%08x - SBOX32_MIX3 %s\n",              \
            (unsigned int)v0,(unsigned int)v1,(unsigned int)v2, text );     \
    v0 = ROTL32(v0,16) - v2;   \
    v1 = ROTR32(v1,13) ^ v2;   \
    v2 = ROTL32(v2,17) + v1;   \
    v0 = ROTR32(v0, 2) + v1;   \
    v1 = ROTR32(v1,17) - v0;   \
    v2 = ROTR32(v2, 7) ^ v0;   \
} STMT_END

#if SBOX32_MAX_LEN > 256
#error "SBOX32_MAX_LEN is set too high!"
#elif SBOX32_MAX_LEN == 256
#define case_256_SBOX32(hash,state,key) _SBOX32_CASE(256,hash,state,key)
#else
#define case_256_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 255
#define case_255_SBOX32(hash,state,key) _SBOX32_CASE(255,hash,state,key)
#else
#define case_255_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 254
#define case_254_SBOX32(hash,state,key) _SBOX32_CASE(254,hash,state,key)
#else
#define case_254_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 253
#define case_253_SBOX32(hash,state,key) _SBOX32_CASE(253,hash,state,key)
#else
#define case_253_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 252
#define case_252_SBOX32(hash,state,key) _SBOX32_CASE(252,hash,state,key)
#else
#define case_252_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 251
#define case_251_SBOX32(hash,state,key) _SBOX32_CASE(251,hash,state,key)
#else
#define case_251_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 250
#define case_250_SBOX32(hash,state,key) _SBOX32_CASE(250,hash,state,key)
#else
#define case_250_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 249
#define case_249_SBOX32(hash,state,key) _SBOX32_CASE(249,hash,state,key)
#else
#define case_249_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 248
#define case_248_SBOX32(hash,state,key) _SBOX32_CASE(248,hash,state,key)
#else
#define case_248_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 247
#define case_247_SBOX32(hash,state,key) _SBOX32_CASE(247,hash,state,key)
#else
#define case_247_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 246
#define case_246_SBOX32(hash,state,key) _SBOX32_CASE(246,hash,state,key)
#else
#define case_246_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 245
#define case_245_SBOX32(hash,state,key) _SBOX32_CASE(245,hash,state,key)
#else
#define case_245_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 244
#define case_244_SBOX32(hash,state,key) _SBOX32_CASE(244,hash,state,key)
#else
#define case_244_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 243
#define case_243_SBOX32(hash,state,key) _SBOX32_CASE(243,hash,state,key)
#else
#define case_243_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 242
#define case_242_SBOX32(hash,state,key) _SBOX32_CASE(242,hash,state,key)
#else
#define case_242_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 241
#define case_241_SBOX32(hash,state,key) _SBOX32_CASE(241,hash,state,key)
#else
#define case_241_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 240
#define case_240_SBOX32(hash,state,key) _SBOX32_CASE(240,hash,state,key)
#else
#define case_240_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 239
#define case_239_SBOX32(hash,state,key) _SBOX32_CASE(239,hash,state,key)
#else
#define case_239_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 238
#define case_238_SBOX32(hash,state,key) _SBOX32_CASE(238,hash,state,key)
#else
#define case_238_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 237
#define case_237_SBOX32(hash,state,key) _SBOX32_CASE(237,hash,state,key)
#else
#define case_237_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 236
#define case_236_SBOX32(hash,state,key) _SBOX32_CASE(236,hash,state,key)
#else
#define case_236_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 235
#define case_235_SBOX32(hash,state,key) _SBOX32_CASE(235,hash,state,key)
#else
#define case_235_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 234
#define case_234_SBOX32(hash,state,key) _SBOX32_CASE(234,hash,state,key)
#else
#define case_234_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 233
#define case_233_SBOX32(hash,state,key) _SBOX32_CASE(233,hash,state,key)
#else
#define case_233_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 232
#define case_232_SBOX32(hash,state,key) _SBOX32_CASE(232,hash,state,key)
#else
#define case_232_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 231
#define case_231_SBOX32(hash,state,key) _SBOX32_CASE(231,hash,state,key)
#else
#define case_231_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 230
#define case_230_SBOX32(hash,state,key) _SBOX32_CASE(230,hash,state,key)
#else
#define case_230_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 229
#define case_229_SBOX32(hash,state,key) _SBOX32_CASE(229,hash,state,key)
#else
#define case_229_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 228
#define case_228_SBOX32(hash,state,key) _SBOX32_CASE(228,hash,state,key)
#else
#define case_228_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 227
#define case_227_SBOX32(hash,state,key) _SBOX32_CASE(227,hash,state,key)
#else
#define case_227_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 226
#define case_226_SBOX32(hash,state,key) _SBOX32_CASE(226,hash,state,key)
#else
#define case_226_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 225
#define case_225_SBOX32(hash,state,key) _SBOX32_CASE(225,hash,state,key)
#else
#define case_225_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 224
#define case_224_SBOX32(hash,state,key) _SBOX32_CASE(224,hash,state,key)
#else
#define case_224_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 223
#define case_223_SBOX32(hash,state,key) _SBOX32_CASE(223,hash,state,key)
#else
#define case_223_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 222
#define case_222_SBOX32(hash,state,key) _SBOX32_CASE(222,hash,state,key)
#else
#define case_222_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 221
#define case_221_SBOX32(hash,state,key) _SBOX32_CASE(221,hash,state,key)
#else
#define case_221_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 220
#define case_220_SBOX32(hash,state,key) _SBOX32_CASE(220,hash,state,key)
#else
#define case_220_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 219
#define case_219_SBOX32(hash,state,key) _SBOX32_CASE(219,hash,state,key)
#else
#define case_219_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 218
#define case_218_SBOX32(hash,state,key) _SBOX32_CASE(218,hash,state,key)
#else
#define case_218_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 217
#define case_217_SBOX32(hash,state,key) _SBOX32_CASE(217,hash,state,key)
#else
#define case_217_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 216
#define case_216_SBOX32(hash,state,key) _SBOX32_CASE(216,hash,state,key)
#else
#define case_216_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 215
#define case_215_SBOX32(hash,state,key) _SBOX32_CASE(215,hash,state,key)
#else
#define case_215_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 214
#define case_214_SBOX32(hash,state,key) _SBOX32_CASE(214,hash,state,key)
#else
#define case_214_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 213
#define case_213_SBOX32(hash,state,key) _SBOX32_CASE(213,hash,state,key)
#else
#define case_213_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 212
#define case_212_SBOX32(hash,state,key) _SBOX32_CASE(212,hash,state,key)
#else
#define case_212_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 211
#define case_211_SBOX32(hash,state,key) _SBOX32_CASE(211,hash,state,key)
#else
#define case_211_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 210
#define case_210_SBOX32(hash,state,key) _SBOX32_CASE(210,hash,state,key)
#else
#define case_210_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 209
#define case_209_SBOX32(hash,state,key) _SBOX32_CASE(209,hash,state,key)
#else
#define case_209_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 208
#define case_208_SBOX32(hash,state,key) _SBOX32_CASE(208,hash,state,key)
#else
#define case_208_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 207
#define case_207_SBOX32(hash,state,key) _SBOX32_CASE(207,hash,state,key)
#else
#define case_207_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 206
#define case_206_SBOX32(hash,state,key) _SBOX32_CASE(206,hash,state,key)
#else
#define case_206_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 205
#define case_205_SBOX32(hash,state,key) _SBOX32_CASE(205,hash,state,key)
#else
#define case_205_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 204
#define case_204_SBOX32(hash,state,key) _SBOX32_CASE(204,hash,state,key)
#else
#define case_204_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 203
#define case_203_SBOX32(hash,state,key) _SBOX32_CASE(203,hash,state,key)
#else
#define case_203_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 202
#define case_202_SBOX32(hash,state,key) _SBOX32_CASE(202,hash,state,key)
#else
#define case_202_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 201
#define case_201_SBOX32(hash,state,key) _SBOX32_CASE(201,hash,state,key)
#else
#define case_201_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 200
#define case_200_SBOX32(hash,state,key) _SBOX32_CASE(200,hash,state,key)
#else
#define case_200_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 199
#define case_199_SBOX32(hash,state,key) _SBOX32_CASE(199,hash,state,key)
#else
#define case_199_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 198
#define case_198_SBOX32(hash,state,key) _SBOX32_CASE(198,hash,state,key)
#else
#define case_198_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 197
#define case_197_SBOX32(hash,state,key) _SBOX32_CASE(197,hash,state,key)
#else
#define case_197_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 196
#define case_196_SBOX32(hash,state,key) _SBOX32_CASE(196,hash,state,key)
#else
#define case_196_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 195
#define case_195_SBOX32(hash,state,key) _SBOX32_CASE(195,hash,state,key)
#else
#define case_195_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 194
#define case_194_SBOX32(hash,state,key) _SBOX32_CASE(194,hash,state,key)
#else
#define case_194_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 193
#define case_193_SBOX32(hash,state,key) _SBOX32_CASE(193,hash,state,key)
#else
#define case_193_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 192
#define case_192_SBOX32(hash,state,key) _SBOX32_CASE(192,hash,state,key)
#else
#define case_192_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 191
#define case_191_SBOX32(hash,state,key) _SBOX32_CASE(191,hash,state,key)
#else
#define case_191_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 190
#define case_190_SBOX32(hash,state,key) _SBOX32_CASE(190,hash,state,key)
#else
#define case_190_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 189
#define case_189_SBOX32(hash,state,key) _SBOX32_CASE(189,hash,state,key)
#else
#define case_189_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 188
#define case_188_SBOX32(hash,state,key) _SBOX32_CASE(188,hash,state,key)
#else
#define case_188_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 187
#define case_187_SBOX32(hash,state,key) _SBOX32_CASE(187,hash,state,key)
#else
#define case_187_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 186
#define case_186_SBOX32(hash,state,key) _SBOX32_CASE(186,hash,state,key)
#else
#define case_186_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 185
#define case_185_SBOX32(hash,state,key) _SBOX32_CASE(185,hash,state,key)
#else
#define case_185_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 184
#define case_184_SBOX32(hash,state,key) _SBOX32_CASE(184,hash,state,key)
#else
#define case_184_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 183
#define case_183_SBOX32(hash,state,key) _SBOX32_CASE(183,hash,state,key)
#else
#define case_183_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 182
#define case_182_SBOX32(hash,state,key) _SBOX32_CASE(182,hash,state,key)
#else
#define case_182_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 181
#define case_181_SBOX32(hash,state,key) _SBOX32_CASE(181,hash,state,key)
#else
#define case_181_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 180
#define case_180_SBOX32(hash,state,key) _SBOX32_CASE(180,hash,state,key)
#else
#define case_180_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 179
#define case_179_SBOX32(hash,state,key) _SBOX32_CASE(179,hash,state,key)
#else
#define case_179_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 178
#define case_178_SBOX32(hash,state,key) _SBOX32_CASE(178,hash,state,key)
#else
#define case_178_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 177
#define case_177_SBOX32(hash,state,key) _SBOX32_CASE(177,hash,state,key)
#else
#define case_177_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 176
#define case_176_SBOX32(hash,state,key) _SBOX32_CASE(176,hash,state,key)
#else
#define case_176_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 175
#define case_175_SBOX32(hash,state,key) _SBOX32_CASE(175,hash,state,key)
#else
#define case_175_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 174
#define case_174_SBOX32(hash,state,key) _SBOX32_CASE(174,hash,state,key)
#else
#define case_174_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 173
#define case_173_SBOX32(hash,state,key) _SBOX32_CASE(173,hash,state,key)
#else
#define case_173_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 172
#define case_172_SBOX32(hash,state,key) _SBOX32_CASE(172,hash,state,key)
#else
#define case_172_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 171
#define case_171_SBOX32(hash,state,key) _SBOX32_CASE(171,hash,state,key)
#else
#define case_171_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 170
#define case_170_SBOX32(hash,state,key) _SBOX32_CASE(170,hash,state,key)
#else
#define case_170_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 169
#define case_169_SBOX32(hash,state,key) _SBOX32_CASE(169,hash,state,key)
#else
#define case_169_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 168
#define case_168_SBOX32(hash,state,key) _SBOX32_CASE(168,hash,state,key)
#else
#define case_168_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 167
#define case_167_SBOX32(hash,state,key) _SBOX32_CASE(167,hash,state,key)
#else
#define case_167_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 166
#define case_166_SBOX32(hash,state,key) _SBOX32_CASE(166,hash,state,key)
#else
#define case_166_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 165
#define case_165_SBOX32(hash,state,key) _SBOX32_CASE(165,hash,state,key)
#else
#define case_165_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 164
#define case_164_SBOX32(hash,state,key) _SBOX32_CASE(164,hash,state,key)
#else
#define case_164_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 163
#define case_163_SBOX32(hash,state,key) _SBOX32_CASE(163,hash,state,key)
#else
#define case_163_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 162
#define case_162_SBOX32(hash,state,key) _SBOX32_CASE(162,hash,state,key)
#else
#define case_162_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 161
#define case_161_SBOX32(hash,state,key) _SBOX32_CASE(161,hash,state,key)
#else
#define case_161_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 160
#define case_160_SBOX32(hash,state,key) _SBOX32_CASE(160,hash,state,key)
#else
#define case_160_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 159
#define case_159_SBOX32(hash,state,key) _SBOX32_CASE(159,hash,state,key)
#else
#define case_159_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 158
#define case_158_SBOX32(hash,state,key) _SBOX32_CASE(158,hash,state,key)
#else
#define case_158_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 157
#define case_157_SBOX32(hash,state,key) _SBOX32_CASE(157,hash,state,key)
#else
#define case_157_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 156
#define case_156_SBOX32(hash,state,key) _SBOX32_CASE(156,hash,state,key)
#else
#define case_156_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 155
#define case_155_SBOX32(hash,state,key) _SBOX32_CASE(155,hash,state,key)
#else
#define case_155_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 154
#define case_154_SBOX32(hash,state,key) _SBOX32_CASE(154,hash,state,key)
#else
#define case_154_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 153
#define case_153_SBOX32(hash,state,key) _SBOX32_CASE(153,hash,state,key)
#else
#define case_153_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 152
#define case_152_SBOX32(hash,state,key) _SBOX32_CASE(152,hash,state,key)
#else
#define case_152_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 151
#define case_151_SBOX32(hash,state,key) _SBOX32_CASE(151,hash,state,key)
#else
#define case_151_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 150
#define case_150_SBOX32(hash,state,key) _SBOX32_CASE(150,hash,state,key)
#else
#define case_150_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 149
#define case_149_SBOX32(hash,state,key) _SBOX32_CASE(149,hash,state,key)
#else
#define case_149_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 148
#define case_148_SBOX32(hash,state,key) _SBOX32_CASE(148,hash,state,key)
#else
#define case_148_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 147
#define case_147_SBOX32(hash,state,key) _SBOX32_CASE(147,hash,state,key)
#else
#define case_147_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 146
#define case_146_SBOX32(hash,state,key) _SBOX32_CASE(146,hash,state,key)
#else
#define case_146_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 145
#define case_145_SBOX32(hash,state,key) _SBOX32_CASE(145,hash,state,key)
#else
#define case_145_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 144
#define case_144_SBOX32(hash,state,key) _SBOX32_CASE(144,hash,state,key)
#else
#define case_144_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 143
#define case_143_SBOX32(hash,state,key) _SBOX32_CASE(143,hash,state,key)
#else
#define case_143_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 142
#define case_142_SBOX32(hash,state,key) _SBOX32_CASE(142,hash,state,key)
#else
#define case_142_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 141
#define case_141_SBOX32(hash,state,key) _SBOX32_CASE(141,hash,state,key)
#else
#define case_141_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 140
#define case_140_SBOX32(hash,state,key) _SBOX32_CASE(140,hash,state,key)
#else
#define case_140_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 139
#define case_139_SBOX32(hash,state,key) _SBOX32_CASE(139,hash,state,key)
#else
#define case_139_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 138
#define case_138_SBOX32(hash,state,key) _SBOX32_CASE(138,hash,state,key)
#else
#define case_138_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 137
#define case_137_SBOX32(hash,state,key) _SBOX32_CASE(137,hash,state,key)
#else
#define case_137_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 136
#define case_136_SBOX32(hash,state,key) _SBOX32_CASE(136,hash,state,key)
#else
#define case_136_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 135
#define case_135_SBOX32(hash,state,key) _SBOX32_CASE(135,hash,state,key)
#else
#define case_135_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 134
#define case_134_SBOX32(hash,state,key) _SBOX32_CASE(134,hash,state,key)
#else
#define case_134_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 133
#define case_133_SBOX32(hash,state,key) _SBOX32_CASE(133,hash,state,key)
#else
#define case_133_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 132
#define case_132_SBOX32(hash,state,key) _SBOX32_CASE(132,hash,state,key)
#else
#define case_132_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 131
#define case_131_SBOX32(hash,state,key) _SBOX32_CASE(131,hash,state,key)
#else
#define case_131_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 130
#define case_130_SBOX32(hash,state,key) _SBOX32_CASE(130,hash,state,key)
#else
#define case_130_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 129
#define case_129_SBOX32(hash,state,key) _SBOX32_CASE(129,hash,state,key)
#else
#define case_129_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 128
#define case_128_SBOX32(hash,state,key) _SBOX32_CASE(128,hash,state,key)
#else
#define case_128_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 127
#define case_127_SBOX32(hash,state,key) _SBOX32_CASE(127,hash,state,key)
#else
#define case_127_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 126
#define case_126_SBOX32(hash,state,key) _SBOX32_CASE(126,hash,state,key)
#else
#define case_126_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 125
#define case_125_SBOX32(hash,state,key) _SBOX32_CASE(125,hash,state,key)
#else
#define case_125_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 124
#define case_124_SBOX32(hash,state,key) _SBOX32_CASE(124,hash,state,key)
#else
#define case_124_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 123
#define case_123_SBOX32(hash,state,key) _SBOX32_CASE(123,hash,state,key)
#else
#define case_123_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 122
#define case_122_SBOX32(hash,state,key) _SBOX32_CASE(122,hash,state,key)
#else
#define case_122_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 121
#define case_121_SBOX32(hash,state,key) _SBOX32_CASE(121,hash,state,key)
#else
#define case_121_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 120
#define case_120_SBOX32(hash,state,key) _SBOX32_CASE(120,hash,state,key)
#else
#define case_120_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 119
#define case_119_SBOX32(hash,state,key) _SBOX32_CASE(119,hash,state,key)
#else
#define case_119_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 118
#define case_118_SBOX32(hash,state,key) _SBOX32_CASE(118,hash,state,key)
#else
#define case_118_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 117
#define case_117_SBOX32(hash,state,key) _SBOX32_CASE(117,hash,state,key)
#else
#define case_117_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 116
#define case_116_SBOX32(hash,state,key) _SBOX32_CASE(116,hash,state,key)
#else
#define case_116_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 115
#define case_115_SBOX32(hash,state,key) _SBOX32_CASE(115,hash,state,key)
#else
#define case_115_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 114
#define case_114_SBOX32(hash,state,key) _SBOX32_CASE(114,hash,state,key)
#else
#define case_114_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 113
#define case_113_SBOX32(hash,state,key) _SBOX32_CASE(113,hash,state,key)
#else
#define case_113_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 112
#define case_112_SBOX32(hash,state,key) _SBOX32_CASE(112,hash,state,key)
#else
#define case_112_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 111
#define case_111_SBOX32(hash,state,key) _SBOX32_CASE(111,hash,state,key)
#else
#define case_111_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 110
#define case_110_SBOX32(hash,state,key) _SBOX32_CASE(110,hash,state,key)
#else
#define case_110_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 109
#define case_109_SBOX32(hash,state,key) _SBOX32_CASE(109,hash,state,key)
#else
#define case_109_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 108
#define case_108_SBOX32(hash,state,key) _SBOX32_CASE(108,hash,state,key)
#else
#define case_108_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 107
#define case_107_SBOX32(hash,state,key) _SBOX32_CASE(107,hash,state,key)
#else
#define case_107_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 106
#define case_106_SBOX32(hash,state,key) _SBOX32_CASE(106,hash,state,key)
#else
#define case_106_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 105
#define case_105_SBOX32(hash,state,key) _SBOX32_CASE(105,hash,state,key)
#else
#define case_105_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 104
#define case_104_SBOX32(hash,state,key) _SBOX32_CASE(104,hash,state,key)
#else
#define case_104_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 103
#define case_103_SBOX32(hash,state,key) _SBOX32_CASE(103,hash,state,key)
#else
#define case_103_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 102
#define case_102_SBOX32(hash,state,key) _SBOX32_CASE(102,hash,state,key)
#else
#define case_102_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 101
#define case_101_SBOX32(hash,state,key) _SBOX32_CASE(101,hash,state,key)
#else
#define case_101_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 100
#define case_100_SBOX32(hash,state,key) _SBOX32_CASE(100,hash,state,key)
#else
#define case_100_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 99
#define case_99_SBOX32(hash,state,key) _SBOX32_CASE(99,hash,state,key)
#else
#define case_99_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 98
#define case_98_SBOX32(hash,state,key) _SBOX32_CASE(98,hash,state,key)
#else
#define case_98_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 97
#define case_97_SBOX32(hash,state,key) _SBOX32_CASE(97,hash,state,key)
#else
#define case_97_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 96
#define case_96_SBOX32(hash,state,key) _SBOX32_CASE(96,hash,state,key)
#else
#define case_96_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 95
#define case_95_SBOX32(hash,state,key) _SBOX32_CASE(95,hash,state,key)
#else
#define case_95_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 94
#define case_94_SBOX32(hash,state,key) _SBOX32_CASE(94,hash,state,key)
#else
#define case_94_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 93
#define case_93_SBOX32(hash,state,key) _SBOX32_CASE(93,hash,state,key)
#else
#define case_93_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 92
#define case_92_SBOX32(hash,state,key) _SBOX32_CASE(92,hash,state,key)
#else
#define case_92_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 91
#define case_91_SBOX32(hash,state,key) _SBOX32_CASE(91,hash,state,key)
#else
#define case_91_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 90
#define case_90_SBOX32(hash,state,key) _SBOX32_CASE(90,hash,state,key)
#else
#define case_90_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 89
#define case_89_SBOX32(hash,state,key) _SBOX32_CASE(89,hash,state,key)
#else
#define case_89_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 88
#define case_88_SBOX32(hash,state,key) _SBOX32_CASE(88,hash,state,key)
#else
#define case_88_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 87
#define case_87_SBOX32(hash,state,key) _SBOX32_CASE(87,hash,state,key)
#else
#define case_87_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 86
#define case_86_SBOX32(hash,state,key) _SBOX32_CASE(86,hash,state,key)
#else
#define case_86_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 85
#define case_85_SBOX32(hash,state,key) _SBOX32_CASE(85,hash,state,key)
#else
#define case_85_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 84
#define case_84_SBOX32(hash,state,key) _SBOX32_CASE(84,hash,state,key)
#else
#define case_84_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 83
#define case_83_SBOX32(hash,state,key) _SBOX32_CASE(83,hash,state,key)
#else
#define case_83_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 82
#define case_82_SBOX32(hash,state,key) _SBOX32_CASE(82,hash,state,key)
#else
#define case_82_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 81
#define case_81_SBOX32(hash,state,key) _SBOX32_CASE(81,hash,state,key)
#else
#define case_81_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 80
#define case_80_SBOX32(hash,state,key) _SBOX32_CASE(80,hash,state,key)
#else
#define case_80_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 79
#define case_79_SBOX32(hash,state,key) _SBOX32_CASE(79,hash,state,key)
#else
#define case_79_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 78
#define case_78_SBOX32(hash,state,key) _SBOX32_CASE(78,hash,state,key)
#else
#define case_78_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 77
#define case_77_SBOX32(hash,state,key) _SBOX32_CASE(77,hash,state,key)
#else
#define case_77_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 76
#define case_76_SBOX32(hash,state,key) _SBOX32_CASE(76,hash,state,key)
#else
#define case_76_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 75
#define case_75_SBOX32(hash,state,key) _SBOX32_CASE(75,hash,state,key)
#else
#define case_75_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 74
#define case_74_SBOX32(hash,state,key) _SBOX32_CASE(74,hash,state,key)
#else
#define case_74_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 73
#define case_73_SBOX32(hash,state,key) _SBOX32_CASE(73,hash,state,key)
#else
#define case_73_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 72
#define case_72_SBOX32(hash,state,key) _SBOX32_CASE(72,hash,state,key)
#else
#define case_72_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 71
#define case_71_SBOX32(hash,state,key) _SBOX32_CASE(71,hash,state,key)
#else
#define case_71_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 70
#define case_70_SBOX32(hash,state,key) _SBOX32_CASE(70,hash,state,key)
#else
#define case_70_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 69
#define case_69_SBOX32(hash,state,key) _SBOX32_CASE(69,hash,state,key)
#else
#define case_69_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 68
#define case_68_SBOX32(hash,state,key) _SBOX32_CASE(68,hash,state,key)
#else
#define case_68_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 67
#define case_67_SBOX32(hash,state,key) _SBOX32_CASE(67,hash,state,key)
#else
#define case_67_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 66
#define case_66_SBOX32(hash,state,key) _SBOX32_CASE(66,hash,state,key)
#else
#define case_66_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 65
#define case_65_SBOX32(hash,state,key) _SBOX32_CASE(65,hash,state,key)
#else
#define case_65_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 64
#define case_64_SBOX32(hash,state,key) _SBOX32_CASE(64,hash,state,key)
#else
#define case_64_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 63
#define case_63_SBOX32(hash,state,key) _SBOX32_CASE(63,hash,state,key)
#else
#define case_63_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 62
#define case_62_SBOX32(hash,state,key) _SBOX32_CASE(62,hash,state,key)
#else
#define case_62_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 61
#define case_61_SBOX32(hash,state,key) _SBOX32_CASE(61,hash,state,key)
#else
#define case_61_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 60
#define case_60_SBOX32(hash,state,key) _SBOX32_CASE(60,hash,state,key)
#else
#define case_60_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 59
#define case_59_SBOX32(hash,state,key) _SBOX32_CASE(59,hash,state,key)
#else
#define case_59_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 58
#define case_58_SBOX32(hash,state,key) _SBOX32_CASE(58,hash,state,key)
#else
#define case_58_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 57
#define case_57_SBOX32(hash,state,key) _SBOX32_CASE(57,hash,state,key)
#else
#define case_57_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 56
#define case_56_SBOX32(hash,state,key) _SBOX32_CASE(56,hash,state,key)
#else
#define case_56_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 55
#define case_55_SBOX32(hash,state,key) _SBOX32_CASE(55,hash,state,key)
#else
#define case_55_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 54
#define case_54_SBOX32(hash,state,key) _SBOX32_CASE(54,hash,state,key)
#else
#define case_54_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 53
#define case_53_SBOX32(hash,state,key) _SBOX32_CASE(53,hash,state,key)
#else
#define case_53_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 52
#define case_52_SBOX32(hash,state,key) _SBOX32_CASE(52,hash,state,key)
#else
#define case_52_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 51
#define case_51_SBOX32(hash,state,key) _SBOX32_CASE(51,hash,state,key)
#else
#define case_51_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 50
#define case_50_SBOX32(hash,state,key) _SBOX32_CASE(50,hash,state,key)
#else
#define case_50_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 49
#define case_49_SBOX32(hash,state,key) _SBOX32_CASE(49,hash,state,key)
#else
#define case_49_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 48
#define case_48_SBOX32(hash,state,key) _SBOX32_CASE(48,hash,state,key)
#else
#define case_48_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 47
#define case_47_SBOX32(hash,state,key) _SBOX32_CASE(47,hash,state,key)
#else
#define case_47_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 46
#define case_46_SBOX32(hash,state,key) _SBOX32_CASE(46,hash,state,key)
#else
#define case_46_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 45
#define case_45_SBOX32(hash,state,key) _SBOX32_CASE(45,hash,state,key)
#else
#define case_45_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 44
#define case_44_SBOX32(hash,state,key) _SBOX32_CASE(44,hash,state,key)
#else
#define case_44_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 43
#define case_43_SBOX32(hash,state,key) _SBOX32_CASE(43,hash,state,key)
#else
#define case_43_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 42
#define case_42_SBOX32(hash,state,key) _SBOX32_CASE(42,hash,state,key)
#else
#define case_42_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 41
#define case_41_SBOX32(hash,state,key) _SBOX32_CASE(41,hash,state,key)
#else
#define case_41_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 40
#define case_40_SBOX32(hash,state,key) _SBOX32_CASE(40,hash,state,key)
#else
#define case_40_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 39
#define case_39_SBOX32(hash,state,key) _SBOX32_CASE(39,hash,state,key)
#else
#define case_39_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 38
#define case_38_SBOX32(hash,state,key) _SBOX32_CASE(38,hash,state,key)
#else
#define case_38_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 37
#define case_37_SBOX32(hash,state,key) _SBOX32_CASE(37,hash,state,key)
#else
#define case_37_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 36
#define case_36_SBOX32(hash,state,key) _SBOX32_CASE(36,hash,state,key)
#else
#define case_36_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 35
#define case_35_SBOX32(hash,state,key) _SBOX32_CASE(35,hash,state,key)
#else
#define case_35_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 34
#define case_34_SBOX32(hash,state,key) _SBOX32_CASE(34,hash,state,key)
#else
#define case_34_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 33
#define case_33_SBOX32(hash,state,key) _SBOX32_CASE(33,hash,state,key)
#else
#define case_33_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 32
#define case_32_SBOX32(hash,state,key) _SBOX32_CASE(32,hash,state,key)
#else
#define case_32_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 31
#define case_31_SBOX32(hash,state,key) _SBOX32_CASE(31,hash,state,key)
#else
#define case_31_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 30
#define case_30_SBOX32(hash,state,key) _SBOX32_CASE(30,hash,state,key)
#else
#define case_30_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 29
#define case_29_SBOX32(hash,state,key) _SBOX32_CASE(29,hash,state,key)
#else
#define case_29_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 28
#define case_28_SBOX32(hash,state,key) _SBOX32_CASE(28,hash,state,key)
#else
#define case_28_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 27
#define case_27_SBOX32(hash,state,key) _SBOX32_CASE(27,hash,state,key)
#else
#define case_27_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 26
#define case_26_SBOX32(hash,state,key) _SBOX32_CASE(26,hash,state,key)
#else
#define case_26_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 25
#define case_25_SBOX32(hash,state,key) _SBOX32_CASE(25,hash,state,key)
#else
#define case_25_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 24
#define case_24_SBOX32(hash,state,key) _SBOX32_CASE(24,hash,state,key)
#else
#define case_24_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 23
#define case_23_SBOX32(hash,state,key) _SBOX32_CASE(23,hash,state,key)
#else
#define case_23_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 22
#define case_22_SBOX32(hash,state,key) _SBOX32_CASE(22,hash,state,key)
#else
#define case_22_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 21
#define case_21_SBOX32(hash,state,key) _SBOX32_CASE(21,hash,state,key)
#else
#define case_21_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 20
#define case_20_SBOX32(hash,state,key) _SBOX32_CASE(20,hash,state,key)
#else
#define case_20_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 19
#define case_19_SBOX32(hash,state,key) _SBOX32_CASE(19,hash,state,key)
#else
#define case_19_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 18
#define case_18_SBOX32(hash,state,key) _SBOX32_CASE(18,hash,state,key)
#else
#define case_18_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 17
#define case_17_SBOX32(hash,state,key) _SBOX32_CASE(17,hash,state,key)
#else
#define case_17_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 16
#define case_16_SBOX32(hash,state,key) _SBOX32_CASE(16,hash,state,key)
#else
#define case_16_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 15
#define case_15_SBOX32(hash,state,key) _SBOX32_CASE(15,hash,state,key)
#else
#define case_15_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 14
#define case_14_SBOX32(hash,state,key) _SBOX32_CASE(14,hash,state,key)
#else
#define case_14_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 13
#define case_13_SBOX32(hash,state,key) _SBOX32_CASE(13,hash,state,key)
#else
#define case_13_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 12
#define case_12_SBOX32(hash,state,key) _SBOX32_CASE(12,hash,state,key)
#else
#define case_12_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 11
#define case_11_SBOX32(hash,state,key) _SBOX32_CASE(11,hash,state,key)
#else
#define case_11_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 10
#define case_10_SBOX32(hash,state,key) _SBOX32_CASE(10,hash,state,key)
#else
#define case_10_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 9
#define case_9_SBOX32(hash,state,key) _SBOX32_CASE(9,hash,state,key)
#else
#define case_9_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 8
#define case_8_SBOX32(hash,state,key) _SBOX32_CASE(8,hash,state,key)
#else
#define case_8_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 7
#define case_7_SBOX32(hash,state,key) _SBOX32_CASE(7,hash,state,key)
#else
#define case_7_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 6
#define case_6_SBOX32(hash,state,key) _SBOX32_CASE(6,hash,state,key)
#else
#define case_6_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 5
#define case_5_SBOX32(hash,state,key) _SBOX32_CASE(5,hash,state,key)
#else
#define case_5_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 4
#define case_4_SBOX32(hash,state,key) _SBOX32_CASE(4,hash,state,key)
#else
#define case_4_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 3
#define case_3_SBOX32(hash,state,key) _SBOX32_CASE(3,hash,state,key)
#else
#define case_3_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 2
#define case_2_SBOX32(hash,state,key) _SBOX32_CASE(2,hash,state,key)
#else
#define case_2_SBOX32(hash,state,key) /**/
#endif
#if SBOX32_MAX_LEN >= 1
#define case_1_SBOX32(hash,state,key) _SBOX32_CASE(1,hash,state,key)
#else
#define case_1_SBOX32(hash,state,key) /**/
#endif

#define XORSHIFT96_set(r,x,y,z,t) STMT_START {          \
    t = (x ^ ( x << 10 ) );                             \
    x = y; y = z;                                       \
    r = z = (z ^ ( z >> 26 ) ) ^ ( t ^ ( t >> 5 ) );    \
} STMT_END

#define XORSHIFT128_set(r,x,y,z,w,t) STMT_START {       \
    t = ( x ^ ( x << 5 ) );                             \
    x = y; y = z; z = w;                                \
    r = w = ( w ^ ( w >> 29 ) ) ^ ( t ^ ( t >> 12 ) );  \
} STMT_END

#define SBOX32_SCRAMBLE32(v,prime) STMT_START {  \
    v ^= (v>>9);                        \
    v ^= (v<<21);                       \
    v ^= (v>>16);                       \
    v *= prime;                         \
    v ^= (v>>17);                       \
    v ^= (v<<15);                       \
    v ^= (v>>23);                       \
} STMT_END

#ifndef SBOX32_CHURN_ROUNDS 
#define SBOX32_CHURN_ROUNDS 5
#endif
#ifndef SBOX32_SKIP_MASK
#define SBOX32_SKIP_MASK 0x3
#endif

#define _SBOX32_CASE(len,hash,state,key) \
    /* FALLTHROUGH */ \
    case len: hash ^= state[ 1 + ( 256 * ( len - 1 ) ) + key[ len - 1 ] ];


SBOX32_STATIC_INLINE void sbox32_seed_state96 (
    const U8 *seed_ch,
    U8 *state_ch
) {
    const U32 *seed= (const U32 *)seed_ch;
    U32 *state= (U32 *)state_ch;
    U32 *state_cursor = state + 1;
    U32 *sbox32_end = state + 1 + (256 * SBOX32_MAX_LEN);
    U32 s0 = seed[0] ^ 0x68736168; /* sbox */
    U32 s1 = seed[1] ^ 0x786f6273; /* hash */
    U32 s2 = seed[2] ^ 0x646f6f67; /* good */
    U32 t1,t2,i;

    /* make sure we have all non-zero state elements */
    if (!s0) s0 = 1;
    if (!s1) s1 = 2;
    if (!s2) s2 = 4;

    /* Do a bunch of mix rounds to avalanche the seedbits
     * before we use them for the XORSHIFT rng. */
    for ( i = 0; i < SBOX32_CHURN_ROUNDS; i++ )
        SBOX32_MIX3(s0,s1,s2,"SEED STATE");

    while ( state_cursor < sbox32_end ) {
        U32 *row_end = state_cursor + 256; 
        for ( ; state_cursor < row_end; state_cursor++ ) {
            XORSHIFT96_set(*state_cursor,s0,s1,s2,t1);
        }
    }
    XORSHIFT96_set(*state,s0,s1,s2,t2);
}

SBOX32_STATIC_INLINE void sbox32_seed_state128 (
    const U8 *seed_ch,
    U8 *state_ch
) {
    U32 *seed= (U32 *)seed_ch;
    U32 *state= (U32 *)state_ch;
    U32 *state_cursor = state + 1;
    U32 *sbox32_end = state + 1 + (256 * SBOX32_MAX_LEN);
    U32 s0 = seed[0] ^ 0x68736168; /* sbox */
    U32 s1 = seed[1] ^ 0x786f6273; /* hash */
    U32 s2 = seed[2] ^ 0x646f6f67; /* good */
    U32 s3 = seed[3] ^ 0x74736166; /* fast */
    U32 t1,t2,i;

    /* make sure we have all non-zero state elements */
    if (!s0) s0 = 1;
    if (!s1) s1 = 2;
    if (!s2) s2 = 4;
    if (!s3) s3 = 8;
    
    /* Do a bunch of mix rounds to avalanche the seedbits
     * before we use them for the XORSHIFT rng. */
    for ( i = 0; i < SBOX32_CHURN_ROUNDS; i++ )
        SBOX32_MIX4(s0,s1,s2,s3,"SEED STATE");

    while ( state_cursor < sbox32_end ) {
        U32 *row_end = state_cursor + 256; 
        for ( ; state_cursor < row_end; state_cursor++ ) {
            XORSHIFT128_set(*state_cursor,s0,s1,s2,s3,t1);
        }
    }
    XORSHIFT128_set(*state,s0,s1,s2,s3,t2);
}

SBOX32_STATIC_INLINE U32 sbox32_hash_with_state(
    const U8 *state_ch,
    const U8 *key,
    const STRLEN key_len
) {
    U32 *state= (U32 *)state_ch;
    U32 hash = *state;
    switch (key_len) {
        default: return zaphod32_hash_with_state(state_ch, key, key_len);
        case_256_SBOX32(hash,state,key)
        case_255_SBOX32(hash,state,key)
        case_254_SBOX32(hash,state,key)
        case_253_SBOX32(hash,state,key)
        case_252_SBOX32(hash,state,key)
        case_251_SBOX32(hash,state,key)
        case_250_SBOX32(hash,state,key)
        case_249_SBOX32(hash,state,key)
        case_248_SBOX32(hash,state,key)
        case_247_SBOX32(hash,state,key)
        case_246_SBOX32(hash,state,key)
        case_245_SBOX32(hash,state,key)
        case_244_SBOX32(hash,state,key)
        case_243_SBOX32(hash,state,key)
        case_242_SBOX32(hash,state,key)
        case_241_SBOX32(hash,state,key)
        case_240_SBOX32(hash,state,key)
        case_239_SBOX32(hash,state,key)
        case_238_SBOX32(hash,state,key)
        case_237_SBOX32(hash,state,key)
        case_236_SBOX32(hash,state,key)
        case_235_SBOX32(hash,state,key)
        case_234_SBOX32(hash,state,key)
        case_233_SBOX32(hash,state,key)
        case_232_SBOX32(hash,state,key)
        case_231_SBOX32(hash,state,key)
        case_230_SBOX32(hash,state,key)
        case_229_SBOX32(hash,state,key)
        case_228_SBOX32(hash,state,key)
        case_227_SBOX32(hash,state,key)
        case_226_SBOX32(hash,state,key)
        case_225_SBOX32(hash,state,key)
        case_224_SBOX32(hash,state,key)
        case_223_SBOX32(hash,state,key)
        case_222_SBOX32(hash,state,key)
        case_221_SBOX32(hash,state,key)
        case_220_SBOX32(hash,state,key)
        case_219_SBOX32(hash,state,key)
        case_218_SBOX32(hash,state,key)
        case_217_SBOX32(hash,state,key)
        case_216_SBOX32(hash,state,key)
        case_215_SBOX32(hash,state,key)
        case_214_SBOX32(hash,state,key)
        case_213_SBOX32(hash,state,key)
        case_212_SBOX32(hash,state,key)
        case_211_SBOX32(hash,state,key)
        case_210_SBOX32(hash,state,key)
        case_209_SBOX32(hash,state,key)
        case_208_SBOX32(hash,state,key)
        case_207_SBOX32(hash,state,key)
        case_206_SBOX32(hash,state,key)
        case_205_SBOX32(hash,state,key)
        case_204_SBOX32(hash,state,key)
        case_203_SBOX32(hash,state,key)
        case_202_SBOX32(hash,state,key)
        case_201_SBOX32(hash,state,key)
        case_200_SBOX32(hash,state,key)
        case_199_SBOX32(hash,state,key)
        case_198_SBOX32(hash,state,key)
        case_197_SBOX32(hash,state,key)
        case_196_SBOX32(hash,state,key)
        case_195_SBOX32(hash,state,key)
        case_194_SBOX32(hash,state,key)
        case_193_SBOX32(hash,state,key)
        case_192_SBOX32(hash,state,key)
        case_191_SBOX32(hash,state,key)
        case_190_SBOX32(hash,state,key)
        case_189_SBOX32(hash,state,key)
        case_188_SBOX32(hash,state,key)
        case_187_SBOX32(hash,state,key)
        case_186_SBOX32(hash,state,key)
        case_185_SBOX32(hash,state,key)
        case_184_SBOX32(hash,state,key)
        case_183_SBOX32(hash,state,key)
        case_182_SBOX32(hash,state,key)
        case_181_SBOX32(hash,state,key)
        case_180_SBOX32(hash,state,key)
        case_179_SBOX32(hash,state,key)
        case_178_SBOX32(hash,state,key)
        case_177_SBOX32(hash,state,key)
        case_176_SBOX32(hash,state,key)
        case_175_SBOX32(hash,state,key)
        case_174_SBOX32(hash,state,key)
        case_173_SBOX32(hash,state,key)
        case_172_SBOX32(hash,state,key)
        case_171_SBOX32(hash,state,key)
        case_170_SBOX32(hash,state,key)
        case_169_SBOX32(hash,state,key)
        case_168_SBOX32(hash,state,key)
        case_167_SBOX32(hash,state,key)
        case_166_SBOX32(hash,state,key)
        case_165_SBOX32(hash,state,key)
        case_164_SBOX32(hash,state,key)
        case_163_SBOX32(hash,state,key)
        case_162_SBOX32(hash,state,key)
        case_161_SBOX32(hash,state,key)
        case_160_SBOX32(hash,state,key)
        case_159_SBOX32(hash,state,key)
        case_158_SBOX32(hash,state,key)
        case_157_SBOX32(hash,state,key)
        case_156_SBOX32(hash,state,key)
        case_155_SBOX32(hash,state,key)
        case_154_SBOX32(hash,state,key)
        case_153_SBOX32(hash,state,key)
        case_152_SBOX32(hash,state,key)
        case_151_SBOX32(hash,state,key)
        case_150_SBOX32(hash,state,key)
        case_149_SBOX32(hash,state,key)
        case_148_SBOX32(hash,state,key)
        case_147_SBOX32(hash,state,key)
        case_146_SBOX32(hash,state,key)
        case_145_SBOX32(hash,state,key)
        case_144_SBOX32(hash,state,key)
        case_143_SBOX32(hash,state,key)
        case_142_SBOX32(hash,state,key)
        case_141_SBOX32(hash,state,key)
        case_140_SBOX32(hash,state,key)
        case_139_SBOX32(hash,state,key)
        case_138_SBOX32(hash,state,key)
        case_137_SBOX32(hash,state,key)
        case_136_SBOX32(hash,state,key)
        case_135_SBOX32(hash,state,key)
        case_134_SBOX32(hash,state,key)
        case_133_SBOX32(hash,state,key)
        case_132_SBOX32(hash,state,key)
        case_131_SBOX32(hash,state,key)
        case_130_SBOX32(hash,state,key)
        case_129_SBOX32(hash,state,key)
        case_128_SBOX32(hash,state,key)
        case_127_SBOX32(hash,state,key)
        case_126_SBOX32(hash,state,key)
        case_125_SBOX32(hash,state,key)
        case_124_SBOX32(hash,state,key)
        case_123_SBOX32(hash,state,key)
        case_122_SBOX32(hash,state,key)
        case_121_SBOX32(hash,state,key)
        case_120_SBOX32(hash,state,key)
        case_119_SBOX32(hash,state,key)
        case_118_SBOX32(hash,state,key)
        case_117_SBOX32(hash,state,key)
        case_116_SBOX32(hash,state,key)
        case_115_SBOX32(hash,state,key)
        case_114_SBOX32(hash,state,key)
        case_113_SBOX32(hash,state,key)
        case_112_SBOX32(hash,state,key)
        case_111_SBOX32(hash,state,key)
        case_110_SBOX32(hash,state,key)
        case_109_SBOX32(hash,state,key)
        case_108_SBOX32(hash,state,key)
        case_107_SBOX32(hash,state,key)
        case_106_SBOX32(hash,state,key)
        case_105_SBOX32(hash,state,key)
        case_104_SBOX32(hash,state,key)
        case_103_SBOX32(hash,state,key)
        case_102_SBOX32(hash,state,key)
        case_101_SBOX32(hash,state,key)
        case_100_SBOX32(hash,state,key)
        case_99_SBOX32(hash,state,key)
        case_98_SBOX32(hash,state,key)
        case_97_SBOX32(hash,state,key)
        case_96_SBOX32(hash,state,key)
        case_95_SBOX32(hash,state,key)
        case_94_SBOX32(hash,state,key)
        case_93_SBOX32(hash,state,key)
        case_92_SBOX32(hash,state,key)
        case_91_SBOX32(hash,state,key)
        case_90_SBOX32(hash,state,key)
        case_89_SBOX32(hash,state,key)
        case_88_SBOX32(hash,state,key)
        case_87_SBOX32(hash,state,key)
        case_86_SBOX32(hash,state,key)
        case_85_SBOX32(hash,state,key)
        case_84_SBOX32(hash,state,key)
        case_83_SBOX32(hash,state,key)
        case_82_SBOX32(hash,state,key)
        case_81_SBOX32(hash,state,key)
        case_80_SBOX32(hash,state,key)
        case_79_SBOX32(hash,state,key)
        case_78_SBOX32(hash,state,key)
        case_77_SBOX32(hash,state,key)
        case_76_SBOX32(hash,state,key)
        case_75_SBOX32(hash,state,key)
        case_74_SBOX32(hash,state,key)
        case_73_SBOX32(hash,state,key)
        case_72_SBOX32(hash,state,key)
        case_71_SBOX32(hash,state,key)
        case_70_SBOX32(hash,state,key)
        case_69_SBOX32(hash,state,key)
        case_68_SBOX32(hash,state,key)
        case_67_SBOX32(hash,state,key)
        case_66_SBOX32(hash,state,key)
        case_65_SBOX32(hash,state,key)
        case_64_SBOX32(hash,state,key)
        case_63_SBOX32(hash,state,key)
        case_62_SBOX32(hash,state,key)
        case_61_SBOX32(hash,state,key)
        case_60_SBOX32(hash,state,key)
        case_59_SBOX32(hash,state,key)
        case_58_SBOX32(hash,state,key)
        case_57_SBOX32(hash,state,key)
        case_56_SBOX32(hash,state,key)
        case_55_SBOX32(hash,state,key)
        case_54_SBOX32(hash,state,key)
        case_53_SBOX32(hash,state,key)
        case_52_SBOX32(hash,state,key)
        case_51_SBOX32(hash,state,key)
        case_50_SBOX32(hash,state,key)
        case_49_SBOX32(hash,state,key)
        case_48_SBOX32(hash,state,key)
        case_47_SBOX32(hash,state,key)
        case_46_SBOX32(hash,state,key)
        case_45_SBOX32(hash,state,key)
        case_44_SBOX32(hash,state,key)
        case_43_SBOX32(hash,state,key)
        case_42_SBOX32(hash,state,key)
        case_41_SBOX32(hash,state,key)
        case_40_SBOX32(hash,state,key)
        case_39_SBOX32(hash,state,key)
        case_38_SBOX32(hash,state,key)
        case_37_SBOX32(hash,state,key)
        case_36_SBOX32(hash,state,key)
        case_35_SBOX32(hash,state,key)
        case_34_SBOX32(hash,state,key)
        case_33_SBOX32(hash,state,key)
        case_32_SBOX32(hash,state,key)
        case_31_SBOX32(hash,state,key)
        case_30_SBOX32(hash,state,key)
        case_29_SBOX32(hash,state,key)
        case_28_SBOX32(hash,state,key)
        case_27_SBOX32(hash,state,key)
        case_26_SBOX32(hash,state,key)
        case_25_SBOX32(hash,state,key)
        case_24_SBOX32(hash,state,key)
        case_23_SBOX32(hash,state,key)
        case_22_SBOX32(hash,state,key)
        case_21_SBOX32(hash,state,key)
        case_20_SBOX32(hash,state,key)
        case_19_SBOX32(hash,state,key)
        case_18_SBOX32(hash,state,key)
        case_17_SBOX32(hash,state,key)
        case_16_SBOX32(hash,state,key)
        case_15_SBOX32(hash,state,key)
        case_14_SBOX32(hash,state,key)
        case_13_SBOX32(hash,state,key)
        case_12_SBOX32(hash,state,key)
        case_11_SBOX32(hash,state,key)
        case_10_SBOX32(hash,state,key)
        case_9_SBOX32(hash,state,key)
        case_8_SBOX32(hash,state,key)
        case_7_SBOX32(hash,state,key)
        case_6_SBOX32(hash,state,key)
        case_5_SBOX32(hash,state,key)
        case_4_SBOX32(hash,state,key)
        case_3_SBOX32(hash,state,key)
        case_2_SBOX32(hash,state,key)
        case_1_SBOX32(hash,state,key)
        case 0: break;
    }
    return hash;
}

SBOX32_STATIC_INLINE U32 sbox32_hash96(
    const U8 *seed_ch,
    const U8 *key,
    const STRLEN key_len
) {
    U32 state[SBOX32_STATE_WORDS];
    sbox32_seed_state96(seed_ch,(U8*)state);
    return sbox32_hash_with_state((U8*)state,key,key_len);
}

SBOX32_STATIC_INLINE U32 sbox32_hash128(
    const U8 *seed_ch,
    const U8 *key,
    const STRLEN key_len
) {
    U32 state[SBOX32_STATE_WORDS];
    sbox32_seed_state128(seed_ch,(U8*)state);
    return sbox32_hash_with_state((U8*)state,key,key_len);
}

#endif
