/* Structures and definitions for magnetic tape I/O control commands.
   Copyright (C) 1996-2020 Free Software Foundation, Inc.
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

/* Written by H. Bergman <hennus@cybercomm.nl>.  */

#ifndef _SYS_MTIO_H
#define _SYS_MTIO_H	1

/* Get necessary definitions from system and kernel headers.  */
#include <sys/types.h>
#include <sys/ioctl.h>


/* Structure for MTIOCTOP - magnetic tape operation command.  */
struct mtop
  {
    short int mt_op;		/* Operations defined below.  */
    int mt_count;		/* How many of them.  */
  };
#define _IOT_mtop /* Hurd ioctl type field.  */ \
  _IOT (_IOTS (short), 1, _IOTS (int), 1, 0, 0)

/* Magnetic Tape operations [Not all operations supported by all drivers].  */
#define MTRESET 0	/* +reset drive in case of problems.  */
#define MTFSF	1	/* Forward space over FileMark,
			 * position at first record of next file.  */
#define MTBSF	2	/* Backward space FileMark (position before FM).  */
#define MTFSR	3	/* Forward space record.  */
#define MTBSR	4	/* Backward space record.  */
#define MTWEOF	5	/* Write an end-of-file record (mark).  */
#define MTREW	6	/* Rewind.  */
#define MTOFFL	7	/* Rewind and put the drive offline (eject?).  */
#define MTNOP	8	/* No op, set status only (read with MTIOCGET).  */
#define MTRETEN 9	/* Retension tape.  */
#define MTBSFM	10	/* +backward space FileMark, position at FM.  */
#define MTFSFM  11	/* +forward space FileMark, position at FM.  */
#define MTEOM	12	/* Goto end of recorded media (for appending files).
			   MTEOM positions after the last FM, ready for
			   appending another file.  */
#define MTERASE 13	/* Erase tape -- be careful!  */

#define MTRAS1  14	/* Run self test 1 (nondestructive).  */
#define MTRAS2	15	/* Run self test 2 (destructive).  */
#define MTRAS3  16	/* Reserved for self test 3.  */

#define MTSETBLK 20	/* Set block length (SCSI).  */
#define MTSETDENSITY 21	/* Set tape density (SCSI).  */
#define MTSEEK	22	/* Seek to block (Tandberg, etc.).  */
#define MTTELL	23	/* Tell block (Tandberg, etc.).  */
#define MTSETDRVBUFFER 24 /* Set the drive buffering according to SCSI-2.
			     Ordinary buffered operation with code 1.  */
#define MTFSS	25	/* Space forward over setmarks.  */
#define MTBSS	26	/* Space backward over setmarks.  */
#define MTWSM	27	/* Write setmarks.  */

#define MTLOCK  28	/* Lock the drive door.  */
#define MTUNLOCK 29	/* Unlock the drive door.  */
#define MTLOAD  30	/* Execute the SCSI load command.  */
#define MTUNLOAD 31	/* Execute the SCSI unload command.  */
#define MTCOMPRESSION 32/* Control compression with SCSI mode page 15.  */
#define MTSETPART 33	/* Change the active tape partition.  */
#define MTMKPART  34	/* Format the tape with one or two partitions.  */

/* structure for MTIOCGET - mag tape get status command */

struct mtget
  {
    long int mt_type;		/* Type of magtape device.  */
    long int mt_resid;		/* Residual count: (not sure)
				   number of bytes ignored, or
				   number of files not skipped, or
				   number of records not skipped.  */
    /* The following registers are device dependent.  */
    long int mt_dsreg;		/* Status register.  */
    long int mt_gstat;		/* Generic (device independent) status.  */
    long int mt_erreg;		/* Error register.  */
    /* The next two fields are not always used.  */
    __daddr_t mt_fileno;	/* Number of current file on tape.  */
    __daddr_t mt_blkno;		/* Current block number.  */
  };
#define _IOT_mtget /* Hurd ioctl type field.  */ \
  _IOT (_IOTS (long), 7, 0, 0, 0, 0)


/* Constants for mt_type. Not all of these are supported, and
   these are not all of the ones that are supported.  */
#define MT_ISUNKNOWN		0x01
#define MT_ISQIC02		0x02	/* Generic QIC-02 tape streamer.  */
#define MT_ISWT5150		0x03	/* Wangtek 5150EQ, QIC-150, QIC-02.  */
#define MT_ISARCHIVE_5945L2	0x04	/* Archive 5945L-2, QIC-24, QIC-02?. */
#define MT_ISCMSJ500		0x05	/* CMS Jumbo 500 (QIC-02?).  */
#define MT_ISTDC3610		0x06	/* Tandberg 6310, QIC-24.  */
#define MT_ISARCHIVE_VP60I	0x07	/* Archive VP60i, QIC-02.  */
#define MT_ISARCHIVE_2150L	0x08	/* Archive Viper 2150L.  */
#define MT_ISARCHIVE_2060L	0x09	/* Archive Viper 2060L.  */
#define MT_ISARCHIVESC499	0x0A	/* Archive SC-499 QIC-36 controller. */
#define MT_ISQIC02_ALL_FEATURES	0x0F	/* Generic QIC-02 with all features. */
#define MT_ISWT5099EEN24	0x11	/* Wangtek 5099-een24, 60MB, QIC-24. */
#define MT_ISTEAC_MT2ST		0x12	/* Teac MT-2ST 155mb drive,
					   Teac DC-1 card (Wangtek type).  */
#define MT_ISEVEREX_FT40A	0x32	/* Everex FT40A (QIC-40).  */
#define MT_ISDDS1		0x51	/* DDS device without partitions.  */
#define MT_ISDDS2		0x52	/* DDS device with partitions.  */
#define MT_ISSCSI1		0x71	/* Generic ANSI SCSI-1 tape unit.  */
#define MT_ISSCSI2		0x72	/* Generic ANSI SCSI-2 tape unit.  */

/* QIC-40/80/3010/3020 ftape supported drives.
   20bit vendor ID + 0x800000 (see vendors.h in ftape distribution).  */
#define MT_ISFTAPE_UNKNOWN	0x800000 /* obsolete */
#define MT_ISFTAPE_FLAG		0x800000

struct mt_tape_info
  {
    long int t_type;		/* Device type id (mt_type).  */
    char *t_name;		/* Descriptive name.  */
  };

#define MT_TAPE_INFO \
  {									      \
	{MT_ISUNKNOWN,		"Unknown type of tape device"},		      \
	{MT_ISQIC02,		"Generic QIC-02 tape streamer"},	      \
	{MT_ISWT5150,		"Wangtek 5150, QIC-150"},		      \
	{MT_ISARCHIVE_5945L2,	"Archive 5945L-2"},			      \
	{MT_ISCMSJ500,		"CMS Jumbo 500"},			      \
	{MT_ISTDC3610,		"Tandberg TDC 3610, QIC-24"},		      \
	{MT_ISARCHIVE_VP60I,	"Archive VP60i, QIC-02"},		      \
	{MT_ISARCHIVE_2150L,	"Archive Viper 2150L"},			      \
	{MT_ISARCHIVE_2060L,	"Archive Viper 2060L"},			      \
	{MT_ISARCHIVESC499,	"Archive SC-499 QIC-36 controller"},	      \
	{MT_ISQIC02_ALL_FEATURES, "Generic QIC-02 tape, all features"},	      \
	{MT_ISWT5099EEN24,	"Wangtek 5099-een24, 60MB"},		      \
	{MT_ISTEAC_MT2ST,	"Teac MT-2ST 155mb data cassette drive"},     \
	{MT_ISEVEREX_FT40A,	"Everex FT40A, QIC-40"},		      \
	{MT_ISSCSI1,		"Generic SCSI-1 tape"},			      \
	{MT_ISSCSI2,		"Generic SCSI-2 tape"},			      \
	{0, NULL}							      \
  }


/* Structure for MTIOCPOS - mag tape get position command.  */

struct mtpos
  {
    long int mt_blkno;	/* Current block number.  */
  };
#define _IOT_mtpos /* Hurd ioctl type field.  */ \
  _IOT_SIMPLE (long)


/* Structure for MTIOCGETCONFIG/MTIOCSETCONFIG primarily intended
   as an interim solution for QIC-02 until DDI is fully implemented.  */
struct mtconfiginfo
  {
    long int mt_type;		/* Drive type.  */
    long int ifc_type;		/* Interface card type.  */
    unsigned short int irqnr;	/* IRQ number to use.  */
    unsigned short int dmanr;	/* DMA channel to use.  */
    unsigned short int port;	/* IO port base address.  */

    unsigned long int debug;	/* Debugging flags.  */

    unsigned have_dens:1;
    unsigned have_bsf:1;
    unsigned have_fsr:1;
    unsigned have_bsr:1;
    unsigned have_eod:1;
    unsigned have_seek:1;
    unsigned have_tell:1;
    unsigned have_ras1:1;
    unsigned have_ras2:1;
    unsigned have_ras3:1;
    unsigned have_qfa:1;

    unsigned pad1:5;
    char reserved[10];
  };
#define _IOT_mtconfiginfo /* Hurd ioctl type field.  */ \
  _IOT (_IOTS (long), 2, _IOTS (short), 3, _IOTS (long), 1) /* XXX wrong */


/* Magnetic tape I/O control commands.  */
#define	MTIOCTOP	_IOW('m', 1, struct mtop)	/* Do a mag tape op. */
#define	MTIOCGET	_IOR('m', 2, struct mtget)	/* Get tape status.  */
#define	MTIOCPOS	_IOR('m', 3, struct mtpos)	/* Get tape position.*/

/* The next two are used by the QIC-02 driver for runtime reconfiguration.
   See tpqic02.h for struct mtconfiginfo.  */
#define	MTIOCGETCONFIG	_IOR('m', 4, struct mtconfiginfo) /* Get tape config.*/
#define	MTIOCSETCONFIG	_IOW('m', 5, struct mtconfiginfo) /* Set tape config.*/

/* Generic Mag Tape (device independent) status macros for examining
   mt_gstat -- HP-UX compatible.
   There is room for more generic status bits here, but I don't
   know which of them are reserved. At least three or so should
   be added to make this really useful.  */
#define GMT_EOF(x)              ((x) & 0x80000000)
#define GMT_BOT(x)              ((x) & 0x40000000)
#define GMT_EOT(x)              ((x) & 0x20000000)
#define GMT_SM(x)               ((x) & 0x10000000)  /* DDS setmark */
#define GMT_EOD(x)              ((x) & 0x08000000)  /* DDS EOD */
#define GMT_WR_PROT(x)          ((x) & 0x04000000)
/* #define GMT_ ? 		((x) & 0x02000000) */
#define GMT_ONLINE(x)           ((x) & 0x01000000)
#define GMT_D_6250(x)           ((x) & 0x00800000)
#define GMT_D_1600(x)           ((x) & 0x00400000)
#define GMT_D_800(x)            ((x) & 0x00200000)
/* #define GMT_ ? 		((x) & 0x00100000) */
/* #define GMT_ ? 		((x) & 0x00080000) */
#define GMT_DR_OPEN(x)          ((x) & 0x00040000)  /* Door open (no tape).  */
/* #define GMT_ ? 		((x) & 0x00020000) */
#define GMT_IM_REP_EN(x)        ((x) & 0x00010000)  /* Immediate report mode.*/
/* 16 generic status bits unused.  */


/* SCSI-tape specific definitions.  Bitfield shifts in the status  */
#define MT_ST_BLKSIZE_SHIFT	0
#define MT_ST_BLKSIZE_MASK	0xffffff
#define MT_ST_DENSITY_SHIFT	24
#define MT_ST_DENSITY_MASK	0xff000000

#define MT_ST_SOFTERR_SHIFT	0
#define MT_ST_SOFTERR_MASK	0xffff

/* Bitfields for the MTSETDRVBUFFER ioctl.  */
#define MT_ST_OPTIONS		0xf0000000
#define MT_ST_BOOLEANS		0x10000000
#define MT_ST_SETBOOLEANS	0x30000000
#define MT_ST_CLEARBOOLEANS	0x40000000
#define MT_ST_WRITE_THRESHOLD	0x20000000
#define MT_ST_DEF_BLKSIZE	0x50000000
#define MT_ST_DEF_OPTIONS	0x60000000

#define MT_ST_BUFFER_WRITES	0x1
#define MT_ST_ASYNC_WRITES	0x2
#define MT_ST_READ_AHEAD	0x4
#define MT_ST_DEBUGGING		0x8
#define MT_ST_TWO_FM		0x10
#define MT_ST_FAST_MTEOM	0x20
#define MT_ST_AUTO_LOCK		0x40
#define MT_ST_DEF_WRITES	0x80
#define MT_ST_CAN_BSR		0x100
#define MT_ST_NO_BLKLIMS	0x200
#define MT_ST_CAN_PARTITIONS    0x400
#define MT_ST_SCSI2LOGICAL      0x800

/* The mode parameters to be controlled. Parameter chosen with bits 20-28.  */
#define MT_ST_CLEAR_DEFAULT	0xfffff
#define MT_ST_DEF_DENSITY	(MT_ST_DEF_OPTIONS | 0x100000)
#define MT_ST_DEF_COMPRESSION	(MT_ST_DEF_OPTIONS | 0x200000)
#define MT_ST_DEF_DRVBUFFER	(MT_ST_DEF_OPTIONS | 0x300000)

/* The offset for the arguments for the special HP changer load command.  */
#define MT_ST_HPLOADER_OFFSET 10000


/* Specify default tape device.  */
#ifndef DEFTAPE
# define DEFTAPE	"/dev/tape"
#endif

#endif /* mtio.h */
