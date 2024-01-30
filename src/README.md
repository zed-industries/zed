![Vim Logo](https://github.com/vim/vim/blob/master/runtime/vimlogo.gif)

# Vim source code #

Here are a few hints for finding your way around the source code.  This
doesn't make it less complex than it is, but it gets you started.

You might also want to read
[`:help development`](http://vimdoc.sourceforge.net/htmldoc/develop.html#development).


## Jumping around ##

First of all, use `:make tags` to generate a tags file, so that you can jump
around in the source code.

To jump to a function or variable definition, move the cursor on the name and
use the `CTRL-]` command.  Use `CTRL-T` or `CTRL-O` to jump back.

To jump to a file, move the cursor on its name and use the `gf` command.

Most code can be found in a file with an obvious name (incomplete list):

File name       | Description
--------------- | -----------
alloc.c		| memory management
arglist.c	| handling argument list
autocmd.c	| autocommands
blob.c		| blob data type
buffer.c	| manipulating buffers (loaded files)
bufwrite.c	| writing a buffer to file
change.c	| handling changes to text
cindent.c	| C and Lisp indentation
clientserver.c	| client server functionality
clipboard.c	| handling the clipboard
cmdexpand.c	| command-line completion
cmdhist.c	| command-line history
debugger.c	| vim script debugger
diff.c		| diff mode (vimdiff)
drawline.c	| drawing a window line
drawscreen.c	| drawing the windows
eval.c		| expression evaluation
evalbuffer.c	| buffer related built-in functions
evalfunc.c	| built-in functions
evalvars.c	| vim variables
evalwindow.c	| window related built-in functions
fileio.c	| reading and writing files
filepath.c	| dealing with file names and paths
findfile.c	| search for files in 'path'
fold.c		| folding
getchar.c	| getting characters and key mapping
help.c		| vim help related functions
highlight.c	| syntax highlighting
indent.c	| text indentation
insexpand.c	| Insert mode completion
locale.c	| locale/language handling
map.c		| mapping and abbreviations
mark.c		| marks
match.c		| highlight matching
float.c		| floating point functions
mbyte.c		| multi-byte character handling
memfile.c	| storing lines for buffers in a swapfile
memline.c	| storing lines for buffers in memory
menu.c		| menus
message.c	| (error) messages
mouse.c		| handling the mouse
ops.c		| handling operators ("d", "y", "p")
option.c	| options
optionstr.c	| handling string options
popupmenu.c	| popup menu
popupwin.c	| popup window
profiler.c	| vim script profiler
quickfix.c	| quickfix commands (":make", ":cn")
regexp.c	| pattern matching
register.c	| handling registers
scriptfile.c	| runtime directory handling and sourcing scripts
screen.c	| lower level screen functions
search.c	| pattern searching
session.c	| sessions and views
sign.c		| signs
spell.c		| spell checking core
spellfile.c	| spell file handling
spellsuggest.c	| spell correction suggestions
strings.c	| string manipulation functions
syntax.c	| syntax and other highlighting
tag.c		| tags
term.c		| terminal handling, termcap codes
testing.c	| testing: assert and test functions
textformat.c	| text formatting
textobject.c	| text objects
textprop.c	| text properties
time.c		| time and timer functions
typval.c	| vim script type/value functions
undo.c		| undo and redo
usercmd.c	| user defined commands
userfunc.c	| user defined functions
viminfo.c	| viminfo handling
window.c	| handling split windows


## Debugging ##

If you have a reasonable recent version of gdb, you can use the `:Termdebug`
command to debug Vim.  See  `:help :Termdebug`.

When something is time critical or stepping through code is a hassle, use the
channel logging to create a time-stamped log file.  Add lines to the code like
this:

	ch_log(NULL, "Value is now %02x", value);

After compiling and starting Vim, do:

	:call ch_logfile('debuglog', 'w')

And edit `debuglog` to see what happens.  The channel functions already have
`ch_log()` calls, thus you always see that in the log.


## Important Variables ##

The current mode is stored in `State`.  The values it can have are `NORMAL`,
`INSERT`, `CMDLINE`, and a few others.

The current window is `curwin`.  The current buffer is `curbuf`.  These point
to structures with the cursor position in the window, option values, the file
name, etc.  These are defined in
[`structs.h`](https://github.com/vim/vim/blob/master/src/structs.h).

All the global variables are declared in
[`globals.h`](https://github.com/vim/vim/blob/master/src/globals.h).


## The main loop ##

This is conveniently called `main_loop()`.  It updates a few things and then
calls `normal_cmd()` to process a command.  This returns when the command is
finished.

The basic idea is that Vim waits for the user to type a character and
processes it until another character is needed.  Thus there are several places
where Vim waits for a character to be typed.  The `vgetc()` function is used
for this.  It also handles mapping.

Updating the screen is mostly postponed until a command or a sequence of
commands has finished.  The work is done by `update_screen()`, which calls
`win_update()` for every window, which calls `win_line()` for every line.
See the start of
[`screen.c`](https://github.com/vim/vim/blob/master/src/screen.c)
for more explanations.


## Command-line mode ##

When typing a `:`, `normal_cmd()` will call `getcmdline()` to obtain a line
with an Ex command.  `getcmdline()` contains a loop that will handle each typed
character.  It returns when hitting `CR` or `Esc` or some other character that
ends the command line mode.


## Ex commands ##

Ex commands are handled by the function `do_cmdline()`.  It does the generic
parsing of the `:` command line and calls `do_one_cmd()` for each separate
command.  It also takes care of while loops.

`do_one_cmd()` parses the range and generic arguments and puts them in the
`exarg_t` and passes it to the function that handles the command.

The `:` commands are listed in `ex_cmds.h`.  The third entry of each item is
the name of the function that handles the command.  The last entry are the
flags that are used for the command.


## Normal mode commands ##

The Normal mode commands are handled by the `normal_cmd()` function.  It also
handles the optional count and an extra character for some commands.  These
are passed in a `cmdarg_t` to the function that handles the command.

There is a table `nv_cmds` in
[`normal.c`](https://github.com/vim/vim/blob/master/src/normal.c)
which lists the first character of every command.  The second entry of each
item is the name of the function that handles the command.


## Insert mode commands ##

When doing an `i` or `a` command, `normal_cmd()` will call the `edit()`
function. It contains a loop that waits for the next character and handles it.
It returns when leaving Insert mode.


## Options ##

There is a list with all option names in
[`option.c`](https://github.com/vim/vim/blob/master/src/option.c),
called `options[]`.


## The GUI ##

Most of the GUI code is implemented like it was a clever terminal.  Typing a
character, moving a scrollbar, clicking the mouse, etc. are all translated
into events which are written in the input buffer.  These are read by the
main code, just like reading from a terminal.  The code for this is scattered
through [`gui.c`](https://github.com/vim/vim/blob/master/src/gui.c).
For example, `gui_send_mouse_event()` for a mouse click and `gui_menu_cb()` for
a menu action.  Key hits are handled by the system-specific GUI code, which
calls `add_to_input_buf()` to send the key code.

Updating the GUI window is done by writing codes in the output buffer, just
like writing to a terminal.  When the buffer gets full or is flushed,
`gui_write()` will parse the codes and draw the appropriate items.  Finally the
system-specific GUI code will be called to do the work.


## Debugging the GUI ##

Remember to prevent that gvim forks and the debugger thinks Vim has exited,
add the `-f` argument.  In gdb: `run -f -g`.

When stepping through display updating code, the focus event is triggered
when going from the debugger to Vim and back.  To avoid this, recompile with
some code in `gui_focus_change()` disabled.


## Contributing ##

If you would like to help making Vim better, see the
[`CONTRIBUTING.md`](https://github.com/vim/vim/blob/master/CONTRIBUTING.md)
file.


This is `README.md` for version 9.1 of the Vim source code.
