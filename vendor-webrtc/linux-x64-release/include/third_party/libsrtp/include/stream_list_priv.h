/*
 * stream_list_priv.h
 *
 * list of SRTP streams, keyed by SSRC
 *
 * Alba Mendez
 */
/*
 *
 * Copyright (c) 2001-2017, Cisco Systems, Inc.
 * Copyright (c) 2022, Dolby Laboratories, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 *   Redistributions in binary form must reproduce the above
 *   copyright notice, this list of conditions and the following
 *   disclaimer in the documentation and/or other materials provided
 *   with the distribution.
 *
 *   Neither the name of the Cisco Systems, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived
 *   from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef SRTP_STREAM_LIST_PRIV_H
#define SRTP_STREAM_LIST_PRIV_H

#include "srtp_priv.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * srtp_stream_list_t holds a list of srtp_stream_t, each identified
 * by their SSRC.
 *
 * the API was extracted to allow downstreams to override its
 * implementation by defining the `SRTP_NO_STREAM_LIST` preprocessor
 * directive, which removes the default implementation of these
 * functions. if this is done, the `next` & `prev` fields are free for
 * the implementation to use.
 *
 * this is still an internal interface; there is no stability
 * guarantee--downstreams should watch this file for changes in
 * signatures or semantics.
 */

/**
 * allocate and initialize a stream list instance
 */
srtp_err_status_t srtp_stream_list_alloc(srtp_stream_list_t *list_ptr);

/**
 * deallocate a stream list instance
 *
 * the list must be empty or else an error is returned.
 */
srtp_err_status_t srtp_stream_list_dealloc(srtp_stream_list_t list);

/**
 * insert a stream into the list
 *
 * returns srtp_err_status_alloc_fail if insertion failed due to unavailable
 * capacity in the list. if operation succeeds, srtp_err_status_ok is returned
 *
 * if another stream with the same SSRC already exists in the list,
 * behavior is undefined. if the SSRC field is mutated while the
 * stream is inserted, further operations have undefined behavior
 */
srtp_err_status_t srtp_stream_list_insert(srtp_stream_list_t list,
                                          srtp_stream_t stream);

/*
 * look up the stream corresponding to the specified SSRC and return it.
 * if no such SSRC is found, NULL is returned.
 */
srtp_stream_t srtp_stream_list_get(srtp_stream_list_t list, uint32_t ssrc);

/**
 * remove the stream from the list.
 *
 * The stream to be removed is referenced "by value", i.e., by the pointer to be
 * removed from the list. This pointer is obtained using `srtp_stream_list_get`
 * or as callback parameter in `srtp_stream_list_for_each`.
 */
void srtp_stream_list_remove(srtp_stream_list_t list, srtp_stream_t stream);

/**
 * iterate through all stored streams. while iterating, it is allowed to delete
 * the current element; any other mutation to the list is undefined behavior.
 * returning non-zero from callback aborts the iteration.
 */
void srtp_stream_list_for_each(srtp_stream_list_t list,
                               int (*callback)(srtp_stream_t, void *),
                               void *data);

#ifdef __cplusplus
}
#endif

#endif /* SRTP_STREAM_LIST_PRIV_H */
