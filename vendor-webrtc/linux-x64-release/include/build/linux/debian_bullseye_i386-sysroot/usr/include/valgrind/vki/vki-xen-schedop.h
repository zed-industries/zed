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

#ifndef __VKI_XEN_SCHED_OP_H
#define __VKI_XEN_SCHED_OP_H

#define VKI_XEN_SCHEDOP_yield           0

#define VKI_XEN_SCHEDOP_block           1

#define VKI_XEN_SCHEDOP_shutdown        2

#define VKI_XEN_SCHEDOP_poll            3

#define VKI_XEN_SCHEDOP_remote_shutdown 4
struct vki_xen_remote_shutdown {
    vki_xen_domid_t domain_id;
    unsigned int reason;
};
typedef struct vki_xen_remote_shutdown vki_xen_remote_shutdown_t;

#define VKI_XEN_SCHEDOP_shutdown_code   5

#define VKI_XEN_SCHEDOP_watchdog        6

#endif /* __VKI_XEN_SCHED_OP_H */
