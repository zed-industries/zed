/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * MzScheme interface by Sergey Khorev <sergey.khorev@gmail.com>
 * Based on work by Brent Fulgham <bfulgham@debian.org>
 * (Based on lots of help from Matthew Flatt)
 *
 * This consists of six parts:
 * 1. MzScheme interpreter main program
 * 2. Routines that handle the external interface between MzScheme and
 *    Vim.
 * 3. MzScheme input/output handlers: writes output via [e]msg().
 * 4. Implementation of the Vim Features for MzScheme
 * 5. Vim Window-related Manipulation Functions.
 * 6. Vim Buffer-related Manipulation Functions
 *
 * NOTES
 * 1. Memory, allocated with scheme_malloc*, need not to be freed explicitly,
 *    garbage collector will do it self
 * 2. Requires at least NORMAL features. I can't imagine why one may want
 *    to build with TINY features but with MzScheme interface.
 * 3. I don't use K&R-style functions. Anyways, MzScheme headers are ANSI.
 */

#include "vim.h"

#include "if_mzsch.h"

// Only do the following when the feature is enabled.  Needed for "make
// depend".
#if defined(FEAT_MZSCHEME) || defined(PROTO)

#ifdef PROTO
typedef int Scheme_Object;
typedef int Scheme_Closed_Prim;
typedef int Scheme_Env;
typedef int Scheme_Hash_Table;
typedef int Scheme_Type;
typedef int Scheme_Thread;
typedef int Scheme_Closed_Prim;
typedef int mzshort;
typedef int Scheme_Prim;
typedef int HINSTANCE;
#endif

/*
 * scheme_register_tls_space is only available on 32-bit Windows until
 * racket-6.3.  See
 * http://docs.racket-lang.org/inside/im_memoryalloc.html?q=scheme_register_tls_space
 */
#if MZSCHEME_VERSION_MAJOR >= 500 && defined(MSWIN) \
	&& defined(USE_THREAD_LOCAL) \
	&& (!defined(_WIN64) || MZSCHEME_VERSION_MAJOR >= 603)
# define HAVE_TLS_SPACE 1
#endif

/*
 * Since version 4.x precise GC requires trampolined startup.
 * Futures and places in version 5.x need it too.
 */
#if defined(MZ_PRECISE_GC) && MZSCHEME_VERSION_MAJOR >= 400 \
    || MZSCHEME_VERSION_MAJOR >= 500 \
	&& (defined(MZ_USE_FUTURES) || defined(MZ_USE_PLACES))
# define TRAMPOLINED_MZVIM_STARTUP
#endif

// Base data structures
#define SCHEME_VIMBUFFERP(obj)  SAME_TYPE(SCHEME_TYPE(obj), mz_buffer_type)
#define SCHEME_VIMWINDOWP(obj)  SAME_TYPE(SCHEME_TYPE(obj), mz_window_type)

typedef struct
{
    Scheme_Object   so;
    buf_T	    *buf;
} vim_mz_buffer;

#define INVALID_BUFFER_VALUE ((buf_T *)(-1))

typedef struct
{
    Scheme_Object   so;
    win_T	    *win;
} vim_mz_window;

#define INVALID_WINDOW_VALUE ((win_T *)(-1))

/*
 * Prims that form MzScheme Vim interface
 */
typedef struct
{
    Scheme_Closed_Prim	*prim;
    char	*name;
    int		mina;	// arity information
    int		maxa;
} Vim_Prim;

typedef struct
{
    char	    *name;
    Scheme_Object   *port;
} Port_Info;

/*
 *========================================================================
 *  Vim-Control Commands
 *========================================================================
 */
/*
 *========================================================================
 *  Utility functions for the vim/mzscheme interface
 *========================================================================
 */
#ifdef HAVE_SANDBOX
static Scheme_Object *sandbox_file_guard(int, Scheme_Object **);
static Scheme_Object *sandbox_network_guard(int, Scheme_Object **);
static void sandbox_check(void);
#endif
//  Buffer-related commands
static Scheme_Object *buffer_new(buf_T *buf);
static Scheme_Object *get_buffer_by_num(void *, int, Scheme_Object **);
static vim_mz_buffer *get_vim_curr_buffer(void);

//  Window-related commands
static Scheme_Object *window_new(win_T *win);
static vim_mz_window *get_vim_curr_window(void);

/*
 *========================================================================
 *  Internal Function Prototypes
 *========================================================================
 */
static int vim_error_check(void);
static int do_mzscheme_command(exarg_T *, void *, Scheme_Closed_Prim *what);
static int startup_mzscheme(void);
static char *string_to_line(Scheme_Object *obj);
#if MZSCHEME_VERSION_MAJOR >= 501
# define OUTPUT_LEN_TYPE intptr_t
#else
# define OUTPUT_LEN_TYPE long
#endif
static void do_output(char *mesg, OUTPUT_LEN_TYPE len);
static void do_printf(char *format, ...) ATTRIBUTE_FORMAT_PRINTF(1, 2);
static void do_flush(void);
static Scheme_Object *_apply_thunk_catch_exceptions(
	Scheme_Object *, Scheme_Object **);
static Scheme_Object *extract_exn_message(Scheme_Object *v);
static Scheme_Object *do_eval(void *, int noargc, Scheme_Object **noargv);
static Scheme_Object *do_load(void *, int noargc, Scheme_Object **noargv);
static void register_vim_exn(void);
static vim_mz_buffer *get_buffer_arg(const char *fname, int argnum,
	int argc, Scheme_Object **argv);
static vim_mz_window *get_window_arg(const char *fname, int argnum,
	int argc, Scheme_Object **argv);
static int line_in_range(linenr_T, buf_T *);
static void check_line_range(linenr_T, buf_T *);
static void mz_fix_cursor(int lo, int hi, int extra);

static int eval_with_exn_handling(void *, Scheme_Closed_Prim *,
	    Scheme_Object **ret);
static void make_modules(void);
static void init_exn_catching_apply(void);
static int mzscheme_env_main(Scheme_Env *env, int argc, char **argv);
static int mzscheme_init(void);
#ifdef FEAT_EVAL
static Scheme_Object *vim_to_mzscheme(typval_T *vim_value);
static Scheme_Object *vim_to_mzscheme_impl(typval_T *vim_value, int depth,
	Scheme_Hash_Table *visited);
static int mzscheme_to_vim(Scheme_Object *obj, typval_T *tv);
static int mzscheme_to_vim_impl(Scheme_Object *obj, typval_T *tv, int depth,
	Scheme_Hash_Table *visited);
static Scheme_Object *vim_funcref(void *data, int argc, Scheme_Object **argv);
#endif

#ifdef MZ_PRECISE_GC
static int buffer_size_proc(void *obj UNUSED)
{
    return gcBYTES_TO_WORDS(sizeof(vim_mz_buffer));
}
static int buffer_mark_proc(void *obj)
{
    return buffer_size_proc(obj);
}
static int buffer_fixup_proc(void *obj)
{
    // apparently not needed as the object will be uncollectable while
    // the buffer is alive
    // vim_mz_buffer* buf = (vim_mz_buffer*) obj;
    // buf->buf->b_mzscheme_ref = GC_fixup_self(obj);

    return buffer_size_proc(obj);
}
static int window_size_proc(void *obj UNUSED)
{
    return gcBYTES_TO_WORDS(sizeof(vim_mz_window));
}
static int window_mark_proc(void *obj)
{
    return window_size_proc(obj);
}
static int window_fixup_proc(void *obj)
{
    // apparently not needed as the object will be uncollectable while
    // the window is alive
    // vim_mz_window* win = (vim_mz_window*) obj;
    // win->win->w_mzscheme_ref = GC_fixup_self(obj);
    //
    return window_size_proc(obj);
}
// with precise GC, w_mzscheme_ref and b_mzscheme_ref are immobile boxes
// containing pointers to a window/buffer
// with conservative GC these are simply pointers
# define WINDOW_REF(win) *(vim_mz_window **)((win)->w_mzscheme_ref)
# define BUFFER_REF(buf) *(vim_mz_buffer **)((buf)->b_mzscheme_ref)
#else
# define WINDOW_REF(win) (vim_mz_window *)((win)->w_mzscheme_ref)
# define BUFFER_REF(buf) (vim_mz_buffer *)((buf)->b_mzscheme_ref)
#endif

#if defined(DYNAMIC_MZSCHEME) || defined(PROTO)
static Scheme_Object *dll_scheme_eof;
static Scheme_Object *dll_scheme_false;
static Scheme_Object *dll_scheme_void;
static Scheme_Object *dll_scheme_null;
static Scheme_Object *dll_scheme_true;

static Scheme_Thread **dll_scheme_current_thread_ptr;

static void (**dll_scheme_console_printf_ptr)(char *str, ...);
static void (**dll_scheme_console_output_ptr)(char *str, OUTPUT_LEN_TYPE len);
static void (**dll_scheme_notify_multithread_ptr)(int on);

static void *(*dll_GC_malloc)(size_t size_in_bytes);
static void *(*dll_GC_malloc_atomic)(size_t size_in_bytes);
static Scheme_Env *(*dll_scheme_basic_env)(void);
static void (*dll_scheme_check_threads)(void);
static void (*dll_scheme_register_static)(void *ptr, long size);
static void (*dll_scheme_set_stack_base)(void *base, int no_auto_statics);
static void (*dll_scheme_add_global)(const char *name, Scheme_Object *val,
	Scheme_Env *env);
static void (*dll_scheme_add_global_symbol)(Scheme_Object *name,
	Scheme_Object *val, Scheme_Env *env);
static Scheme_Object *(*dll_scheme_apply)(Scheme_Object *rator, int num_rands,
	Scheme_Object **rands);
static Scheme_Object *(*dll_scheme_builtin_value)(const char *name);
# if MZSCHEME_VERSION_MAJOR >= 299
static Scheme_Object *(*dll_scheme_byte_string_to_char_string)(Scheme_Object *s);
static Scheme_Object *(*dll_scheme_make_path)(const char *chars);
# endif
static void (*dll_scheme_close_input_port)(Scheme_Object *port);
static void (*dll_scheme_count_lines)(Scheme_Object *port);
#if MZSCHEME_VERSION_MAJOR < 360
static Scheme_Object *(*dll_scheme_current_continuation_marks)(void);
#else
static Scheme_Object *(*dll_scheme_current_continuation_marks)(Scheme_Object *prompt_tag);
#endif
static void (*dll_scheme_display)(Scheme_Object *obj, Scheme_Object *port);
static char *(*dll_scheme_display_to_string)(Scheme_Object *obj, OUTPUT_LEN_TYPE *len);
static int (*dll_scheme_eq)(Scheme_Object *obj1, Scheme_Object *obj2);
static Scheme_Object *(*dll_scheme_do_eval)(Scheme_Object *obj,
	int _num_rands, Scheme_Object **rands, int val);
static void (*dll_scheme_dont_gc_ptr)(void *p);
static Scheme_Object *(*dll_scheme_eval)(Scheme_Object *obj, Scheme_Env *env);
static Scheme_Object *(*dll_scheme_eval_string)(const char *str,
	Scheme_Env *env);
static Scheme_Object *(*dll_scheme_eval_string_all)(const char *str,
	Scheme_Env *env, int all);
static void (*dll_scheme_finish_primitive_module)(Scheme_Env *env);
# if MZSCHEME_VERSION_MAJOR < 299
static char *(*dll_scheme_format)(char *format, int flen, int argc,
	Scheme_Object **argv, long *rlen);
# else
static char *(*dll_scheme_format_utf8)(char *format, int flen, int argc,
	Scheme_Object **argv, OUTPUT_LEN_TYPE *rlen);
static Scheme_Object *(*dll_scheme_get_param)(Scheme_Config *c, int pos);
# endif
static void (*dll_scheme_gc_ptr_ok)(void *p);
# if MZSCHEME_VERSION_MAJOR < 299
static char *(*dll_scheme_get_sized_string_output)(Scheme_Object *,
	long *len);
# else
static char *(*dll_scheme_get_sized_byte_string_output)(Scheme_Object *,
	OUTPUT_LEN_TYPE *len);
# endif
static Scheme_Object *(*dll_scheme_intern_symbol)(const char *name);
static Scheme_Object *(*dll_scheme_lookup_global)(Scheme_Object *symbol,
	Scheme_Env *env);
static Scheme_Object *(*dll_scheme_make_closed_prim_w_arity)
    (Scheme_Closed_Prim *prim, void *data, const char *name, mzshort mina,
     mzshort maxa);
static Scheme_Object *(*dll_scheme_make_integer_value)(long i);
static Scheme_Object *(*dll_scheme_make_pair)(Scheme_Object *car,
	Scheme_Object *cdr);
static Scheme_Object *(*dll_scheme_make_prim_w_arity)(Scheme_Prim *prim,
	const char *name, mzshort mina, mzshort maxa);
# if MZSCHEME_VERSION_MAJOR < 299
static Scheme_Object *(*dll_scheme_make_string)(const char *chars);
static Scheme_Object *(*dll_scheme_make_string_output_port)();
# else
static Scheme_Object *(*dll_scheme_make_byte_string)(const char *chars);
static Scheme_Object *(*dll_scheme_make_byte_string_output_port)();
# endif
static Scheme_Object *(*dll_scheme_make_struct_instance)(Scheme_Object *stype,
	int argc, Scheme_Object **argv);
static Scheme_Object **(*dll_scheme_make_struct_names)(Scheme_Object *base,
	Scheme_Object *field_names, int flags, int *count_out);
static Scheme_Object *(*dll_scheme_make_struct_type)(Scheme_Object *base,
	Scheme_Object *parent, Scheme_Object *inspector, int num_fields,
	int num_uninit_fields, Scheme_Object *uninit_val,
	Scheme_Object *properties
# if MZSCHEME_VERSION_MAJOR >= 299
	, Scheme_Object *guard
# endif
	);
static Scheme_Object **(*dll_scheme_make_struct_values)(
	Scheme_Object *struct_type, Scheme_Object **names, int count,
	int flags);
static Scheme_Type (*dll_scheme_make_type)(const char *name);
static Scheme_Object *(*dll_scheme_make_vector)(int size,
	Scheme_Object *fill);
static void *(*dll_scheme_malloc_fail_ok)(void *(*f)(size_t), size_t);
static Scheme_Object *(*dll_scheme_open_input_file)(const char *name,
	const char *who);
static Scheme_Env *(*dll_scheme_primitive_module)(Scheme_Object *name,
	Scheme_Env *for_env);
static int (*dll_scheme_proper_list_length)(Scheme_Object *list);
static void (*dll_scheme_raise)(Scheme_Object *exn);
static Scheme_Object *(*dll_scheme_read)(Scheme_Object *port);
static void (*dll_scheme_signal_error)(const char *msg, ...);
static void (*dll_scheme_wrong_type)(const char *name, const char *expected,
	int which, int argc, Scheme_Object **argv);
# if MZSCHEME_VERSION_MAJOR >= 299
static void (*dll_scheme_set_param)(Scheme_Config *c, int pos,
	Scheme_Object *o);
static Scheme_Config *(*dll_scheme_current_config)(void);
static Scheme_Object *(*dll_scheme_char_string_to_byte_string)
    (Scheme_Object *s);
static Scheme_Object *(*dll_scheme_char_string_to_path)
    (Scheme_Object *s);
static void *(*dll_scheme_set_collects_path)(Scheme_Object *p);
# endif
static Scheme_Hash_Table *(*dll_scheme_make_hash_table)(int type);
static void (*dll_scheme_hash_set)(Scheme_Hash_Table *table,
	Scheme_Object *key, Scheme_Object *value);
static Scheme_Object *(*dll_scheme_hash_get)(Scheme_Hash_Table *table,
	Scheme_Object *key);
