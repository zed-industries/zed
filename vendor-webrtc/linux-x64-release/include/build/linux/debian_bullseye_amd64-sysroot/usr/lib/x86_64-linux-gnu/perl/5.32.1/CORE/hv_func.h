/* hash a key
 *--------------------------------------------------------------------------------------
 * The "hash seed" feature was added in Perl 5.8.1 to perturb the results
 * to avoid "algorithmic complexity attacks".
 *
 * If USE_HASH_SEED is defined, hash randomisation is done by default
 * (see also perl.c:perl_parse() and S_init_tls_and_interp() and util.c:get_hash_seed())
 */
#ifndef PERL_SEEN_HV_FUNC_H /* compile once */
#define PERL_SEEN_HV_FUNC_H
#include "hv_macro.h"

#if !( 0 \
        || defined(PERL_HASH_FUNC_SIPHASH) \
        || defined(PERL_HASH_FUNC_SIPHASH13) \
        || defined(PERL_HASH_FUNC_STADTX) \
        || defined(PERL_HASH_FUNC_ZAPHOD32) \
    )
#   ifdef CAN64BITHASH
#       define PERL_HASH_FUNC_STADTX
#   else
#       define PERL_HASH_FUNC_ZAPHOD32
#   endif
#endif

#ifndef PERL_HASH_USE_SBOX32_ALSO
#define PERL_HASH_USE_SBOX32_ALSO 1
#endif

#ifndef SBOX32_MAX_LEN
#define SBOX32_MAX_LEN 24
#endif

/* this must be after the SBOX32_MAX_LEN define */
#include "sbox32_hash.h"

#if defined(PERL_HASH_FUNC_SIPHASH)
# define __PERL_HASH_FUNC "SIPHASH_2_4"
# define __PERL_HASH_SEED_BYTES 16
# define __PERL_HASH_STATE_BYTES 32
# define __PERL_HASH_SEED_STATE(seed,state) S_perl_siphash_seed_state(seed,state)
# define __PERL_HASH_WITH_STATE(state,str,len) S_perl_hash_siphash_2_4_with_state((state),(U8*)(str),(len))
#elif defined(PERL_HASH_FUNC_SIPHASH13)
# define __PERL_HASH_FUNC "SIPHASH_1_3"
# define __PERL_HASH_SEED_BYTES 16
# define __PERL_HASH_STATE_BYTES 32
# define __PERL_HASH_SEED_STATE(seed,state) S_perl_siphash_seed_state(seed,state)
# define __PERL_HASH_WITH_STATE(state,str,len) S_perl_hash_siphash_1_3_with_state((state),(U8*)(str),(len))
#elif defined(PERL_HASH_FUNC_STADTX)
# define __PERL_HASH_FUNC "STADTX"
# define __PERL_HASH_SEED_BYTES 16
# define __PERL_HASH_STATE_BYTES 32
# define __PERL_HASH_SEED_STATE(seed,state) stadtx_seed_state(seed,state)
# define __PERL_HASH_WITH_STATE(state,str,len) (U32)stadtx_hash_with_state((state),(U8*)(str),(len))
# include "stadtx_hash.h"
#elif defined(PERL_HASH_FUNC_ZAPHOD32)
# define __PERL_HASH_FUNC "ZAPHOD32"
# define __PERL_HASH_SEED_BYTES 12
# define __PERL_HASH_STATE_BYTES 12
# define __PERL_HASH_SEED_STATE(seed,state) zaphod32_seed_state(seed,state)
# define __PERL_HASH_WITH_STATE(state,str,len) (U32)zaphod32_hash_with_state((state),(U8*)(str),(len))
# include "zaphod32_hash.h"
#endif

#ifndef __PERL_HASH_WITH_STATE
#error "No hash function defined!"
#endif
#ifndef __PERL_HASH_SEED_BYTES
#error "__PERL_HASH_SEED_BYTES not defined"
#endif
#ifndef __PERL_HASH_FUNC
#error "__PERL_HASH_FUNC not defined"
#endif


#if PERL_HASH_USE_SBOX32_ALSO != 1
# define _PERL_HASH_FUNC                        __PERL_HASH_FUNC
# define _PERL_HASH_SEED_BYTES                  __PERL_HASH_SEED_BYTES
# define _PERL_HASH_STATE_BYTES                 __PERL_HASH_STATE_BYTES
# define _PERL_HASH_SEED_STATE(seed,state)      __PERL_HASH_SEED_STATE(seed,state)
# define _PERL_HASH_WITH_STATE(state,str,len)   __PERL_HASH_WITH_STATE(state,str,len)
#else

#define _PERL_HASH_FUNC         "SBOX32_WITH_" __PERL_HASH_FUNC

