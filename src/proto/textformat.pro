/* textformat.c */
int has_format_option(int x);
void internal_format(int textwidth, int second_indent, int flags, int format_only, int c);
void auto_format(int trailblank, int prev_line);
void check_auto_format(int end_insert);
int comp_textwidth(int ff);
void op_format(oparg_T *oap, int keep_cursor);
void op_formatexpr(oparg_T *oap);
int fex_format(linenr_T lnum, long count, int c);
void format_lines(linenr_T line_count, int avoid_fex);
/* vim: set ft=c : */
