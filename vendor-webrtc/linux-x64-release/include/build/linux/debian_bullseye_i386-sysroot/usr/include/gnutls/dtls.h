/*
 * Copyright (C) 2011-2012 Free Software Foundation, Inc.
 *
 * Author: Nikos Mavrogiannopoulos
 *
 * This file is part of GnuTLS.
 *
 * The GnuTLS is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 *
 */

/* This file contains the types and prototypes for the X.509
 * certificate and CRL handling functions.
 */

#ifndef GNUTLS_DTLS_H
#define GNUTLS_DTLS_H

#include <gnutls/gnutls.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

#define GNUTLS_COOKIE_KEY_SIZE 16

void gnutls_dtls_set_timeouts(gnutls_session_t session,
			      unsigned int retrans_timeout,
			      unsigned int total_timeout);

unsigned int gnutls_dtls_get_mtu(gnutls_session_t session);
unsigned int gnutls_dtls_get_data_mtu(gnutls_session_t session);

void gnutls_dtls_set_mtu(gnutls_session_t session, unsigned int mtu);
int gnutls_dtls_set_data_mtu(gnutls_session_t session, unsigned int mtu);

unsigned int gnutls_dtls_get_timeout(gnutls_session_t session);

/**
 * gnutls_dtls_prestate_st:
 * @record_seq: record sequence number
 * @hsk_read_seq: handshake read sequence number
 * @hsk_write_seq: handshake write sequence number
 *
 * DTLS cookie prestate struct.  This is usually never modified by
 * the application, it is used to carry the cookie data between
 * gnutls_dtls_cookie_send(), gnutls_dtls_cookie_verify() and
 * gnutls_dtls_prestate_set().
 */
typedef struct {
	unsigned int record_seq;
	unsigned int hsk_read_seq;
	unsigned int hsk_write_seq;
} gnutls_dtls_prestate_st;

int gnutls_dtls_cookie_send(gnutls_datum_t * key,
			    void *client_data,
			    size_t client_data_size,
			    gnutls_dtls_prestate_st * prestate,
			    gnutls_transport_ptr_t ptr,
			    gnutls_push_func push_func);

int gnutls_dtls_cookie_verify(gnutls_datum_t * key,
			      void *client_data,
			      size_t client_data_size, void *_msg,
			      size_t msg_size,
			      gnutls_dtls_prestate_st * prestate);

void gnutls_dtls_prestate_set(gnutls_session_t session,
			      gnutls_dtls_prestate_st * prestate);

unsigned int gnutls_record_get_discarded(gnutls_session_t session);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_DTLS_H */