static Scheme_Object *(*dll_scheme_make_double)(double d);
static Scheme_Object *(*dll_scheme_make_sized_byte_string)(char *chars,
	long len, int copy);
static Scheme_Object *(*dll_scheme_namespace_require)(Scheme_Object *req);
static Scheme_Object *(*dll_scheme_dynamic_wind)(void (*pre)(void *), Scheme_Object *(* volatile act)(void *), void (* volatile post)(void *), Scheme_Object *(*jmp_handler)(void *), void * volatile data);
# ifdef MZ_PRECISE_GC
static void *(*dll_GC_malloc_one_tagged)(size_t size_in_bytes);
static void (*dll_GC_register_traversers)(short tag, Size_Proc size, Mark_Proc mark, Fixup_Proc fixup, int is_constant_size, int is_atomic);
# endif
# if MZSCHEME_VERSION_MAJOR >= 400
static void (*dll_scheme_init_collection_paths)(Scheme_Env *global_env, Scheme_Object *extra_dirs);
static void **(*dll_scheme_malloc_immobile_box)(void *p);
static void (*dll_scheme_free_immobile_box)(void **b);
# endif
# if MZSCHEME_VERSION_MAJOR >= 500
#  ifdef TRAMPOLINED_MZVIM_STARTUP
static int (*dll_scheme_main_setup)(int no_auto_statics, Scheme_Env_Main _main, int argc, char **argv);
#   if defined(IMPLEMENT_THREAD_LOCAL_VIA_WIN_TLS) || MZSCHEME_VERSION_MAJOR >= 603
static void (*dll_scheme_register_tls_space)(void *tls_space, int _tls_index);
#   endif
#  endif
#  if defined(IMPLEMENT_THREAD_LOCAL_VIA_WIN_TLS) || defined(IMPLEMENT_THREAD_LOCAL_EXTERNALLY_VIA_PROC)
static Thread_Local_Variables *(*dll_scheme_external_get_thread_local_variables)(void);
#  endif
# endif
# if MZSCHEME_VERSION_MAJOR >= 600
static void (*dll_scheme_embedded_load)(intptr_t len, const char *s, int predefined);
static void (*dll_scheme_register_embedded_load)(intptr_t len, const char *s);
static void (*dll_scheme_set_config_path)(Scheme_Object *p);
# endif

#if defined(DYNAMIC_MZSCHEME) // not when defined(PROTO)

// arrays are imported directly
# define scheme_eof dll_scheme_eof
# define scheme_false dll_scheme_false
# define scheme_void dll_scheme_void
# define scheme_null dll_scheme_null
# define scheme_true dll_scheme_true

// pointers are GetProceAddress'ed as pointers to pointer
#if !defined(USE_THREAD_LOCAL) && !defined(LINK_EXTENSIONS_BY_TABLE)
#  define scheme_current_thread (*dll_scheme_current_thread_ptr)
# endif
# define scheme_console_printf (*dll_scheme_console_printf_ptr)
# define scheme_console_output (*dll_scheme_console_output_ptr)
# define scheme_notify_multithread (*dll_scheme_notify_multithread_ptr)

// and functions in a usual way
# define GC_malloc dll_GC_malloc
# define GC_malloc_atomic dll_GC_malloc_atomic

# define scheme_add_global dll_scheme_add_global
# define scheme_add_global_symbol dll_scheme_add_global_symbol
# define scheme_apply dll_scheme_apply
# define scheme_basic_env dll_scheme_basic_env
# define scheme_builtin_value dll_scheme_builtin_value
# if MZSCHEME_VERSION_MAJOR >= 299
#  define scheme_byte_string_to_char_string dll_scheme_byte_string_to_char_string
#  define scheme_make_path dll_scheme_make_path
# endif
# define scheme_check_threads dll_scheme_check_threads
# define scheme_close_input_port dll_scheme_close_input_port
# define scheme_count_lines dll_scheme_count_lines
# define scheme_current_continuation_marks \
    dll_scheme_current_continuation_marks
# define scheme_display dll_scheme_display
# define scheme_display_to_string dll_scheme_display_to_string
# define scheme_do_eval dll_scheme_do_eval
# define scheme_dont_gc_ptr dll_scheme_dont_gc_ptr
# define scheme_eq dll_scheme_eq
# define scheme_eval dll_scheme_eval
# define scheme_eval_string dll_scheme_eval_string
# define scheme_eval_string_all dll_scheme_eval_string_all
# define scheme_finish_primitive_module dll_scheme_finish_primitive_module
# if MZSCHEME_VERSION_MAJOR < 299
#  define scheme_format dll_scheme_format
# else
#  define scheme_format_utf8 dll_scheme_format_utf8
# endif
# define scheme_gc_ptr_ok dll_scheme_gc_ptr_ok
# if MZSCHEME_VERSION_MAJOR < 299
#  define scheme_get_sized_byte_string_output dll_scheme_get_sized_string_output
# else
#  define scheme_get_sized_byte_string_output \
    dll_scheme_get_sized_byte_string_output
#  define scheme_get_param dll_scheme_get_param
# endif
# define scheme_intern_symbol dll_scheme_intern_symbol
# define scheme_lookup_global dll_scheme_lookup_global
# define scheme_make_closed_prim_w_arity dll_scheme_make_closed_prim_w_arity
# define scheme_make_integer_value dll_scheme_make_integer_value
# define scheme_make_pair dll_scheme_make_pair
# define scheme_make_prim_w_arity dll_scheme_make_prim_w_arity
# if MZSCHEME_VERSION_MAJOR < 299
#  define scheme_make_byte_string dll_scheme_make_string
#  define scheme_make_byte_string_output_port dll_scheme_make_string_output_port
# else
#  define scheme_make_byte_string dll_scheme_make_byte_string
#  define scheme_make_byte_string_output_port \
    dll_scheme_make_byte_string_output_port
# endif
# define scheme_make_struct_instance dll_scheme_make_struct_instance
# define scheme_make_struct_names dll_scheme_make_struct_names
# define scheme_make_struct_type dll_scheme_make_struct_type
# define scheme_make_struct_values dll_scheme_make_struct_values
# define scheme_make_type dll_scheme_make_type
# define scheme_make_vector dll_scheme_make_vector
# define scheme_malloc_fail_ok dll_scheme_malloc_fail_ok
# define scheme_open_input_file dll_scheme_open_input_file
# define scheme_primitive_module dll_scheme_primitive_module
# define scheme_proper_list_length dll_scheme_proper_list_length
# define scheme_raise dll_scheme_raise
# define scheme_read dll_scheme_read
# define scheme_register_static dll_scheme_register_static
# define scheme_set_stack_base dll_scheme_set_stack_base
# define scheme_signal_error dll_scheme_signal_error
# define scheme_wrong_type dll_scheme_wrong_type
# if MZSCHEME_VERSION_MAJOR >= 299
#  define scheme_set_param dll_scheme_set_param
#  define scheme_current_config dll_scheme_current_config
#  define scheme_char_string_to_byte_string \
    dll_scheme_char_string_to_byte_string
#  define scheme_char_string_to_path \
    dll_scheme_char_string_to_path
#  define scheme_set_collects_path dll_scheme_set_collects_path
# endif
# define scheme_make_hash_table dll_scheme_make_hash_table
# define scheme_hash_set dll_scheme_hash_set
# define scheme_hash_get dll_scheme_hash_get
# define scheme_make_double dll_scheme_make_double
# define scheme_make_sized_byte_string dll_scheme_make_sized_byte_string
# define scheme_namespace_require dll_scheme_namespace_require
# define scheme_dynamic_wind dll_scheme_dynamic_wind
# ifdef MZ_PRECISE_GC
#  define GC_malloc_one_tagged dll_GC_malloc_one_tagged
#  define GC_register_traversers dll_GC_register_traversers
# endif
# if MZSCHEME_VERSION_MAJOR >= 400
#  ifdef TRAMPOLINED_MZVIM_STARTUP
#   define scheme_main_setup dll_scheme_main_setup
#   if defined(IMPLEMENT_THREAD_LOCAL_VIA_WIN_TLS) || MZSCHEME_VERSION_MAJOR >= 603
#    define scheme_register_tls_space dll_scheme_register_tls_space
#   endif
#  endif
#  define scheme_init_collection_paths dll_scheme_init_collection_paths
#  define scheme_malloc_immobile_box dll_scheme_malloc_immobile_box
#  define scheme_free_immobile_box dll_scheme_free_immobile_box
# endif
# if MZSCHEME_VERSION_MAJOR >= 600
#  define scheme_embedded_load dll_scheme_embedded_load
#  define scheme_register_embedded_load dll_scheme_register_embedded_load
#  define scheme_set_config_path dll_scheme_set_config_path
# endif

# if MZSCHEME_VERSION_MAJOR >= 500
#  if defined(IMPLEMENT_THREAD_LOCAL_VIA_WIN_TLS) || defined(IMPLEMENT_THREAD_LOCAL_EXTERNALLY_VIA_PROC)
// define as function for macro in schthread.h
Thread_Local_Variables *
scheme_external_get_thread_local_variables(void)
{
    return dll_scheme_external_get_thread_local_variables();
}
#  endif
# endif

#endif

typedef struct
{
    char    *name;
    void    **ptr;
} Thunk_Info;

static Thunk_Info mzgc_imports[] = {
    {"GC_malloc", (void **)&dll_GC_malloc},
    {"GC_malloc_atomic", (void **)&dll_GC_malloc_atomic},
    {NULL, NULL}};

static Thunk_Info mzsch_imports[] = {
    {"scheme_eof", (void **)&dll_scheme_eof},
    {"scheme_false", (void **)&dll_scheme_false},
    {"scheme_void", (void **)&dll_scheme_void},
    {"scheme_null", (void **)&dll_scheme_null},
    {"scheme_true", (void **)&dll_scheme_true},
#if !defined(USE_THREAD_LOCAL) && !defined(LINK_EXTENSIONS_BY_TABLE)
    {"scheme_current_thread", (void **)&dll_scheme_current_thread_ptr},
#endif
    {"scheme_console_printf", (void **)&dll_scheme_console_printf_ptr},
    {"scheme_console_output", (void **)&dll_scheme_console_output_ptr},
    {"scheme_notify_multithread",
	(void **)&dll_scheme_notify_multithread_ptr},
    {"scheme_add_global", (void **)&dll_scheme_add_global},
    {"scheme_add_global_symbol", (void **)&dll_scheme_add_global_symbol},
    {"scheme_apply", (void **)&dll_scheme_apply},
    {"scheme_basic_env", (void **)&dll_scheme_basic_env},
# if MZSCHEME_VERSION_MAJOR >= 299
    {"scheme_byte_string_to_char_string", (void **)&dll_scheme_byte_string_to_char_string},
    {"scheme_make_path", (void **)&dll_scheme_make_path},
# endif
    {"scheme_builtin_value", (void **)&dll_scheme_builtin_value},
    {"scheme_check_threads", (void **)&dll_scheme_check_threads},
    {"scheme_close_input_port", (void **)&dll_scheme_close_input_port},
    {"scheme_count_lines", (void **)&dll_scheme_count_lines},
    {"scheme_current_continuation_marks",
	(void **)&dll_scheme_current_continuation_marks},
    {"scheme_display", (void **)&dll_scheme_display},
    {"scheme_display_to_string", (void **)&dll_scheme_display_to_string},
    {"scheme_do_eval", (void **)&dll_scheme_do_eval},
    {"scheme_dont_gc_ptr", (void **)&dll_scheme_dont_gc_ptr},
    {"scheme_eq", (void **)&dll_scheme_eq},
    {"scheme_eval", (void **)&dll_scheme_eval},
    {"scheme_eval_string", (void **)&dll_scheme_eval_string},
    {"scheme_eval_string_all", (void **)&dll_scheme_eval_string_all},
    {"scheme_finish_primitive_module",
	(void **)&dll_scheme_finish_primitive_module},
# if MZSCHEME_VERSION_MAJOR < 299
    {"scheme_format", (void **)&dll_scheme_format},
# else
    {"scheme_format_utf8", (void **)&dll_scheme_format_utf8},
    {"scheme_get_param", (void **)&dll_scheme_get_param},
#endif
    {"scheme_gc_ptr_ok", (void **)&dll_scheme_gc_ptr_ok},
# if MZSCHEME_VERSION_MAJOR < 299
    {"scheme_get_sized_string_output",
	(void **)&dll_scheme_get_sized_string_output},
# else
    {"scheme_get_sized_byte_string_output",
	(void **)&dll_scheme_get_sized_byte_string_output},
#endif
    {"scheme_intern_symbol", (void **)&dll_scheme_intern_symbol},
    {"scheme_lookup_global", (void **)&dll_scheme_lookup_global},
    {"scheme_make_closed_prim_w_arity",
	(void **)&dll_scheme_make_closed_prim_w_arity},
    {"scheme_make_integer_value", (void **)&dll_scheme_make_integer_value},
    {"scheme_make_pair", (void **)&dll_scheme_make_pair},
    {"scheme_make_prim_w_arity", (void **)&dll_scheme_make_prim_w_arity},
# if MZSCHEME_VERSION_MAJOR < 299
    {"scheme_make_string", (void **)&dll_scheme_make_string},
    {"scheme_make_string_output_port",
	(void **)&dll_scheme_make_string_output_port},
# else
    {"scheme_make_byte_string", (void **)&dll_scheme_make_byte_string},
    {"scheme_make_byte_string_output_port",
	(void **)&dll_scheme_make_byte_string_output_port},
# endif
    {"scheme_make_struct_instance",
	(void **)&dll_scheme_make_struct_instance},
    {"scheme_make_struct_names", (void **)&dll_scheme_make_struct_names},
    {"scheme_make_struct_type", (void **)&dll_scheme_make_struct_type},
    {"scheme_make_struct_values", (void **)&dll_scheme_make_struct_values},
    {"scheme_make_type", (void **)&dll_scheme_make_type},
    {"scheme_make_vector", (void **)&dll_scheme_make_vector},
    {"scheme_malloc_fail_ok", (void **)&dll_scheme_malloc_fail_ok},
    {"scheme_open_input_file", (void **)&dll_scheme_open_input_file},
    {"scheme_primitive_module", (void **)&dll_scheme_primitive_module},
    {"scheme_proper_list_length", (void **)&dll_scheme_proper_list_length},
    {"scheme_raise", (void **)&dll_scheme_raise},
    {"scheme_read", (void **)&dll_scheme_read},
    {"scheme_register_static", (void **)&dll_scheme_register_static},
    {"scheme_set_stack_base", (void **)&dll_scheme_set_stack_base},
    {"scheme_signal_error", (void **)&dll_scheme_signal_error},
    {"scheme_wrong_type", (void **)&dll_scheme_wrong_type},
# if MZSCHEME_VERSION_MAJOR >= 299
    {"scheme_set_param", (void **)&dll_scheme_set_param},
    {"scheme_current_config", (void **)&dll_scheme_current_config},
    {"scheme_char_string_to_byte_string",
	(void **)&dll_scheme_char_string_to_byte_string},
    {"scheme_char_string_to_path", (void **)&dll_scheme_char_string_to_path},
    {"scheme_set_collects_path", (void **)&dll_scheme_set_collects_path},
# endif
    {"scheme_make_hash_table", (void **)&dll_scheme_make_hash_table},
    {"scheme_hash_set", (void **)&dll_scheme_hash_set},
    {"scheme_hash_get", (void **)&dll_scheme_hash_get},
    {"scheme_make_double", (void **)&dll_scheme_make_double},
    {"scheme_make_sized_byte_string", (void **)&dll_scheme_make_sized_byte_string},
    {"scheme_namespace_require", (void **)&dll_scheme_namespace_require},
    {"scheme_dynamic_wind", (void **)&dll_scheme_dynamic_wind},
# ifdef MZ_PRECISE_GC
    {"GC_malloc_one_tagged", (void **)&dll_GC_malloc_one_tagged},
    {"GC_register_traversers", (void **)&dll_GC_register_traversers},
# endif
# if MZSCHEME_VERSION_MAJOR >= 400
#  ifdef TRAMPOLINED_MZVIM_STARTUP
    {"scheme_main_setup", (void **)&dll_scheme_main_setup},
#   if defined(IMPLEMENT_THREAD_LOCAL_VIA_WIN_TLS) || MZSCHEME_VERSION_MAJOR >= 603
    {"scheme_register_tls_space", (void **)&dll_scheme_register_tls_space},
#   endif
#  endif
    {"scheme_init_collection_paths", (void **)&dll_scheme_init_collection_paths},
    {"scheme_malloc_immobile_box", (void **)&dll_scheme_malloc_immobile_box},
    {"scheme_free_immobile_box", (void **)&dll_scheme_free_immobile_box},
# endif
# if MZSCHEME_VERSION_MAJOR >= 500
#  if defined(IMPLEMENT_THREAD_LOCAL_VIA_WIN_TLS) || defined(IMPLEMENT_THREAD_LOCAL_EXTERNALLY_VIA_PROC)
    {"scheme_external_get_thread_local_variables", (void **)&dll_scheme_external_get_thread_local_variables},
#  endif
# endif
# if MZSCHEME_VERSION_MAJOR >= 600
    {"scheme_embedded_load", (void **)&dll_scheme_embedded_load},
    {"scheme_register_embedded_load", (void **)&dll_scheme_register_embedded_load},
    {"scheme_set_config_path", (void **)&dll_scheme_set_config_path},
# endif
    {NULL, NULL}};

