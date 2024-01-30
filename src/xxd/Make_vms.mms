# VMS MM[KS] makefile for XXD
# tested with MMK and MMS as well.
#
# Maintained by Zoltan Arpadffy <arpadffy@polarhome.com>
#
# Edit the lines in the Configuration section below to select.
#
# To build: use the following command line:
#
#	mms/descrip=Make_vms.mms
#	  or if you use mmk
#	mmk/descrip=Make_vms.mms
#
# To cleanup: mms/descrip=Make_vms.mms clean 
#
######################################################################
# Configuration section.
######################################################################
# Compiler selection.
# Comment out if you use the VAXC compiler
######################################################################
# DECC = YES

#####################################################################
# Uncomment if want a debug version. Resulting executable is DVIM.EXE
######################################################################
# DEBUG = YES

######################################################################
# End of configuration section.
#
# Please, do not change anything below without programming experience.
######################################################################

CC      = cc

.IFDEF DECC
CC_DEF  = $(CC)/decc
PREFIX  = /prefix=all
.ELSE
CC_DEF  = $(CC)
PREFIX  =
.ENDIF

LD_DEF  = link

.IFDEF DEBUG
TARGET  = dxxd.exe
CFLAGS  = /debug/noopt$(PREFIX)/cross_reference/include=[]
LDFLAGS = /debug
.ELSE
TARGET  = xxd.exe
CFLAGS  = /opt$(PREFIX)/include=[]
LDFLAGS =
.ENDIF

.SUFFIXES : .obj .c

SOURCES	= xxd.c
OBJ     = xxd.obj

.c.obj :
	$(CC_DEF) $(CFLAGS) $<

$(TARGET) : $(OBJ)
	$(LD_DEF) $(LDFLAGS) /exe=$(TARGET) $+

clean :
	-@ if "''F$SEARCH("*.obj")'" .NES. ""  then delete/noconfirm/nolog *.obj;*
	-@ if "''F$SEARCH("*.exe")'" .NES. ""  then delete/noconfirm/nolog *.exe;*

xxd.obj : xxd.c
