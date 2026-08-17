/*
 * libdeflate.h - public header for libdeflate
 */

#ifndef LIBDEFLATE_H
#define LIBDEFLATE_H

#ifdef __cplusplus
extern "C" {
#endif

#define LIBDEFLATE_VERSION_MAJOR	1
#define LIBDEFLATE_VERSION_MINOR	10
#define LIBDEFLATE_VERSION_STRING	"1.10"

#include <stddef.h>
#include <stdint.h>

/*
 * On Windows, if you want to link to the DLL version of libdeflate, then
 * #define LIBDEFLATE_DLL.  Note that the calling convention is "stdcall".
 */
#ifdef LIBDEFLATE_DLL
#  ifdef BUILDING_LIBDEFLATE
#    define LIBDEFLATEEXPORT	LIBEXPORT
#  elif defined(_WIN32) || defined(__CYGWIN__)
#    define LIBDEFLATEEXPORT	__declspec(dllimport)
#  endif
#endif
#ifndef LIBDEFLATEEXPORT
#  define LIBDEFLATEEXPORT
#endif

#if defined(_WIN32) && !defined(_WIN64)
#  define LIBDEFLATEAPI_ABI	__stdcall
#else
#  define LIBDEFLATEAPI_ABI
#endif

#if defined(BUILDING_LIBDEFLATE) && defined(__GNUC__) && \
	defined(_WIN32) && !defined(_WIN64)
    /*
     * On 32-bit Windows, gcc assumes 16-byte stack alignment but MSVC only 4.
     * Realign the stack when entering libdeflate to avoid crashing in SSE/AVX
     * code when called from an MSVC-compiled application.
     */
#  define LIBDEFLATEAPI_STACKALIGN	__attribute__((force_align_arg_pointer))
#else
#  define LIBDEFLATEAPI_STACKALIGN
#endif

#define LIBDEFLATEAPI	LIBDEFLATEAPI_ABI LIBDEFLATEAPI_STACKALIGN

/* ========================================================================== */
/*                             Compression                                    */
/* ========================================================================== */

struct libdeflate_compressor;

/*
 * libdeflate_alloc_compressor() allocates a new compressor that supports
 * DEFLATE, zlib, and gzip compression.  'compression_level' is the compression
 * level on a zlib-like scale but with a higher maximum value (1 = fastest, 6 =
 * medium/default, 9 = slow, 12 = slowest).  Level 0 is also supported and means
 * "no compression", specifically "create a valid stream, but only emit
 * uncompressed blocks" (this will expand the data slightly).
 *
 * The return value is a pointer to the new compressor, or NULL if out of memory
 * or if the compression level is invalid (i.e. outside the range [0, 12]).
 *
 * Note: for compression, the sliding window size is defined at compilation time
 * to 32768, the largest size permissible in the DEFLATE format.  It cannot be
 * changed at runtime.
 *
 * A single compressor is not safe to use by multiple threads concurrently.
 * However, different threads may use different compressors concurrently.
 */
LIBDEFLATEEXPORT struct libdeflate_compressor * LIBDEFLATEAPI
libdeflate_alloc_compressor(int compression_level);

/*
 * libdeflate_deflate_compress() performs raw DEFLATE compression on a buffer of
 * data.  The function attempts to compress 'in_nbytes' bytes of data located at
 * 'in' and write the results to 'out', which has space for 'out_nbytes_avail'
 * bytes.  The return value is the compressed size in bytes, or 0 if the data
 * could not be compressed to 'out_nbytes_avail' bytes or fewer.
 */
LIBDEFLATEEXPORT size_t LIBDEFLATEAPI
libdeflate_deflate_compress(struct libdeflate_compressor *compressor,
			    const void *in, size_t in_nbytes,
			    void *out, size_t out_nbytes_avail);

/*
 * libdeflate_deflate_compress_bound() returns a worst-case upper bound on the
 * number of bytes of compressed data that may be produced by compressing any
 * buffer of length less than or equal to 'in_nbytes' using
 * libdeflate_deflate_compress() with the specified compressor.  Mathematically,
 * this bound will necessarily be a number greater than or equal to 'in_nbytes'.
 * It may be an overestimate of the true upper bound.  The return value is
 * guaranteed to be the same for all invocations with the same compressor and
 * same 'in_nbytes'.
 *
 * As a special case, 'compressor' may be NULL.  This causes the bound to be
 * taken across *any* libdeflate_compressor that could ever be allocated with
 * this build of the library, with any options.
 *
 * Note that this function is not necessary in many applications.  With
 * block-based compression, it is usually preferable to separately store the
 * uncompressed size of each block and to store any blocks that did not compress
 * to less than their original size uncompressed.  In that scenario, there is no
 * need to know the worst-case compressed size, since the maximum number of
 * bytes of compressed data that may be used would always be one less than the
 * input length.  You can just pass a buffer of that size to
 * libdeflate_deflate_compress() and store the data uncompressed if
 * libdeflate_deflate_compress() returns 0, indicating that the compressed data
 * did not fit into the provided output buffer.
 */
LIBDEFLATEEXPORT size_t LIBDEFLATEAPI
libdeflate_deflate_compress_bound(struct libdeflate_compressor *compressor,
				  size_t in_nbytes);

/*
 * Like libdeflate_deflate_compress(), but stores the data in the zlib wrapper
 * format.
 */
LIBDEFLATEEXPORT size_t LIBDEFLATEAPI
libdeflate_zlib_compress(struct libdeflate_compressor *compressor,
			 const void *in, size_t in_nbytes,
			 void *out, size_t out_nbytes_avail);

/*
 * Like libdeflate_deflate_compress_bound(), but assumes the data will be
 * compressed with libdeflate_zlib_compress() rather than with
 * libdeflate_deflate_compress().
 */
LIBDEFLATEEXPORT size_t LIBDEFLATEAPI
libdeflate_zlib_compress_bound(struct libdeflate_compressor *compressor,
			       size_t in_nbytes);

/*
 * Like libdeflate_deflate_compress(), but stores the data in the gzip wrapper
 * format.
 */
LIBDEFLATEEXPORT size_t LIBDEFLATEAPI
libdeflate_gzip_compress(struct libdeflate_compressor *compressor,
			 const void *in, size_t in_nbytes,
			 void *out, size_t out_nbytes_avail);

/*
 * Like libdeflate_deflate_compress_bound(), but assumes the data will be
 * compressed with libdeflate_gzip_compress() rather than with
 * libdeflate_deflate_compress().
 */
LIBDEFLATEEXPORT size_t LIBDEFLATEAPI
libdeflate_gzip_compress_bound(struct libdeflate_compressor *compressor,
			       size_t in_nbytes);

/*
 * libdeflate_free_compressor() frees a compressor that was allocated with
 * libdeflate_alloc_compressor().  If a NULL pointer is passed in, no action is
 * taken.
 */
LIBDEFLATEEXPORT void LIBDEFLATEAPI
libdeflate_free_compressor(struct libdeflate_compressor *compressor);

/* ========================================================================== */
/*                             Decompression                                  */
/* ========================================================================== */

struct libdeflate_decompressor;

/*
 * libdeflate_alloc_decompressor() allocates a new decompressor that can be used
 * for DEFLATE, zlib, and gzip decompression.  The return value is a pointer to
 * the new decompressor, or NULL if out of memory.
 *
 * This function takes no parameters, and the returned decompressor is valid for
 * decompressing data that was compressed at any compression level and with any
 * sliding window size.
 *
 * A single decompressor is not safe to use by multiple threads concurrently.
 * However, different threads may use different decompressors concurrently.
 */
LIBDEFLATEEXPORT struct libdeflate_decompressor * LIBDEFLATEAPI
libdeflate_alloc_decompressor(void);

/*
 * Result of a call to libdeflate_deflate_decompress(),
 * libdeflate_zlib_decompress(), or libdeflate_gzip_decompress().
 */
enum libdeflate_result {
	/* Decompression was successful.  */
	LIBDEFLATE_SUCCESS = 0,

	/* Decompressed failed because the compressed data was invalid, corrupt,
	 * or otherwise unsupported.  */
	LIBDEFLATE_BAD_DATA = 1,

	/* A NULL 'actual_out_nbytes_ret' was provided, but the data would have
	 * decompressed to fewer than 'out_nbytes_avail' bytes.  */
	LIBDEFLATE_SHORT_OUTPUT = 2,

	/* The data would have decompressed to more than 'out_nbytes_avail'
	 * bytes.  */
	LIBDEFLATE_INSUFFICIENT_SPACE = 3,
};

/*
 * libdeflate_deflate_decompress() decompresses the DEFLATE-compressed stream
 * from the buffer 'in' with compressed size up to 'in_nbytes' bytes.  The
 * uncompressed data is written to 'out', a buffer with size 'out_nbytes_avail'
 * bytes.  If decompression succeeds, then 0 (LIBDEFLATE_SUCCESS) is returned.
 * Otherwise, a nonzero result code such as LIBDEFLATE_BAD_DATA is returned.  If
 * a nonzero result code is returned, then the contents of the output buffer are
 * undefined.
 *
 * Decompression stops at the end of the DEFLATE stream (as indicated by the
 * BFINAL flag), even if it is actually shorter than 'in_nbytes' bytes.
 *
 * libdeflate_deflate_decompress() can be used in cases where the actual
 * uncompressed size is known (recommended) or unknown (not recommended):
 *
 *   - If the actual uncompressed size is known, then pass the actual
 *     uncompressed size as 'out_nbytes_avail' and pass NULL for
 *     'actual_out_nbytes_ret'.  This makes libdeflate_deflate_decompress() fail
 *     with LIBDEFLATE_SHORT_OUTPUT if the data decompressed to fewer than the
 *     specified number of bytes.
 *
 *   - If the actual uncompressed size is unknown, then provide a non-NULL
 *     'actual_out_nbytes_ret' and provide a buffer with some size
 *     'out_nbytes_avail' that you think is large enough to hold all the
 *     uncompressed data.  In this case, if the data decompresses to less than
 *     or equal to 'out_nbytes_avail' bytes, then
 *     libdeflate_deflate_decompress() will write the actual uncompressed size
 *     to *actual_out_nbytes_ret and return 0 (LIBDEFLATE_SUCCESS).  Otherwise,
 *     it will return LIBDEFLATE_INSUFFICIENT_SPACE if the provided buffer was
 *     not large enough but no other problems were encountered, or another
 *     nonzero result code if decompression failed for another reason.
 */
LIBDEFLATEEXPORT enum libdeflate_result LIBDEFLATEAPI
libdeflate_deflate_decompress(struct libdeflate_decompressor *decompressor,
			      const void *in, size_t in_nbytes,
			      void *out, size_t out_nbytes_avail,
			      size_t *actual_out_nbytes_ret);

/*
 * Like libdeflate_deflate_decompress(), but adds the 'actual_in_nbytes_ret'
 * argument.  If decompression succeeds and 'actual_in_nbytes_ret' is not NULL,
 * then the actual compressed size of the DEFLATE stream (aligned to the next
 * byte boundary) is written to *actual_in_nbytes_ret.
 */
LIBDEFLATEEXPORT enum libdeflate_result LIBDEFLATEAPI
libdeflate_deflate_decompress_ex(struct libdeflate_decompressor *decompressor,
				 const void *in, size_t in_nbytes,
				 void *out, size_t out_nbytes_avail,
				 size_t *actual_in_nbytes_ret,
				 size_t *actual_out_nbytes_ret);

/*
 * Like libdeflate_deflate_decompress(), but assumes the zlib wrapper format
 * instead of raw DEFLATE.
 *
 * Decompression will stop at the end of the zlib stream, even if it is shorter
 * than 'in_nbytes'.  If you need to know exactly where the zlib stream ended,
 * use libdeflate_zlib_decompress_ex().
 */
LIBDEFLATEEXPORT enum libdeflate_result LIBDEFLATEAPI
libdeflate_zlib_decompress(struct libdeflate_decompressor *decompressor,
			   const void *in, size_t in_nbytes,
			   void *out, size_t out_nbytes_avail,
			   size_t *actual_out_nbytes_ret);

/*
 * Like libdeflate_zlib_decompress(), but adds the 'actual_in_nbytes_ret'
 * argument.  If 'actual_in_nbytes_ret' is not NULL and the decompression
 * succeeds (indicating that the first zlib-compressed stream in the input
 * buffer was decompressed), then the actual number of input bytes consumed is
 * written to *actual_in_nbytes_ret.
 */
LIBDEFLATEEXPORT enum libdeflate_result LIBDEFLATEAPI
libdeflate_zlib_decompress_ex(struct libdeflate_decompressor *decompressor,
			      const void *in, size_t in_nbytes,
			      void *out, size_t out_nbytes_avail,
			      size_t *actual_in_nbytes_ret,
			      size_t *actual_out_nbytes_ret);

/*
 * Like libdeflate_deflate_decompress(), but assumes the gzip wrapper format
 * instead of raw DEFLATE.
 *
 * If multiple gzip-compressed members are concatenated, then only the first
 * will be decompressed.  Use libdeflate_gzip_decompress_ex() if you need
 * multi-member support.
 */
LIBDEFLATEEXPORT enum libdeflate_result LIBDEFLATEAPI
libdeflate_gzip_decompress(struct libdeflate_decompressor *decompressor,
			   const void *in, size_t in_nbytes,
			   void *out, size_t out_nbytes_avail,
			   size_t *actual_out_nbytes_ret);

/*
 * Like libdeflate_gzip_decompress(), but adds the 'actual_in_nbytes_ret'
 * argument.  If 'actual_in_nbytes_ret' is not NULL and the decompression
 * succeeds (indicating that the first gzip-compressed member in the input
 * buffer was decompressed), then the actual number of input bytes consumed is
 * written to *actual_in_nbytes_ret.
 */
LIBDEFLATEEXPORT enum libdeflate_result LIBDEFLATEAPI
libdeflate_gzip_decompress_ex(struct libdeflate_decompressor *decompressor,
			      const void *in, size_t in_nbytes,
			      void *out, size_t out_nbytes_avail,
			      size_t *actual_in_nbytes_ret,
			      size_t *actual_out_nbytes_ret);

/*
 * libdeflate_free_decompressor() frees a decompressor that was allocated with
 * libdeflate_alloc_decompressor().  If a NULL pointer is passed in, no action
 * is taken.
 */
LIBDEFLATEEXPORT void LIBDEFLATEAPI
libdeflate_free_decompressor(struct libdeflate_decompressor *decompressor);

/* ========================================================================== */
/*                                Checksums                                   */
/* ========================================================================== */

/*
 * libdeflate_adler32() updates a running Adler-32 checksum with 'len' bytes of
 * data and returns the updated checksum.  When starting a new checksum, the
 * required initial value for 'adler' is 1.  This value is also returned when
 * 'buffer' is specified as NULL.
 */
LIBDEFLATEEXPORT uint32_t LIBDEFLATEAPI
libdeflate_adler32(uint32_t adler, const void *buffer, size_t len);


/*
 * libdeflate_crc32() updates a running CRC-32 checksum with 'len' bytes of data
 * and returns the updated checksum.  When starting a new checksum, the required
 * initial value for 'crc' is 0.  This value is also returned when 'buffer' is
 * specified as NULL.
 */
LIBDEFLATEEXPORT uint32_t LIBDEFLATEAPI
libdeflate_crc32(uint32_t crc, const void *buffer, size_t len);

/* ========================================================================== */
/*                           Custom memory allocator                          */
/* ========================================================================== */

/*
 * Install a custom memory allocator which libdeflate will use for all memory
 * allocations.  'malloc_func' is a function that must behave like malloc(), and
 * 'free_func' is a function that must behave like free().
 *
 * There must not be any libdeflate_compressor or libdeflate_decompressor
 * structures in existence when calling this function.
 */
LIBDEFLATEEXPORT void LIBDEFLATEAPI
libdeflate_set_memory_allocator(void *(*malloc_func)(size_t),
				void (*free_func)(void *));

#ifdef __cplusplus
}
#endif

#endif /* LIBDEFLATE_H */
