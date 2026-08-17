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

#ifndef __VKI_XEN_EVTCHN_H
#define __VKI_XEN_EVTCHN_H

#define VKI_XEN_EVTCHNOP_bind_interdomain 0
#define VKI_XEN_EVTCHNOP_bind_virq        1
#define VKI_XEN_EVTCHNOP_bind_pirq        2
#define VKI_XEN_EVTCHNOP_close            3
#define VKI_XEN_EVTCHNOP_send             4
#define VKI_XEN_EVTCHNOP_status           5
#define VKI_XEN_EVTCHNOP_alloc_unbound    6
#define VKI_XEN_EVTCHNOP_bind_ipi         7
#define VKI_XEN_EVTCHNOP_bind_vcpu        8
#define VKI_XEN_EVTCHNOP_unmask           9
#define VKI_XEN_EVTCHNOP_reset           10

typedef vki_uint32_t vki_xen_evtchn_port_t;
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_evtchn_port_t);

struct vki_xen_evtchn_alloc_unbound {
    /* IN parameters */
    vki_xen_domid_t dom, remote_dom;
    /* OUT parameters */
    vki_xen_evtchn_port_t port;
};

struct vki_xen_evtchn_op {
    vki_uint32_t cmd; /* enum event_channel_op */
    union {
        struct vki_xen_evtchn_alloc_unbound    alloc_unbound;
        //struct vki_xen_evtchn_bind_interdomain bind_interdomain;
        //struct vki_xen_evtchn_bind_virq        bind_virq;
        //struct vki_xen_evtchn_bind_pirq        bind_pirq;
        //struct vki_xen_evtchn_bind_ipi         bind_ipi;
        //struct vki_xen_evtchn_close            close;
        //struct vki_xen_evtchn_send             send;
        //struct vki_xen_evtchn_status           status;
        //struct vki_xen_evtchn_bind_vcpu        bind_vcpu;
        //struct vki_xen_evtchn_unmask           unmask;
    } u;
};

#endif // __VKI_XEN_EVTCHN_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
