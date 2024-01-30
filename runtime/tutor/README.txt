Tutor is a "hands on" tutorial for new users of the Vim editor.

Most new users can get through it in less than one hour. The result
is that you can do a simple editing task using the Vim editor.

Tutor is a file that contains the tutorial lessons. You can simply
execute "vim tutor" and then follow the instructions in the lessons.
The lessons tell you to modify the file, so DON'T DO THIS ON YOUR
ORIGINAL COPY.

On Unix you can also use the "vimtutor" program.  It will make a
scratch copy of the tutor first.

I have considered adding more advanced lessons but have not found the
time. Please let me know how you like it and send any improvements you
make.

Bob Ware, Colorado School of Mines, Golden, Co 80401, USA
(303) 273-3987
bware@mines.colorado.edu bware@slate.mines.colorado.edu bware@mines.bitnet


Translation
-----------

The tutor.xx and tutor.xx.utf-8 files are translated files (where xx is the
language code).  The encoding of tutor.xx might be latin1 or other traditional
encoding.  If you don't need a translation with such traditional encoding,
you just need to prepare the tutor.xx.utf-8 file.
If you need another encoding, you can also prepare a file named tutor.xx.enc
(replace enc with the actual encoding name).  You might also need to adjust the
tutor.vim file.
The "make" command can be used for creating tutor.xx from tutor.xx.utf-8.
See the Makefile for detail.  (For some languages, tutor.xx.utf-8 is created
from tutor.xx for historical reasons.)

[This file was modified for Vim by Bram Moolenaar et al.]
