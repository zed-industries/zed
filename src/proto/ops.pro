/* ops.c */
int get_op_type(int char1, int char2);
int op_is_change(int op);
int get_op_char(int optype);
int get_extra_op_char(int optype);
void op_shift(oparg_T *oap, int curs_top, int amount);
void shift_line(int left, int round, int amount, int call_changed_bytes);
int op_delete(oparg_T *oap);
int op_replace(oparg_T *oap, int c);
int swapchar(int op_type, pos_T *pos);
void op_insert(oparg_T *oap, long count1);
int op_change(oparg_T *oap);
void adjust_cursor_eol(void);
char_u *skip_comment(char_u *line, int process, int include_space, int *is_comment);
int do_join(long count, int insert_space, int save_undo, int use_formatoptions, int setmark);
void block_prep(oparg_T *oap, struct block_def *bdp, linenr_T lnum, int is_del);
void op_addsub(oparg_T *oap, linenr_T Prenum1, int g_cmd);
void clear_oparg(oparg_T *oap);
void cursor_pos_info(dict_T *dict);
char *did_set_operatorfunc(optset_T *args);
void free_operatorfunc_option(void);
int set_ref_in_opfunc(int copyID);
void do_pending_operator(cmdarg_T *cap, int old_col, int gui_yank);
/* vim: set ft=c : */
