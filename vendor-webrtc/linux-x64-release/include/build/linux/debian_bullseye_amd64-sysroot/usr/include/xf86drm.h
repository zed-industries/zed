/**
 * \file xf86drm.h 
 * OS-independent header for DRM user-level library interface.
 *
 * \author Rickard E. (Rik) Faith <faith@valinux.com>
 */
 
/*
 * Copyright 1999, 2000 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * PRECISION INSIGHT AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 */

#ifndef _XF86DRM_H_
#define _XF86DRM_H_

#include <stdarg.h>
#include <sys/types.h>
#include <stdint.h>
#include <drm.h>

#if defined(__cplusplus)
extern "C" {
#endif

#ifndef DRM_MAX_MINOR
#define DRM_MAX_MINOR   16
#endif

#if defined(__linux__)

#define DRM_IOCTL_NR(n)		_IOC_NR(n)
#define DRM_IOC_VOID		_IOC_NONE
#define DRM_IOC_READ		_IOC_READ
#define DRM_IOC_WRITE		_IOC_WRITE
#define DRM_IOC_READWRITE	_IOC_READ|_IOC_WRITE
#define DRM_IOC(dir, group, nr, size) _IOC(dir, group, nr, size)

#else /* One of the *BSDs */

#include <sys/ioccom.h>
#define DRM_IOCTL_NR(n)         ((n) & 0xff)
#define DRM_IOC_VOID            IOC_VOID
#define DRM_IOC_READ            IOC_OUT
#define DRM_IOC_WRITE           IOC_IN
#define DRM_IOC_READWRITE       IOC_INOUT
#define DRM_IOC(dir, group, nr, size) _IOC(dir, group, nr, size)

#endif

				/* Defaults, if nothing set in xf86config */
#define DRM_DEV_UID	 0
#define DRM_DEV_GID	 0
/* Default /dev/dri directory permissions 0755 */
#define DRM_DEV_DIRMODE	 	\
	(S_IRUSR|S_IWUSR|S_IXUSR|S_IRGRP|S_IXGRP|S_IROTH|S_IXOTH)
#define DRM_DEV_MODE	 (S_IRUSR|S_IWUSR|S_IRGRP|S_IWGRP|S_IROTH|S_IWOTH)

#ifdef __OpenBSD__
#define DRM_DIR_NAME  "/dev"
#define DRM_PRIMARY_MINOR_NAME  "drm"
#define DRM_CONTROL_MINOR_NAME  "drmC"
#define DRM_RENDER_MINOR_NAME   "drmR"
#else
#define DRM_DIR_NAME  "/dev/dri"
#define DRM_PRIMARY_MINOR_NAME  "card"
#define DRM_CONTROL_MINOR_NAME  "controlD"
#define DRM_RENDER_MINOR_NAME   "renderD"
#define DRM_PROC_NAME "/proc/dri/" /* For backward Linux compatibility */
#endif

#define DRM_DEV_NAME          "%s/" DRM_PRIMARY_MINOR_NAME "%d"
#define DRM_CONTROL_DEV_NAME  "%s/" DRM_CONTROL_MINOR_NAME "%d"
#define DRM_RENDER_DEV_NAME   "%s/" DRM_RENDER_MINOR_NAME  "%d"

#define DRM_NODE_NAME_MAX \
    (sizeof(DRM_DIR_NAME) + 1 /* slash */ \
     + MAX3(sizeof(DRM_PRIMARY_MINOR_NAME), \
            sizeof(DRM_CONTROL_MINOR_NAME), \
            sizeof(DRM_RENDER_MINOR_NAME)) \
     + sizeof("144") /* highest possible node number */ \
     + 1) /* NULL-terminator */

#define DRM_ERR_NO_DEVICE  (-1001)
#define DRM_ERR_NO_ACCESS  (-1002)
#define DRM_ERR_NOT_ROOT   (-1003)
#define DRM_ERR_INVALID    (-1004)
#define DRM_ERR_NO_FD      (-1005)

#define DRM_AGP_NO_HANDLE 0

typedef unsigned int  drmSize,     *drmSizePtr;	    /**< For mapped regions */
typedef void          *drmAddress, **drmAddressPtr; /**< For mapped regions */

#if (__GNUC__ >= 3)
#define DRM_PRINTFLIKE(f, a) __attribute__ ((format(__printf__, f, a)))
#else
#define DRM_PRINTFLIKE(f, a)
#endif

typedef struct _drmServerInfo {
  int (*debug_print)(const char *format, va_list ap) DRM_PRINTFLIKE(1,0);
  int (*load_module)(const char *name);
  void (*get_perms)(gid_t *, mode_t *);
} drmServerInfo, *drmServerInfoPtr;

typedef struct drmHashEntry {
    int      fd;
    void     (*f)(int, void *, void *);
    void     *tagTable;
} drmHashEntry;

extern int drmIoctl(int fd, unsigned long request, void *arg);
extern void *drmGetHashTable(void);
extern drmHashEntry *drmGetEntry(int fd);

/**
 * Driver version information.
 *
 * \sa drmGetVersion() and drmSetVersion().
 */
typedef struct _drmVersion {
    int     version_major;        /**< Major version */
    int     version_minor;        /**< Minor version */
    int     version_patchlevel;   /**< Patch level */
    int     name_len; 	          /**< Length of name buffer */
    char    *name;	          /**< Name of driver */
    int     date_len;             /**< Length of date buffer */
    char    *date;                /**< User-space buffer to hold date */
    int     desc_len;	          /**< Length of desc buffer */
    char    *desc;                /**< User-space buffer to hold desc */
} drmVersion, *drmVersionPtr;

typedef struct _drmStats {
    unsigned long count;	     /**< Number of data */
    struct {
	unsigned long value;	     /**< Value from kernel */
	const char    *long_format;  /**< Suggested format for long_name */
	const char    *long_name;    /**< Long name for value */
	const char    *rate_format;  /**< Suggested format for rate_name */
	const char    *rate_name;    /**< Short name for value per second */
	int           isvalue;       /**< True if value (vs. counter) */
	const char    *mult_names;   /**< Multiplier names (e.g., "KGM") */
	int           mult;          /**< Multiplier value (e.g., 1024) */
	int           verbose;       /**< Suggest only in verbose output */
    } data[15];
} drmStatsT;


				/* All of these enums *MUST* match with the
                                   kernel implementation -- so do *NOT*
                                   change them!  (The drmlib implementation
                                   will just copy the flags instead of
                                   translating them.) */
typedef enum {
    DRM_FRAME_BUFFER    = 0,      /**< WC, no caching, no core dump */
    DRM_REGISTERS       = 1,      /**< no caching, no core dump */
    DRM_SHM             = 2,      /**< shared, cached */
    DRM_AGP             = 3,	  /**< AGP/GART */
    DRM_SCATTER_GATHER  = 4,	  /**< PCI scatter/gather */
    DRM_CONSISTENT      = 5	  /**< PCI consistent */
} drmMapType;

typedef enum {
    DRM_RESTRICTED      = 0x0001, /**< Cannot be mapped to client-virtual */
    DRM_READ_ONLY       = 0x0002, /**< Read-only in client-virtual */
    DRM_LOCKED          = 0x0004, /**< Physical pages locked */
    DRM_KERNEL          = 0x0008, /**< Kernel requires access */
    DRM_WRITE_COMBINING = 0x0010, /**< Use write-combining, if available */
    DRM_CONTAINS_LOCK   = 0x0020, /**< SHM page that contains lock */
    DRM_REMOVABLE	= 0x0040  /**< Removable mapping */
} drmMapFlags;

/**
 * \warning These values *MUST* match drm.h
 */
typedef enum {
    /** \name Flags for DMA buffer dispatch */
    /*@{*/
    DRM_DMA_BLOCK        = 0x01, /**< 
				  * Block until buffer dispatched.
				  * 
				  * \note the buffer may not yet have been
				  * processed by the hardware -- getting a
				  * hardware lock with the hardware quiescent
				  * will ensure that the buffer has been
				  * processed.
				  */
    DRM_DMA_WHILE_LOCKED = 0x02, /**< Dispatch while lock held */
    DRM_DMA_PRIORITY     = 0x04, /**< High priority dispatch */
    /*@}*/

    /** \name Flags for DMA buffer request */
    /*@{*/
    DRM_DMA_WAIT         = 0x10, /**< Wait for free buffers */
    DRM_DMA_SMALLER_OK   = 0x20, /**< Smaller-than-requested buffers OK */
    DRM_DMA_LARGER_OK    = 0x40  /**< Larger-than-requested buffers OK */
    /*@}*/
} drmDMAFlags;

typedef enum {
    DRM_PAGE_ALIGN       = 0x01,
    DRM_AGP_BUFFER       = 0x02,
    DRM_SG_BUFFER        = 0x04,
    DRM_FB_BUFFER        = 0x08,
    DRM_PCI_BUFFER_RO    = 0x10
} drmBufDescFlags;

typedef enum {
    DRM_LOCK_READY      = 0x01, /**< Wait until hardware is ready for DMA */
    DRM_LOCK_QUIESCENT  = 0x02, /**< Wait until hardware quiescent */
    DRM_LOCK_FLUSH      = 0x04, /**< Flush this context's DMA queue first */
    DRM_LOCK_FLUSH_ALL  = 0x08, /**< Flush all DMA queues first */
				/* These *HALT* flags aren't supported yet
                                   -- they will be used to support the
                                   full-screen DGA-like mode. */
    DRM_HALT_ALL_QUEUES = 0x10, /**< Halt all current and future queues */
    DRM_HALT_CUR_QUEUES = 0x20  /**< Halt all current queues */
} drmLockFlags;

typedef enum {
    DRM_CONTEXT_PRESERVED = 0x01, /**< This context is preserved and
				     never swapped. */
    DRM_CONTEXT_2DONLY    = 0x02  /**< This context is for 2D rendering only. */
} drm_context_tFlags, *drm_context_tFlagsPtr;

typedef struct _drmBufDesc {
    int              count;	  /**< Number of buffers of this size */
    int              size;	  /**< Size in bytes */
    int              low_mark;	  /**< Low water mark */
    int              high_mark;	  /**< High water mark */
} drmBufDesc, *drmBufDescPtr;

typedef struct _drmBufInfo {
    int              count;	  /**< Number of buffers described in list */
    drmBufDescPtr    list;	  /**< List of buffer descriptions */
} drmBufInfo, *drmBufInfoPtr;

typedef struct _drmBuf {
    int              idx;	  /**< Index into the master buffer list */
    int              total;	  /**< Buffer size */
    int              used;	  /**< Amount of buffer in use (for DMA) */
    drmAddress       address;	  /**< Address */
} drmBuf, *drmBufPtr;

/**
 * Buffer mapping information.
 *
 * Used by drmMapBufs() and drmUnmapBufs() to store information about the
 * mapped buffers.
 */
typedef struct _drmBufMap {
    int              count;	  /**< Number of buffers mapped */
    drmBufPtr        list;	  /**< Buffers */
} drmBufMap, *drmBufMapPtr;

typedef struct _drmLock {
    volatile unsigned int lock;
    char                      padding[60];
    /* This is big enough for most current (and future?) architectures:
       DEC Alpha:              32 bytes
       Intel Merced:           ?
       Intel P5/PPro/PII/PIII: 32 bytes
       Intel StrongARM:        32 bytes
       Intel i386/i486:        16 bytes
       MIPS:                   32 bytes (?)
       Motorola 68k:           16 bytes
       Motorola PowerPC:       32 bytes
       Sun SPARC:              32 bytes
    */
} drmLock, *drmLockPtr;

/**
 * Indices here refer to the offset into
 * list in drmBufInfo
 */
typedef struct _drmDMAReq {
    drm_context_t    context;  	  /**< Context handle */
    int           send_count;     /**< Number of buffers to send */
    int           *send_list;     /**< List of handles to buffers */
    int           *send_sizes;    /**< Lengths of data to send, in bytes */
    drmDMAFlags   flags;          /**< Flags */
    int           request_count;  /**< Number of buffers requested */
    int           request_size;	  /**< Desired size of buffers requested */
    int           *request_list;  /**< Buffer information */
    int           *request_sizes; /**< Minimum acceptable sizes */
    int           granted_count;  /**< Number of buffers granted at this size */
} drmDMAReq, *drmDMAReqPtr;

typedef struct _drmRegion {
    drm_handle_t     handle;
    unsigned int  offset;
    drmSize       size;
    drmAddress    map;
} drmRegion, *drmRegionPtr;

typedef struct _drmTextureRegion {
    unsigned char next;
    unsigned char prev;
    unsigned char in_use;
    unsigned char padding;	/**< Explicitly pad this out */
    unsigned int  age;
} drmTextureRegion, *drmTextureRegionPtr;


typedef enum {
    DRM_VBLANK_ABSOLUTE = 0x0,	/**< Wait for specific vblank sequence number */
    DRM_VBLANK_RELATIVE = 0x1,	/**< Wait for given number of vblanks */
    /* bits 1-6 are reserved for high crtcs */
    DRM_VBLANK_HIGH_CRTC_MASK = 0x0000003e,
    DRM_VBLANK_EVENT = 0x4000000,	/**< Send event instead of blocking */
    DRM_VBLANK_FLIP = 0x8000000,	/**< Scheduled buffer swap should flip */
    DRM_VBLANK_NEXTONMISS = 0x10000000,	/**< If missed, wait for next vblank */
    DRM_VBLANK_SECONDARY = 0x20000000,	/**< Secondary display controller */
    DRM_VBLANK_SIGNAL   = 0x40000000	/* Send signal instead of blocking */
} drmVBlankSeqType;
#define DRM_VBLANK_HIGH_CRTC_SHIFT 1

typedef struct _drmVBlankReq {
	drmVBlankSeqType type;
	unsigned int sequence;
	unsigned long signal;
} drmVBlankReq, *drmVBlankReqPtr;

typedef struct _drmVBlankReply {
	drmVBlankSeqType type;
	unsigned int sequence;
	long tval_sec;
	long tval_usec;
} drmVBlankReply, *drmVBlankReplyPtr;

typedef union _drmVBlank {
	drmVBlankReq request;
	drmVBlankReply reply;
} drmVBlank, *drmVBlankPtr;

typedef struct _drmSetVersion {
	int drm_di_major;
	int drm_di_minor;
	int drm_dd_major;
	int drm_dd_minor;
} drmSetVersion, *drmSetVersionPtr;

#define __drm_dummy_lock(lock) (*(__volatile__ unsigned int *)lock)

#define DRM_LOCK_HELD  0x80000000U /**< Hardware lock is held */
#define DRM_LOCK_CONT  0x40000000U /**< Hardware lock is contended */

#if defined(__GNUC__) && (__GNUC__ >= 2)
# if defined(__i386) || defined(__AMD64__) || defined(__x86_64__) || defined(__amd64__)
				/* Reflect changes here to drmP.h */
#define DRM_CAS(lock,old,new,__ret)                                    \
	do {                                                           \
                int __dummy;	/* Can't mark eax as clobbered */      \
		__asm__ __volatile__(                                  \
			"lock ; cmpxchg %4,%1\n\t"                     \
                        "setnz %0"                                     \
			: "=d" (__ret),                                \
   			  "=m" (__drm_dummy_lock(lock)),               \
                          "=a" (__dummy)                               \
			: "2" (old),                                   \
			  "r" (new));                                  \
	} while (0)

#elif defined(__alpha__)

#define	DRM_CAS(lock, old, new, ret)		\
	do {					\
		int tmp, old32;			\
		__asm__ __volatile__(		\
		"	addl	$31, %5, %3\n"	\
		"1:	ldl_l	%0, %2\n"	\
		"	cmpeq	%0, %3, %1\n"	\
		"	beq	%1, 2f\n"	\
		"	mov	%4, %0\n"	\
		"	stl_c	%0, %2\n"	\
		"	beq	%0, 3f\n"	\
		"	mb\n"			\
		"2:	cmpeq	%1, 0, %1\n"	\
		".subsection 2\n"		\
		"3:	br	1b\n"		\
		".previous"			\
		: "=&r"(tmp), "=&r"(ret),	\
		  "=m"(__drm_dummy_lock(lock)),	\
		  "=&r"(old32)			\
		: "r"(new), "r"(old)		\
		: "memory");			\
	} while (0)

#elif defined(__sparc__)

#define DRM_CAS(lock,old,new,__ret)				\
do {	register unsigned int __old __asm("o0");		\
	register unsigned int __new __asm("o1");		\
	register volatile unsigned int *__lock __asm("o2");	\
	__old = old;						\
	__new = new;						\
	__lock = (volatile unsigned int *)lock;			\
	__asm__ __volatile__(					\
		/*"cas [%2], %3, %0"*/				\
		".word 0xd3e29008\n\t"				\
		/*"membar #StoreStore | #StoreLoad"*/		\
		".word 0x8143e00a"				\
		: "=&r" (__new)					\
		: "0" (__new),					\
		  "r" (__lock),					\
		  "r" (__old)					\
		: "memory");					\
	__ret = (__new != __old);				\
} while(0)

#elif defined(__ia64__)

#ifdef __INTEL_COMPILER
/* this currently generates bad code (missing stop bits)... */
#include <ia64intrin.h>

#define DRM_CAS(lock,old,new,__ret)					      \
	do {								      \
		unsigned long __result, __old = (old) & 0xffffffff;		\
		__mf();							      	\
		__result = _InterlockedCompareExchange_acq(&__drm_dummy_lock(lock), (new), __old);\
		__ret = (__result) != (__old);					\
/*		__ret = (__sync_val_compare_and_swap(&__drm_dummy_lock(lock), \
						     (old), (new))	      \
			 != (old));					      */\
	} while (0)

#else
#define DRM_CAS(lock,old,new,__ret)					  \
	do {								  \
		unsigned int __result, __old = (old);			  \
		__asm__ __volatile__(					  \
			"mf\n"						  \
			"mov ar.ccv=%2\n"				  \
			";;\n"						  \
			"cmpxchg4.acq %0=%1,%3,ar.ccv"			  \
			: "=r" (__result), "=m" (__drm_dummy_lock(lock))  \
			: "r" ((unsigned long)__old), "r" (new)			  \
			: "memory");					  \
		__ret = (__result) != (__old);				  \
	} while (0)

#endif

#elif defined(__powerpc__)

#define DRM_CAS(lock,old,new,__ret)			\
	do {						\
		__asm__ __volatile__(			\
			"sync;"				\
			"0:    lwarx %0,0,%1;"		\
			"      xor. %0,%3,%0;"		\
			"      bne 1f;"			\
			"      stwcx. %2,0,%1;"		\
			"      bne- 0b;"		\
			"1:    "			\
			"sync;"				\
		: "=&r"(__ret)				\
		: "r"(lock), "r"(new), "r"(old)		\
		: "cr0", "memory");			\
	} while (0)

# elif defined (__ARM_ARCH_6__) || defined(__ARM_ARCH_6J__) \
	|| defined (__ARM_ARCH_6Z__) || defined(__ARM_ARCH_6ZK__) \
	|| defined (__ARM_ARCH_6K__) || defined(__ARM_ARCH_6T2__) \
	|| defined (__ARM_ARCH_7__) || defined(__ARM_ARCH_7A__) \
	|| defined(__ARM_ARCH_7R__) || defined(__ARM_ARCH_7M__) \
	|| defined(__ARM_ARCH_7EM__)
       /* excluding ARMv4/ARMv5 and lower (lacking ldrex/strex support) */
       #undef DRM_DEV_MODE
       #define DRM_DEV_MODE     (S_IRUSR|S_IWUSR|S_IRGRP|S_IWGRP|S_IROTH|S_IWOTH)

       #define DRM_CAS(lock,old,new,__ret)             \
       do {                                            \
               __asm__ __volatile__ (                  \
                       "1: ldrex %0, [%1]\n"           \
                       "   teq %0, %2\n"               \
                       "   ite eq\n"                   \
                       "   strexeq %0, %3, [%1]\n"     \
                       "   movne   %0, #1\n"           \
               : "=&r" (__ret)                         \
               : "r" (lock), "r" (old), "r" (new)      \
               : "cc","memory");                       \
       } while (0)

#endif /* architecture */
#endif /* __GNUC__ >= 2 */

#ifndef DRM_CAS
#define DRM_CAS(lock,old,new,ret) do { ret=1; } while (0) /* FAST LOCK FAILS */
#endif

#if defined(__alpha__)
#define DRM_CAS_RESULT(_result)		long _result
#elif defined(__powerpc__)
#define DRM_CAS_RESULT(_result)		int _result
#else
#define DRM_CAS_RESULT(_result)		char _result
#endif

#define DRM_LIGHT_LOCK(fd,lock,context)                                \
	do {                                                           \
                DRM_CAS_RESULT(__ret);                                 \
		DRM_CAS(lock,context,DRM_LOCK_HELD|context,__ret);     \
                if (__ret) drmGetLock(fd,context,0);                   \
        } while(0)

				/* This one counts fast locks -- for
                                   benchmarking only. */
#define DRM_LIGHT_LOCK_COUNT(fd,lock,context,count)                    \
	do {                                                           \
                DRM_CAS_RESULT(__ret);                                 \
		DRM_CAS(lock,context,DRM_LOCK_HELD|context,__ret);     \
                if (__ret) drmGetLock(fd,context,0);                   \
                else       ++count;                                    \
        } while(0)

#define DRM_LOCK(fd,lock,context,flags)                                \
	do {                                                           \
		if (flags) drmGetLock(fd,context,flags);               \
		else       DRM_LIGHT_LOCK(fd,lock,context);            \
	} while(0)

#define DRM_UNLOCK(fd,lock,context)                                    \
	do {                                                           \
                DRM_CAS_RESULT(__ret);                                 \
		DRM_CAS(lock,DRM_LOCK_HELD|context,context,__ret);     \
                if (__ret) drmUnlock(fd,context);                      \
        } while(0)

				/* Simple spin locks */
#define DRM_SPINLOCK(spin,val)                                         \
	do {                                                           \
            DRM_CAS_RESULT(__ret);                                     \
	    do {                                                       \
		DRM_CAS(spin,0,val,__ret);                             \
		if (__ret) while ((spin)->lock);                       \
	    } while (__ret);                                           \
	} while(0)

#define DRM_SPINLOCK_TAKE(spin,val)                                    \
	do {                                                           \
            DRM_CAS_RESULT(__ret);                                     \
            int  cur;                                                  \
	    do {                                                       \
                cur = (*spin).lock;                                    \
		DRM_CAS(spin,cur,val,__ret);                           \
	    } while (__ret);                                           \
	} while(0)

#define DRM_SPINLOCK_COUNT(spin,val,count,__ret)                       \
	do {                                                           \
            int  __i;                                                  \
            __ret = 1;                                                 \
            for (__i = 0; __ret && __i < count; __i++) {               \
		DRM_CAS(spin,0,val,__ret);                             \
		if (__ret) for (;__i < count && (spin)->lock; __i++);  \
	    }                                                          \
	} while(0)

#define DRM_SPINUNLOCK(spin,val)                                       \
	do {                                                           \
            DRM_CAS_RESULT(__ret);                                     \
            if ((*spin).lock == val) { /* else server stole lock */    \
	        do {                                                   \
		    DRM_CAS(spin,val,0,__ret);                         \
	        } while (__ret);                                       \
            }                                                          \
	} while(0)



/* General user-level programmer's API: unprivileged */
extern int           drmAvailable(void);
extern int           drmOpen(const char *name, const char *busid);

#define DRM_NODE_PRIMARY 0
#define DRM_NODE_CONTROL 1
#define DRM_NODE_RENDER  2
#define DRM_NODE_MAX     3

extern int           drmOpenWithType(const char *name, const char *busid,
                                     int type);

extern int           drmOpenControl(int minor);
extern int           drmOpenRender(int minor);
extern int           drmClose(int fd);
extern drmVersionPtr drmGetVersion(int fd);
extern drmVersionPtr drmGetLibVersion(int fd);
extern int           drmGetCap(int fd, uint64_t capability, uint64_t *value);
extern void          drmFreeVersion(drmVersionPtr);
extern int           drmGetMagic(int fd, drm_magic_t * magic);
extern char          *drmGetBusid(int fd);
extern int           drmGetInterruptFromBusID(int fd, int busnum, int devnum,
					      int funcnum);
extern int           drmGetMap(int fd, int idx, drm_handle_t *offset,
			       drmSize *size, drmMapType *type,
			       drmMapFlags *flags, drm_handle_t *handle,
			       int *mtrr);
extern int           drmGetClient(int fd, int idx, int *auth, int *pid,
				  int *uid, unsigned long *magic,
				  unsigned long *iocs);
extern int           drmGetStats(int fd, drmStatsT *stats);
extern int           drmSetInterfaceVersion(int fd, drmSetVersion *version);
extern int           drmCommandNone(int fd, unsigned long drmCommandIndex);
extern int           drmCommandRead(int fd, unsigned long drmCommandIndex,
                                    void *data, unsigned long size);
extern int           drmCommandWrite(int fd, unsigned long drmCommandIndex,
                                     void *data, unsigned long size);
extern int           drmCommandWriteRead(int fd, unsigned long drmCommandIndex,
                                         void *data, unsigned long size);

/* General user-level programmer's API: X server (root) only  */
extern void          drmFreeBusid(const char *busid);
extern int           drmSetBusid(int fd, const char *busid);
extern int           drmAuthMagic(int fd, drm_magic_t magic);
extern int           drmAddMap(int fd,
			       drm_handle_t offset,
			       drmSize size,
			       drmMapType type,
			       drmMapFlags flags,
			       drm_handle_t * handle);
extern int	     drmRmMap(int fd, drm_handle_t handle);
extern int	     drmAddContextPrivateMapping(int fd, drm_context_t ctx_id,
						 drm_handle_t handle);

extern int           drmAddBufs(int fd, int count, int size,
				drmBufDescFlags flags,
				int agp_offset);
extern int           drmMarkBufs(int fd, double low, double high);
extern int           drmCreateContext(int fd, drm_context_t * handle);
extern int           drmSetContextFlags(int fd, drm_context_t context,
					drm_context_tFlags flags);
extern int           drmGetContextFlags(int fd, drm_context_t context,
					drm_context_tFlagsPtr flags);
extern int           drmAddContextTag(int fd, drm_context_t context, void *tag);
extern int           drmDelContextTag(int fd, drm_context_t context);
extern void          *drmGetContextTag(int fd, drm_context_t context);
extern drm_context_t * drmGetReservedContextList(int fd, int *count);
extern void          drmFreeReservedContextList(drm_context_t *);
extern int           drmSwitchToContext(int fd, drm_context_t context);
extern int           drmDestroyContext(int fd, drm_context_t handle);
extern int           drmCreateDrawable(int fd, drm_drawable_t * handle);
extern int           drmDestroyDrawable(int fd, drm_drawable_t handle);
extern int           drmUpdateDrawableInfo(int fd, drm_drawable_t handle,
					   drm_drawable_info_type_t type,
					   unsigned int num, void *data);
extern int           drmCtlInstHandler(int fd, int irq);
extern int           drmCtlUninstHandler(int fd);
extern int           drmSetClientCap(int fd, uint64_t capability,
				     uint64_t value);

extern int           drmCrtcGetSequence(int fd, uint32_t crtcId,
					uint64_t *sequence, uint64_t *ns);
extern int           drmCrtcQueueSequence(int fd, uint32_t crtcId,
					  uint32_t flags, uint64_t sequence,
					  uint64_t *sequence_queued,
					  uint64_t user_data);
/* General user-level programmer's API: authenticated client and/or X */
extern int           drmMap(int fd,
			    drm_handle_t handle,
			    drmSize size,
			    drmAddressPtr address);
extern int           drmUnmap(drmAddress address, drmSize size);
extern drmBufInfoPtr drmGetBufInfo(int fd);
extern drmBufMapPtr  drmMapBufs(int fd);
extern int           drmUnmapBufs(drmBufMapPtr bufs);
extern int           drmDMA(int fd, drmDMAReqPtr request);
extern int           drmFreeBufs(int fd, int count, int *list);
extern int           drmGetLock(int fd,
			        drm_context_t context,
			        drmLockFlags flags);
extern int           drmUnlock(int fd, drm_context_t context);
extern int           drmFinish(int fd, int context, drmLockFlags flags);
extern int	     drmGetContextPrivateMapping(int fd, drm_context_t ctx_id, 
						 drm_handle_t * handle);

/* AGP/GART support: X server (root) only */
extern int           drmAgpAcquire(int fd);
extern int           drmAgpRelease(int fd);
extern int           drmAgpEnable(int fd, unsigned long mode);
extern int           drmAgpAlloc(int fd, unsigned long size,
				 unsigned long type, unsigned long *address,
				 drm_handle_t *handle);
extern int           drmAgpFree(int fd, drm_handle_t handle);
extern int 	     drmAgpBind(int fd, drm_handle_t handle,
				unsigned long offset);
extern int           drmAgpUnbind(int fd, drm_handle_t handle);

/* AGP/GART info: authenticated client and/or X */
extern int           drmAgpVersionMajor(int fd);
extern int           drmAgpVersionMinor(int fd);
extern unsigned long drmAgpGetMode(int fd);
extern unsigned long drmAgpBase(int fd); /* Physical location */
extern unsigned long drmAgpSize(int fd); /* Bytes */
extern unsigned long drmAgpMemoryUsed(int fd);
extern unsigned long drmAgpMemoryAvail(int fd);
extern unsigned int  drmAgpVendorId(int fd);
extern unsigned int  drmAgpDeviceId(int fd);

/* PCI scatter/gather support: X server (root) only */
extern int           drmScatterGatherAlloc(int fd, unsigned long size,
					   drm_handle_t *handle);
extern int           drmScatterGatherFree(int fd, drm_handle_t handle);

extern int           drmWaitVBlank(int fd, drmVBlankPtr vbl);

/* Support routines */
extern void          drmSetServerInfo(drmServerInfoPtr info);
extern int           drmError(int err, const char *label);
extern void          *drmMalloc(int size);
extern void          drmFree(void *pt);

/* Hash table routines */
extern void *drmHashCreate(void);
extern int  drmHashDestroy(void *t);
extern int  drmHashLookup(void *t, unsigned long key, void **value);
extern int  drmHashInsert(void *t, unsigned long key, void *value);
extern int  drmHashDelete(void *t, unsigned long key);
extern int  drmHashFirst(void *t, unsigned long *key, void **value);
extern int  drmHashNext(void *t, unsigned long *key, void **value);

/* PRNG routines */
extern void          *drmRandomCreate(unsigned long seed);
extern int           drmRandomDestroy(void *state);
extern unsigned long drmRandom(void *state);
extern double        drmRandomDouble(void *state);

/* Skip list routines */

extern void *drmSLCreate(void);
extern int  drmSLDestroy(void *l);
extern int  drmSLLookup(void *l, unsigned long key, void **value);
extern int  drmSLInsert(void *l, unsigned long key, void *value);
extern int  drmSLDelete(void *l, unsigned long key);
extern int  drmSLNext(void *l, unsigned long *key, void **value);
extern int  drmSLFirst(void *l, unsigned long *key, void **value);
extern void drmSLDump(void *l);
extern int  drmSLLookupNeighbors(void *l, unsigned long key,
				 unsigned long *prev_key, void **prev_value,
				 unsigned long *next_key, void **next_value);

extern int drmOpenOnce(void *unused, const char *BusID, int *newlyopened);
extern int drmOpenOnceWithType(const char *BusID, int *newlyopened, int type);
extern void drmCloseOnce(int fd);
extern void drmMsg(const char *format, ...) DRM_PRINTFLIKE(1, 2);

extern int drmSetMaster(int fd);
extern int drmDropMaster(int fd);
extern int drmIsMaster(int fd);

#define DRM_EVENT_CONTEXT_VERSION 4

typedef struct _drmEventContext {

	/* This struct is versioned so we can add more pointers if we
	 * add more events. */
	int version;

	void (*vblank_handler)(int fd,
			       unsigned int sequence, 
			       unsigned int tv_sec,
			       unsigned int tv_usec,
			       void *user_data);

	void (*page_flip_handler)(int fd,
				  unsigned int sequence,
				  unsigned int tv_sec,
				  unsigned int tv_usec,
				  void *user_data);

	void (*page_flip_handler2)(int fd,
				   unsigned int sequence,
				   unsigned int tv_sec,
				   unsigned int tv_usec,
				   unsigned int crtc_id,
				   void *user_data);

	void (*sequence_handler)(int fd,
				 uint64_t sequence,
				 uint64_t ns,
				 uint64_t user_data);
} drmEventContext, *drmEventContextPtr;

extern int drmHandleEvent(int fd, drmEventContextPtr evctx);

extern char *drmGetDeviceNameFromFd(int fd);

/* Improved version of drmGetDeviceNameFromFd which attributes for any type of
 * device/node - card, control or renderD.
 */
extern char *drmGetDeviceNameFromFd2(int fd);
extern int drmGetNodeTypeFromFd(int fd);

extern int drmPrimeHandleToFD(int fd, uint32_t handle, uint32_t flags, int *prime_fd);
extern int drmPrimeFDToHandle(int fd, int prime_fd, uint32_t *handle);

extern char *drmGetPrimaryDeviceNameFromFd(int fd);
extern char *drmGetRenderDeviceNameFromFd(int fd);

#define DRM_BUS_PCI       0
#define DRM_BUS_USB       1
#define DRM_BUS_PLATFORM  2
#define DRM_BUS_HOST1X    3

typedef struct _drmPciBusInfo {
    uint16_t domain;
    uint8_t bus;
    uint8_t dev;
    uint8_t func;
} drmPciBusInfo, *drmPciBusInfoPtr;

typedef struct _drmPciDeviceInfo {
    uint16_t vendor_id;
    uint16_t device_id;
    uint16_t subvendor_id;
    uint16_t subdevice_id;
    uint8_t revision_id;
} drmPciDeviceInfo, *drmPciDeviceInfoPtr;

typedef struct _drmUsbBusInfo {
    uint8_t bus;
    uint8_t dev;
} drmUsbBusInfo, *drmUsbBusInfoPtr;

typedef struct _drmUsbDeviceInfo {
    uint16_t vendor;
    uint16_t product;
} drmUsbDeviceInfo, *drmUsbDeviceInfoPtr;

#define DRM_PLATFORM_DEVICE_NAME_LEN 512

typedef struct _drmPlatformBusInfo {
    char fullname[DRM_PLATFORM_DEVICE_NAME_LEN];
} drmPlatformBusInfo, *drmPlatformBusInfoPtr;

typedef struct _drmPlatformDeviceInfo {
    char **compatible; /* NULL terminated list of compatible strings */
} drmPlatformDeviceInfo, *drmPlatformDeviceInfoPtr;

#define DRM_HOST1X_DEVICE_NAME_LEN 512

typedef struct _drmHost1xBusInfo {
    char fullname[DRM_HOST1X_DEVICE_NAME_LEN];
} drmHost1xBusInfo, *drmHost1xBusInfoPtr;

typedef struct _drmHost1xDeviceInfo {
    char **compatible; /* NULL terminated list of compatible strings */
} drmHost1xDeviceInfo, *drmHost1xDeviceInfoPtr;

typedef struct _drmDevice {
    char **nodes; /* DRM_NODE_MAX sized array */
    int available_nodes; /* DRM_NODE_* bitmask */
    int bustype;
    union {
        drmPciBusInfoPtr pci;
        drmUsbBusInfoPtr usb;
        drmPlatformBusInfoPtr platform;
        drmHost1xBusInfoPtr host1x;
    } businfo;
    union {
        drmPciDeviceInfoPtr pci;
        drmUsbDeviceInfoPtr usb;
        drmPlatformDeviceInfoPtr platform;
        drmHost1xDeviceInfoPtr host1x;
    } deviceinfo;
} drmDevice, *drmDevicePtr;

extern int drmGetDevice(int fd, drmDevicePtr *device);
extern void drmFreeDevice(drmDevicePtr *device);

extern int drmGetDevices(drmDevicePtr devices[], int max_devices);
extern void drmFreeDevices(drmDevicePtr devices[], int count);

#define DRM_DEVICE_GET_PCI_REVISION (1 << 0)
extern int drmGetDevice2(int fd, uint32_t flags, drmDevicePtr *device);
extern int drmGetDevices2(uint32_t flags, drmDevicePtr devices[], int max_devices);

extern int drmDevicesEqual(drmDevicePtr a, drmDevicePtr b);

extern int drmSyncobjCreate(int fd, uint32_t flags, uint32_t *handle);
extern int drmSyncobjDestroy(int fd, uint32_t handle);
extern int drmSyncobjHandleToFD(int fd, uint32_t handle, int *obj_fd);
extern int drmSyncobjFDToHandle(int fd, int obj_fd, uint32_t *handle);

extern int drmSyncobjImportSyncFile(int fd, uint32_t handle, int sync_file_fd);
extern int drmSyncobjExportSyncFile(int fd, uint32_t handle, int *sync_file_fd);
extern int drmSyncobjWait(int fd, uint32_t *handles, unsigned num_handles,
			  int64_t timeout_nsec, unsigned flags,
			  uint32_t *first_signaled);
extern int drmSyncobjReset(int fd, const uint32_t *handles, uint32_t handle_count);
extern int drmSyncobjSignal(int fd, const uint32_t *handles, uint32_t handle_count);
extern int drmSyncobjTimelineSignal(int fd, const uint32_t *handles,
				    uint64_t *points, uint32_t handle_count);
extern int drmSyncobjTimelineWait(int fd, uint32_t *handles, uint64_t *points,
				  unsigned num_handles,
				  int64_t timeout_nsec, unsigned flags,
				  uint32_t *first_signaled);
extern int drmSyncobjQuery(int fd, uint32_t *handles, uint64_t *points,
			   uint32_t handle_count);
extern int drmSyncobjQuery2(int fd, uint32_t *handles, uint64_t *points,
			    uint32_t handle_count, uint32_t flags);
extern int drmSyncobjTransfer(int fd,
			      uint32_t dst_handle, uint64_t dst_point,
			      uint32_t src_handle, uint64_t src_point,
			      uint32_t flags);

#if defined(__cplusplus)
}
#endif

#endif
