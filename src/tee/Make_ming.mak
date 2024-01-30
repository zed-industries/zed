# A very (if not the most) simplistic Makefile for MS-Windows and OS/2

CC=gcc
CFLAGS=-O2 -fno-strength-reduce

ifneq (sh.exe, $(SHELL))
DEL = rm -f
else
DEL = del
endif

tee.exe: tee.o
	$(CC) $(CFLAGS) -s -o $@ $<

tee.o: tee.c
	$(CC) $(CFLAGS) -c $<

clean:
	- $(DEL) tee.o
	- $(DEL) tee.exe

