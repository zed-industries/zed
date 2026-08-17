#ifndef MODULES_THIRD_PARTY_PORTAUDIO_PA_RINGBUFFER_H_
#define MODULES_THIRD_PARTY_PORTAUDIO_PA_RINGBUFFER_H_
/*
 * $Id$
 * Portable Audio I/O Library
 * Ring Buffer utility.
 *
 * Author: Phil Burk, http://www.softsynth.com
 * modified for SMP safety on OS X by Bjorn Roche.
 * also allowed for const where possible.
 * modified for multiple-byte-sized data elements by Sven Fischer
 *
 * Note that this is safe only for a single-thread reader
 * and a single-thread writer.
 *
 * This program is distributed with the PortAudio Portable Audio Library.
 * For more information see: http://www.portaudio.com
 * Copyright (c) 1999-2000 Ross Bencina and Phil Burk
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
 * CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
 * WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/*
 * The text above constitutes the entire PortAudio license; however,
 * the PortAudio community also makes the following non-binding requests:
 *
 * Any person wishing to distribute modifications to the Software is
 * requested to send the modifications to the original developer so that
 * they can be incorporated into the canonical version. It is also
 * requested that these non-binding requests be included along with the
 * license above.
 */

/** @file
 @ingroup common_src
 @brief Single-reader single-writer lock-free ring buffer

 PaUtilRingBuffer is a ring buffer used to transport samples between
 different execution contexts (threads, OS callbacks, interrupt handlers)
 without requiring the use of any locks. This only works when there is
 a single reader and a single writer (ie. one thread or callback writes
 to the ring buffer, another thread or callback reads from it).

 The PaUtilRingBuffer structure manages a ring buffer containing N
 elements, where N must be a power of two. An element may be any size
 (specified in bytes).

 The memory area used to store the buffer elements must be allocated by
 the client prior to calling PaUtil_InitializeRingBuffer() and must outlive
 the use of the ring buffer.

 @note The ring buffer functions are not normally exposed in the PortAudio
 libraries. If you want to call them then you will need to add pa_ringbuffer.c
 to your application source code.
*/

#if defined(__APPLE__)
#include <sys/types.h>
typedef int32_t ring_buffer_size_t;
#elif defined(__GNUC__)
typedef long ring_buffer_size_t;
#elif (_MSC_VER >= 1400)
typedef long ring_buffer_size_t;
#elif defined(_MSC_VER) || defined(__BORLANDC__)
typedef long ring_buffer_size_t;
#else
typedef long ring_buffer_size_t;
#endif

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

typedef struct PaUtilRingBuffer {
  ring_buffer_size_t bufferSize; /**< Number of elements in FIFO. Power of 2.
                                    Set by PaUtil_InitRingBuffer. */
  volatile ring_buffer_size_t
      writeIndex; /**< Index of next writable element. Set by
                     PaUtil_AdvanceRingBufferWriteIndex. */
  volatile ring_buffer_size_t
      readIndex;                /**< Index of next readable element. Set by
                                   PaUtil_AdvanceRingBufferReadIndex. */
  ring_buffer_size_t bigMask;   /**< Used for wrapping indices with extra bit to
                                   distinguish full/empty. */
  ring_buffer_size_t smallMask; /**< Used for fitting indices to buffer. */
  ring_buffer_size_t elementSizeBytes; /**< Number of bytes per element. */
  char* buffer; /**< Pointer to the buffer containing the actual data. */
} PaUtilRingBuffer;

/** Initialize Ring Buffer to empty state ready to have elements written to it.

 @param rbuf The ring buffer.

 @param elementSizeBytes The size of a single data element in bytes.

 @param elementCount The number of elements in the buffer (must be a power of
 2).

 @param dataPtr A pointer to a previously allocated area where the data
 will be maintained.  It must be elementCount*elementSizeBytes long.

 @return -1 if elementCount is not a power of 2, otherwise 0.
*/
ring_buffer_size_t PaUtil_InitializeRingBuffer(
    PaUtilRingBuffer* rbuf,
    ring_buffer_size_t elementSizeBytes,
    ring_buffer_size_t elementCount,
    void* dataPtr);

/** Reset buffer to empty. Should only be called when buffer is NOT being read
 or written.

 @param rbuf The ring buffer.
*/
void PaUtil_FlushRingBuffer(PaUtilRingBuffer* rbuf);

/** Retrieve the number of elements available in the ring buffer for writing.

 @param rbuf The ring buffer.

 @return The number of elements available for writing.
*/
ring_buffer_size_t PaUtil_GetRingBufferWriteAvailable(
    const PaUtilRingBuffer* rbuf);

/** Retrieve the number of elements available in the ring buffer for reading.

 @param rbuf The ring buffer.

 @return The number of elements available for reading.
*/
ring_buffer_size_t PaUtil_GetRingBufferReadAvailable(
    const PaUtilRingBuffer* rbuf);

/** Write data to the ring buffer.

 @param rbuf The ring buffer.

 @param data The address of new data to write to the buffer.

 @param elementCount The number of elements to be written.

 @return The number of elements written.
*/
ring_buffer_size_t PaUtil_WriteRingBuffer(PaUtilRingBuffer* rbuf,
                                          const void* data,
                                          ring_buffer_size_t elementCount);

/** Read data from the ring buffer.

 @param rbuf The ring buffer.

 @param data The address where the data should be stored.

 @param elementCount The number of elements to be read.

 @return The number of elements read.
*/
ring_buffer_size_t PaUtil_ReadRingBuffer(PaUtilRingBuffer* rbuf,
                                         void* data,
                                         ring_buffer_size_t elementCount);

/** Get address of region(s) to which we can write data.

 @param rbuf The ring buffer.

 @param elementCount The number of elements desired.

 @param dataPtr1 The address where the first (or only) region pointer will be
 stored.

 @param sizePtr1 The address where the first (or only) region length will be
 stored.

 @param dataPtr2 The address where the second region pointer will be stored if
 the first region is too small to satisfy elementCount.

 @param sizePtr2 The address where the second region length will be stored if
 the first region is too small to satisfy elementCount.

 @return The room available to be written or elementCount, whichever is smaller.
*/
ring_buffer_size_t PaUtil_GetRingBufferWriteRegions(
    PaUtilRingBuffer* rbuf,
    ring_buffer_size_t elementCount,
    void** dataPtr1,
    ring_buffer_size_t* sizePtr1,
    void** dataPtr2,
    ring_buffer_size_t* sizePtr2);

/** Advance the write index to the next location to be written.

 @param rbuf The ring buffer.

 @param elementCount The number of elements to advance.

 @return The new position.
*/
ring_buffer_size_t PaUtil_AdvanceRingBufferWriteIndex(
    PaUtilRingBuffer* rbuf,
    ring_buffer_size_t elementCount);

/** Get address of region(s) from which we can read data.

 @param rbuf The ring buffer.

 @param elementCount The number of elements desired.

 @param dataPtr1 The address where the first (or only) region pointer will be
 stored.

 @param sizePtr1 The address where the first (or only) region length will be
 stored.

 @param dataPtr2 The address where the second region pointer will be stored if
 the first region is too small to satisfy elementCount.

 @param sizePtr2 The address where the second region length will be stored if
 the first region is too small to satisfy elementCount.

 @return The number of elements available for reading.
*/
ring_buffer_size_t PaUtil_GetRingBufferReadRegions(
    PaUtilRingBuffer* rbuf,
    ring_buffer_size_t elementCount,
    void** dataPtr1,
    ring_buffer_size_t* sizePtr1,
    void** dataPtr2,
    ring_buffer_size_t* sizePtr2);

/** Advance the read index to the next location to be read.

 @param rbuf The ring buffer.

 @param elementCount The number of elements to advance.

 @return The new position.
*/
ring_buffer_size_t PaUtil_AdvanceRingBufferReadIndex(
    PaUtilRingBuffer* rbuf,
    ring_buffer_size_t elementCount);

#ifdef __cplusplus
}
#endif /* __cplusplus */
#endif /* MODULES_THIRD_PARTY_PORTAUDIO_PA_RINGBUFFER_H_ */
