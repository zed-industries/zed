#
# Makefile for converted the Vim menu files on Windows
#
# 08.11.23, Restorer, <restorer@mail2k.ru>

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
PS = PowerShell.exe

PSFLAGS = -NoLogo -NoProfile -Command


all : $(CONVERTED)

# Convert menu_zh_cn.utf-8.vim to create menu_chinese_gb.936.vim.
menu_chinese_gb.936.vim : menu_zh_cn.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP936 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(936))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(936)) -replace \
		'scriptencoding utf-8', 'scriptencoding cp936' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(936))

# Convert menu_zh_tw.utf-8.vim to create menu_chinese_taiwan.950.vim.
menu_chinese_taiwan.950.vim : menu_zh_tw.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP950 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(950))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(950)) -replace \
		'scriptencoding utf-8', 'scriptencoding cp950' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(950))

# Convert menu_cs_cz.utf-8.vim to create menu_cs_cz.iso_8859-2.vim.
menu_cs_cz.iso_8859-2.vim : menu_cs_cz.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-2 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28592))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(28592)) -replace \
		' Czech \(UTF-8\)', ' Czech (ISO-8859-2)' -replace \
		\"scriptencoding utf-8\", \"scriptencoding iso-8859-2\" -replace \
		\" Original translations\", \" Generated from $?, DO NOT EDIT\"; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(28592))

# Convert menu_cs_cz.utf-8.vim to create menu_czech_czech_republic.1250.vim.
menu_czech_czech_republic.1250.vim : menu_cs_cz.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1250 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1250))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		' Czech \(UTF-8\)', ' Czech (CP1250)' -replace \
		\"scriptencoding utf-8\", \"scriptencoding cp1250\" -replace \
		\" Original translations\", \" Generated from $?, DO NOT EDIT\"; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))

# Convert menu_cs_cz.utf-8.vim to create menu_czech_czech_republic.ascii.vim.
menu_czech_czech_republic.ascii.vim : menu_cs_cz.utf-8.vim
	-$(RM) $@
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)) -replace \
		'scriptencoding utf-8', 'scriptencoding latin1' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT' -replace \
		'Czech \(UTF-8\)', 'Czech (ASCII - without diacritics)' -creplace \
		[char]193, 'A' -creplace [char]225, 'a' -creplace [char]268, 'C' -creplace \
		[char]269, 'c' -creplace [char]270, 'D' -creplace [char]271, 'd' -creplace \
		[char]201, 'E' -creplace [char]233, 'e' -creplace [char]282, 'E' -creplace \
		[char]283, 'e' -creplace [char]205, 'I' -creplace [char]237, 'i' -creplace \
		[char]327, 'N' -creplace [char]328, 'n' -creplace [char]211, 'O' -creplace \
		[char]243, 'o' -creplace [char]344, 'R' -creplace [char]345, 'r' -creplace \
		[char]352, 'S' -creplace [char]353, 's' -creplace [char]356, 'T' -creplace \
		[char]357, 't' -creplace [char]218, 'U' -creplace [char]250, 'u' -creplace \
		[char]366, 'U' -creplace [char]367, 'u' -creplace [char]221, 'Y' -creplace \
		[char]253, 'y' -creplace [char]381, 'Z' -creplace [char]382, 'z' ^| \
		1>nul New-Item -Force -Path . -ItemType file -Name $@

# Convert menu_hu_hu.utf-8.vim to create menu_hu_hu.iso_8859-2.vim.
menu_hu_hu.iso_8859-2.vim : menu_hu_hu.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-2 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28592))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(28592)) -replace \
		'scriptencoding utf-8', 'scriptencoding iso-8859-2' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(28592))

# Convert menu_ja_jp.utf-8.vim to create menu_ja_jp.euc-jp.vim.
menu_ja_jp.euc-jp.vim : menu_ja_jp.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t EUC-JP $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(51932))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(51932)) -replace \
		'Japanese \(UTF-8\)', 'Japanese (EUC-JP)' -replace \
		'scriptencoding utf-8', 'scriptencoding euc-jp' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(51932))

# Convert menu_ja_jp.utf-8.vim to create menu_japanese_japan.932.vim.
menu_japanese_japan.932.vim : menu_ja_jp.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP932 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(932))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(932)) -replace \
		'Japanese \(UTF-8\)', 'Japanese (CP932)' -replace \
		'scriptencoding utf-8', 'scriptencoding cp932' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(932))

# Convert menu_ko_kr.utf-8.vim to create menu_ko_kr.euckr.vim.
menu_ko_kr.euckr.vim : menu_ko_kr.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t EUC-KR $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(51949))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(51949)) -replace \
		'scriptencoding utf-8', 'scriptencoding euc-kr' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(51949))

# Convert menu_pl_pl.utf-8.vim to create menu_pl_pl.iso_8859-2.vim.
menu_pl_pl.iso_8859-2.vim : menu_pl_pl.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-2 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28592))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(28592)) -replace \
		'scriptencoding utf-8', 'scriptencoding iso-8859-2' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(28592))

