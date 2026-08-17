
# Function to handle assembly files for ARM64 targets when using MSBuild (the "Visual Studio" generator)
# This function is necessary because MSBuild ignores ARM64 assembly file dependencies
include(CMakeParseArguments)
function(msbuild_aarch64_asm)
  set(options "")
  set(oneValueArgs TARGET OUTPUT_OBJECTS)
  set(multiValueArgs ASM_FILES)

  cmake_parse_arguments(MSBUILD_AARCH64_ASM "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGV})

  # Set the output directory for object files
  set(OBJ_DIR "${CMAKE_CURRENT_BINARY_DIR}/${MSBUILD_AARCH64_ASM_TARGET}.dir/$<CONFIG>")

  # Initialize list to store assembled object files
  set(ASSEMBLED_OBJECTS "")

  foreach(ASM_FILE ${MSBUILD_AARCH64_ASM_ASM_FILES})
    # Get the filename without extension
    get_filename_component(ASM_NAME ${ASM_FILE} NAME_WE)
    set(OBJ_FILE "${OBJ_DIR}/${ASM_NAME}.obj")

    add_custom_command(
      OUTPUT ${OBJ_FILE}
      COMMAND "${CMAKE_ASM_COMPILER}"
              --target=arm64-pc-windows-msvc
              /c
              /I "${AWSLC_BINARY_DIR}/symbol_prefix_include"
              /I "${AWSLC_SOURCE_DIR}/include"
              /I "${AWSLC_SOURCE_DIR}/third_party/s2n-bignum/s2n-bignum-imported/include"
              /o "${OBJ_FILE}"
              "${ASM_FILE}"
      DEPENDS ${ASM_FILE}
      COMMENT "Assembling ${ASM_FILE}"
    )

    # Mark the generated object file as an external object
    set_source_files_properties(${OBJ_FILE} PROPERTIES EXTERNAL_OBJECT TRUE)

    list(APPEND ASSEMBLED_OBJECTS ${OBJ_FILE})
  endforeach()

  # Set the output variable in the parent scope
  set(${MSBUILD_AARCH64_ASM_OUTPUT_OBJECTS} ${ASSEMBLED_OBJECTS} PARENT_SCOPE)
endfunction()

if(NOT OPENSSL_NO_ASM)
  if(NOT WIN32)
    if(ARCH STREQUAL "aarch64")
      # The "armx" Perl scripts look for "64" in the style argument
      # in order to decide whether to generate 32- or 64-bit asm.
      if(APPLE)
        set(PERLASM_STYLE ios64)
      else()
        set(PERLASM_STYLE linux64)
      endif()
    elseif(ARCH STREQUAL "arm")
      if(APPLE)
        set(PERLASM_STYLE ios32)
      else()
        set(PERLASM_STYLE linux32)
      endif()
    elseif(ARCH STREQUAL "ppc64le")
      set(PERLASM_STYLE linux64le)
    else()
      if(ARCH STREQUAL "x86")
        set(PERLASM_FLAGS "-fPIC")
      endif()
      if(APPLE)
        set(PERLASM_STYLE macosx)
      else()
        set(PERLASM_STYLE elf)
      endif()
    endif()
    set(ASM_EXT S)
    enable_language(ASM)
    set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -Wa,--noexecstack")

    # Since "--Wa,--noexecstack" is not used during the preprocessor step of Android (because assembler is not invoked),
    # Clang reports that argument as unused. We quiet the unused argument warning for Android as it's expected behavior.
    if(ANDROID)
      set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -Qunused-arguments")
    endif()

    if(CMAKE_BUILD_TYPE_LOWER STREQUAL "debug" OR CMAKE_BUILD_TYPE_LOWER STREQUAL "relwithdebinfo" )
      if (CMAKE_ASM_COMPILER_ID MATCHES "Clang" OR CMAKE_ASM_COMPILER MATCHES "clang")
        set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -g")
      else()
        set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -Wa,-g")
      endif()
    endif()

    # Work around https://gitlab.kitware.com/cmake/cmake/-/issues/20771 in older
    # CMake versions.
    if(APPLE AND CMAKE_VERSION VERSION_LESS 3.19)
      if(CMAKE_OSX_SYSROOT)
        set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -isysroot \"${CMAKE_OSX_SYSROOT}\"")
      endif()
      foreach(arch ${CMAKE_OSX_ARCHITECTURES})
        set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -arch ${arch}")
      endforeach()
    endif()
  else()
    if(ARCH STREQUAL "aarch64")
      set(PERLASM_STYLE win64)
      if("${CMAKE_BUILD_TYPE_LOWER}" STREQUAL "relwithdebinfo" OR
              "${CMAKE_BUILD_TYPE_LOWER}" STREQUAL "debug")
        # Provide debug in the default format
        set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -g")
      endif()

      set(ASM_EXT S)
      enable_language(ASM)
    else()
      if(ARCH STREQUAL "x86_64")
        set(PERLASM_STYLE nasm)
      else()
        set(PERLASM_STYLE win32n)
      endif()
      find_program(NASM_EXECUTABLE nasm)
      set(CMAKE_ASM_NASM_COMPILER ${NASM_EXECUTABLE})
      if("${CMAKE_BUILD_TYPE_LOWER}" STREQUAL "relwithdebinfo" OR
              "${CMAKE_BUILD_TYPE_LOWER}" STREQUAL "debug")
        # Provide debug in the default format
        set(CMAKE_ASM_NASM_FLAGS "${CMAKE_ASM_NASM_FLAGS} -g")
      endif()

      # On Windows, we use the NASM output.
      set(ASM_EXT asm)
      enable_language(ASM_NASM)
    endif()
  endif()
