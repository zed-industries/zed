#ifndef DEBUG_ZAPHOD32_HASH
#define DEBUG_ZAPHOD32_HASH 0

#if DEBUG_ZAPHOD32_HASH == 1
#include <stdio.h>
#define ZAPHOD32_WARN6(pat,v0,v1,v2,v3,v4,v5)    printf(pat, v0, v1, v2, v3, v4, v5)
#define ZAPHOD32_WARN5(pat,v0,v1,v2,v3,v4)       printf(pat, v0, v1, v2, v3, v4)
#define ZAPHOD32_WARN4(pat,v0,v1,v2,v3)          printf(pat, v0, v1, v2, v3)
#define ZAPHOD32_WARN3(pat,v0,v1,v2)             printf(pat, v0, v1, v2)
#define ZAPHOD32_WARN2(pat,v0,v1)                printf(pat, v0, v1)
#define NOTE3(pat,v0,v1,v2)             printf(pat, v0, v1, v2)
#elif DEBUG_ZAPHOD32_HASH == 2
#define ZAPHOD32_WARN6(pat,v0,v1,v2,v3,v4,v5)
#define ZAPHOD32_WARN5(pat,v0,v1,v2,v3,v4)
#define ZAPHOD32_WARN4(pat,v0,v1,v2,v3)
#define ZAPHOD32_WARN3(pat,v0,v1,v2)
#define ZAPHOD32_WARN2(pat,v0,v1)
#define NOTE3(pat,v0,v1,v2)             printf(pat, v0, v1, v2)
#else
#define ZAPHOD32_WARN6(pat,v0,v1,v2,v3,v4,v5)
#define ZAPHOD32_WARN5(pat,v0,v1,v2,v3,v4)
#define ZAPHOD32_WARN4(pat,v0,v1,v2,v3)
#define ZAPHOD32_WARN3(pat,v0,v1,v2)
#define NOTE3(pat,v0,v1,v2)
#define ZAPHOD32_WARN2(pat,v0,v1)
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

#ifndef PERL_SEEN_HV_FUNC_H
#if !defined(U64)
#include <stdint.h>
#define U64 uint64_t
#endif

#if !defined(U32)
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

#ifndef ZAPHOD32_STATIC_INLINE
#ifdef PERL_STATIC_INLINE
#define ZAPHOD32_STATIC_INLINE PERL_STATIC_INLINE
#else
#define ZAPHOD32_STATIC_INLINE static inline
#endif
#endif

#ifndef STMT_START
#define STMT_START do
#define STMT_END while(0)
#endif

/* This is two marsaglia xor-shift permutes, with a prime-multiple
 * sandwiched inside. The end result of doing this twice with different
 * primes is a completely avalanched v.  */
#define ZAPHOD32_SCRAMBLE32(v,prime) STMT_START {  \
    v ^= (v>>9);                        \
    v ^= (v<<21);                       \
    v ^= (v>>16);                       \
    v *= prime;                         \
    v ^= (v>>17);                       \
    v ^= (v<<15);                       \
    v ^= (v>>23);                       \
} STMT_END

#define ZAPHOD32_FINALIZE(v0,v1,v2) STMT_START {          \
    ZAPHOD32_WARN3("v0=%08x v1=%08x v2=%08x - ZAPHOD32 FINALIZE\n", \
            (unsigned int)v0, (unsigned int)v1, (unsigned int)v2);  \
    v2 += v0;                       \
    v1 -= v2;                       \
    v1 = ROTL32(v1,  6);           \
    v2 ^= v1;                       \
    v2 = ROTL32(v2, 28);           \
    v1 ^= v2;                       \
    v0 += v1;                       \
    v1 = ROTL32(v1, 24);           \
    v2 += v1;                       \
    v2 = ROTL32(v2, 18) + v1;      \
    v0 ^= v2;                       \
    v0 = ROTL32(v0, 20);           \
    v2 += v0;                       \
    v1 ^= v2;                       \
    v0 += v1;                       \
    v0 = ROTL32(v0,  5);           \
    v2 += v0;                       \
    v2 = ROTL32(v2, 22);           \
    v0 -= v1;                       \
    v1 -= v2;                       \
    v1 = ROTL32(v1, 17);           \
} STMT_END

#define ZAPHOD32_MIX(v0,v1,v2,text) STMT_START {                              \
    ZAPHOD32_WARN4("v0=%08x v1=%08x v2=%08x - ZAPHOD32 %s MIX\n",                   \
            (unsigned int)v0,(unsigned int)v1,(unsigned int)v2, text );  \
    v0 = ROTL32(v0,16) - v2;   \
    v1 = ROTR32(v1,13) ^ v2;   \
    v2 = ROTL32(v2,17) + v1;   \
    v0 = ROTR32(v0, 2) + v1;   \
    v1 = ROTR32(v1,17) - v0;   \
    v2 = ROTR32(v2, 7) ^ v0;   \
} STMT_END


