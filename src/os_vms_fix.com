$!
$! OS_VMS_FIX.COM
$! Copyright (C) 2000, Stephen P. Wall
$!
$! Filter files for "#if" line continuations using a '\' and convert
$! them to use comments for the continuation.  Necessary for VAXC - it
$! doesn't understand the '\'.
$!
$! Yes, this is honkin' ugly code, but I deliberately avoided
$!     if ...
$!     then
$!	   ....
$!     endif
$! and call/subroutine/endsubroutine constructs, because I can still
$! remember when DCL didn't have them, and I wanted this to be as
$! portable as possible, so...  If you want to structure it nicer for
$! your own use, please feel free to do so.  However, please only
$! distribute it in its original form.
$!
$! I wrote it in DCL for portability and ease of use - a C version
$! would definitely run faster, but then I'd have to deal with compiler
$! differences, and users would have to deal with configuring and
$! building it.  With DCL, it runs out-of-the-box.
$!
$! Note that if you use this from a VMS system to modify files on a
$! mounted network drive, f$search() may return only the first matching
$! file when it tries to resolve wildcards.  I have been unable to find
$! a way around this.  Either copy the files to a local disk, or specify
$! each file individually (Keep in mind if you do this that VMS limits
$! you to eight parameters, so you'll only be able to filter eight files
$! at a time).
$!
$! Ideas...
$! - Use 'search filespec "#","if","\"/mat=and' to quickly eliminate
$!   files that definitely don't need filtering.  This should speed
$!   things up considerable.  Reading and writing every line from every
$!   file takes quite a bit of time...
$! - Error handling isn't great.  Come up with something better....
$!
$! E-mail addresses:
$! Steve Wall		hitched97@velnet.com
$! Zoltan Arpadffy      arpadffy@polarhome.com
$! John W. Hamill       jhamill3@ford.com
$!
$! Modification History:
$! 13Jul00 SWall	Initial Version
$! 14Jul00 ZArpadffy    Display usage
$! 06Mar01 JHamill      Ctrl-M problem fix
$!
$! If no parameters, or "-h" for a parameter, print usage and exit
$
$ all = "''p1'''p2'''p3'''p4'''p5'''p6'''p7'''p8'"
$ if (all .nes. "") .and. (p1 .nes. "-h") .and. (p1 .nes. "-H") then goto startup
$
$ write sys$output "OS_VMS_FIX - DECC->VAXC pre-processor directive convert script"
$ write sys$output "Usage: @OS_VMS_FIX <filename_1> <filename_2> <...>"
$ write sys$output "       @OS_VMS_FIX <filename with wildcard> <...>"
$ write sys$output ""
$ write sys$output "Example: @OS_VMS_FIX *.c *.h [.proto]*.pro"
$ write sys$output "Please note, you can define up to 8 parameters."
$ write sys$output ""
$ exit
$
$! Create an FDL file to convert VFC format files to Stream_LF.
$! VMS OPEN/WRITE command creates VFC files.  When VFC files are read
$! out under unix, they appear to have binary data embedded in them.
$! To be friendly, we'll convert them to Stream_LF, which reads just
$! file on unix.
$
$startup:
$ on control_y then goto stopfdl
$ open/write fdl []convert.fdl
$ write fdl "SYSTEM"
$ write fdl " SOURCE VAX/VMS"
$ write fdl "FILE"
$ write fdl " ORGANIZATION SEQUENTIAL"
$ write fdl "RECORD"
$ write fdl " BLOCK_SPAN YES"
$ write fdl " CARRIAGE_CONTROL CARRIAGE_RETURN"
$ write fdl " FORMAT STREAM"
$ write fdl " SIZE 0"
$ close fdl
$ on control_y then goto endparamloop
$
$! Some symbols for use later on...
$
$ spc = ""
$ spc[0,8] = 32
$ tab = ""
$ tab[0,8] = 9
$
$! Scan all positional arguments, do wildcard expansion, and call the
$! filter routine on each resulting filename.
$
$ cnt = 0
$paramloop:
$ cnt = cnt + 1
$
$! VMS only allows command line parameters P1 - P8, so stop after
$! processing 8 arguments.
$
$ if cnt .eq. 9 then goto endparamloop
$
$! Skip any empty parameter.
$
$ if P'cnt' .eqs. "" then goto paramloop
$
$! Got a parameter - do wildcard expansion.
$
$ arg = f$parse(P'cnt')
$ write sys$output "Parsing ''arg'..."
$ last = ""
$fileloop:
$ file = f$search(arg, 1)
$
$! f$search() returns "" after the last of multiple matches.
$
$ if file .eqs. "" then goto endfileloop
$
$! Strip the version number.
$
$ file = f$parse(file,,,"DEVICE") + f$parse(file,,,"DIRECTORY") + -
         f$parse(file,,,"NAME") + f$parse(file,,,"TYPE")
$
$! f$search() returns the same filename over and over if there are no
$! wildcards in it.
$
$ if file .eqs. last then goto endfileloop
$ last = file
$
$! Got a valid file - filter it.
$
$ gosub filter
$
$! Reset our error handling.
$
$ on control_y then goto endparamloop
$
$! See if there's another matching filename.
$
$ goto fileloop
$endfileloop:
$
$! Check for another parameter.
$
$ goto paramloop
$endparamloop:
$
$! Finished - delete the FDL file.
$
$ delete []convert.fdl;
$
$! So long, and thanks for all the fish...
$
$ exit
$
$
$! User aborted with Control-Y during creation of FDL file.
$! Close the file, delete it, and exit with an error status.
$
$stopfdl:
$ close fdl
$ delete []convert.fdl;
$ exit %X10000000
$
$
$! Filter a file.
$
$filter:
$ write sys$output "Filtering ''file'..."
$
$! Get a temporary filename from the subroutine parameter.
$
$ tmp = f$parse(file,,,"DEVICE") + f$parse(file,,,"DIRECTORY") + -
        "tmp_" + f$parse(file,,,"NAME") + f$parse(file,,,"TYPE")
$ on control_y then goto aborted
$ open /read input 'file'
$ open /write output 'tmp'
$ changed = 0
$readloop:
$ read/end_of_file=endreadloop/error=readlooperror input line
$
$! Get the first 3 non-blank character on the line.
$
$ start = f$extract(0,3,f$edit(line,"COLLAPSE,LOWERCASE"))
$
$! If the line doesn't start with some form of "#if", just write it to
$! the temp file.
$
$ if start .nes. "#if" then goto writeit
$chkbkslsh:
$
$! See if the line ends in a backslash.  If not, write it to the temp file.
$
$ if f$extract(f$length(line)-1,1,line) .nes. "\" then goto writeit
$
$! Ok, got a line that needs to be modified.  Mark this file as changed,
$! then replace the backslash at the end with the beginning of a comment
$! (/*), and write it to the temp file.
$
$ changed = 1
$ line = f$extract(0,f$length(line)-1,line) + "/*"
$ write/symbol output line
$
$! Get another line from the input.
$
$ read/end_of_file=endreadloop/error=readlooperror input line
$
$! Grab all the blank space from the beginning of the line.
$
$ spaces = ""
$spaceloop:
$ if (f$extract(0,1,line) .nes. spc) .and. (f$extract(0,1,line) .nes. tab) -
        then goto endspaceloop
$ spaces = spaces + f$extract(0,1,line)
$ line = f$extract(1,f$length(line)-1,line)
$ goto spaceloop
$endspaceloop:
$
$! Stick an end-comment (*/) after the leading blanks, then go back and
$! check for a trailing backslash again, to catch code that continues
$! across multiple lines.
$
$ line = spaces + "*/ " + line
$ goto chkbkslsh
$
$! Write the current line, (will either be an untouched line, or the
$! last line of a continuation) to the temp file, and go back to look
$! for more input.
$!
$writeit:
$ write/symbol output line
$ goto readloop
$
$! Hit EOF.  Close the input & output, and if the file was marked as
$! changed, convert it from VMS VFC format, to the more common Stream_LF
$! format, so it doesn't show up full of garbage if someone tries to
$! edit it on another OS.
$!
$endreadloop:
$ close input
$ close output
$ if changed .eq. 0 then goto nocopy
$ convert 'tmp' 'file' /fdl=[]convert.fdl
$nocopy:
$ delete 'tmp';
$
$! Exit this subroutine.
$
$ goto endfunc
$
$! Got a read error.  Say so, and trash the temp file.
$
$readlooperror:
$ write sys$error "Error processing file ''file'"
$ goto errorend
$
$! Got an interrupt.  Say so, and trash the temp file.
$
$aborted:
$ write sys$error "Aborted while processing file ''file'"
$
$! Common code for read errors and interrupts.
$
$errorend:
$ close input
$ close output
$ delete 'tmp';
$ return %X10000000
$
$! End of filter subroutine.
$
$endfunc:
$ return
$
$! EOF