# Convert menu_pl_pl.utf-8.vim to create menu_polish_poland.1250.vim.
menu_polish_poland.1250.vim : menu_pl_pl.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1250 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1250))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		'scriptencoding utf-8', 'scriptencoding cp1250' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))

# Convert menu_ru_ru.utf-8.vim to create menu_ru_ru.cp1251.vim.
menu_ru_ru.cp1251.vim : menu_ru_ru.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1251 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1251))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1251)) -replace \
		'scriptencoding utf-8', 'scriptencoding cp1251' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1251))

# Convert menu_ru_ru.utf-8.vim to create menu_ru_ru.koi8-r.vim.
menu_ru_ru.koi8-r.vim : menu_ru_ru.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t KOI8-R $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(20866))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(20866)) -replace \
		'scriptencoding utf-8', 'scriptencoding koi8-r' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(20866))

# Convert menu_slovak_slovak_republic.1250.vim to create menu_sk_sk.iso_8859-2.vim.
menu_sk_sk.iso_8859-2.vim : menu_slovak_slovak_republic.1250.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f CP1250 -t ISO-8859-2 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(1250)), \
		[System.Text.Encoding]::GetEncoding(28592))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(28592)) -replace \
		'scriptencoding cp1250', 'scriptencoding iso-8859-2' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(28592))

# Convert menu_sl_si.utf-8.vim to create menu_sl_si.cp1250.vim.
menu_sl_si.cp1250.vim : menu_sl_si.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1250 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1250))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1250)) -replace \
		'scriptencoding utf-8', 'scriptencoding cp1250' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1250))

# Convert menu_sl_si.utf-8.vim to create menu_sl_si.latin2.vim.
menu_sl_si.latin2.vim : menu_sl_si.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-2 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28592))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(28592)) -replace \
		'scriptencoding utf-8', 'scriptencoding iso-8859-2' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(28592))

# Convert menu_sr_rs.utf-8.vim to create menu_sr_rs.ascii.vim.
menu_sr_rs.ascii.vim : menu_sr_rs.utf-8.vim
	-$(RM) $@
	$(PS) $(PSFLAGS) [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)) -replace \
		'scriptencoding utf-8', 'scriptencoding latin1' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT' -creplace \
		[char]1072, 'a' -creplace [char]1073, 'b' -creplace [char]1074, 'v' -creplace \
		[char]1075, 'g' -creplace [char]1076, 'd' -creplace [char]1106, 'dj' -creplace \
		[char]1077, 'e' -creplace [char]1078, 'z' -creplace [char]1079, 'z' -creplace \
		[char]1080, 'i' -creplace [char]1112, 'j' -creplace [char]1082, 'k' -creplace \
		[char]1083, 'l' -creplace [char]1113, 'lj' -creplace [char]1084, 'm' -creplace \
		[char]1085, 'n' -creplace [char]1114, 'nj' -creplace [char]1086, 'o' -creplace \
		[char]1087, 'p' -creplace [char]1088, 'r' -creplace [char]1089, 's' -creplace \
		[char]1090, 't' -creplace [char]1115, 'c' -creplace [char]1091, 'u' -creplace \
		[char]1092, 'f' -creplace [char]1093, 'h' -creplace [char]1094, 'c' -creplace \
		[char]1095, 'c' -creplace [char]1119, 'dz' -creplace [char]1096, 's' -creplace \
		[char]1040, 'A' -creplace [char]1041, 'B' -creplace [char]1042, 'V' -creplace \
		[char]1043, 'G' -creplace [char]1044, 'D' -creplace [char]1026, 'Đ' -creplace \
		[char]1045, 'E' -creplace [char]1046, 'Z' -creplace [char]1047, 'Z' -creplace \
		[char]1048, 'I' -creplace [char]1032, 'J' -creplace [char]1050, 'K' -creplace \
		[char]1051, 'L' -creplace [char]1033, 'Lj' -creplace [char]1052, 'M' -creplace \
		[char]1053, 'N' -creplace [char]1034, 'Nj' -creplace [char]1054, 'O' -creplace \
		[char]1055, 'P' -creplace [char]1056, 'R' -creplace [char]1057, 'S' -creplace \
		[char]1058, 'T' -creplace [char]1035, 'C' -creplace [char]1059, 'U' -creplace \
		[char]1060, 'F' -creplace [char]1061, 'H' -creplace [char]1062, 'C' -creplace \
		[char]1063, 'C' -creplace [char]1039, 'Dz' -creplace [char]1064, 'S' ^| \
		1>nul New-Item -Force -Path . -ItemType file -Name $@

# Convert menu_sr_rs.utf-8.vim to create menu_sr_rs.iso_8859-2.vim.
menu_sr_rs.iso_8859-2.vim : menu_sr_rs.utf-8.vim
	-$(RM) $@
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)) -creplace \
		[char]1072, 'a' -creplace [char]1073, 'b' -creplace [char]1074, 'v' -creplace \
		[char]1075, 'g' -creplace [char]1076, 'd' -creplace [char]1106, [char]273 -creplace \
		[char]1077, 'e' -creplace [char]1078, [char]382 -creplace [char]1079, 'z' -creplace \
		[char]1080, 'i' -creplace [char]1112, 'j' -creplace [char]1082, 'k' -creplace \
		[char]1083, 'l' -creplace [char]1113, 'lj' -creplace [char]1084, 'm' -creplace \
		[char]1085, 'n' -creplace [char]1114, 'nj' -creplace [char]1086, 'o' -creplace \
		[char]1087, 'p' -creplace [char]1088, 'r' -creplace [char]1089, 's' -creplace \
		[char]1090, 't' -creplace [char]1115, [char]263 -creplace [char]1091, 'u' -creplace \
		[char]1092, 'f' -creplace [char]1093, 'h' -creplace [char]1094, 'c' -creplace \
		[char]1095, [char]269 -creplace [char]1119, 'dz' -creplace [char]1096, [char]353 -creplace \
		[char]1040, 'A' -creplace [char]1041, 'B' -creplace [char]1042, 'V' -creplace \
		[char]1043, 'G' -creplace [char]1044, 'D' -creplace [char]1026, 'Đ' -creplace \
		[char]1045, 'E' -creplace [char]1046, [char]381 -creplace [char]1047, 'Z' -creplace \
		[char]1048, 'I' -creplace [char]1032, 'J' -creplace [char]1050, 'K' -creplace \
		[char]1051, 'L' -creplace [char]1033, 'Lj'-creplace [char]1052, 'M' -creplace \
		[char]1053, 'N' -creplace [char]1034, 'Nj' -creplace [char]1054, 'O' -creplace \
		[char]1055, 'P' -creplace [char]1056, 'R' -creplace [char]1057, 'S' -creplace \
		[char]1058, 'T' -creplace [char]1035, [char]262 -creplace [char]1059, 'U' -creplace \
		[char]1060, 'F' -creplace [char]1061, 'H' -creplace [char]1062, 'C' -creplace \
		[char]1063, [char]268 -creplace [char]1039, 'Dz' -creplace [char]1064, [char]352 -replace \
		'scriptencoding utf-8', 'scriptencoding iso-8859-2' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, [System.Text.Encoding]::GetEncoding(28592))

# Convert menu_sr_rs.utf-8.vim to create menu_sr_rs.iso_8859-5.vim.
menu_sr_rs.iso_8859-5.vim : menu_sr_rs.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-5 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28595))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(28595)) -replace \
		'scriptencoding utf-8', 'scriptencoding iso-8859-5' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(28595))

# Convert menu_tr_tr.utf-8.vim to create menu_tr_tr.cp1254.vim.
menu_tr_tr.cp1254.vim : menu_tr_tr.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1254 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1254))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1254)) -replace \
		'scriptencoding utf-8', 'scriptencoding cp1254' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1254))

# Convert menu_tr_tr.utf-8.vim to create menu_tr_tr.iso_8859-9.vim.
menu_tr_tr.iso_8859-9.vim : menu_tr_tr.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t ISO-8859-9 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(28599))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(28599)) -replace \
		'scriptencoding utf-8', 'scriptencoding iso-8859-9' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(28599))

# Convert menu_uk_ua.utf-8.vim to create menu_uk_ua.cp1251.vim.
menu_uk_ua.cp1251.vim : menu_uk_ua.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t CP1251 $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(1251))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(1251)) -replace \
		'scriptencoding utf-8', 'scriptencoding cp1251' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(1251))

# Convert menu_uk_ua.utf-8.vim to create menu_uk_ua.koi8-u.vim.
menu_uk_ua.koi8-u.vim : menu_uk_ua.utf-8.vim
	-$(RM) $@
!IF DEFINED (ICONV)
	$(ICONV) -f UTF-8 -t KOI8-U $? >$@
!ELSE
	$(PS) $(PSFLAGS) [System.IO.File]::WriteAllText(\"$@\", \
		[System.IO.File]::ReadAllText(\"$?\", \
		[System.Text.Encoding]::GetEncoding(65001)), \
		[System.Text.Encoding]::GetEncoding(21866))
!ENDIF
	$(PS) $(PSFLAGS) $$out = [System.IO.File]::ReadAllText(\"$@\", \
		[System.Text.Encoding]::GetEncoding(21866)) -replace \
		'scriptencoding utf-8', 'scriptencoding koi8-u' -replace \
		' Original translations', ' Generated from $?, DO NOT EDIT'; \
		[System.IO.File]::WriteAllText(\"$@\", $$out, \
		[System.Text.Encoding]::GetEncoding(21866))

clean :
	@for %%G in ($(CONVERTED)) do (if exist .\%%G ($(RM) %%G))

# vim: set noet sw=8 ts=8 sts=0 wm=0 tw=0 ft=make:
