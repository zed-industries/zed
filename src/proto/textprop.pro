/* textprop.c */
int find_prop_type_id(char_u *name, buf_T *buf);
void f_prop_add(typval_T *argvars, typval_T *rettv);
void f_prop_add_list(typval_T *argvars, typval_T *rettv);
int prop_add_common(linenr_T start_lnum, colnr_T start_col, dict_T *dict, buf_T *default_buf, typval_T *dict_arg);
int get_text_props(buf_T *buf, linenr_T lnum, char_u **props, int will_change);
int prop_count_above_below(buf_T *buf, linenr_T lnum);
int count_props(linenr_T lnum, int only_starting, int last_line);
void sort_text_props(buf_T *buf, textprop_T *props, int *idxs, int count);
int find_visible_prop(win_T *wp, int type_id, int id, textprop_T *prop, linenr_T *found_lnum);
void add_text_props(linenr_T lnum, textprop_T *text_props, int text_prop_count);
proptype_T *text_prop_type_by_id(buf_T *buf, int id);
int text_prop_type_valid(buf_T *buf, textprop_T *prop);
void f_prop_clear(typval_T *argvars, typval_T *rettv);
void f_prop_find(typval_T *argvars, typval_T *rettv);
void f_prop_list(typval_T *argvars, typval_T *rettv);
void f_prop_remove(typval_T *argvars, typval_T *rettv);
void f_prop_type_add(typval_T *argvars, typval_T *rettv);
void f_prop_type_change(typval_T *argvars, typval_T *rettv);
void f_prop_type_delete(typval_T *argvars, typval_T *rettv);
void f_prop_type_get(typval_T *argvars, typval_T *rettv);
void f_prop_type_list(typval_T *argvars, typval_T *rettv);
void clear_global_prop_types(void);
void clear_buf_prop_types(buf_T *buf);
int adjust_prop_columns(linenr_T lnum, colnr_T col, int bytes_added, int flags);
void adjust_props_for_split(linenr_T lnum_props, linenr_T lnum_top, int kept, int deleted, int at_eol);
void prepend_joined_props(char_u *new_props, int propcount, int *props_remaining, linenr_T lnum, int last_line, long col, int removed);
/* vim: set ft=c : */
