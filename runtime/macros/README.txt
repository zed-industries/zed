The macros in the maze, hanoi and urm directories can be used to test Vim for
vi compatibility.  They have been written for vi to show its unlimited
possibilities.	The life macros can be used for performance comparisons.

hanoi	Macros that solve the tower of hanoi problem.
life	Macros that run Conway's game of life.
maze	Macros that solve a maze (amazing!).
urm	Macros that simulate a simple computer: "Universal Register Machine"



The other files contain some handy utilities.  They also serve as examples for
how to use Vi and Vim functionality.

less.sh + less.vim	make Vim work like less (or more)



The following have been moved to an optional package.  Add the command to your
vimrc file to use the package:

packadd! dvorak		" Dvorak keyboard support; adds mappings

packadd! editexisting	" when editing a file that is already edited with
			" another Vim instance, go to that Vim instance

packadd! justify	" justifying text.

packadd! matchit	" makes the % command work better

packadd! shellmenu	" menus for editing shell scripts in the GUI version

packadd! swapmouse	" swap left and right mouse buttons
