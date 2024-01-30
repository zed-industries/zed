/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * alloc.h: enumeration of alloc IDs.
 * Used by test_alloc_fail() to test memory allocation failures.
 * Each entry must be on exactly one line, GetAllocId() depends on that.
 */
typedef enum {
    aid_none = 0,
    aid_qf_dirname_start,
    aid_qf_dirname_now,
    aid_qf_namebuf,
    aid_qf_module,
    aid_qf_errmsg,
    aid_qf_pattern,
    aid_qf_efm_fmtstr,
    aid_qf_efm_fmtpart,
    aid_qf_title,
    aid_qf_mef_name,
    aid_qf_qfline,
    aid_qf_qfinfo,
    aid_qf_dirstack,
    aid_qf_multiline_pfx,
    aid_qf_makecmd,
    aid_qf_linebuf,
    aid_tagstack_items,
    aid_tagstack_from,
    aid_tagstack_details,
    aid_sign_getdefined,
    aid_sign_getplaced,
    aid_sign_define_by_name,
    aid_sign_getlist,
    aid_sign_getplaced_dict,
    aid_sign_getplaced_list,
    aid_insert_sign,
    aid_sign_getinfo,
    aid_newbuf_bvars,
    aid_newwin_wvars,
    aid_newtabpage_tvars,
    aid_blob_alloc,
    aid_get_func,
    aid_last
} alloc_id_T;
