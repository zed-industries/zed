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

#ifndef __VKI_XEN_HVM_H
#define __VKI_XEN_HVM_H

/* Get/set subcommands: extra argument == pointer to xen_hvm_param struct. */
#define VKI_XEN_HVMOP_set_param           0
#define VKI_XEN_HVMOP_get_param           1
struct vki_xen_hvm_param {
    vki_xen_domid_t  domid;    /* IN */
    vki_uint32_t index;    /* IN */
    vki_uint64_t value;    /* IN/OUT */
};

#define VKI_XEN_HVMOP_set_pci_intx_level  2
struct vki_xen_hvm_set_pci_intx_level {
    vki_xen_domid_t  domid;
    vki_uint8_t  domain, bus, device, intx;
    vki_uint8_t  level;
};
typedef struct vki_xen_hvm_set_pci_intx_level vki_xen_hvm_set_pci_intx_level_t;

#define VKI_XEN_HVMOP_set_isa_irq_level 3
struct vki_xen_hvm_set_isa_irq_level {
    vki_xen_domid_t  domid;
    vki_uint8_t  isa_irq;
    vki_uint8_t  level;
};
typedef struct vki_xen_hvm_set_isa_irq_level vki_xen_hvm_set_isa_irq_level_t;

#define VKI_XEN_HVMOP_set_pci_link_route 4
struct vki_xen_hvm_set_pci_link_route {
    vki_xen_domid_t  domid;
    vki_uint8_t  link;
    vki_uint8_t  isa_irq;
};
typedef struct vki_xen_hvm_set_pci_link_route vki_xen_hvm_set_pci_link_route_t;

#define VKI_XEN_HVMOP_track_dirty_vram 6
struct vki_xen_hvm_track_dirty_vram {
    vki_xen_domid_t  domid;                          /* IN  */
    vki_xen_uint64_aligned_t first_pfn;              /* IN  */
    vki_xen_uint64_aligned_t nr;                     /* IN  */
    VKI_XEN_GUEST_HANDLE_64(vki_uint8) dirty_bitmap; /* OUT */
};
typedef struct vki_xen_hvm_track_dirty_vram vki_xen_hvm_track_dirty_vram_t;

#define VKI_XEN_HVMOP_set_mem_type 8
struct vki_xen_hvm_set_mem_type {
    vki_xen_domid_t  domid;
    vki_uint16_t hvmmem_type;
    vki_uint32_t nr;
    vki_uint64_t first_pfn;
};
typedef struct vki_xen_hvm_set_mem_type vki_xen_hvm_set_mem_type_t;

#define VKI_XEN_HVMOP_set_mem_access        12
struct vki_xen_hvm_set_mem_access {
    vki_xen_domid_t domid;
    vki_uint16_t hvmmem_access;
    vki_uint32_t nr;
    vki_uint64_t first_pfn;
};
typedef struct vki_xen_hvm_set_mem_access vki_xen_hvm_set_mem_access_t;

#define VKI_XEN_HVMOP_get_mem_access        13
struct vki_xen_hvm_get_mem_access {
    vki_xen_domid_t domid;
    vki_uint16_t hvmmem_access; /* OUT */
    vki_uint64_t pfn;
};
typedef struct vki_xen_hvm_get_mem_access vki_xen_hvm_get_mem_access_t;

#define VKI_XEN_HVMOP_inject_trap            14
struct vki_xen_hvm_inject_trap {
    vki_xen_domid_t domid;
    vki_uint32_t vcpuid;
    vki_uint32_t vector;
    vki_uint32_t type;
    vki_uint32_t error_code;
    vki_uint32_t insn_len;
    vki_uint64_t cr2;
};
typedef struct vki_xen_hvm_inject_trap vki_xen_hvm_inject_trap_t;

#define VKI_XEN_HVMOP_altp2m 25
#define VKI_XEN_HVMOP_altp2m_get_domain_state     1
#define VKI_XEN_HVMOP_altp2m_set_domain_state     2
#define VKI_XEN_HVMOP_altp2m_vcpu_enable_notify   3
#define VKI_XEN_HVMOP_altp2m_create_p2m           4
#define VKI_XEN_HVMOP_altp2m_destroy_p2m          5
#define VKI_XEN_HVMOP_altp2m_switch_p2m           6
#define VKI_XEN_HVMOP_altp2m_set_mem_access       7
#define VKI_XEN_HVMOP_altp2m_change_gfn           8
struct vki_xen_hvm_altp2m_domain_state {
    /* IN or OUT variable on/off */
    vki_uint8_t state;
};
typedef struct vki_xen_hvm_altp2m_domain_state vki_xen_hvm_altp2m_domain_state_t;
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_hvm_altp2m_domain_state_t);

struct vki_xen_hvm_altp2m_vcpu_enable_notify {
    vki_uint32_t vcpu_id;
    vki_uint32_t pad;
    /* #VE info area gfn */
    vki_uint64_t gfn;
};
typedef struct vki_xen_hvm_altp2m_vcpu_enable_notify vki_xen_hvm_altp2m_vcpu_enable_notify_t;
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_hvm_altp2m_vcpu_enable_notify_t);

struct vki_xen_hvm_altp2m_view {
    /* IN/OUT variable */
    vki_uint16_t view;
    /* Create view only: default access type
     * NOTE: currently ignored */
    vki_uint16_t hvmmem_default_access; /* xenmem_access_t */
};
typedef struct vki_xen_hvm_altp2m_view vki_xen_hvm_altp2m_view_t;
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_hvm_altp2m_view_t);

struct vki_xen_hvm_altp2m_set_mem_access {
    /* view */
    vki_uint16_t view;
    /* Memory type */
    vki_uint16_t hvmmem_access; /* xenmem_access_t */
    vki_uint32_t pad;
    /* gfn */
    vki_uint64_t gfn;
};
typedef struct vki_xen_hvm_altp2m_set_mem_access vki_xen_hvm_altp2m_set_mem_access_t;
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_hvm_altp2m_set_mem_access_t);

struct vki_xen_hvm_altp2m_change_gfn {
    /* view */
    vki_uint16_t view;
    vki_uint16_t pad1;
    vki_uint32_t pad2;
    /* old gfn */
    vki_uint64_t old_gfn;
    /* new gfn, INVALID_GFN (~0UL) means revert */
    vki_uint64_t new_gfn;
};
typedef struct vki_xen_hvm_altp2m_change_gfn vki_xen_hvm_altp2m_change_gfn_t;
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_hvm_altp2m_change_gfn_t);

struct vki_xen_hvm_altp2m_op {
    vki_uint32_t version;   /* HVMOP_ALTP2M_INTERFACE_VERSION */
    vki_uint32_t cmd;
    vki_xen_domid_t domain;
    vki_uint16_t pad1;
    vki_uint32_t pad2;
    union {
        struct vki_xen_hvm_altp2m_domain_state       domain_state;
        struct vki_xen_hvm_altp2m_vcpu_enable_notify enable_notify;
        struct vki_xen_hvm_altp2m_view               view;
        struct vki_xen_hvm_altp2m_set_mem_access     set_mem_access;
        struct vki_xen_hvm_altp2m_change_gfn         change_gfn;
        vki_uint8_t pad[64];
    } u;
};
typedef struct vki_xen_hvm_altp2m_op vki_xen_hvm_altp2m_op_t;
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_hvm_altp2m_op_t);

#endif // __VKI_XEN_HVM_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