static HINSTANCE hMzGC = 0;
static HINSTANCE hMzSch = 0;

static void dynamic_mzscheme_end(void);
static int mzscheme_runtime_link_init(char *sch_dll, char *gc_dll,
	int verbose);

    static int
mzscheme_runtime_link_init(char *sch_dll, char *gc_dll, int verbose)
{
    Thunk_Info *thunk = NULL;

    if (hMzGC && hMzSch)
	return OK;
    hMzSch = vimLoadLib(sch_dll);
    hMzGC = vimLoadLib(gc_dll);

    if (!hMzGC)
    {
	if (verbose)
	    semsg(_(e_could_not_load_library_str_str), gc_dll, GetWin32Error());
	return FAIL;
    }

    if (!hMzSch)
    {
	if (verbose)
	    semsg(_(e_could_not_load_library_str_str), sch_dll, GetWin32Error());
	return FAIL;
    }

    for (thunk = mzsch_imports; thunk->name; thunk++)
    {
	if ((*thunk->ptr =
		    (void *)GetProcAddress(hMzSch, thunk->name)) == NULL)
	{
	    FreeLibrary(hMzSch);
	    hMzSch = 0;
	    FreeLibrary(hMzGC);
	    hMzGC = 0;
	    if (verbose)
		semsg(_(e_could_not_load_library_function_str), thunk->name);
	    return FAIL;
	}
    }
    for (thunk = mzgc_imports; thunk->name; thunk++)
    {
	if ((*thunk->ptr =
		    (void *)GetProcAddress(hMzGC, thunk->name)) == NULL)
	{
	    FreeLibrary(hMzSch);
	    hMzSch = 0;
	    FreeLibrary(hMzGC);
	    hMzGC = 0;
	    if (verbose)
		semsg(_(e_could_not_load_library_function_str), thunk->name);
	    return FAIL;
	}
    }
    return OK;
}

    int
mzscheme_enabled(int verbose)
{
    return mzscheme_runtime_link_init(
	    (char *)p_mzschemedll, (char *)p_mzschemegcdll, verbose) == OK;
}

    static void
dynamic_mzscheme_end(void)
{
    if (hMzSch)
    {
	FreeLibrary(hMzSch);
	hMzSch = 0;
    }
    if (hMzGC)
    {
	FreeLibrary(hMzGC);
	hMzGC = 0;
    }
}
#endif // DYNAMIC_MZSCHEME

#if MZSCHEME_VERSION_MAJOR < 299
# define GUARANTEED_STRING_ARG(proc, num) GUARANTEE_STRING(proc, num)
#else
    static Scheme_Object *
guaranteed_byte_string_arg(char *proc, int num, int argc, Scheme_Object **argv)
{
    if (SCHEME_BYTE_STRINGP(argv[num]))
    {
	return argv[num];
    }
    else if (SCHEME_CHAR_STRINGP(argv[num]))
    {
	Scheme_Object *tmp = NULL;
	MZ_GC_DECL_REG(2);
	MZ_GC_VAR_IN_REG(0, argv[num]);
	MZ_GC_VAR_IN_REG(1, tmp);
	MZ_GC_REG();
	tmp = scheme_char_string_to_byte_string(argv[num]);
	MZ_GC_UNREG();
	return tmp;
    }
    else
	scheme_wrong_type(proc, "string", num, argc, argv);
    // unreachable
    return scheme_void;
}
# define GUARANTEED_STRING_ARG(proc, num) guaranteed_byte_string_arg(proc, num, argc, argv)
#endif

// need to put it here for dynamic stuff to work
#if defined(INCLUDE_MZSCHEME_BASE)
# include "mzscheme_base.c"
#endif

/*
 *========================================================================
 *  1. MzScheme interpreter startup
 *========================================================================
 */

static Scheme_Type mz_buffer_type;
static Scheme_Type mz_window_type;

static int initialized = FALSE;
#ifdef DYNAMIC_MZSCHEME
static int disabled = FALSE;
#endif
static int load_base_module_failed = FALSE;

// global environment
static Scheme_Env    *environment = NULL;
// output/error handlers
static Scheme_Object *curout = NULL;
static Scheme_Object *curerr = NULL;
// exn:vim exception
static Scheme_Object *exn_catching_apply = NULL;
static Scheme_Object *exn_p = NULL;
static Scheme_Object *exn_message = NULL;
static Scheme_Object *vim_exn = NULL; // Vim Error exception

#if !defined(MZ_PRECISE_GC) || MZSCHEME_VERSION_MAJOR < 400
static void *stack_base = NULL;
#endif

static long range_start;
static long range_end;

// MzScheme threads scheduling stuff
static int mz_threads_allow = 0;

#if defined(FEAT_GUI_MSWIN)
static void CALLBACK timer_proc(HWND, UINT, UINT_PTR, DWORD);
static UINT timer_id = 0;
#elif defined(FEAT_GUI_GTK)
static gboolean timer_proc(gpointer);
static guint timer_id = 0;
#elif defined(FEAT_GUI_MOTIF)
static void timer_proc(XtPointer, XtIntervalId *);
static XtIntervalId timer_id = (XtIntervalId)0;
#endif

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL) // Win32 console and Unix
    void
mzvim_check_threads(void)
{
    // Last time MzScheme threads were scheduled
    static time_t mz_last_time = 0;

    if (mz_threads_allow && p_mzq > 0)
    {
	time_t now = time(NULL);

	if ((now - mz_last_time) * 1000 > p_mzq)
	{
	    mz_last_time = now;
	    scheme_check_threads();
	}
    }
}
#endif

#if defined(MZSCHEME_GUI_THREADS) || defined(PROTO)
static void setup_timer(void);
static void remove_timer(void);

// timers are presented in GUI only
# if defined(FEAT_GUI_MSWIN)
    static void CALLBACK
timer_proc(HWND hwnd UNUSED, UINT uMsg UNUSED, UINT_PTR idEvent UNUSED, DWORD dwTime UNUSED)
# elif defined(FEAT_GUI_GTK)
    static gboolean
timer_proc(gpointer data UNUSED)
# elif defined(FEAT_GUI_MOTIF)
    static void
timer_proc(XtPointer timed_out UNUSED, XtIntervalId *interval_id UNUSED)
# endif
{
    scheme_check_threads();
# if defined(FEAT_GUI_GTK)
    return TRUE; // continue receiving notifications
# elif defined(FEAT_GUI_MOTIF)
    // renew timeout
    if (mz_threads_allow && p_mzq > 0)
	timer_id = XtAppAddTimeOut(app_context, p_mzq,
		timer_proc, NULL);
# endif
}

    static void
setup_timer(void)
{
# if defined(FEAT_GUI_MSWIN)
    timer_id = SetTimer(NULL, 0, p_mzq, timer_proc);
# elif defined(FEAT_GUI_GTK)
    timer_id = g_timeout_add((guint)p_mzq, (GSourceFunc)timer_proc, NULL);
# elif defined(FEAT_GUI_MOTIF)
    timer_id = XtAppAddTimeOut(app_context, p_mzq, timer_proc, NULL);
# endif
}

    static void
remove_timer(void)
{
# if defined(FEAT_GUI_MSWIN)
    KillTimer(NULL, timer_id);
# elif defined(FEAT_GUI_GTK)
    g_source_remove(timer_id);
# elif defined(FEAT_GUI_MOTIF)
    XtRemoveTimeOut(timer_id);
# endif
    timer_id = 0;
}

#endif // MZSCHEME_GUI_THREADS

    char *
did_set_mzquantum(optset_T *args UNUSED)
{
#if defined(MZSCHEME_GUI_THREADS)
    // reset timer
    if (timer_id != 0)
	remove_timer();
    if (mz_threads_allow && p_mzq > 0 && gui.in_use)
	setup_timer();
#endif
    return NULL;
}

    static void
notify_multithread(int on)
{
    mz_threads_allow = on;
#ifdef MZSCHEME_GUI_THREADS
    if (on && timer_id == 0 && p_mzq > 0 && gui.in_use)
	setup_timer();
    if (!on && timer_id != 0)
	remove_timer();
#endif
}

    void
mzscheme_end(void)
{
    // We can not unload the DLL.  Racket's thread might be still alive.
#if 0
#ifdef DYNAMIC_MZSCHEME
    dynamic_mzscheme_end();
#endif
#endif
}

#if HAVE_TLS_SPACE
# if defined(_MSC_VER)
static __declspec(thread) void *tls_space;
extern intptr_t _tls_index;
# elif defined(__MINGW32__)
static __thread void *tls_space;
extern intptr_t _tls_index;
# else
static THREAD_LOCAL void *tls_space;
static intptr_t _tls_index = 0;
# endif
#endif

/*
 * mzscheme_main() is called early in main().
 * We may call scheme_main_setup() which calls mzscheme_env_main() which then
 * trampolines into vim_main2(), which never returns.
 */
    int
mzscheme_main(void)
{
    int	    argc = 0;
    char    *argv = NULL;

#ifdef DYNAMIC_MZSCHEME
    /*
     * Racket requires trampolined startup.  We can not load it later.
     * If dynamic dll loading is failed, disable it.
     */
    if (!mzscheme_enabled(FALSE))
    {
	disabled = TRUE;
	return vim_main2();
    }
#endif
#ifdef HAVE_TLS_SPACE
    scheme_register_tls_space(&tls_space, _tls_index);
#endif
#ifdef TRAMPOLINED_MZVIM_STARTUP
    return scheme_main_setup(TRUE, mzscheme_env_main, argc, &argv);
#else
    return mzscheme_env_main(NULL, argc, &argv);
#endif
}

    static int
mzscheme_env_main(Scheme_Env *env, int argc UNUSED, char **argv UNUSED)
{
#ifdef TRAMPOLINED_MZVIM_STARTUP
    // Scheme has created the environment for us
    environment = env;
#else
# ifdef MZ_PRECISE_GC
    Scheme_Object   *dummy = NULL;
    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, dummy);

    stack_base = &__gc_var_stack__;
# else
    int dummy = 0;
    stack_base = (void *)&dummy;
# endif
#endif

    vim_main2();
    // not reached, vim_main2() will loop until exit()

    return 0;
}

    static Scheme_Object*
load_base_module(void *data)
{
    scheme_namespace_require(scheme_intern_symbol((char *)data));
    return scheme_null;
}

    static Scheme_Object *
load_base_module_on_error(void *data UNUSED)
{
    load_base_module_failed = TRUE;
    return scheme_null;
}

    static int
