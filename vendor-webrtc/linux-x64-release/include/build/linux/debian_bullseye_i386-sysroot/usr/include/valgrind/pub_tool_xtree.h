
/*--------------------------------------------------------------------*/
/*--- An xtree, tree of stacktraces with data     pub_tool_xtree.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2015-2017 Philippe Waroquiers

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

#ifndef __PUB_TOOL_XTREE_H
#define __PUB_TOOL_XTREE_H

#include "pub_tool_basics.h"
#include "pub_tool_execontext.h"

//--------------------------------------------------------------------
// PURPOSE: an XTree is conceptually a set of stacktraces organised
// as a tree structure. 
// A stacktrace (an Addr* ips, i.e. an array of IPs : Instruction Pointers)
// can be added to the tree once transformed into an execontext (ec).
// Some data (typically one or more integer values) can be attached to
// leafs of the tree.
// Non-leaf nodes data is build by combining (typically adding together)
// the data of their children nodes.
// An XTree can be output in various formats.
//
//--------------------------------------------------------------------


/* It's an abstract type. */
typedef struct _XTree  XTree;

/* 3 functions types used by an xtree to manipulate the data attached to leafs
   of an XTree.
   XT_init_data_t function is used to initialise (typically to 0) the data
   of a new node.
   XT_add_data_t function is used to add 'value' to the data 'to'.
   XT_sub_data_t function is used to substract 'value' from the data 'from'.

   Note that the add/sub functions can do whatever operations to
   combine/integrate value with/into to or from. In other words, add/sub
   functions are in fact equivalent to 'Reduce' functions. Add/sub is used
   as it is assumed that this module will be mostly used to follow
   resource consumption, which can be more clearly modelled with add/sub.
   For such resource consumption usage, typically, a call to add means that
   some additional resource has been allocated or consumed or ... by the
   given ExeContext. Similarly, a call to sub means that some resource
   has been released/freed/... by the given execontext.

   Note however that there is no constraints in what add (or sub) can do. For
   example, the add function could maintain Min/Max values, or an histogram of
   values, or ... */
typedef void (*XT_init_data_t) (void* value);
typedef void (*XT_add_data_t) (void* to,   const void* value);
typedef void (*XT_sub_data_t) (void* from, const void* value);

/* If not NULL, the XT_filter_IPs_t function is called when a new ec is inserted
   in the XTree. 
   It indicates to the XTree to filter a range of IPs at the top and/or at
   the bottom of the ec Stacktrace : *top is the offset of the first IP to take
   into account. *n_ips_sel is the nr of IPs selected starting from *top.

   If XT_filter_IPs_t gives *n_ips_sel equal to 0, then the inserted ec will
   be fully ignored when outputting the xtree: 
     the ec value(s) will not be counted in the XTree total,
     the ec will not be printed/shown.
   Note however that the filtering only influences the output of an XTree :
     the ec is still inserted in the XTree, and the XT_*_data_t functions are
     called in any case for such filtered ec. */
typedef void (*XT_filter_IPs_t) (Addr* ips, Int n_ips,
                                 UInt* top, UInt* n_ips_sel);

/* Create new XTree, using given allocation and free function.
   This function never returns NULL.
   cc is the allocation cost centre.
   alloc_fn must not return NULL (that is, if it returns it must have
   succeeded.).
   See respective typedef for *_fn arguments. */
extern XTree* VG_(XT_create) ( Alloc_Fn_t alloc_fn,
                               const HChar* cc,
                               Free_Fn_t free_fn,
                               Word dataSzB,
                               XT_init_data_t init_data_fn,
                               XT_add_data_t add_data_fn,
                               XT_sub_data_t sub_data_fn,
                               XT_filter_IPs_t filter_IPs_fn);


/* General useful filtering functions. */

/* Filter functions below main, unless VG_(clo_show_below_main) is True. */
extern void VG_(XT_filter_maybe_below_main)
     (Addr* ips, Int n_ips,
      UInt* top, UInt* n_ips_sel);
/* Same as VG_(XT_filter_maybe_below_main) but also filters one top function
   (typically to ignore the top level malloc/new/... fn). */
extern void VG_(XT_filter_1top_and_maybe_below_main)
     (Addr* ips, Int n_ips,
      UInt* top, UInt* n_ips_sel);

/* Search in ips[0..n_ips-1] the first function which is main or below main
   and return its offset.
   If no main or below main is found, return n_ips-1 */
extern Int VG_(XT_offset_main_or_below_main)(DiEpoch ep, Addr* ips, Int n_ips);


/* Take a (frozen) snapshot of xt.
   Note that the resulting XTree is 'read-only' : calls to 
   VG_(XT_add_to_*)/VG_(XT_sub_from_*) will assert.

   Note: to spare memory, some data is shared between an xt and all its
   snapshots. This memory is released when the last XTree using this memory
   is deleted. */
extern XTree* VG_(XT_snapshot)(XTree* xt);

/*  Non frozen dup currently not needed : 
    extern XTree* VG_(XT_dup)(XTree* xt); */

/* Free all memory associated with an XTRee. */
extern void VG_(XT_delete)(XTree* xt);

/* an Xecu identifies an exe context+its associated data in an XTree. */
typedef UInt Xecu;

/* If not yet in xt, inserts the provided ec and initialises its
   data by calling init_data_fn.
   If already present (or after insertion), updates the data by calling
   add_data_fn. */
extern Xecu VG_(XT_add_to_ec)(XTree* xt, ExeContext* ec, const void* value);

/* If not yet in xt, inserts the provided ec and initialises its
   data by calling init_data_fn.
   If already present (or after insertion), updates the data by calling
   sub_data_fn to substract value from the data associated to ec. */
extern Xecu VG_(XT_sub_from_ec)(XTree* xt, ExeContext* ec, const void* value);

/* Same as (but more efficient than) VG_(XT_add_to_ec) and VG_(XT_sub_from_ec)
   for an ec already inserted in xt. */
extern void VG_(XT_add_to_xecu)(XTree* xt, Xecu xecu, const void* value);
extern void VG_(XT_sub_from_xecu)(XTree* xt, Xecu xecu, const void* value);

/* Return the nr of IPs selected for xecu. 0 means fully filtered. */
extern UInt VG_(XT_n_ips_sel)(XTree* xt, Xecu xecu);

/* Return the ExeContext associated to the Xecu. */
extern ExeContext* VG_(XT_get_ec_from_xecu) (XTree* xt, Xecu xecu);

/* -------------------- CALLGRIND/KCACHEGRIND OUTPUT FORMAT --------------*/
/* Prints xt in outfilename in callgrind/kcachegrind format.
   events is a comma separated list of events, used by 
   kcachegrind/callgrind_annotate/... to name the value various components.
   An event can optionally have a longer description, separated from the
   event name by " : ", e.g.
   "curB : currently allocated Bytes,curBk : Currently allocated Blocks"
   img_value returns an image of the value. The image must be a space
   separated set of integers, matching the corresponding event in events.
   Note that the returned pointer can be static data. 
   img_value can return NULL if value (and its associated ExeContext) should
   not be printed.
*/
extern void VG_(XT_callgrind_print) 
     (XTree* xt,
      const HChar* outfilename,
      const HChar* events,
      const HChar* (*img_value) (const void* value));


/* -------------------- MASSIF OUTPUT FORMAT --------------*/
// Time is measured either in i or ms or bytes, depending on the --time-unit
// option.  It's a Long because it can exceed 32-bits reasonably easily, and
// because we need to allow negative values to represent unset times.
typedef Long Time;

typedef void MsFile;

/* Create a new file or truncate existing file for printing xtrees in
   massif format. time_unit is a string describing the unit used
   in Massif_Header time.
   Produces a user error msg and returns NULL if file cannot be opened.
   Caller must VG_(XT_massif_close) the returned file. */
extern MsFile* VG_(XT_massif_open)(const HChar* outfilename,
                                   const HChar* desc, // can be NULL
                                   const XArray* desc_args, // can be NULL
                                   const HChar* time_unit);

extern void VG_(XT_massif_close)(MsFile* fp);

typedef 
   struct {
      int snapshot_n; // starting at 0.
      Time time;

      ULong sz_B;     // sum of values, only used when printing a NULL xt.
      ULong extra_B;
      ULong stacks_B;

      Bool detailed;
      Bool peak;

      /*   top_node_desc: description for the top node.
           Typically for memory usage, give xt_heap_alloc_functions_desc. */
      const HChar* top_node_desc;

      /* children with less than sig_threshold * total xt sz will be aggregated
         and printed as one single child. */
      double sig_threshold;

   } Massif_Header;

/* Prints xt in outfilename in massif format.
   If a NULL xt is provided, then only the header information is used
   to produce the (necessarily not detailed) snapshot.
   report_value must return the value to be used for the report production.
   It will typically be the nr of bytes allocated stored with the execontext
   but it could be anything measured with a ULong (e.g. the nr of blocks
   allocated, or a number of calls, ...).
*/
extern void VG_(XT_massif_print)
     (MsFile* fp,
      XTree* xt,
      const Massif_Header* header,
      ULong (*report_value)(const void* value));

#endif   // __PUB_TOOL_XTREE_H

/*--------------------------------------------------------------------*/
/*--- end                                         pub_tool_xtree.h ---*/
/*--------------------------------------------------------------------*/
