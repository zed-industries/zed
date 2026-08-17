
/*--------------------------------------------------------------------*/
/*--- The core/tool interface.                pub_tool_tooliface.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2000-2017 Julian Seward
      jseward@acm.org

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

#ifndef __PUB_TOOL_TOOLIFACE_H
#define __PUB_TOOL_TOOLIFACE_H

#include "pub_tool_errormgr.h"   // for Error, Supp
#include "libvex.h"              // for all Vex stuff

/* ------------------------------------------------------------------ */
/* The interface version */

/* Initialise tool.   Must do the following:
   - initialise the `details' struct, via the VG_(details_*)() functions
   - register the basic tool functions, via VG_(basic_tool_funcs)().
   May do the following:
   - initialise the `needs' struct to indicate certain requirements, via
     the VG_(needs_*)() functions
   - any other tool-specific initialisation
*/
extern void (*VG_(tl_pre_clo_init)) ( void );

/* Every tool must include this macro somewhere, exactly once.  The
   interface version is no longer relevant, but we kept the same name
   to avoid requiring changes to tools.
*/
#define VG_DETERMINE_INTERFACE_VERSION(pre_clo_init) \
   void (*VG_(tl_pre_clo_init)) ( void ) = pre_clo_init;

/* ------------------------------------------------------------------ */
/* Basic tool functions */

/* The tool_instrument function is passed as a callback to
   LibVEX_Translate.  VgCallbackClosure carries additional info
   which the instrumenter might like to know, but which is opaque to
   Vex.
*/
typedef 
   struct {
      Addr     nraddr; /* non-redirected guest address */
      Addr     readdr; /* redirected guest address */
      ThreadId tid;    /* tid requesting translation */
   }
   VgCallbackClosure;

extern void VG_(basic_tool_funcs)(
   // Do any initialisation that can only be done after command line
   // processing.
   void  (*post_clo_init)(void),

   // Instrument a basic block.  Must be a true function, ie. the same
   // input always results in the same output, because basic blocks
   // can be retranslated, unless you're doing something really
   // strange.  Anyway, the arguments.  Mostly they are straightforward
   // except for the distinction between redirected and non-redirected
   // guest code addresses, which is important to understand.
   //
   // VgCallBackClosure* closure contains extra arguments passed
   // from Valgrind to the instrumenter, which Vex doesn't know about.
   // You are free to look inside this structure.
   //
   // * closure->tid is the ThreadId of the thread requesting the
   //   translation.  Not sure why this is here; perhaps callgrind
   //   uses it.
   //
   // * closure->nraddr is the non-redirected guest address of the
   //   start of the translation.  In other words, the translation is
   //   being constructed because the guest program jumped to 
   //   closure->nraddr but no translation of it was found.
   //
   // * closure->readdr is the redirected guest address, from which
   //   the translation was really made.
   //
   //   To clarify this, consider what happens when, in Memcheck, the
   //   first call to malloc() happens.  The guest program will be
   //   trying to jump to malloc() in libc; hence ->nraddr will contain
   //   that address.  However, Memcheck intercepts and replaces
   //   malloc, hence ->readdr will be the address of Memcheck's
   //   malloc replacement in
   //   coregrind/m_replacemalloc/vg_replacemalloc.c.  It follows
   //   that the first IMark in the translation will be labelled as
   //   from ->readdr rather than ->nraddr.
   //
   //   Since most functions are not redirected, the majority of the
   //   time ->nraddr will be the same as ->readdr.  However, you
   //   cannot assume this: if your tool has metadata associated
   //   with code addresses it will get into deep trouble if it does
   //   make this assumption.
   //
   // IRSB* sb_in is the incoming superblock to be instrumented,
   // in flat IR form.
   //
   // VexGuestLayout* layout contains limited info on the layout of
   // the guest state: where the stack pointer and program counter
   // are, and which fields should be regarded as 'always defined'.
   // Memcheck uses this.
   //
   // VexGuestExtents* vge points to a structure which states the
   // precise byte ranges of original code from which this translation
   // was made (there may be up to three different ranges involved).
   // Note again that these are the real addresses from which the code
   // came.  And so it should be the case that closure->readdr is the
   // same as vge->base[0]; indeed Cachegrind contains this assertion.
   //
   // Tools which associate shadow data with code addresses
   // (cachegrind, callgrind) need to be particularly clear about
   // whether they are making the association with redirected or
   // non-redirected code addresses.  Both approaches are viable
   // but you do need to understand what's going on.  See comments
   // below on discard_basic_block_info().
   //
   // IRType gWordTy and IRType hWordTy contain the types of native
   // words on the guest (simulated) and host (real) CPUs.  They will
   // by either Ity_I32 or Ity_I64.  So far we have never built a
   // cross-architecture Valgrind so they should always be the same.
   //
   /* --- Further comments about the IR that your --- */
   /* --- instrumentation function will receive. --- */
   /*
      In the incoming IRSB, the IR for each instruction begins with an
      IRStmt_IMark, which states the address and length of the
      instruction from which this IR came.  This makes it easy for
      profiling-style tools to know precisely which guest code
      addresses are being executed.

      However, before the first IRStmt_IMark, there may be other IR
      statements -- a preamble.  In most cases this preamble is empty,
      but when it isn't, what it contains is some supporting IR that
      the JIT uses to ensure control flow works correctly.  This
      preamble does not modify any architecturally defined guest state
      (registers or memory) and so does not contain anything that will
      be of interest to your tool.

      You should therefore 

      (1) copy any IR preceding the first IMark verbatim to the start
          of the output IRSB.

      (2) not try to instrument it or modify it in any way.

      For the record, stuff that may be in the preamble at
      present is:

      - A self-modifying-code check has been requested for this block.
        The preamble will contain instructions to checksum the block,
        compare against the expected value, and exit the dispatcher
        requesting a discard (hence forcing a retranslation) if they
        don't match.

      - This block is known to be the entry point of a wrapper of some
        function F.  In this case the preamble contains code to write
        the address of the original F (the fn being wrapped) into a
        'hidden' guest state register _NRADDR.  The wrapper can later
        read this register using a client request and make a
        non-redirected call to it using another client-request-like
        magic macro.

      - For platforms that use the AIX ABI (including ppc64-linux), it
        is necessary to have a preamble even for replacement functions
        (not just for wrappers), because it is necessary to switch the
        R2 register (constant-pool pointer) to a different value when
        swizzling the program counter.

        Hence the preamble pushes both R2 and LR (the return address)
        on a small 16-entry stack in the guest state and sets R2 to an
        appropriate value for the wrapper/replacement fn.  LR is then
        set so that the wrapper/replacement fn returns to a magic IR
        stub which restores R2 and LR and returns.

        It's all hugely ugly and fragile.  And it places a stringent
        requirement on m_debuginfo to find out the correct R2 (toc
        pointer) value for the wrapper/replacement function.  So much
        so that m_redir will refuse to honour a redirect-to-me request
        if it cannot find (by asking m_debuginfo) a plausible R2 value
        for 'me'.

        Because this mechanism maintains a shadow stack of (R2,LR)
        pairs in the guest state, it will fail if the
        wrapper/redirection function, or anything it calls, longjumps
        out past the wrapper, because then the magic return stub will
        not be run and so the shadow stack will not be popped.  So it
        will quickly fill up.  Fortunately none of this applies to
        {x86,amd64,ppc32}-linux; on those platforms, wrappers can
        longjump and recurse arbitrarily and everything should work
        fine.

      Note that copying the preamble verbatim may cause complications
      for your instrumenter if you shadow IR temporaries.  See big
      comment in MC_(instrument) in memcheck/mc_translate.c for
      details.
   */
   IRSB*(*instrument)(VgCallbackClosure* closure, 
                      IRSB*              sb_in, 
                      const VexGuestLayout*  layout, 
                      const VexGuestExtents* vge, 
                      const VexArchInfo*     archinfo_host,
                      IRType             gWordTy, 
                      IRType             hWordTy),

   // Finish up, print out any results, etc.  `exitcode' is program's exit
   // code.  The shadow can be found with VG_(get_exit_status_shadow)().
   void  (*fini)(Int)
);

/* ------------------------------------------------------------------ */
/* Details */

/* Default value for avg_translations_sizeB (in bytes), indicating typical
   code expansion of about 6:1. */
#define VG_DEFAULT_TRANS_SIZEB   172

/* Information used in the startup message.  `name' also determines the
   string used for identifying suppressions in a suppression file as
   belonging to this tool.  `version' can be NULL, in which case (not
   surprisingly) no version info is printed; this mechanism is designed for
   tools distributed with Valgrind that share a version number with
   Valgrind.  Other tools not distributed as part of Valgrind should
   probably have their own version number.  */
