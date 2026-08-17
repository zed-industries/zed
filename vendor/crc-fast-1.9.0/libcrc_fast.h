/* crc_fast library C/C++ API - Copyright 2025 Don MacAskill */
/* This header is auto-generated. Do not edit directly. */

#ifndef CRC_FAST_H
#define CRC_FAST_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Error codes for FFI operations
 */
typedef enum CrcFastError {
  /**
   * Operation completed successfully
   */
  Success = 0,
  /**
   * Lock was poisoned (thread panicked while holding lock)
   */
  LockPoisoned = 1,
  /**
   * Null pointer was passed where non-null required
   */
  NullPointer = 2,
  /**
   * Invalid key count for CRC parameters
   */
  InvalidKeyCount = 3,
  /**
   * Unsupported CRC width (must be 32 or 64)
   */
  UnsupportedWidth = 4,
  /**
   * Invalid UTF-8 string
   */
  InvalidUtf8 = 5,
  /**
   * File I/O error
   */
  IoError = 6,
  /**
   * Internal string conversion error
   */
  StringConversionError = 7,
} CrcFastError;

/**
 * The supported CRC algorithms
 */
typedef enum CrcFastAlgorithm {
  CrcCustom,
  Crc16Arc,
  Crc16Cdma2000,
  Crc16Cms,
  Crc16Dds110,
  Crc16DectR,
  Crc16DectX,
  Crc16Dnp,
  Crc16En13757,
  Crc16Genibus,
  Crc16Gsm,
  Crc16Ibm3740,
  Crc16IbmSdlc,
  Crc16IsoIec144433A,
  Crc16Kermit,
  Crc16Lj1200,
  Crc16M17,
  Crc16MaximDow,
  Crc16Mcrf4xx,
  Crc16Modbus,
  Crc16Nrsc5,
  Crc16OpensafetyA,
  Crc16OpensafetyB,
  Crc16Profibus,
  Crc16Riello,
  Crc16SpiFujitsu,
  Crc16T10Dif,
  Crc16Teledisk,
  Crc16Tms37157,
  Crc16Umts,
  Crc16Usb,
  Crc16Xmodem,
  Crc32Aixm,
  Crc32Autosar,
  Crc32Base91D,
  Crc32Bzip2,
  Crc32CdRomEdc,
  Crc32Cksum,
  Crc32Custom,
  Crc32Iscsi,
  Crc32IsoHdlc,
  Crc32Jamcrc,
  Crc32Mef,
  Crc32Mpeg2,
  Crc32Xfer,
  Crc64Custom,
  Crc64Ecma182,
  Crc64GoIso,
  Crc64Ms,
  Crc64Nvme,
  Crc64Redis,
  Crc64We,
  Crc64Xz,
} CrcFastAlgorithm;

/**
 * Represents a CRC Digest, which is used to compute CRC checksums.
 *
 * The `Digest` struct maintains the state of the CRC computation, including
 * the current state, the amount of data processed, the CRC parameters, and
 * the calculator function used to perform the CRC calculation.
 */
typedef struct CrcFastDigest CrcFastDigest;

/**
 * A handle to the Digest object
 */
typedef struct CrcFastDigestHandle {
  struct CrcFastDigest *_0;
} CrcFastDigestHandle;

/**
 * Custom CRC parameters
 */
typedef struct CrcFastParams {
  enum CrcFastAlgorithm algorithm;
  uint8_t width;
  uint64_t poly;
  uint64_t init;
  bool refin;
  bool refout;
  uint64_t xorout;
  uint64_t check;
  uint32_t key_count;
  const uint64_t *keys;
} CrcFastParams;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Gets the last error that occurred in the current thread
 * Returns CrcFastError::Success if no error has occurred
 */
enum CrcFastError crc_fast_get_last_error(void);

/**
 * Clears the last error for the current thread
 */
void crc_fast_clear_error(void);

/**
 * Gets a human-readable error message for the given error code
 * Returns a pointer to a static string (do not free)
 */
const char *crc_fast_error_message(enum CrcFastError error);

/**
 * Creates a new Digest to compute CRC checksums using algorithm
 */
struct CrcFastDigestHandle *crc_fast_digest_new(enum CrcFastAlgorithm algorithm);

/**
 * Creates a new Digest with a custom initial state
 */
struct CrcFastDigestHandle *crc_fast_digest_new_with_init_state(enum CrcFastAlgorithm algorithm,
                                                                uint64_t init_state);

/**
 * Creates a new Digest to compute CRC checksums using custom parameters
 * Returns NULL if parameters are invalid (invalid key count or null pointer)
 * Call crc_fast_get_last_error() to get the specific error code
 */
struct CrcFastDigestHandle *crc_fast_digest_new_with_params(struct CrcFastParams params);

/**
 * Updates the Digest with data
 */
void crc_fast_digest_update(struct CrcFastDigestHandle *handle, const char *data, uintptr_t len);

/**
 * Calculates the CRC checksum for data that's been written to the Digest
 * Returns 0 on error (e.g. null handle)
 */
uint64_t crc_fast_digest_finalize(struct CrcFastDigestHandle *handle);

/**
 * Free the Digest resources without finalizing
 */
void crc_fast_digest_free(struct CrcFastDigestHandle *handle);

/**
 * Reset the Digest state
 */
void crc_fast_digest_reset(struct CrcFastDigestHandle *handle);

/**
 * Finalize and reset the Digest in one operation
 * Returns 0 on error (e.g. null handle)
 */
uint64_t crc_fast_digest_finalize_reset(struct CrcFastDigestHandle *handle);

/**
 * Combine two Digest checksums
 */
void crc_fast_digest_combine(struct CrcFastDigestHandle *handle1,
                             struct CrcFastDigestHandle *handle2);

/**
 * Gets the amount of data processed by the Digest so far
 * Returns 0 on error (e.g. null handle)
 */
uint64_t crc_fast_digest_get_amount(struct CrcFastDigestHandle *handle);

/**
 * Gets the current state of the Digest
 * Returns 0 on error (e.g. null handle)
 */
uint64_t crc_fast_digest_get_state(struct CrcFastDigestHandle *handle);

/**
 * Helper method to calculate a CRC checksum directly for a string using algorithm
 * Returns 0 on error (e.g. null data pointer)
 */
uint64_t crc_fast_checksum(enum CrcFastAlgorithm algorithm, const char *data, uintptr_t len);

/**
 * Helper method to calculate a CRC checksum directly for data using custom parameters
 * Returns 0 if parameters are invalid or data is null
 * Call crc_fast_get_last_error() to get the specific error code
 */
uint64_t crc_fast_checksum_with_params(struct CrcFastParams params,
                                       const char *data,
                                       uintptr_t len);

/**
 * Helper method to just calculate a CRC checksum directly for a file using algorithm
 * Returns 0 if path is null or file I/O fails
 * Call crc_fast_get_last_error() to get the specific error code
 */
uint64_t crc_fast_checksum_file(enum CrcFastAlgorithm algorithm,
                                const uint8_t *path_ptr,
                                uintptr_t path_len);

/**
 * Helper method to calculate a CRC checksum directly for a file using custom parameters
 * Returns 0 if parameters are invalid, path is null, or file I/O fails
 * Call crc_fast_get_last_error() to get the specific error code
 */
uint64_t crc_fast_checksum_file_with_params(struct CrcFastParams params,
                                            const uint8_t *path_ptr,
                                            uintptr_t path_len);

/**
 * Combine two CRC checksums using algorithm
 */
uint64_t crc_fast_checksum_combine(enum CrcFastAlgorithm algorithm,
                                   uint64_t checksum1,
                                   uint64_t checksum2,
                                   uint64_t checksum2_len);

/**
 * Combine two CRC checksums using custom parameters
 * Returns 0 if parameters are invalid
 * Call crc_fast_get_last_error() to get the specific error code
 */
uint64_t crc_fast_checksum_combine_with_params(struct CrcFastParams params,
                                               uint64_t checksum1,
                                               uint64_t checksum2,
                                               uint64_t checksum2_len);

/**
 * Returns the custom CRC parameters for a given set of Rocksoft CRC parameters
 * If width is not 32 or 64, sets error to UnsupportedWidth
 */
struct CrcFastParams crc_fast_get_custom_params(const char *name_ptr,
                                                uint8_t width,
                                                uint64_t poly,
                                                uint64_t init,
                                                bool reflected,
                                                uint64_t xorout,
                                                uint64_t check);

/**
 * Gets the target build properties (CPU architecture and fine-tuning parameters) for this algorithm
 * Returns NULL if string conversion fails
 * Call crc_fast_get_last_error() to get the specific error code
 */
const char *crc_fast_get_calculator_target(enum CrcFastAlgorithm algorithm);

/**
 * Gets the version of this library
 * Returns a pointer to "unknown" if version string is invalid
 */
const char *crc_fast_get_version(void);

/**
 * Calculates the CRC-32/ISCSI checksum (commonly called "crc32c" in many, but not all,
 * implementations).
 *
 * https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iscsi
 *
 * Returns 0 on error (e.g. null data pointer)
 */
uint32_t crc_fast_crc32_iscsi(const char *data, uintptr_t len);

/**
 * Calculates the CRC-32/ISO-HDLC checksum (commonly called "crc32" in many, but not all,
 * implementations).
 *
 * https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc
 *
 * Returns 0 on error (e.g. null data pointer)
 */
uint32_t crc_fast_crc32_iso_hdlc(const char *data, uintptr_t len);

/**
 * Calculates the CRC-64/NVME checksum.
 *
 * https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme
 *
 * Returns 0 on error (e.g. null data pointer)
 */
uint64_t crc_fast_crc64_nvme(const char *data, uintptr_t len);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* CRC_FAST_H */