startup_mzscheme(void)
{
#ifndef TRAMPOLINED_MZVIM_STARTUP
    scheme_set_stack_base(stack_base, 1);
#endif

#ifndef TRAMPOLINED_MZVIM_STARTUP
    // in newer versions of precise GC the initial env has been created
    environment = scheme_basic_env();
#endif

    MZ_REGISTER_STATIC(environment);
    MZ_REGISTER_STATIC(curout);
    MZ_REGISTER_STATIC(curerr);
    MZ_REGISTER_STATIC(exn_catching_apply);
    MZ_REGISTER_STATIC(exn_p);
    MZ_REGISTER_STATIC(exn_message);
    MZ_REGISTER_STATIC(vim_exn);

    MZ_GC_CHECK();

#ifdef INCLUDE_MZSCHEME_BASE
    // invoke function from generated and included mzscheme_base.c
    declare_modules(environment);
#endif

    // setup 'current-library-collection-paths' parameter
# if MZSCHEME_VERSION_MAJOR >= 299
    {
	Scheme_Object *coll_path = NULL;
	int mustfree = FALSE;
	char_u *s;

	MZ_GC_DECL_REG(1);
	MZ_GC_VAR_IN_REG(0, coll_path);
	MZ_GC_REG();
	// workaround for dynamic loading on windows
	s = vim_getenv((char_u *)"PLTCOLLECTS", &mustfree);
	if (s != NULL)
	{
	    coll_path = scheme_make_path((char *)s);
	    MZ_GC_CHECK();
	    if (mustfree)
		vim_free(s);
	}
#  ifdef MZSCHEME_COLLECTS
	if (coll_path == NULL)
	{
	    coll_path = scheme_make_path(MZSCHEME_COLLECTS);
	    MZ_GC_CHECK();
	}
#  endif
	if (coll_path != NULL)
	{
	    scheme_set_collects_path(coll_path);
	    MZ_GC_CHECK();
	}
	MZ_GC_UNREG();
    }
# else
#  ifdef MZSCHEME_COLLECTS
    {
	Scheme_Object	*coll_string = NULL;
	Scheme_Object	*coll_pair = NULL;
	Scheme_Config	*config = NULL;

	MZ_GC_DECL_REG(3);
	MZ_GC_VAR_IN_REG(0, coll_string);
	MZ_GC_VAR_IN_REG(1, coll_pair);
	MZ_GC_VAR_IN_REG(2, config);
	MZ_GC_REG();
	coll_string = scheme_make_byte_string(MZSCHEME_COLLECTS);
	MZ_GC_CHECK();
	coll_pair = scheme_make_pair(coll_string, scheme_null);
	MZ_GC_CHECK();
	config = scheme_current_config();
	MZ_GC_CHECK();
	scheme_set_param(config, MZCONFIG_COLLECTION_PATHS, coll_pair);
	MZ_GC_CHECK();
	MZ_GC_UNREG();
    }
# endif
#endif

# if MZSCHEME_VERSION_MAJOR >= 600
    {
	Scheme_Object *config_path = NULL;
	int mustfree = FALSE;
	char_u *s;

	MZ_GC_DECL_REG(1);
	MZ_GC_VAR_IN_REG(0, config_path);
	MZ_GC_REG();
	// workaround for dynamic loading on windows
	s = vim_getenv((char_u *)"PLTCONFIGDIR", &mustfree);
	if (s != NULL)
	{
	    config_path = scheme_make_path((char *)s);
	    MZ_GC_CHECK();
	    if (mustfree)
		vim_free(s);
	}
#ifdef MZSCHEME_CONFIGDIR
	if (config_path == NULL)
	{
	    config_path = scheme_make_path(MZSCHEME_CONFIGDIR);
	    MZ_GC_CHECK();
	}
#endif
	if (config_path != NULL)
	{
	    scheme_set_config_path(config_path);
	    MZ_GC_CHECK();
	}
	MZ_GC_UNREG();
    }
# endif

#if MZSCHEME_VERSION_MAJOR >= 400
    scheme_init_collection_paths(environment, scheme_null);
#endif

    /*
     * versions 4.x do not provide Scheme bindings by default
     * we need to add them explicitly
     */
    {
	// use error handler to avoid abort
	scheme_dynamic_wind(NULL, load_base_module, NULL,
				    load_base_module_on_error, "racket/base");
	if (load_base_module_failed)
	{
	    load_base_module_failed = FALSE;
	    scheme_dynamic_wind(NULL, load_base_module, NULL,
				    load_base_module_on_error, "scheme/base");
	    if (load_base_module_failed)
		return -1;
	}
    }

    register_vim_exn();
    // use new environment to initialise exception handling
    init_exn_catching_apply();

    // redirect output
    scheme_console_output = do_output;
    scheme_console_printf = do_printf;

#ifdef HAVE_SANDBOX
    {
	Scheme_Object	*make_security_guard = NULL;
	MZ_GC_DECL_REG(1);
	MZ_GC_VAR_IN_REG(0, make_security_guard);
	MZ_GC_REG();

#if MZSCHEME_VERSION_MAJOR < 400
	{
	    Scheme_Object	*make_security_guard_symbol = NULL;
	    MZ_GC_DECL_REG(1);
	    MZ_GC_VAR_IN_REG(0, make_security_guard_symbol);
	    MZ_GC_REG();
	    make_security_guard_symbol = scheme_intern_symbol("make-security-guard");
	    MZ_GC_CHECK();
	    make_security_guard = scheme_lookup_global(
		    make_security_guard_symbol, environment);
	    MZ_GC_UNREG();
	}
#else
	make_security_guard = scheme_builtin_value("make-security-guard");
	MZ_GC_CHECK();
#endif

	// setup sandbox guards
	if (make_security_guard != NULL)
	{
	    Scheme_Object   *args[3] = {NULL, NULL, NULL};
	    Scheme_Object   *guard = NULL;
	    Scheme_Config   *config = NULL;
	    MZ_GC_DECL_REG(5);
	    MZ_GC_ARRAY_VAR_IN_REG(0, args, 3);
	    MZ_GC_VAR_IN_REG(3, guard);
	    MZ_GC_VAR_IN_REG(4, config);
	    MZ_GC_REG();
	    config = scheme_current_config();
	    MZ_GC_CHECK();
	    args[0] = scheme_get_param(config, MZCONFIG_SECURITY_GUARD);
	    MZ_GC_CHECK();
	    args[1] = scheme_make_prim_w_arity(sandbox_file_guard,
		    "sandbox-file-guard", 3, 3);
	    args[2] = scheme_make_prim_w_arity(sandbox_network_guard,
		    "sandbox-network-guard", 4, 4);
	    guard = scheme_apply(make_security_guard, 3, args);
	    MZ_GC_CHECK();
	    scheme_set_param(config, MZCONFIG_SECURITY_GUARD, guard);
	    MZ_GC_CHECK();
	    MZ_GC_UNREG();
	}
	MZ_GC_UNREG();
    }
#endif
    // Create buffer and window types for use in Scheme code
    mz_buffer_type = scheme_make_type("<vim-buffer>");
    MZ_GC_CHECK();
    mz_window_type = scheme_make_type("<vim-window>");
    MZ_GC_CHECK();
#ifdef MZ_PRECISE_GC
    GC_register_traversers(mz_buffer_type,
	    buffer_size_proc, buffer_mark_proc, buffer_fixup_proc,
	    TRUE, TRUE);
    GC_register_traversers(mz_window_type,
	    window_size_proc, window_mark_proc, window_fixup_proc,
	    TRUE, TRUE);
#endif

    make_modules();

    /*
     * setup callback to receive notifications
     * whether thread scheduling is (or not) required
     */
    scheme_notify_multithread = notify_multithread;

    return 0;
}

/*
 * This routine is called for each new invocation of MzScheme
 * to make sure things are properly initialized.
 */
    static int
mzscheme_init(void)
{
    if (!initialized)
    {
#ifdef DYNAMIC_MZSCHEME
	if (disabled || !mzscheme_enabled(TRUE))
	{
	    emsg(_(e_sorry_this_command_is_disabled_the_mzscheme_libraries_could_not_be_loaded));
	    return -1;
	}
#endif
	if (load_base_module_failed || startup_mzscheme())
	{
	    emsg(_(e_sorry_this_command_is_disabled_the_mzscheme_racket_base_module_could_not_be_loaded));
	    return -1;
	}
	initialized = TRUE;
    }
    {
	Scheme_Config	*config = NULL;
	MZ_GC_DECL_REG(1);
	MZ_GC_VAR_IN_REG(0, config);
	MZ_GC_REG();
	config = scheme_current_config();
	MZ_GC_CHECK();
	// recreate ports each call effectively clearing these ones
	curout = scheme_make_byte_string_output_port();
	MZ_GC_CHECK();
	curerr = scheme_make_byte_string_output_port();
	MZ_GC_CHECK();
	scheme_set_param(config, MZCONFIG_OUTPUT_PORT, curout);
	MZ_GC_CHECK();
	scheme_set_param(config, MZCONFIG_ERROR_PORT, curerr);
	MZ_GC_CHECK();
	MZ_GC_UNREG();
    }

    return 0;
}

/*
 *========================================================================
 *  2.  External Interface
 *========================================================================
 */

/*
 * Evaluate command with exception handling
 */
    static int
eval_with_exn_handling(void *data, Scheme_Closed_Prim *what, Scheme_Object **ret)
{
    Scheme_Object   *value = NULL;
    Scheme_Object   *exn = NULL;
    Scheme_Object   *prim = NULL;

    MZ_GC_DECL_REG(3);
    MZ_GC_VAR_IN_REG(0, value);
    MZ_GC_VAR_IN_REG(1, exn);
    MZ_GC_VAR_IN_REG(2, prim);
    MZ_GC_REG();

    prim = scheme_make_closed_prim_w_arity(what, data, "mzvim", 0, 0);
    MZ_GC_CHECK();
    value = _apply_thunk_catch_exceptions(prim, &exn);
    MZ_GC_CHECK();

    if (!value)
    {
	value = extract_exn_message(exn);
	// Got an exn?
	if (value)
	{
	    scheme_display(value, curerr);   //  Send to stderr-vim
	    MZ_GC_CHECK();
	    do_flush();
	}
	MZ_GC_UNREG();
	// `raise' was called on some arbitrary value
	return FAIL;
    }

    if (ret != NULL)	// if pointer to retval supported give it up
	*ret = value;
    // Print any result, as long as it's not a void
    else if (!SCHEME_VOIDP(value))
    {
	scheme_display(value, curout);  // Send to stdout-vim
	MZ_GC_CHECK();
    }

    do_flush();
    MZ_GC_UNREG();
    return OK;
}

/*
 * :mzscheme
 */
    static int
do_mzscheme_command(exarg_T *eap, void *data, Scheme_Closed_Prim *what)
{
    if (mzscheme_init())
	return FAIL;

    range_start = eap->line1;
    range_end = eap->line2;

    return eval_with_exn_handling(data, what, NULL);
}

/*
 * Routine called by VIM when deleting a buffer
 */
    void
mzscheme_buffer_free(buf_T *buf)
{
    if (buf->b_mzscheme_ref == NULL)
	return;

    vim_mz_buffer *bp = NULL;
    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, bp);
    MZ_GC_REG();

    bp = BUFFER_REF(buf);
    bp->buf = INVALID_BUFFER_VALUE;
#ifdef MZ_PRECISE_GC
    scheme_free_immobile_box(buf->b_mzscheme_ref);
#else
    scheme_gc_ptr_ok(bp);
#endif
    buf->b_mzscheme_ref = NULL;
    MZ_GC_CHECK();
    MZ_GC_UNREG();
}

/*
 * Routine called by VIM when deleting a Window
 */
    void
mzscheme_window_free(win_T *win)
{
    if (win->w_mzscheme_ref == NULL)
	return;

    vim_mz_window *wp = NULL;
    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, wp);
    MZ_GC_REG();
    wp = WINDOW_REF(win);
    wp->win = INVALID_WINDOW_VALUE;
#ifdef MZ_PRECISE_GC
    scheme_free_immobile_box(win->w_mzscheme_ref);
#else
    scheme_gc_ptr_ok(wp);
#endif
    win->w_mzscheme_ref = NULL;
    MZ_GC_CHECK();
    MZ_GC_UNREG();
}

/*
 * ":mzscheme" (or ":mz")
 */
    void
ex_mzscheme(exarg_T *eap)
{
    char_u	*script;

    script = script_get(eap, eap->arg);
    if (eap->skip)
	return;

    if (script == NULL)
	do_mzscheme_command(eap, eap->arg, do_eval);
    else
    {
	do_mzscheme_command(eap, script, do_eval);
	vim_free(script);
    }
}

    static Scheme_Object *
do_load(void *data, int noargc UNUSED, Scheme_Object **noargv UNUSED)
{
    Scheme_Object   *expr = NULL;
    Scheme_Object   *result = NULL;
    char	    *file = NULL;
    Port_Info	    *pinfo = (Port_Info *)data;

    MZ_GC_DECL_REG(3);
    MZ_GC_VAR_IN_REG(0, expr);
    MZ_GC_VAR_IN_REG(1, result);
    MZ_GC_VAR_IN_REG(2, file);
    MZ_GC_REG();

    file = (char *)scheme_malloc_fail_ok(scheme_malloc_atomic, MAXPATHL + 1);
    MZ_GC_CHECK();

    // make Vim expansion
    expand_env((char_u *)pinfo->name, (char_u *)file, MAXPATHL);
    pinfo->port = scheme_open_input_file(file, "mzfile");
    MZ_GC_CHECK();
    scheme_count_lines(pinfo->port);  // to get accurate read error location
    MZ_GC_CHECK();

    // Like REPL but print only last result
    while (!SCHEME_EOFP(expr = scheme_read(pinfo->port)))
    {
	result = scheme_eval(expr, environment);
	MZ_GC_CHECK();
    }

    // errors will be caught in do_mzscheme_command and ex_mzfile
    scheme_close_input_port(pinfo->port);
    MZ_GC_CHECK();
    pinfo->port = NULL;
    MZ_GC_UNREG();
    return result;
}

/*
 * :mzfile
 */
    void
ex_mzfile(exarg_T *eap)
{
    Port_Info	pinfo = {NULL, NULL};

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, pinfo.port);
    MZ_GC_REG();

    pinfo.name = (char *)eap->arg;
    if (do_mzscheme_command(eap, &pinfo, do_load) != OK
	    && pinfo.port != NULL)	// looks like port was not closed
    {
	scheme_close_input_port(pinfo.port);
	MZ_GC_CHECK();
    }
    MZ_GC_UNREG();
}


/*
 *========================================================================
 * Exception handling code -- cribbed form the MzScheme sources and
 * Matthew Flatt's "Inside PLT MzScheme" document.
 *========================================================================
 */
    static void
init_exn_catching_apply(void)
{
    if (exn_catching_apply)
	return;

    char *e =
	"(lambda (thunk) "
	"(with-handlers ([void (lambda (exn) (cons #f exn))]) "
	"(cons #t (thunk))))";

    exn_catching_apply = scheme_eval_string(e, environment);
    MZ_GC_CHECK();
    exn_p = scheme_builtin_value("exn?");
    MZ_GC_CHECK();
    exn_message = scheme_builtin_value("exn-message");
    MZ_GC_CHECK();
}

/*
 * This function applies a thunk, returning the Scheme value if there's
 * no exception, otherwise returning NULL and setting *exn to the raised
 * value (usually an exn structure).
 */
    static Scheme_Object *
_apply_thunk_catch_exceptions(Scheme_Object *f, Scheme_Object **exn)
{
    Scheme_Object *v;

    v = _scheme_apply(exn_catching_apply, 1, &f);
    // v is a pair: (cons #t value) or (cons #f exn)

    if (SCHEME_TRUEP(SCHEME_CAR(v)))
	return SCHEME_CDR(v);
    else
    {
	*exn = SCHEME_CDR(v);
	return NULL;
    }
}

    static Scheme_Object *
extract_exn_message(Scheme_Object *v)
{
    if (SCHEME_TRUEP(_scheme_apply(exn_p, 1, &v)))
	return _scheme_apply(exn_message, 1, &v);
    else
	return NULL; // Not an exn structure
}

    static Scheme_Object *
do_eval(void *s, int noargc UNUSED, Scheme_Object **noargv UNUSED)
{
    return scheme_eval_string_all((char *)s, environment, TRUE);
}

/*
 *========================================================================
 *  3.  MzScheme I/O Handlers
 *========================================================================
 */
    static void
do_intrnl_output(char *mesg, int error)
{
    char *p, *prev;

    prev = mesg;
    p = strchr(prev, '\n');
    while (p)
    {
	*p = '\0';
	if (error)
	    emsg(prev);
	else
	    msg(prev);
	prev = p + 1;
	p = strchr(prev, '\n');
    }

    if (error)
	emsg(prev);
    else
	msg(prev);
}

    static void
do_output(char *mesg, OUTPUT_LEN_TYPE len UNUSED)
{
    // TODO: use len, the string may not be NUL terminated
    do_intrnl_output(mesg, 0);
}

    static void
do_err_output(char *mesg)
{
    do_intrnl_output(mesg, 1);
}

    static void
do_printf(char *format, ...)
{
    do_intrnl_output(format, 1);
}

    static void
do_flush(void)
{
    char *buff;
    OUTPUT_LEN_TYPE length;

    buff = scheme_get_sized_byte_string_output(curerr, &length);
    MZ_GC_CHECK();
    if (length)
    {
	do_err_output(buff);
	return;
    }

    buff = scheme_get_sized_byte_string_output(curout, &length);
    MZ_GC_CHECK();
    if (length)
	do_output(buff, length);
}

/*
 *========================================================================
 *  4. Implementation of the Vim Features for MzScheme
 *========================================================================
 */

/*
 * (command {command-string})
 */
    static Scheme_Object *
vim_command(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	*prim = (Vim_Prim *)data;
    Scheme_Object *cmd = NULL;
    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, cmd);
    MZ_GC_REG();
    cmd = GUARANTEED_STRING_ARG(prim->name, 0);

    // may be use do_cmdline_cmd?
    do_cmdline(BYTE_STRING_VALUE(cmd), NULL, NULL, DOCMD_NOWAIT|DOCMD_VERBOSE);
    update_screen(UPD_VALID);

    MZ_GC_UNREG();
    raise_if_error();
    return scheme_void;
}

/*
 * (eval {expr-string})
 */
    static Scheme_Object *
vim_eval(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
#ifdef FEAT_EVAL
    Vim_Prim		*prim = (Vim_Prim *)data;
    Scheme_Object	*result = NULL;
    typval_T		*vim_result;
    Scheme_Object *expr = NULL;
    MZ_GC_DECL_REG(2);
    MZ_GC_VAR_IN_REG(0, result);
    MZ_GC_VAR_IN_REG(1, expr);
    MZ_GC_REG();
    expr = GUARANTEED_STRING_ARG(prim->name, 0);

    vim_result = eval_expr(BYTE_STRING_VALUE(expr), NULL);

    if (vim_result == NULL)
	raise_vim_exn(_("invalid expression"));

    result = vim_to_mzscheme(vim_result);
    MZ_GC_CHECK();
    free_tv(vim_result);

    MZ_GC_UNREG();
    return result;
#else
    raise_vim_exn(_("expressions disabled at compile time"));
    // unreachable
    return scheme_false;
#endif
}

