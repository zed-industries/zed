/**
 * @file YAML grammar for tree-sitter
 * @author Ika <ikatyang@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />

module.exports = grammar({
  name: "yaml",

  externals: $ => [
    $._eof,

    // s  = starts at column 0 in the current or following row
    // r  = starts with 0 or more s_white in the current row
    // br = starts with more than `current_indent` s_white in the following row
    // b  = starts with `current_indent` s_white in the following row
    // bl = starts with `parent_indent` or less s_white in the following row
    $._s_dir_yml_bgn,  $._r_dir_yml_ver,                    // %YAML 1.2
    $._s_dir_tag_bgn,  $._r_dir_tag_hdl,  $._r_dir_tag_pfx, // %TAG !yaml! tag:yaml.org,2002:
    $._s_dir_rsv_bgn,  $._r_dir_rsv_prm,                    // %FOO bar baz
    $._s_drs_end,                                           // ---
    $._s_doc_end,                                           // ...
    $._r_blk_seq_bgn,  $._br_blk_seq_bgn, $._b_blk_seq_bgn, // -
    $._r_blk_key_bgn,  $._br_blk_key_bgn, $._b_blk_key_bgn, // ?
    $._r_blk_val_bgn,  $._br_blk_val_bgn, $._b_blk_val_bgn, // :
    $._r_blk_imp_bgn,                                       // : (implicit)
    $._r_blk_lit_bgn,  $._br_blk_lit_bgn,                   // |
    $._r_blk_fld_bgn,  $._br_blk_fld_bgn,                   // >
                       $._br_blk_str_ctn,                   // block scalar content
    $._r_flw_seq_bgn,  $._br_flw_seq_bgn, $._b_flw_seq_bgn, // [
    $._r_flw_seq_end,  $._br_flw_seq_end, $._b_flw_seq_end, // ]
    $._r_flw_map_bgn,  $._br_flw_map_bgn, $._b_flw_map_bgn, // {
    $._r_flw_map_end,  $._br_flw_map_end, $._b_flw_map_end, // }
    $._r_flw_sep_bgn,  $._br_flw_sep_bgn,                   // ,
    $._r_flw_key_bgn,  $._br_flw_key_bgn,                   // ?
    $._r_flw_jsv_bgn,  $._br_flw_jsv_bgn,                   // : (json key)
    $._r_flw_njv_bgn,  $._br_flw_njv_bgn,                   // : (non-json key)
    $._r_dqt_str_bgn,  $._br_dqt_str_bgn, $._b_dqt_str_bgn, // " (start)
    $._r_dqt_str_ctn,  $._br_dqt_str_ctn,                   // double quote scalar content
    $._r_dqt_esc_nwl,  $._br_dqt_esc_nwl,                   // escape newline
    $._r_dqt_esc_seq,  $._br_dqt_esc_seq,                   // escape sequence
    $._r_dqt_str_end,  $._br_dqt_str_end,                   // " (end)
    $._r_sqt_str_bgn,  $._br_sqt_str_bgn, $._b_sqt_str_bgn, // ' (start)
    $._r_sqt_str_ctn,  $._br_sqt_str_ctn,                   // single quote scalar content
    $._r_sqt_esc_sqt,  $._br_sqt_esc_sqt,                   // ''
    $._r_sqt_str_end,  $._br_sqt_str_end,                   // ' (end)

    // plain scalar (singleline in block/flow)
    $._r_sgl_pln_nul_blk,  $._br_sgl_pln_nul_blk, $._b_sgl_pln_nul_blk, $._r_sgl_pln_nul_flw,  $._br_sgl_pln_nul_flw,
    $._r_sgl_pln_bol_blk,  $._br_sgl_pln_bol_blk, $._b_sgl_pln_bol_blk, $._r_sgl_pln_bol_flw,  $._br_sgl_pln_bol_flw,
    $._r_sgl_pln_int_blk,  $._br_sgl_pln_int_blk, $._b_sgl_pln_int_blk, $._r_sgl_pln_int_flw,  $._br_sgl_pln_int_flw,
    $._r_sgl_pln_flt_blk,  $._br_sgl_pln_flt_blk, $._b_sgl_pln_flt_blk, $._r_sgl_pln_flt_flw,  $._br_sgl_pln_flt_flw,
    $._r_sgl_pln_str_blk,  $._br_sgl_pln_str_blk, $._b_sgl_pln_str_blk, $._r_sgl_pln_str_flw,  $._br_sgl_pln_str_flw,

    // plain scalar (multiline in block/flow)
    $._r_mtl_pln_str_blk,  $._br_mtl_pln_str_blk,
    $._r_mtl_pln_str_flw,  $._br_mtl_pln_str_flw,

    $._r_tag,     $._br_tag,     $._b_tag,                   // !tag
    $._r_acr_bgn, $._br_acr_bgn, $._b_acr_bgn, $._r_acr_ctn, // &id
    $._r_als_bgn, $._br_als_bgn, $._b_als_bgn, $._r_als_ctn, // *id

    $._bl,
    $.comment,

    $._err_rec,
  ],

  extras: $ => [$.comment],

  conflicts: $ => [
    [$._r_prp, $._r_sgl_prp],
    [$._br_prp, $._br_sgl_prp],
    [$._flw_seq_tal, $._sgl_flw_seq_tal],
    [$._flw_map_tal, $._sgl_flw_map_tal],
    [$._flw_ann_par_tal, $._sgl_flw_ann_par_tal],
    [$._r_flw_seq_itm, $._r_sgl_flw_col_itm],
    [$._r_flw_map_itm, $._r_sgl_flw_col_itm],
    [$._r_flw_njl_ann_par, $._r_sgl_flw_njl_ann_par],
    [$._r_flw_exp_par, $._r_sgl_flw_exp_par],
    [$._r_dqt_str, $._r_sgl_dqt_str],
    [$._r_sqt_str, $._r_sgl_sqt_str],
    [$._r_pln_flw_val, $._r_sgl_pln_flw_val],

    /**
     * (_r_prp  _r_acr  •  _br_tag)
     *
     *    &str
     *    !!str a
     *
     * (_r_prp  _r_acr)  •  _br_tag
     *
     *    &map
     *    !!str a: 1
     */
    [$._r_prp],
    [$._br_prp],
  ],

  inline: $ => [
    $._r_pln_blk,
    $._br_pln_blk,
    $._r_pln_flw,
    $._br_pln_flw,
    $._r_blk_seq_val,
    $._r_blk_map_val,
    $._r_flw_val_blk,
    $._br_flw_val_blk,
    $._r_sgl_flw_val_blk,
    $._br_sgl_flw_val_blk,
    $._b_sgl_flw_val_blk,
    $._r_flw_val_flw,
    $._br_flw_val_flw,
    $._r_sgl_flw_val_flw,
    $._r_flw_jsl_val,
    $._br_flw_jsl_val,
    $._r_sgl_flw_jsl_val,
    $._br_sgl_flw_jsl_val,
    $._b_sgl_flw_jsl_val,
    $._r_flw_njl_val_blk,
    $._br_flw_njl_val_blk,
    $._r_sgl_flw_njl_val_blk,
    $._br_sgl_flw_njl_val_blk,
    $._b_sgl_flw_njl_val_blk,
    $._r_flw_njl_val_flw,
    $._br_flw_njl_val_flw,
    $._r_sgl_flw_njl_val_flw,
  ],

  rules: {
    stream: $ => seq(optional(choice(
        seq(
          choice($._bgn_imp_doc, $._drs_doc, $._exp_doc),
          optional(choice($._doc_w_bgn_w_end_seq, $._doc_w_bgn_wo_end_seq))),
        seq(
          choice($._bgn_imp_doc_end, $._drs_doc_end, $._exp_doc_end, $._doc_end),
          optional(choice($._doc_w_bgn_w_end_seq, $._doc_w_bgn_wo_end_seq, $._doc_wo_bgn_w_end_seq, $._doc_wo_bgn_wo_end_seq))),
    )), $._eof),

    _doc_w_bgn_w_end_seq: $ => seq($._doc_w_bgn_w_end, optional(choice($._doc_w_bgn_w_end_seq, $._doc_w_bgn_wo_end_seq, $._doc_wo_bgn_w_end_seq, $._doc_wo_bgn_wo_end_seq))),
    _doc_w_bgn_wo_end_seq: $ => seq($._doc_w_bgn_wo_end, optional(choice($._doc_w_bgn_w_end_seq, $._doc_w_bgn_wo_end_seq))),
    _doc_wo_bgn_w_end_seq: $ => seq($._doc_wo_bgn_w_end, optional(choice($._doc_w_bgn_w_end_seq, $._doc_w_bgn_wo_end_seq, $._doc_wo_bgn_w_end_seq, $._doc_wo_bgn_wo_end_seq))),
    _doc_wo_bgn_wo_end_seq: $ => seq($._doc_wo_bgn_wo_end, optional(choice($._doc_w_bgn_w_end_seq, $._doc_w_bgn_wo_end_seq))),

    _doc_w_bgn_w_end: $ => choice($._exp_doc_end, $._doc_end),
    _doc_w_bgn_wo_end: $ => $._exp_doc,
    _doc_wo_bgn_w_end: $ => choice($._drs_doc_end, $._imp_doc_end),
    _doc_wo_bgn_wo_end: $ => choice($._drs_doc, $._imp_doc),

    // document

    _bgn_imp_doc: $ => choice($._exp_doc_tal, $._r_blk_seq_r_val, $._r_blk_map_r_val),
    _drs_doc: $ => seq(repeat1($._s_dir), $._exp_doc),
    _exp_doc: $ => seq($._s_drs_end, optional($._exp_doc_tal)),
    _imp_doc: $ => choice($._br_blk_seq_val, $._br_blk_map_val, $._br_blk_str_val, $._br_flw_val_blk),

    _drs_doc_end: $ => prec(1, seq($._drs_doc, $._s_doc_end)),
    _exp_doc_end: $ => prec(1, seq($._exp_doc, $._s_doc_end)),
    _imp_doc_end: $ => prec(1, seq($._imp_doc, $._s_doc_end)),
    _bgn_imp_doc_end: $ => prec(1, seq($._bgn_imp_doc, $._s_doc_end)),
    _doc_end: $ => $._s_doc_end,

    _exp_doc_tal: $ => choice($._r_blk_seq_br_val, $._br_blk_seq_val, $._r_blk_map_br_val, $._br_blk_map_val, $._r_blk_str_val, $._br_blk_str_val, $._r_flw_val_blk, $._br_flw_val_blk),

    // directive

    _s_dir: $ => choice($._s_dir_yml, $._s_dir_tag, $._s_dir_rsv),

    _s_dir_yml: $ => seq($._s_dir_yml_bgn, $._r_dir_yml_ver),
    _s_dir_tag: $ => seq($._s_dir_tag_bgn, $._r_dir_tag_hdl, $._r_dir_tag_pfx),
    _s_dir_rsv: $ => seq($._s_dir_rsv_bgn, repeat($._r_dir_rsv_prm)),

    // property

    _r_prp_val: $ => $._r_prp,
    _br_prp_val: $ => $._br_prp,

    _r_sgl_prp_val: $ => $._r_sgl_prp,
    _br_sgl_prp_val: $ => $._br_sgl_prp,
    _b_sgl_prp_val: $ => $._b_sgl_prp,

    _r_prp: $ => choice(seq($._r_acr, optional(choice($._r_tag, $._br_tag))), seq($._r_tag, optional(choice($._r_acr, $._br_acr)))),
    _br_prp: $ => choice(seq($._br_acr, optional(choice($._r_tag, $._br_tag))), seq($._br_tag, optional(choice($._r_acr, $._br_acr)))),

    _r_sgl_prp: $ => choice(seq($._r_acr, optional($._r_tag)), seq($._r_tag, optional($._r_acr))),
    _br_sgl_prp: $ => choice(seq($._br_acr, optional($._r_tag)), seq($._br_tag, optional($._r_acr))),
    _b_sgl_prp: $ => choice(seq($._b_acr, optional($._r_tag)), seq($._b_tag, optional($._r_acr))),

    // block sequence

    _r_blk_seq_val: $ => choice($._r_blk_seq_r_val, $._r_blk_seq_br_val),
    _r_blk_seq_r_val: $ => $._r_blk_seq,
    _r_blk_seq_br_val: $ => seq($._r_prp, $._br_blk_seq),
    _br_blk_seq_val: $ => choice($._br_blk_seq, seq($._br_prp, $._br_blk_seq)),

    _r_blk_seq_spc_val: $ => seq($._r_prp, $._b_blk_seq_spc),
    _br_blk_seq_spc_val: $ => seq($._br_prp, $._b_blk_seq_spc),
    _b_blk_seq_spc_val: $ => $._b_blk_seq_spc,

    _r_blk_seq: $ => seq($._r_blk_seq_itm, repeat($._b_blk_seq_itm), $._bl),
    _br_blk_seq: $ => seq($._br_blk_seq_itm, repeat($._b_blk_seq_itm), $._bl),

    _b_blk_seq_spc: $ => seq(repeat1($._b_blk_seq_itm), $._bl),

    _r_blk_seq_itm: $ => seq($._r_blk_seq_bgn, optional($._blk_seq_itm_tal)),
    _br_blk_seq_itm: $ => seq($._br_blk_seq_bgn, optional($._blk_seq_itm_tal)),
    _b_blk_seq_itm: $ => seq($._b_blk_seq_bgn, optional($._blk_seq_itm_tal)),

    _blk_seq_itm_tal: $ => choice($._r_blk_seq_val, $._br_blk_seq_val, $._r_blk_map_val, $._br_blk_map_val, $._r_blk_str_val, $._br_blk_str_val, $._r_flw_val_blk, $._br_flw_val_blk),

    // block mapping

    _r_blk_map_val: $ => choice($._r_blk_map_r_val, $._r_blk_map_br_val),
    _r_blk_map_r_val: $ => $._r_blk_map,
    _r_blk_map_br_val: $ => seq($._r_prp, $._br_blk_map),
    _br_blk_map_val: $ => choice($._br_blk_map, seq($._br_prp, $._br_blk_map)),

    _r_blk_map: $ => seq($._r_blk_map_itm, repeat($._b_blk_map_itm), $._bl),
    _br_blk_map: $ => seq($._br_blk_map_itm, repeat($._b_blk_map_itm), $._bl),

    _r_blk_map_itm: $ => choice($._r_blk_exp_itm, $._r_blk_imp_itm),
    _br_blk_map_itm: $ => choice($._br_blk_exp_itm, $._br_blk_imp_itm),
    _b_blk_map_itm: $ => choice($._b_blk_exp_itm, $._b_blk_imp_itm),

    _r_blk_exp_itm: $ => prec.right(choice(seq($._r_blk_key_itm, optional($._b_blk_val_itm)), $._r_blk_val_itm)),
    _br_blk_exp_itm: $ => prec.right(choice(seq($._br_blk_key_itm, optional($._b_blk_val_itm)), $._br_blk_val_itm)),
    _b_blk_exp_itm: $ => prec.right(choice(seq($._b_blk_key_itm, optional($._b_blk_val_itm)), $._b_blk_val_itm)),

    _r_blk_key_itm: $ => seq($._r_blk_key_bgn, optional(field("key", $._blk_exp_itm_tal))),
    _br_blk_key_itm: $ => seq($._br_blk_key_bgn, optional(field("key", $._blk_exp_itm_tal))),
    _b_blk_key_itm: $ => seq($._b_blk_key_bgn, optional(field("key", $._blk_exp_itm_tal))),

    _r_blk_val_itm: $ => seq($._r_blk_val_bgn, optional(field("value", $._blk_exp_itm_tal))),
    _br_blk_val_itm: $ => seq($._br_blk_val_bgn, optional(field("value", $._blk_exp_itm_tal))),
    _b_blk_val_itm: $ => seq($._b_blk_val_bgn, optional(field("value", $._blk_exp_itm_tal))),

    _r_blk_imp_itm: $ => seq(field("key", $._r_sgl_flw_val_blk), $._blk_imp_itm_tal),
    _br_blk_imp_itm: $ => seq(field("key", $._br_sgl_flw_val_blk), $._blk_imp_itm_tal),
    _b_blk_imp_itm: $ => seq(field("key", $._b_sgl_flw_val_blk), $._blk_imp_itm_tal),

    _blk_exp_itm_tal: $ => choice($._blk_seq_itm_tal, $._r_blk_seq_spc_val, $._br_blk_seq_spc_val, $._b_blk_seq_spc_val),
    _blk_imp_itm_tal: $ => seq($._r_blk_imp_bgn, optional(field("value", choice($._r_blk_seq_br_val, $._br_blk_seq_val, $._r_blk_seq_spc_val, $._br_blk_seq_spc_val, $._b_blk_seq_spc_val, $._r_blk_map_br_val, $._br_blk_map_val, $._r_blk_str_val, $._br_blk_str_val, $._r_flw_val_blk, $._br_flw_val_blk)))),

    // block scalar

    _r_blk_str_val: $ => choice($._r_blk_str, seq($._r_prp, choice($._r_blk_str, $._br_blk_str))),
    _br_blk_str_val: $ => choice($._br_blk_str, seq($._br_prp, choice($._r_blk_str, $._br_blk_str))),

    _r_blk_str: $ => seq(choice($._r_blk_lit_bgn, $._r_blk_fld_bgn), repeat($._br_blk_str_ctn), $._bl),
    _br_blk_str: $ => seq(choice($._br_blk_lit_bgn, $._br_blk_fld_bgn), repeat($._br_blk_str_ctn), $._bl),

    // flow value in block

    _r_flw_val_blk: $ => choice($._r_flw_jsl_val, $._r_flw_njl_val_blk),
    _br_flw_val_blk: $ => choice($._br_flw_jsl_val, $._br_flw_njl_val_blk),

    _r_sgl_flw_val_blk: $ => choice($._r_sgl_flw_jsl_val, $._r_sgl_flw_njl_val_blk),
    _br_sgl_flw_val_blk: $ => choice($._br_sgl_flw_jsl_val, $._br_sgl_flw_njl_val_blk),
    _b_sgl_flw_val_blk: $ => choice($._b_sgl_flw_jsl_val, $._b_sgl_flw_njl_val_blk),

    // flow value in flow

    _r_flw_val_flw: $ => choice($._r_flw_jsl_val, $._r_flw_njl_val_flw),
    _br_flw_val_flw: $ => choice($._br_flw_jsl_val, $._br_flw_njl_val_flw),

    _r_sgl_flw_val_flw: $ => choice($._r_sgl_flw_jsl_val, $._r_sgl_flw_njl_val_flw),

    // json-like flow value

    _r_flw_jsl_val: $ => choice($._r_flw_seq_val, $._r_flw_map_val, $._r_dqt_str_val, $._r_sqt_str_val),
    _br_flw_jsl_val: $ => choice($._br_flw_seq_val, $._br_flw_map_val, $._br_dqt_str_val, $._br_sqt_str_val),

    _r_sgl_flw_jsl_val: $ => choice($._r_sgl_flw_seq_val, $._r_sgl_flw_map_val, $._r_sgl_dqt_str_val, $._r_sgl_sqt_str_val),
    _br_sgl_flw_jsl_val: $ => choice($._br_sgl_flw_seq_val, $._br_sgl_flw_map_val, $._br_sgl_dqt_str_val, $._br_sgl_sqt_str_val),
    _b_sgl_flw_jsl_val: $ => choice($._b_sgl_flw_seq_val, $._b_sgl_flw_map_val, $._b_sgl_dqt_str_val, $._b_sgl_sqt_str_val),

    // non-json-like flow value in block

    _r_flw_njl_val_blk: $ => choice($._r_als_val, $._r_prp_val, $._r_pln_blk_val),
    _br_flw_njl_val_blk: $ => choice($._br_als_val, $._br_prp_val, $._br_pln_blk_val),

    _r_sgl_flw_njl_val_blk: $ => choice($._r_als_val, $._r_sgl_prp_val, $._r_sgl_pln_blk_val),
    _br_sgl_flw_njl_val_blk: $ => choice($._br_als_val, $._br_sgl_prp_val, $._br_sgl_pln_blk_val),
    _b_sgl_flw_njl_val_blk: $ => choice($._b_als_val, $._b_sgl_prp_val, $._b_sgl_pln_blk_val),

    // non-json-like flow value in flow

    _r_flw_njl_val_flw: $ => choice($._r_als_val, $._r_prp_val, $._r_pln_flw_val),
    _br_flw_njl_val_flw: $ => choice($._br_als_val, $._br_prp_val, $._br_pln_flw_val),

    _r_sgl_flw_njl_val_flw: $ => choice($._r_als_val, $._r_sgl_prp_val, $._r_sgl_pln_flw_val),

    // flow sequence

    _r_flw_seq_val: $ => choice($._r_flw_seq, seq($._r_prp, choice($._r_flw_seq, $._br_flw_seq))),
    _br_flw_seq_val: $ => choice($._br_flw_seq, seq($._br_prp, choice($._r_flw_seq, $._br_flw_seq))),

    _r_sgl_flw_seq_val: $ => choice($._r_sgl_flw_seq, seq($._r_sgl_prp, $._r_sgl_flw_seq)),
    _br_sgl_flw_seq_val: $ => choice($._br_sgl_flw_seq, seq($._br_sgl_prp, $._r_sgl_flw_seq)),
    _b_sgl_flw_seq_val: $ => choice($._b_sgl_flw_seq, seq($._b_sgl_prp, $._r_sgl_flw_seq)),

    _r_flw_seq: $ => seq($._r_flw_seq_bgn, $._flw_seq_tal),
    _br_flw_seq: $ => seq($._br_flw_seq_bgn, $._flw_seq_tal),

    _r_sgl_flw_seq: $ => seq($._r_flw_seq_bgn, $._sgl_flw_seq_tal),
    _br_sgl_flw_seq: $ => seq($._br_flw_seq_bgn, $._sgl_flw_seq_tal),
    _b_sgl_flw_seq: $ => seq($._b_flw_seq_bgn, $._sgl_flw_seq_tal),

    _flw_seq_tal: $ => seq(optional(choice($._r_flw_seq_dat, $._br_flw_seq_dat)), choice($._r_flw_seq_end, $._br_flw_seq_end, $._b_flw_seq_end)),
    _sgl_flw_seq_tal: $ => seq(optional($._r_sgl_flw_col_dat), $._r_flw_seq_end),

    // flow mapping

    _r_flw_map_val: $ => choice($._r_flw_map, seq($._r_prp, choice($._r_flw_map, $._br_flw_map))),
    _br_flw_map_val: $ => choice($._br_flw_map, seq($._br_prp, choice($._r_flw_map, $._br_flw_map))),

    _r_sgl_flw_map_val: $ => choice($._r_sgl_flw_map, seq($._r_sgl_prp, $._r_sgl_flw_map)),
    _br_sgl_flw_map_val: $ => choice($._br_sgl_flw_map, seq($._br_sgl_prp, $._r_sgl_flw_map)),
    _b_sgl_flw_map_val: $ => choice($._b_sgl_flw_map, seq($._b_sgl_prp, $._r_sgl_flw_map)),

    _r_flw_map: $ => seq($._r_flw_map_bgn, $._flw_map_tal),
    _br_flw_map: $ => seq($._br_flw_map_bgn, $._flw_map_tal),

    _r_sgl_flw_map: $ => seq($._r_flw_map_bgn, $._sgl_flw_map_tal),
    _br_sgl_flw_map: $ => seq($._br_flw_map_bgn, $._sgl_flw_map_tal),
    _b_sgl_flw_map: $ => seq($._b_flw_map_bgn, $._sgl_flw_map_tal),

    _flw_map_tal: $ => seq(optional(choice($._r_flw_map_dat, $._br_flw_map_dat)), choice($._r_flw_map_end, $._br_flw_map_end, $._b_flw_map_end)),
    _sgl_flw_map_tal: $ => seq(optional($._r_sgl_flw_col_dat), $._r_flw_map_end),

    // flow collection data

    _r_flw_seq_dat: $ => seq($._r_flw_seq_itm, repeat($._flw_seq_dat_rpt), optional(choice($._r_flw_sep_bgn, $._br_flw_sep_bgn))),
    _br_flw_seq_dat: $ => seq($._br_flw_seq_itm, repeat($._flw_seq_dat_rpt), optional(choice($._r_flw_sep_bgn, $._br_flw_sep_bgn))),

    _r_flw_map_dat: $ => seq($._r_flw_map_itm, repeat($._flw_map_dat_rpt), optional(choice($._r_flw_sep_bgn, $._br_flw_sep_bgn))),
    _br_flw_map_dat: $ => seq($._br_flw_map_itm, repeat($._flw_map_dat_rpt), optional(choice($._r_flw_sep_bgn, $._br_flw_sep_bgn))),

    _r_sgl_flw_col_dat: $ => seq($._r_sgl_flw_col_itm, repeat($._sgl_flw_col_dat_rpt), optional($._r_flw_sep_bgn)),

    _flw_seq_dat_rpt: $ => seq(choice($._r_flw_sep_bgn, $._br_flw_sep_bgn), choice($._r_flw_seq_itm, $._br_flw_seq_itm)),
    _flw_map_dat_rpt: $ => seq(choice($._r_flw_sep_bgn, $._br_flw_sep_bgn), choice($._r_flw_map_itm, $._br_flw_map_itm)),

    _sgl_flw_col_dat_rpt: $ => seq($._r_flw_sep_bgn, $._r_sgl_flw_col_itm),

    // flow collection item

    _r_flw_seq_itm: $ => choice($._r_flw_val_flw, $._r_flw_exp_par, $._r_flw_imp_r_par, $._r_flw_njl_ann_par),
    _br_flw_seq_itm: $ => choice($._br_flw_val_flw, $._br_flw_exp_par, $._br_flw_imp_r_par, $._br_flw_njl_ann_par),

    _r_flw_map_itm: $ => choice($._r_flw_val_flw, $._r_flw_exp_par, $._r_flw_imp_r_par, $._r_flw_imp_br_par, $._r_flw_njl_ann_par),
    _br_flw_map_itm: $ => choice($._br_flw_val_flw, $._br_flw_exp_par, $._br_flw_imp_r_par, $._br_flw_imp_br_par, $._br_flw_njl_ann_par),

    _r_sgl_flw_col_itm: $ => choice($._r_sgl_flw_val_flw, $._r_sgl_flw_exp_par, $._r_sgl_flw_imp_par, $._r_sgl_flw_njl_ann_par),

    // explicit flow pair

    _r_flw_exp_par: $ => seq($._r_flw_key_bgn, optional(choice($._r_flw_imp_r_par, $._r_flw_imp_br_par, $._br_flw_imp_r_par, $._br_flw_imp_br_par))),
    _br_flw_exp_par: $ => seq($._br_flw_key_bgn, optional(choice($._r_flw_imp_r_par, $._r_flw_imp_br_par, $._br_flw_imp_r_par, $._br_flw_imp_br_par))),

    _r_sgl_flw_exp_par: $ => seq($._r_flw_key_bgn, optional($._r_sgl_flw_imp_par)),

    // implicit flow pair

    _r_flw_imp_r_par: $ => choice(seq(field("key", $._r_flw_jsl_val), $._r_flw_jsl_ann_par), seq(field("key", $._r_flw_njl_val_flw), $._r_flw_njl_ann_par)),
    _r_flw_imp_br_par: $ => choice(seq(field("key", $._r_flw_jsl_val), $._br_flw_jsl_ann_par), seq(field("key", $._r_flw_njl_val_flw), $._br_flw_njl_ann_par)),
    _br_flw_imp_r_par: $ => choice(seq(field("key", $._br_flw_jsl_val), $._r_flw_jsl_ann_par), seq(field("key", $._br_flw_njl_val_flw), $._r_flw_njl_ann_par)),
    _br_flw_imp_br_par: $ => choice(seq(field("key", $._br_flw_jsl_val), $._br_flw_jsl_ann_par), seq(field("key", $._br_flw_njl_val_flw), $._br_flw_njl_ann_par)),

    _r_sgl_flw_imp_par: $ => choice(seq(field("key", $._r_sgl_flw_jsl_val), $._r_sgl_flw_jsl_ann_par), seq(field("key", $._r_sgl_flw_njl_val_flw), $._r_sgl_flw_njl_ann_par)),

    // anonymous flow pair

    _r_flw_jsl_ann_par: $ => seq($._r_flw_jsv_bgn, optional(field("value", $._flw_ann_par_tal))),
    _br_flw_jsl_ann_par: $ => seq($._br_flw_jsv_bgn, optional(field("value", $._flw_ann_par_tal))),

    _r_sgl_flw_jsl_ann_par: $ => seq($._r_flw_jsv_bgn, optional(field("value", $._sgl_flw_ann_par_tal))),

    _r_flw_njl_ann_par: $ => seq($._r_flw_njv_bgn, optional(field("value", $._flw_ann_par_tal))),
    _br_flw_njl_ann_par: $ => seq($._br_flw_njv_bgn, optional(field("value", $._flw_ann_par_tal))),

    _r_sgl_flw_njl_ann_par: $ => seq($._r_flw_njv_bgn, optional(field("value", $._sgl_flw_ann_par_tal))),

    _flw_ann_par_tal: $ => choice($._r_flw_val_flw, $._br_flw_val_flw),
    _sgl_flw_ann_par_tal: $ => $._r_sgl_flw_val_flw,

    // double quote scalar

    _r_dqt_str_val: $ => choice($._r_dqt_str, seq($._r_prp, choice($._r_dqt_str, $._br_dqt_str))),
    _br_dqt_str_val: $ => choice($._br_dqt_str, seq($._br_prp, choice($._r_dqt_str, $._br_dqt_str))),

    _r_sgl_dqt_str_val: $ => choice($._r_sgl_dqt_str, seq($._r_sgl_prp, $._r_sgl_dqt_str)),
    _br_sgl_dqt_str_val: $ => choice($._br_sgl_dqt_str, seq($._br_sgl_prp, $._r_sgl_dqt_str)),
    _b_sgl_dqt_str_val: $ => choice($._b_sgl_dqt_str, seq($._b_sgl_prp, $._r_sgl_dqt_str)),

    _r_dqt_str: $ => seq($._r_dqt_str_bgn, optional($._r_sgl_dqt_ctn), optional($._r_dqt_esc_nwl), repeat($._br_mtl_dqt_ctn), choice($._r_dqt_str_end, $._br_dqt_str_end)),
    _br_dqt_str: $ => seq($._br_dqt_str_bgn, optional($._r_sgl_dqt_ctn), optional($._r_dqt_esc_nwl), repeat($._br_mtl_dqt_ctn), choice($._r_dqt_str_end, $._br_dqt_str_end)),

    _r_sgl_dqt_str: $ => seq($._r_dqt_str_bgn, optional($._r_sgl_dqt_ctn), $._r_dqt_str_end),
    _br_sgl_dqt_str: $ => seq($._br_dqt_str_bgn, optional($._r_sgl_dqt_ctn), $._r_dqt_str_end),
    _b_sgl_dqt_str: $ => seq($._b_dqt_str_bgn, optional($._r_sgl_dqt_ctn), $._r_dqt_str_end),

    _r_sgl_dqt_ctn: $ => repeat1(choice($._r_dqt_str_ctn, $._r_dqt_esc_seq)),
    _br_mtl_dqt_ctn: $ => choice($._br_dqt_esc_nwl, seq(choice($._br_dqt_str_ctn, $._br_dqt_esc_seq), repeat(choice($._r_dqt_str_ctn, $._r_dqt_esc_seq)), optional($._r_dqt_esc_nwl))),

    // single quote scalar

    _r_sqt_str_val: $ => choice($._r_sqt_str, seq($._r_prp, choice($._r_sqt_str, $._br_sqt_str))),
    _br_sqt_str_val: $ => choice($._br_sqt_str, seq($._br_prp, choice($._r_sqt_str, $._br_sqt_str))),

    _r_sgl_sqt_str_val: $ => choice($._r_sgl_sqt_str, seq($._r_sgl_prp, $._r_sgl_sqt_str)),
    _br_sgl_sqt_str_val: $ => choice($._br_sgl_sqt_str, seq($._br_sgl_prp, $._r_sgl_sqt_str)),
    _b_sgl_sqt_str_val: $ => choice($._b_sgl_sqt_str, seq($._b_sgl_prp, $._r_sgl_sqt_str)),

    _r_sqt_str: $ => seq($._r_sqt_str_bgn, optional($._r_sgl_sqt_ctn), repeat($._br_mtl_sqt_ctn), choice($._r_sqt_str_end, $._br_sqt_str_end)),
    _br_sqt_str: $ => seq($._br_sqt_str_bgn, optional($._r_sgl_sqt_ctn), repeat($._br_mtl_sqt_ctn), choice($._r_sqt_str_end, $._br_sqt_str_end)),

    _r_sgl_sqt_str: $ => seq($._r_sqt_str_bgn, optional($._r_sgl_sqt_ctn), $._r_sqt_str_end),
    _br_sgl_sqt_str: $ => seq($._br_sqt_str_bgn, optional($._r_sgl_sqt_ctn), $._r_sqt_str_end),
    _b_sgl_sqt_str: $ => seq($._b_sqt_str_bgn, optional($._r_sgl_sqt_ctn), $._r_sqt_str_end),

    _r_sgl_sqt_ctn: $ => repeat1(choice($._r_sqt_str_ctn, $._r_sqt_esc_sqt)),
    _br_mtl_sqt_ctn: $ => seq(choice($._br_sqt_str_ctn, $._br_sqt_esc_sqt), repeat(choice($._r_sqt_str_ctn, $._r_sqt_esc_sqt))),

    // plain scalar in block

    _r_pln_blk_val: $ => choice($._r_pln_blk, seq($._r_prp, choice($._r_pln_blk, $._br_pln_blk))),
    _br_pln_blk_val: $ => choice($._br_pln_blk, seq($._br_prp, choice($._r_pln_blk, $._br_pln_blk))),

    _r_sgl_pln_blk_val: $ => choice($._r_sgl_pln_blk, seq($._r_sgl_prp, $._r_sgl_pln_blk)),
    _br_sgl_pln_blk_val: $ => choice($._br_sgl_pln_blk, seq($._br_sgl_prp, $._r_sgl_pln_blk)),
    _b_sgl_pln_blk_val: $ => choice($._b_sgl_pln_blk, seq($._b_sgl_prp, $._r_sgl_pln_blk)),

    _r_pln_blk: $ => choice($._r_sgl_pln_blk, $._r_mtl_pln_blk),
    _br_pln_blk: $ => choice($._br_sgl_pln_blk, $._br_mtl_pln_blk),

    // plain scalar in flow

    _r_pln_flw_val: $ => choice($._r_pln_flw, seq($._r_prp, choice($._r_pln_flw, $._br_pln_flw))),
    _br_pln_flw_val: $ => choice($._br_pln_flw, seq($._br_prp, choice($._r_pln_flw, $._br_pln_flw))),

    _r_sgl_pln_flw_val: $ => choice($._r_sgl_pln_flw, seq($._r_sgl_prp, $._r_sgl_pln_flw)),

    _r_pln_flw: $ => choice($._r_sgl_pln_flw, $._r_mtl_pln_flw),
    _br_pln_flw: $ => choice($._br_sgl_pln_flw, $._br_mtl_pln_flw),

    // plain scalar schema

    _r_sgl_pln_blk: $ => choice($._r_sgl_pln_nul_blk, $._r_sgl_pln_bol_blk, $._r_sgl_pln_int_blk, $._r_sgl_pln_flt_blk, $._r_sgl_pln_str_blk),
    _br_sgl_pln_blk: $ => choice($._br_sgl_pln_nul_blk, $._br_sgl_pln_bol_blk, $._br_sgl_pln_int_blk, $._br_sgl_pln_flt_blk, $._br_sgl_pln_str_blk),
    _b_sgl_pln_blk: $ => choice($._b_sgl_pln_nul_blk, $._b_sgl_pln_bol_blk, $._b_sgl_pln_int_blk, $._b_sgl_pln_flt_blk, $._b_sgl_pln_str_blk),
    _r_sgl_pln_flw: $ => choice($._r_sgl_pln_nul_flw, $._r_sgl_pln_bol_flw, $._r_sgl_pln_int_flw, $._r_sgl_pln_flt_flw, $._r_sgl_pln_str_flw),
    _br_sgl_pln_flw: $ => choice($._br_sgl_pln_nul_flw, $._br_sgl_pln_bol_flw, $._br_sgl_pln_int_flw, $._br_sgl_pln_flt_flw, $._br_sgl_pln_str_flw),

    _r_mtl_pln_blk: $ => $._r_mtl_pln_str_blk,
    _br_mtl_pln_blk: $ => $._br_mtl_pln_str_blk,
    _r_mtl_pln_flw: $ => $._r_mtl_pln_str_flw,
    _br_mtl_pln_flw: $ => $._br_mtl_pln_str_flw,

    // alias

    _r_als_val: $ => $._r_als,
    _br_als_val: $ => $._br_als,
    _b_als_val: $ => $._b_als,

    _r_als: $ => seq($._r_als_bgn, $._r_als_ctn),
    _br_als: $ => seq($._br_als_bgn, $._r_als_ctn),
    _b_als: $ => seq($._b_als_bgn, $._r_als_ctn),

    // anchor

    _r_acr: $ => seq($._r_acr_bgn, $._r_acr_ctn),
    _br_acr: $ => seq($._br_acr_bgn, $._r_acr_ctn),
    _b_acr: $ => seq($._b_acr_bgn, $._r_acr_ctn),
  },
});

module.exports.grammar = global_alias(global_alias(module.exports.grammar, {
  ..._("yaml_directive", "_s_dir_yml"),
  ..._("yaml_version", "_r_dir_yml_ver"),
  ..._("tag_directive", "_s_dir_tag"),
  ..._("tag_handle", "_r_dir_tag_hdl"),
  ..._("tag_prefix", "_r_dir_tag_pfx"),
  ..._("reserved_directive", "_s_dir_rsv"),
  ..._("directive_name", "_s_dir_rsv_bgn"),
  ..._("directive_parameter", "_r_dir_rsv_prm"),
  ..._("flow_node", "_r_prp_val", "_br_prp_val", "_r_sgl_prp_val", "_br_sgl_prp_val", "_b_sgl_prp_val"),
  ..._("tag", "_r_tag", "_br_tag", "_b_tag"),
  ..._("anchor", "_r_acr", "_br_acr", "_b_acr"),
  ..._("anchor_name", "_r_acr_ctn"),
  ..._("flow_node", "_r_als_val", "_br_als_val", "_b_als_val"),
  ..._("alias", "_r_als", "_br_als", "_b_als"),
  ..._("alias_name", "_r_als_ctn"),
  ..._("document", "_bgn_imp_doc", "_imp_doc"),
  ..._(["document"], "_drs_doc", "_exp_doc", "_doc_end",
                     "_bgn_imp_doc_end", "_drs_doc_end", "_exp_doc_end", "_imp_doc_end"),
  ..._("block_node", "_r_blk_seq_r_val", "_r_blk_seq_br_val", "_br_blk_seq_val", "_r_blk_seq_spc_val", "_br_blk_seq_spc_val", "_b_blk_seq_spc_val"),
  ..._("block_node", "_r_blk_map_r_val", "_r_blk_map_br_val", "_br_blk_map_val"),
  ..._("block_node", "_r_blk_str_val", "_br_blk_str_val"),
  ..._("block_sequence", "_r_blk_seq", "_br_blk_seq", "_b_blk_seq_spc"),
  ..._("block_mapping", "_r_blk_map", "_br_blk_map"),
  ..._("block_scalar", "_r_blk_str", "_br_blk_str"),
  ..._("block_sequence_item", "_r_blk_seq_itm", "_br_blk_seq_itm", "_b_blk_seq_itm"),
  ..._("block_mapping_pair", "_r_blk_exp_itm", "_br_blk_exp_itm", "_b_blk_exp_itm"),
  ..._("block_mapping_pair", "_r_blk_imp_itm", "_br_blk_imp_itm", "_b_blk_imp_itm"),
  ..._("flow_node", "_r_flw_seq_val", "_br_flw_seq_val", "_r_sgl_flw_seq_val", "_br_sgl_flw_seq_val", "_b_sgl_flw_seq_val"),
  ..._("flow_node", "_r_flw_map_val", "_br_flw_map_val", "_r_sgl_flw_map_val", "_br_sgl_flw_map_val", "_b_sgl_flw_map_val"),
  ..._("flow_sequence", "_r_flw_seq", "_br_flw_seq", "_r_sgl_flw_seq", "_br_sgl_flw_seq", "_b_sgl_flw_seq"),
  ..._("flow_mapping", "_r_flw_map", "_br_flw_map", "_r_sgl_flw_map", "_br_sgl_flw_map", "_b_sgl_flw_map"),
  ..._(["flow_pair"], "_r_flw_exp_par", "_br_flw_exp_par", "_r_sgl_flw_exp_par",
                      "_r_flw_imp_r_par", "_r_flw_imp_br_par", "_br_flw_imp_r_par", "_br_flw_imp_br_par", "_r_sgl_flw_imp_par",
                      "_r_flw_njl_ann_par", "_br_flw_njl_ann_par", "_r_sgl_flw_njl_ann_par"),
  ..._("flow_node", "_r_dqt_str_val", "_br_dqt_str_val", "_r_sgl_dqt_str_val", "_br_sgl_dqt_str_val", "_b_sgl_dqt_str_val"),
  ..._("flow_node", "_r_sqt_str_val", "_br_sqt_str_val", "_r_sgl_sqt_str_val", "_br_sgl_sqt_str_val", "_b_sgl_sqt_str_val"),
  ..._("flow_node", "_r_pln_blk_val", "_br_pln_blk_val", "_r_sgl_pln_blk_val", "_br_sgl_pln_blk_val", "_b_sgl_pln_blk_val",
                    "_r_pln_flw_val", "_br_pln_flw_val", "_r_sgl_pln_flw_val"),
  ..._("double_quote_scalar", "_r_dqt_str", "_br_dqt_str", "_r_sgl_dqt_str", "_br_sgl_dqt_str", "_b_sgl_dqt_str"),
  ..._("single_quote_scalar", "_r_sqt_str", "_br_sqt_str", "_r_sgl_sqt_str", "_br_sgl_sqt_str", "_b_sgl_sqt_str"),
  ..._("plain_scalar", "_r_mtl_pln_blk", "_br_mtl_pln_blk", "_r_sgl_pln_blk", "_br_sgl_pln_blk", "_b_sgl_pln_blk",
                       "_r_mtl_pln_flw", "_br_mtl_pln_flw", "_r_sgl_pln_flw", "_br_sgl_pln_flw"),
  ..._("escape_sequence", "_r_dqt_esc_nwl", "_br_dqt_esc_nwl",
                          "_r_dqt_esc_seq", "_br_dqt_esc_seq",
                          "_r_sqt_esc_sqt", "_br_sqt_esc_sqt"),
  ..._("null_scalar", "_r_sgl_pln_nul_blk", "_br_sgl_pln_nul_blk", "_b_sgl_pln_nul_blk", "_r_sgl_pln_nul_flw", "_br_sgl_pln_nul_flw"),
  ..._("boolean_scalar", "_r_sgl_pln_bol_blk", "_br_sgl_pln_bol_blk", "_b_sgl_pln_bol_blk", "_r_sgl_pln_bol_flw", "_br_sgl_pln_bol_flw"),
  ..._("integer_scalar", "_r_sgl_pln_int_blk", "_br_sgl_pln_int_blk", "_b_sgl_pln_int_blk", "_r_sgl_pln_int_flw", "_br_sgl_pln_int_flw"),
  ..._("float_scalar", "_r_sgl_pln_flt_blk", "_br_sgl_pln_flt_blk", "_b_sgl_pln_flt_blk", "_r_sgl_pln_flt_flw", "_br_sgl_pln_flt_flw"),
  ..._("string_scalar", "_r_sgl_pln_str_blk", "_br_sgl_pln_str_blk", "_b_sgl_pln_str_blk", "_r_sgl_pln_str_flw", "_br_sgl_pln_str_flw",
                        "_r_mtl_pln_str_blk", "_br_mtl_pln_str_blk", "_r_mtl_pln_str_flw", "_br_mtl_pln_str_flw"),
}), {
  ..._("---", "_s_drs_end"),
  ..._("...", "_s_doc_end"),
  ..._("-", "_r_blk_seq_bgn", "_br_blk_seq_bgn", "_b_blk_seq_bgn"),
  ..._("?", "_r_blk_key_bgn", "_br_blk_key_bgn", "_b_blk_key_bgn"),
  ..._(":", "_r_blk_val_bgn", "_br_blk_val_bgn", "_b_blk_val_bgn"),
  ..._(":", "_r_blk_imp_bgn"),
  ..._("|", "_r_blk_lit_bgn", "_br_blk_lit_bgn"),
  ..._(">", "_r_blk_fld_bgn", "_br_blk_fld_bgn"),
  ..._("[", "_r_flw_seq_bgn", "_br_flw_seq_bgn", "_b_flw_seq_bgn"),
  ..._("]", "_r_flw_seq_end", "_br_flw_seq_end", "_b_flw_seq_end"),
  ..._("{", "_r_flw_map_bgn", "_br_flw_map_bgn", "_b_flw_map_bgn"),
  ..._("}", "_r_flw_map_end", "_br_flw_map_end", "_b_flw_map_end"),
  ..._(",", "_r_flw_sep_bgn", "_br_flw_sep_bgn"),
  ..._("?", "_r_flw_key_bgn", "_br_flw_key_bgn"),
  ..._(":", "_r_flw_jsv_bgn", "_br_flw_jsv_bgn"),
  ..._(":", "_r_flw_njv_bgn", "_br_flw_njv_bgn"),
  ..._("\"", "_r_dqt_str_bgn", "_br_dqt_str_bgn", "_b_dqt_str_bgn"),
  ..._("\"", "_r_dqt_str_end", "_br_dqt_str_end"),
  ..._("'", "_r_sqt_str_bgn", "_br_sqt_str_bgn", "_b_sqt_str_bgn"),
  ..._("'", "_r_sqt_str_end", "_br_sqt_str_end"),
  ..._("*", "_r_als_bgn", "_br_als_bgn", "_b_als_bgn"),
  ..._("&", "_r_acr_bgn", "_br_acr_bgn", "_b_acr_bgn"),
});

function _(alias_value, ...rule_names) {
  const alias_content = {};
  if (typeof alias_value === "string") {
    alias_content.name = alias_value;
  } else if (Array.isArray(alias_value)) {
    alias_content.name = alias_value[0];
    alias_content.shallow = true;
  } else {
    throw new Error(`Unexpected value ${JSON.stringify(alias_value)}`);
  }
  const alias_map = {};
  for (const rule_name of rule_names) {
    alias_map[rule_name] = alias_content;
  }
  return alias_map;
}

function global_alias(grammar_json, alias_map) {
  const new_rules = {};
  const new_grammar = { ...grammar_json, rules: new_rules };
  const checklist = Object.fromEntries(Object.entries(alias_map).map(([k, v]) => [k, 0]));
  for (const [rule_name, rule] of Object.entries(grammar_json.rules)) {
    new_rules[rule_name] = rule_name in alias_map && alias_map[rule_name].shallow
      ? rule
      : recursive_alias(rule, alias_map, checklist);
  }
  for (const [rule_name, counter] of Object.entries(checklist)) {
    if (counter === 0) {
      console.warn(`warning: global_alias for ${JSON.stringify(rule_name)} is not used.`);
    }
  }
  return new_grammar;
}

function recursive_alias(rule, alias_map, checklist) {
  switch (rule.type) {
    case "CHOICE":
    case "SEQ":
      return { ...rule, members: rule.members.map(member => recursive_alias(member, alias_map, checklist)) };
    case "REPEAT":
    case "REPEAT1":
    case "FIELD":
    case "PREC":
    case "PREC_RIGHT":
      return { ...rule, content: recursive_alias(rule.content, alias_map, checklist) };
    case "SYMBOL":
      if (rule.name in alias_map) {
        checklist[rule.name]++;
        const alias = alias_map[rule.name].name;
        return { type: "ALIAS", content: rule, named: /[a-z]/i.test(alias), value: alias };
      }
    case "BLANK":
    case "ALIAS":
      return rule;
    default:
      throw new Error(`Unexpected rule type ${JSON.stringify(rule.type)}`);
  }
}