#define _PERL_HASH_SEED_BYTES   ( __PERL_HASH_SEED_BYTES + (int)( 3 * sizeof(U32) ) )

#define _PERL_HASH_STATE_BYTES  \
    ( __PERL_HASH_STATE_BYTES + ( ( 1 + ( 256 * SBOX32_MAX_LEN ) ) * sizeof(U32) ) )

#define _PERL_HASH_SEED_STATE(seed,state) STMT_START {                                      \
    __PERL_HASH_SEED_STATE(seed,state);                                                     \
    sbox32_seed_state96(seed + __PERL_HASH_SEED_BYTES, state + __PERL_HASH_STATE_BYTES);    \
} STMT_END

#define _PERL_HASH_WITH_STATE(state,str,len)                                            \
    (LIKELY(len <= SBOX32_MAX_LEN)                                                      \
        ? sbox32_hash_with_state((state + __PERL_HASH_STATE_BYTES),(U8*)(str),(len))    \
        : __PERL_HASH_WITH_STATE((state),(str),(len)))

#endif

PERL_STATIC_INLINE
U32 S_perl_hash_with_seed(const U8 * const seed, const U8 * const str, const STRLEN len)
{
    U8 state[_PERL_HASH_STATE_BYTES];
    _PERL_HASH_SEED_STATE(seed,state);
    return _PERL_HASH_WITH_STATE(state,str,len);
}

#define PERL_HASH_WITH_SEED(seed,hash,str,len) \
    (hash) = S_perl_hash_with_seed((const U8 *) seed, (const U8 *) str,len)
#define PERL_HASH_WITH_STATE(state,hash,str,len) \
    (hash) = _PERL_HASH_WITH_STATE((state),(U8*)(str),(len))
#define PERL_HASH_SEED_STATE(seed,state) _PERL_HASH_SEED_STATE(seed,state)
#define PERL_HASH_SEED_BYTES _PERL_HASH_SEED_BYTES
#define PERL_HASH_STATE_BYTES _PERL_HASH_STATE_BYTES
#define PERL_HASH_FUNC        _PERL_HASH_FUNC

#ifdef PERL_USE_SINGLE_CHAR_HASH_CACHE
#define PERL_HASH(state,str,len) \
    (hash) = ((len) < 2 ? ( (len) == 0 ? PL_hash_chars[256] : PL_hash_chars[(U8)(str)[0]] ) \
                       : _PERL_HASH_WITH_STATE(PL_hash_state,(U8*)(str),(len)))
#else
#define PERL_HASH(hash,str,len) \
    PERL_HASH_WITH_STATE(PL_hash_state,hash,(U8*)(str),(len))
#endif

/* Setup the hash seed, either we do things dynamically at start up,
 * including reading from the environment, or we randomly setup the
 * seed. The seed will be passed into the PERL_HASH_SEED_STATE() function
 * defined for the configuration defined for this perl, which will then
 * initialize whatever state it might need later in hashing. */

#ifndef PERL_HASH_SEED
#   if defined(USE_HASH_SEED)
#       define PERL_HASH_SEED PL_hash_seed
#   else
       /* this is a 512 bit seed, which should be more than enough for the
        * configuration of any of our hash functions (with or without sbox).
        * If you actually use a hard coded seed, you are strongly encouraged
        * to replace this with something else of the correct length
        * for the hash function you are using (24-32 bytes depending on build
        * options). Repeat, you are *STRONGLY* encouraged not to use the value
        * provided here.
        */
#       define PERL_HASH_SEED \
           ((const U8 *)"A long string of pseudorandomly "  \
                        "chosen bytes for hashing in Perl")
#   endif
#endif

/* legacy - only mod_perl should be doing this.  */
#ifdef PERL_HASH_INTERNAL_ACCESS
#define PERL_HASH_INTERNAL(hash,str,len) PERL_HASH(hash,str,len)
#endif

/* This is SipHash by Jean-Philippe Aumasson and Daniel J. Bernstein.
 * The authors claim it is relatively secure compared to the alternatives
 * and that performance wise it is a suitable hash for languages like Perl.
 * See:
 *
 * https://www.131002.net/siphash/
 *
 * This implementation seems to perform slightly slower than one-at-a-time for
 * short keys, but degrades slower for longer keys. Murmur Hash outperforms it
 * regardless of keys size.
 *
 * It is 64 bit only.
 */

#ifdef CAN64BITHASH

