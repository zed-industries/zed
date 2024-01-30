/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * Tcl extensions by Ingo Wilken <Ingo.Wilken@informatik.uni-oldenburg.de>
 * Last modification: Wed May 10 21:28:44 CEST 2000
 * Requires Tcl 8.0 or higher.
 *
 *  Variables:
 *  ::vim::current(buffer)	# Name of buffer command for current buffer.
 *  ::vim::current(window)	# Name of window command for current window.
 *  ::vim::range(start)		# Start of current range (line number).
 *  ::vim::range(end)		# End of current range (line number).
 *  ::vim::lbase		# Start of line/column numbers (1 or 0).
 *
 *  Commands:
 *  ::vim::command {cmd}	# Execute ex command {cmd}.
 *  ::vim::option {opt} [val]	# Get/Set option {opt}.
 *  ::vim::expr {expr}		# Evaluate {expr} using vim's evaluator.
 *  ::vim::beep			# Guess.
 *
 *  set buf [::vim::buffer {n}]	# Create Tcl command for buffer N.
 *  set bl [::vim::buffer list] # Get list of Tcl commands of all buffers.
 *  ::vim::buffer exists {n}	# True if buffer {n} exists.
 *
 *  set wl [::vim::window list] # Get list of Tcl commands of all windows.
 *
 *  set n [$win height]		# Report window height.
 *  $win height {n}		# Set window height to {n}.
 *  array set pos [$win cursor] # Get cursor position.
 *  $win cursor {row} {col}	# Set cursor position.
 *  $win cursor pos		# Set cursor position from array var "pos"
 *  $win delcmd {cmd}		# Register callback command for closed window.
 *  $win option {opt} [val]	# Get/Set vim option in context of $win.
 *  $win command {cmd}		# Execute ex command in context of $win.
 *  $win expr {expr}		# Evaluate vim expression in context of $win.
 *  set buf [$win buffer]	# Create Tcl command for window's buffer.
 *
 *  $buf name			# Reports file name in buffer.
 *  $buf number			# Reports buffer number.
 *  set l [$buf get {n}]	# Get buffer line {n} as a string.
 *  set L [$buf get {n} {m}]	# Get lines {n} through {m} as a list.
 *  $buf count			# Reports number of lines in buffer.
 *  $buf last			# Reports number of last line in buffer.
 *  $buf delete {n}		# Delete line {n}.
 *  $buf delete {n} {m}		# Delete lines {n} through {m}.
 *  $buf set {n} {l}		# Set line {n} to string {l}.
 *  $buf set {n} {m} {L}	# Set lines {n} through {m} from list {L}.
 *				# Delete/inserts lines as appropriate.
 *  $buf option {opt} [val]	# Get/Set vim option in context of $buf.
 *  $buf command {cmd}		# Execute ex command in context of $buf
 *  $buf expr {cmd}		# Evaluate vim expression in context of $buf.
 *  array set pos [$buf mark {m}]   # Get position of mark.
 *  $buf append {n} {str}	# Append string {str} to buffer,after line {n}.
 *  $buf insert {n} {str}	# Insert string {str} in buffer as line {n}.
 *  $buf delcmd {cmd}		# Register callback command for deleted buffer.
 *  set wl [$buf windows]	# Get list of Tcl commands for all windows of
 *				# this buffer.
TODO:
 *  ::vim::buffer new		#   create new buffer + Tcl command
 */

#include "vim.h"
#undef EXTERN			// tcl.h defines it too

#ifdef DYNAMIC_TCL
# define USE_TCL_STUBS // use tcl's stubs mechanism
#endif

#include <tcl.h>
#include <string.h>

typedef struct
{
    Tcl_Interp *interp;
    int exitvalue;
    int range_start, range_end;
    int lbase;
    char *curbuf, *curwin;
} tcl_info;

static tcl_info tclinfo = { NULL, 0, 0, 0, 0, NULL, NULL };

#define VAR_RANGE1	"::vim::range(start)"
#define VAR_RANGE2	"::vim::range(begin)"
#define VAR_RANGE3	"::vim::range(end)"
#define VAR_CURBUF	"::vim::current(buffer)"
#define VAR_CURWIN	"::vim::current(window)"
#define VAR_LBASE	"::vim::lbase"
#define VAR_CURLINE	"line"
#define VAR_CURLNUM	"lnum"
#define VARNAME_SIZE	64

#define row2tcl(x)  ((x) - (tclinfo.lbase==0))
#define row2vim(x)  ((x) + (tclinfo.lbase==0))
#define col2tcl(x)  ((x) + (tclinfo.lbase!=0))
#define col2vim(x)  ((x) - (tclinfo.lbase!=0))


#define VIMOUT	((ClientData)1)
#define VIMERR	((ClientData)2)

// This appears to be new in Tcl 8.4.
#ifndef CONST84
# define CONST84
#endif

/*
 *  List of Tcl interpreters who reference a vim window or buffer.
 *  Each buffer and window has its own list in the w_tcl_ref or b_tcl_ref
 *  struct member.  We need this because Tcl can create sub-interpreters with
 *  the "interp" command, and each interpreter can reference all windows and
 *  buffers.
 */
struct ref
{
    struct ref	*next;

    Tcl_Interp	*interp;
    Tcl_Command cmd;	    // Tcl command that represents this object
    Tcl_Obj	*delcmd;    // Tcl command to call when object is being del.
    void	*vimobj;    // Vim window or buffer (win_T* or buf_T*)
};
static char * tclgetbuffer _ANSI_ARGS_((Tcl_Interp *interp, buf_T *buf));
static char * tclgetwindow _ANSI_ARGS_((Tcl_Interp *interp, win_T *win));
static int tclsetdelcmd _ANSI_ARGS_((Tcl_Interp *interp, struct ref *reflist, void *vimobj, Tcl_Obj *delcmd));
static int tclgetlinenum _ANSI_ARGS_ ((Tcl_Interp *interp, Tcl_Obj *obj, int *valueP, buf_T *buf));
static win_T *tclfindwin _ANSI_ARGS_ ((buf_T *buf));
static int tcldoexcommand _ANSI_ARGS_ ((Tcl_Interp *interp, int objc, Tcl_Obj *CONST objv[], int objn));
static int tclsetoption _ANSI_ARGS_ ((Tcl_Interp *interp, int objc, Tcl_Obj *CONST objv[], int objn));
static int tclvimexpr _ANSI_ARGS_ ((Tcl_Interp *interp, int objc, Tcl_Obj *CONST objv[], int objn));
static void tcldelthisinterp _ANSI_ARGS_ ((void));

static int vimerror _ANSI_ARGS_((Tcl_Interp *interp));
static void tclmsg _ANSI_ARGS_((char *text));
static void tclerrmsg _ANSI_ARGS_((char *text));
static void tclupdatevars _ANSI_ARGS_((void));

static struct ref refsdeleted;	// dummy object for deleted ref list

//////////////////////////////////////////////////////////////////////////////
// TCL interface manager
////////////////////////////////////////////////////////////////////////////

#if defined(DYNAMIC_TCL) || defined(PROTO)
# ifndef DYNAMIC_TCL_DLL
#  define DYNAMIC_TCL_DLL "tcl83.dll"
# endif
# ifndef DYNAMIC_TCL_VER
#  define DYNAMIC_TCL_VER "8.3"
# endif

# ifndef  DYNAMIC_TCL // Just generating prototypes
typedef int HANDLE;
# endif

# ifdef MSWIN
#  define TCL_PROC FARPROC
#  define load_dll vimLoadLib
#  define symbol_from_dll GetProcAddress
#  define close_dll FreeLibrary
#  define load_dll_error GetWin32Error
# else
#  include <dlfcn.h>
#  define HANDLE void*
#  define TCL_PROC void*
#  define load_dll(n) dlopen((n), RTLD_LAZY|RTLD_GLOBAL)
#  define symbol_from_dll dlsym
#  define close_dll dlclose
#  define load_dll_error dlerror
# endif

/*
 * Declare HANDLE for tcl.dll and function pointers.
 */
static HANDLE hTclLib = NULL;
Tcl_Interp* (*dll_Tcl_CreateInterp)(void);
void (*dll_Tcl_FindExecutable)(const void *);

/*
 * Table of name to function pointer of tcl.
 */
static struct {
    char* name;
    TCL_PROC* ptr;
} tcl_funcname_table[] = {
    {"Tcl_CreateInterp", (TCL_PROC*)&dll_Tcl_CreateInterp},
    {"Tcl_FindExecutable", (TCL_PROC*)&dll_Tcl_FindExecutable},
    {NULL, NULL},
};

/*
 * Make all runtime-links of tcl.
 *
 * 1. Get module handle using LoadLibraryEx.
 * 2. Get pointer to tcl function by GetProcAddress.
 * 3. Repeat 2, until get all functions will be used.
 *
 * Parameter 'libname' provides name of DLL.
 * Return OK or FAIL.
 */
    static int
tcl_runtime_link_init(char *libname, int verbose)
{
    int i;

    if (hTclLib)
	return OK;
    if (!(hTclLib = load_dll(libname)))
    {
	if (verbose)
	    semsg(_(e_could_not_load_library_str_str), libname, load_dll_error());
	return FAIL;
    }
    for (i = 0; tcl_funcname_table[i].ptr; ++i)
    {
	if (!(*tcl_funcname_table[i].ptr = symbol_from_dll(hTclLib,
						  tcl_funcname_table[i].name)))
	{
	    close_dll(hTclLib);
	    hTclLib = NULL;
	    if (verbose)
		semsg(_(e_could_not_load_library_function_str),
						   tcl_funcname_table[i].name);
	    return FAIL;
	}
    }
    return OK;
}
#endif // defined(DYNAMIC_TCL) || defined(PROTO)

#ifdef DYNAMIC_TCL
static char *find_executable_arg = NULL;
#endif

    void
vim_tcl_init(char *arg)
{
#ifdef DYNAMIC_TCL
    find_executable_arg = arg;
#else
    Tcl_FindExecutable(arg);
#endif
}

#if defined(DYNAMIC_TCL) || defined(PROTO)

static int stubs_initialized = FALSE;

/*
 * Return TRUE if the TCL interface can be used.
 */
    int
tcl_enabled(int verbose)
{
    if (!stubs_initialized && find_executable_arg != NULL
	    && tcl_runtime_link_init((char *)p_tcldll, verbose) == OK)
    {
	Tcl_Interp *interp;

	// Note: the library will allocate memory to store the executable name,
	// which will be reported as possibly leaked by valgrind.
	dll_Tcl_FindExecutable(find_executable_arg);

	if ((interp = dll_Tcl_CreateInterp()) != NULL)
	{
	    if (Tcl_InitStubs(interp, DYNAMIC_TCL_VER, 0) != NULL)
	    {
		Tcl_DeleteInterp(interp);
		stubs_initialized = TRUE;
	    }
	    // FIXME: When Tcl_InitStubs() was failed, how delete interp?
	}
    }
    return stubs_initialized;
}
#endif

#if defined(EXITFREE) || defined(PROTO)
/*
 * Called once when exiting.
 */
    void
vim_tcl_finalize(void)
{
# ifdef DYNAMIC_TCL
    if (stubs_initialized)
# endif
	Tcl_Finalize();
}
#endif

    void
tcl_end(void)
{
}

/////////////////////////////////////////////////////////////////////////////
// Tcl commands
////////////////////////////////////////////////////////////////////////////

/*
 * Replace standard "exit" command.
 *
 * Delete the Tcl interpreter; a new one will be created with the next
 * :tcl command). The exit code is saved (and retrieved in tclexit()).
 * Since Tcl's exit is never expected to return and this replacement
 * does, then (except for a trivial case) additional Tcl commands will
 * be run. Since the interpreter is now marked as deleted, an error
 * will be returned -- typically "attempt to call eval in deleted
 * interpreter". Hopefully, at this point, checks for TCL_ERROR take
 * place and control percolates back up to Vim -- but with this new error
 * string in the interpreter's result value. Therefore it would be
 * useless for this routine to return the exit code via Tcl_SetResult().
 */
    static int
exitcmd(
    ClientData dummy UNUSED,
    Tcl_Interp *interp,
    int objc,
    Tcl_Obj *CONST objv[])
{
    int value = 0;

    switch (objc)
    {
	case 2:
	    if (Tcl_GetIntFromObj(interp, objv[1], &value) != TCL_OK)
		break;
	    // FALLTHROUGH
	case 1:
	    tclinfo.exitvalue = value;

	    Tcl_DeleteInterp(interp);
	    break;
	default:
	    Tcl_WrongNumArgs(interp, 1, objv, "?returnCode?");
    }
    return TCL_ERROR;
}

/*
 *  "::vim::beep" - what Vi[m] does best :-)
 */
    static int
beepcmd(
    ClientData dummy UNUSED,
    Tcl_Interp *interp,
    int objc,
    Tcl_Obj *CONST objv[])
{
    if (objc != 1)
    {
	Tcl_WrongNumArgs(interp, 1, objv, NULL);
	return TCL_ERROR;
    }
    vim_beep(BO_LANG);
    return TCL_OK;
}

/*
 *  "::vim::buffer list" - create a list of buffer commands.
 *  "::vim::buffer {N}" - create buffer command for buffer N.
 *  "::vim::buffer exists {N}" - test if buffer N exists.
 *  "::vim::buffer new" - create a new buffer (not implemented)
 */
    static int
buffercmd(
    ClientData dummy UNUSED,
    Tcl_Interp *interp,
    int objc,
    Tcl_Obj *CONST objv[])
{
    char	*name;
    buf_T	*buf;
    Tcl_Obj	*resobj;
    int		err, n, idx;
    enum {BCMD_EXISTS, BCMD_LIST};
    static CONST84 char *bcmdoptions[] =
    {
	"exists", "list", (char *)0
    };

    if (objc < 2)
    {
	Tcl_WrongNumArgs(interp, 1, objv, "option");
	return TCL_ERROR;
    }
    err = Tcl_GetIntFromObj(interp, objv[1], &n);
    if (err == TCL_OK)
    {
	if (objc != 2)
	{
	    Tcl_WrongNumArgs(interp, 1, objv, "bufNumber");
	    return TCL_ERROR;
	}
	FOR_ALL_BUFFERS(buf)
	{
	    if (buf->b_fnum == n)
	    {
		name = tclgetbuffer(interp, buf);
		if (name == NULL)
		    return TCL_ERROR;
		Tcl_SetResult(interp, name, TCL_VOLATILE);
		return TCL_OK;
	    }
	}
	Tcl_SetResult(interp, _("invalid buffer number"), TCL_STATIC);
	return TCL_ERROR;
    }
    Tcl_ResetResult(interp); // clear error from Tcl_GetIntFromObj

    err = Tcl_GetIndexFromObj(interp, objv[1], bcmdoptions, "option", 0, &idx);
    if (err != TCL_OK)
	return err;
    switch (idx)
    {
	case BCMD_LIST:
	    if (objc != 2)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "");
		err = TCL_ERROR;
		break;
	    }
	    FOR_ALL_BUFFERS(buf)
	    {
		name = tclgetbuffer(interp, buf);
		if (name == NULL)
		{
		    err = TCL_ERROR;
		    break;
		}
		Tcl_AppendElement(interp, name);
	    }
	    break;

	case BCMD_EXISTS:
	    if (objc != 3)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "bufNumber");
		err = TCL_ERROR;
		break;
	    }
	    err = Tcl_GetIntFromObj(interp, objv[2], &n);
	    if (err == TCL_OK)
	    {
		buf = buflist_findnr(n);
		resobj = Tcl_NewIntObj(buf != NULL);
		Tcl_SetObjResult(interp, resobj);
	    }
	    break;

	default:
	    Tcl_SetResult(interp, _("not implemented yet"), TCL_STATIC);
	    err = TCL_ERROR;
    }
    return err;
}

/*
 * "::vim::window list" - create list of window commands.
 */
    static int
windowcmd(
    ClientData	dummy UNUSED,
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[])
{
    char	*what, *string;
    win_T	*win;

    if (objc != 2)
    {
	Tcl_WrongNumArgs(interp, 1, objv, "option");
	return TCL_ERROR;
    }
    what = Tcl_GetStringFromObj(objv[1], NULL);
    if (strcmp(what, "list") == 0)
    {
	FOR_ALL_WINDOWS(win)
	{
	    string = tclgetwindow(interp, win);
	    if (string == NULL)
		return TCL_ERROR;
	    Tcl_AppendElement(interp, string);
	}
	return TCL_OK;
    }
    Tcl_SetResult(interp, _("unknown option"), TCL_STATIC);
    return TCL_ERROR;
}

/*
 * flags for bufselfcmd and winselfcmd to indicate outstanding actions.
 */
#define FL_UPDATE_SCREEN	(1<<0)
#define FL_UPDATE_CURBUF	(1<<1)
#define FL_ADJUST_CURSOR	(1<<2)

/*
 * This function implements the buffer commands.
 */
    static int
bufselfcmd(
    ClientData	ref,
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[])
{
    int		opt, err, idx, flags;
    int		val1, val2, n, i;
    buf_T	*buf, *savebuf;
    win_T	*win, *savewin;
    Tcl_Obj	*resobj;
    pos_T	*pos;
    char	*line;

    enum
    {
	BUF_APPEND, BUF_COMMAND, BUF_COUNT, BUF_DELCMD, BUF_DELETE, BUF_EXPR,
	BUF_GET, BUF_INSERT, BUF_LAST, BUF_MARK, BUF_NAME, BUF_NUMBER,
	BUF_OPTION, BUF_SET, BUF_WINDOWS
    };
    static CONST84 char *bufoptions[] =
    {
	"append", "command", "count", "delcmd", "delete", "expr",
	"get", "insert", "last", "mark", "name", "number",
	"option", "set", "windows", (char *)0
    };

    if (objc < 2)
    {
	Tcl_WrongNumArgs(interp, 1, objv, "option ?arg ...?");
	return TCL_ERROR;
    }

    err = Tcl_GetIndexFromObj(interp, objv[1], bufoptions, "option", 0, &idx);
    if (err != TCL_OK)
	return err;

    buf = (buf_T *)((struct ref *)ref)->vimobj;
    savebuf = curbuf;  curbuf = buf;
    savewin = curwin;  curwin = tclfindwin(buf);
    flags = 0;
    opt = 0;

    switch (idx)
    {
	case BUF_COMMAND:
	    err = tcldoexcommand(interp, objc, objv, 2);
	    flags |= FL_UPDATE_SCREEN;
	    break;

	case BUF_OPTION:
	    err = tclsetoption(interp, objc, objv, 2);
	    flags |= FL_UPDATE_SCREEN;
	    break;

	case BUF_EXPR:
	    err = tclvimexpr(interp, objc, objv, 2);
	    break;

	case BUF_NAME:
	    /*
	     *	Get filename of buffer.
	     */
	    if (objc != 2)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, NULL);
		err = TCL_ERROR;
		break;
	    }
	    if (buf->b_ffname)
		Tcl_SetResult(interp, (char *)buf->b_ffname, TCL_VOLATILE);
	    else
		Tcl_SetResult(interp, "", TCL_STATIC);
	    break;

	case BUF_LAST:
	    /*
	     * Get line number of last line.
	     */
	    opt = 1;
	    // fallthrough
	case BUF_COUNT:
	    /*
	     * Get number of lines in buffer.
	     */
	    if (objc != 2)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, NULL);
		err = TCL_ERROR;
		break;
	    }
	    val1 = (int)buf->b_ml.ml_line_count;
	    if (opt)
		val1 = row2tcl(val1);

	    resobj = Tcl_NewIntObj(val1);
	    Tcl_SetObjResult(interp, resobj);
	    break;

	case BUF_NUMBER:
	    /*
	     * Get buffer's number.
	     */
	    if (objc != 2)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, NULL);
		err = TCL_ERROR;
		break;
	    }
	    resobj = Tcl_NewIntObj((int)buf->b_fnum);
	    Tcl_SetObjResult(interp, resobj);
	    break;

	case BUF_GET:
	    if (objc != 3 && objc != 4)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "lineNumber ?lineNumber?");
		err = TCL_ERROR;
		break;
	    }
	    err = tclgetlinenum(interp, objv[2], &val1, buf);
	    if (err != TCL_OK)
		break;
	    if (objc == 4)
	    {
		err = tclgetlinenum(interp, objv[3], &val2, buf);
		if (err != TCL_OK)
		    break;
		if (val1 > val2)
		{
		    n = val1; val1 = val2; val2 = n;
		}
		Tcl_ResetResult(interp);

		for (n = val1; n <= val2 && err == TCL_OK; n++)
		{
		    line = (char *)ml_get_buf(buf, (linenr_T)n, FALSE);
		    if (line)
			Tcl_AppendElement(interp, line);
		    else
			err = TCL_ERROR;
		}
	    }
	    else
	    {  // objc == 3
		line = (char *)ml_get_buf(buf, (linenr_T)val1, FALSE);
		Tcl_SetResult(interp, line, TCL_VOLATILE);
	    }
	    break;

	case BUF_SET:
	    if (objc != 4 && objc != 5)
	    {
		Tcl_WrongNumArgs(interp, 3, objv, "lineNumber ?lineNumber? stringOrList");
		err = TCL_ERROR;
		break;
	    }
	    err = tclgetlinenum(interp, objv[2], &val1, buf);
	    if (err != TCL_OK)
		return TCL_ERROR;
	    if (objc == 4)
	    {
		/*
		 *  Replace one line with a string.
		 *	$buf set {n} {string}
		 */
		line = Tcl_GetStringFromObj(objv[3], NULL);
		if (u_savesub((linenr_T)val1) != OK)
		{
		    Tcl_SetResult(interp, _("cannot save undo information"), TCL_STATIC);
		    err = TCL_ERROR;
		}
		else
		if (ml_replace((linenr_T)val1, (char_u *)line, TRUE) != OK)
		{
		    Tcl_SetResult(interp, _("cannot replace line"), TCL_STATIC);
		    err = TCL_ERROR;
		}
		else
		{
		    changed_bytes((linenr_T)val1, 0);
		    flags |= FL_UPDATE_CURBUF;
		}
		break;
	    }
	    else
	    {
		/*
		 * Replace several lines with the elements of a Tcl list.
		 *	$buf set {n} {m} {list}
		 * If the list contains more than {m}-{n}+1 elements, they
		 * are * inserted after line {m}.  If the list contains fewer
		 * elements, * the lines from {n}+length({list}) through {m}
		 * are deleted.
		 */
		int	    lc;
		Tcl_Obj	    **lv;

		err = tclgetlinenum(interp, objv[3], &val2, buf);
		if (err != TCL_OK)
		    break;
		err = Tcl_ListObjGetElements(interp, objv[4], &lc, &lv);
		if (err != TCL_OK)
		    break;
		if (val1 > val2)
		{
		    n = val1;
		    val1 = val2;
		    val2 = n;
		}

		n = val1;
		if (u_save((linenr_T)(val1 - 1), (linenr_T)(val2 + 1)) != OK)
		{
		    Tcl_SetResult(interp, _("cannot save undo information"),
								  TCL_STATIC);
		    err = TCL_ERROR;
		    break;
		}
		flags |= FL_UPDATE_CURBUF;

		for (i = 0; i < lc && n <= val2; i++)
		{
		    line = Tcl_GetStringFromObj(lv[i], NULL);
		    if (ml_replace((linenr_T)n, (char_u *)line, TRUE) != OK)
			goto setListError;
		    ++n;
		}
		if (i < lc)
		{
		    // append lines
		    do
		    {
			line = Tcl_GetStringFromObj(lv[i], NULL);
			if (ml_append((linenr_T)(n - 1),
					      (char_u *)line, 0, FALSE) != OK)
			    goto setListError;
			++n;
			++i;
		    } while (i < lc);
		}
		else if (n <= val2)
		{
		    // did not replace all lines, delete
		    i = n;
		    do
		    {
			if (ml_delete((linenr_T)i) != OK)
			    goto setListError;
			++n;
		    } while (n <= val2);
		}
		lc -= val2 - val1 + 1;	// number of lines to be replaced
		mark_adjust((linenr_T)val1, (linenr_T)val2, (long)MAXLNUM,
								    (long)lc);
		changed_lines((linenr_T)val1, 0, (linenr_T)val2 + 1, (long)lc);
		break;
    setListError:
		u_undo(1);  // ???
		Tcl_SetResult(interp, _("cannot set line(s)"), TCL_STATIC);
		err = TCL_ERROR;
	    }
	    break;

	case BUF_DELETE:
	    if (objc != 3  &&  objc != 4)
	    {
		Tcl_WrongNumArgs(interp, 3, objv, "lineNumber ?lineNumber?");
		err = TCL_ERROR;
		break;
	    }
	    err = tclgetlinenum(interp, objv[2], &val1, buf);
	    if (err != TCL_OK)
		break;
	    val2 = val1;
	    if (objc == 4)
	    {
		err = tclgetlinenum(interp, objv[3], &val2, buf);
		if (err != TCL_OK)
		    return err;
		if (val1 > val2)
		{
		    i = val1; val1 = val2; val2 = i;
		}
	    }
	    n = val2 - val1 + 1;
	    if (u_savedel((linenr_T)val1, (long)n) != OK)
	    {
		Tcl_SetResult(interp, _("cannot save undo information"),
								  TCL_STATIC);
		err = TCL_ERROR;
		break;
	    }
	    for (i = 0; i < n; i++)
	    {
		ml_delete((linenr_T)val1);
		err = vimerror(interp);
		if (err != TCL_OK)
		    break;
	    }
	    if (i > 0)
		deleted_lines_mark((linenr_T)val1, (long)i);
	    flags |= FL_ADJUST_CURSOR|FL_UPDATE_SCREEN;
	    break;

	case BUF_MARK:
	    if (objc != 3)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "markName");
		err = TCL_ERROR;
		break;
	    }
	    line = Tcl_GetStringFromObj(objv[2], NULL);

	    pos = NULL;
	    if (line[0] != '\0'  &&  line[1] == '\0')
		pos = getmark(line[0], FALSE);
	    if (pos == NULL)
	    {
		Tcl_SetResult(interp, _("invalid mark name"), TCL_STATIC);
		err = TCL_ERROR;
		break;
	    }
	    err = vimerror(interp);
	    if (err != TCL_OK)
		break;
	    if (pos->lnum <= 0)
	    {
		Tcl_SetResult(interp, _("mark not set"), TCL_STATIC);
		err = TCL_ERROR;
	    }
	    else
	    {
		char rbuf[64];

		sprintf(rbuf, _("row %d column %d"),
			     (int)row2tcl(pos->lnum), (int)col2tcl(pos->col));
		Tcl_SetResult(interp, rbuf, TCL_VOLATILE);
	    }
	    break;

	case BUF_INSERT:
	    opt = 1;
	    // fallthrough
	case BUF_APPEND:
	    if (objc != 4)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "lineNum text");
		err = TCL_ERROR;
		break;
	    }
	    err = tclgetlinenum(interp, objv[2], &val1, buf);
	    if (err != TCL_OK)
		break;
	    if (opt)
		--val1;
	    if (u_save((linenr_T)val1, (linenr_T)(val1+1)) != OK)
	    {
		Tcl_SetResult(interp, _("cannot save undo information"),
								  TCL_STATIC);
		err = TCL_ERROR;
		break;
	    }

	    line = Tcl_GetStringFromObj(objv[3], NULL);
	    if (ml_append((linenr_T)val1, (char_u *)line, 0, FALSE) != OK)
	    {
		Tcl_SetResult(interp, _("cannot insert/append line"),
								  TCL_STATIC);
		err = TCL_ERROR;
		break;
	    }
	    appended_lines_mark((linenr_T)val1, 1L);
	    flags |= FL_UPDATE_SCREEN;
	    break;

	case BUF_WINDOWS:
	    /*
	     * Return list of window commands.
	     */
	    if (objc != 2)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, NULL);
		err = TCL_ERROR;
		break;
	    }
	    Tcl_ResetResult(interp);
	    FOR_ALL_WINDOWS(win)
	    {
		if (win->w_buffer == buf)
		{
		    line = tclgetwindow(interp, win);
		    if (line != NULL)
			Tcl_AppendElement(interp, line);
		    else
		    {
			err = TCL_ERROR;
			break;
		    }
		}
	    }
	    break;

	case BUF_DELCMD:
	    /*
	     * Register deletion callback.
	     * TODO: Should be able to register multiple callbacks
	     */
	    if (objc != 3)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "command");
		err = TCL_ERROR;
		break;
	    }
	    err = tclsetdelcmd(interp, buf->b_tcl_ref, (void *)buf, objv[2]);
	    break;

	default:
	    Tcl_SetResult(interp, _("not implemented yet"), TCL_STATIC);
	    err = TCL_ERROR;
    }

    if (flags & FL_UPDATE_CURBUF)
	redraw_curbuf_later(UPD_NOT_VALID);
    curbuf = savebuf;
    curwin = savewin;
    if (flags & FL_ADJUST_CURSOR)
	check_cursor();
    if (flags & (FL_UPDATE_SCREEN | FL_UPDATE_CURBUF))
	update_screen(UPD_NOT_VALID);

    return err;
}

/*
 * This function implements the window commands.
 */
    static int
winselfcmd(
    ClientData	ref,
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[])
{
    int		err, idx, flags;
    int		val1, val2;
    Tcl_Obj	*resobj;
    win_T	*savewin, *win;
    buf_T	*savebuf;
    char	*str;

    enum
    {
	WIN_BUFFER, WIN_COMMAND, WIN_CURSOR, WIN_DELCMD, WIN_EXPR,
	WIN_HEIGHT, WIN_OPTION
    };
    static CONST84 char *winoptions[] =
    {
	"buffer", "command", "cursor", "delcmd", "expr",
	"height", "option", (char *)0
    };

    if (objc < 2)
    {
	Tcl_WrongNumArgs(interp, 1, objv, "option ?arg ...?");
	return TCL_ERROR;
    }

    err = Tcl_GetIndexFromObj(interp, objv[1], winoptions, "option", 0,  &idx);
    if (err != TCL_OK)
	return TCL_ERROR;

    win = (win_T *)((struct ref *)ref)->vimobj;
    savewin = curwin;  curwin = win;
    savebuf = curbuf;  curbuf = win->w_buffer;
    flags = 0;

    switch (idx)
    {
	case WIN_OPTION:
	    err = tclsetoption(interp, objc, objv, 2);
	    flags |= FL_UPDATE_SCREEN;
	    break;

	case WIN_COMMAND:
	    err = tcldoexcommand(interp, objc, objv, 2);
	    flags |= FL_UPDATE_SCREEN;
	    break;

	case WIN_EXPR:
	    err = tclvimexpr(interp, objc, objv, 2);
	    break;

	case WIN_HEIGHT:
	    if (objc == 3)
	    {
		err = Tcl_GetIntFromObj(interp, objv[2], &val1);
		if (err != TCL_OK)
		    break;
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		win_setheight(val1);
		err = vimerror(interp);
		if (err != TCL_OK)
		    break;
	    }
	    else
	    if (objc != 2)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "?value?");
		err = TCL_ERROR;
		break;
	    }

	    resobj = Tcl_NewIntObj((int)(win->w_height));
	    Tcl_SetObjResult(interp, resobj);
	    break;

	case WIN_BUFFER:
	    if (objc != 2)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, NULL);
		err = TCL_ERROR;
		break;
	    }
	    str = tclgetbuffer(interp, win->w_buffer);
	    if (str)
		Tcl_SetResult(interp, str, TCL_VOLATILE);
	    else
		err = TCL_ERROR;
	    break;

	case WIN_DELCMD:
	    if (objc != 3)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "command");
		err = TCL_ERROR;
		break;
	    }
	    err = tclsetdelcmd(interp, win->w_tcl_ref, (void *)win, objv[2]);
	    break;

	case WIN_CURSOR:
	    if (objc > 4)
	    {
		Tcl_WrongNumArgs(interp, 2, objv, "?arg1 ?arg2??");
		err = TCL_ERROR;
		break;
	    }
	    if (objc == 2)
	    {
		char buf[64];

		sprintf(buf, _("row %d column %d"), (int)row2tcl(win->w_cursor.lnum), (int)col2tcl(win->w_cursor.col));
		Tcl_SetResult(interp, buf, TCL_VOLATILE);
		break;
	    }
	    else if (objc == 3)
	    {
		Tcl_Obj *part, *var;

		part = Tcl_NewStringObj("row", -1);
		var = Tcl_ObjGetVar2(interp, objv[2], part, TCL_LEAVE_ERR_MSG);
		if (var == NULL)
		{
		    err = TCL_ERROR;
		    break;
		}
		err = tclgetlinenum(interp, var, &val1, win->w_buffer);
		if (err != TCL_OK)
		    break;
		part = Tcl_NewStringObj("column", -1);
		var = Tcl_ObjGetVar2(interp, objv[2], part, TCL_LEAVE_ERR_MSG);
		if (var == NULL)
		{
		    err = TCL_ERROR;
		    break;
		}
		err = Tcl_GetIntFromObj(interp, var, &val2);
		if (err != TCL_OK)
		    break;
	    }
	    else
	    {  // objc == 4
		err = tclgetlinenum(interp, objv[2], &val1, win->w_buffer);
		if (err != TCL_OK)
		    break;
		err = Tcl_GetIntFromObj(interp, objv[3], &val2);
		if (err != TCL_OK)
		    break;
	    }
	    // TODO: should check column
	    win->w_cursor.lnum = val1;
	    win->w_cursor.col = col2vim(val2);
	    win->w_set_curswant = TRUE;
	    flags |= FL_UPDATE_SCREEN;
	    break;

	default:
	    Tcl_SetResult(interp, _("not implemented yet"), TCL_STATIC);
	    break;
    }

    curwin = savewin;
    curbuf = savebuf;
    if (flags & FL_UPDATE_SCREEN)
	update_screen(UPD_NOT_VALID);

    return err;
}


    static int
