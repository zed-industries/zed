# The most simplistic Makefile, for MinGW and Cygwin gcc on MS-DOS

ifndef USEDLL
USEDLL = no
endif

ifeq (yes, $(USEDLL))
DEFINES =
LIBS    = -lc
else
DEFINES =
LIBS    =
endif

CC = gcc
CFLAGS = -O2 -Wall -DWIN32 $(DEFINES)

ifneq (sh.exe, $(SHELL))
DEL = rm
else
DEL = del
endif

xxd.exe: xxd.c
	$(CC) $(CFLAGS) -s -o xxd.exe xxd.c $(LIBS)

clean:
	-$(DEL) xxd.exe