extern void VG_(details_name)                  ( const HChar* name );
extern void VG_(details_version)               ( const HChar* version );
extern void VG_(details_description)           ( const HChar* description );
extern void VG_(details_copyright_author)      ( const HChar* copyright_author );

/* Average size of a translation, in bytes, so that the translation
   storage machinery can allocate memory appropriately.  Not critical,
   setting is optional. */
extern void VG_(details_avg_translation_sizeB) ( UInt size );

/* String printed if an `tl_assert' assertion fails or VG_(tool_panic)
   is called.  Should probably be an email address. */
extern void VG_(details_bug_reports_to)   ( const HChar* bug_reports_to );

/* ------------------------------------------------------------------ */
/* Needs */

/* Should __libc_freeres() be run?  Bugs in it can crash the tool. */
extern void VG_(needs_libc_freeres) ( void );

/* Should __gnu_cxx::__freeres() be run?  Bugs in it can crash the tool. */
extern void VG_(needs_cxx_freeres) ( void );

/* Want to have errors detected by Valgrind's core reported?  Includes:
   - pthread API errors (many;  eg. unlocking a non-locked mutex) 
     [currently disabled]
   - invalid file descriptors to syscalls like read() and write()
   - bad signal numbers passed to sigaction()
   - attempt to install signal handler for SIGKILL or SIGSTOP */
extern void VG_(needs_core_errors) ( void );

/* Booleans that indicate extra operations are defined;  if these are True,
   the corresponding template functions (given below) must be defined.  A
   lot like being a member of a type class. */

/* Want to report errors from tool?  This implies use of suppressions, too. */
extern void VG_(needs_tool_errors) (
   // Identify if two errors are equal, or close enough.  This function is
   // only called if e1 and e2 will have the same error kind.  `res' indicates
   // how close is "close enough".  `res' should be passed on as necessary,
   // eg. if the Error's `extra' part contains an ExeContext, `res' should be
   // passed to VG_(eq_ExeContext)() if the ExeContexts are considered.  Other
   // than that, probably don't worry about it unless you have lots of very
   // similar errors occurring.
   Bool (*eq_Error)(VgRes res, const Error* e1, const Error* e2),

   // We give tools a chance to have a look at errors
   // just before they are printed.  That is, before_pp_Error is 
   // called just before pp_Error itself.  This gives the tool a
   // chance to look at the just-about-to-be-printed error, so as to 
   // emit any arbitrary output if wants to, before the error itself
   // is printed.  This functionality was added to allow Helgrind to
   // print thread-announcement messages immediately before the 
   // errors that refer to them.
   void (*before_pp_Error)(const Error* err),

   // Print error context.
   void (*pp_Error)(const Error* err),

   // Should the core indicate which ThreadId each error comes from?
   Bool show_ThreadIDs_for_errors,

   // Should fill in any details that could be postponed until after the
   // decision whether to ignore the error (ie. details not affecting the
   // result of VG_(tdict).tool_eq_Error()).  This saves time when errors
   // are ignored.
   // Yuk.
   // Return value: must be the size of the `extra' part in bytes -- used by
   // the core to make a copy.
   UInt (*update_extra)(const Error* err),

   // Return value indicates recognition.  If recognised, must set skind using
   // VG_(set_supp_kind)().
   Bool (*recognised_suppression)(const HChar* name, Supp* su),

   // Read any extra info for this suppression kind.  Most likely for filling
   // in the `extra' and `string' parts (with VG_(set_supp_{extra, string})())
   // of a suppression if necessary.  Should return False if a syntax error
   // occurred, True otherwise.
   // fd, bufpp, nBufp and lineno are the same as for VG_(get_line).
   Bool (*read_extra_suppression_info)(Int fd, HChar** bufpp, SizeT* nBufp,
                                       Int* lineno, Supp* su),

   // This should just check the kinds match and maybe some stuff in the
   // `string' and `extra' field if appropriate (using VG_(get_supp_*)() to
   // get the relevant suppression parts).
   Bool (*error_matches_suppression)(const Error* err, const Supp* su),

   // This should return the suppression name, for --gen-suppressions, or NULL
   // if that error type cannot be suppressed.  This is the inverse of
   // VG_(tdict).tool_recognised_suppression().
   const HChar* (*get_error_name)(const Error* err),

   // This should print into buf[0..nBuf-1] any extra info for the
   // error, for --gen-suppressions, but not including any leading
   // spaces nor a trailing newline.  The string needs to be null 
   // terminated. If the buffer is large enough to hold the string
   // including the terminating null character the function shall
   // return the value that strlen would return for the string.
   // If the buffer is too small the function shall return nBuf.
   SizeT (*print_extra_suppression_info)(const Error* err,
                                         /*OUT*/HChar* buf, Int nBuf),

   // This is similar to print_extra_suppression_info, but is used
   // to print information such as additional statistical counters
   // as part of the used suppression list produced by -v.
   SizeT (*print_extra_suppression_use)(const Supp* su,
                                        /*OUT*/HChar* buf, Int nBuf),

   // Called by error mgr once it has been established that err
   // is suppressed by su. update_extra_suppression_use typically
   // can be used to update suppression extra information such as
   // some statistical counters that will be printed by
   // print_extra_suppression_use.
   void (*update_extra_suppression_use)(const Error* err, const Supp* su)
);

/* Is information kept by the tool about specific instructions or
   translations?  (Eg. for cachegrind there are cost-centres for every
   instruction, stored in a per-translation fashion.)  If so, the info
   may have to be discarded when translations are unloaded (eg. due to
   .so unloading, or otherwise at the discretion of m_transtab, eg
   when the table becomes too full) to avoid stale information being
   reused for new translations. */
extern void VG_(needs_superblock_discards) (
   // Discard any information that pertains to specific translations
   // or instructions within the address range given.  There are two
   // possible approaches.
   // - If info is being stored at a per-translation level, use orig_addr
   //   to identify which translation is being discarded.  Each translation
   //   will be discarded exactly once.
   //   This orig_addr will match the closure->nraddr which was passed to
   //   to instrument() (see extensive comments above) when this 
   //   translation was made.  Note that orig_addr won't necessarily be 
   //   the same as the first address in "extents".
   // - If info is being stored at a per-instruction level, you can get
   //   the address range(s) being discarded by stepping through "extents".
   //   Note that any single instruction may belong to more than one
   //   translation, and so could be covered by the "extents" of more than
   //   one call to this function.
   // Doing it the first way (as eg. Cachegrind does) is probably easier.
   void (*discard_superblock_info)(Addr orig_addr, VexGuestExtents extents)
);

/* Tool defines its own command line options? */
extern void VG_(needs_command_line_options) (
   // Return True if option was recognised, False if it wasn't (but also see
   // below).  Presumably sets some state to record the option as well.  
   //
   // Nb: tools can assume that the argv will never disappear.  So they can,
   // for example, store a pointer to a string within an option, rather than
   // having to make a copy.
   //
   // Options (and combinations of options) should be checked in this function
   // if possible rather than in post_clo_init(), and if they are bad then
   // VG_(fmsg_bad_option)() should be called.  This ensures that the
   // messaging is consistent with command line option errors from the core.
   Bool (*process_cmd_line_option)(const HChar* argv),

   // Print out command line usage for options for normal tool operation.
   void (*print_usage)(void),

   // Print out command line usage for options for debugging the tool.
   void (*print_debug_usage)(void)
);

/* Tool defines its own client requests? */
extern void VG_(needs_client_requests) (
   // If using client requests, the number of the first request should be equal
   // to VG_USERREQ_TOOL_BASE('X', 'Y'), where 'X' and 'Y' form a suitable two
   // character identification for the string.  The second and subsequent
   // requests should follow.
   //
   // This function should use the VG_IS_TOOL_USERREQ macro (in
   // include/valgrind.h) to first check if it's a request for this tool.  Then
   // should handle it if it's recognised (and return True), or return False if
   // not recognised.  arg_block[0] holds the request number, any further args
   // from the request are in arg_block[1..].  'ret' is for the return value...
   // it should probably be filled, if only with 0.
   Bool (*handle_client_request)(ThreadId tid, UWord* arg_block, UWord* ret)
);

/* Tool does stuff before and/or after system calls? */
// Nb: If either of the pre_ functions malloc() something to return, the
// corresponding post_ function had better free() it!
// Also, the args are the 'original args' -- that is, it may be
// that the syscall pre-wrapper will modify the args before the
// syscall happens.  So these args are the original, un-modified
// args.  Finally, nArgs merely indicates the length of args[..],
// it does not indicate how many of those values are actually
// relevant to the syscall.  args[0 .. nArgs-1] is guaranteed
// to be defined and to contain all the args for this syscall,
// possibly including some trailing zeroes.
extern void VG_(needs_syscall_wrapper) (
               void (* pre_syscall)(ThreadId tid, UInt syscallno,
                                    UWord* args, UInt nArgs),
               void (*post_syscall)(ThreadId tid, UInt syscallno,
                                    UWord* args, UInt nArgs, SysRes res)
);