commandcmd(
    ClientData	dummy UNUSED,
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[])
{
    int		err;

    err = tcldoexcommand(interp, objc, objv, 1);
    update_screen(UPD_VALID);
    return err;
}

    static int
optioncmd(
    ClientData	dummy UNUSED,
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[])
{
    int		err;

    err = tclsetoption(interp, objc, objv, 1);
    update_screen(UPD_VALID);
    return err;
}

    static int
exprcmd(
    ClientData	dummy UNUSED,
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[])
{
    return tclvimexpr(interp, objc, objv, 1);
}

/////////////////////////////////////////////////////////////////////////////
// Support functions for Tcl commands
////////////////////////////////////////////////////////////////////////////

/*
 * Get a line number from 'obj' and convert it to vim's range.
 */
    static int
tclgetlinenum(
    Tcl_Interp	*interp,
    Tcl_Obj	*obj,
    int		*valueP,
    buf_T	*buf)
{
    int err, i;

    enum { LN_BEGIN, LN_BOTTOM, LN_END, LN_FIRST, LN_LAST, LN_START, LN_TOP };

    static CONST84 char *keyw[] =
    {
	"begin", "bottom", "end", "first", "last", "start", "top", (char *)0
    };

    err = Tcl_GetIndexFromObj(interp, obj, keyw, "", 0, &i);
    if (err == TCL_OK)
    {
	switch (i)
	{
	    case LN_BEGIN:
	    case LN_FIRST:
	    case LN_START:
	    case LN_TOP:
		*valueP = 1;
		break;
	    case LN_BOTTOM:
	    case LN_END:
	    case LN_LAST:
		*valueP = buf->b_ml.ml_line_count;
		break;
	}
	return TCL_OK;
    }
    Tcl_ResetResult(interp);

    err = Tcl_GetIntFromObj(interp, obj, &i);
    if (err != TCL_OK)
	return err;
    i = row2vim(i);
    if (i < 1  ||  i > buf->b_ml.ml_line_count)
    {
	Tcl_SetResult(interp, _("line number out of range"), TCL_STATIC);
	return TCL_ERROR;
    }
    *valueP = i;
    return TCL_OK;
}

/*
 * Find the first window in the window list that displays the buffer.
 */
    static win_T *
tclfindwin(buf_T *buf)
{
    win_T *win;

    FOR_ALL_WINDOWS(win)
    {
	if (win->w_buffer == buf)
	    return win;
    }
    return curwin;  // keep current window context
}

/*
 * Do-it-all function for "::vim::command", "$buf command" and "$win command".
 */
    static int
tcldoexcommand(
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[],
    int		objn)
{
    tcl_info	saveinfo;
    int		err, flag, nobjs;
    char	*arg;

    nobjs = objc - objn;
    if (nobjs < 1 || nobjs > 2)
    {
	Tcl_WrongNumArgs(interp, objn, objv, "?-quiet? exCommand");
	return TCL_ERROR;
    }

    flag = 0;
    if (nobjs == 2)
    {
	arg = Tcl_GetStringFromObj(objv[objn], NULL);
	if (strcmp(arg, "-quiet") == 0)
	    flag = 1;
	else
	{
	    Tcl_ResetResult(interp);
	    Tcl_AppendResult(interp, _("unknown flag: "), arg, (char *)0);
	    return TCL_ERROR;
	}
	++objn;
    }

    memcpy(&saveinfo, &tclinfo, sizeof(tcl_info));
    tclinfo.interp = NULL;
    tclinfo.curwin = NULL;
    tclinfo.curbuf = NULL;

    arg = Tcl_GetStringFromObj(objv[objn], NULL);
    if (flag)
	++emsg_off;
    do_cmdline_cmd((char_u *)arg);
    if (flag)
	--emsg_off;
    err = vimerror(interp);

    // If the ex command created a new Tcl interpreter, remove it
    if (tclinfo.interp)
	tcldelthisinterp();
    memcpy(&tclinfo, &saveinfo, sizeof(tcl_info));
    tclupdatevars();

    return err;
}

/*
 * Do-it-all function for "::vim::option", "$buf option" and "$win option".
 */
    static int
tclsetoption(
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[],
    int		objn)
{
    int		err, nobjs, idx;
    char_u	*option;
    getoption_T	gov;
    long	lval;
    char_u	*sval;
    Tcl_Obj	*resobj;

    enum { OPT_OFF, OPT_ON, OPT_TOGGLE };
    static CONST84 char *optkw[] = { "off", "on", "toggle", (char *)0 };

    nobjs = objc - objn;
    if (nobjs != 1 && nobjs != 2)
    {
	Tcl_WrongNumArgs(interp, objn, objv, "vimOption ?value?");
	return TCL_ERROR;
    }

    option = (char_u *)Tcl_GetStringFromObj(objv[objn], NULL);
    ++objn;
    gov = get_option_value(option, &lval, &sval, NULL, 0);
    err = TCL_OK;
    switch (gov)
    {
	case gov_string:
	    Tcl_SetResult(interp, (char *)sval, TCL_VOLATILE);
	    vim_free(sval);
	    break;
	case gov_bool:
	case gov_number:
	    resobj = Tcl_NewLongObj(lval);
	    Tcl_SetObjResult(interp, resobj);
	    break;
	default:
	    Tcl_SetResult(interp, _("unknown vimOption"), TCL_STATIC);
	    return TCL_ERROR;
    }
    if (nobjs == 2)
    {
	if (gov != gov_string)
	{
	    sval = NULL;    // avoid compiler warning
	    err = Tcl_GetIndexFromObj(interp, objv[objn], optkw, "", 0, &idx);
	    if (err != TCL_OK)
	    {
		Tcl_ResetResult(interp);
		err = Tcl_GetLongFromObj(interp, objv[objn], &lval);
	    }
	    else
	    {
		switch (idx)
		{
		    case OPT_ON:
			lval = 1;
			break;
		    case OPT_OFF:
			lval = 0;
			break;
		    case OPT_TOGGLE:
			lval = !lval;
			break;
		}
	    }
	}
	else
	    sval = (char_u *)Tcl_GetStringFromObj(objv[objn], NULL);
	if (err == TCL_OK)
	{
	    set_option_value_give_err(option, lval, sval, OPT_LOCAL);
	    err = vimerror(interp);
	}
    }
    return err;
}

