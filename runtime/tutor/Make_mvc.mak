#
# Makefile for converting the Vim tutorial on Windows.
#
# 21.11.23, Restorer, restorer@mail2k.ru


!IF [powershell -nologo -noprofile "exit $$psversiontable.psversion.major"] == 2
!ERROR The program "PowerShell" version 3.0 or higher is required to work
!ENDIF

# Common components
!INCLUDE Make_all.mak

# Correct the following line for the directory where iconv is installed.
# Please do not put the path in quotes.
ICONV_PATH = D:\Programs\GetText\bin

# In case some package like GnuWin32, UnixUtils, gettext
# or something similar is installed on the system.
# If the "iconv" program is installed on the system, but it is not registered
# in the %PATH% environment variable, then specify the full path to this file.
!IF EXIST ("iconv.exe")
ICONV = "iconv.exe"
!ELSEIF EXIST ("$(ICONV_PATH)\iconv.exe")
ICONV = "$(ICONV_PATH)\iconv.exe"
!ENDIF

RM = del /q
CP = copy /y
PS = PowerShell.exe

PSFLAGS = -NoLogo -NoProfile -Command

all : $(CONVERTED)

tutor.utf-8 : tutor
!IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-1 -t UTF-8 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28591)) ^| \
		1>nul New-Item -Force -ItemType file -Path . -Name $@
!ENDIF

tutor.bar : tutor.bar.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-1 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28591))
!ENDIF

tutor.ca.utf-8 : tutor.ca
!IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-1 -t UTF-8 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28591)) ^| \
		1>nul New-Item -Force -ItemType file -Path . -Name $@
!ENDIF

tutor.de.utf-8 : tutor.de
!IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-1 -t UTF-8 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28591)) ^| \
		1>nul New-Item -Force -ItemType file -Path . -Name $@
!ENDIF

tutor.el : tutor.el.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-7 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28597))
!ENDIF

tutor.el.cp737 : tutor.el.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP737 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(737))
!ENDIF

tutor.eo : tutor.eo.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-3 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28593))
!ENDIF

tutor.es : tutor.es.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-1 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28591))
!ENDIF

tutor.fr.utf-8 : tutor.fr
!IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-1 -t UTF-8 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28591)) ^| \
		1>nul New-Item -Force -ItemType file -Path . -Name $@
!ENDIF

tutor.hr : tutor.hr.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-2 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28592))
!ENDIF

tutor.hr.cp1250 : tutor.hr.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1250 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1250))
!ENDIF

tutor.hu : tutor.hu.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-2 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28592))
!ENDIF

tutor.hu.cp1250 : tutor.hu.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1250 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1250))
!ENDIF

tutor.it.utf-8 : tutor.it
!IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-1 -t UTF-8 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28591)) ^| \
		1>nul New-Item -Force -ItemType file -Path . -Name $@
!ENDIF

tutor.ja.sjis : tutor.ja.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP932 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(932))
!ENDIF

tutor.ja.euc : tutor.ja.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t EUC-JP $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(51932))
!ENDIF

tutor.ko.euc : tutor.ko.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t EUC-KR $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(51949))
!ENDIF

tutor.nl : tutor.nl.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-1 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28591))
!ENDIF

tutor.no.utf-8 : tutor.no
!IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-1 -t UTF-8 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28591)) ^| \
		1>nul New-Item -Force -ItemType file -Path . -Name $@
!ENDIF

# nb is an alias for no
tutor.nb : tutor.no
	$(CP) tutor.no tutor.nb

tutor.nb.utf-8 : tutor.no.utf-8
	$(CP) tutor.no.utf-8 tutor.nb.utf-8

tutor.ru : tutor.ru.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t KOI8-R $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(20866))
!ENDIF

tutor.ru.cp1251 : tutor.ru.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1251 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1251))
!ENDIF

tutor.sv.utf-8 : tutor.sv
!IF DEFINED (ICONV)
	$(ICONV) -f ISO-8859-1 -t UTF-8 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(28591)) ^| \
		1>nul New-Item -Force -ItemType file -Path . -Name $@
!ENDIF

tutor.tr.iso9 : tutor.tr.utf-8
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-9 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28599))
!ENDIF

tutor.zh.utf-8 : tutor.zh.big5
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(950)) ^| \
		1>nul New-Item -Force -ItemType file -Path . -Name $@

clean :
	@for %%G in ($(CONVERTED)) do (if exist .\%%G ($(RM) %%G))

# vim: set noet sw=8 ts=8 sts=0 wm=0 tw=0 ft=make:
