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

#ifndef __VKI_XEN_X86_H
#define __VKI_XEN_X86_H

#if defined(__i386__)
#define ___DEFINE_VKI_XEN_GUEST_HANDLE(name, type)			\
    typedef struct { type *p; }						\
        __vki_xen_guest_handle_ ## name;                                \
    typedef struct { union { type *p; vki_xen_uint64_aligned_t q; }; }  \
        __vki_xen_guest_handle_64_ ## name
#define vki_xen_uint64_aligned_t vki_uint64_t __attribute__((aligned(8)))
#define __VKI_XEN_GUEST_HANDLE_64(name) __vki_xen_guest_handle_64_ ## name
#define VKI_XEN_GUEST_HANDLE_64(name) __VKI_XEN_GUEST_HANDLE_64(name)
#else
#define ___DEFINE_VKI_XEN_GUEST_HANDLE(name, type) \
    typedef struct { type *p; } __vki_xen_guest_handle_ ## name
#define vki_xen_uint64_aligned_t vki_uint64_t
#define __DEFINE_VKI_XEN_GUEST_HANDLE(name, type) \
    ___DEFINE_VKI_XEN_GUEST_HANDLE(name, type);   \
    ___DEFINE_VKI_XEN_GUEST_HANDLE(const_##name, const type)
#define DEFINE_VKI_XEN_GUEST_HANDLE(name)   __DEFINE_VKI_XEN_GUEST_HANDLE(name, name)
#define VKI_XEN_GUEST_HANDLE_64(name) VKI_XEN_GUEST_HANDLE(name)
#endif

#define __VKI_XEN_GUEST_HANDLE(name)  __vki_xen_guest_handle_ ## name
#define VKI_XEN_GUEST_HANDLE(name)    __VKI_XEN_GUEST_HANDLE(name)

typedef unsigned long vki_xen_pfn_t;
typedef unsigned long vki_xen_ulong_t;

#if defined(__i386__)
struct vki_xen_cpu_user_regs {
    vki_uint32_t ebx;
    vki_uint32_t ecx;
    vki_uint32_t edx;
    vki_uint32_t esi;
    vki_uint32_t edi;
    vki_uint32_t ebp;
    vki_uint32_t eax;
    vki_uint16_t error_code;    /* private */
    vki_uint16_t entry_vector;  /* private */
    vki_uint32_t eip;
    vki_uint16_t cs;
    vki_uint8_t  saved_upcall_mask;
    vki_uint8_t  _pad0;
    vki_uint32_t eflags;        /* eflags.IF == !saved_upcall_mask */
    vki_uint32_t esp;
    vki_uint16_t ss, _pad1;
    vki_uint16_t es, _pad2;
    vki_uint16_t ds, _pad3;
    vki_uint16_t fs, _pad4;
    vki_uint16_t gs, _pad5;
};
#else
struct vki_xen_cpu_user_regs {
    vki_uint64_t r15;
    vki_uint64_t r14;
    vki_uint64_t r13;
    vki_uint64_t r12;
    vki_uint64_t rbp;
    vki_uint64_t rbx;
    vki_uint64_t r11;
    vki_uint64_t r10;
    vki_uint64_t r9;
    vki_uint64_t r8;
    vki_uint64_t rax;
    vki_uint64_t rcx;
    vki_uint64_t rdx;
    vki_uint64_t rsi;
    vki_uint64_t rdi;
    vki_uint32_t error_code;    /* private */
    vki_uint32_t entry_vector;  /* private */
    vki_uint64_t rip;
    vki_uint16_t cs, _pad0[1];
    vki_uint8_t  saved_upcall_mask;
    vki_uint8_t  _pad1[3];
    vki_uint64_t rflags;      /* rflags.IF == !saved_upcall_mask */
    vki_uint64_t rsp;
    vki_uint16_t ss, _pad2[3];
    vki_uint16_t es, _pad3[3];
    vki_uint16_t ds, _pad4[3];
    vki_uint16_t fs, _pad5[3]; /* Non-zero => takes precedence over fs_base.     */
    vki_uint16_t gs, _pad6[3]; /* Non-zero => takes precedence over gs_base_usr. */
};
#endif

struct vki_xen_trap_info {
    vki_uint8_t   vector;  /* exception vector                              */
    vki_uint8_t   flags;   /* 0-3: privilege level; 4: clear event enable?  */
    vki_uint16_t  cs;      /* code selector                                 */
    unsigned long address; /* code offset                                   */
};

struct vki_xen_vcpu_guest_context {
    /* FPU registers come first so they can be aligned for FXSAVE/FXRSTOR. */
    struct { char x[512]; } fpu_ctxt;       /* User-level FPU registers     */
    unsigned long flags;                    /* VGCF_* flags                 */
    struct vki_xen_cpu_user_regs user_regs; /* User-level CPU registers     */
    struct vki_xen_trap_info trap_ctxt[256];/* Virtual IDT                  */
    unsigned long ldt_base, ldt_ents;       /* LDT (linear address, # ents) */
    unsigned long gdt_frames[16], gdt_ents; /* GDT (machine frames, # ents) */
    unsigned long kernel_ss, kernel_sp;     /* Virtual TSS (only SS1/SP1)   */
    /* NB. User pagetable on x86/64 is placed in ctrlreg[1]. */
    unsigned long ctrlreg[8];               /* CR0-CR7 (control registers)  */
    unsigned long debugreg[8];              /* DB0-DB7 (debug registers)    */
#ifdef __i386__
    unsigned long event_callback_cs;        /* CS:EIP of event callback     */
    unsigned long event_callback_eip;
    unsigned long failsafe_callback_cs;     /* CS:EIP of failsafe callback  */
    unsigned long failsafe_callback_eip;
#else
    unsigned long event_callback_eip;
    unsigned long failsafe_callback_eip;
    unsigned long syscall_callback_eip;
#endif
    unsigned long vm_assist;                /* VMASST_TYPE_* bitmap */
#ifdef __x86_64__
    /* Segment base addresses. */
    vki_uint64_t  fs_base;
    vki_uint64_t  gs_base_kernel;
    vki_uint64_t  gs_base_user;
#endif
};
typedef struct vki_xen_vcpu_guest_context vki_xen_vcpu_guest_context_t;
DEFINE_VKI_XEN_GUEST_HANDLE(vki_xen_vcpu_guest_context_t);


/* HVM_SAVE types and declarations for getcontext_partial */
# define VKI_DECLARE_HVM_SAVE_TYPE(_x, _code, _type)                         \
    struct __VKI_HVM_SAVE_TYPE_##_x { _type t; char c[_code]; char cpt[1];}

#define VKI_HVM_SAVE_TYPE(_x) typeof (((struct __VKI_HVM_SAVE_TYPE_##_x *)(0))->t)
#define VKI_HVM_SAVE_LENGTH(_x) (sizeof (VKI_HVM_SAVE_TYPE(_x)))
#define VKI_HVM_SAVE_CODE(_x) (sizeof (((struct __VKI_HVM_SAVE_TYPE_##_x *)(0))->c))

struct vki_hvm_hw_cpu {
   vki_uint8_t  fpu_regs[512];

   vki_uint64_t rax;
   vki_uint64_t rbx;
   vki_uint64_t rcx;
   vki_uint64_t rdx;
   vki_uint64_t rbp;
   vki_uint64_t rsi;
   vki_uint64_t rdi;
   vki_uint64_t rsp;
   vki_uint64_t r8;
   vki_uint64_t r9;
   vki_uint64_t r10;
   vki_uint64_t r11;
   vki_uint64_t r12;
   vki_uint64_t r13;
   vki_uint64_t r14;
   vki_uint64_t r15;

   vki_uint64_t rip;
   vki_uint64_t rflags;

   vki_uint64_t cr0;
   vki_uint64_t cr2;
   vki_uint64_t cr3;
   vki_uint64_t cr4;

   vki_uint64_t dr0;
   vki_uint64_t dr1;
   vki_uint64_t dr2;
   vki_uint64_t dr3;
   vki_uint64_t dr6;
   vki_uint64_t dr7;

   vki_uint32_t cs_sel;
   vki_uint32_t ds_sel;
   vki_uint32_t es_sel;
   vki_uint32_t fs_sel;
   vki_uint32_t gs_sel;
   vki_uint32_t ss_sel;
   vki_uint32_t tr_sel;
   vki_uint32_t ldtr_sel;

   vki_uint32_t cs_limit;
   vki_uint32_t ds_limit;
   vki_uint32_t es_limit;
   vki_uint32_t fs_limit;
   vki_uint32_t gs_limit;
   vki_uint32_t ss_limit;
   vki_uint32_t tr_limit;
   vki_uint32_t ldtr_limit;
   vki_uint32_t idtr_limit;
   vki_uint32_t gdtr_limit;

   vki_uint64_t cs_base;
   vki_uint64_t ds_base;
   vki_uint64_t es_base;
   vki_uint64_t fs_base;
   vki_uint64_t gs_base;
   vki_uint64_t ss_base;
   vki_uint64_t tr_base;
   vki_uint64_t ldtr_base;
   vki_uint64_t idtr_base;
   vki_uint64_t gdtr_base;

   vki_uint32_t cs_arbytes;
   vki_uint32_t ds_arbytes;
   vki_uint32_t es_arbytes;
   vki_uint32_t fs_arbytes;
   vki_uint32_t gs_arbytes;
   vki_uint32_t ss_arbytes;
   vki_uint32_t tr_arbytes;
   vki_uint32_t ldtr_arbytes;

   vki_uint64_t sysenter_cs;
   vki_uint64_t sysenter_esp;
   vki_uint64_t sysenter_eip;

    /* msr for em64t */
   vki_uint64_t shadow_gs;

    /* msr content saved/restored. */
   vki_uint64_t msr_flags;
   vki_uint64_t msr_lstar;
   vki_uint64_t msr_star;
   vki_uint64_t msr_cstar;
   vki_uint64_t msr_syscall_mask;
   vki_uint64_t msr_efer;
   vki_uint64_t msr_tsc_aux;

    /* guest's idea of what rdtsc() would return */
   vki_uint64_t tsc;

    /* pending event, if any */
    union {
       vki_uint32_t pending_event;
        struct {
           vki_uint8_t  pending_vector:8;
           vki_uint8_t  pending_type:3;
           vki_uint8_t  pending_error_valid:1;
           vki_uint32_t pending_reserved:19;
           vki_uint8_t  pending_valid:1;
        };
    };
    /* error code for pending event */
   vki_uint32_t error_code;
};

VKI_DECLARE_HVM_SAVE_TYPE(CPU, 2, struct vki_hvm_hw_cpu);

struct vki_hvm_hw_mtrr {
#define VKI_MTRR_VCNT     8
#define VKI_NUM_FIXED_MSR 11
   vki_uint64_t msr_pat_cr;
   /* mtrr physbase & physmask msr pair*/
   vki_uint64_t msr_mtrr_var[VKI_MTRR_VCNT*2];
   vki_uint64_t msr_mtrr_fixed[VKI_NUM_FIXED_MSR];
   vki_uint64_t msr_mtrr_cap;
   vki_uint64_t msr_mtrr_def_type;
};

VKI_DECLARE_HVM_SAVE_TYPE(MTRR, 14, struct vki_hvm_hw_mtrr);

#endif // __VKI_XEN_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
