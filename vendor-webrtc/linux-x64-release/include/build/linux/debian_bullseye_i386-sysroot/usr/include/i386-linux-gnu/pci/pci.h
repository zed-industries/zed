/*
 *	The PCI Library
 *
 *	Copyright (c) 1997--2020 Martin Mares <mj@ucw.cz>
 *
 *	Can be freely distributed and used under the terms of the GNU GPL.
 */

#ifndef _PCI_LIB_H
#define _PCI_LIB_H

#ifndef PCI_CONFIG_H
#include "config.h"
#endif

#include "header.h"
#include "types.h"

#define PCI_LIB_VERSION 0x030700

#ifndef PCI_ABI
#define PCI_ABI
#endif

/*
 *	PCI Access Structure
 */

struct pci_methods;

enum pci_access_type {
  /* Known access methods, remember to update init.c as well */
  PCI_ACCESS_AUTO,			/* Autodetection */
  PCI_ACCESS_SYS_BUS_PCI,		/* Linux /sys/bus/pci */
  PCI_ACCESS_PROC_BUS_PCI,		/* Linux /proc/bus/pci */
  PCI_ACCESS_I386_TYPE1,		/* i386 ports, type 1 */
  PCI_ACCESS_I386_TYPE2,		/* i386 ports, type 2 */
  PCI_ACCESS_FBSD_DEVICE,		/* FreeBSD /dev/pci */
  PCI_ACCESS_AIX_DEVICE,		/* /dev/pci0, /dev/bus0, etc. */
  PCI_ACCESS_NBSD_LIBPCI,		/* NetBSD libpci */
  PCI_ACCESS_OBSD_DEVICE,		/* OpenBSD /dev/pci */
  PCI_ACCESS_DUMP,			/* Dump file */
  PCI_ACCESS_DARWIN,			/* Darwin */
  PCI_ACCESS_SYLIXOS_DEVICE,		/* SylixOS pci */
  PCI_ACCESS_HURD,			/* GNU/Hurd */
  PCI_ACCESS_MAX
};

struct pci_access {
  /* Options you can change: */
  unsigned int method;			/* Access method */
  int writeable;			/* Open in read/write mode */
  int buscentric;			/* Bus-centric view of the world */

  char *id_file_name;			/* Name of ID list file (use pci_set_name_list_path()) */
  int free_id_name;			/* Set if id_file_name is malloced */
  int numeric_ids;			/* Enforce PCI_LOOKUP_NUMERIC (>1 => PCI_LOOKUP_MIXED) */

  unsigned int id_lookup_mode;		/* pci_lookup_mode flags which are set automatically */
					/* Default: PCI_LOOKUP_CACHE */

  int debugging;			/* Turn on debugging messages */

  /* Functions you can override: */
  void (*error)(char *msg, ...) PCI_PRINTF(1,2);	/* Write error message and quit */
  void (*warning)(char *msg, ...) PCI_PRINTF(1,2);	/* Write a warning message */
  void (*debug)(char *msg, ...) PCI_PRINTF(1,2);	/* Write a debugging message */

  struct pci_dev *devices;		/* Devices found on this bus */

  /* Fields used internally: */
  struct pci_methods *methods;
  struct pci_param *params;
  struct id_entry **id_hash;		/* names.c */
  struct id_bucket *current_id_bucket;
  int id_load_failed;
  int id_cache_status;			/* 0=not read, 1=read, 2=dirty */
  struct udev *id_udev;			/* names-hwdb.c */
  struct udev_hwdb *id_udev_hwdb;
  int fd;				/* proc/sys: fd for config space */
  int fd_rw;				/* proc/sys: fd opened read-write */
  int fd_pos;				/* proc/sys: current position */
  int fd_vpd;				/* sys: fd for VPD */
  struct pci_dev *cached_dev;		/* proc/sys: device the fds are for */
};

/* Initialize PCI access */
struct pci_access *pci_alloc(void) PCI_ABI;
void pci_init(struct pci_access *) PCI_ABI;
void pci_cleanup(struct pci_access *) PCI_ABI;

/* Scanning of devices */
void pci_scan_bus(struct pci_access *acc) PCI_ABI;
struct pci_dev *pci_get_dev(struct pci_access *acc, int domain, int bus, int dev, int func) PCI_ABI; /* Raw access to specified device */
void pci_free_dev(struct pci_dev *) PCI_ABI;

/* Names of access methods */
int pci_lookup_method(char *name) PCI_ABI;	/* Returns -1 if not found */
char *pci_get_method_name(int index) PCI_ABI;	/* Returns "" if unavailable, NULL if index out of range */

/*
 *	Named parameters
 */

struct pci_param {
  struct pci_param *next;		/* Please use pci_walk_params() for traversing the list */
  char *param;				/* Name of the parameter */
  char *value;				/* Value of the parameter */
  int value_malloced;			/* used internally */
  char *help;				/* Explanation of the parameter */
};

char *pci_get_param(struct pci_access *acc, char *param) PCI_ABI;
int pci_set_param(struct pci_access *acc, char *param, char *value) PCI_ABI;	/* 0 on success, -1 if no such parameter */
/* To traverse the list, call pci_walk_params repeatedly, first with prev=NULL, and do not modify the parameters during traversal. */
struct pci_param *pci_walk_params(struct pci_access *acc, struct pci_param *prev) PCI_ABI;

/*
 *	Devices
 */

struct pci_dev {
  struct pci_dev *next;			/* Next device in the chain */
  u16 domain_16;			/* 16-bit version of the PCI domain for backward compatibility */
					/* 0xffff if the real domain doesn't fit in 16 bits */
  u8 bus, dev, func;			/* Bus inside domain, device and function */

  /* These fields are set by pci_fill_info() */
  unsigned int known_fields;		/* Set of info fields already known (see pci_fill_info()) */
  u16 vendor_id, device_id;		/* Identity of the device */
  u16 device_class;			/* PCI device class */
  int irq;				/* IRQ number */
  pciaddr_t base_addr[6];		/* Base addresses including flags in lower bits */
  pciaddr_t size[6];			/* Region sizes */
  pciaddr_t rom_base_addr;		/* Expansion ROM base address */
  pciaddr_t rom_size;			/* Expansion ROM size */
  struct pci_cap *first_cap;		/* List of capabilities */
  char *phy_slot;			/* Physical slot */
  char *module_alias;			/* Linux kernel module alias */
  char *label;				/* Device name as exported by BIOS */
  int numa_node;			/* NUMA node */
  pciaddr_t flags[6];			/* PCI_IORESOURCE_* flags for regions */
  pciaddr_t rom_flags;			/* PCI_IORESOURCE_* flags for expansion ROM */
  int domain;				/* PCI domain (host bridge) */

  /* Fields used internally */
  struct pci_access *access;
  struct pci_methods *methods;
  u8 *cache;				/* Cached config registers */
  int cache_len;
  int hdrtype;				/* Cached low 7 bits of header type, -1 if unknown */
  void *aux;				/* Auxiliary data for use by the back-end */
  struct pci_property *properties;	/* A linked list of extra properties */
  struct pci_cap *last_cap;		/* Last capability in the list */
};

#define PCI_ADDR_IO_MASK (~(pciaddr_t) 0x3)
#define PCI_ADDR_MEM_MASK (~(pciaddr_t) 0xf)
#define PCI_ADDR_FLAG_MASK 0xf

u8 pci_read_byte(struct pci_dev *, int pos) PCI_ABI; /* Access to configuration space */
u16 pci_read_word(struct pci_dev *, int pos) PCI_ABI;
u32 pci_read_long(struct pci_dev *, int pos) PCI_ABI;
int pci_read_block(struct pci_dev *, int pos, u8 *buf, int len) PCI_ABI;
int pci_read_vpd(struct pci_dev *d, int pos, u8 *buf, int len) PCI_ABI;
int pci_write_byte(struct pci_dev *, int pos, u8 data) PCI_ABI;
int pci_write_word(struct pci_dev *, int pos, u16 data) PCI_ABI;
int pci_write_long(struct pci_dev *, int pos, u32 data) PCI_ABI;
int pci_write_block(struct pci_dev *, int pos, u8 *buf, int len) PCI_ABI;

/*
 * Most device properties take some effort to obtain, so libpci does not
 * initialize them during default bus scan. Instead, you have to call
 * pci_fill_info() with the proper PCI_FILL_xxx constants OR'ed together.
 *
 * Some properties are stored directly in the pci_dev structure.
 * The remaining ones can be accessed through pci_get_string_property().
 *
 * pci_fill_info() returns the current value of pci_dev->known_fields.
 * This is a bit mask of all fields, which were already obtained during
 * the lifetime of the device. This includes fields which are not supported
 * by the particular device -- in that case, the field is left at its default
 * value, which is 0 for integer fields and NULL for pointers. On the other
 * hand, we never consider known fields unsupported by the current back-end;
 * such fields always contain the default value.
 *
 * XXX: flags and the result should be unsigned, but we do not want to break the ABI.
 */

int pci_fill_info(struct pci_dev *, int flags) PCI_ABI;
char *pci_get_string_property(struct pci_dev *d, u32 prop) PCI_ABI;