ZAPHOD32_STATIC_INLINE
void zaphod32_seed_state (
    const U8 *seed_ch,
    U8 *state_ch
) {
    const U32 *seed= (const U32 *)seed_ch;
    U32 *state= (U32 *)state_ch;
  
    /* hex expansion of pi, skipping first two digits. pi= 3.2[43f6...]*/
    /* pi value in hex from here:
     * http://turner.faculty.swau.edu/mathematics/materialslibrary/pi/pibases.html*/
    /* Ensure that the three state vectors are nonzero regardless of the seed. */
    /* The idea of these two steps is to ensure that the 0 state comes from a seed
     * utterly unlike that of the value we replace it with.*/
    state[0]= seed[0] ^ 0x43f6a888;
    state[1]= seed[1] ^ 0x5a308d31;
    state[2]= seed[2] ^ 0x3198a2e0;
    if (!state[0]) state[0] = 1;
    if (!state[1]) state[1] = 2;
    if (!state[2]) state[2] = 4;
    /* these are pseduo-randomly selected primes between 2**31 and 2**32
     * (I generated a big list and then randomly chose some from the list) */
    ZAPHOD32_SCRAMBLE32(state[0],0x9fade23b);
    ZAPHOD32_SCRAMBLE32(state[1],0xaa6f908d);
    ZAPHOD32_SCRAMBLE32(state[2],0xcdf6b72d);

    /* now that we have scrambled we do some mixing to avalanche the
     * state bits to gether */
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE A 1/4");
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE A 2/4");
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE A 3/4");
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE A 4/4");

    /* and then scramble them again with different primes */
    ZAPHOD32_SCRAMBLE32(state[0],0xc95d22a9);
    ZAPHOD32_SCRAMBLE32(state[1],0x8497242b);
    ZAPHOD32_SCRAMBLE32(state[2],0x9c5cc4e9);

    /* and a thorough final mix */
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE B 1/5");
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE B 2/5");
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE B 3/5");
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE B 4/5");
    ZAPHOD32_MIX(state[0],state[1],state[2],"ZAPHOD32 SEED-STATE B 5/5");

}

ZAPHOD32_STATIC_INLINE
U32 zaphod32_hash_with_state(
    const U8 *state_ch,
    const U8 *key,
    const STRLEN key_len
) {
    U32 *state= (U32 *)state_ch;
    const U8 *end;
    STRLEN len = key_len;
    U32 v0= state[0];
    U32 v1= state[1];
    U32 v2= state[2] ^ (0xC41A7AB1 * ((U32)key_len + 1));

    ZAPHOD32_WARN4("v0=%08x v1=%08x v2=%08x ln=%08x HASH START\n",
            (unsigned int)state[0], (unsigned int)state[1],
            (unsigned int)state[2], (unsigned int)key_len);
    {
        switch (len) {
            default: goto zaphod32_read8;
            case 12: v2 += (U32)key[11] << 24;  /* FALLTHROUGH */
            case 11: v2 += (U32)key[10] << 16;  /* FALLTHROUGH */
            case 10: v2 += (U32)U8TO16_LE(key+8);
                     v1 -= U8TO32_LE(key+4);
                     v0 += U8TO32_LE(key+0);
                     goto zaphod32_finalize;
            case 9: v2 += (U32)key[8];          /* FALLTHROUGH */
            case 8: v1 -= U8TO32_LE(key+4);
                    v0 += U8TO32_LE(key+0);
                    goto zaphod32_finalize;
            case 7: v2 += (U32)key[6];          /* FALLTHROUGH */
            case 6: v0 += (U32)U8TO16_LE(key+4);
                    v1 -= U8TO32_LE(key+0);
                    goto zaphod32_finalize;
            case 5: v0 += (U32)key[4];          /* FALLTHROUGH */
            case 4: v1 -= U8TO32_LE(key+0);
                    goto zaphod32_finalize;
            case 3: v2 += (U32)key[2];          /* FALLTHROUGH */
            case 2: v0 += (U32)U8TO16_LE(key);
                    break;
            case 1: v0 += (U32)key[0];
                    break;
            case 0: v2 ^= 0xFF;
                    break;

        }
        v0 -= v2;
        v2 = ROTL32(v2, 8) ^ v0;
        v0 = ROTR32(v0,16) + v2;
        v2 += v0;
        v0 += v0 >> 9;
        v0 += v2;
        v2 ^= v0;
        v2 += v2 << 4;
        v0 -= v2;
        v2 = ROTR32(v2, 8) ^ v0;
        v0 = ROTL32(v0,16) ^ v2;
        v2 = ROTL32(v2,10) + v0;
        v0 = ROTR32(v0,30) + v2;
        v2 = ROTR32(v2,12);
        return v0 ^ v2;
    }

/*  if (len >= 8) */ /* this block is only reached by a goto above, so this condition
                        is commented out, but if the above block is removed it would
                        be necessary to use this. */
    {
zaphod32_read8:
        len = key_len & 0x7;
        end = key + key_len - len;
        do {
            v1 -= U8TO32_LE(key+0);
            v0 += U8TO32_LE(key+4);
            ZAPHOD32_MIX(v0,v1,v2,"MIX 2-WORDS A");
            key += 8;
        } while ( key < end );
    }

    if ( len >= 4 ) {
        v1 -= U8TO32_LE(key);
        key += 4;
    }

    v0 += (U32)(key_len) << 24;
    switch (len & 0x3) {
        case 3: v2 += (U32)key[2];          /* FALLTHROUGH */
        case 2: v0 += (U32)U8TO16_LE(key);
                break;
        case 1: v0 += (U32)key[0];
                break;
        case 0: v2 ^= 0xFF;
                break;
    }
zaphod32_finalize:
    ZAPHOD32_FINALIZE(v0,v1,v2);

    ZAPHOD32_WARN4("v0=%08x v1=%08x v2=%08x hh=%08x - FINAL\n\n",
            (unsigned int)v0, (unsigned int)v1, (unsigned int)v2,
            (unsigned int)v0 ^ v1 ^ v2);

    return v0 ^ v1 ^ v2;
}

ZAPHOD32_STATIC_INLINE U32 zaphod32_hash(
    const U8 *seed_ch,
    const U8 *key,
    const STRLEN key_len
) {
    U32 state[3];
    zaphod32_seed_state(seed_ch,(U8*)state);
    return zaphod32_hash_with_state((U8*)state,key,key_len);
}

#endif