/* Are tool-state sanity checks performed? */
// Can be useful for ensuring a tool's correctness.  cheap_sanity_check()
// is called very frequently;  expensive_sanity_check() is called less
// frequently and can be more involved.
extern void VG_(needs_sanity_checks) (
   Bool(*cheap_sanity_check)(void),
   Bool(*expensive_sanity_check)(void)
);

/* Can the tool produce stats during execution? */
extern void VG_(needs_print_stats) (
   // Print out tool status. Note that the stats at end of execution
   // should be output by the VG_(basic_tool_funcs) "fini" function.
   void (*print_stats)(void)
);

/* Has the tool a tool specific function to retrieve and print location info
   of an address ? */
extern void VG_(needs_info_location) (
   // Get and pp information about Addr
   void (*info_location)(DiEpoch, Addr)
);

/* Do we need to see variable type and location information? */
extern void VG_(needs_var_info) ( void );

/* Does the tool replace malloc() and friends with its own versions?
   This has to be combined with the use of a vgpreload_<tool>.so module
   or it won't work.  See massif/Makefile.am for how to build it. */
// The 'p' prefix avoids GCC complaints about overshadowing global names.
extern void VG_(needs_malloc_replacement)(
   void* (*pmalloc)               ( ThreadId tid, SizeT n ),
   void* (*p__builtin_new)        ( ThreadId tid, SizeT n ),
   void* (*p__builtin_vec_new)    ( ThreadId tid, SizeT n ),
   void* (*pmemalign)             ( ThreadId tid, SizeT align, SizeT n ),
   void* (*pcalloc)               ( ThreadId tid, SizeT nmemb, SizeT size1 ),
   void  (*pfree)                 ( ThreadId tid, void* p ),
   void  (*p__builtin_delete)     ( ThreadId tid, void* p ),
   void  (*p__builtin_vec_delete) ( ThreadId tid, void* p ),
   void* (*prealloc)              ( ThreadId tid, void* p, SizeT new_size ),
   SizeT (*pmalloc_usable_size)   ( ThreadId tid, void* p), 
   SizeT client_malloc_redzone_szB
);

/* Can the tool do XML output?  This is a slight misnomer, because the tool
 * is not requesting the core to do anything, rather saying "I can handle
 * it". */
extern void VG_(needs_xml_output) ( void );

/* Does the tool want to have one final pass over the IR after tree
   building but before instruction selection?  If so specify the
   function here. */
extern void VG_(needs_final_IR_tidy_pass) ( IRSB*(*final_tidy)(IRSB*) );


/* ------------------------------------------------------------------ */
/* Core events to track */

/* Part of the core from which this call was made.  Useful for determining
   what kind of error message should be emitted. */
typedef
   enum { Vg_CoreStartup=1, Vg_CoreSignal, Vg_CoreSysCall,
          // This is for platforms where syscall args are passed on the
          // stack; although pre_mem_read is the callback that will be
          // called, such an arg should be treated (with respect to
          // presenting information to the user) as if it was passed in a
          // register, ie. like pre_reg_read.
          Vg_CoreSysCallArgInMem,  
          Vg_CoreTranslate, Vg_CoreClientReq
   } CorePart;

/* Events happening in core to track.  To be notified, pass a callback
   function to the appropriate function.  To ignore an event, don't do
   anything (the default is for events to be ignored).

   Note that most events aren't passed a ThreadId.  If the event is one called
   from generated code (eg. new_mem_stack_*), you can use
   VG_(get_running_tid)() to find it.  Otherwise, it has to be passed in,
   as in pre_mem_read, and so the event signature will require changing.

   Memory events (Nb: to track heap allocation/freeing, a tool must replace
   malloc() et al.  See above how to do this.)

   These ones occur at startup, upon some signals, and upon some syscalls.

   For new_mem_brk and new_mem_stack_signal, the supplied ThreadId
   indicates the thread for whom the new memory is being allocated.

   For new_mem_startup and new_mem_mmap, the di_handle argument is a
   handle which can be used to retrieve debug info associated with the
   mapping or allocation (because it is of a file that Valgrind has
   decided to read debug info from).  If the value is zero, there is
   no associated debug info.  If the value exceeds zero, it can be
   supplied as an argument to selected queries in m_debuginfo.
*/
void VG_(track_new_mem_startup)     (void(*f)(Addr a, SizeT len,
                                              Bool rr, Bool ww, Bool xx,
                                              ULong di_handle));