/*
 * Do-it-all function for "::vim::expr", "$buf expr" and "$win expr".
 */
    static int
tclvimexpr(
    Tcl_Interp	*interp,
    int		objc,
    Tcl_Obj	*CONST objv[],
    int		objn)
{
#ifdef FEAT_EVAL
    char	*expr, *str;
#endif
    int		err;

    if (objc - objn != 1)
    {
	Tcl_WrongNumArgs(interp, objn, objv, "vimExpr");
	return TCL_ERROR;
    }

#ifdef FEAT_EVAL
    expr = Tcl_GetStringFromObj(objv[objn], NULL);
    str = (char *)eval_to_string((char_u *)expr, TRUE, FALSE);
    if (str == NULL)
	Tcl_SetResult(interp, _("invalid expression"), TCL_STATIC);
    else
    {
	Tcl_SetResult(interp, str, TCL_VOLATILE);
	vim_free(str);
    }
    err = vimerror(interp);
#else
    Tcl_SetResult(interp, _("expressions disabled at compile time"), TCL_STATIC);
    err = TCL_ERROR;
#endif

    return err;
}

/*
 * Check for internal vim errors.
 */
    static int
vimerror(Tcl_Interp *interp)
{
    if (got_int)
    {
	Tcl_SetResult(interp, _("keyboard interrupt"), TCL_STATIC);
	return TCL_ERROR;
    }
    else if (did_emsg)
    {
	Tcl_SetResult(interp, _("Vim error"), TCL_STATIC);
	return TCL_ERROR;
    }
    return TCL_OK;
}

/*
 * Functions that handle the reference lists:
 *   delref() - callback for Tcl's DeleteCommand
 *   tclgetref() - find/create Tcl command for a win_T* or buf_T* object
 *   tclgetwindow() - window frontend for tclgetref()
 *   tclgetbuffer() - buffer frontend for tclgetref()
 *   tclsetdelcmd() - add Tcl callback command to a vim object
 */
    static void
delref(ClientData cref)
{
    struct ref *ref = (struct ref *)cref;

    if (ref->delcmd)
    {
	Tcl_DecrRefCount(ref->delcmd);
	ref->delcmd = NULL;
    }
    ref->interp = NULL;
}

    static char *
tclgetref(
    Tcl_Interp	*interp,
    void	**refstartP,	// ptr to w_tcl_ref/b_tcl-ref member of
				// win_T/buf_T struct
    char	*prefix,	// "win" or "buf"
    void	*vimobj,	// win_T* or buf_T*
    Tcl_ObjCmdProc *proc)	// winselfcmd or bufselfcmd
{
    struct ref *ref, *unused = NULL;
    static char name[VARNAME_SIZE];
    Tcl_Command cmd;

    ref = (struct ref *)(*refstartP);
    if (ref == &refsdeleted)
    {
	Tcl_SetResult(interp, _("cannot create buffer/window command: object is being deleted"), TCL_STATIC);
	return NULL;
    }

    while (ref != NULL)
    {
	if (ref->interp == interp)
	    break;
	if (ref->interp == NULL)
	    unused = ref;
	ref = ref->next;
    }

    if (ref)
	vim_snprintf(name, sizeof(name), "::vim::%s",
					Tcl_GetCommandName(interp, ref->cmd));
    else
    {
	if (unused)
	    ref = unused;
	else
	{
	    ref = (struct ref *)Tcl_Alloc(sizeof(struct ref));
	    ref->interp = NULL;
	    ref->next = (struct ref *)(*refstartP);
	    (*refstartP) = (void *)ref;
	}

	// This might break on some exotic systems...
	vim_snprintf(name, sizeof(name), "::vim::%s_%lx",
					       prefix, (unsigned long)vimobj);
	cmd = Tcl_CreateObjCommand(interp, name, proc,
	    (ClientData)ref, (Tcl_CmdDeleteProc *)delref);
	if (!cmd)
	    return NULL;

	ref->interp = interp;
	ref->cmd = cmd;
	ref->delcmd = NULL;
	ref->vimobj = vimobj;
    }
    return name;
}

    static char *
tclgetwindow(Tcl_Interp *interp, win_T *win)
{
    return tclgetref(interp, &(win->w_tcl_ref), "win", (void *)win, winselfcmd);
}

    static char *
tclgetbuffer(Tcl_Interp *interp, buf_T *buf)
{
    return tclgetref(interp, &(buf->b_tcl_ref), "buf", (void *)buf, bufselfcmd);
}

    static int
tclsetdelcmd(
    Tcl_Interp	*interp,
    struct ref	*reflist,
    void	*vimobj,
    Tcl_Obj	*delcmd)
{
    if (reflist == &refsdeleted)
    {
	Tcl_SetResult(interp, _("cannot register callback command: buffer/window is already being deleted"), TCL_STATIC);
	return TCL_ERROR;
    }

    while (reflist != NULL)
    {
	if (reflist->interp == interp && reflist->vimobj == vimobj)
	{
	    if (reflist->delcmd)
		Tcl_DecrRefCount(reflist->delcmd);
	    Tcl_IncrRefCount(delcmd);
	    reflist->delcmd = delcmd;
	    return TCL_OK;
	}
	reflist = reflist->next;
    }
    // This should never happen.  Famous last word?
    iemsg(e_tcl_fatal_error_reflist_corrupt_please_report_this);
    Tcl_SetResult(interp, _("cannot register callback command: buffer/window reference not found"), TCL_STATIC);
    return TCL_ERROR;
}


////////////////////////////////////////////
//    I/O Channel
////////////////////////////////////////////

    static int
tcl_channel_close(ClientData instance, Tcl_Interp *interp UNUSED)
{
    int		err = 0;

    // currently does nothing

    if (instance != VIMOUT && instance != VIMERR)
    {
	Tcl_SetErrno(EBADF);
	err = EBADF;
    }
    return err;
}

    static int
tcl_channel_input(
    ClientData	instance UNUSED,
    char	*buf UNUSED,
    int		bufsiz UNUSED,
    int		*errptr)
{

    // input is currently not supported

    Tcl_SetErrno(EINVAL);
    if (errptr)
	*errptr = EINVAL;
    return -1;
}

    static int
tcl_channel_output(
    ClientData	instance,
    const char	*buf,
    int		bufsiz,
    int		*errptr)
{
    char_u	*str;
    int		result;

    // The buffer is not guaranteed to be 0-terminated, and we don't if
    // there is enough room to add a '\0'.  So we have to create a copy
    // of the buffer...
    str = vim_strnsave((char_u *)buf, bufsiz);
    if (!str)
    {
	Tcl_SetErrno(ENOMEM);
	if (errptr)
	    *errptr = ENOMEM;
	return -1;
    }

    result = bufsiz;
    if (instance == VIMOUT)
	tclmsg((char *)str);
    else
    if (instance == VIMERR)
	tclerrmsg((char *)str);
    else
    {
	Tcl_SetErrno(EBADF);
	if (errptr)
	    *errptr = EBADF;
	result = -1;
    }
    vim_free(str);
    return result;
}

    static void
tcl_channel_watch(ClientData instance UNUSED, int mask UNUSED)
{
    Tcl_SetErrno(EINVAL);
}

    static int
