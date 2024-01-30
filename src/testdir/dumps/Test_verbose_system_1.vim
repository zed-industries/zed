" Filter that removes the ever changing temp directory name from the screendump
" that shows the system() command executed.
" This should be on the first line, but if it isn't there ignore the error,
" the screendump will then show the problem.
1s+|t|m|p|/|.|.|.*| |+|t|m|p|/|x|x|x|x|x|x|x|/|1| |+e
