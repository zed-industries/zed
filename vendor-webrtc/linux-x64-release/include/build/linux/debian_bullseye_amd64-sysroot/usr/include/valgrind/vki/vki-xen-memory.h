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

#ifndef __VKI_XEN_MEMORY_H
#define __VKI_XEN_MEMORY_H

#define VKI_XENMEM_increase_reservation 0
#define VKI_XENMEM_decrease_reservation 1
#define VKI_XENMEM_maximum_ram_page     2
#define VKI_XENMEM_current_reservation  3
#define VKI_XENMEM_maximum_reservation  4
#define VKI_XENMEM_machphys_mfn_list    5
#define VKI_XENMEM_populate_physmap     6
#define VKI_XENMEM_add_to_physmap       7
#define VKI_XENMEM_memory_map           9
#define VKI_XENMEM_machine_memory_map   10
#define VKI_XENMEM_exchange             11
#define VKI_XENMEM_machphys_mapping     12
#define VKI_XENMEM_set_memory_map       13
#define VKI_XENMEM_maximum_gpfn         14
#define VKI_XENMEM_remove_from_physmap  15
#define VKI_XENMEM_set_pod_target       16
#define VKI_XENMEM_get_pod_target       17
#define VKI_XENMEM_get_sharing_freed_pages    18
#define VKI_XENMEM_get_sharing_shared_pages   19
#define VKI_XENMEM_access_op                  21
#define VKI_XENMEM_claim_pages                24
#define VKI_XENMEM_machphys_compat_mfn_list   25

struct vki_xen_memory_map {
    unsigned int nr_entries;
    VKI_XEN_GUEST_HANDLE(void) buffer;
};

struct vki_xen_foreign_memory_map {
    vki_xen_domid_t domid;
    struct vki_xen_memory_map map;
};

struct xen_memory_reservation {
    VKI_XEN_GUEST_HANDLE(vki_xen_pfn_t) extent_start;
    vki_xen_ulong_t    nr_extents;
    unsigned int   extent_order;
    unsigned int   mem_flags;
    vki_xen_domid_t domid;
};

struct vki_xen_machphys_mfn_list {
    unsigned int max_extents; /* IN */
    VKI_XEN_GUEST_HANDLE(vki_xen_pfn_t) extent_start; /* OUT */
    unsigned int nr_extents; /* OUT */
};

struct vki_xen_add_to_physmap {
    vki_xen_domid_t domid;
    vki_uint16_t size;

#define VKI_XENMAPSPACE_shared_info  0
#define VKI_XENMAPSPACE_grant_table  1
#define VKI_XENMAPSPACE_gmfn         2
#define VKI_XENMAPSPACE_gmfn_range   3
#define VKI_XENMAPSPACE_gmfn_foreign 4

    unsigned int space;
    vki_xen_ulong_t idx;
    vki_xen_pfn_t gpfn;
};

struct vki_xen_remove_from_physmap {
    vki_xen_domid_t domid;
    vki_xen_pfn_t gpfn;
};

struct vki_xen_mem_event_op {
    vki_uint8_t     op;
    vki_xen_domid_t     domain;
    vki_uint64_t    buffer;
    vki_uint64_t    gfn;
};

#endif // __VKI_XEN_MEMORY_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
