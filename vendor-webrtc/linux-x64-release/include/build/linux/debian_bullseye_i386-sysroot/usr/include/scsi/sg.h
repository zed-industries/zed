/* Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

/*
   History:
    Started: Aug 9 by Lawrence Foard (entropy@world.std.com), to allow user
     process control of SCSI devices.
    Development Sponsored by Killy Corp. NY NY
*/

#ifndef _SCSI_SG_H
#define _SCSI_SG_H	1

#include <features.h>
#define __need_size_t
#include <stddef.h>


/* New interface introduced in the 3.x SG drivers follows */

/* Same structure as used by readv() Linux system call. It defines one
   scatter-gather element. */
typedef struct sg_iovec
{
  void * iov_base;            /* Starting address  */
  size_t iov_len;             /* Length in bytes  */
} sg_iovec_t;


typedef struct sg_io_hdr
{
  int interface_id;           /* [i] 'S' for SCSI generic (required) */
  int dxfer_direction;        /* [i] data transfer direction  */
  unsigned char cmd_len;      /* [i] SCSI command length ( <= 16 bytes) */
  unsigned char mx_sb_len;    /* [i] max length to write to sbp */
  unsigned short int iovec_count; /* [i] 0 implies no scatter gather */
  unsigned int dxfer_len;     /* [i] byte count of data transfer */
  void * dxferp;              /* [i], [*io] points to data transfer memory
				 or scatter gather list */
  unsigned char * cmdp;       /* [i], [*i] points to command to perform */
  unsigned char * sbp;        /* [i], [*o] points to sense_buffer memory */
  unsigned int timeout;       /* [i] MAX_UINT->no timeout (unit: millisec) */
  unsigned int flags;         /* [i] 0 -> default, see SG_FLAG... */
  int pack_id;                /* [i->o] unused internally (normally) */
  void * usr_ptr;             /* [i->o] unused internally */
  unsigned char status;       /* [o] scsi status */
  unsigned char masked_status;/* [o] shifted, masked scsi status */
  unsigned char msg_status;   /* [o] messaging level data (optional) */
  unsigned char sb_len_wr;    /* [o] byte count actually written to sbp */
  unsigned short int host_status; /* [o] errors from host adapter */
  unsigned short int driver_status;/* [o] errors from software driver */
  int resid;                  /* [o] dxfer_len - actual_transferred */
  unsigned int duration;      /* [o] time taken by cmd (unit: millisec) */
  unsigned int info;          /* [o] auxiliary information */
} sg_io_hdr_t;


/* Use negative values to flag difference from original sg_header structure.  */
#define SG_DXFER_NONE -1        /* e.g. a SCSI Test Unit Ready command */
#define SG_DXFER_TO_DEV -2      /* e.g. a SCSI WRITE command */
#define SG_DXFER_FROM_DEV -3    /* e.g. a SCSI READ command */
#define SG_DXFER_TO_FROM_DEV -4 /* treated like SG_DXFER_FROM_DEV with the
				   additional property than during indirect
				   IO the user buffer is copied into the
				   kernel buffers before the transfer */


/* following flag values can be "or"-ed together */
#define SG_FLAG_DIRECT_IO 1     /* default is indirect IO */
#define SG_FLAG_LUN_INHIBIT 2   /* default is to put device's lun into */
				/* the 2nd byte of SCSI command */
#define SG_FLAG_NO_DXFER 0x10000 /* no transfer of kernel buffers to/from */
				/* user space (debug indirect IO) */

/* The following 'info' values are "or"-ed together.  */
#define SG_INFO_OK_MASK	0x1
#define SG_INFO_OK	0x0	/* no sense, host nor driver "noise" */
#define SG_INFO_CHECK	0x1     /* something abnormal happened */

#define SG_INFO_DIRECT_IO_MASK	0x6
#define SG_INFO_INDIRECT_IO 	0x0	/* data xfer via kernel buffers (or no xfer) */
#define SG_INFO_DIRECT_IO 	0x2	/* direct IO requested and performed */
#define SG_INFO_MIXED_IO 	0x4	/* part direct, part indirect IO */


/* Request information about a specific SG device, used by
   SG_GET_SCSI_ID ioctl ().  */
