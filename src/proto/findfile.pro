/* findfile.c */
void *vim_findfile_init(char_u *path, char_u *filename, char_u *stopdirs, int level, int free_visited, int find_what, void *search_ctx_arg, int tagfile, char_u *rel_fname);
char_u *vim_findfile_stopdir(char_u *buf);
void vim_findfile_cleanup(void *ctx);
char_u *vim_findfile(void *search_ctx_arg);
char_u *find_file_in_path(char_u *ptr, int len, int options, int first, char_u *rel_fname, char_u **file_to_find, char **search_ctx);
void free_findfile(void);
char_u *find_directory_in_path(char_u *ptr, int len, int options, char_u *rel_fname, char_u **file_to_find, char **search_ctx);
char_u *find_file_in_path_option(char_u *ptr, int len, int options, int first, char_u *path_option, int find_what, char_u *rel_fname, char_u *suffixes, char_u **file_to_find, char **search_ctx_arg);
char_u *grab_file_name(long count, linenr_T *file_lnum);
char_u *file_name_at_cursor(int options, long count, linenr_T *file_lnum);
char_u *file_name_in_line(char_u *line, int col, int options, long count, char_u *rel_fname, linenr_T *file_lnum);
char_u *find_file_name_in_path(char_u *ptr, int len, int options, long count, char_u *rel_fname);
int vim_ispathlistsep(int c);
void uniquefy_paths(garray_T *gap, char_u *pattern);
int expand_in_path(garray_T *gap, char_u *pattern, int flags);
void simplify_filename(char_u *filename);
void f_simplify(typval_T *argvars, typval_T *rettv);
/* vim: set ft=c : */
