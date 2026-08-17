/*
 * Copyright (c) 1999 Apple Computer, Inc. All rights reserved.
 *
 * @APPLE_LICENSE_HEADER_START@
 * 
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this
 * file.
 * 
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 * 
 * @APPLE_LICENSE_HEADER_END@
 */
#ifndef _MACH_O_ARCH_H_
#define _MACH_O_ARCH_H_
/*
 * Copyright (c) 1997 Apple Computer, Inc.
 *
 * Functions that deal with information about architectures.
 *
 */

#include <stdint.h>
#include <mach/machine.h>
#include <architecture/byte_order.h>

/* The NXArchInfo structs contain the architectures symbolic name
 * (such as "ppc"), its CPU type and CPU subtype as defined in
 * mach/machine.h, the byte order for the architecture, and a
 * describing string (such as "PowerPC").
 * There will both be entries for specific CPUs (such as ppc604e) as
 * well as generic "family" entries (such as ppc).
 */
typedef struct {
    const char *name;
    cpu_type_t cputype;
    cpu_subtype_t cpusubtype;
    enum NXByteOrder byteorder;
    const char *description;
} NXArchInfo;

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/* NXGetAllArchInfos() returns a pointer to an array of all known
 * NXArchInfo structures.  The last NXArchInfo is marked by a NULL name.
 */
extern const NXArchInfo *NXGetAllArchInfos(void);

/* NXGetLocalArchInfo() returns the NXArchInfo for the local host, or NULL
 * if none is known. 
 */
extern const NXArchInfo *NXGetLocalArchInfo(void);

/* NXGetArchInfoFromName() and NXGetArchInfoFromCpuType() return the
 * NXArchInfo from the architecture's name or cputype/cpusubtype
 * combination.  A cpusubtype of CPU_SUBTYPE_MULTIPLE can be used
 * to request the most general NXArchInfo known for the given cputype.
 * NULL is returned if no matching NXArchInfo can be found.
 */
extern const NXArchInfo *NXGetArchInfoFromName(const char *name);
extern const NXArchInfo *NXGetArchInfoFromCpuType(cpu_type_t cputype,
						  cpu_subtype_t cpusubtype);

/* The above interfaces that return pointers to NXArchInfo structs in normal
 * cases returns a pointer from the array returned in NXGetAllArchInfos().
 * In some cases when the cputype is CPU_TYPE_I386 or CPU_TYPE_POWERPC it will
 * retun malloc(3)'ed NXArchInfo struct which contains a string in the
 * description field also a malloc(3)'ed pointer.  To allow programs not to
 * leak memory they can call NXFreeArchInfo() on pointers returned from the
 * above interfaces.  Since this is a new API on older systems can use the
 * code below.  Going forward the above interfaces will only return pointers
 * from the array returned in NXGetAllArchInfos().
 */
extern void NXFreeArchInfo(const NXArchInfo *x);

/* The code that can be used for NXFreeArchInfo() when it is not available is:
 *
 *	static void NXFreeArchInfo(
 *	const NXArchInfo *x)
 *	{
 *	    const NXArchInfo *p;
 *	
 *	        p = NXGetAllArchInfos();
 *	        while(p->name != NULL){
 *	            if(x == p)
 *	                return;
 *	            p++;
 *	        }
 *	        free((char *)x->description);
 *	        free((NXArchInfo *)x);
 *	}
 */

/* NXFindBestFatArch() is passed a cputype and cpusubtype and a set of
 * fat_arch structs and selects the best one that matches (if any) and returns
 * a pointer to that fat_arch struct (or NULL).  The fat_arch structs must be
 * in the host byte order and correct such that the fat_archs really points to
 * enough memory for nfat_arch structs.  It is possible that this routine could
 * fail if new cputypes or cpusubtypes are added and an old version of this
 * routine is used.  But if there is an exact match between the cputype and
 * cpusubtype and one of the fat_arch structs this routine will always succeed.
 */
extern struct fat_arch *NXFindBestFatArch(cpu_type_t cputype,
					  cpu_subtype_t cpusubtype,
					  struct fat_arch *fat_archs,
					  uint32_t nfat_archs);

/* NXFindBestFatArch_64() is passed a cputype and cpusubtype and a set of
 * fat_arch_64 structs and selects the best one that matches (if any) and
 * returns a pointer to that fat_arch_64 struct (or NULL).  The fat_arch_64
 * structs must be in the host byte order and correct such that the fat_archs64
 * really points to enough memory for nfat_arch structs.  It is possible that
 * this routine could fail if new cputypes or cpusubtypes are added and an old
 * version of this routine is used.  But if there is an exact match between the
 * cputype and cpusubtype and one of the fat_arch_64 structs this routine will
 * always succeed.
 */
extern struct fat_arch_64 *NXFindBestFatArch_64(cpu_type_t cputype,
					        cpu_subtype_t cpusubtype,
					        struct fat_arch_64 *fat_archs64,
					        uint32_t nfat_archs);

/* NXCombineCpuSubtypes() returns the resulting cpusubtype when combining two
 * different cpusubtypes for the specified cputype.  If the two cpusubtypes
 * can't be combined (the specific subtypes are mutually exclusive) -1 is
 * returned indicating it is an error to combine them.  This can also fail and
 * return -1 if new cputypes or cpusubtypes are added and an old version of
 * this routine is used.  But if the cpusubtypes are the same they can always
 * be combined and this routine will return the cpusubtype pass in.
 */
extern cpu_subtype_t NXCombineCpuSubtypes(cpu_type_t cputype,
					  cpu_subtype_t cpusubtype1,
					  cpu_subtype_t cpusubtype2);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* _MACH_O_ARCH_H_ */