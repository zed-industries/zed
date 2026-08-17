if(${CMAKE_SYSTEM_NAME} STREQUAL "OpenBSD" AND FIPS)
  if(NOT BUILD_SHARED_LIBS)
    # Disable the stack smash protector for FIPS static builds
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fno-stack-protector")
  endif()
  if(CMAKE_C_COMPILER_ID STREQUAL "Clang")
    # OpenBSD's Clang version has a return trampoline enabled by default that
    # it may inject and cause issues with our custom linker script.
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -mno-retpoline")
  endif()
endif()

if(ANDROID)
  # Since "--Wa,--noexecstack" is not used during the preprocessor step of Android (because assembler is not invoked),
  # Clang reports that argument as unused. We remove the flag only for the FIPS build of Android.
  string(FIND ${CMAKE_CXX_FLAGS} "noexecstack" CXX_EXTRA_WA)
  string(FIND ${CMAKE_C_FLAGS} "noexecstack" C_EXTRA_WA)
  if(NOT CXX_EXTRA_WA EQUAL -1)
    string(REPLACE "-Wa,--noexecstack" "" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
  endif()
  if(NOT C_EXTRA_WA EQUAL -1)
    string(REPLACE "-Wa,--noexecstack" "" CMAKE_C_FLAGS "${CMAKE_C_FLAGS}")
  endif()
endif()

if(ARCH STREQUAL "x86_64")
  set(
    BCM_ASM_SOURCES

    aesni-gcm-avx512.${ASM_EXT}
    aesni-gcm-x86_64.${ASM_EXT}
    aesni-xts-avx512.${ASM_EXT}
    aesni-x86_64.${ASM_EXT}
    ghash-ssse3-x86_64.${ASM_EXT}
    ghash-x86_64.${ASM_EXT}
    md5-x86_64.${ASM_EXT}
    p256-x86_64-asm.${ASM_EXT}
    p256_beeu-x86_64-asm.${ASM_EXT}
    rdrand-x86_64.${ASM_EXT}
    rsaz-avx2.${ASM_EXT}
    rsaz-2k-avx512.${ASM_EXT}
    rsaz-3k-avx512.${ASM_EXT}
    rsaz-4k-avx512.${ASM_EXT}
    sha1-x86_64.${ASM_EXT}
    sha256-x86_64.${ASM_EXT}
    sha512-x86_64.${ASM_EXT}
    vpaes-x86_64.${ASM_EXT}
    x86_64-mont5.${ASM_EXT}
    x86_64-mont.${ASM_EXT}
  )
endif()

if(ARCH STREQUAL "x86")
  set(
    BCM_ASM_SOURCES

    aesni-x86.${ASM_EXT}
    bn-586.${ASM_EXT}
    co-586.${ASM_EXT}
    ghash-ssse3-x86.${ASM_EXT}
    ghash-x86.${ASM_EXT}
    md5-586.${ASM_EXT}
    sha1-586.${ASM_EXT}
    sha256-586.${ASM_EXT}
    sha512-586.${ASM_EXT}
    vpaes-x86.${ASM_EXT}
    x86-mont.${ASM_EXT}
  )
endif()

if(ARCH STREQUAL "arm")
  set(
    BCM_ASM_SOURCES

    aesv8-armx.${ASM_EXT}
    armv4-mont.${ASM_EXT}
    bsaes-armv7.${ASM_EXT}
    ghash-armv4.${ASM_EXT}
    ghashv8-armx.${ASM_EXT}
    sha1-armv4-large.${ASM_EXT}
    sha256-armv4.${ASM_EXT}
    sha512-armv4.${ASM_EXT}
    vpaes-armv7.${ASM_EXT}
  )
endif()

if(ARCH STREQUAL "aarch64")

  set(
    BCM_ASM_SOURCES

    aesv8-armx.${ASM_EXT}
    aesv8-gcm-armv8.${ASM_EXT}
    aesv8-gcm-armv8-unroll8.${ASM_EXT}
    armv8-mont.${ASM_EXT}
    bn-armv8.${ASM_EXT}
    ghash-neon-armv8.${ASM_EXT}
    ghashv8-armx.${ASM_EXT}
    keccak1600-armv8.${ASM_EXT}
    md5-armv8.${ASM_EXT}
    p256-armv8-asm.${ASM_EXT}
    p256_beeu-armv8-asm.${ASM_EXT}
    rndr-armv8.${ASM_EXT}
    sha1-armv8.${ASM_EXT}
    sha256-armv8.${ASM_EXT}
    sha512-armv8.${ASM_EXT}
    vpaes-armv8.${ASM_EXT}
  )
endif()

if(ARCH STREQUAL "ppc64le")
  set(
    BCM_ASM_SOURCES

    aesp8-ppc.${ASM_EXT}
    ghashp8-ppc.${ASM_EXT}
  )
endif()

if(PERL_EXECUTABLE)
  perlasm(aesni-gcm-x86_64.${ASM_EXT} modes/asm/aesni-gcm-x86_64.pl)
  perlasm(aesni-gcm-avx512.${ASM_EXT} modes/asm/aesni-gcm-avx512.pl)
  perlasm(aesni-xts-avx512.${ASM_EXT} aes/asm/aesni-xts-avx512.pl)
  perlasm(aesni-x86_64.${ASM_EXT} aes/asm/aesni-x86_64.pl)
  perlasm(aesni-x86.${ASM_EXT} aes/asm/aesni-x86.pl)
  perlasm(aesp8-ppc.${ASM_EXT} aes/asm/aesp8-ppc.pl)
  perlasm(aesv8-armx.${ASM_EXT} aes/asm/aesv8-armx.pl)
  perlasm(aesv8-gcm-armv8.${ASM_EXT} modes/asm/aesv8-gcm-armv8.pl)
  perlasm(aesv8-gcm-armv8-unroll8.${ASM_EXT} modes/asm/aesv8-gcm-armv8-unroll8.pl)
  perlasm(armv4-mont.${ASM_EXT} bn/asm/armv4-mont.pl)
  perlasm(armv8-mont.${ASM_EXT} bn/asm/armv8-mont.pl)
  perlasm(bn-586.${ASM_EXT} bn/asm/bn-586.pl)
  perlasm(bn-armv8.${ASM_EXT} bn/asm/bn-armv8.pl)
  perlasm(bsaes-armv7.${ASM_EXT} aes/asm/bsaes-armv7.pl)
  perlasm(co-586.${ASM_EXT} bn/asm/co-586.pl)
  perlasm(ghash-armv4.${ASM_EXT} modes/asm/ghash-armv4.pl)
  perlasm(ghashp8-ppc.${ASM_EXT} modes/asm/ghashp8-ppc.pl)
  perlasm(ghashv8-armx.${ASM_EXT} modes/asm/ghashv8-armx.pl)
  perlasm(ghash-neon-armv8.${ASM_EXT} modes/asm/ghash-neon-armv8.pl)
  perlasm(ghash-ssse3-x86_64.${ASM_EXT} modes/asm/ghash-ssse3-x86_64.pl)
  perlasm(ghash-ssse3-x86.${ASM_EXT} modes/asm/ghash-ssse3-x86.pl)
  perlasm(ghash-x86_64.${ASM_EXT} modes/asm/ghash-x86_64.pl)
  perlasm(ghash-x86.${ASM_EXT} modes/asm/ghash-x86.pl)
  perlasm(keccak1600-armv8.${ASM_EXT} sha/asm/keccak1600-armv8.pl)
  perlasm(md5-586.${ASM_EXT} md5/asm/md5-586.pl)
  perlasm(md5-armv8.${ASM_EXT} md5/asm/md5-armv8.pl)
  perlasm(md5-x86_64.${ASM_EXT} md5/asm/md5-x86_64.pl)
  perlasm(p256-x86_64-asm.${ASM_EXT} ec/asm/p256-x86_64-asm.pl)
  perlasm(p256_beeu-x86_64-asm.${ASM_EXT} ec/asm/p256_beeu-x86_64-asm.pl)
  perlasm(p256-armv8-asm.${ASM_EXT} ec/asm/p256-armv8-asm.pl)
  perlasm(p256_beeu-armv8-asm.${ASM_EXT} ec/asm/p256_beeu-armv8-asm.pl)
  perlasm(rdrand-x86_64.${ASM_EXT} rand/asm/rdrand-x86_64.pl)
  perlasm(rndr-armv8.${ASM_EXT} rand/asm/rndr-armv8.pl)
  perlasm(rsaz-avx2.${ASM_EXT} bn/asm/rsaz-avx2.pl)
  perlasm(rsaz-2k-avx512.${ASM_EXT} bn/asm/rsaz-2k-avx512.pl)
  perlasm(rsaz-3k-avx512.${ASM_EXT} bn/asm/rsaz-3k-avx512.pl)
  perlasm(rsaz-4k-avx512.${ASM_EXT} bn/asm/rsaz-4k-avx512.pl)
  perlasm(sha1-586.${ASM_EXT} sha/asm/sha1-586.pl)
  perlasm(sha1-armv4-large.${ASM_EXT} sha/asm/sha1-armv4-large.pl)
  perlasm(sha1-armv8.${ASM_EXT} sha/asm/sha1-armv8.pl)
  perlasm(sha1-x86_64.${ASM_EXT} sha/asm/sha1-x86_64.pl)
  perlasm(sha256-586.${ASM_EXT} sha/asm/sha256-586.pl)
  perlasm(sha256-armv4.${ASM_EXT} sha/asm/sha256-armv4.pl)
  perlasm(sha256-armv8.${ASM_EXT} sha/asm/sha512-armv8.pl)
  perlasm(sha256-x86_64.${ASM_EXT} sha/asm/sha512-x86_64.pl)
  perlasm(sha512-586.${ASM_EXT} sha/asm/sha512-586.pl)
  perlasm(sha512-armv4.${ASM_EXT} sha/asm/sha512-armv4.pl)
  perlasm(sha512-armv8.${ASM_EXT} sha/asm/sha512-armv8.pl)
  perlasm(sha512-x86_64.${ASM_EXT} sha/asm/sha512-x86_64.pl)
  perlasm(vpaes-armv7.${ASM_EXT} aes/asm/vpaes-armv7.pl)
  perlasm(vpaes-armv8.${ASM_EXT} aes/asm/vpaes-armv8.pl)
  perlasm(vpaes-x86_64.${ASM_EXT} aes/asm/vpaes-x86_64.pl)
  perlasm(vpaes-x86.${ASM_EXT} aes/asm/vpaes-x86.pl)
  perlasm(x86_64-mont5.${ASM_EXT} bn/asm/x86_64-mont5.pl)
  perlasm(x86_64-mont.${ASM_EXT} bn/asm/x86_64-mont.pl)
  perlasm(x86-mont.${ASM_EXT} bn/asm/x86-mont.pl)
endif()

# clang-6 (and older) knows how to compile AVX512 assembly instructions,
# but only if it's given the right flags (e.g. -mavx512*).
# The flags are not required for any other compiler we are running in the CI.
if (CLANG AND (CMAKE_ASM_COMPILER_ID MATCHES "Clang" OR CMAKE_ASM_COMPILER MATCHES "clang") AND
    (CMAKE_C_COMPILER_VERSION VERSION_LESS "7.0.0") AND (ARCH STREQUAL "x86_64"))
  set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/aesni-gcm-avx512.${ASM_EXT} PROPERTIES COMPILE_FLAGS "-mavx512f -mavx512bw -mavx512dq -mavx512vl")
  set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/aesni-xts-avx512.${ASM_EXT} PROPERTIES COMPILE_FLAGS "-mavx512f -mavx512bw -mavx512dq -mavx512vl")
  set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/rsaz-2k-avx512.${ASM_EXT} PROPERTIES COMPILE_FLAGS "-mavx512f -mavx512bw -mavx512dq -mavx512vl -mavx512ifma")
  set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/rsaz-3k-avx512.${ASM_EXT} PROPERTIES COMPILE_FLAGS "-mavx512f -mavx512bw -mavx512dq -mavx512vl -mavx512ifma")
  set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/rsaz-4k-avx512.${ASM_EXT} PROPERTIES COMPILE_FLAGS "-mavx512f -mavx512bw -mavx512dq -mavx512vl -mavx512ifma")