/*
 * (range-start)
 */
    static Scheme_Object *
get_range_start(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
    return scheme_make_integer(range_start);
}

/*
 * (range-end)
 */
    static Scheme_Object *
get_range_end(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
    return scheme_make_integer(range_end);
}

/*
 * (beep)
 */
    static Scheme_Object *
mzscheme_beep(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
    vim_beep(BO_LANG);
    return scheme_void;
}

static Scheme_Object *M_global = NULL;

/*
 * (get-option {option-name}) [buffer/window]
 */
    static Scheme_Object *
get_option(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    long	    value;
    char	    *strval;
    getoption_T	    rc;
    Scheme_Object   *rval = NULL;
    Scheme_Object   *name = NULL;
    int		    scope = 0;
    buf_T	    *save_curb = curbuf;
    win_T	    *save_curw = curwin;

    MZ_GC_DECL_REG(2);
    MZ_GC_VAR_IN_REG(0, rval);
    MZ_GC_VAR_IN_REG(1, name);
    MZ_GC_REG();

    name = GUARANTEED_STRING_ARG(prim->name, 0);

    if (argc > 1)
    {
	if (M_global == NULL)
	{
	    MZ_REGISTER_STATIC(M_global);
	    M_global = scheme_intern_symbol("global");
	    MZ_GC_CHECK();
	}

	if (argv[1] == M_global)
	    scope = OPT_GLOBAL;
	else if (SCHEME_VIMBUFFERP(argv[1]))
	{
	    curbuf = get_valid_buffer(argv[1]);
	    scope = OPT_LOCAL;
	}
	else if (SCHEME_VIMWINDOWP(argv[1]))
	{
	    win_T *win = get_valid_window(argv[1]);

	    curwin = win;
	    curbuf = win->w_buffer;
	    scope = OPT_LOCAL;
	}
	else
	    scheme_wrong_type(prim->name, "vim-buffer/window", 1, argc, argv);
    }

    rc = get_option_value(BYTE_STRING_VALUE(name), &value, (char_u **)&strval,
								  NULL, scope);
    curbuf = save_curb;
    curwin = save_curw;

    switch (rc)
    {
    case gov_bool:
    case gov_number:
	MZ_GC_UNREG();
	return scheme_make_integer_value(value);
    case gov_string:
	rval = scheme_make_byte_string(strval);
	MZ_GC_CHECK();
	vim_free(strval);
	MZ_GC_UNREG();
	return rval;
    case gov_hidden_bool:
    case gov_hidden_number:
    case gov_hidden_string:
	MZ_GC_UNREG();
	raise_vim_exn(_("hidden option"));
	//NOTREACHED
    case gov_unknown:
	MZ_GC_UNREG();
	raise_vim_exn(_("unknown option"));
	//NOTREACHED
    }
    // unreachable
    return scheme_void;
}

/*
 * (set-option {option-changing-string} [buffer/window])
 */
    static Scheme_Object *
set_option(void *data, int argc, Scheme_Object **argv)
{
    char_u	*command = NULL;
    int		scope = 0;
    buf_T	*save_curb = curbuf;
    win_T	*save_curw = curwin;
    Vim_Prim	*prim = (Vim_Prim *)data;
    Scheme_Object *cmd = NULL;

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, cmd);
    MZ_GC_REG();
    cmd = GUARANTEED_STRING_ARG(prim->name, 0);

    if (argc > 1)
    {
	if (M_global == NULL)
	{
	    MZ_REGISTER_STATIC(M_global);
	    M_global = scheme_intern_symbol("global");
	    MZ_GC_CHECK();
	}

	if (argv[1] == M_global)
	    scope = OPT_GLOBAL;
	else if (SCHEME_VIMBUFFERP(argv[1]))
	{
	    curbuf = get_valid_buffer(argv[1]);
	    scope = OPT_LOCAL;
	}
	else if (SCHEME_VIMWINDOWP(argv[1]))
	{
	    win_T *win = get_valid_window(argv[1]);
	    curwin = win;
	    curbuf = win->w_buffer;
	    scope = OPT_LOCAL;
	}
	else
	    scheme_wrong_type(prim->name, "vim-buffer/window", 1, argc, argv);
    }

    // do_set can modify cmd, make copy
    command = vim_strsave(BYTE_STRING_VALUE(cmd));
    MZ_GC_UNREG();
    do_set(command, scope);
    vim_free(command);
    update_screen(UPD_NOT_VALID);
    curbuf = save_curb;
    curwin = save_curw;
    raise_if_error();
    return scheme_void;
}

/*
 *===========================================================================
 *  5. Vim Window-related Manipulation Functions
 *===========================================================================
 */

/*
 * (curr-win)
 */
    static Scheme_Object *
get_curr_win(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
    return (Scheme_Object *)get_vim_curr_window();
}

/*
 * (win-count)
 */
    static Scheme_Object *
get_window_count(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
    int	    n = 0;
    win_T   *w;

    FOR_ALL_WINDOWS(w)
	++n;
    return scheme_make_integer(n);
}

/*
 * (get-win-list [buffer])
 */
    static Scheme_Object *
get_window_list(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf;
    Scheme_Object   *list;
    win_T	    *w = firstwin;

    buf = get_buffer_arg(prim->name, 0, argc, argv);
    list = scheme_null;

    for ( ; w != NULL; w = w->w_next)
	if (w->w_buffer == buf->buf)
	{
	    list = scheme_make_pair(window_new(w), list);
	    MZ_GC_CHECK();
	}

    return list;
}

    static Scheme_Object *
window_new(win_T *win)
{
    vim_mz_window *self = NULL;

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, self);

    // We need to handle deletion of windows underneath us.
    // If we add a "w_mzscheme_ref" field to the win_T structure,
    // then we can get at it in win_free() in vim.
    //
    // On a win_free() we set the Scheme object's win_T *field
    // to an invalid value. We trap all uses of a window
    // object, and reject them if the win_T *field is invalid.
    if (win->w_mzscheme_ref != NULL)
	return (Scheme_Object *)WINDOW_REF(win);

    MZ_GC_REG();
    self = scheme_malloc_fail_ok(scheme_malloc_tagged, sizeof(vim_mz_window));
    CLEAR_POINTER(self);
#ifdef MZ_PRECISE_GC
    win->w_mzscheme_ref = scheme_malloc_immobile_box(NULL);
#else
    scheme_dont_gc_ptr(self);	// because win isn't visible to GC
#endif
    MZ_GC_CHECK();
    WINDOW_REF(win) = self;
    MZ_GC_CHECK();
    self->win = win;
    self->so.type = mz_window_type;

    MZ_GC_UNREG();
    return (Scheme_Object *)self;
}

/*
 * (get-win-num [window])
 */
    static Scheme_Object *
get_window_num(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
    int		nr = 1;
    Vim_Prim	*prim = (Vim_Prim *)data;
    win_T	*win = get_window_arg(prim->name, 0, argc, argv)->win;
    win_T	*wp;

    for (wp = firstwin; wp != win; wp = wp->w_next)
	++nr;

    return scheme_make_integer(nr);
}

/*
 * (get-win-by-num {windownum})
 */
    static Scheme_Object *
get_window_by_num(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	*prim = (Vim_Prim *)data;
    win_T	*win = firstwin;
    int		fnum;

    fnum = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));
    if (fnum < 1)
	scheme_signal_error(_("window index is out of range"));

    for ( ; win != NULL; win = win->w_next, --fnum)
	if (fnum == 1)	    // to be 1-based
	    return window_new(win);

    return scheme_false;
}

/*
 * (get-win-buffer [window])
 */
    static Scheme_Object *
get_window_buffer(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_window   *win = get_window_arg(prim->name, 0, argc, argv);

    return buffer_new(win->win->w_buffer);
}

/*
 * (get-win-height [window])
 */
    static Scheme_Object *
get_window_height(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_window   *win = get_window_arg(prim->name, 0, argc, argv);

    return scheme_make_integer(win->win->w_height);
}

/*
 * (set-win-height {height} [window])
 */
    static Scheme_Object *
set_window_height(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_window   *win;
    win_T	    *savewin;
    int		    height;

    win = get_window_arg(prim->name, 1, argc, argv);
    height = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));

#ifdef FEAT_GUI
    need_mouse_correct = TRUE;
#endif

    savewin = curwin;
    curwin = win->win;
    win_setheight(height);
    curwin = savewin;

    raise_if_error();
    return scheme_void;
}

/*
 * (get-win-width [window])
 */
    static Scheme_Object *
get_window_width(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_window   *win = get_window_arg(prim->name, 0, argc, argv);

    return scheme_make_integer(win->win->w_width);
}

/*
 * (set-win-width {width} [window])
 */
    static Scheme_Object *
set_window_width(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_window   *win;
    win_T	    *savewin;
    int		    width = 0;

    win = get_window_arg(prim->name, 1, argc, argv);
    width = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));

# ifdef FEAT_GUI
    need_mouse_correct = TRUE;
# endif

    savewin = curwin;
    curwin = win->win;
    win_setwidth(width);
    curwin = savewin;

    raise_if_error();
    return scheme_void;
}

/*
 * (get-cursor [window]) -> (line . col)
 */
    static Scheme_Object *
get_cursor(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_window   *win;
    pos_T	    pos;

    win = get_window_arg(prim->name, 0, argc, argv);
    pos = win->win->w_cursor;
    return scheme_make_pair(scheme_make_integer_value((long)pos.lnum),
		    scheme_make_integer_value((long)pos.col + 1));
}

/*
 * (set-cursor (line . col) [window])
 */
    static Scheme_Object *
set_cursor(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_window   *win;
    long	    lnum = 0;
    long	    col = 0;

#ifdef HAVE_SANDBOX
    sandbox_check();
#endif
    win = get_window_arg(prim->name, 1, argc, argv);
    GUARANTEE_PAIR(prim->name, 0);

    if (!SCHEME_INTP(SCHEME_CAR(argv[0]))
	    || !SCHEME_INTP(SCHEME_CDR(argv[0])))
	scheme_wrong_type(prim->name, "integer pair", 0, argc, argv);

    lnum = SCHEME_INT_VAL(SCHEME_CAR(argv[0]));
    col = SCHEME_INT_VAL(SCHEME_CDR(argv[0])) - 1;

    check_line_range(lnum, win->win->w_buffer);
    // don't know how to catch invalid column value

    win->win->w_cursor.lnum = lnum;
    win->win->w_cursor.col = col;
    win->win->w_set_curswant = TRUE;
    update_screen(UPD_VALID);

    raise_if_error();
    return scheme_void;
}
/*
 *===========================================================================
 *  6. Vim Buffer-related Manipulation Functions
 *===========================================================================
 */

/*
 * (open-buff {filename})
 */
    static Scheme_Object *
mzscheme_open_buffer(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    int		    num = 0;
    Scheme_Object   *onum = NULL;
    Scheme_Object   *buf = NULL;
    Scheme_Object   *fname;

    MZ_GC_DECL_REG(3);
    MZ_GC_VAR_IN_REG(0, onum);
    MZ_GC_VAR_IN_REG(1, buf);
    MZ_GC_VAR_IN_REG(2, fname);
    MZ_GC_REG();
    fname = GUARANTEED_STRING_ARG(prim->name, 0);

#ifdef HAVE_SANDBOX
    sandbox_check();
#endif
    // TODO make open existing file
    num = buflist_add(BYTE_STRING_VALUE(fname), BLN_LISTED | BLN_CURBUF);

    if (num == 0)
	raise_vim_exn(_("couldn't open buffer"));

    onum = scheme_make_integer(num);
    buf = get_buffer_by_num(data, 1, &onum);
    MZ_GC_UNREG();
    return buf;
}

/*
 * (get-buff-by-num {buffernum})
 */
    static Scheme_Object *
get_buffer_by_num(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	*prim = (Vim_Prim *)data;
    buf_T	*buf;
    int		fnum;

    fnum = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));

    FOR_ALL_BUFFERS(buf)
	if (buf->b_fnum == fnum)
	    return buffer_new(buf);

    return scheme_false;
}

/*
 * (get-buff-by-name {buffername})
 */
    static Scheme_Object *
get_buffer_by_name(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	*prim = (Vim_Prim *)data;
    buf_T	*buf;
    Scheme_Object *buffer = NULL;
    Scheme_Object *fname = NULL;

    MZ_GC_DECL_REG(2);
    MZ_GC_VAR_IN_REG(0, buffer);
    MZ_GC_VAR_IN_REG(1, fname);
    MZ_GC_REG();
    fname = GUARANTEED_STRING_ARG(prim->name, 0);
    buffer = scheme_false;

    FOR_ALL_BUFFERS(buf)
    {
	if (buf->b_ffname == NULL || buf->b_sfname == NULL)
	    // empty string
	{
	    if (BYTE_STRING_VALUE(fname)[0] == NUL)
		buffer = buffer_new(buf);
	}
	else if (!fnamecmp(buf->b_ffname, BYTE_STRING_VALUE(fname))
		|| !fnamecmp(buf->b_sfname, BYTE_STRING_VALUE(fname)))
	{
	    // either short or long filename matches
	    buffer = buffer_new(buf);
	}
    }

    MZ_GC_UNREG();
    return buffer;
}

/*
 * (get-next-buff [buffer])
 */
    static Scheme_Object *
get_next_buffer(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	*prim = (Vim_Prim *)data;
    buf_T	*buf = get_buffer_arg(prim->name, 0, argc, argv)->buf;

    if (buf->b_next == NULL)
	return scheme_false;
    else
	return buffer_new(buf->b_next);
}

/*
 * (get-prev-buff [buffer])
 */
    static Scheme_Object *
get_prev_buffer(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	*prim = (Vim_Prim *)data;
    buf_T	*buf = get_buffer_arg(prim->name, 0, argc, argv)->buf;

    if (buf->b_prev == NULL)
	return scheme_false;
    else
	return buffer_new(buf->b_prev);
}

/*
 * (get-buff-num [buffer])
 */
    static Scheme_Object *
get_buffer_num(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf = get_buffer_arg(prim->name, 0, argc, argv);

    return scheme_make_integer(buf->buf->b_fnum);
}

/*
 * (buff-count)
 */
    static Scheme_Object *
get_buffer_count(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
    buf_T   *b;
    int	    n = 0;

    FOR_ALL_BUFFERS(b) ++n;
    return scheme_make_integer(n);
}

/*
 * (get-buff-name [buffer])
 */
    static Scheme_Object *
get_buffer_name(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf = get_buffer_arg(prim->name, 0, argc, argv);

    return scheme_make_byte_string((char *)buf->buf->b_ffname);
}

/*
 * (curr-buff)
 */
    static Scheme_Object *
get_curr_buffer(void *data UNUSED, int argc UNUSED, Scheme_Object **argv UNUSED)
{
    return (Scheme_Object *)get_vim_curr_buffer();
}

    static Scheme_Object *
buffer_new(buf_T *buf)
{
    vim_mz_buffer *self = NULL;

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, self);

    // We need to handle deletion of buffers underneath us.
    // If we add a "b_mzscheme_ref" field to the buf_T structure,
    // then we can get at it in buf_freeall() in vim.
    if (buf->b_mzscheme_ref)
	return (Scheme_Object *)BUFFER_REF(buf);

    MZ_GC_REG();
    self = scheme_malloc_fail_ok(scheme_malloc_tagged, sizeof(vim_mz_buffer));
    CLEAR_POINTER(self);
#ifdef MZ_PRECISE_GC
    buf->b_mzscheme_ref = scheme_malloc_immobile_box(NULL);