tcl_channel_gethandle(
    ClientData	instance UNUSED,
    int		direction UNUSED,
    ClientData	*handleptr UNUSED)
{
    Tcl_SetErrno(EINVAL);
    return EINVAL;
}


static Tcl_ChannelType tcl_channel_type =
{
    "vimmessage",	// typeName
    TCL_CHANNEL_VERSION_2, // version
    tcl_channel_close,	// closeProc
    tcl_channel_input,	// inputProc
    tcl_channel_output,	// outputProc
    NULL,		// seekProc
    NULL,		// setOptionProc
    NULL,		// getOptionProc
    tcl_channel_watch,	// watchProc
    tcl_channel_gethandle, // getHandleProc
    NULL,		// close2Proc
    NULL,		// blockModeProc
#ifdef TCL_CHANNEL_VERSION_2
    NULL,		// flushProc
    NULL,		// handlerProc
#endif
// The following should not be necessary since TCL_CHANNEL_VERSION_2 was
// set above
#ifdef TCL_CHANNEL_VERSION_3
    NULL,		// wideSeekProc
#endif
#ifdef TCL_CHANNEL_VERSION_4
    NULL,		// threadActionProc
#endif
#ifdef TCL_CHANNEL_VERSION_5
    NULL		// truncateProc
#endif
};

///////////////////////////////////
// Interface to vim
//////////////////////////////////

    static void
tclupdatevars(void)
{
    char varname[VARNAME_SIZE];	// must be writeable
    char *name;

    strcpy(varname, VAR_RANGE1);
    Tcl_UpdateLinkedVar(tclinfo.interp, varname);
    strcpy(varname, VAR_RANGE2);
    Tcl_UpdateLinkedVar(tclinfo.interp, varname);
    strcpy(varname, VAR_RANGE3);
    Tcl_UpdateLinkedVar(tclinfo.interp, varname);

    strcpy(varname, VAR_LBASE);
    Tcl_UpdateLinkedVar(tclinfo.interp, varname);

    name = tclgetbuffer(tclinfo.interp, curbuf);
    strcpy(tclinfo.curbuf, name);
    strcpy(varname, VAR_CURBUF);
    Tcl_UpdateLinkedVar(tclinfo.interp, varname);

    name = tclgetwindow(tclinfo.interp, curwin);
    strcpy(tclinfo.curwin, name);
    strcpy(varname, VAR_CURWIN);
    Tcl_UpdateLinkedVar(tclinfo.interp, varname);
}


    static int
tclinit(exarg_T *eap)
{
    char varname[VARNAME_SIZE];	// Tcl_LinkVar requires writeable varname
    char *name;

#ifdef DYNAMIC_TCL
    if (!tcl_enabled(TRUE))
    {
	emsg(_(e_sorry_this_command_is_disabled_tcl_library_could_not_be_loaded));
	return FAIL;
    }
#endif

    if (!tclinfo.interp)
    {
	Tcl_Interp *interp;
	static Tcl_Channel ch1, ch2;

	// Create replacement channels for stdout and stderr; this has to be
	// done each time an interpreter is created since the channels are closed
	// when the interpreter is deleted
	ch1 = Tcl_CreateChannel(&tcl_channel_type, "vimout", VIMOUT, TCL_WRITABLE);
	ch2 = Tcl_CreateChannel(&tcl_channel_type, "vimerr", VIMERR, TCL_WRITABLE);
	Tcl_SetStdChannel(ch1, TCL_STDOUT);
	Tcl_SetStdChannel(ch2, TCL_STDERR);

	interp = Tcl_CreateInterp();
	Tcl_Preserve(interp);
	if (Tcl_Init(interp) == TCL_ERROR)
	{
	    Tcl_Release(interp);
	    Tcl_DeleteInterp(interp);
	    return FAIL;
	}
#if 0
	// VIM sure is interactive
	Tcl_SetVar(interp, "tcl_interactive", "1", TCL_GLOBAL_ONLY);
#endif

	Tcl_SetChannelOption(interp, ch1, "-buffering", "line");
#ifdef MSWIN
	Tcl_SetChannelOption(interp, ch1, "-translation", "lf");
#endif
	Tcl_SetChannelOption(interp, ch2, "-buffering", "line");
#ifdef MSWIN
	Tcl_SetChannelOption(interp, ch2, "-translation", "lf");
#endif

	// replace standard Tcl exit command
	Tcl_DeleteCommand(interp, "exit");
	Tcl_CreateObjCommand(interp, "exit", exitcmd,
	    (ClientData)NULL, (Tcl_CmdDeleteProc *)NULL);

	// new commands, in ::vim namespace
	Tcl_CreateObjCommand(interp, "::vim::buffer", buffercmd,
	    (ClientData)NULL, (Tcl_CmdDeleteProc *)NULL);
	Tcl_CreateObjCommand(interp, "::vim::window", windowcmd,
	   (ClientData)NULL, (Tcl_CmdDeleteProc *)NULL);
	Tcl_CreateObjCommand(interp, "::vim::command", commandcmd,
	   (ClientData)NULL, (Tcl_CmdDeleteProc *)NULL);
	Tcl_CreateObjCommand(interp, "::vim::beep", beepcmd,
	   (ClientData)NULL, (Tcl_CmdDeleteProc *)NULL);
	Tcl_CreateObjCommand(interp, "::vim::option", optioncmd,
	   (ClientData)NULL, (Tcl_CmdDeleteProc *)NULL);
	Tcl_CreateObjCommand(interp, "::vim::expr", exprcmd,
	   (ClientData)NULL, (Tcl_CmdDeleteProc *)NULL);

	// "lbase" variable
	tclinfo.lbase = 1;
	strcpy(varname, VAR_LBASE);
	Tcl_LinkVar(interp, varname, (char *)&tclinfo.lbase, TCL_LINK_INT);

	// "range" variable
	tclinfo.range_start = eap->line1;
	strcpy(varname, VAR_RANGE1);
	Tcl_LinkVar(interp, varname, (char *)&tclinfo.range_start, TCL_LINK_INT|TCL_LINK_READ_ONLY);
	strcpy(varname, VAR_RANGE2);
	Tcl_LinkVar(interp, varname, (char *)&tclinfo.range_start, TCL_LINK_INT|TCL_LINK_READ_ONLY);
	tclinfo.range_end   = eap->line2;
	strcpy(varname, VAR_RANGE3);
	Tcl_LinkVar(interp, varname, (char *)&tclinfo.range_end, TCL_LINK_INT|TCL_LINK_READ_ONLY);

	// "current" variable
	tclinfo.curbuf = Tcl_Alloc(VARNAME_SIZE);
	tclinfo.curwin = Tcl_Alloc(VARNAME_SIZE);
	name = tclgetbuffer(interp, curbuf);
	strcpy(tclinfo.curbuf, name);
	strcpy(varname, VAR_CURBUF);
	Tcl_LinkVar(interp, varname, (char *)&tclinfo.curbuf, TCL_LINK_STRING|TCL_LINK_READ_ONLY);
	name = tclgetwindow(interp, curwin);
	strcpy(tclinfo.curwin, name);
	strcpy(varname, VAR_CURWIN);
	Tcl_LinkVar(interp, varname, (char *)&tclinfo.curwin, TCL_LINK_STRING|TCL_LINK_READ_ONLY);

	tclinfo.interp = interp;
    }
    else
    {
	// Interpreter already exists, just update variables
	tclinfo.range_start = row2tcl(eap->line1);
	tclinfo.range_end = row2tcl(eap->line2);
	tclupdatevars();
    }

    tclinfo.exitvalue = 0;
    return OK;
}

    static void
tclerrmsg(char *text)
{
    char *next;

    while ((next=strchr(text, '\n')))
    {
	*next++ = '\0';
	emsg(text);
	text = next;
    }
    if (*text)
	emsg(text);
}

    static void
tclmsg(char *text)
{
    char *next;

    while ((next=strchr(text, '\n')))
    {
	*next++ = '\0';
	msg(text);
	text = next;
    }
    if (*text)
	msg(text);
}

    static void
