/*
 *
 *  BlueZ - Bluetooth protocol stack for Linux
 *
 *  Copyright (C) 2002-2003  Maxim Krasnyansky <maxk@qualcomm.com>
 *  Copyright (C) 2002-2010  Marcel Holtmann <marcel@holtmann.org>
 *
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 */

#ifndef __RFCOMM_H
#define __RFCOMM_H

#ifdef __cplusplus
extern "C" {
#endif

#include <sys/socket.h>

/* RFCOMM defaults */
#define RFCOMM_DEFAULT_MTU	127

#define RFCOMM_PSM 3

/* RFCOMM socket address */
struct sockaddr_rc {
	sa_family_t	rc_family;
	bdaddr_t	rc_bdaddr;
	uint8_t		rc_channel;
};

/* RFCOMM socket options */
#define RFCOMM_CONNINFO	0x02
struct rfcomm_conninfo {
	uint16_t	hci_handle;
	uint8_t		dev_class[3];
};

#define RFCOMM_LM	0x03
#define RFCOMM_LM_MASTER	0x0001
#define RFCOMM_LM_AUTH		0x0002
#define RFCOMM_LM_ENCRYPT	0x0004
#define RFCOMM_LM_TRUSTED	0x0008
#define RFCOMM_LM_RELIABLE	0x0010
#define RFCOMM_LM_SECURE	0x0020

/* RFCOMM TTY support */
#define RFCOMM_MAX_DEV	256

#define RFCOMMCREATEDEV		_IOW('R', 200, int)
#define RFCOMMRELEASEDEV	_IOW('R', 201, int)
#define RFCOMMGETDEVLIST	_IOR('R', 210, int)
#define RFCOMMGETDEVINFO	_IOR('R', 211, int)

struct rfcomm_dev_req {
	int16_t		dev_id;
	uint32_t	flags;
	bdaddr_t	src;
	bdaddr_t	dst;
	uint8_t	channel;
};
#define RFCOMM_REUSE_DLC	0
#define RFCOMM_RELEASE_ONHUP	1
#define RFCOMM_HANGUP_NOW	2
#define RFCOMM_TTY_ATTACHED	3

struct rfcomm_dev_info {
	int16_t		id;
	uint32_t	flags;
	uint16_t	state;
	bdaddr_t	src;
	bdaddr_t	dst;
	uint8_t		channel;
};

struct rfcomm_dev_list_req {
	uint16_t	dev_num;
	struct rfcomm_dev_info dev_info[0];
};

#ifdef __cplusplus
}
#endif

#endif /* __RFCOMM_H */