void VG_(track_new_mem_stack_signal)(void(*f)(Addr a, SizeT len, ThreadId tid));
void VG_(track_new_mem_brk)         (void(*f)(Addr a, SizeT len, ThreadId tid));
void VG_(track_new_mem_mmap)        (void(*f)(Addr a, SizeT len,
                                              Bool rr, Bool ww, Bool xx,
                                              ULong di_handle));

void VG_(track_copy_mem_remap)      (void(*f)(Addr from, Addr to, SizeT len));
void VG_(track_change_mem_mprotect) (void(*f)(Addr a, SizeT len,
                                              Bool rr, Bool ww, Bool xx));
void VG_(track_die_mem_stack_signal)(void(*f)(Addr a, SizeT len));
void VG_(track_die_mem_brk)         (void(*f)(Addr a, SizeT len));
void VG_(track_die_mem_munmap)      (void(*f)(Addr a, SizeT len));

/* These ones are called when SP changes.  A tool could track these itself
   (except for ban_mem_stack) but it's much easier to use the core's help.

   The specialised ones are called in preference to the general one, if they
   are defined.  These functions are called a lot if they are used, so
   specialising can optimise things significantly.  If any of the
   specialised cases are defined, the general case must be defined too.

   Nb: all the specialised ones must use the VG_REGPARM(n) attribute.

   For the _new functions, a tool may specify with with-ECU
   (ExeContext Unique) or without-ECU version for each size, but not
   both.  If the with-ECU version is supplied, then the core will
   arrange to pass, as the ecu argument, a 32-bit int which uniquely
   identifies the instruction moving the stack pointer down.  This
   32-bit value is as obtained from VG_(get_ECU_from_ExeContext).
   VG_(get_ExeContext_from_ECU) can then be used to retrieve the
   associated depth-1 ExeContext for the location.  All this
   complexity is provided to support origin tracking in Memcheck.
*/
void VG_(track_new_mem_stack_4_w_ECU)  (VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_8_w_ECU)  (VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_12_w_ECU) (VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_16_w_ECU) (VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_32_w_ECU) (VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_112_w_ECU)(VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_128_w_ECU)(VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_144_w_ECU)(VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_160_w_ECU)(VG_REGPARM(2) void(*f)(Addr new_ESP, UInt ecu));
void VG_(track_new_mem_stack_w_ECU)                  (void(*f)(Addr a, SizeT len,
                                                                       UInt ecu));

void VG_(track_new_mem_stack_4)  (VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack_8)  (VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack_12) (VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack_16) (VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack_32) (VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack_112)(VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack_128)(VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack_144)(VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack_160)(VG_REGPARM(1) void(*f)(Addr new_ESP));
void VG_(track_new_mem_stack)                  (void(*f)(Addr a, SizeT len));

void VG_(track_die_mem_stack_4)  (VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack_8)  (VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack_12) (VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack_16) (VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack_32) (VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack_112)(VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack_128)(VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack_144)(VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack_160)(VG_REGPARM(1) void(*f)(Addr die_ESP));
void VG_(track_die_mem_stack)                  (void(*f)(Addr a, SizeT len));

/* Used for redzone at end of thread stacks */
void VG_(track_ban_mem_stack)      (void(*f)(Addr a, SizeT len));

/* These ones occur around syscalls, signal handling, etc */
void VG_(track_pre_mem_read)       (void(*f)(CorePart part, ThreadId tid,
                                             const HChar* s, Addr a, SizeT size));
void VG_(track_pre_mem_read_asciiz)(void(*f)(CorePart part, ThreadId tid,
                                             const HChar* s, Addr a));
void VG_(track_pre_mem_write)      (void(*f)(CorePart part, ThreadId tid,
                                             const HChar* s, Addr a, SizeT size));
void VG_(track_post_mem_write)     (void(*f)(CorePart part, ThreadId tid,
                                             Addr a, SizeT size));

