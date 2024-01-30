# Makefile for the Vim message translations for MSVC
# (based on make_ming.mak)
#
# Mike Williams, <mrw@eandem.co.uk>
# 06.01.24, Restorer, <restorer@mail2k.ru>
#
# Please read README_mvc.txt before using this file.
#

!IF [powershell.exe -nologo -noprofile "exit $$psversiontable.psversion.major"] == 2
!ERROR The program "PowerShell" version 3.0 or higher is required to work
!ENDIF

!IFNDEF LANGUAGE
! IF [powershell.exe -nologo -noprofile $$lng=(Get-UICulture).TwoLetterISOLanguageName;$$Env:LANGUAGE=$$lng;Set-Content -Path .\lng.tmp -Value "LANGUAGE=$$lng"]
#! IF [powershell.exe -nologo -noprofile -command $$Env:LANGUAGE=(Get-UICulture).TwoLetterISOLanguageName]
! ENDIF
# In order for the "install" and "cleanup-po" rule to work.
# The others work with just setting the environment variable.
# And to show in the message.
! INCLUDE lng.tmp
! IF [del /q .\lng.tmp]
! ENDIF
! MESSAGE
! MESSAGE The %LANGUAGE% environment variable is not set.
! MESSAGE This variable will be temporarily set to "$(LANGUAGE)" while "nmake.exe" is running.
! MESSAGE See README_mvc.txt for more information on the %LANGUAGE% environment variable.
! MESSAGE
!ELSE
! MESSAGE LANGUAGE is already set "$(LANGUAGE)"
!ENDIF

# get LANGUAGES, MOFILES, MOCONVERTED and others
!INCLUDE Make_all.mak

!IFNDEF VIMRUNTIME
VIMRUNTIME = ..\..\runtime
!ENDIF

PACKAGE = vim
# Correct the following line for the where executeable file vim is
# installed.  Please do not put the path in quotes.
VIM = ..\vim.exe

# Correct the following line for the directory where gettext et al is
# installed.  Please do not put the path in quotes.
GETTEXT_PATH = D:\Programs\GetText\bin

MSGFMT = "$(GETTEXT_PATH)\msgfmt.exe" -v
XGETTEXT = "$(GETTEXT_PATH)\xgettext.exe"
MSGMERGE = "$(GETTEXT_PATH)\msgmerge.exe"

# In case some package like GnuWin32, UnixUtils, gettext
# or something similar is installed on the system.
# If the "iconv" program is installed on the system, but it is not registered
# in the %PATH% environment variable, then specify the full path to this file.
!IF EXIST ("iconv.exe")
ICONV = "iconv.exe"
!ELSEIF EXIST ("$(GETTEXT_PATH)\iconv.exe")
ICONV="$(GETTEXT_PATH)\iconv.exe"
!ENDIF

# In case some package like GnuWin32, UnixUtils
# or something similar is installed on the system.
# If the "touch" program is installed on the system, but it is not registered
# in the %PATH% environment variable, then specify the full path to this file.
!IF EXIST ("touch.exe")
TOUCH_TARGET = touch.exe $@
!ELSE
TOUCH_TARGET = @if exist $@ ( copy /b $@+,, ) else ( type nul >$@ )
!ENDIF

MV = move /y
CP = copy /y
RM = del /q
MKD = mkdir
LS = dir
PS = PowerShell.exe

LSFLAGS = /b /on /l /s
PSFLAGS = -NoLogo -NoProfile -Command

INSTALLDIR = $(VIMRUNTIME)\lang\$(LANGUAGE)\LC_MESSAGES

.SUFFIXES:
.SUFFIXES: .po .mo .pot .ck

all: $(MOFILES) $(MOCONVERTED)

originals : $(MOFILES)

converted: $(MOCONVERTED)

.po.ck:
	"$(VIM)" -u NONE --noplugins -e -s -X --cmd "set enc=utf-8" -S check.vim \
		-c "if error == 0 | q | else | num 2 | cq | endif" $<
	$(TOUCH_TARGET)

check: $(CHECKFILES)

checkclean:
	$(RM) *.ck

nl.po:
	@( echo ^# >> nl.po )

# Norwegian/Bokmal: "nb" is an alias for "no".
nb.po: no.po
	$(CP) no.po nb.po

# Convert ja.po to create ja.sjis.po.
ja.sjis.po: ja.po
	@$(MAKE) -nologo -f Make_mvc.mak sjiscorr
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t CP932 $? | .\sjiscorr.exe > $@
!ELSEIF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP932 $? | .\sjiscorr.exe > $@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(932))
	type $@ | .\sjiscorr.exe > tmp.$@
	@$(MV) tmp.$@ $@
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(932)) \
		-replace \"`r`n\", \"`n\"; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(932))

sjiscorr: sjiscorr.c
	$(CC) sjiscorr.c

# Convert ja.po to create ja.euc-jp.po
ja.euc-jp.po: ja.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t EUC-JP -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t EUC-JP $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(20932))
! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(20932)) -replace \
		'charset=utf-8', 'charset=EUC-JP'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(20932))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(20932)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(20932))

# Convert cs.po to create cs.cp1250.po.
cs.cp1250.po: cs.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t CP1250 -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-2 -t CP1250 $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28592)), \
		[System.Text.Encoding]::GetEncoding(1250))
! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		'charset=iso-8859-2', 'charset=CP1250'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))

# Convert pl.po to create pl.cp1250.po.
pl.cp1250.po: pl.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t CP1250 -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-2 -t CP1250 $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28592)), \
		[System.Text.Encoding]::GetEncoding(1250))
! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		'charset=iso-8859-2', 'charset=CP1250'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))

# Convert pl.po to create pl.UTF-8.po.
pl.UTF-8.po: pl.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t UTF-8 -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-2 -t UTF-8 $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28592)))
! ENDIF
	$(PS) $(PSFLAGS) (Get-Content -Raw -Encoding UTF8 $@ \
		^| % {$$_-replace 'charset=iso-8859-2', 'charset=UTF-8'}) \
		^| 1>nul New-Item -Force -Path . -ItemType file -Name $@
!ENDIF
	$(PS) $(PSFLAGS) (Get-Content -Raw -Encoding UTF8 $@ \
		^| % {$$_-replace '# Original translations', \
		'# Generated from $?, DO NOT EDIT'}) \
		^| 1>nul New-Item -Force -Path . -ItemType file -Name $@

# Convert sk.po to create sk.cp1250.po.
sk.cp1250.po: sk.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t CP1250 -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-2 -t CP1250 $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28592)), \
		[System.Text.Encoding]::GetEncoding(1250))
! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		'charset=iso-8859-2', 'charset=CP1250'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))

# Convert zh_CN.UTF-8.po to create zh_CN.po.
zh_CN.po: zh_CN.UTF-8.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t GB2312 -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t GB2312 $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(936))

! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(936)) -replace \
		'charset=UTF-8', 'charset=GB2312'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(936))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(936)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(936))

# Convert zh_CN.UTF-8.po to create zh_CN.cp936.po.
# Set 'charset' to gbk to avoid that msfmt generates a warning.
# This used to convert from zh_CN.po, but that results in a conversion error.
zh_CN.cp936.po: zh_CN.UTF-8.po
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP936 $? > $@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(20936))

!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(20936)) \
		-replace 'charset=UTF-8', 'charset=GBK'\
		-replace '# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(20936))

# Convert zh_TW.UTF-8.po to create zh_TW.po
zh_TW.po: zh_TW.UTF-8.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t BIG5 -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t BIG5 $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(950))

! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(950)) -replace \
		'charset=UTF-8', 'charset=BIG5'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(950))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(950)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(950))

# Convert zh_TW.UTF-8.po to create zh_TW.po with backslash characters
# Requires doubling backslashes in the second byte.  Don't depend on big5corr,
# it should only be compiled when zh_TW.po is outdated.

#
#  06.11.23, added by Restorer:
#  For more details, see:
#  https://github.com/vim/vim/pull/3261
#  https://github.com/vim/vim/pull/3476
#  https://github.com/vim/vim/pull/12153
#  (read all comments)
#
#  I checked the workability on the list of backslash characters
#  specified in zh_TW.UTF-8.po. It works.
#  But it is better to have someone native speaker check it.
#

#zh_TW.po: zh_TW.UTF-8.po
#	@$(MAKE) -nologo -f Make_mvc.mak big5corr
#	-$(RM) $@
#!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
#	"$(GETTEXT_PATH)\msgconv.exe" -t BIG5 $? | .\big5corr.exe > $@
#!ELSEIF DEFINED (ICONV)
#	$(ICONV) -f UTF-8 -t BIG5 $? | .\big5corr.exe > $@
#!ELSE
#	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
#		[System.IO.File]::ReadAllText(\"$?\", \
#		[System.Text.Encoding]::GetEncoding(65001)), \
#		[System.Text.Encoding]::GetEncoding(950))
#	type $@ | .\big5corr.exe > tmp.$@
#	@$(MV) tmp.$@ $@
#!ENDIF
#	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
#		[System.Text.Encoding]::GetEncoding(950)) \
#		-replace \"`r`n\", \"`n\"; \
#		[System.IO.File]::WriteAllText(\"$@\", $$out, \
#		[System.Text.Encoding]::GetEncoding(950))

# see above in the zh_TW.po conversion section for backslashes.
#big5corr: big5corr.c
#	$(CC) big5corr.c

