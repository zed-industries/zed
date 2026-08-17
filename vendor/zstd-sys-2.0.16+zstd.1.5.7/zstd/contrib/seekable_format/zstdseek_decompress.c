/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

/* *********************************************************
*  Turn on Large Files support (>4GB) for 32-bit Linux/Unix
***********************************************************/
#if !defined(__64BIT__) || defined(__MINGW32__)       /* No point defining Large file for 64 bit but MinGW-w64 requires it */
#  if !defined(_FILE_OFFSET_BITS)
#    define _FILE_OFFSET_BITS 64                      /* turn off_t into a 64-bit type for ftello, fseeko */
#  endif
#  if !defined(_LARGEFILE_SOURCE)                     /* obsolete macro, replaced with _FILE_OFFSET_BITS */
#    define _LARGEFILE_SOURCE 1                       /* Large File Support extension (LFS) - fseeko, ftello */
#  endif
#  if defined(_AIX) || defined(__hpux)
#    define _LARGE_FILES                              /* Large file support on 32-bits AIX and HP-UX */
#  endif
#endif

/* ************************************************************
*  Detect POSIX version
*  PLATFORM_POSIX_VERSION = 0 for non-Unix e.g. Windows
*  PLATFORM_POSIX_VERSION = 1 for Unix-like but non-POSIX
*  PLATFORM_POSIX_VERSION > 1 is equal to found _POSIX_VERSION
*  Value of PLATFORM_POSIX_VERSION can be forced on command line
***************************************************************/
#ifndef PLATFORM_POSIX_VERSION

#  if (defined(__APPLE__) && defined(__MACH__)) || defined(__SVR4) || defined(_AIX) || defined(__hpux) /* POSIX.1-2001 (SUSv3) conformant */ \
     || defined(__DragonFly__) || defined(__FreeBSD__) || defined(__NetBSD__) || defined(__OpenBSD__)  /* BSD distros */
     /* exception rule : force posix version to 200112L,
      * note: it's better to use unistd.h's _POSIX_VERSION whenever possible */
#    define PLATFORM_POSIX_VERSION 200112L

/* try to determine posix version through official unistd.h's _POSIX_VERSION (https://pubs.opengroup.org/onlinepubs/7908799/xsh/unistd.h.html).
 * note : there is no simple way to know in advance if <unistd.h> is present or not on target system,
 * Posix specification mandates its presence and its content, but target system must respect this spec.
 * It's necessary to _not_ #include <unistd.h> whenever target OS is not unix-like
 * otherwise it will block preprocessing stage.
 * The following list of build macros tries to "guess" if target OS is likely unix-like, and therefore can #include <unistd.h>
 */
#  elif !defined(_WIN32) \
     && ( defined(__unix__) || defined(__unix) \
       || defined(__midipix__) || defined(__VMS) || defined(__HAIKU__) )

#    if defined(__linux__) || defined(__linux) || defined(__CYGWIN__)
#      ifndef _POSIX_C_SOURCE
#        define _POSIX_C_SOURCE 200809L  /* feature test macro : https://www.gnu.org/software/libc/manual/html_node/Feature-Test-Macros.html */
#      endif
#    endif
#    include <unistd.h>  /* declares _POSIX_VERSION */
#    if defined(_POSIX_VERSION)  /* POSIX compliant */
#      define PLATFORM_POSIX_VERSION _POSIX_VERSION
#    else
#      define PLATFORM_POSIX_VERSION 1
#    endif

#    ifdef __UCLIBC__
#     ifndef __USE_MISC
#      define __USE_MISC /* enable st_mtim on uclibc */
#     endif
#    endif

#  else  /* non-unix target platform (like Windows) */
#    define PLATFORM_POSIX_VERSION 0
#  endif

#endif   /* PLATFORM_POSIX_VERSION */


/* ************************************************************
* Avoid fseek()'s 2GiB barrier with MSVC, macOS, *BSD, MinGW
***************************************************************/
#if defined(LIBC_NO_FSEEKO)
/* Some older libc implementations don't include these functions (e.g. Bionic < 24) */
#   define LONG_SEEK fseek
#elif defined(_MSC_VER) && _MSC_VER >= 1400
#   define LONG_SEEK _fseeki64
#elif !defined(__64BIT__) && (PLATFORM_POSIX_VERSION >= 200112L) /* No point defining Large file for 64 bit */
#   define LONG_SEEK fseeko
#elif defined(__MINGW32__) && !defined(__STRICT_ANSI__) && !defined(__NO_MINGW_LFS) && defined(__MSVCRT__)
#   define LONG_SEEK fseeko64
#elif defined(_WIN32) && !defined(__DJGPP__)
#   include <windows.h>
    static int LONG_SEEK(FILE* file, __int64 offset, int origin) {
        LARGE_INTEGER off;
        DWORD method;
        off.QuadPart = offset;
        if (origin == SEEK_END)
            method = FILE_END;
        else if (origin == SEEK_CUR)
            method = FILE_CURRENT;
        else
            method = FILE_BEGIN;

        if (SetFilePointerEx((HANDLE) _get_osfhandle(_fileno(file)), off, NULL, method))
            return 0;
        else
            return -1;
    }
#else
#   define LONG_SEEK fseek
#endif

#include <stdlib.h>  /* malloc, free */
#include <stdio.h>   /* FILE* */
#include <limits.h>  /* UNIT_MAX */
#include <assert.h>

#define XXH_STATIC_LINKING_ONLY
#include "xxhash.h"

#define ZSTD_STATIC_LINKING_ONLY
#include "zstd.h"
#include "zstd_errors.h"
#include "mem.h"
#include "zstd_seekable.h"

#undef ERROR
#define ERROR(name) ((size_t)-ZSTD_error_##name)

#define CHECK_IO(f) { int const errcod = (f); if (errcod < 0) return ERROR(seekableIO); }

#undef MIN
#undef MAX
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define ZSTD_SEEKABLE_NO_OUTPUT_PROGRESS_MAX 16

/* Special-case callbacks for FILE* and in-memory modes, so that we can treat
 * them the same way as the advanced API */
static int ZSTD_seekable_read_FILE(void* opaque, void* buffer, size_t n)
{
    size_t const result = fread(buffer, 1, n, (FILE*)opaque);
    if (result != n) {
        return -1;
    }
    return 0;
}

static int ZSTD_seekable_seek_FILE(void* opaque, long long offset, int origin)
{
    int const ret = LONG_SEEK((FILE*)opaque, offset, origin);
    if (ret) return ret;
    return fflush((FILE*)opaque);
}

typedef struct {
    const void *ptr;
    size_t size;
    size_t pos;
} buffWrapper_t;

static int ZSTD_seekable_read_buff(void* opaque, void* buffer, size_t n)
{
    buffWrapper_t* const buff = (buffWrapper_t*)opaque;
    assert(buff != NULL);
    if (buff->pos + n > buff->size) return -1;
    memcpy(buffer, (const BYTE*)buff->ptr + buff->pos, n);
    buff->pos += n;
    return 0;
}

static int ZSTD_seekable_seek_buff(void* opaque, long long offset, int origin)
{
    buffWrapper_t* const buff = (buffWrapper_t*) opaque;
    unsigned long long newOffset;
    assert(buff != NULL);
    switch (origin) {
    case SEEK_SET:
        assert(offset >= 0);
        newOffset = (unsigned long long)offset;
        break;
    case SEEK_CUR:
        newOffset = (unsigned long long)((long long)buff->pos + offset);
        break;
    case SEEK_END:
        newOffset = (unsigned long long)((long long)buff->size + offset);
        break;
    default:
        assert(0);  /* not possible */
    }
    if (newOffset > buff->size) {
        return -1;
    }
    buff->pos = newOffset;
    return 0;
}

typedef struct {
    U64 cOffset;
    U64 dOffset;
    U32 checksum;
} seekEntry_t;

struct ZSTD_seekTable_s {
    seekEntry_t* entries;
    size_t tableLen;

    int checksumFlag;
};

#define SEEKABLE_BUFF_SIZE ZSTD_BLOCKSIZE_MAX

struct ZSTD_seekable_s {
    ZSTD_DStream* dstream;
    ZSTD_seekTable seekTable;
    ZSTD_seekable_customFile src;

    U64 decompressedOffset;
    U32 curFrame;

    BYTE inBuff[SEEKABLE_BUFF_SIZE]; /* need to do our own input buffering */
    BYTE outBuff[SEEKABLE_BUFF_SIZE]; /* so we can efficiently decompress the
                                         starts of chunks before we get to the
                                         desired section */
    ZSTD_inBuffer in; /* maintain continuity across ZSTD_seekable_decompress operations */
    buffWrapper_t buffWrapper; /* for `src.opaque` in in-memory mode */

    XXH64_state_t xxhState;
};

ZSTD_seekable* ZSTD_seekable_create(void)
{
    ZSTD_seekable* const zs = (ZSTD_seekable*)malloc(sizeof(ZSTD_seekable));
    if (zs == NULL) return NULL;

    /* also initializes stage to zsds_init */
    memset(zs, 0, sizeof(*zs));

    zs->dstream = ZSTD_createDStream();
    if (zs->dstream == NULL) {
        free(zs);
        return NULL;
    }

    return zs;
}

size_t ZSTD_seekable_free(ZSTD_seekable* zs)
{
    if (zs == NULL) return 0; /* support free on null */
    ZSTD_freeDStream(zs->dstream);
    free(zs->seekTable.entries);
    free(zs);
    return 0;
}

ZSTD_seekTable* ZSTD_seekTable_create_fromSeekable(const ZSTD_seekable* zs)
{
    assert(zs != NULL);
    if (zs->seekTable.entries == NULL) return NULL;
    ZSTD_seekTable* const st = (ZSTD_seekTable*)malloc(sizeof(ZSTD_seekTable));
    if (st==NULL) return NULL;

    st->checksumFlag = zs->seekTable.checksumFlag;
    st->tableLen = zs->seekTable.tableLen;

    /* Allocate an extra entry at the end to match logic of initial allocation */
    size_t const entriesSize = sizeof(seekEntry_t) * (zs->seekTable.tableLen + 1);
    seekEntry_t* const entries = (seekEntry_t*)malloc(entriesSize);
    if (entries==NULL) {
        free(st);
        return NULL;
    }

    memcpy(entries, zs->seekTable.entries, entriesSize);
    st->entries = entries;
    return st;
}

size_t ZSTD_seekTable_free(ZSTD_seekTable* st)
{
    if (st == NULL) return 0; /* support free on null */
    free(st->entries);
    free(st);
    return 0;
}

/** ZSTD_seekable_offsetToFrameIndex() :
 *  Performs a binary search to find the last frame with a decompressed offset
 *  <= pos
 *  @return : the frame's index */
unsigned ZSTD_seekable_offsetToFrameIndex(const ZSTD_seekable* zs, unsigned long long pos)
{
    return ZSTD_seekTable_offsetToFrameIndex(&zs->seekTable, pos);
}

unsigned ZSTD_seekTable_offsetToFrameIndex(const ZSTD_seekTable* st, unsigned long long pos)
{
    U32 lo = 0;
    U32 hi = (U32)st->tableLen;
    assert(st->tableLen <= UINT_MAX);

    if (pos >= st->entries[st->tableLen].dOffset) {
        return (unsigned)st->tableLen;
    }

    while (lo + 1 < hi) {
        U32 const mid = lo + ((hi - lo) >> 1);
        if (st->entries[mid].dOffset <= pos) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    return lo;
}

unsigned ZSTD_seekable_getNumFrames(const ZSTD_seekable* zs)
{
    return ZSTD_seekTable_getNumFrames(&zs->seekTable);
}

unsigned ZSTD_seekTable_getNumFrames(const ZSTD_seekTable* st)
{
    assert(st->tableLen <= UINT_MAX);
    return (unsigned)st->tableLen;
}

unsigned long long ZSTD_seekable_getFrameCompressedOffset(const ZSTD_seekable* zs, unsigned frameIndex)
{
    return ZSTD_seekTable_getFrameCompressedOffset(&zs->seekTable, frameIndex);
}

unsigned long long ZSTD_seekTable_getFrameCompressedOffset(const ZSTD_seekTable* st, unsigned frameIndex)
{
    if (frameIndex >= st->tableLen) return ZSTD_SEEKABLE_FRAMEINDEX_TOOLARGE;
    return st->entries[frameIndex].cOffset;
}

unsigned long long ZSTD_seekable_getFrameDecompressedOffset(const ZSTD_seekable* zs, unsigned frameIndex)
{
    return ZSTD_seekTable_getFrameDecompressedOffset(&zs->seekTable, frameIndex);
}

unsigned long long ZSTD_seekTable_getFrameDecompressedOffset(const ZSTD_seekTable* st, unsigned frameIndex)
{
    if (frameIndex >= st->tableLen) return ZSTD_SEEKABLE_FRAMEINDEX_TOOLARGE;
    return st->entries[frameIndex].dOffset;
}

size_t ZSTD_seekable_getFrameCompressedSize(const ZSTD_seekable* zs, unsigned frameIndex)
{
    return ZSTD_seekTable_getFrameCompressedSize(&zs->seekTable, frameIndex);
}

size_t ZSTD_seekTable_getFrameCompressedSize(const ZSTD_seekTable* st, unsigned frameIndex)
{
    if (frameIndex >= st->tableLen) return ERROR(frameIndex_tooLarge);
    return st->entries[frameIndex + 1].cOffset -
           st->entries[frameIndex].cOffset;
}

size_t ZSTD_seekable_getFrameDecompressedSize(const ZSTD_seekable* zs, unsigned frameIndex)
{
    return ZSTD_seekTable_getFrameDecompressedSize(&zs->seekTable, frameIndex);
}

size_t ZSTD_seekTable_getFrameDecompressedSize(const ZSTD_seekTable* st, unsigned frameIndex)
{
    if (frameIndex > st->tableLen) return ERROR(frameIndex_tooLarge);
    return st->entries[frameIndex + 1].dOffset -
           st->entries[frameIndex].dOffset;
}

static size_t ZSTD_seekable_loadSeekTable(ZSTD_seekable* zs)
{
    int checksumFlag;
    ZSTD_seekable_customFile src = zs->src;
    /* read the footer, fixed size */
    CHECK_IO(src.seek(src.opaque, -(int)ZSTD_seekTableFooterSize, SEEK_END));
    CHECK_IO(src.read(src.opaque, zs->inBuff, ZSTD_seekTableFooterSize));

    if (MEM_readLE32(zs->inBuff + 5) != ZSTD_SEEKABLE_MAGICNUMBER) {
        return ERROR(prefix_unknown);
    }

    {   BYTE const sfd = zs->inBuff[4];
        checksumFlag = sfd >> 7;

        /* check reserved bits */
        if ((sfd >> 2) & 0x1f) {
            return ERROR(corruption_detected);
    }   }

    {   U32 const numFrames = MEM_readLE32(zs->inBuff);
        U32 const sizePerEntry = 8 + (checksumFlag?4:0);
        U32 const tableSize = sizePerEntry * numFrames;
        U32 const frameSize = tableSize + ZSTD_seekTableFooterSize + ZSTD_SKIPPABLEHEADERSIZE;

        U32 remaining = frameSize - ZSTD_seekTableFooterSize; /* don't need to re-read footer */
        {   U32 const toRead = MIN(remaining, SEEKABLE_BUFF_SIZE);
            CHECK_IO(src.seek(src.opaque, -(S64)frameSize, SEEK_END));
            CHECK_IO(src.read(src.opaque, zs->inBuff, toRead));
            remaining -= toRead;
        }

        if (MEM_readLE32(zs->inBuff) != (ZSTD_MAGIC_SKIPPABLE_START | 0xE)) {
            return ERROR(prefix_unknown);
        }
        if (MEM_readLE32(zs->inBuff+4) + ZSTD_SKIPPABLEHEADERSIZE != frameSize) {
            return ERROR(prefix_unknown);
        }

        {   /* Allocate an extra entry at the end so that we can do size
             * computations on the last element without special case */
            seekEntry_t* const entries = (seekEntry_t*)malloc(sizeof(seekEntry_t) * (numFrames + 1));

            U32 idx = 0;
            U32 pos = 8;

            U64 cOffset = 0;
            U64 dOffset = 0;

            if (entries == NULL) return ERROR(memory_allocation);

            /* compute cumulative positions */
            for (; idx < numFrames; idx++) {
                if (pos + sizePerEntry > SEEKABLE_BUFF_SIZE) {
                    U32 const offset = SEEKABLE_BUFF_SIZE - pos;
                    U32 const toRead = MIN(remaining, SEEKABLE_BUFF_SIZE - offset);
                    memmove(zs->inBuff, zs->inBuff + pos, offset); /* move any data we haven't read yet */
                    CHECK_IO(src.read(src.opaque, zs->inBuff+offset, toRead));
                    remaining -= toRead;
                    pos = 0;
                }
                entries[idx].cOffset = cOffset;
                entries[idx].dOffset = dOffset;

                cOffset += MEM_readLE32(zs->inBuff + pos);
                pos += 4;
                dOffset += MEM_readLE32(zs->inBuff + pos);
                pos += 4;
                if (checksumFlag) {
                    entries[idx].checksum = MEM_readLE32(zs->inBuff + pos);
                    pos += 4;
                }
            }
            entries[numFrames].cOffset = cOffset;
            entries[numFrames].dOffset = dOffset;

            zs->seekTable.entries = entries;
            zs->seekTable.tableLen = numFrames;
            zs->seekTable.checksumFlag = checksumFlag;
            return 0;
        }
    }
}

size_t ZSTD_seekable_initBuff(ZSTD_seekable* zs, const void* src, size_t srcSize)
{
    zs->buffWrapper = (buffWrapper_t){src, srcSize, 0};
    {   ZSTD_seekable_customFile srcFile = {&zs->buffWrapper,
                                            &ZSTD_seekable_read_buff,
                                            &ZSTD_seekable_seek_buff};
        return ZSTD_seekable_initAdvanced(zs, srcFile); }
}

size_t ZSTD_seekable_initFile(ZSTD_seekable* zs, FILE* src)
{
    ZSTD_seekable_customFile srcFile = {src, &ZSTD_seekable_read_FILE,
                                        &ZSTD_seekable_seek_FILE};
    return ZSTD_seekable_initAdvanced(zs, srcFile);
}

size_t ZSTD_seekable_initAdvanced(ZSTD_seekable* zs, ZSTD_seekable_customFile src)
{
    zs->src = src;

    {   const size_t seekTableInit = ZSTD_seekable_loadSeekTable(zs);
        if (ZSTD_isError(seekTableInit)) return seekTableInit; }

    zs->decompressedOffset = (U64)-1;
    zs->curFrame = (U32)-1;

    {   const size_t dstreamInit = ZSTD_initDStream(zs->dstream);
        if (ZSTD_isError(dstreamInit)) return dstreamInit; }
    return 0;
}

size_t ZSTD_seekable_decompress(ZSTD_seekable* zs, void* dst, size_t len, unsigned long long offset)
{
    unsigned long long const eos = zs->seekTable.entries[zs->seekTable.tableLen].dOffset;
    if (offset + len > eos) {
        len = eos - offset;
    }

    U32 targetFrame = ZSTD_seekable_offsetToFrameIndex(zs, offset);
    U32 noOutputProgressCount = 0;
    size_t srcBytesRead = 0;
    do {
        /* check if we can continue from a previous decompress job */
        if (targetFrame != zs->curFrame || offset < zs->decompressedOffset) {
            zs->decompressedOffset = zs->seekTable.entries[targetFrame].dOffset;
            zs->curFrame = targetFrame;

            assert(zs->seekTable.entries[targetFrame].cOffset < LLONG_MAX);
            CHECK_IO(zs->src.seek(zs->src.opaque,
                                  (long long)zs->seekTable.entries[targetFrame].cOffset,
                                  SEEK_SET));
            zs->in = (ZSTD_inBuffer){zs->inBuff, 0, 0};
            XXH64_reset(&zs->xxhState, 0);
            ZSTD_DCtx_reset(zs->dstream, ZSTD_reset_session_only);
            if (zs->buffWrapper.size && srcBytesRead > zs->buffWrapper.size) {
                return ERROR(seekableIO);
            }
        }

        while (zs->decompressedOffset < offset + len) {
            size_t toRead;
            ZSTD_outBuffer outTmp;
            size_t prevOutPos;
            size_t prevInPos;
            size_t forwardProgress;
            if (zs->decompressedOffset < offset) {
                /* dummy decompressions until we get to the target offset */
                outTmp = (ZSTD_outBuffer){zs->outBuff, (size_t) (MIN(SEEKABLE_BUFF_SIZE, offset - zs->decompressedOffset)), 0};
            } else {
                outTmp = (ZSTD_outBuffer){dst, len, (size_t) (zs->decompressedOffset - offset)};
            }

            prevOutPos = outTmp.pos;
            prevInPos = zs->in.pos;
            toRead = ZSTD_decompressStream(zs->dstream, &outTmp, &zs->in);
            if (ZSTD_isError(toRead)) {
                return toRead;
            }

            if (zs->seekTable.checksumFlag) {
                XXH64_update(&zs->xxhState, (BYTE*)outTmp.dst + prevOutPos,
                             outTmp.pos - prevOutPos);
            }
            forwardProgress = outTmp.pos - prevOutPos;
            if (forwardProgress == 0) {
                if (noOutputProgressCount++ > ZSTD_SEEKABLE_NO_OUTPUT_PROGRESS_MAX) {
                    return ERROR(seekableIO);
                }
            } else {
                noOutputProgressCount = 0;
            }
            zs->decompressedOffset += forwardProgress;
            srcBytesRead += zs->in.pos - prevInPos;

            if (toRead == 0) {
                /* frame complete */

                /* verify checksum */
                if (zs->seekTable.checksumFlag &&
                    (XXH64_digest(&zs->xxhState) & 0xFFFFFFFFU) !=
                            zs->seekTable.entries[targetFrame].checksum) {
                    return ERROR(corruption_detected);
                }

                if (zs->decompressedOffset < offset + len) {
                    /* go back to the start and force a reset of the stream */
                    targetFrame = ZSTD_seekable_offsetToFrameIndex(zs, zs->decompressedOffset);
                    /* in this case it will fail later with corruption_detected, since last block does not have checksum */
                    assert(targetFrame != zs->seekTable.tableLen);
                }
                break;
            }

            /* read in more data if we're done with this buffer */
            if (zs->in.pos == zs->in.size) {
                toRead = MIN(toRead, SEEKABLE_BUFF_SIZE);
                CHECK_IO(zs->src.read(zs->src.opaque, zs->inBuff, toRead));
                zs->in.size = toRead;
                zs->in.pos = 0;
            }
        }  /* while (zs->decompressedOffset < offset + len) */
    } while (zs->decompressedOffset != offset + len);

    return len;
}

size_t ZSTD_seekable_decompressFrame(ZSTD_seekable* zs, void* dst, size_t dstSize, unsigned frameIndex)
{
    if (frameIndex >= zs->seekTable.tableLen) {
        return ERROR(frameIndex_tooLarge);
    }

    {   size_t const decompressedSize =
                zs->seekTable.entries[frameIndex + 1].dOffset -
                zs->seekTable.entries[frameIndex].dOffset;
        if (dstSize < decompressedSize) {
            return ERROR(dstSize_tooSmall);
        }
        return ZSTD_seekable_decompress(
                zs, dst, decompressedSize,
                zs->seekTable.entries[frameIndex].dOffset);
    }
}