endif()

# s2n-bignum files can be compiled on Unix platforms for x86_64 and arm64 only.
if((((ARCH STREQUAL "x86_64") AND NOT MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX) OR
      ARCH STREQUAL "aarch64") AND UNIX)

  # Set the source directory for s2n-bignum assembly files
  if(ARCH STREQUAL "x86_64")
    set(S2N_BIGNUM_DIR "${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-imported/x86_att")
  else()
    set(S2N_BIGNUM_DIR "${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-imported/arm")
  endif()

  set(S2N_BIGNUM_INCLUDE_DIR "${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-imported/include")

  list(APPEND BCM_ASM_SOURCES

    ${S2N_BIGNUM_DIR}/p256/p256_montjscalarmul.S
    ${S2N_BIGNUM_DIR}/p256/p256_montjscalarmul_alt.S
    ${S2N_BIGNUM_DIR}/p256/bignum_montinv_p256.S

    ${S2N_BIGNUM_DIR}/p384/bignum_add_p384.S
    ${S2N_BIGNUM_DIR}/p384/bignum_sub_p384.S
    ${S2N_BIGNUM_DIR}/p384/bignum_neg_p384.S
    ${S2N_BIGNUM_DIR}/p384/bignum_tomont_p384.S
    ${S2N_BIGNUM_DIR}/p384/bignum_deamont_p384.S
    ${S2N_BIGNUM_DIR}/p384/bignum_montmul_p384.S
    ${S2N_BIGNUM_DIR}/p384/bignum_montmul_p384_alt.S
    ${S2N_BIGNUM_DIR}/p384/bignum_montsqr_p384.S
    ${S2N_BIGNUM_DIR}/p384/bignum_montsqr_p384_alt.S
    ${S2N_BIGNUM_DIR}/p384/bignum_nonzero_6.S
    ${S2N_BIGNUM_DIR}/p384/bignum_littleendian_6.S
    ${S2N_BIGNUM_DIR}/p384/p384_montjdouble.S
    ${S2N_BIGNUM_DIR}/p384/p384_montjdouble_alt.S
    ${S2N_BIGNUM_DIR}/p384/p384_montjscalarmul.S
    ${S2N_BIGNUM_DIR}/p384/p384_montjscalarmul_alt.S
    ${S2N_BIGNUM_DIR}/p384/bignum_montinv_p384.S

    ${S2N_BIGNUM_DIR}/p521/bignum_add_p521.S
    ${S2N_BIGNUM_DIR}/p521/bignum_sub_p521.S
    ${S2N_BIGNUM_DIR}/p521/bignum_neg_p521.S
    ${S2N_BIGNUM_DIR}/p521/bignum_mul_p521.S
    ${S2N_BIGNUM_DIR}/p521/bignum_mul_p521_alt.S
    ${S2N_BIGNUM_DIR}/p521/bignum_sqr_p521.S
    ${S2N_BIGNUM_DIR}/p521/bignum_sqr_p521_alt.S
    ${S2N_BIGNUM_DIR}/p521/bignum_tolebytes_p521.S
    ${S2N_BIGNUM_DIR}/p521/bignum_fromlebytes_p521.S
    ${S2N_BIGNUM_DIR}/p521/p521_jdouble.S
    ${S2N_BIGNUM_DIR}/p521/p521_jdouble_alt.S
    ${S2N_BIGNUM_DIR}/p521/p521_jscalarmul.S
    ${S2N_BIGNUM_DIR}/p521/p521_jscalarmul_alt.S
    ${S2N_BIGNUM_DIR}/p521/bignum_inv_p521.S

    ${S2N_BIGNUM_DIR}/curve25519/bignum_mod_n25519.S
    ${S2N_BIGNUM_DIR}/curve25519/bignum_neg_p25519.S
    ${S2N_BIGNUM_DIR}/curve25519/bignum_madd_n25519.S
    ${S2N_BIGNUM_DIR}/curve25519/bignum_madd_n25519_alt.S
    ${S2N_BIGNUM_DIR}/curve25519/edwards25519_decode.S
    ${S2N_BIGNUM_DIR}/curve25519/edwards25519_decode_alt.S
    ${S2N_BIGNUM_DIR}/curve25519/edwards25519_encode.S
    ${S2N_BIGNUM_DIR}/curve25519/edwards25519_scalarmulbase.S
    ${S2N_BIGNUM_DIR}/curve25519/edwards25519_scalarmulbase_alt.S
    ${S2N_BIGNUM_DIR}/curve25519/edwards25519_scalarmuldouble.S
    ${S2N_BIGNUM_DIR}/curve25519/edwards25519_scalarmuldouble_alt.S

    ${S2N_BIGNUM_DIR}/sha3/sha3_keccak_f1600.S
    ${S2N_BIGNUM_DIR}/sha3/sha3_keccak4_f1600_alt.S
  )

  if(ARCH STREQUAL "x86_64")
    # The files below contain the alternative functions for x86_64.
    # For AArch64, the alternative tomont/deamont functions are
    # the same as the non-alternative ones, so they are defined
    # in the corresponding "non-alt" files (bignum_<deamont|tomont>_p384.S)
    list(APPEND BCM_ASM_SOURCES
                ${S2N_BIGNUM_DIR}/p384/bignum_tomont_p384_alt.S
                ${S2N_BIGNUM_DIR}/p384/bignum_deamont_p384_alt.S
                ${S2N_BIGNUM_DIR}/curve25519/curve25519_x25519.S
                ${S2N_BIGNUM_DIR}/curve25519/curve25519_x25519_alt.S
                ${S2N_BIGNUM_DIR}/curve25519/curve25519_x25519base.S
                ${S2N_BIGNUM_DIR}/curve25519/curve25519_x25519base_alt.S
    )
  elseif(ARCH STREQUAL "aarch64")
    # byte-level interface for aarch64 s2n-bignum x25519 are in
    # files with "byte" tags, but ed25519 is not, neither are they byte-level...
    list(APPEND BCM_ASM_SOURCES
                ${S2N_BIGNUM_DIR}/curve25519/curve25519_x25519_byte.S
                ${S2N_BIGNUM_DIR}/curve25519/curve25519_x25519_byte_alt.S
                ${S2N_BIGNUM_DIR}/curve25519/curve25519_x25519base_byte.S
                ${S2N_BIGNUM_DIR}/curve25519/curve25519_x25519base_byte_alt.S
    )

    # Big integer arithmetics using s2n-bignum
    list(APPEND BCM_ASM_SOURCES
                ${S2N_BIGNUM_DIR}/fastmul/bignum_kmul_16_32.S
                ${S2N_BIGNUM_DIR}/fastmul/bignum_kmul_32_64.S
                ${S2N_BIGNUM_DIR}/fastmul/bignum_ksqr_16_32.S
                ${S2N_BIGNUM_DIR}/fastmul/bignum_ksqr_32_64.S
                ${S2N_BIGNUM_DIR}/fastmul/bignum_emontredc_8n.S

                ${S2N_BIGNUM_DIR}/generic/bignum_ge.S
                ${S2N_BIGNUM_DIR}/generic/bignum_mul.S
                ${S2N_BIGNUM_DIR}/generic/bignum_optsub.S
                ${S2N_BIGNUM_DIR}/generic/bignum_sqr.S

                ${S2N_BIGNUM_DIR}/generic/bignum_copy_row_from_table.S
                ${S2N_BIGNUM_DIR}/generic/bignum_copy_row_from_table_8n.S
                ${S2N_BIGNUM_DIR}/generic/bignum_copy_row_from_table_16.S
                ${S2N_BIGNUM_DIR}/generic/bignum_copy_row_from_table_32.S
    )

    #
    # Keccak assembly from s2n-bignum/mlkem-native
    #

    # Check if assembler supports SHA3 extension
    # (gcc supports it since gcc8, clang supports it since clang7)
    check_compiler("neon_sha3_check.c" MY_ASSEMBLER_SUPPORTS_NEON_SHA3_EXTENSION "-march=armv8.4-a+sha3")

    # Note: Scalar Keccak-x1 and Batched Neoverse N1 Keccak-x4 assembly from s2n-bignum
    # are included in the common s2n-bignum block above

    if(MY_ASSEMBLER_SUPPORTS_NEON_SHA3_EXTENSION)
        list(APPEND BCM_ASM_SOURCES
           # Scalar Keccak-x1 assembly from s2n-bignum/mlkem-native, using SHA3 extension
           ${S2N_BIGNUM_DIR}/sha3/sha3_keccak_f1600_alt.S

           # SIMD versions using SHA3 extension
           ${S2N_BIGNUM_DIR}/sha3/sha3_keccak2_f1600.S
           ${S2N_BIGNUM_DIR}/sha3/sha3_keccak4_f1600_alt2.S)

        set_source_files_properties(${S2N_BIGNUM_DIR}/sha3/sha3_keccak_f1600_alt.S
                                    PROPERTIES COMPILE_FLAGS "-march=armv8.4-a+sha3")
        set_source_files_properties(${S2N_BIGNUM_DIR}/sha3/sha3_keccak2_f1600.S
                                    PROPERTIES COMPILE_FLAGS "-march=armv8.4-a+sha3")
        set_source_files_properties(${S2N_BIGNUM_DIR}/sha3/sha3_keccak4_f1600_alt2.S
                                    PROPERTIES COMPILE_FLAGS "-march=armv8.4-a+sha3")
    endif()
  endif()

endif()

# Include s2n-bignum files in Windows AArch64 which replace similar implementation that was available on Windows
# TODO: Attempt more AArch64 s2n-bignum files in Windows build
if(ARCH STREQUAL "aarch64")
  set(S2N_BIGNUM_INCLUDE_DIR "${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-imported/include")
  list(APPEND BCM_ASM_SOURCES
          ${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-to-be-imported/arm/aes/aes-xts-enc.S
          ${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-to-be-imported/arm/aes/aes-xts-dec.S
  )
endif()

# mlkem-native assembly files can be compiled on Unix platforms for x86_64 and arm64 only.
if((ARCH STREQUAL "aarch64") AND UNIX)

  # Set the source directory for s2n-bignum assembly files
  set(MLKEM_NATIVE_DIR "${AWSLC_SOURCE_DIR}/crypto/fipsmodule/ml_kem")

  set(MLKEM_NATIVE_AARCH64_ASM_SOURCES

    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/intt.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/ntt.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/poly_mulcache_compute_asm.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/poly_reduce_asm.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/poly_tobytes_asm.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/poly_tomont_asm.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/polyvec_basemul_acc_montgomery_cached_asm_k2.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/polyvec_basemul_acc_montgomery_cached_asm_k3.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/polyvec_basemul_acc_montgomery_cached_asm_k4.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/aarch64/src/rej_uniform_asm.S
  )

  list(APPEND BCM_ASM_SOURCES ${MLKEM_NATIVE_AARCH64_ASM_SOURCES})

endif()

if((ARCH STREQUAL "x86_64") AND UNIX AND NOT MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)

  # Set the source directory for s2n-bignum assembly files
  set(MLKEM_NATIVE_DIR "${AWSLC_SOURCE_DIR}/crypto/fipsmodule/ml_kem")

  set(MLKEM_NATIVE_X86_64_ASM_SOURCES

    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/intt.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/ntt.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/mulcache_compute.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/nttfrombytes.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/ntttobytes.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/nttunpack.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/reduce.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/tomont.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/polyvec_basemul_acc_montgomery_cached_asm_k2.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/polyvec_basemul_acc_montgomery_cached_asm_k3.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/polyvec_basemul_acc_montgomery_cached_asm_k4.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/rej_uniform_asm.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/poly_compress_d10.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/poly_compress_d11.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/poly_compress_d4.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/poly_compress_d5.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/poly_decompress_d10.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/poly_decompress_d11.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/poly_decompress_d4.S
    ${MLKEM_NATIVE_DIR}/mlkem/native/x86_64/src/poly_decompress_d5.S
  )

  list(APPEND BCM_ASM_SOURCES ${MLKEM_NATIVE_X86_64_ASM_SOURCES})

  set(S2N_BIGNUM_INCLUDE_DIR "${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-imported/include")

endif()

# mldsa-native assembly files can be compiled on Unix platforms for x86_64
# and aarch64.
if(UNIX AND ((ARCH STREQUAL "x86_64" AND NOT MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX) OR (ARCH STREQUAL "aarch64")))

  set(MLDSA_NATIVE_DIR "${AWSLC_SOURCE_DIR}/crypto/fipsmodule/ml_dsa")

  # Every .S file in this directory is imported by importer.sh and must be
  # compiled; glob so that refreshes which add/remove files don't need a
  # matching edit here. CONFIGURE_DEPENDS makes CMake re-run when the set of
  # matching files changes.
  if(ARCH STREQUAL "x86_64")
    file(GLOB MLDSA_NATIVE_ASM_SOURCES CONFIGURE_DEPENDS
      "${MLDSA_NATIVE_DIR}/mldsa/native/x86_64/src/*.S")
  else()
    file(GLOB MLDSA_NATIVE_ASM_SOURCES CONFIGURE_DEPENDS
      "${MLDSA_NATIVE_DIR}/mldsa/native/aarch64/src/*.S")
  endif()

  list(APPEND BCM_ASM_SOURCES ${MLDSA_NATIVE_ASM_SOURCES})

  set(S2N_BIGNUM_INCLUDE_DIR "${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-imported/include")

endif()


if(FIPS_DELOCATE)
  if(FIPS_SHARED)
    message(FATAL_ERROR "Can't set both delocate and shared mode for FIPS build")
  endif()

  if(OPENSSL_NO_ASM)
    # If OPENSSL_NO_ASM was defined then ASM will not have been enabled, but in
    # FIPS mode we have to have it because the module build requires going via
    # textual assembly.
    enable_language(ASM)
  endif()

  add_library(
    bcm_c_generated_asm

    STATIC

    bcm.c
  )
  target_compile_definitions(bcm_c_generated_asm PRIVATE BORINGSSL_IMPLEMENTATION S2N_BN_HIDE_SYMBOLS)

  # Important: We do not want to add the generated prefix symbols to the include path here!
  # Delocator expects symbols to not be prefixed.
  target_add_awslc_include_paths(TARGET bcm_c_generated_asm SCOPE PRIVATE EXCLUDE_PREFIX_HEADERS)
  target_include_directories(bcm_c_generated_asm PRIVATE "${S2N_BIGNUM_INCLUDE_DIR}")
  set_target_properties(bcm_c_generated_asm PROPERTIES COMPILE_OPTIONS "-S")
  set_target_properties(bcm_c_generated_asm PROPERTIES POSITION_INDEPENDENT_CODE ON)

  get_filename_component(COMPILER_WRAPPER_PATH ${AWSLC_SOURCE_DIR}/util/compiler_wrapper/CompilerWrapper.cmake REALPATH)
  include(${COMPILER_WRAPPER_PATH})
  setup_compiler_wrapper()
  if(COMPILER_WRAPPER_AVAILABLE)
    use_compiler_wrapper_for_target(bcm_c_generated_asm)
  endif()

  set(TARGET "")
  if(CMAKE_ASM_COMPILER_TARGET)
    set(TARGET "--target=${CMAKE_ASM_COMPILER_TARGET}")
  endif()

  set(DELOCATE_EXTRA_ARGS "")
  # clang-6 (and older) do not appreciate the file number starting at a higher value, and incorrectly
  # assume that all file numbers less than that value are defined upon later use.
  if (CLANG AND (CMAKE_ASM_COMPILER_ID MATCHES "Clang" OR CMAKE_ASM_COMPILER MATCHES "clang") AND
      (CMAKE_C_COMPILER_VERSION VERSION_LESS "7.0.0"))
    set(DELOCATE_EXTRA_ARGS "-no-se-debug-directives")
  endif()

  go_executable(delocate github.com/aws/aws-lc/util/fipstools/delocate)
  add_custom_command(
    OUTPUT bcm-delocated.S
    COMMAND
    ./delocate
    -a $<TARGET_FILE:bcm_c_generated_asm>
    -o bcm-delocated.S
    -cc ${CMAKE_ASM_COMPILER}
    -cc-flags "${TARGET} ${CMAKE_ASM_FLAGS} -DS2N_BN_HIDE_SYMBOLS"
    -s2n-bignum-include "${S2N_BIGNUM_INCLUDE_DIR}"
    ${DELOCATE_EXTRA_ARGS}
    ${AWSLC_SOURCE_DIR}/include/openssl/arm_arch.h
    ${AWSLC_SOURCE_DIR}/include/openssl/asm_base.h
    ${AWSLC_SOURCE_DIR}/include/openssl/target.h
    ${BCM_ASM_SOURCES}
    DEPENDS
    bcm_c_generated_asm
    delocate
    ${BCM_ASM_SOURCES}
    ${AWSLC_SOURCE_DIR}/include/openssl/arm_arch.h
    ${AWSLC_SOURCE_DIR}/include/openssl/asm_base.h
    ${AWSLC_SOURCE_DIR}/include/openssl/target.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
  )

  # clang-6 (and older) knows how to compile AVX512 assembly instructions,
  # but only if it's given the right flags (e.g. -mavx512*).
  # The flags are not required for any other compiler we are running in the CI.
  if (CLANG AND (CMAKE_ASM_COMPILER_ID MATCHES "Clang" OR CMAKE_ASM_COMPILER MATCHES "clang") AND
      (CMAKE_C_COMPILER_VERSION VERSION_LESS "7.0.0") AND (ARCH STREQUAL "x86_64"))
    set_source_files_properties(${CMAKE_CURRENT_BINARY_DIR}/bcm-delocated.S PROPERTIES COMPILE_FLAGS "-mavx512f -mavx512bw -mavx512dq -mavx512vl -mavx512ifma")
  endif()

  add_library(
    bcm_hashunset

    STATIC

    bcm-delocated.S
  )
  target_compile_definitions(bcm_hashunset PRIVATE BORINGSSL_IMPLEMENTATION S2N_BN_HIDE_SYMBOLS)

  target_add_awslc_include_paths(TARGET bcm_hashunset SCOPE PRIVATE)
  target_include_directories(bcm_hashunset PRIVATE "${S2N_BIGNUM_INCLUDE_DIR}")

  set_target_properties(bcm_hashunset PROPERTIES POSITION_INDEPENDENT_CODE ON)
  set_target_properties(bcm_hashunset PROPERTIES LINKER_LANGUAGE C)

  go_executable(inject_hash
                github.com/aws/aws-lc/util/fipstools/inject_hash)
  add_custom_command(
    OUTPUT bcm.o
    COMMAND ./inject_hash -o bcm.o -in-archive $<TARGET_FILE:bcm_hashunset>
    DEPENDS bcm_hashunset inject_hash
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
  )

  # The outputs of add_custom_command cannot be referenced outside of the
  # CMakeLists.txt that defines it. Thus we have to wrap bcm.o in a custom target
  # so that crypto can depend on it.
  add_custom_target(bcm_o_target DEPENDS bcm.o)
  set(BCM_NAME bcm.o PARENT_SCOPE)

  add_library(
    fipsmodule

    OBJECT

    fips_shared_support.c
    cpucap/cpucap.c
  )
  target_compile_definitions(fipsmodule PRIVATE BORINGSSL_IMPLEMENTATION)

  target_add_awslc_include_paths(TARGET fipsmodule SCOPE PRIVATE)

  set_target_properties(fipsmodule PROPERTIES LINKER_LANGUAGE C)
elseif(FIPS_SHARED)
  if(NOT BUILD_SHARED_LIBS)
    message(FATAL_ERROR "FIPS_SHARED set but not BUILD_SHARED_LIBS")
  endif()

  if (ARCH STREQUAL "aarch64" AND CMAKE_GENERATOR MATCHES "Visual Studio")
    msbuild_aarch64_asm(TARGET fipsmodule ASM_FILES ${BCM_ASM_SOURCES} OUTPUT_OBJECTS BCM_ASM_OBJECTS)
  endif()

  add_library(
    fipsmodule

    OBJECT

    fips_shared_support.c
    cpucap/cpucap.c
  )
  target_compile_definitions(fipsmodule PRIVATE BORINGSSL_IMPLEMENTATION S2N_BN_HIDE_SYMBOLS)
  target_add_awslc_include_paths(TARGET fipsmodule SCOPE PRIVATE)
  target_include_directories(fipsmodule PRIVATE "${S2N_BIGNUM_INCLUDE_DIR}")

  add_library(
    bcm_library

    STATIC

    bcm.c

    ${BCM_ASM_SOURCES}
  )
  target_compile_definitions(bcm_library PRIVATE BORINGSSL_IMPLEMENTATION S2N_BN_HIDE_SYMBOLS )
  target_add_awslc_include_paths(TARGET bcm_library SCOPE PRIVATE)
  target_include_directories(bcm_library PRIVATE "${S2N_BIGNUM_INCLUDE_DIR}")

  # On GCC 14+ the Superword Level Parallelism (SLP) vectorizer causes the
  # compiler to aggressively store static function pointer addresses in the
  # .data.rel.ro.local section which is discarded by our linker script and
  # results in undefined references when linking libcrypto.
  if(CMAKE_C_COMPILER_ID STREQUAL "GNU" AND
     CMAKE_C_COMPILER_VERSION VERSION_GREATER_EQUAL "14" AND
     CMAKE_SYSTEM_NAME STREQUAL "Linux" AND
     ARCH STREQUAL "x86_64")
    target_compile_options(bcm_library PRIVATE -fno-tree-slp-vectorize)
  endif()

  if (APPLE)
    set(BCM_NAME bcm.o)
    # The linker on macOS doesn't have the ability to process linker scripts,
    # so we build the FIPS module differently than on Linux. Similarly to
    # what OpenSSL does we produce two object files: fips_apple_{start, end}.o
    # that contain only the markers for the start and end of __text and __const
    # sections. The module is then produced by linking the files specified in
    # the following order: fips_apple_start.o bcm.o fips_apple_end.o. This will
    # generate the output object file where all the code in the __text section
    # and all the read-only data in the __const section are between the
    # respective start and end markers.
    if (CMAKE_OSX_DEPLOYMENT_TARGET)
      if(IOS)
        set(OSX_VERSION_MIN_FLAG "-miphoneos-version-min=${CMAKE_OSX_DEPLOYMENT_TARGET}")
      else()
        set(OSX_VERSION_MIN_FLAG "-mmacosx-version-min=${CMAKE_OSX_DEPLOYMENT_TARGET}")
      endif()
    endif()
    if(CMAKE_OSX_SYSROOT)
      set(FIPS_EXTRA_FLAGS -isysroot \"${CMAKE_OSX_SYSROOT}\")
    endif()
    add_custom_command(
      OUTPUT fips_apple_start.o
      COMMAND ${CMAKE_C_COMPILER} -arch ${CMAKE_SYSTEM_PROCESSOR} ${FIPS_EXTRA_FLAGS} ${OSX_VERSION_MIN_FLAG} -c ${CMAKE_CURRENT_SOURCE_DIR}/fips_shared_library_marker.c -DAWSLC_FIPS_SHARED_START -o fips_apple_start.o
      DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/fips_shared_library_marker.c
    )
    add_custom_command(
      OUTPUT fips_apple_end.o
      COMMAND ${CMAKE_C_COMPILER} -arch ${CMAKE_SYSTEM_PROCESSOR} ${FIPS_EXTRA_FLAGS} ${OSX_VERSION_MIN_FLAG} -c ${CMAKE_CURRENT_SOURCE_DIR}/fips_shared_library_marker.c -DAWSLC_FIPS_SHARED_END -o fips_apple_end.o
      DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/fips_shared_library_marker.c
    )
    add_custom_command(
      OUTPUT bcm.o
      COMMAND ${CMAKE_LINKER} -r fips_apple_start.o -force_load $<TARGET_FILE:bcm_library> fips_apple_end.o -keep_private_externs -o bcm.o
      DEPENDS bcm_library fips_apple_start.o fips_apple_end.o
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  elseif(MSVC)
    set(BCM_NAME bcm.lib)
    # Like the macOS linker MSVC doesn't support linker scripts. The Windows build uses the same start/end markers
    # as the mac build to control the placement of symbols. However, on Windows the start/bcm/end object files also
    # contain pragmas that instruct the compiler to put the code in specific FIPS sections. These sections have a
    # specific suffix $a through $c that controls the order of the FIPS sections when it is linked together into a
    # Grouped Section.
    #
    # These custom commands invoke the compiler directly (outside CMake's normal
    # target compilation), so we must forward CMAKE_C_FLAGS manually. This is
    # necessary for cross-compilation where the flags contain --target and
    # system include paths (e.g. clang-cl + xwin on Linux).
    separate_arguments(FIPS_MARKER_C_FLAGS NATIVE_COMMAND "${CMAKE_C_FLAGS}")
    add_custom_command(
      OUTPUT fips_msvc_start.obj
      COMMAND ${CMAKE_C_COMPILER} ${FIPS_MARKER_C_FLAGS} -w /nologo /c /DAWSLC_FIPS_SHARED_START /Fofips_msvc_start.obj ${CMAKE_CURRENT_SOURCE_DIR}/fips_shared_library_marker.c
      DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/fips_shared_library_marker.c
    )
    add_custom_command(
      OUTPUT fips_msvc_end.obj
      COMMAND ${CMAKE_C_COMPILER} ${FIPS_MARKER_C_FLAGS} -w /nologo /c /DAWSLC_FIPS_SHARED_END /Fofips_msvc_end.obj ${CMAKE_CURRENT_SOURCE_DIR}/fips_shared_library_marker.c
      DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/fips_shared_library_marker.c
    )

    get_filename_component(MSVC_BIN ${CMAKE_LINKER} DIRECTORY)
    find_program(MSVC_LIB NAMES lib lib.exe llvm-lib llvm-lib.exe
                 HINTS ${MSVC_BIN} NO_DEFAULT_PATH)
    if(NOT MSVC_LIB)
      find_program(MSVC_LIB NAMES lib lib.exe llvm-lib llvm-lib.exe)
    endif()
    if(NOT MSVC_LIB)
      message(FATAL_ERROR "Could not find lib.exe or llvm-lib for creating bcm.lib")
    endif()

    file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/bcm_objects.rsp"
         CONTENT "\"$<JOIN:$<TARGET_OBJECTS:bcm_library>,\"\n\">\"")
    add_custom_command(
      OUTPUT ${BCM_NAME}
      COMMAND ${MSVC_LIB} /nologo fips_msvc_start.obj @bcm_objects.rsp fips_msvc_end.obj /OUT:${BCM_NAME}
      DEPENDS fips_msvc_start.obj fips_msvc_end.obj bcm_library
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  else()
    set(BCM_NAME bcm.o)
    # fips_shared.lds does not have 'clang' prefix because we want to keep merging any changes from upstream.
    set(FIPS_CUSTOM_LINKER_SCRIPT "${CMAKE_CURRENT_SOURCE_DIR}/fips_shared.lds")
    if (GCC)
      # gcc puts some code in sections named ".text.unlikely", ".text.exit" and ".text.startup".
      # so we have a separate linker script for gcc.
      set(FIPS_CUSTOM_LINKER_SCRIPT "${CMAKE_CURRENT_SOURCE_DIR}/gcc_fips_shared.lds")
    endif()

    add_custom_command(
      OUTPUT ${BCM_NAME}
      COMMAND ${CMAKE_LINKER} -r -T ${FIPS_CUSTOM_LINKER_SCRIPT} -o ${BCM_NAME} --whole-archive $<TARGET_FILE:bcm_library>
      DEPENDS bcm_library ${FIPS_CUSTOM_LINKER_SCRIPT}
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endif()

  add_custom_target(bcm_o_target DEPENDS ${BCM_NAME})
  set(BCM_NAME ${BCM_NAME} PARENT_SCOPE)

else()
  set(BCM_ASM_OBJECTS "")
  if (ARCH STREQUAL "aarch64" AND CMAKE_GENERATOR MATCHES "Visual Studio")
    msbuild_aarch64_asm(TARGET fipsmodule ASM_FILES ${BCM_ASM_SOURCES} OUTPUT_OBJECTS BCM_ASM_OBJECTS)
  endif()

  add_library(
    fipsmodule

    OBJECT

    bcm.c
    fips_shared_support.c
    cpucap/cpucap.c

    ${BCM_ASM_SOURCES}
    ${BCM_ASM_OBJECTS}
  )
  target_compile_definitions(fipsmodule PRIVATE BORINGSSL_IMPLEMENTATION S2N_BN_HIDE_SYMBOLS)
  target_add_awslc_include_paths(TARGET fipsmodule SCOPE PRIVATE)
  target_include_directories(fipsmodule PRIVATE "${S2N_BIGNUM_INCLUDE_DIR}")
endif()
