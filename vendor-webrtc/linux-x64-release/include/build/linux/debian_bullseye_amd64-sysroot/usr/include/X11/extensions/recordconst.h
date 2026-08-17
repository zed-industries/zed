/***************************************************************************
 * Copyright 1995 Network Computing Devices
 *
 * Permission to use, copy, modify, distribute, and sell this software and
 * its documentation for any purpose is hereby granted without fee, provided
 * that the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Network Computing Devices
 * not be used in advertising or publicity pertaining to distribution
 * of the software without specific, written prior permission.
 *
 * NETWORK COMPUTING DEVICES DISCLAIMs ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL NETWORK COMPUTING DEVICES BE LIABLE
 * FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 **************************************************************************/

#ifndef _RECORDCONST_H_
#define _RECORDCONST_H_

#define RECORD_NAME			"RECORD"
#define RECORD_MAJOR_VERSION		1
#define RECORD_MINOR_VERSION		13
#define RECORD_LOWEST_MAJOR_VERSION	1
#define RECORD_LOWEST_MINOR_VERSION	12

#define XRecordBadContext       0	/* Not a valid RC */

#define RecordNumErrors         (XRecordBadContext + 1)
#define RecordNumEvents		0L

/*
 * Constants for arguments of various requests
 */
#define	XRecordFromServerTime		0x01
#define	XRecordFromClientTime		0x02
#define	XRecordFromClientSequence	0x04

#define XRecordCurrentClients		1
#define XRecordFutureClients		2
#define XRecordAllClients		3

#define XRecordFromServer           	0
#define XRecordFromClient               1
#define XRecordClientStarted           	2
#define XRecordClientDied               3
#define XRecordStartOfData		4
#define XRecordEndOfData		5


#endif /* _RECORD_H_ */
