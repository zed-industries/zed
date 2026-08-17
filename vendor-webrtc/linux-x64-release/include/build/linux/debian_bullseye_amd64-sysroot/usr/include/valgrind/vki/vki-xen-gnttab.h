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

#ifndef __VKI_XEN_GNTTAB_H
#define __VKI_XEN_GNTTAB_H

typedef vki_uint32_t vki_xen_grant_ref_t;

#define VKI_XEN_GNTTABOP_map_grant_ref        0
#define VKI_XEN_GNTTABOP_unmap_grant_ref      1
#define VKI_XEN_GNTTABOP_setup_table          2
#define VKI_XEN_GNTTABOP_dump_table           3
#define VKI_XEN_GNTTABOP_transfer             4
#define VKI_XEN_GNTTABOP_copy                 5
#define VKI_XEN_GNTTABOP_query_size           6
#define VKI_XEN_GNTTABOP_unmap_and_replace    7
#define VKI_XEN_GNTTABOP_set_version          8
#define VKI_XEN_GNTTABOP_get_status_frames    9
#define VKI_XEN_GNTTABOP_get_version          10
#define VKI_XEN_GNTTABOP_swap_grant_ref	      11

struct vki_xen_gnttab_setup_table {
    /* IN parameters. */
    vki_xen_domid_t  dom;
    vki_uint32_t nr_frames;
    /* OUT parameters. */
    vki_int16_t  status;              /* => enum grant_status */
    VKI_XEN_GUEST_HANDLE(vki_ulong) frame_list;
};

#endif // __VKI_XEN_GNTTAB_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
