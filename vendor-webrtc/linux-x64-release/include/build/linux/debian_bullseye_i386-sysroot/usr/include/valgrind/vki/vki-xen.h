/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2012-2017 Citrix

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.

   The GNU General Public License is contained in the file COPYING.
*/

/* Contributed by Andrew Cooper <andrew.cooper3@citrix.com>
   and Ian Campbell <ian.campbell@citrix.com> */

#ifndef __VKI_XEN_H
#define __VKI_XEN_H

#define ENABLE_XEN 1

#define __VKI_XEN_set_trap_table        0
#define __VKI_XEN_mmu_update            1
#define __VKI_XEN_set_gdt               2
#define __VKI_XEN_stack_switch          3
#define __VKI_XEN_set_callbacks         4
#define __VKI_XEN_fpu_taskswitch        5
#define __VKI_XEN_sched_op_compat       6 /* compat since 0x00030101 */
#define __VKI_XEN_platform_op           7
#define __VKI_XEN_set_debugreg          8
#define __VKI_XEN_get_debugreg          9
#define __VKI_XEN_update_descriptor    10
#define __VKI_XEN_memory_op            12
#define __VKI_XEN_multicall            13
#define __VKI_XEN_update_va_mapping    14
#define __VKI_XEN_set_timer_op         15
#define __VKI_XEN_event_channel_op_compat 16 /* compat since 0x00030202 */
#define __VKI_XEN_xen_version          17
#define __VKI_XEN_console_io           18
#define __VKI_XEN_physdev_op_compat    19 /* compat since 0x00030202 */
#define __VKI_XEN_grant_table_op       20
#define __VKI_XEN_vm_assist            21
#define __VKI_XEN_update_va_mapping_otherdomain 22
#define __VKI_XEN_iret                 23 /* x86 only */
#define __VKI_XEN_vcpu_op              24
#define __VKI_XEN_set_segment_base     25 /* x86/64 only */
#define __VKI_XEN_mmuext_op            26
#define __VKI_XEN_xsm_op               27
#define __VKI_XEN_nmi_op               28
#define __VKI_XEN_sched_op             29
#define __VKI_XEN_callback_op          30
#define __VKI_XEN_xenoprof_op          31
#define __VKI_XEN_event_channel_op     32
#define __VKI_XEN_physdev_op           33
#define __VKI_XEN_hvm_op               34
#define __VKI_XEN_sysctl               35
#define __VKI_XEN_domctl               36
#define __VKI_XEN_kexec_op             37
#define __VKI_XEN_tmem_op              38
#define __VKI_XEN_xc_reserved_op       39 /* reserved for XenClient */

#define __DEFINE_VKI_XEN_GUEST_HANDLE(name, type) \
    ___DEFINE_VKI_XEN_GUEST_HANDLE(name, type);   \
    ___DEFINE_VKI_XEN_GUEST_HANDLE(const_##name, const type)
#define DEFINE_VKI_XEN_GUEST_HANDLE(name)   __DEFINE_VKI_XEN_GUEST_HANDLE(name, name)

typedef vki_uint8_t vki_xen_domain_handle_t[16];
typedef vki_uint16_t vki_xen_domid_t;

#if defined(__i386__) || defined(__x86_64__)
#include <vki/vki-xen-x86.h>
#else
#error "Need to define per-ARCH Xen types for this platform"
#endif

DEFINE_VKI_XEN_GUEST_HANDLE(void);
DEFINE_VKI_XEN_GUEST_HANDLE(char);
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_pfn_t);

__DEFINE_VKI_XEN_GUEST_HANDLE(vki_ulong, unsigned long);

__DEFINE_VKI_XEN_GUEST_HANDLE(vki_int16, vki_int16_t);
__DEFINE_VKI_XEN_GUEST_HANDLE(vki_int32, vki_int32_t);
__DEFINE_VKI_XEN_GUEST_HANDLE(vki_int64, vki_int64_t);

__DEFINE_VKI_XEN_GUEST_HANDLE(vki_uint8, vki_uint8_t);
__DEFINE_VKI_XEN_GUEST_HANDLE(vki_uint16, vki_uint16_t);
__DEFINE_VKI_XEN_GUEST_HANDLE(vki_uint32, vki_uint32_t);
__DEFINE_VKI_XEN_GUEST_HANDLE(vki_uint64, vki_uint64_t);

struct vki_xenctl_bitmap {
    VKI_XEN_GUEST_HANDLE_64(vki_uint8) bitmap;
    vki_uint32_t nr_bits;
};

#include <vki/vki-xen-domctl.h>
#include <vki/vki-xen-sysctl.h>
#include <vki/vki-xen-mmuext.h>
#include <vki/vki-xen-schedop.h>
#include <vki/vki-xen-memory.h>
#include <vki/vki-xen-evtchn.h>
#include <vki/vki-xen-gnttab.h>
#include <vki/vki-xen-version.h>
#include <vki/vki-xen-hvm.h>
#include <vki/vki-xen-tmem.h>
#include <vki/vki-xen-xsm.h>
#include <vki/vki-xen-physdev.h>

#endif // __VKI_XEN_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