#define PCI_FILL_IDENT		0x0001
#define PCI_FILL_IRQ		0x0002
#define PCI_FILL_BASES		0x0004
#define PCI_FILL_ROM_BASE	0x0008
#define PCI_FILL_SIZES		0x0010
#define PCI_FILL_CLASS		0x0020
#define PCI_FILL_CAPS		0x0040
#define PCI_FILL_EXT_CAPS	0x0080
#define PCI_FILL_PHYS_SLOT	0x0100
#define PCI_FILL_MODULE_ALIAS	0x0200
#define PCI_FILL_LABEL		0x0400
#define PCI_FILL_NUMA_NODE	0x0800
#define PCI_FILL_IO_FLAGS	0x1000
#define PCI_FILL_DT_NODE	0x2000		/* Device tree node */
#define PCI_FILL_IOMMU_GROUP	0x4000
#define PCI_FILL_RESCAN		0x00010000

void pci_setup_cache(struct pci_dev *, u8 *cache, int len) PCI_ABI;

/*
 *	Capabilities
 */

struct pci_cap {
  struct pci_cap *next;
  u16 id;				/* PCI_CAP_ID_xxx */
  u16 type;				/* PCI_CAP_xxx */
  unsigned int addr;			/* Position in the config space */
};

#define PCI_CAP_NORMAL		1	/* Traditional PCI capabilities */
#define PCI_CAP_EXTENDED	2	/* PCIe extended capabilities */

struct pci_cap *pci_find_cap(struct pci_dev *, unsigned int id, unsigned int type) PCI_ABI;
struct pci_cap *pci_find_cap_nr(struct pci_dev *, unsigned int id, unsigned int type,
                                unsigned int *cap_number) PCI_ABI;

/*
 *	Filters
 */

struct pci_filter {
  int domain, bus, slot, func;			/* -1 = ANY */
  int vendor, device, device_class;
  int rfu[3];
};

void pci_filter_init(struct pci_access *, struct pci_filter *) PCI_ABI;
char *pci_filter_parse_slot(struct pci_filter *, char *) PCI_ABI;
char *pci_filter_parse_id(struct pci_filter *, char *) PCI_ABI;
int pci_filter_match(struct pci_filter *, struct pci_dev *) PCI_ABI;

/*
 *	Conversion of PCI ID's to names (according to the pci.ids file)
 *
 *	Call pci_lookup_name() to identify different types of ID's:
 *
 *	VENDOR				(vendorID) -> vendor
 *	DEVICE				(vendorID, deviceID) -> device
 *	VENDOR | DEVICE			(vendorID, deviceID) -> combined vendor and device
 *	SUBSYSTEM | VENDOR		(subvendorID) -> subsystem vendor
 *	SUBSYSTEM | DEVICE		(vendorID, deviceID, subvendorID, subdevID) -> subsystem device
 *	SUBSYSTEM | VENDOR | DEVICE	(vendorID, deviceID, subvendorID, subdevID) -> combined subsystem v+d
 *	SUBSYSTEM | ...			(-1, -1, subvendorID, subdevID) -> generic subsystem
 *	CLASS				(classID) -> class
 *	PROGIF				(classID, progif) -> programming interface
 */

char *pci_lookup_name(struct pci_access *a, char *buf, int size, int flags, ...) PCI_ABI;

int pci_load_name_list(struct pci_access *a) PCI_ABI;	/* Called automatically by pci_lookup_*() when needed; returns success */
void pci_free_name_list(struct pci_access *a) PCI_ABI;	/* Called automatically by pci_cleanup() */
void pci_set_name_list_path(struct pci_access *a, char *name, int to_be_freed) PCI_ABI;
void pci_id_cache_flush(struct pci_access *a) PCI_ABI;

enum pci_lookup_mode {
  PCI_LOOKUP_VENDOR = 1,		/* Vendor name (args: vendorID) */
  PCI_LOOKUP_DEVICE = 2,		/* Device name (args: vendorID, deviceID) */
  PCI_LOOKUP_CLASS = 4,			/* Device class (args: classID) */
  PCI_LOOKUP_SUBSYSTEM = 8,
  PCI_LOOKUP_PROGIF = 16,		/* Programming interface (args: classID, prog_if) */
  PCI_LOOKUP_NUMERIC = 0x10000,		/* Want only formatted numbers; default if access->numeric_ids is set */
  PCI_LOOKUP_NO_NUMBERS = 0x20000,	/* Return NULL if not found in the database; default is to print numerically */
  PCI_LOOKUP_MIXED = 0x40000,		/* Include both numbers and names */
  PCI_LOOKUP_NETWORK = 0x80000,		/* Try to resolve unknown ID's by DNS */
  PCI_LOOKUP_SKIP_LOCAL = 0x100000,	/* Do not consult local database */
  PCI_LOOKUP_CACHE = 0x200000,		/* Consult the local cache before using DNS */
  PCI_LOOKUP_REFRESH_CACHE = 0x400000,	/* Forget all previously cached entries, but still allow updating the cache */
  PCI_LOOKUP_NO_HWDB = 0x800000,	/* Do not ask udev's hwdb */
};

#endif