# Convert ko.UTF-8.po to create ko.po.
ko.po: ko.UTF-8.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t EUC-KR -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t EUC-KR $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(51949))

! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(51949)) -replace \
		'charset=UTF-8', 'charset=EUC-KR'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(51949))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(51949)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(51949))

# Convert ru.po to create ru.cp1251.po.
ru.cp1251.po: ru.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t CP1251 -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1251 $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1251))

! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1251)) -replace \
		'charset=UTF-8', 'charset=CP1251'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1251))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1251)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1251))

# Convert uk.po to create uk.cp1251.po.
uk.cp1251.po: uk.po
	-$(RM) $@
!IF EXIST ("$(GETTEXT_PATH)\msgconv.exe")
	"$(GETTEXT_PATH)\msgconv.exe" -t CP1251 -o $@ $?
!ELSE
! IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1251 $? > $@
! ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1251))

! ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1251)) -replace \
		'charset=UTF-8', 'charset=CP1251'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1251))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1251)) -replace \
		'# Original translations', \
		'# Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1251))

.po.mo:
	set OLD_PO_FILE_INPUT=yes
	$(MSGFMT) -o $@ $<

PO_INPUTLIST = \
	..\*.c \
	..\if_perl.xs \
	..\GvimExt\gvimext.cpp \
	..\errors.h \
	..\globals.h \
	..\if_py_both.h \
	..\vim.h \
	gvim.desktop.in \
	vim.desktop.in

files: $(PO_INPUTLIST)
	$(LS) $(LSFLAGS) $(PO_INPUTLIST) > .\files

first_time: files
	"$(VIM)" -u NONE --not-a-term -S tojavascript.vim $(LANGUAGE).po \
		$(PO_VIM_INPUTLIST)
	set OLD_PO_FILE_INPUT=yes
	set OLD_PO_FILE_OUTPUT=yes
	$(XGETTEXT) --default-domain=$(LANGUAGE) --add-comments $(XGETTEXT_KEYWORDS) \
		--files-from=.\files $(PO_VIM_JSLIST)
	"$(VIM)" -u NONE --not-a-term -S fixfilenames.vim $(LANGUAGE).po \
		$(PO_VIM_INPUTLIST)
	$(RM) *.js

$(PACKAGE).pot: files
	"$(VIM)" -u NONE --not-a-term -S tojavascript.vim $(PACKAGE).pot \
		$(PO_VIM_INPUTLIST)
	set OLD_PO_FILE_INPUT=yes
	set OLD_PO_FILE_OUTPUT=yes
	$(XGETTEXT) --default-domain=$(PACKAGE) --add-comments $(XGETTEXT_KEYWORDS) \
		--files-from=.\files $(PO_VIM_JSLIST)
	$(MV) $(PACKAGE).po $(PACKAGE).pot
	"$(VIM)" -u NONE --not-a-term -S fixfilenames.vim $(PACKAGE).pot \
		$(PO_VIM_INPUTLIST)
	$(RM) *.js

# Only original translations with default encoding should be updated.
# The files that are converted to a different encoding clearly state "DO NOT EDIT".
update-po: $(MOFILES:.mo=)

# Don't add a dependency here, we only want to update the .po files manually
$(LANGUAGES):
	@$(MAKE) -nologo -f Make_mvc.mak GETTEXT_PATH="$(GETTEXT_PATH)" $(PACKAGE).pot
	$(CP) $@.po $@.po.orig
	$(MV) $@.po $@.po.old
	$(MSGMERGE) $@.po.old $(PACKAGE).pot -o $@.po
	$(RM) $@.po.old

install: $(LANGUAGE).mo
	if not exist $(INSTALLDIR) $(MKD) $(INSTALLDIR)
	$(CP) $(LANGUAGE).mo $(INSTALLDIR)\$(PACKAGE).mo

install-all: all
	for %%l in ($(LANGUAGES)) do @if not exist $(VIMRUNTIME)\lang\%%l\LC_MESSAGES \
		$(MKD) $(VIMRUNTIME)\lang\%%l\LC_MESSAGES
	for %%l in ($(LANGUAGES)) do @$(CP) %%l.mo \
		$(VIMRUNTIME)\lang\%%l\LC_MESSAGES\$(PACKAGE).mo

cleanup-po: $(LANGUAGE).po
	"$(VIM)" -u NONE -e -X -S cleanup.vim -c wq $(LANGUAGE).po

cleanup-po-all: $(POFILES)
	!"$(VIM)" -u NONE -e -X -S cleanup.vim -c wq $**

clean: checkclean
	$(RM) *.mo
	$(RM) *.pot
	$(RM) *.orig
	$(RM) files
	$(RM) sjiscorr.obj sjiscorr.exe
#	$(RM) big5corr.obj big5corr.exe

# vim: set noet sw=8 ts=8 sts=0 wm=0 tw=0 ft=make:
