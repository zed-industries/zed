# The most simplistic Makefile for Win32 using Microsoft Visual C++
# (NT and Windows 95)

SUBSYSTEM = console
!if "$(SUBSYSTEM_VER)" != ""
SUBSYSTEM = $(SUBSYSTEM),$(SUBSYSTEM_VER)
!endif

xxd: xxd.exe

xxd.exe: xxd.c
	cl /nologo -DWIN32 xxd.c -link -subsystem:$(SUBSYSTEM)

# This was for an older compiler
#    cl /nologo -DWIN32 xxd.c /link setargv.obj

clean:
	- if exist xxd.obj del xxd.obj
	- if exist xxd.exe del xxd.exe
