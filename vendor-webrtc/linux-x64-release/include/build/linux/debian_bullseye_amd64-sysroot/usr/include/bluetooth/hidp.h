/*
 *
 *  BlueZ - Bluetooth protocol stack for Linux
 *
 *  Copyright (C) 2003-2010  Marcel Holtmann <marcel@holtmann.org>
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

#ifndef __HIDP_H
#define __HIDP_H

#ifdef __cplusplus
extern "C" {
#endif

/* HIDP defaults */
#define HIDP_MINIMUM_MTU 48
#define HIDP_DEFAULT_MTU 48

/* HIDP ioctl defines */
#define HIDPCONNADD	_IOW('H', 200, int)
#define HIDPCONNDEL	_IOW('H', 201, int)
#define HIDPGETCONNLIST	_IOR('H', 210, int)
#define HIDPGETCONNINFO	_IOR('H', 211, int)

#define HIDP_VIRTUAL_CABLE_UNPLUG	0
#define HIDP_BOOT_PROTOCOL_MODE		1
#define HIDP_BLUETOOTH_VENDOR_ID	9

struct hidp_connadd_req {
	int ctrl_sock;		/* Connected control socket */
	int intr_sock;		/* Connected interrupt socket */
	uint16_t parser;	/* Parser version */
	uint16_t rd_size;	/* Report descriptor size */
	uint8_t *rd_data;	/* Report descriptor data */
	uint8_t  country;
	uint8_t  subclass;
	uint16_t vendor;
	uint16_t product;
	uint16_t version;
	uint32_t flags;
	uint32_t idle_to;
	char name[128];		/* Device name */
};

struct hidp_conndel_req {
	bdaddr_t bdaddr;
	uint32_t flags;
};

struct hidp_conninfo {
	bdaddr_t bdaddr;
	uint32_t flags;
	uint16_t state;
	uint16_t vendor;
	uint16_t product;
	uint16_t version;
	char name[128];
};

struct hidp_connlist_req {
	uint32_t cnum;
	struct hidp_conninfo *ci;
};

#ifdef __cplusplus
}
#endif

#endif /* __HIDP_H */
