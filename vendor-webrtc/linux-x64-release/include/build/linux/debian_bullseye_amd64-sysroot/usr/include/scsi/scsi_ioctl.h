/* Copyright (C) 1999-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _SCSI_IOCTL_H
#define _SCSI_IOCTL_H

/* IOCTLs for SCSI.  */
#define SCSI_IOCTL_SEND_COMMAND		1	/* Send a command to the SCSI host.  */
#define SCSI_IOCTL_TEST_UNIT_READY	2	/* Test if unit is ready.  */
#define SCSI_IOCTL_BENCHMARK_COMMAND	3
#define SCSI_IOCTL_SYNC			4	/* Request synchronous parameters.  */
#define SCSI_IOCTL_START_UNIT		5
#define SCSI_IOCTL_STOP_UNIT		6
#define SCSI_IOCTL_DOORLOCK		0x5380	/* Lock the eject mechanism.  */
#define SCSI_IOCTL_DOORUNLOCK		0x5381	/* Unlock the mechanism.  */

#endif