tcldelthisinterp(void)
{
    if (!Tcl_InterpDeleted(tclinfo.interp))
	Tcl_DeleteInterp(tclinfo.interp);
    Tcl_Release(tclinfo.interp);
    // The interpreter is now gets deleted.  All registered commands (esp.
    // window and buffer commands) are deleted, triggering their deletion
    // callback, which deletes all refs pointing to this interpreter.
    // We could garbage-collect the unused ref structs in all windows and
    // buffers, but unless the user creates hundreds of sub-interpreters
    // all referring to lots of windows and buffers, this is hardly worth
    // the effort.  Unused refs are recycled by other interpreters, and
    // all refs are free'd when the window/buffer gets closed by vim.

    tclinfo.interp = NULL;
    Tcl_Free(tclinfo.curbuf);
    Tcl_Free(tclinfo.curwin);
    tclinfo.curbuf = tclinfo.curwin = NULL;
}

    static int
tclexit(int error)
{
    int newerr = OK;

    if (Tcl_InterpDeleted(tclinfo.interp)     // True if we intercepted Tcl's exit command
#if (TCL_MAJOR_VERSION == 8 && TCL_MINOR_VERSION >= 5) || TCL_MAJOR_VERSION > 8
	|| Tcl_LimitExceeded(tclinfo.interp)  // True if the interpreter cannot continue
#endif
	)
    {
	char buf[50];

	sprintf(buf, _(e_exit_code_nr), tclinfo.exitvalue);
	tclerrmsg(buf);
	if (tclinfo.exitvalue == 0)
	{
	    did_emsg = 0;
	    newerr = OK;
	}
	else
	    newerr = FAIL;

	tcldelthisinterp();
    }
    else
    {
	char *result;

	result = (char *)Tcl_GetStringResult(tclinfo.interp);
	if (error == TCL_OK)
	{
	    tclmsg(result);
	    newerr = OK;
	}
	else
	{
	    tclerrmsg(result);
	    newerr = FAIL;
	}
    }

    return newerr;
}

/*
 * ":tcl"
 */
    void
ex_tcl(exarg_T *eap)
{
    char_u	*script;
    int		err;

    script = script_get(eap, eap->arg);
    if (!eap->skip)
    {
	err = tclinit(eap);
	if (err == OK)
	{
	    Tcl_AllowExceptions(tclinfo.interp);
	    if (script == NULL)
		err = Tcl_Eval(tclinfo.interp, (char *)eap->arg);
	    else
		err = Tcl_Eval(tclinfo.interp, (char *)script);
	    err = tclexit(err);
	}
    }
    vim_free(script);
}

/*
 * ":tclfile"
 */
    void
ex_tclfile(exarg_T *eap)
{
    char *file = (char *)eap->arg;
    int err;

    err = tclinit(eap);
    if (err == OK)
    {
	Tcl_AllowExceptions(tclinfo.interp);
	err = Tcl_EvalFile(tclinfo.interp, file);
	err = tclexit(err);
    }
}

/*
 * ":tcldo"
 */
    void
ex_tcldo(exarg_T *eap)
{
    char	*script, *line;
    int		err, rs, re, lnum;
    char	var_lnum[VARNAME_SIZE]; // must be writeable memory
    char	var_line[VARNAME_SIZE];
    linenr_T	first_line = 0;
    linenr_T	last_line = 0;
    buf_T	*was_curbuf = curbuf;

    rs = eap->line1;
    re = eap->line2;
    script = (char *)eap->arg;
    strcpy(var_lnum, VAR_CURLNUM);
    strcpy(var_line, VAR_CURLINE);

    err = tclinit(eap);
    if (err != OK)
	return;

    lnum = row2tcl(rs);
    Tcl_LinkVar(tclinfo.interp, var_lnum, (char *)&lnum, TCL_LINK_INT|TCL_LINK_READ_ONLY);
    err = TCL_OK;
    if (u_save((linenr_T)(rs-1), (linenr_T)(re+1)) != OK)
    {
	Tcl_SetResult(tclinfo.interp, _("cannot save undo information"), TCL_STATIC);
	err = TCL_ERROR;
    }
    while (err == TCL_OK  &&  rs <= re)
    {
	if ((linenr_T)rs > curbuf->b_ml.ml_line_count)
	    break;
	line = (char *)ml_get_buf(curbuf, (linenr_T)rs, FALSE);
	if (!line)
	{
	    Tcl_SetResult(tclinfo.interp, _("cannot get line"), TCL_STATIC);
	    err = TCL_ERROR;
	    break;
	}
	Tcl_SetVar(tclinfo.interp, var_line, line, 0);
	Tcl_AllowExceptions(tclinfo.interp);
	err = Tcl_Eval(tclinfo.interp, script);
	if (err != TCL_OK
	    || Tcl_InterpDeleted(tclinfo.interp)
#if (TCL_MAJOR_VERSION == 8 && TCL_MINOR_VERSION >= 5) || TCL_MAJOR_VERSION > 8
	    || Tcl_LimitExceeded(tclinfo.interp)
#endif
	    || curbuf != was_curbuf
	    || (linenr_T)rs > curbuf->b_ml.ml_line_count)
	    break;
	line = (char *)Tcl_GetVar(tclinfo.interp, var_line, 0);
	if (line)
	{
	    if (ml_replace((linenr_T)rs, (char_u *)line, TRUE) != OK)
	    {
		Tcl_SetResult(tclinfo.interp, _("cannot replace line"), TCL_STATIC);
		err = TCL_ERROR;
		break;
	    }
	    if (first_line == 0)
		first_line = rs;
	    last_line = rs;
	}
	++rs;
	++lnum;
	Tcl_UpdateLinkedVar(tclinfo.interp, var_lnum);
    }
    if (first_line)
	changed_lines(first_line, 0, last_line + 1, (long)0);

    Tcl_UnsetVar(tclinfo.interp, var_line, 0);
    Tcl_UnlinkVar(tclinfo.interp, var_lnum);
    if (err == TCL_OK)
	Tcl_ResetResult(tclinfo.interp);

    (void)tclexit(err);
}

    static void
tcldelallrefs(struct ref *ref)
{
    struct ref	*next;
    int		err;
    char	*result;

#ifdef DYNAMIC_TCL
    // TODO: this code currently crashes Vim on exit
    if (exiting)
	return;
#endif

    while (ref != NULL)
    {
	next = ref->next;
	if (ref->interp)
	{
	    if (ref->delcmd)
	    {
		err = Tcl_GlobalEvalObj(ref->interp, ref->delcmd);
		if (err != TCL_OK)
		{
		    result = (char *)Tcl_GetStringResult(ref->interp);
		    if (result)
			tclerrmsg(result);
		}
		Tcl_DecrRefCount(ref->delcmd);
		ref->delcmd = NULL;
	    }
	    Tcl_DeleteCommandFromToken(ref->interp, ref->cmd);
	}
	Tcl_Free((char *)ref);
	ref = next;
    }
}

    void
tcl_buffer_free(buf_T *buf)
{
    struct ref *reflist;

#ifdef DYNAMIC_TCL
    if (!stubs_initialized)	// Not using Tcl, nothing to do.
	return;
#endif

    reflist = (struct ref *)(buf->b_tcl_ref);
    if (reflist != &refsdeleted)
    {
	buf->b_tcl_ref = (void *)&refsdeleted;
	tcldelallrefs(reflist);
	buf->b_tcl_ref = NULL;
    }
}

    void
tcl_window_free(win_T *win)
{
    struct ref *reflist;

#ifdef DYNAMIC_TCL
    if (!stubs_initialized)	// Not using Tcl, nothing to do.
	return;
#endif

    reflist = (struct ref*)(win->w_tcl_ref);
    if (reflist != &refsdeleted)
    {
	win->w_tcl_ref = (void *)&refsdeleted;
	tcldelallrefs(reflist);
	win->w_tcl_ref = NULL;
    }
}

// The End