#else
    scheme_dont_gc_ptr(self);	// because buf isn't visible to GC
#endif
    MZ_GC_CHECK();
    BUFFER_REF(buf) = self;
    MZ_GC_CHECK();
    self->buf = buf;
    self->so.type = mz_buffer_type;

    MZ_GC_UNREG();
    return (Scheme_Object *)self;
}

/*
 * (get-buff-size [buffer])
 *
 * Get the size (number of lines) in the current buffer.
 */
    static Scheme_Object *
get_buffer_size(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf = get_buffer_arg(prim->name, 0, argc, argv);

    return scheme_make_integer(buf->buf->b_ml.ml_line_count);
}

/*
 * (get-buff-line {linenr} [buffer])
 *
 * Get a line from the specified buffer. The line number is
 * in Vim format (1-based). The line is returned as a MzScheme
 * string object.
 */
    static Scheme_Object *
get_buffer_line(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf;
    int		    linenr;
    char_u	    *line;

    buf = get_buffer_arg(prim->name, 1, argc, argv);
    linenr = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));
    line = ml_get_buf(buf->buf, (linenr_T)linenr, FALSE);

    raise_if_error();
    return scheme_make_byte_string((char *)line);
}


/*
 * (get-buff-line-list {start} {end} [buffer])
 *
 * Get a list of lines from the specified buffer. The line numbers
 * are in Vim format (1-based). The range is from lo up to, but not
 * including, hi. The list is returned as a list of string objects.
 */
    static Scheme_Object *
get_buffer_line_list(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf;
    int		    i, hi, lo, n;
    Scheme_Object   *list = NULL;

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, list);
    MZ_GC_REG();

    buf = get_buffer_arg(prim->name, 2, argc, argv);
    list = scheme_null;
    hi = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 1));
    lo = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));

    /*
     * Handle some error conditions
     */
    if (lo < 0)
	lo = 0;

    if (hi < 0)
	hi = 0;
    if (hi < lo)
	hi = lo;

    n = hi - lo;

    for (i = n; i >= 0; --i)
    {
	Scheme_Object *str = scheme_make_byte_string(
		       (char *)ml_get_buf(buf->buf, (linenr_T)(lo+i), FALSE));
	raise_if_error();

	// Set the list item
	list = scheme_make_pair(str, list);
	MZ_GC_CHECK();
    }
    MZ_GC_UNREG();
    return list;
}

/*
 * (set-buff-line {linenr} {string/#f} [buffer])
 *
 * Replace a line in the specified buffer. The line number is
 * in Vim format (1-based). The replacement line is given as
 * an MzScheme string object. The object is checked for validity
 * and correct format. An exception is thrown if the values are not
 * the correct format.
 *
 * It returns a Scheme Object that indicates the length of the
 * string changed.
 */
    static Scheme_Object *
set_buffer_line(void *data, int argc, Scheme_Object **argv)
{
    // First of all, we check the value of the supplied MzScheme object.
    // There are three cases:
    //	  1. #f - this is a deletion.
    //	  2. A string	   - this is a replacement.
    //	  3. Anything else - this is an error.
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf;
    Scheme_Object   *line = NULL;
    char	    *save;
    int		    n;

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, line);
    MZ_GC_REG();

#ifdef HAVE_SANDBOX
    sandbox_check();
#endif
    n = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));
    if (!SCHEME_STRINGP(argv[1]) && !SCHEME_FALSEP(argv[1]))
	scheme_wrong_type(prim->name, "string or #f", 1, argc, argv);
    line = argv[1];
    buf = get_buffer_arg(prim->name, 2, argc, argv);

    check_line_range(n, buf->buf);

    if (SCHEME_FALSEP(line))
    {
	buf_T	    *savebuf = curbuf;

	curbuf = buf->buf;

	if (u_savedel((linenr_T)n, 1L) == FAIL)
	{
	    curbuf = savebuf;
	    raise_vim_exn(_("cannot save undo information"));
	}
	else if (ml_delete((linenr_T)n) == FAIL)
	{
	    curbuf = savebuf;
	    raise_vim_exn(_("cannot delete line"));
	}
	if (buf->buf == curwin->w_buffer)
	    mz_fix_cursor(n, n + 1, -1);
	deleted_lines_mark((linenr_T)n, 1L);

	curbuf = savebuf;

	MZ_GC_UNREG();
	raise_if_error();
	return scheme_void;
    }
    else
    {
	// Otherwise it's a line
	buf_T	    *savebuf = curbuf;

	save = string_to_line(line);

	curbuf = buf->buf;

	if (u_savesub((linenr_T)n) == FAIL)
	{
	    curbuf = savebuf;
	    vim_free(save);
	    raise_vim_exn(_("cannot save undo information"));
	}
	else if (ml_replace((linenr_T)n, (char_u *)save, TRUE) == FAIL)
	{
	    curbuf = savebuf;
	    vim_free(save);
	    raise_vim_exn(_("cannot replace line"));
	}
	else
	{
	    vim_free(save);
	    changed_bytes((linenr_T)n, 0);
	}

	curbuf = savebuf;

	// Check that the cursor is not beyond the end of the line now.
	if (buf->buf == curwin->w_buffer)
	    check_cursor_col();

	MZ_GC_UNREG();
	raise_if_error();
	return scheme_void;
    }
}

    static void
free_array(char **array)
{
    char **curr = array;
    while (*curr != NULL)
	vim_free(*curr++);
    vim_free(array);
}

/*
 * (set-buff-line-list {start} {end} {string-list/#f/null} [buffer])
 *
 * Replace a range of lines in the specified buffer. The line numbers are in
 * Vim format (1-based). The range is from lo up to, but not including, hi.
 * The replacement lines are given as a Scheme list of string objects. The
 * list is checked for validity and correct format.
 *
 * Errors are returned as a value of FAIL. The return value is OK on success.
 * If OK is returned and len_change is not NULL, *len_change is set to the
 * change in the buffer length.
 */
    static Scheme_Object *
set_buffer_line_list(void *data, int argc, Scheme_Object **argv)
{
    // First of all, we check the type of the supplied MzScheme object.
    // There are three cases:
    //	  1. #f - this is a deletion.
    //	  2. A list	   - this is a replacement.
    //	  3. Anything else - this is an error.
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf = NULL;
    Scheme_Object   *line_list = NULL;
    int		    i, old_len, new_len, hi, lo;
    long	    extra;

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, line_list);
    MZ_GC_REG();

#ifdef HAVE_SANDBOX
    sandbox_check();
#endif
    lo = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));
    hi = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 1));
    if (!SCHEME_PAIRP(argv[2])
	    && !SCHEME_FALSEP(argv[2]) && !SCHEME_NULLP(argv[2]))
	scheme_wrong_type(prim->name, "list or #f", 2, argc, argv);
    line_list = argv[2];
    buf = get_buffer_arg(prim->name, 3, argc, argv);
    old_len = hi - lo;
    if (old_len < 0) // process inverse values wisely
    {
	i = lo;
	lo = hi;
	hi = i;
	old_len = -old_len;
    }
    extra = 0;

    check_line_range(lo, buf->buf);	    // inclusive
    check_line_range(hi - 1, buf->buf);	    // exclusive

    if (SCHEME_FALSEP(line_list) || SCHEME_NULLP(line_list))
    {
	buf_T	*savebuf = curbuf;
	curbuf = buf->buf;

	if (u_savedel((linenr_T)lo, (long)old_len) == FAIL)
	{
	    curbuf = savebuf;
	    raise_vim_exn(_("cannot save undo information"));
	}
	else
	{
	    for (i = 0; i < old_len; i++)
		if (ml_delete((linenr_T)lo) == FAIL)
		{
		    curbuf = savebuf;
		    raise_vim_exn(_("cannot delete line"));
		}
	    if (buf->buf == curwin->w_buffer)
		mz_fix_cursor(lo, hi, -old_len);
	    deleted_lines_mark((linenr_T)lo, (long)old_len);
	}

	curbuf = savebuf;

	MZ_GC_UNREG();
	raise_if_error();
	return scheme_void;
    }
    else
    {
	buf_T	*savebuf = curbuf;

	// List
	new_len = scheme_proper_list_length(line_list);
	MZ_GC_CHECK();
	if (new_len < 0)	// improper or cyclic list
	    scheme_wrong_type(prim->name, "proper list",
		    2, argc, argv);
	else
	{
	    char		**array = NULL;
	    Scheme_Object   *line = NULL;
	    Scheme_Object   *rest = NULL;

	    MZ_GC_DECL_REG(2);
	    MZ_GC_VAR_IN_REG(0, line);
	    MZ_GC_VAR_IN_REG(1, rest);
	    MZ_GC_REG();

	    array = ALLOC_CLEAR_MULT(char *, new_len + 1);

	    rest = line_list;
	    for (i = 0; i < new_len; ++i)
	    {
		line = SCHEME_CAR(rest);
		rest = SCHEME_CDR(rest);
		if (!SCHEME_STRINGP(line))
		{
		    free_array(array);
		    scheme_wrong_type(prim->name, "string-list", 2, argc, argv);
		}
		array[i] = string_to_line(line);
	    }

	    curbuf = buf->buf;

	    if (u_save((linenr_T)(lo-1), (linenr_T)hi) == FAIL)
	    {
		curbuf = savebuf;
		free_array(array);
		raise_vim_exn(_("cannot save undo information"));
	    }

	    /*
	     * If the size of the range is reducing (ie, new_len < old_len) we
	     * need to delete some old_len. We do this at the start, by
	     * repeatedly deleting line "lo".
	     */
	    for (i = 0; i < old_len - new_len; ++i)
	    {
		if (ml_delete((linenr_T)lo) == FAIL)
		{
		    curbuf = savebuf;
		    free_array(array);
		    raise_vim_exn(_("cannot delete line"));
		}
		extra--;
	    }

	    /*
	     * For as long as possible, replace the existing old_len with the
	     * new old_len. This is a more efficient operation, as it requires
	     * less memory allocation and freeing.
	     */
	    for (i = 0; i < old_len && i < new_len; i++)
		if (ml_replace((linenr_T)(lo+i), (char_u *)array[i], TRUE) == FAIL)
		{
		    curbuf = savebuf;
		    free_array(array);
		    raise_vim_exn(_("cannot replace line"));
		}

	    /*
	     * Now we may need to insert the remaining new_len.  We don't need to
	     * free the string passed back because MzScheme has control of that
	     * memory.
	     */
	    while (i < new_len)
	    {
		if (ml_append((linenr_T)(lo + i - 1),
			    (char_u *)array[i], 0, FALSE) == FAIL)
		{
		    curbuf = savebuf;
		    free_array(array);
		    raise_vim_exn(_("cannot insert line"));
		}
		++i;
		++extra;
	    }
	    MZ_GC_UNREG();
	    free_array(array);
	}

	/*
	 * Adjust marks. Invalidate any which lie in the
	 * changed range, and move any in the remainder of the buffer.
	 */
	mark_adjust((linenr_T)lo, (linenr_T)(hi - 1),
						  (long)MAXLNUM, (long)extra);
	changed_lines((linenr_T)lo, 0, (linenr_T)hi, (long)extra);

	if (buf->buf == curwin->w_buffer)
	    mz_fix_cursor(lo, hi, extra);
	curbuf = savebuf;

	MZ_GC_UNREG();
	raise_if_error();
	return scheme_void;
    }
}

/*
 * (insert-buff-line-list {linenr} {string/string-list} [buffer])
 *
 * Insert a number of lines into the specified buffer after the specified line.
 * The line number is in Vim format (1-based). The lines to be inserted are
 * given as an MzScheme list of string objects or as a single string. The lines
 * to be added are checked for validity and correct format. Errors are
 * returned as a value of FAIL.  The return value is OK on success.
 * If OK is returned and len_change is not NULL, *len_change
 * is set to the change in the buffer length.
 */
    static Scheme_Object *
insert_buffer_line_list(void *data, int argc, Scheme_Object **argv)
{
    Vim_Prim	    *prim = (Vim_Prim *)data;
    vim_mz_buffer   *buf = NULL;
    Scheme_Object   *list = NULL;
    char	    *str = NULL;
    int		    i, n, size;

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, list);
    MZ_GC_REG();

#ifdef HAVE_SANDBOX
    sandbox_check();
#endif
    /*
     * First of all, we check the type of the supplied MzScheme object.
     * It must be a string or a list, or the call is in error.
     */
    n = SCHEME_INT_VAL(GUARANTEE_INTEGER(prim->name, 0));
    list = argv[1];

    if (!SCHEME_STRINGP(list) && !SCHEME_PAIRP(list))
	scheme_wrong_type(prim->name, "string or list", 1, argc, argv);
    buf = get_buffer_arg(prim->name, 2, argc, argv);

    if (n != 0)	    // 0 can be used in insert
	check_line_range(n, buf->buf);
    if (SCHEME_STRINGP(list))
    {
	buf_T	    *savebuf = curbuf;

	str = string_to_line(list);
	curbuf = buf->buf;

	if (u_save((linenr_T)n, (linenr_T)(n+1)) == FAIL)
	{
	    curbuf = savebuf;
	    vim_free(str);
	    raise_vim_exn(_("cannot save undo information"));
	}
	else if (ml_append((linenr_T)n, (char_u *)str, 0, FALSE) == FAIL)
	{
	    curbuf = savebuf;
	    vim_free(str);
	    raise_vim_exn(_("cannot insert line"));
	}
	else
	{
	    vim_free(str);
	    appended_lines_mark((linenr_T)n, 1L);
	}

	curbuf = savebuf;
	update_screen(UPD_VALID);

	MZ_GC_UNREG();
	raise_if_error();
	return scheme_void;
    }

    // List
    size = scheme_proper_list_length(list);
    MZ_GC_CHECK();
    if (size < 0)	// improper or cyclic list
	scheme_wrong_type(prim->name, "proper list",
		2, argc, argv);
    else
    {
	Scheme_Object   *line = NULL;
	Scheme_Object   *rest = NULL;
	char		**array;
	buf_T		*savebuf = curbuf;

	MZ_GC_DECL_REG(2);
	MZ_GC_VAR_IN_REG(0, line);
	MZ_GC_VAR_IN_REG(1, rest);
	MZ_GC_REG();

	array = ALLOC_CLEAR_MULT(char *, size + 1);

	rest = list;
	for (i = 0; i < size; ++i)
	{
	    line = SCHEME_CAR(rest);
	    rest = SCHEME_CDR(rest);
	    array[i] = string_to_line(line);
	}

	curbuf = buf->buf;

	if (u_save((linenr_T)n, (linenr_T)(n + 1)) == FAIL)
	{
	    curbuf = savebuf;
	    free_array(array);
	    raise_vim_exn(_("cannot save undo information"));
	}
	else
	{
	    for (i = 0; i < size; ++i)
		if (ml_append((linenr_T)(n + i), (char_u *)array[i],
			    0, FALSE) == FAIL)
		{
		    curbuf = savebuf;
		    free_array(array);
		    raise_vim_exn(_("cannot insert line"));
		}

	    if (i > 0)
		appended_lines_mark((linenr_T)n, (long)i);
	}
	free_array(array);
	MZ_GC_UNREG();
	curbuf = savebuf;
	update_screen(UPD_VALID);
    }

    MZ_GC_UNREG();
    raise_if_error();
    return scheme_void;
}

/*
 * Predicates
 */
/*
 * (buff? obj)
 */
    static Scheme_Object *
vim_bufferp(void *data UNUSED, int argc UNUSED, Scheme_Object **argv)
{
    if (SCHEME_VIMBUFFERP(argv[0]))
	return scheme_true;
    else
	return scheme_false;
}

/*
 * (win? obj)
 */
    static Scheme_Object *
