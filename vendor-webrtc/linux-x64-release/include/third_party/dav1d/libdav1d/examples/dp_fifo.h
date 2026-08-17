/*
 * Copyright © 2019, VideoLAN and dav1d authors
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*
 * Dav1dPlay FIFO helper
 */

typedef struct dp_fifo Dav1dPlayPtrFifo;

/* Create a FIFO
 *
 * Creates a FIFO with the given capacity.
 * If the capacity is reached, new inserts into the FIFO
 * will block until enough space is available again.
 */
Dav1dPlayPtrFifo *dp_fifo_create(size_t capacity);

/* Destroy a FIFO
 *
 * The FIFO must be empty before it is destroyed!
 */
void dp_fifo_destroy(Dav1dPlayPtrFifo *fifo);

/* Shift FIFO
 *
 * Return the first item from the FIFO, thereby removing it from
 * the FIFO and making room for new entries.
 */
void *dp_fifo_shift(Dav1dPlayPtrFifo *fifo);

/* Push to FIFO
 *
 * Add an item to the end of the FIFO.
 * If the FIFO is full, this call will block until there is again enough
 * space in the FIFO, so calling this from the "consumer" thread if no
 * other thread will call dp_fifo_shift will lead to a deadlock.
 */
void dp_fifo_push(Dav1dPlayPtrFifo *fifo, void *element);

void dp_fifo_flush(Dav1dPlayPtrFifo *fifo, void (*destroy_elem)(void *));