#define SIPROUND            \
  STMT_START {              \
    v0 += v1; v1=ROTL64(v1,13); v1 ^= v0; v0=ROTL64(v0,32); \
    v2 += v3; v3=ROTL64(v3,16); v3 ^= v2;     \
    v0 += v3; v3=ROTL64(v3,21); v3 ^= v0;     \
    v2 += v1; v1=ROTL64(v1,17); v1 ^= v2; v2=ROTL64(v2,32); \
  } STMT_END

#define SIPHASH_SEED_STATE(key,v0,v1,v2,v3) \
do {                                    \
    v0 = v2 = U8TO64_LE(key + 0);       \
    v1 = v3 = U8TO64_LE(key + 8);       \
  /* "somepseudorandomlygeneratedbytes" */  \
    v0 ^= UINT64_C(0x736f6d6570736575);  \
    v1 ^= UINT64_C(0x646f72616e646f6d);      \
    v2 ^= UINT64_C(0x6c7967656e657261);      \
    v3 ^= UINT64_C(0x7465646279746573);      \
} while (0)

PERL_STATIC_INLINE
void S_perl_siphash_seed_state(const unsigned char * const seed_buf, unsigned char * state_buf) {
    U64 *v= (U64*) state_buf;
    SIPHASH_SEED_STATE(seed_buf, v[0],v[1],v[2],v[3]);
}

#define PERL_SIPHASH_FNC(FNC,SIP_ROUNDS,SIP_FINAL_ROUNDS) \
PERL_STATIC_INLINE U64 \
FNC ## _with_state_64 \
  (const unsigned char * const state, const unsigned char *in, const STRLEN inlen) \
{                                           \
  const int left = inlen & 7;               \
  const U8 *end = in + inlen - left;        \
                                            \
  U64 b = ( ( U64 )(inlen) ) << 56;         \
  U64 m;                                    \
  U64 v0 = U8TO64_LE(state);                \
  U64 v1 = U8TO64_LE(state+8);              \
  U64 v2 = U8TO64_LE(state+16);             \
  U64 v3 = U8TO64_LE(state+24);             \
                                            \
  for ( ; in != end; in += 8 )              \
  {                                         \
    m = U8TO64_LE( in );                    \
    v3 ^= m;                                \
                                            \
    SIP_ROUNDS;                             \
                                            \
    v0 ^= m;                                \
  }                                         \
                                            \
  switch( left )                            \
  {                                         \
  case 7: b |= ( ( U64 )in[ 6] )  << 48; /*FALLTHROUGH*/    \
  case 6: b |= ( ( U64 )in[ 5] )  << 40; /*FALLTHROUGH*/    \
  case 5: b |= ( ( U64 )in[ 4] )  << 32; /*FALLTHROUGH*/    \
  case 4: b |= ( ( U64 )in[ 3] )  << 24; /*FALLTHROUGH*/    \
  case 3: b |= ( ( U64 )in[ 2] )  << 16; /*FALLTHROUGH*/    \
  case 2: b |= ( ( U64 )in[ 1] )  <<  8; /*FALLTHROUGH*/    \
  case 1: b |= ( ( U64 )in[ 0] ); break;    \
  case 0: break;                            \
  }                                         \
                                            \
  v3 ^= b;                                  \
                                            \
  SIP_ROUNDS;                               \
                                            \
  v0 ^= b;                                  \
                                            \
  v2 ^= 0xff;                               \
                                            \
  SIP_FINAL_ROUNDS                          \
                                            \
  b = v0 ^ v1 ^ v2  ^ v3;                   \
  return b;                                 \
}                                           \
                                            \
PERL_STATIC_INLINE U32                      \
FNC ## _with_state                          \
  (const unsigned char * const state, const unsigned char *in, const STRLEN inlen) \
{                                           \
    union {                                 \
        U64 h64;                            \
        U32 h32[2];                         \
    } h;                                    \
    h.h64= FNC ## _with_state_64(state,in,inlen); \
    return h.h32[0] ^ h.h32[1];             \
}                                           \
                                            \
                                            \
PERL_STATIC_INLINE U32                      \
FNC (const unsigned char * const seed, const unsigned char *in, const STRLEN inlen) \
{                                                                   \
    U64 state[4];                                                   \
    SIPHASH_SEED_STATE(seed,state[0],state[1],state[2],state[3]);   \
    return FNC ## _with_state((U8*)state,in,inlen);                 \
}


PERL_SIPHASH_FNC(
    S_perl_hash_siphash_1_3
    ,SIPROUND;
    ,SIPROUND;SIPROUND;SIPROUND;
)

PERL_SIPHASH_FNC(
    S_perl_hash_siphash_2_4
    ,SIPROUND;SIPROUND;
    ,SIPROUND;SIPROUND;SIPROUND;SIPROUND;
)

#endif /* defined(CAN64BITHASH) */


#endif /*compile once*/

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