/* Register events.  Use VG_(set_shadow_state_area)() to set the shadow regs
   for these events.  */
void VG_(track_pre_reg_read)  (void(*f)(CorePart part, ThreadId tid,
                                        const HChar* s, PtrdiffT guest_state_offset,
                                        SizeT size));
void VG_(track_post_reg_write)(void(*f)(CorePart part, ThreadId tid,
                                        PtrdiffT guest_state_offset,
                                        SizeT size));

/* This one is called for malloc() et al if they are replaced by a tool. */
void VG_(track_post_reg_write_clientcall_return)(
      void(*f)(ThreadId tid, PtrdiffT guest_state_offset, SizeT size, Addr f));

/* Mem-to-reg or reg-to-mem copy functions, these ones occur around syscalls
   and signal handling when the VCPU state is saved to (or restored from) the
   client memory. */
void VG_(track_copy_mem_to_reg)(void(*f)(CorePart part, ThreadId tid,
                                         Addr a, PtrdiffT guest_state_offset,
                                         SizeT size));
void VG_(track_copy_reg_to_mem)(void(*f)(CorePart part, ThreadId tid,
                                         PtrdiffT guest_state_offset,
                                         Addr a, SizeT size));


/* Scheduler events (not exhaustive) */

/* Called when 'tid' starts or stops running client code blocks.
   Gives the total dispatched block count at that event.  Note, this
   is not the same as 'tid' holding the BigLock (the lock that ensures
   that only one thread runs at a time): a thread can hold the lock
   for other purposes (making translations, etc) yet not be running
   client blocks.  Obviously though, a thread must hold the lock in
   order to run client code blocks, so the times bracketed by
   'start_client_code'..'stop_client_code' are a subset of the times
   when thread 'tid' holds the cpu lock.
*/
void VG_(track_start_client_code)(
        void(*f)(ThreadId tid, ULong blocks_dispatched)
     );
void VG_(track_stop_client_code)(
        void(*f)(ThreadId tid, ULong blocks_dispatched)
     );


/* Thread events (not exhaustive)

   ll_create: low level thread creation.  Called before the new thread
   has run any instructions (or touched any memory).  In fact, called
   immediately before the new thread has come into existence; the new
   thread can be assumed to exist when notified by this call.

   ll_exit: low level thread exit.  Called after the exiting thread
   has run its last instruction.

   The _ll_ part makes it clear these events are not to do with
   pthread_create or pthread_exit/pthread_join (etc), which are a
   higher level abstraction synthesised by libpthread.  What you can
   be sure of from _ll_create/_ll_exit is the absolute limits of each
   thread's lifetime, and hence be assured that all memory references
   made by the thread fall inside the _ll_create/_ll_exit pair.  This
   is important for tools that need a 100% accurate account of which
   thread is responsible for every memory reference in the process.

   pthread_create/join/exit do not give this property.  Calls/returns
   to/from them happen arbitrarily far away from the relevant
   low-level thread create/quit event.  In general a few hundred
   instructions; hence a few hundred(ish) memory references could get
   misclassified each time.

   pre_thread_first_insn: is called when the thread is all set up and
   ready to go (stack in place, etc) but has not executed its first
   instruction yet.  Gives threading tools a chance to ask questions
   about the thread (eg, what is its initial client stack pointer)
   that are not easily answered at pre_thread_ll_create time.

   For a given thread, the call sequence is:
      ll_create (in the parent's context)
      first_insn (in the child's context)
      ll_exit (in the child's context)
*/
void VG_(track_pre_thread_ll_create) (void(*f)(ThreadId tid, ThreadId child));
void VG_(track_pre_thread_first_insn)(void(*f)(ThreadId tid));
void VG_(track_pre_thread_ll_exit)   (void(*f)(ThreadId tid));


/* Signal events (not exhaustive)

   ... pre_send_signal, post_send_signal ...

   Called before a signal is delivered;  `alt_stack' indicates if it is
   delivered on an alternative stack.  */
void VG_(track_pre_deliver_signal) (void(*f)(ThreadId tid, Int sigNo,
                                             Bool alt_stack));
/* Called after a signal is delivered.  Nb: unfortunately, if the signal
   handler longjmps, this won't be called.  */
void VG_(track_post_deliver_signal)(void(*f)(ThreadId tid, Int sigNo));

#endif   // __PUB_TOOL_TOOLIFACE_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
