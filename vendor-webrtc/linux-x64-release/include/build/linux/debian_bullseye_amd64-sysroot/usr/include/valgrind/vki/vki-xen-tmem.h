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

#ifndef __VKI_XEN_TMEM_H
#define __VKI_XEN_TMEM_H

typedef VKI_XEN_GUEST_HANDLE(char) vki_xen_tmem_cli_va_t;


/* version of ABI */
#define VKI_XEN_TMEM_spec_version          1

/* Commands to HYPERVISOR_tmem_op() */
#define VKI_XEN_TMEM_control               0
#define VKI_XEN_TMEM_new_pool              1
#define VKI_XEN_TMEM_destroy_pool          2
#define VKI_XEN_TMEM_new_page              3
#define VKI_XEN_TMEM_put_page              4
#define VKI_XEN_TMEM_get_page              5
#define VKI_XEN_TMEM_flush_page            6
#define VKI_XEN_TMEM_flush_object          7
#define VKI_XEN_TMEM_read                  8
#define VKI_XEN_TMEM_write                 9
#define VKI_XEN_TMEM_xchg                 10
/* Privileged commands to HYPERVISOR_tmem_op() */
#define VKI_XEN_tmem_auth                 101
#define VKI_XEN_tmem_restore_new          102

/* for cmd = TMEM_CONTROL */
struct vki_xen_tmem_ctrl {
    vki_uint32_t subop;

/* Subops for HYPERVISOR_tmem_op(TMEM_CONTROL) */
#define VKI_XEN_TMEMC_thaw                   0
#define VKI_XEN_TMEMC_freeze                 1
#define VKI_XEN_TMEMC_flush                  2
#define VKI_XEN_TMEMC_destroy                3
#define VKI_XEN_TMEMC_list                   4
#define VKI_XEN_TMEMC_set_weight             5
#define VKI_XEN_TMEMC_set_cap                6
#define VKI_XEN_TMEMC_set_compress           7
#define VKI_XEN_TMEMC_query_freeable_mb      8
#define VKI_XEN_TMEMC_save_begin             10
#define VKI_XEN_TMEMC_save_get_version       11
#define VKI_XEN_TMEMC_save_get_maxpools      12
#define VKI_XEN_TMEMC_save_get_client_weight 13
#define VKI_XEN_TMEMC_save_get_client_cap    14
#define VKI_XEN_TMEMC_save_get_client_flags  15
#define VKI_XEN_TMEMC_save_get_pool_flags    16
#define VKI_XEN_TMEMC_save_get_pool_npages   17
#define VKI_XEN_TMEMC_save_get_pool_uuid     18
#define VKI_XEN_TMEMC_save_get_next_page     19
#define VKI_XEN_TMEMC_save_get_next_inv      20
#define VKI_XEN_TMEMC_save_end               21
#define VKI_XEN_TMEMC_restore_begin          30
#define VKI_XEN_TMEMC_restore_put_page       32
#define VKI_XEN_TMEMC_restore_flush_page     33

    vki_uint32_t cli_id;
    vki_uint32_t arg1;
    vki_uint32_t arg2;
    vki_uint64_t oid[3];
    vki_xen_tmem_cli_va_t buf;
};

struct vki_xen_tmem_op {
    vki_uint32_t cmd;
    vki_int32_t pool_id;
    union {
        struct vki_xen_tmem_ctrl ctrl;
    } u;
};

#endif // __VKI_XEN_TMEM_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