struct sg_scsi_id {
  /* Host number as in "scsi<n>" where 'n' is one of 0, 1, 2 etc.  */
  int host_no;
  int channel;
  /* SCSI id of target device.  */
  int scsi_id;
  int lun;
  /* TYPE_... defined in <scsi/scsi.h>.  */
  int scsi_type;
  /* Host (adapter) maximum commands per lun.  */
  short int h_cmd_per_lun;
  /* Device (or adapter) maximum queue length.  */
  short int d_queue_depth;
  /* Unused, set to 0 for now.  */
  int unused[2];
};

/* Used by SG_GET_REQUEST_TABLE ioctl().  */
typedef struct sg_req_info {
    char req_state;     /* 0 -> not used, 1 -> written, 2 -> ready to read */
    char orphan;        /* 0 -> normal request, 1 -> from interruped SG_IO */
    char sg_io_owned;   /* 0 -> complete with read(), 1 -> owned by SG_IO */
    char problem;       /* 0 -> no problem detected, 1 -> error to report */
    int pack_id;        /* pack_id associated with request */
    void * usr_ptr;     /* user provided pointer (in new interface) */
    unsigned int duration; /* millisecs elapsed since written (req_state==1)
			      or request duration (req_state==2) */
    int unused;
} sg_req_info_t;


/* IOCTLs: Those ioctls that are relevant to the SG 3.x drivers follow.
 [Those that only apply to the SG 2.x drivers are at the end of the file.]
 (_GET_s yield result via 'int *' 3rd argument unless otherwise indicated) */

#define SG_EMULATED_HOST 0x2203 /* true for emulated host adapter (ATAPI) */

/* Used to configure SCSI command transformation layer for ATAPI devices */
/* Only supported by the ide-scsi driver */
#define SG_SET_TRANSFORM 0x2204 /* N.B. 3rd arg is not pointer but value: */
		      /* 3rd arg = 0 to disable transform, 1 to enable it */
#define SG_GET_TRANSFORM 0x2205

#define SG_SET_RESERVED_SIZE 0x2275  /* request a new reserved buffer size */
#define SG_GET_RESERVED_SIZE 0x2272  /* actual size of reserved buffer */

/* The following ioctl has a 'sg_scsi_id_t *' object as its 3rd argument. */
#define SG_GET_SCSI_ID 0x2276   /* Yields fd's bus, chan, dev, lun + type */
/* SCSI id information can also be obtained from SCSI_IOCTL_GET_IDLUN */

/* Override host setting and always DMA using low memory ( <16MB on i386) */
#define SG_SET_FORCE_LOW_DMA 0x2279  /* 0-> use adapter setting, 1-> force */
#define SG_GET_LOW_DMA 0x227a   /* 0-> use all ram for dma; 1-> low dma ram */

/* When SG_SET_FORCE_PACK_ID set to 1, pack_id is input to read() which
   tries to fetch a packet with a matching pack_id, waits, or returns EAGAIN.
   If pack_id is -1 then read oldest waiting. When ...FORCE_PACK_ID set to 0
   then pack_id ignored by read() and oldest readable fetched. */
#define SG_SET_FORCE_PACK_ID 0x227b
#define SG_GET_PACK_ID 0x227c /* Yields oldest readable pack_id (or -1) */

#define SG_GET_NUM_WAITING 0x227d /* Number of commands awaiting read() */

/* Yields max scatter gather tablesize allowed by current host adapter */
#define SG_GET_SG_TABLESIZE 0x227F  /* 0 implies can't do scatter gather */

#define SG_GET_VERSION_NUM 0x2282 /* Example: version 2.1.34 yields 20134 */

/* Returns -EBUSY if occupied. 3rd argument pointer to int (see next) */
#define SG_SCSI_RESET 0x2284
/* Associated values that can be given to SG_SCSI_RESET follow */
#define SG_SCSI_RESET_NOTHING	0
#define SG_SCSI_RESET_DEVICE	1
#define SG_SCSI_RESET_BUS	2
#define SG_SCSI_RESET_HOST	3

/* synchronous SCSI command ioctl, (only in version 3 interface) */
#define SG_IO 0x2285   /* similar effect as write() followed by read() */

#define SG_GET_REQUEST_TABLE 0x2286   /* yields table of active requests */

