/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *				GUI/Motif support by Robert Webb
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * Icons used by the toolbar code.
 */
#include "../pixmaps/tb_new.xpm"
#include "../pixmaps/tb_open.xpm"
#include "../pixmaps/tb_close.xpm"
#include "../pixmaps/tb_save.xpm"
#include "../pixmaps/tb_print.xpm"
#include "../pixmaps/tb_cut.xpm"
#include "../pixmaps/tb_copy.xpm"
#include "../pixmaps/tb_paste.xpm"
#include "../pixmaps/tb_find.xpm"
#include "../pixmaps/tb_find_next.xpm"
#include "../pixmaps/tb_find_prev.xpm"
#include "../pixmaps/tb_find_help.xpm"
#include "../pixmaps/tb_exit.xpm"
#include "../pixmaps/tb_undo.xpm"
#include "../pixmaps/tb_redo.xpm"
#include "../pixmaps/tb_help.xpm"
#include "../pixmaps/tb_macro.xpm"
#include "../pixmaps/tb_make.xpm"
#include "../pixmaps/tb_save_all.xpm"
#include "../pixmaps/tb_jump.xpm"
#include "../pixmaps/tb_ctags.xpm"
#include "../pixmaps/tb_load_session.xpm"
#include "../pixmaps/tb_save_session.xpm"
#include "../pixmaps/tb_new_session.xpm"
#include "../pixmaps/tb_blank.xpm"
#include "../pixmaps/tb_maximize.xpm"
#include "../pixmaps/tb_split.xpm"
#include "../pixmaps/tb_minimize.xpm"
#include "../pixmaps/tb_shell.xpm"
#include "../pixmaps/tb_replace.xpm"
#include "../pixmaps/tb_vsplit.xpm"
#include "../pixmaps/tb_maxwidth.xpm"
#include "../pixmaps/tb_minwidth.xpm"

/*
 * Those are the pixmaps used for the default buttons.
 */
static char **(built_in_pixmaps[]) =
{
    tb_new_xpm,
    tb_open_xpm,
    tb_save_xpm,
    tb_undo_xpm,
    tb_redo_xpm,
    tb_cut_xpm,
    tb_copy_xpm,
    tb_paste_xpm,
    tb_print_xpm,
    tb_help_xpm,
    tb_find_xpm,
    tb_save_all_xpm,
    tb_save_session_xpm,
    tb_new_session_xpm,
    tb_load_session_xpm,
    tb_macro_xpm,
    tb_replace_xpm,
    tb_close_xpm,
    tb_maximize_xpm,
    tb_minimize_xpm,
    tb_split_xpm,
    tb_shell_xpm,
    tb_find_prev_xpm,
    tb_find_next_xpm,
    tb_find_help_xpm,
    tb_make_xpm,
    tb_jump_xpm,
    tb_ctags_xpm,
    tb_vsplit_xpm,
    tb_maxwidth_xpm,
    tb_minwidth_xpm,
    tb_exit_xpm
};

// Indices for named colors
#define BACKGROUND	0
#define FOREGROUND	1
#define BOTTOM_SHADOW	2
#define TOP_SHADOW	3
#define HIGHLIGHT	4
