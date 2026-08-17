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

#ifndef __VKI_XEN_PHYSDEV_H
#define __VKI_XEN_PHYSDEV_H

#define VKI_XEN_MAP_PIRQ_TYPE_MSI               0x0
#define VKI_XEN_MAP_PIRQ_TYPE_GSI               0x1
#define VKI_XEN_MAP_PIRQ_TYPE_UNKNOWN           0x2
#define VKI_XEN_MAP_PIRQ_TYPE_MSI_SEG           0x3
#define VKI_XEN_MAP_PIRQ_TYPE_MULTI_MSI         0x4

#define VKI_XEN_PHYSDEVOP_map_pirq               13
struct vki_xen_physdev_map_pirq {
    vki_xen_domid_t domid;
    /* IN */
    int type;
    /* IN (ignored for ..._MULTI_MSI) */
    int index;
    /* IN or OUT */
    int pirq;
    /* IN - high 16 bits hold segment for ..._MSI_SEG and ..._MULTI_MSI */
    int bus;
    /* IN */
    int devfn;
    /* IN (also OUT for ..._MULTI_MSI) */
    int entry_nr;
    /* IN */
    vki_uint64_t table_base;
};

#define VKI_XEN_PHYSDEVOP_unmap_pirq             14
struct vki_xen_physdev_unmap_pirq {
    vki_xen_domid_t domid;
    /* IN */
    int pirq;
};

#endif // __VKI_XEN_PHYSDEV_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