endif()

if(PERL_EXECUTABLE)
  function(perlasm dest src)
    get_filename_component(dir ${dest} DIRECTORY)
    if (dir STREQUAL "")
      set(dir ".")
    endif()

    if(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
      set(PERLASM_FLAGS "${PERLASM_FLAGS} -DMY_ASSEMBLER_IS_TOO_OLD_FOR_AVX")
    endif()

    if(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
      set(PERLASM_FLAGS "${PERLASM_FLAGS} -DMY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX")
    endif()

    set(PERL_COMMAND "${PERL_EXECUTABLE}")
    list(APPEND PERL_COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/${src}")
    list(APPEND PERL_COMMAND "${PERLASM_STYLE}")
    list(APPEND PERL_COMMAND "${dest}")
    list(APPEND PERL_COMMAND ${PERLASM_FLAGS})

    add_custom_command(
      OUTPUT ${dest}
      COMMAND ${CMAKE_COMMAND} -E make_directory ${dir}
      COMMAND ${PERL_COMMAND}
      DEPENDS
      ${src}
      ${AWSLC_SOURCE_DIR}/crypto/perlasm/arm-xlate.pl
      ${AWSLC_SOURCE_DIR}/crypto/perlasm/ppc-xlate.pl
      ${AWSLC_SOURCE_DIR}/crypto/perlasm/x86_64-xlate.pl
      ${AWSLC_SOURCE_DIR}/crypto/perlasm/x86asm.pl
      ${AWSLC_SOURCE_DIR}/crypto/perlasm/x86gas.pl
      ${AWSLC_SOURCE_DIR}/crypto/perlasm/x86masm.pl
      ${AWSLC_SOURCE_DIR}/crypto/perlasm/x86nasm.pl
      WORKING_DIRECTORY .
    )
  endfunction()
  perlasm(chacha/chacha-armv4.${ASM_EXT} chacha/asm/chacha-armv4.pl)
  perlasm(chacha/chacha-armv8.${ASM_EXT} chacha/asm/chacha-armv8.pl)
  perlasm(chacha/chacha-x86.${ASM_EXT} chacha/asm/chacha-x86.pl)
  perlasm(chacha/chacha-x86_64.${ASM_EXT} chacha/asm/chacha-x86_64.pl)
  perlasm(cipher_extra/aes128gcmsiv-x86_64.${ASM_EXT} cipher_extra/asm/aes128gcmsiv-x86_64.pl)
  perlasm(cipher_extra/chacha20_poly1305_x86_64.${ASM_EXT} cipher_extra/asm/chacha20_poly1305_x86_64.pl)
  perlasm(cipher_extra/chacha20_poly1305_armv8.${ASM_EXT} cipher_extra/asm/chacha20_poly1305_armv8.pl)
  perlasm(cipher_extra/aesni-sha1-x86_64.${ASM_EXT} cipher_extra/asm/aesni-sha1-x86_64.pl)
  perlasm(cipher_extra/aesni-sha256-x86_64.${ASM_EXT} cipher_extra/asm/aesni-sha256-x86_64.pl)
  perlasm(test/trampoline-armv4.${ASM_EXT} test/asm/trampoline-armv4.pl)
  perlasm(test/trampoline-armv8.${ASM_EXT} test/asm/trampoline-armv8.pl)
  perlasm(test/trampoline-ppc.${ASM_EXT} test/asm/trampoline-ppc.pl)
  perlasm(test/trampoline-x86.${ASM_EXT} test/asm/trampoline-x86.pl)
  perlasm(test/trampoline-x86_64.${ASM_EXT} test/asm/trampoline-x86_64.pl)

else()
  if(APPLE AND ${ARCH} STREQUAL "aarch64")
    set(ASSEMBLY_SOURCE ${GENERATE_CODE_ROOT}/ios-aarch64/crypto/)
  elseif(APPLE AND ${ARCH} STREQUAL "arm")
    set(ASSEMBLY_SOURCE ${GENERATE_CODE_ROOT}/ios-arm/crypto/)
  elseif(APPLE)
    set(ASSEMBLY_SOURCE ${GENERATE_CODE_ROOT}/mac-${ARCH}/crypto/)
  elseif(NOT WIN32)
    if(${ARCH} STREQUAL "generic")
      message(STATUS "Detected generic linux platform. No assembly files will be included.")
    else()
      set(ASSEMBLY_SOURCE ${GENERATE_CODE_ROOT}/linux-${ARCH}/crypto/)
    endif()
  elseif(WIN32)
    set(ASSEMBLY_SOURCE ${GENERATE_CODE_ROOT}/win-${ARCH}/crypto/)
  endif()

  if(DEFINED ASSEMBLY_SOURCE)
    if(EXISTS ${ASSEMBLY_SOURCE})
      message("Copying platform assembly files from ${ASSEMBLY_SOURCE} to ${CMAKE_CURRENT_BINARY_DIR}")
      file(COPY ${ASSEMBLY_SOURCE}/ DESTINATION ${CMAKE_CURRENT_BINARY_DIR} NO_SOURCE_PERMISSIONS)
    else()
      message(STATUS "${ASSEMBLY_SOURCE} assembly files do not exist.")
    endif()
  endif()

endif()
add_subdirectory(fipsmodule)

if(FIPS_DELOCATE OR FIPS_SHARED)
  SET_SOURCE_FILES_PROPERTIES(fipsmodule/${BCM_NAME} PROPERTIES EXTERNAL_OBJECT true)
  SET_SOURCE_FILES_PROPERTIES(fipsmodule/${BCM_NAME} PROPERTIES GENERATED true)

  set(
    CRYPTO_FIPS_OBJECTS

    fipsmodule/${BCM_NAME}
  )
endif()

if(ARCH STREQUAL "arm")
  set(
    CRYPTO_ARCH_SOURCES

    chacha/chacha-armv4.${ASM_EXT}
    poly1305/poly1305_arm_asm.S
    test/trampoline-armv4.${ASM_EXT}
  )
endif()

if(ARCH STREQUAL "aarch64")
  set(
    CRYPTO_ARCH_SOURCES

    chacha/chacha-armv8.${ASM_EXT}
    test/trampoline-armv8.${ASM_EXT}
    cipher_extra/chacha20_poly1305_armv8.${ASM_EXT}
  )
endif()

if(ARCH STREQUAL "ppc64le")
  set(
    CRYPTO_ARCH_SOURCES

    test/trampoline-ppc.${ASM_EXT}
  )
endif()

if(ARCH STREQUAL "x86")
  set(
    CRYPTO_ARCH_SOURCES

    chacha/chacha-x86.${ASM_EXT}
    test/trampoline-x86.${ASM_EXT}
  )
endif()

if(ARCH STREQUAL "x86_64")
  set(
    CRYPTO_ARCH_SOURCES

    chacha/chacha-x86_64.${ASM_EXT}
    cipher_extra/chacha20_poly1305_x86_64.${ASM_EXT}
    cipher_extra/aes128gcmsiv-x86_64.${ASM_EXT}
    cipher_extra/aesni-sha1-x86_64.${ASM_EXT}
    cipher_extra/aesni-sha256-x86_64.${ASM_EXT}
    test/trampoline-x86_64.${ASM_EXT}
    hrss/asm/poly_rq_mul.S
  )
endif()

if(GO_EXECUTABLE)
  add_custom_command(
    OUTPUT err_data.c
    COMMAND ${GO_EXECUTABLE} run err_data_generate.go > ${CMAKE_CURRENT_BINARY_DIR}/err_data.c
    DEPENDS
    err/err_data_generate.go
    err/asn1.errordata
    err/bio.errordata
    err/bn.errordata
    err/cipher.errordata
    err/conf.errordata
    err/dh.errordata
    err/digest.errordata
    err/dsa.errordata
    err/ecdh.errordata
    err/ecdsa.errordata
    err/ec.errordata
    err/engine.errordata
    err/evp.errordata
    err/hkdf.errordata
    err/hmac.errordata
    err/obj.errordata
    err/ocsp.errordata
    err/pem.errordata
    err/pkcs7.errordata
    err/pkcs8.errordata
    err/rsa.errordata
    err/ssl.errordata
    err/trust_token.errordata
    err/x509.errordata
    err/x509v3.errordata
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/err
  )
else()
  file(COPY ${GENERATE_CODE_ROOT}/err_data.c DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/)
endif()

set(CRYPTO_ARCH_OBJECTS "")
if (ARCH STREQUAL "aarch64" AND CMAKE_GENERATOR MATCHES "Visual Studio")
  msbuild_aarch64_asm(TARGET crypto_objects ASM_FILES ${CRYPTO_ARCH_SOURCES} OUTPUT_OBJECTS CRYPTO_ARCH_OBJECTS)
endif()

add_library(
  crypto_objects
  OBJECT
  asn1/a_bitstr.c
  asn1/a_bool.c
  asn1/a_d2i_fp.c
  asn1/a_dup.c
  asn1/a_gentm.c
  asn1/a_i2d_fp.c
  asn1/a_int.c
  asn1/a_mbstr.c
  asn1/a_object.c
  asn1/a_octet.c
  asn1/a_strex.c
  asn1/a_strnid.c
  asn1/a_time.c
  asn1/a_type.c
  asn1/a_utctm.c
  asn1/a_utf8.c
  asn1/asn1_lib.c
  asn1/asn1_par.c
  asn1/asn_pack.c
  asn1/f_int.c
  asn1/f_string.c
  asn1/tasn_dec.c
  asn1/tasn_enc.c
  asn1/tasn_fre.c
  asn1/tasn_new.c
  asn1/tasn_typ.c
  asn1/tasn_utl.c
  asn1/posix_time.c
  base64/base64.c
  bio/bio.c
  bio/bio_addr.c
  bio/bio_mem.c
  bio/connect.c
  bio/dgram.c
  bio/errno.c
  bio/fd.c
  bio/file.c
  bio/hexdump.c
  bio/md.c
  bio/pair.c
  bio/printf.c
  bio/socket.c
  bio/socket_helper.c
  blake2/blake2.c
  bn_extra/bn_asn1.c
  bn_extra/convert.c
  buf/buf.c
  bytestring/asn1_compat.c
  bytestring/ber.c
  bytestring/cbb.c
  bytestring/cbs.c
  bytestring/unicode.c
  chacha/chacha.c
  cipher_extra/cipher_extra.c
  cipher_extra/derive_key.c
  cipher_extra/e_aesctrhmac.c
  cipher_extra/e_aesgcmsiv.c
  cipher_extra/e_chacha20poly1305.c
  cipher_extra/e_aes_cbc_hmac_sha1.c
  cipher_extra/e_aes_cbc_hmac_sha256.c
  cipher_extra/e_des.c
  cipher_extra/e_null.c
  cipher_extra/e_rc2.c
  cipher_extra/e_rc4.c
  cipher_extra/e_tls.c
  cipher_extra/tls_cbc.c
  conf/conf.c
  console/console.c
  crypto.c
  des/des.c
  dh_extra/params.c
  dh_extra/dh_asn1.c
  digest_extra/digest_extra.c
  dsa/dsa.c
  dsa/dsa_asn1.c
  ecdh_extra/ecdh_extra.c
  ecdsa_extra/ecdsa_asn1.c
  ec_extra/ec_asn1.c
  ec_extra/ec_derive.c
  ec_extra/hash_to_curve.c
  err/err.c
  err_data.c
  engine/engine.c
  evp_extra/evp_asn1.c
  evp_extra/p_dh.c
  evp_extra/p_dh_asn1.c
  evp_extra/p_dsa.c
  evp_extra/p_dsa_asn1.c
  evp_extra/p_ec_asn1.c
  evp_extra/p_ed25519_asn1.c
  evp_extra/p_hmac_asn1.c
  evp_extra/p_kem_asn1.c
  evp_extra/p_pqdsa_asn1.c
  evp_extra/p_rsa_asn1.c
  evp_extra/p_x25519.c
  evp_extra/p_x25519_asn1.c
  evp_extra/p_methods.c
  evp_extra/print.c
  evp_extra/scrypt.c
  evp_extra/sign.c
  ex_data.c
  hpke/hpke.c
  hrss/hrss.c
  lhash/lhash.c
  md4/md4.c
  mem.c
  obj/obj.c
  obj/obj_xref.c
  ocsp/ocsp_asn.c
  ocsp/ocsp_client.c
  ocsp/ocsp_extension.c
  ocsp/ocsp_http.c
  ocsp/ocsp_lib.c
  ocsp/ocsp_print.c
  ocsp/ocsp_server.c
  ocsp/ocsp_verify.c
  pem/pem_all.c
  pem/pem_info.c
  pem/pem_lib.c
  pem/pem_oth.c
  pem/pem_pk8.c
  pem/pem_pkey.c
  pem/pem_x509.c
  pem/pem_xaux.c
  pkcs7/bio/cipher.c
  pkcs7/pkcs7.c
  pkcs7/pkcs7_asn1.c
  pkcs7/pkcs7_x509.c
  pkcs8/pkcs8.c
  pkcs8/pkcs8_x509.c
  pkcs8/p5_pbev2.c
  poly1305/poly1305.c
  poly1305/poly1305_arm.c
  poly1305/poly1305_vec.c
  pool/pool.c
  rand_extra/ccrandomgeneratebytes.c
  rand_extra/deterministic.c
  rand_extra/getentropy.c
  rand_extra/rand_extra.c
  rand_extra/vm_ube_fallback.c
  rand_extra/urandom.c
  rand_extra/windows.c
  rc4/rc4.c
  refcount_c11.c
  refcount_lock.c
  refcount_win.c
  rsa_extra/rsa_asn1.c
  rsa_extra/rsassa_pss_asn1.c
  rsa_extra/rsa_crypt.c
  rsa_extra/rsa_print.c
  stack/stack.c
  siphash/siphash.c
  spake25519/spake25519.c
  thread.c
  thread_none.c
  thread_pthread.c
  thread_win.c
  trust_token/pmbtoken.c
  trust_token/trust_token.c
  trust_token/voprf.c
  ube/ube.c
  ube/fork_ube_detect.c
  ube/vm_ube_detect.c
  x509/a_digest.c
  x509/a_sign.c
  x509/a_verify.c
  x509/algorithm.c
  x509/asn1_gen.c
  x509/by_dir.c
  x509/by_file.c
  x509/i2d_pr.c
  x509/name_print.c
  x509/policy.c
  x509/rsa_pss.c
  x509/t_crl.c
  x509/t_req.c
  x509/t_x509.c
  x509/t_x509a.c
  x509/v3_akey.c
  x509/v3_akeya.c
  x509/v3_alt.c
  x509/v3_bcons.c
  x509/v3_bitst.c
  x509/v3_conf.c
  x509/v3_cpols.c
  x509/v3_crld.c
  x509/v3_enum.c
  x509/v3_extku.c
  x509/v3_genn.c
  x509/v3_ia5.c
  x509/v3_info.c
  x509/v3_int.c
  x509/v3_lib.c
  x509/v3_ncons.c
  x509/v3_ocsp.c
  x509/v3_pcons.c
  x509/v3_pmaps.c
  x509/v3_prn.c
  x509/v3_purp.c
  x509/v3_skey.c
  x509/v3_utl.c
  x509/x_algor.c
  x509/x_all.c
  x509/x_attrib.c
  x509/x_crl.c
  x509/x_exten.c
  x509/x_name.c
  x509/x_pubkey.c
  x509/x_req.c
  x509/x_sig.c
  x509/x_spki.c
  x509/x_val.c
  x509/x_x509.c
  x509/x_x509a.c
  x509/x509_att.c
  x509/x509_cmp.c
  x509/x509_d2.c
  x509/x509_def.c
  x509/x509_ext.c
  x509/x509_lu.c
  x509/x509_obj.c
  x509/x509_req.c
  x509/x509_set.c
  x509/x509_trs.c
  x509/x509_txt.c
  x509/x509_v3.c
  x509/x509_vfy.c
  x509/x509_vpm.c
  x509/x509.c
  x509/x509cset.c
  x509/x509name.c
  x509/x509rset.c
  x509/x509spki.c
  ui/ui.c
  decrepit/bio/base64_bio.c
  decrepit/blowfish/blowfish.c
  decrepit/cast/cast.c
  decrepit/cast/cast_tables.c
  decrepit/cfb/cfb.c
  decrepit/dh/dh_decrepit.c
  decrepit/evp/evp_do_all.c
  decrepit/obj/obj_decrepit.c
  decrepit/ripemd/ripemd.c
  decrepit/rsa/rsa_decrepit.c
  decrepit/x509/x509_decrepit.c

  ${CRYPTO_ARCH_SOURCES}
  ${CRYPTO_ARCH_OBJECTS}
)

target_compile_definitions(crypto_objects PRIVATE BORINGSSL_IMPLEMENTATION)
target_add_awslc_include_paths(TARGET crypto_objects SCOPE PRIVATE)

function(build_libcrypto)
  set(options SET_OUTPUT_NAME)
  set(oneValueArgs NAME)
  set(multiValueArgs MODULE_SOURCE)

  if(CMAKE_VERSION VERSION_LESS "3.7")
    cmake_parse_arguments(arg "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})
  else()
    cmake_parse_arguments(PARSE_ARGV 0 arg "${options}" "${oneValueArgs}" "${multiValueArgs}")
  endif()

  if(arg_UNPARSED_ARGUMENTS)
    message(FATAL_ERROR "Unparsed arguments to build_libcrypto: ${arg_UNPARSED_ARGUMENTS}")
  endif()

  if(arg_KEYWORDS_MISSING_VALUES)
    message(FATAL_ERROR "Missing value(s) for keyword(s) to build_libcrypto: ${arg_KEYWORDS_MISSING_VALUES}")
  endif()

  if(NOT DEFINED arg_NAME OR NOT DEFINED arg_MODULE_SOURCE)
    message(FATAL_ERROR "NAME, MODULE_SOURCE are required arguments to build_libcrypto")
  endif()

  add_library(${arg_NAME} $<TARGET_OBJECTS:crypto_objects> ${CRYPTO_FIPS_OBJECTS} ${arg_MODULE_SOURCE})

  if(NOT DISABLE_CPU_JITTER_ENTROPY)
    target_sources(${arg_NAME} PRIVATE $<TARGET_OBJECTS:jitterentropy>)
  endif()

  if(FIPS_DELOCATE OR FIPS_SHARED)
    add_dependencies(${arg_NAME} bcm_o_target)
  endif()
  if(WIN32)
    target_link_libraries(${arg_NAME} PUBLIC ws2_32)
    # MinGW (non-Clang) builds target Windows 7 (see the top-level
    # CMakeLists.txt, which adds -D_WIN32_WINNT=_WIN32_WINNT_WIN7), so
    # `crypto/rand_extra/windows.c` compiles under the AWSLC_WINDOWS_7_COMPAT
    # branch and calls BCryptGenRandom from bcrypt.dll. On MSVC this is
    # handled by `#pragma comment(lib, "bcrypt.lib")` in that file, but
    # MinGW ignores that pragma so we need to add the link explicitly.
    if(MINGW)
      target_link_libraries(${arg_NAME} PUBLIC bcrypt)
    endif()
  endif()

  if(AWSLC_LINK_THREADS)
    target_link_libraries(${arg_NAME} PUBLIC Threads::Threads)
  endif()
  target_add_awslc_include_paths(TARGET ${arg_NAME} SCOPE PUBLIC)

  if(arg_SET_OUTPUT_NAME)
    set_target_properties(${arg_NAME} PROPERTIES OUTPUT_NAME "${CRYPTO_LIB_NAME}")
  endif()

  if(SET_LIB_SONAME)
    set_target_properties(${arg_NAME} PROPERTIES
      VERSION "${SOFTWARE_VERSION}"
      SOVERSION "${ABI_VERSION}")
  endif()
endfunction()

if(FIPS_SHARED)
  # Rewrite libcrypto.so, libcrypto.dylib, or crypto.dll to inject the correct module
  # hash value. For now we support the FIPS build only on Linux, macOS, iOS, and Windows.
  if(MSVC)
    # On Windows, inject_hash.go parses the linker map file and the PE to locate
    # module boundaries, computes the integrity hash, and patches it directly
    # into the DLL. This matches the Linux/Apple approach and does not require
    # running a binary, enabling cross-compilation without Wine.
    set(CRYPTO_MAP_FILE "${CMAKE_CURRENT_BINARY_DIR}/fips_crypto.map")
    build_libcrypto(NAME crypto MODULE_SOURCE $<TARGET_OBJECTS:fipsmodule> SET_OUTPUT_NAME)
    target_link_options(crypto PRIVATE "/MAP:${CRYPTO_MAP_FILE}")

    add_custom_command(
      TARGET crypto POST_BUILD
      COMMAND ${GO_EXECUTABLE} run
      ${AWSLC_SOURCE_DIR}/util/fipstools/inject_hash/inject_hash.go
      -o $<TARGET_FILE:crypto> -in-object $<TARGET_FILE:crypto>
      -map ${CRYPTO_MAP_FILE} -windows
      WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
    )
  else()
    # On Apple and Linux platforms inject_hash.go can parse libcrypto and inject
    # the hash directly into the final library.
    build_libcrypto(NAME crypto MODULE_SOURCE $<TARGET_OBJECTS:fipsmodule> SET_OUTPUT_NAME)
    if (APPLE)
      set(INJECT_HASH_APPLE_FLAG "-apple")
    endif()

    add_custom_command(
      TARGET crypto POST_BUILD
      COMMAND ${GO_EXECUTABLE} run
      ${AWSLC_SOURCE_DIR}/util/fipstools/inject_hash/inject_hash.go
      -o $<TARGET_FILE:crypto> -in-object $<TARGET_FILE:crypto> ${INJECT_HASH_APPLE_FLAG}
      WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
    )

    # On macOS 11 and higher on Apple Silicon, codesigning is mandatory for
    # binaries to run. This applies to both executables and dylibs. An ad-hoc
    # signature is sufficient, and the linker will automatically apply one when
    # a binary is created (see https://github.com/Homebrew/brew/issues/9082).
    #
    # When we build libcrypto.dylib the linker automatically signs it. But then
    # we inject the FIPS integrity hash into libcrypto.dylib which changes the
    # binary so the signature applied by the linker is obviously not valid
    # anymore. So when an application, like crypto_test, tries to load
    # libcrypto.dylib it crashes because the signature is not valid. To work
    # around this we add an ad-hoc signature to `libcrypto.dylib` after the
    # FIPS integrity hash is injected.
    #
    # Note: we use CMAKE_SYSTEM_PROCESSOR directly instead of the ARCH variable
    # because if NO_ASM build flag is defined then ARCH is set to "generic".
    if (APPLE AND CMAKE_SYSTEM_PROCESSOR_LOWER MATCHES "arm64.*|aarch64")
      add_custom_command(
        TARGET crypto POST_BUILD
        COMMAND codesign -s - $<TARGET_FILE:crypto>
        WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
      )
    endif()

  endif()
else()
  build_libcrypto(NAME crypto MODULE_SOURCE $<TARGET_OBJECTS:fipsmodule> SET_OUTPUT_NAME)
endif()

# Every target depends on crypto, so we add libcxx as a dependency here to
# simplify injecting it everywhere.
if(USE_CUSTOM_LIBCXX)
  install(TARGETS libcxx libcxxabi
          EXPORT crypto-targets
          ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
          LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  )
  target_link_libraries(crypto PUBLIC libcxx)
endif()

if(BUILD_TESTING)
  add_subdirectory(test)

  macro(add_test_executable executable_name test_file)
    message(STATUS "Generating test executable ${executable_name}.")
    add_executable(${executable_name} ${test_file})
    target_compile_definitions(${executable_name} PRIVATE BORINGSSL_IMPLEMENTATION)
    target_link_libraries(${executable_name} boringssl_gtest_main)
    add_dependencies(all_tests ${executable_name})
    set_test_location(${executable_name})
  endmacro()

  # Below tests are added as new executables to be executed in single process.
  # mem_test.cc tests memory overrides via defining strong symbols.
  add_test_executable(${MEM_TEST_EXEC} mem_test.cc)
  target_add_awslc_include_paths(TARGET ${MEM_TEST_EXEC} SCOPE PRIVATE)
  # mem_set_test.cc tests memory overrides via |CRYPTO_set_mem_functions|.
  add_test_executable(${MEM_SET_TEST_EXEC} mem_set_test.cc)
  target_add_awslc_include_paths(TARGET ${MEM_SET_TEST_EXEC} SCOPE PRIVATE)

  # Test randomness generation
  add_test_executable(${RANDOM_ISOLATED_EXEC} fipsmodule/rand/rand_isolated_test.cc)
  add_test_executable(${TREE_DRBG_JITTER_ENTROPY_ISOLATED_EXEC} fipsmodule/rand/entropy/tree_drbg_jitter_entropy_isolated_test.cc)

  # urandom_test is a separate binary because it needs to be able to observe the
  # PRNG initialisation, which means that it can't have other tests running before
  # it does.
  add_executable(
    ${RANDOM_TEST_EXEC}
    rand_extra/urandom_test.cc
  )
  set_test_location(${RANDOM_TEST_EXEC})

  target_link_libraries(${RANDOM_TEST_EXEC} test_support_lib boringssl_gtest crypto)
  target_add_awslc_include_paths(TARGET ${RANDOM_TEST_EXEC} SCOPE PRIVATE)

  add_dependencies(all_tests ${RANDOM_TEST_EXEC})

  add_executable(
    ${CRYPTO_TEST_EXEC}

    abi_self_test.cc
    asn1/asn1_test.cc
    base64/base64_test.cc
    bio/bio_md_test.cc
    bio/bio_test.cc
    bio/bio_socket_test.cc
    blake2/blake2_test.cc
    buf/buf_test.cc
    bytestring/bytestring_test.cc
    chacha/chacha_test.cc
    cipher_extra/aead_test.cc
    cipher_extra/cipher_test.cc
    compiler_test.cc
    conf/conf_test.cc
    console/console_test.cc
    constant_time_test.cc
    crypto_test.cc
    ecdh_extra/ecdh_test.cc
    decrepit/blowfish/blowfish_test.cc
    decrepit/cast/cast_test.cc
    decrepit/cfb/cfb_test.cc
    decrepit/evp/evp_test.cc
    decrepit/ripemd/ripemd_test.cc
    dh_extra/dh_test.cc
    digest_extra/digest_test.cc
    dsa/dsa_test.cc
    des/des_test.cc
    endian_test.cc
    err/err_test.cc
    evp_extra/evp_extra_test.cc
    evp_extra/evp_test.cc
    evp_extra/p_pqdsa_test.cc
    evp_extra/p_kem_test.cc
    evp_extra/scrypt_test.cc
    evp_extra/mldsa_test.cc
    fips_callback_test.cc
    fipsmodule/aes/aes_test.cc
    fipsmodule/bn/bn_test.cc
    fipsmodule/bn/bn_assert_test.cc
    fipsmodule/cmac/cmac_test.cc
    fipsmodule/curve25519/ed25519_test.cc
    fipsmodule/curve25519/x25519_test.cc
    fipsmodule/ec/ec_test.cc
    fipsmodule/ec/p256-nistz_test.cc
    fipsmodule/ecdsa/ecdsa_test.cc
    fipsmodule/evp/evp_ctx_test.cc
    fipsmodule/kdf/kdf_test.cc
    fipsmodule/md5/md5_test.cc
    fipsmodule/ml_kem/ml_kem_test.cc
    fipsmodule/modes/gcm_test.cc
    fipsmodule/modes/xts_test.cc
    fipsmodule/pbkdf/pbkdf_test.cc
    fipsmodule/rand/ctrdrbg_test.cc
    fipsmodule/rand/cpu_jitter_test.cc
    fipsmodule/rand/rand_test.cc
    fipsmodule/rand/entropy/entropy_source_test.cc
    fipsmodule/service_indicator/service_indicator_test.cc
    fipsmodule/sha/sha_test.cc
    fipsmodule/sha/sha3_test.cc
    fipsmodule/cpucap/cpu_arm_linux_test.cc
    fipsmodule/cpucap/cpu_aarch64_dit_test.cc
    fipsmodule/hkdf/hkdf_test.cc
    fipsmodule/sshkdf/sshkdf_test.cc
    hpke/hpke_test.cc
    hmac_extra/hmac_test.cc
    hrss/hrss_test.cc
    impl_dispatch_test.cc
    lhash/lhash_test.cc
    obj/obj_test.cc
    ocsp/ocsp_test.cc
    pem/pem_test.cc
    pkcs7/bio/bio_cipher_test.cc
    pkcs7/pkcs7_test.cc
    pkcs8/pkcs8_test.cc
    pkcs8/pkcs12_test.cc
    poly1305/poly1305_test.cc
    pool/pool_test.cc
    rand_extra/ccrandomgeneratebytes_test.cc
    rand_extra/getentropy_test.cc
    rand_extra/vm_ube_fallback_test.cc
    refcount_test.cc
    rsa_extra/rsa_test.cc
    rsa_extra/rsassa_pss_asn1_test.cc
    self_test.cc
    stack/stack_test.cc
    siphash/siphash_test.cc
    spake25519/spake25519_test.cc
    test/file_util.cc
    test/file_test_gtest.cc
    test/x509_util.cc
    thread_test.cc
    trust_token/trust_token_test.cc
    ube/ube_test.cc
    ube/fork_ube_detect_test.cc
    ube/vm_ube_detect_test.cc
    x509/tab_test.cc
    x509/x509_test.cc
    x509/x509_compat_test.cc
    x509/x509_time_test.cc

    $<TARGET_OBJECTS:crypto_test_data>
  )
  set_test_location(${CRYPTO_TEST_EXEC})

  target_link_libraries(${CRYPTO_TEST_EXEC} boringssl_gtest_main)
  target_add_awslc_include_paths(TARGET ${CRYPTO_TEST_EXEC} SCOPE PRIVATE)
  target_compile_definitions(${CRYPTO_TEST_EXEC} PRIVATE AWS_LC_TEST_ENV)
  # Ensure test has access to the SHA3 feature detection macro
  if(MY_ASSEMBLER_SUPPORTS_NEON_SHA3_EXTENSION)
    target_compile_definitions(${CRYPTO_TEST_EXEC} PRIVATE MY_ASSEMBLER_SUPPORTS_NEON_SHA3_EXTENSION)
  endif()
  if(MSVC)
    target_link_libraries(${CRYPTO_TEST_EXEC} ws2_32)
  else()
    target_compile_options(${CRYPTO_TEST_EXEC} PUBLIC -Wno-deprecated-declarations)
  endif()
  add_dependencies(all_tests ${CRYPTO_TEST_EXEC})

  message(STATUS "Generating test executable ${DYNAMIC_LOADING_TEST_EXEC}.")

  add_executable(${DYNAMIC_LOADING_TEST_EXEC} dynamic_loading_test.c)
  add_dependencies(${DYNAMIC_LOADING_TEST_EXEC} crypto)
  set_test_location(${DYNAMIC_LOADING_TEST_EXEC})

  set(DYNAMIC_LINKER_LIB "")
  if(CMAKE_SYSTEM_NAME STREQUAL "Linux" OR APPLE OR ANDROID)
    set(DYNAMIC_LINKER_LIB "dl")
  endif()

  if (BUILD_SHARED_LIBS AND NOT WIN32)
    target_compile_options(${DYNAMIC_LOADING_TEST_EXEC} PUBLIC "-DLIBCRYPTO_PATH=\"$<TARGET_FILE:crypto>\"")
    # Do not link to boringssl_gtest_main or any library that might be linked to libcrypto.so.
    if(ANDROID)
      target_link_libraries(${DYNAMIC_LOADING_TEST_EXEC} PUBLIC boringssl_gtest ${DYNAMIC_LINKER_LIB})
    else()
      target_link_libraries(${DYNAMIC_LOADING_TEST_EXEC} PUBLIC boringssl_gtest pthread ${DYNAMIC_LINKER_LIB})
    endif()
  endif()

  add_dependencies(all_tests ${DYNAMIC_LOADING_TEST_EXEC})

  # rwlock_static_init requires threading support (std::thread)
  # Skip on Generic (WASI) which doesn't have pthreads
  if(NOT CMAKE_SYSTEM_NAME STREQUAL "Generic")
    message(STATUS "Generating test executable ${RWLOCK_STATIC_INIT_TEST_EXEC}.")
    add_executable(${RWLOCK_STATIC_INIT_TEST_EXEC} rwlock_static_init.cc)
    add_dependencies(${RWLOCK_STATIC_INIT_TEST_EXEC} crypto)
    set_test_location(${RWLOCK_STATIC_INIT_TEST_EXEC})

    target_link_libraries(${RWLOCK_STATIC_INIT_TEST_EXEC} crypto)
    target_add_awslc_include_paths(TARGET ${RWLOCK_STATIC_INIT_TEST_EXEC} SCOPE PRIVATE)
    if(MSVC)
      target_link_libraries(${RWLOCK_STATIC_INIT_TEST_EXEC} ws2_32)
    else()
      target_compile_options(${RWLOCK_STATIC_INIT_TEST_EXEC} PUBLIC -Wno-deprecated-declarations)
    endif()
    add_dependencies(all_tests ${RWLOCK_STATIC_INIT_TEST_EXEC})
  endif()

endif()

if(NOT CMAKE_VERSION VERSION_LESS "3.12")
  install(TARGETS crypto
          EXPORT crypto-targets
          ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
          LIBRARY
            DESTINATION ${CMAKE_INSTALL_LIBDIR}
            NAMELINK_COMPONENT Development
          RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
else()
  install(TARGETS crypto
          EXPORT crypto-targets
          ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
          LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
          RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
endif()

if(MSVC AND CMAKE_BUILD_TYPE_LOWER MATCHES "relwithdebinfo" AND FIPS)
  install (FILES $<TARGET_FILE_DIR:crypto>/crypto.pdb DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif()

configure_file("cmake/crypto-config.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/crypto-config.cmake"
    @ONLY)

install(FILES "${CMAKE_CURRENT_BINARY_DIR}/crypto-config.cmake"
    DESTINATION "${CMAKE_INSTALL_LIBDIR}/crypto/cmake"
    COMPONENT Development)

if (BUILD_SHARED_LIBS)
   set (TARGET_DIR "shared")
else()
   set (TARGET_DIR "static")
endif()

install(EXPORT crypto-targets
    DESTINATION "${CMAKE_INSTALL_LIBDIR}/crypto/cmake/${TARGET_DIR}"
    NAMESPACE AWS::
    COMPONENT Development)