vim_windowp(void *data UNUSED, int argc UNUSED, Scheme_Object **argv)
{
    if (SCHEME_VIMWINDOWP(argv[0]))
	return scheme_true;
    else
	return scheme_false;
}

/*
 * (buff-valid? obj)
 */
    static Scheme_Object *
vim_buffer_validp(void *data UNUSED, int argc UNUSED, Scheme_Object **argv)
{
    if (SCHEME_VIMBUFFERP(argv[0])
	    && ((vim_mz_buffer *)argv[0])->buf != INVALID_BUFFER_VALUE)
	return scheme_true;
    else
	return scheme_false;
}

/*
 * (win-valid? obj)
 */
    static Scheme_Object *
vim_window_validp(void *data UNUSED, int argc UNUSED, Scheme_Object **argv)
{
    if (SCHEME_VIMWINDOWP(argv[0])
	    && ((vim_mz_window *)argv[0])->win != INVALID_WINDOW_VALUE)
	return scheme_true;
    else
	return scheme_false;
}

/*
 *===========================================================================
 * Utilities
 *===========================================================================
 */

/*
 * Convert an MzScheme string into a Vim line.
 *
 * All internal nulls are replaced by newline characters.
 * It is an error for the string to contain newline characters.
 *
 * Returns pointer to Vim allocated memory
 */
    static char *
string_to_line(Scheme_Object *obj)
{
    char	*scheme_str = NULL;
    char	*vim_str = NULL;
    OUTPUT_LEN_TYPE	len;
    int		i;

    scheme_str = scheme_display_to_string(obj, &len);

    // Error checking: String must not contain newlines, as we
    // are replacing a single line, and we must replace it with
    // a single line.
    if (memchr(scheme_str, '\n', len))
	scheme_signal_error(_("string cannot contain newlines"));

    vim_str = alloc(len + 1);

    // Create a copy of the string, with internal nulls replaced by
    // newline characters, as is the vim convention.
    for (i = 0; i < len; ++i)
    {
	if (scheme_str[i] == '\0')
	    vim_str[i] = '\n';
	else
	    vim_str[i] = scheme_str[i];
    }

    vim_str[i] = '\0';

    MZ_GC_CHECK();
    return vim_str;
}

#ifdef FEAT_EVAL
/*
 * Convert Vim value into MzScheme, adopted from if_python.c
 */
    static Scheme_Object *
vim_to_mzscheme(typval_T *vim_value)
{
    Scheme_Object	*result = NULL;
    // hash table to store visited values to avoid infinite loops
    Scheme_Hash_Table	*visited = NULL;

    MZ_GC_DECL_REG(2);
    MZ_GC_VAR_IN_REG(0, result);
    MZ_GC_VAR_IN_REG(1, visited);
    MZ_GC_REG();

    visited = scheme_make_hash_table(SCHEME_hash_ptr);
    MZ_GC_CHECK();

    result = vim_to_mzscheme_impl(vim_value, 1, visited);

    MZ_GC_UNREG();
    return result;
}

    static Scheme_Object *
vim_to_mzscheme_impl(typval_T *vim_value, int depth, Scheme_Hash_Table *visited)
{
    Scheme_Object   *result = NULL;
    int		    new_value = TRUE;

    MZ_GC_DECL_REG(2);
    MZ_GC_VAR_IN_REG(0, result);
    MZ_GC_VAR_IN_REG(1, visited);
    MZ_GC_REG();

    // Avoid infinite recursion
    if (depth > 100)
    {
	MZ_GC_UNREG();
	return scheme_void;
    }

    // Check if we run into a recursive loop.  The item must be in visited
    // then and we can use it again.
    result = scheme_hash_get(visited, (Scheme_Object *)vim_value);
    MZ_GC_CHECK();
    if (result != NULL) // found, do nothing
	new_value = FALSE;
    else if (vim_value->v_type == VAR_STRING)
    {
	result = scheme_make_byte_string((char *)vim_value->vval.v_string);
	MZ_GC_CHECK();
    }
    else if (vim_value->v_type == VAR_NUMBER)
    {
	result = scheme_make_integer((long)vim_value->vval.v_number);
	MZ_GC_CHECK();
    }
    else if (vim_value->v_type == VAR_FLOAT)
    {
	result = scheme_make_double((double)vim_value->vval.v_float);
	MZ_GC_CHECK();
    }
    else if (vim_value->v_type == VAR_LIST)
    {
	list_T		*list = vim_value->vval.v_list;
	listitem_T	*curr;

	if (list == NULL || list->lv_first == NULL)
	    result = scheme_null;
	else
	{
	    Scheme_Object   *obj = NULL;

	    MZ_GC_DECL_REG(1);
	    MZ_GC_VAR_IN_REG(0, obj);
	    MZ_GC_REG();

	    curr = list->lv_u.mat.lv_last;
	    obj = vim_to_mzscheme_impl(&curr->li_tv, depth + 1, visited);
	    result = scheme_make_pair(obj, scheme_null);
	    MZ_GC_CHECK();

	    while (curr != list->lv_first)
	    {
		curr = curr->li_prev;
		obj = vim_to_mzscheme_impl(&curr->li_tv, depth + 1, visited);
		result = scheme_make_pair(obj, result);
		MZ_GC_CHECK();
	    }
	}
	MZ_GC_UNREG();
    }
    else if (vim_value->v_type == VAR_DICT)
    {
	Scheme_Object	  *key = NULL;
	Scheme_Object	  *obj = NULL;

	MZ_GC_DECL_REG(2);
	MZ_GC_VAR_IN_REG(0, key);
	MZ_GC_VAR_IN_REG(1, obj);
	MZ_GC_REG();

	result = (Scheme_Object *)scheme_make_hash_table(SCHEME_hash_ptr);
	MZ_GC_CHECK();
	if (vim_value->vval.v_dict != NULL)
	{
	    hashtab_T	*ht = &vim_value->vval.v_dict->dv_hashtab;
	    long_u	todo = ht->ht_used;
	    hashitem_T	*hi;
	    dictitem_T	*di;

	    FOR_ALL_HASHTAB_ITEMS(ht, hi, todo)
	    {
		if (!HASHITEM_EMPTY(hi))
		{
		    --todo;

		    di = dict_lookup(hi);
		    obj = vim_to_mzscheme_impl(&di->di_tv, depth + 1, visited);
		    key = scheme_make_byte_string((char *)hi->hi_key);
		    MZ_GC_CHECK();
		    scheme_hash_set((Scheme_Hash_Table *)result, key, obj);
		    MZ_GC_CHECK();
		}
	    }
	}
	MZ_GC_UNREG();
    }
    else if (vim_value->v_type == VAR_FUNC)
    {
	Scheme_Object *funcname = NULL;

	MZ_GC_DECL_REG(1);
	MZ_GC_VAR_IN_REG(0, funcname);
	MZ_GC_REG();

	// FIXME: func_ref() and func_unref() are needed.
	funcname = scheme_make_byte_string((char *)vim_value->vval.v_string);
	MZ_GC_CHECK();
	result = scheme_make_closed_prim_w_arity(vim_funcref, funcname,
		(const char *)BYTE_STRING_VALUE(funcname), 0, -1);
	MZ_GC_CHECK();

	MZ_GC_UNREG();
    }
    else if (vim_value->v_type == VAR_PARTIAL)
    {
	if (vim_value->vval.v_partial == NULL)
	    result = scheme_null;
	else
	{
	    Scheme_Object *funcname = NULL;

	    MZ_GC_DECL_REG(1);
	    MZ_GC_VAR_IN_REG(0, funcname);
	    MZ_GC_REG();

	    // FIXME: func_ref() and func_unref() are needed.
	    // TODO: Support pt_dict and pt_argv.
	    funcname = scheme_make_byte_string(
			      (char *)partial_name(vim_value->vval.v_partial));
	    MZ_GC_CHECK();
	    result = scheme_make_closed_prim_w_arity(vim_funcref, funcname,
		    (const char *)BYTE_STRING_VALUE(funcname), 0, -1);
	    MZ_GC_CHECK();

	    MZ_GC_UNREG();
	}
    }
    else if (vim_value->v_type == VAR_BOOL || vim_value->v_type == VAR_SPECIAL)
    {
	if (vim_value->vval.v_number <= VVAL_TRUE)
	    result = scheme_make_integer((long)vim_value->vval.v_number);
	else
	    result = scheme_null;
	MZ_GC_CHECK();
    }
    else
    {
	result = scheme_void;
	new_value = FALSE;
    }
    if (new_value)
    {
	scheme_hash_set(visited, (Scheme_Object *)vim_value, result);
	MZ_GC_CHECK();
    }
    MZ_GC_UNREG();
    return result;
}

    static int
mzscheme_to_vim(Scheme_Object *obj, typval_T *tv)
{
    int i, status;
    Scheme_Hash_Table	*visited = NULL;

    MZ_GC_DECL_REG(2);
    MZ_GC_VAR_IN_REG(0, obj);
    MZ_GC_VAR_IN_REG(1, visited);
    MZ_GC_REG();

    visited = scheme_make_hash_table(SCHEME_hash_ptr);
    MZ_GC_CHECK();

    status = mzscheme_to_vim_impl(obj, tv, 1, visited);
    for (i = 0; i < visited->size; ++i)
    {
	// free up remembered objects
	if (visited->vals[i] != NULL)
	    free_tv((typval_T *)visited->vals[i]);
    }

    MZ_GC_UNREG();
    return status;
}
    static int
mzscheme_to_vim_impl(Scheme_Object *obj, typval_T *tv, int depth,
	Scheme_Hash_Table *visited)
{
    int		status = OK;
    typval_T	*found;

    MZ_GC_DECL_REG(2);
    MZ_GC_VAR_IN_REG(0, obj);
    MZ_GC_VAR_IN_REG(1, visited);
    MZ_GC_REG();

    MZ_GC_CHECK();
    if (depth > 100) // limit the deepest recursion level
    {
	tv->v_type = VAR_NUMBER;
	tv->vval.v_number = 0;
	return FAIL;
    }

    found = (typval_T *)scheme_hash_get(visited, obj);
    if (found != NULL)
	copy_tv(found, tv);
    else if (SCHEME_VOIDP(obj))
    {
	tv->v_type = VAR_SPECIAL;
	tv->vval.v_number = VVAL_NULL;
    }
    else if (SCHEME_INTP(obj))
    {
	tv->v_type = VAR_NUMBER;
	tv->vval.v_number = SCHEME_INT_VAL(obj);
    }
    else if (SCHEME_BOOLP(obj))
    {
	tv->v_type = VAR_BOOL;
	tv->vval.v_number = SCHEME_TRUEP(obj);
    }
    else if (SCHEME_DBLP(obj))
    {
	tv->v_type = VAR_FLOAT;
	tv->vval.v_float = SCHEME_DBL_VAL(obj);
    }
    else if (SCHEME_BYTE_STRINGP(obj))
    {
	tv->v_type = VAR_STRING;
	tv->vval.v_string = vim_strsave(BYTE_STRING_VALUE(obj));
    }
# if MZSCHEME_VERSION_MAJOR >= 299
    else if (SCHEME_CHAR_STRINGP(obj))
    {
	Scheme_Object *tmp = NULL;
	MZ_GC_DECL_REG(1);
	MZ_GC_VAR_IN_REG(0, tmp);
	MZ_GC_REG();

	tmp = scheme_char_string_to_byte_string(obj);
	tv->v_type = VAR_STRING;
	tv->vval.v_string = vim_strsave(BYTE_STRING_VALUE(tmp));
	MZ_GC_UNREG();
    }
#endif
    else if (SCHEME_VECTORP(obj) || SCHEME_NULLP(obj)
	    || SCHEME_PAIRP(obj) || SCHEME_MUTABLE_PAIRP(obj))
    {
	list_T  *list = list_alloc();
	if (list == NULL)
	    status = FAIL;
	else
	{
	    int		    i;
	    Scheme_Object   *curr = NULL;
	    Scheme_Object   *cval = NULL;
	    // temporary var to hold current element of vectors and pairs
	    typval_T	    *v;

	    MZ_GC_DECL_REG(2);
	    MZ_GC_VAR_IN_REG(0, curr);
	    MZ_GC_VAR_IN_REG(1, cval);
	    MZ_GC_REG();

	    tv->v_type = VAR_LIST;
	    tv->vval.v_list = list;
	    ++list->lv_refcount;

	    v = ALLOC_ONE(typval_T);
	    if (v == NULL)
		status = FAIL;
	    else
	    {
		// add the value in advance to allow handling of self-referential
		// data structures
		typval_T    *visited_tv = ALLOC_ONE(typval_T);
		copy_tv(tv, visited_tv);
		scheme_hash_set(visited, obj, (Scheme_Object *)visited_tv);

		if (SCHEME_VECTORP(obj))
		{
		    for (i = 0; i < SCHEME_VEC_SIZE(obj); ++i)
		    {
			cval = SCHEME_VEC_ELS(obj)[i];
			status = mzscheme_to_vim_impl(cval, v, depth + 1, visited);
			if (status == FAIL)
			    break;
			status = list_append_tv(list, v);
			clear_tv(v);
			if (status == FAIL)
			    break;
		    }
		}
		else if (SCHEME_PAIRP(obj) || SCHEME_MUTABLE_PAIRP(obj))
		{
		    for (curr = obj;
			    SCHEME_PAIRP(curr) || SCHEME_MUTABLE_PAIRP(curr);
			    curr = SCHEME_CDR(curr))
		    {
			cval = SCHEME_CAR(curr);
			status = mzscheme_to_vim_impl(cval, v, depth + 1, visited);
			if (status == FAIL)
			    break;
			status = list_append_tv(list, v);
			clear_tv(v);
			if (status == FAIL)
			    break;
		    }
		    // improper list not terminated with null
		    // need to handle the last element
		    if (status == OK && !SCHEME_NULLP(curr))
		    {
			status = mzscheme_to_vim_impl(cval, v, depth + 1, visited);
			if (status == OK)
			{
			    status = list_append_tv(list, v);
			    clear_tv(v);
			}
		    }
		}
		// nothing to do for scheme_null
		vim_free(v);
	    }
	    MZ_GC_UNREG();
	}
    }
    else if (SCHEME_HASHTP(obj))
    {
	int		i;
	dict_T		*dict;
	Scheme_Object   *key = NULL;
	Scheme_Object   *val = NULL;

	MZ_GC_DECL_REG(2);
	MZ_GC_VAR_IN_REG(0, key);
	MZ_GC_VAR_IN_REG(1, val);
	MZ_GC_REG();

	dict = dict_alloc();
	if (dict == NULL)
	    status = FAIL;
	else
	{
	    typval_T    *visited_tv = ALLOC_ONE(typval_T);

	    tv->v_type = VAR_DICT;
	    tv->vval.v_dict = dict;
	    ++dict->dv_refcount;

	    copy_tv(tv, visited_tv);
	    scheme_hash_set(visited, obj, (Scheme_Object *)visited_tv);

	    for (i = 0; i < ((Scheme_Hash_Table *)obj)->size; ++i)
	    {
		if (((Scheme_Hash_Table *) obj)->vals[i] != NULL)
		{
		    // generate item for `display'ed Scheme key
		    dictitem_T  *item = dictitem_alloc((char_u *)string_to_line(
				((Scheme_Hash_Table *) obj)->keys[i]));
		    // convert Scheme val to Vim and add it to the dict
		    if (mzscheme_to_vim_impl(((Scheme_Hash_Table *) obj)->vals[i],
				    &item->di_tv, depth + 1, visited) == FAIL
			    || dict_add(dict, item) == FAIL)
		    {
			dictitem_free(item);
			status = FAIL;
			break;
		    }
		}

	    }
	}
	MZ_GC_UNREG();
    }
    else
    {
	// `display' any other value to string
	tv->v_type = VAR_STRING;
	tv->vval.v_string = (char_u *)string_to_line(obj);
    }
    MZ_GC_UNREG();
    return status;
}