/* How to treat EINTR during SG_IO ioctl(), only in SG 3.x series */
#define SG_SET_KEEP_ORPHAN 0x2287 /* 1 -> hold for read(), 0 -> drop (def) */
#define SG_GET_KEEP_ORPHAN 0x2288


#define SG_SCATTER_SZ (8 * 4096)  /* PAGE_SIZE not available to user */
/* Largest size (in bytes) a single scatter-gather list element can have.
   The value must be a power of 2 and <= (PAGE_SIZE * 32) [131072 bytes on
   i386]. The minimum value is PAGE_SIZE. If scatter-gather not supported
   by adapter then this value is the largest data block that can be
   read/written by a single scsi command. The user can find the value of
   PAGE_SIZE by calling getpagesize() defined in unistd.h . */

#define SG_DEFAULT_RETRIES 1

/* Defaults, commented if they differ from original sg driver */
#define SG_DEF_FORCE_LOW_DMA 0  /* was 1 -> memory below 16MB on i386 */
#define SG_DEF_FORCE_PACK_ID 0
#define SG_DEF_KEEP_ORPHAN 0
#define SG_DEF_RESERVED_SIZE SG_SCATTER_SZ /* load time option */

/* maximum outstanding requests, write() yields EDOM if exceeded */
#define SG_MAX_QUEUE 16

#define SG_BIG_BUFF SG_DEF_RESERVED_SIZE    /* for backward compatibility */

/* Alternate style type names, "..._t" variants preferred */
typedef struct sg_io_hdr Sg_io_hdr;
typedef struct sg_io_vec Sg_io_vec;
typedef struct sg_scsi_id Sg_scsi_id;
typedef struct sg_req_info Sg_req_info;


/* vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv */
/*   The older SG interface based on the 'sg_header' structure follows.   */
/* ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ */

#define SG_MAX_SENSE 16   /* this only applies to the sg_header interface */

struct sg_header
 {
   /* Length of incoming packet (including header).  */
   int pack_len;
   /* Maximal length of expected reply.  */
   int reply_len;
   /* Id number of packet.  */
   int pack_id;
   /* 0==ok, otherwise error number.  */
   int result;
   /* Force 12 byte command length for group 6 & 7 commands.  */
   unsigned int twelve_byte:1;
   /* SCSI status from target.  */
   unsigned int target_status:5;
   /* Host status (see "DID" codes).  */
   unsigned int host_status:8;
   /* Driver status+suggestion.  */
   unsigned int driver_status:8;
   /* Unused.  */
   unsigned int other_flags:10;
   /* Output in 3 cases:
      when target_status is CHECK_CONDITION or
      when target_status is COMMAND_TERMINATED or
      when (driver_status & DRIVER_SENSE) is true.  */
   unsigned char sense_buffer[SG_MAX_SENSE];
 };


/* IOCTLs: The following are not required (or ignored) when the sg_io_hdr_t
	   interface is used. They are kept for backward compatibility with
	   the original and version 2 drivers. */

#define SG_SET_TIMEOUT		0x2201	/* Set timeout; *(int *)arg==timeout.  */
#define SG_GET_TIMEOUT		0x2202	/* Get timeout; return timeout.  */

/* Get/set command queuing state per fd (default is SG_DEF_COMMAND_Q). */
#define SG_GET_COMMAND_Q	0x2270	/* Yields 0 (queuing off) or 1 (on).  */
#define SG_SET_COMMAND_Q 	0x2271	/* Change queuing state with 0 or 1.  */

/* Turn on error sense trace (1..8), dump this device to log/console (9)
   or dump all sg device states ( >9 ) to log/console.  */
#define SG_SET_DEBUG		0x227e	/* 0 -> turn off debug */

#define SG_NEXT_CMD_LEN		0x2283	/* Override SCSI command length with given
					   number on the next write() on this file
					   descriptor.  */

/* Defaults, commented if they differ from original sg driver */
#define SG_DEFAULT_TIMEOUT (60*HZ) /* HZ == 'jiffies in 1 second' */
#define SG_DEF_COMMAND_Q 0     /* command queuing is always on when
				  the new interface is used */
#define SG_DEF_UNDERRUN_FLAG 0


#endif	/* scsi/sg.h */