/*
 * Scheme prim procedure wrapping Vim funcref
 */
    static Scheme_Object *
vim_funcref(void *name, int argc, Scheme_Object **argv)
{
    int i;
    typval_T args;
    int status = OK;
    Scheme_Object *result = NULL;
    list_T *list = list_alloc();

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, result);
    MZ_GC_REG();

    result = scheme_void;
    if (list == NULL)
	status = FAIL;
    else
    {
	args.v_type = VAR_LIST;
	args.vval.v_list = list;
	++list->lv_refcount;
	for (i = 0; status == OK && i < argc; ++i)
	{
	    typval_T *v = ALLOC_ONE(typval_T);
	    if (v == NULL)
		status = FAIL;
	    else
	    {
		status = mzscheme_to_vim(argv[i], v);
		if (status == OK)
		{
		    status = list_append_tv(list, v);
		    clear_tv(v);
		}
		vim_free(v);
	    }
	}
	if (status == OK)
	{
	    typval_T ret;
	    ret.v_type = VAR_UNKNOWN;

	    mzscheme_call_vim(BYTE_STRING_VALUE((Scheme_Object *)name), &args, &ret);
	    MZ_GC_CHECK();
	    result = vim_to_mzscheme(&ret);
	    clear_tv(&ret);
	    MZ_GC_CHECK();
	}
    }
    clear_tv(&args);
    MZ_GC_UNREG();
    if (status != OK)
	raise_vim_exn(_("error converting Scheme values to Vim"));
    else
	raise_if_error();
    return result;
}

    void
do_mzeval(char_u *str, typval_T *rettv)
{
    Scheme_Object	*ret = NULL;

    MZ_GC_DECL_REG(1);
    MZ_GC_VAR_IN_REG(0, ret);
    MZ_GC_REG();

    if (mzscheme_init())
    {
	MZ_GC_UNREG();
	return;
    }

    MZ_GC_CHECK();
    if (eval_with_exn_handling(str, do_eval, &ret) == OK)
	mzscheme_to_vim(ret, rettv);

    MZ_GC_UNREG();
}
#endif

/*
 * Check to see whether a Vim error has been reported, or a keyboard
 * interrupt (from vim --> got_int) has been detected.
 */
    static int
vim_error_check(void)
{
    return (got_int || did_emsg);
}

/*
 * register Scheme exn:vim
 */
    static void
register_vim_exn(void)
{
    int	nc = 0;
    int i;
    Scheme_Object   *struct_exn = NULL;
    Scheme_Object   *exn_name = NULL;

    MZ_GC_DECL_REG(2);
    MZ_GC_VAR_IN_REG(0, struct_exn);
    MZ_GC_VAR_IN_REG(1, exn_name);
    MZ_GC_REG();

    exn_name = scheme_intern_symbol("exn:vim");
    MZ_GC_CHECK();
    struct_exn = scheme_builtin_value("struct:exn");
    MZ_GC_CHECK();

    if (vim_exn == NULL)
	vim_exn = scheme_make_struct_type(exn_name,
		struct_exn, NULL, 0, 0, NULL, NULL
#if MZSCHEME_VERSION_MAJOR >= 299
		, NULL
#endif
		);


    {
	Scheme_Object   **tmp = NULL;
	Scheme_Object   *exn_names[5] = {NULL, NULL, NULL, NULL, NULL};
	Scheme_Object   *exn_values[5] = {NULL, NULL, NULL, NULL, NULL};
	MZ_GC_DECL_REG(6);
	MZ_GC_ARRAY_VAR_IN_REG(0, exn_names, 5);
	MZ_GC_ARRAY_VAR_IN_REG(3, exn_values, 5);
	MZ_GC_REG();

	tmp = scheme_make_struct_names(exn_name, scheme_null, 0, &nc);
	mch_memmove(exn_names, tmp, nc * sizeof(Scheme_Object *));
	MZ_GC_CHECK();

	tmp = scheme_make_struct_values(vim_exn, exn_names, nc, 0);
	mch_memmove(exn_values, tmp, nc * sizeof(Scheme_Object *));
	MZ_GC_CHECK();

	for (i = 0; i < nc; i++)
	{
	    scheme_add_global_symbol(exn_names[i],
		    exn_values[i], environment);
	    MZ_GC_CHECK();
	}
	MZ_GC_UNREG();
    }
    MZ_GC_UNREG();
}

/*
 * raise exn:vim, may be with additional info string
 */
    void
raise_vim_exn(const char *add_info)
{
    char	    *fmt = _("Vim error: ~a");
    Scheme_Object   *argv[2] = {NULL, NULL};
    Scheme_Object   *exn = NULL;
    Scheme_Object   *byte_string = NULL;

    MZ_GC_DECL_REG(5);
    MZ_GC_ARRAY_VAR_IN_REG(0, argv, 2);
    MZ_GC_VAR_IN_REG(3, exn);
    MZ_GC_VAR_IN_REG(4, byte_string);
    MZ_GC_REG();

    if (add_info != NULL)
    {
	char		*c_string = NULL;
	Scheme_Object   *info = NULL;

	MZ_GC_DECL_REG(3);
	MZ_GC_VAR_IN_REG(0, c_string);
	MZ_GC_VAR_IN_REG(2, info);
	MZ_GC_REG();

	info = scheme_make_byte_string(add_info);
	MZ_GC_CHECK();
	c_string = scheme_format_utf8(fmt, (int)STRLEN(fmt), 1, &info, NULL);
	MZ_GC_CHECK();
	byte_string = scheme_make_byte_string(c_string);
	MZ_GC_CHECK();
	argv[0] = scheme_byte_string_to_char_string(byte_string);
	SCHEME_SET_IMMUTABLE(argv[0]);
	MZ_GC_UNREG();
    }
    else
    {
	byte_string = scheme_make_byte_string(_("Vim error"));
	MZ_GC_CHECK();
	argv[0] = scheme_byte_string_to_char_string(byte_string);
	MZ_GC_CHECK();
    }
    MZ_GC_CHECK();

#if MZSCHEME_VERSION_MAJOR < 360
    argv[1] = scheme_current_continuation_marks();
    MZ_GC_CHECK();
#else
    argv[1] = scheme_current_continuation_marks(NULL);
    MZ_GC_CHECK();
#endif

    exn = scheme_make_struct_instance(vim_exn, 2, argv);
    MZ_GC_CHECK();
    scheme_raise(exn);
    MZ_GC_UNREG();
}

    void
raise_if_error(void)
{
    if (vim_error_check())
	raise_vim_exn(NULL);
}

/*
 * get buffer:
 * either current
 * or passed as argv[argnum] with checks
 */
    static vim_mz_buffer *
get_buffer_arg(const char *fname, int argnum, int argc, Scheme_Object **argv)
{
    vim_mz_buffer *b;

    if (argc < argnum + 1)
	return get_vim_curr_buffer();
    if (!SCHEME_VIMBUFFERP(argv[argnum]))
	scheme_wrong_type(fname, "vim-buffer", argnum, argc, argv);
    b = (vim_mz_buffer *)argv[argnum];
    (void)get_valid_buffer(argv[argnum]);
    return b;
}

/*
 * get window:
 * either current
 * or passed as argv[argnum] with checks
 */
    static vim_mz_window *
get_window_arg(const char *fname, int argnum, int argc, Scheme_Object **argv)
{
    vim_mz_window *w;

    if (argc < argnum + 1)
	return get_vim_curr_window();
    w = (vim_mz_window *)argv[argnum];
    if (!SCHEME_VIMWINDOWP(argv[argnum]))
	scheme_wrong_type(fname, "vim-window", argnum, argc, argv);
    (void)get_valid_window(argv[argnum]);
    return w;
}

/*
 * get valid Vim buffer from Scheme_Object*
 */
buf_T *get_valid_buffer(void *obj)
{
    buf_T *buf = ((vim_mz_buffer *)obj)->buf;

    if (buf == INVALID_BUFFER_VALUE)
	scheme_signal_error(_("buffer is invalid"));
    return buf;
}

/*
 * get valid Vim window from Scheme_Object*
 */
win_T *get_valid_window(void *obj)
{
    win_T *win = ((vim_mz_window *)obj)->win;
    if (win == INVALID_WINDOW_VALUE)
	scheme_signal_error(_("window is invalid"));
    return win;
}

    int
mzthreads_allowed(void)
{
    return mz_threads_allow;
}

    static int
line_in_range(linenr_T lnum, buf_T *buf)
{
    return (lnum > 0 && lnum <= buf->b_ml.ml_line_count);
}

    static void
check_line_range(linenr_T lnum, buf_T *buf)
{
    if (!line_in_range(lnum, buf))
	scheme_signal_error(_("linenr out of range"));
}

/*
 * Check if deleting lines made the cursor position invalid
 * (or you'll get msg from Vim about invalid linenr).
 * Changed the lines from "lo" to "hi" and added "extra" lines (negative if
 * deleted). Got from if_python.c
 */
    static void
mz_fix_cursor(int lo, int hi, int extra)
{
    if (curwin->w_cursor.lnum >= lo)
    {
	// Adjust the cursor position if it's in/after the changed
	// lines.
	if (curwin->w_cursor.lnum >= hi)
	{
	    curwin->w_cursor.lnum += extra;
	    check_cursor_col();
	}
	else if (extra < 0)
	{
	    curwin->w_cursor.lnum = lo;
	    check_cursor();
	}
	else
	    check_cursor_col();
	changed_cline_bef_curs();
    }
    invalidate_botline();
}

static Vim_Prim prims[]=
{
    /*
     * Buffer-related commands
     */
    {get_buffer_line, "get-buff-line", 1, 2},
    {set_buffer_line, "set-buff-line", 2, 3},
    {get_buffer_line_list, "get-buff-line-list", 2, 3},
    {get_buffer_name, "get-buff-name", 0, 1},
    {get_buffer_num, "get-buff-num", 0, 1},
    {get_buffer_size, "get-buff-size", 0, 1},
    {set_buffer_line_list, "set-buff-line-list", 3, 4},
    {insert_buffer_line_list, "insert-buff-line-list", 2, 3},
    {get_curr_buffer, "curr-buff", 0, 0},
    {get_buffer_count, "buff-count", 0, 0},
    {get_next_buffer, "get-next-buff", 0, 1},
    {get_prev_buffer, "get-prev-buff", 0, 1},
    {mzscheme_open_buffer, "open-buff", 1, 1},
    {get_buffer_by_name, "get-buff-by-name", 1, 1},
    {get_buffer_by_num, "get-buff-by-num", 1, 1},
    /*
     * Window-related commands
     */
    {get_curr_win, "curr-win", 0, 0},
    {get_window_count, "win-count", 0, 0},
    {get_window_by_num, "get-win-by-num", 1, 1},
    {get_window_num, "get-win-num", 0, 1},
    {get_window_buffer, "get-win-buffer", 0, 1},
    {get_window_height, "get-win-height", 0, 1},
    {set_window_height, "set-win-height", 1, 2},
    {get_window_width, "get-win-width", 0, 1},
    {set_window_width, "set-win-width", 1, 2},
    {get_cursor, "get-cursor", 0, 1},
    {set_cursor, "set-cursor", 1, 2},
    {get_window_list, "get-win-list", 0, 1},
    /*
     * Vim-related commands
     */
    {vim_command, "command", 1, 1},
    {vim_eval, "eval", 1, 1},
    {get_range_start, "range-start", 0, 0},
    {get_range_end, "range-end", 0, 0},
    {mzscheme_beep, "beep", 0, 0},
    {get_option, "get-option", 1, 2},
    {set_option, "set-option", 1, 2},
    /*
     * small utilities
     */
    {vim_bufferp, "buff?", 1, 1},
    {vim_windowp, "win?", 1, 1},
    {vim_buffer_validp, "buff-valid?", 1, 1},
    {vim_window_validp, "win-valid?", 1, 1}
};

/*
 * return MzScheme wrapper for curbuf
 */
    static vim_mz_buffer *
get_vim_curr_buffer(void)
{
    if (curbuf->b_mzscheme_ref == NULL)
	return (vim_mz_buffer *)buffer_new(curbuf);
    else
	return BUFFER_REF(curbuf);
}

/*
 * return MzScheme wrapper for curwin
 */
    static vim_mz_window *
get_vim_curr_window(void)
{
    if (curwin->w_mzscheme_ref == NULL)
	return (vim_mz_window *)window_new(curwin);
    else
	return WINDOW_REF(curwin);
}

    static void
make_modules(void)
{
    int		    i;
    Scheme_Env	    *mod = NULL;
    Scheme_Object   *vimext_symbol = NULL;
    Scheme_Object   *closed_prim = NULL;

    MZ_GC_DECL_REG(3);
    MZ_GC_VAR_IN_REG(0, mod);
    MZ_GC_VAR_IN_REG(1, vimext_symbol);
    MZ_GC_VAR_IN_REG(2, closed_prim);
    MZ_GC_REG();

    vimext_symbol = scheme_intern_symbol("vimext");
    MZ_GC_CHECK();
    mod = scheme_primitive_module(vimext_symbol, environment);
    MZ_GC_CHECK();
    // all prims made closed so they can access their own names
    for (i = 0; i < (int)ARRAY_LENGTH(prims); i++)
    {
	Vim_Prim *prim = prims + i;
	closed_prim = scheme_make_closed_prim_w_arity(prim->prim, prim, prim->name,
			    prim->mina, prim->maxa);
	scheme_add_global(prim->name, closed_prim, mod);
	MZ_GC_CHECK();
    }
    scheme_finish_primitive_module(mod);
    MZ_GC_CHECK();
    MZ_GC_UNREG();
}

#ifdef HAVE_SANDBOX
static Scheme_Object *M_write = NULL;
static Scheme_Object *M_read = NULL;
static Scheme_Object *M_execute = NULL;
static Scheme_Object *M_delete = NULL;

    static void
sandbox_check(void)
{
    if (sandbox)
	raise_vim_exn(_("not allowed in the Vim sandbox"));
}

/*
 * security guards to force Vim's sandbox restrictions on MzScheme level
 */
    static Scheme_Object *
sandbox_file_guard(int argc UNUSED, Scheme_Object **argv)
{
    if (!sandbox)
	return scheme_void;

    Scheme_Object *requested_access = argv[2];

    if (M_write == NULL)
    {
	MZ_REGISTER_STATIC(M_write);
	M_write = scheme_intern_symbol("write");
	MZ_GC_CHECK();
    }
    if (M_read == NULL)
    {
	MZ_REGISTER_STATIC(M_read);
	M_read = scheme_intern_symbol("read");
	MZ_GC_CHECK();
    }
    if (M_execute == NULL)
    {
	MZ_REGISTER_STATIC(M_execute);
	M_execute = scheme_intern_symbol("execute");
	MZ_GC_CHECK();
    }
    if (M_delete == NULL)
    {
	MZ_REGISTER_STATIC(M_delete);
	M_delete = scheme_intern_symbol("delete");
	MZ_GC_CHECK();
    }

    while (!SCHEME_NULLP(requested_access))
    {
	Scheme_Object *item = SCHEME_CAR(requested_access);
	if (scheme_eq(item, M_write) || scheme_eq(item, M_read)
		|| scheme_eq(item, M_execute) || scheme_eq(item, M_delete))
	    raise_vim_exn(_("not allowed in the Vim sandbox"));
	requested_access = SCHEME_CDR(requested_access);
    }

    return scheme_void;
}

    static Scheme_Object *
sandbox_network_guard(int argc UNUSED, Scheme_Object **argv UNUSED)
{
    return scheme_void;
}
#endif

#endif
