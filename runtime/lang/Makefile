# Author: Ada (Haowen) Yu <me@yuhaowen.com>

# Common components
include Make_all.mak

SED = LANG=C sed

.PHONY: all clean

all: $(CONVERTED)


# Convert menu_zh_cn.utf-8.vim to create menu_chinese_gb.936.vim.
menu_chinese_gb.936.vim: menu_zh_cn.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP936 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp936/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_zh_tw.utf-8.vim to create menu_chinese_taiwan.950.vim.
menu_chinese_taiwan.950.vim: menu_zh_tw.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP950 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp950/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_cs_cz.utf-8.vim to create menu_cs_cz.iso_8859-2.vim.
menu_cs_cz.iso_8859-2.vim: menu_cs_cz.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t ISO-8859-2 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding iso-8859-2/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			-e 's/\(" Menu Translations:.*\)(.*)/\1(ISO-8859-2)/' \
			> $@

# Convert menu_cs_cz.utf-8.vim to create menu_czech_czech_republic.1250.vim.
menu_czech_czech_republic.1250.vim: menu_cs_cz.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP1250 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp1250/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			-e 's/\(" Menu Translations:.*\)(.*)/\1(CP1250)/' \
			> $@

# Convert menu_cs_cz.utf-8.vim to create menu_czech_czech_republic.ascii.vim.
menu_czech_czech_republic.ascii.vim: menu_cs_cz.utf-8.vim
	rm -f $@
	$(SED) -e 's/Á/A/g' -e 's/á/a/g' -e 's/Č/C/g' -e 's/č/c/g' -e 's/Ď/D/g' \
		-e 's/ď/d/g' -e 's/É/E/g' -e 's/é/e/g' -e 's/Ě/E/g' -e 's/ě/e/g' \
		-e 's/Í/I/g' -e 's/í/i/g' -e 's/Ň/N/g' -e 's/ň/n/g' -e 's/Ó/O/g' \
		-e 's/ó/o/g' -e 's/Ř/R/g' -e 's/ř/r/g' -e 's/Š/S/g' -e 's/š/s/g' \
		-e 's/Ť/T/g' -e 's/ť/t/g' -e 's/Ú/U/g' -e 's/ú/u/g' -e 's/Ů/U/g' \
		-e 's/ů/u/g' -e 's/Ý/Y/g' -e 's/ý/y/g' -e 's/Ž/Z/g' -e 's/ž/z/g' \
		-e 's/scriptencoding utf-8/scriptencoding latin1/' \
		-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
		-e 's/\(" Menu Translations:.*\)(.*)/\1(ASCII - without diacritics)/' \
		$< > $@

# Convert menu_ja_jp.utf-8.vim to create menu_ja_jp.euc-jp.vim.
menu_ja_jp.euc-jp.vim: menu_ja_jp.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t EUC-JP $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding euc-jp/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			-e 's/\(" Menu Translations:.*\)(.*)/\1(EUC-JP)/' \
			> $@

# Convert menu_ja_jp.utf-8.vim to create menu_japanese_japan.932.vim.
menu_japanese_japan.932.vim: menu_ja_jp.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP932 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp932/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			-e 's/\(" Menu Translations:.*\)(.*)/\1(CP932)/' \
			> $@

# Convert menu_ko_kr.utf-8.vim to create menu_ko_kr.euckr.vim.
menu_ko_kr.euckr.vim: menu_ko_kr.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t EUC-KR $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding euc-kr/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_pl_pl.utf-8.vim to create menu_pl_pl.iso_8859-2.vim.
menu_pl_pl.iso_8859-2.vim: menu_pl_pl.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t ISO-8859-2 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding iso-8859-2/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_pl_pl.utf-8.vim to create menu_polish_poland.1250.vim.
menu_polish_poland.1250.vim: menu_pl_pl.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP1250 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp1250/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_ru_ru.utf-8.vim to create menu_ru_ru.koi8-r.vim.
menu_ru_ru.koi8-r.vim: menu_ru_ru.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t KOI8-R $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding koi8-r/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_ru_ru.utf-8.vim to create menu_ru_ru.cp1251.vim.
menu_ru_ru.cp1251.vim: menu_ru_ru.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP1251 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp1251/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_sl_si.utf-8.vim to create menu_sl_si.cp1250.vim.
menu_sl_si.cp1250.vim: menu_sl_si.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP1250 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp1250/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# for naming encodings, see `:h encoding-names`
# Convert menu_sl_si.utf-8.vim to create menu_sl_si.latin2.vim.
menu_sl_si.latin2.vim: menu_sl_si.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t ISO-8859-2 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding iso-8859-2/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_tr_tr.utf-8.vim to create menu_tr_tr.cp1254.vim.
menu_tr_tr.cp1254.vim: menu_tr_tr.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP1254 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp1254/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_tr_tr.utf-8.vim to create menu_tr_tr.iso_8859-9.vim.
menu_tr_tr.iso_8859-9.vim: menu_tr_tr.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t ISO-8859-9 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding iso-8859-9/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_sr_rs.utf-8.vim to create menu_sr_rs.iso_8859-5.vim.
menu_sr_rs.iso_8859-5.vim: menu_sr_rs.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t ISO-8859-5 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding iso-8859-5/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# for naming encodings, see `:h encoding-names`
# Convert menu_sr_rs.utf-8.vim to create menu_sr_rs.iso_8859-2.vim.
menu_sr_rs.iso_8859-2.vim: menu_sr_rs.utf-8.vim
	rm -f $@
	$(SED) -e 's/а/a/g' -e 's/б/b/g' -e 's/в/v/g' -e 's/г/g/g' -e 's/д/d/g' \
		-e 's/ђ/đ/g' -e 's/е/e/g' -e 's/ж/ž/g' -e 's/з/z/g' -e 's/и/i/g' \
		-e 's/ј/j/g' -e 's/к/k/g' -e 's/л/l/g' -e 's/љ/lj/g' -e 's/м/m/g' \
		-e 's/н/n/g' -e 's/њ/nj/g' -e 's/о/o/g' -e 's/п/p/g' -e 's/р/r/g' \
		-e 's/с/s/g' -e 's/т/t/g' -e 's/ћ/ć/g' -e 's/у/u/g' -e 's/ф/f/g' \
		-e 's/х/h/g' -e 's/ц/c/g' -e 's/ч/č/g' -e 's/џ/dž/g' -e 's/ш/š/g' \
		-e 's/А/A/g' -e 's/Б/B/g' -e 's/В/V/g' -e 's/Г/G/g' -e 's/Д/D/g' \
		-e 's/Ђ/Đ/g' -e 's/Е/E/g' -e 's/Ж/Ž/g' -e 's/З/Z/g' -e 's/И/I/g' \
		-e 's/Ј/J/g' -e 's/К/K/g' -e 's/Л/L/g' -e 's/Љ/Lj/g' -e 's/М/M/g' \
		-e 's/Н/N/g' -e 's/Њ/Nj/g' -e 's/О/O/g' -e 's/П/P/g' -e 's/Р/R/g' \
		-e 's/С/S/g' -e 's/Т/T/g' -e 's/Ћ/Ć/g' -e 's/У/U/g' -e 's/Ф/F/g' \
		-e 's/Х/H/g' -e 's/Ц/C/g' -e 's/Ч/Č/g' -e 's/Џ/Dž/g' -e 's/Ш/Š/g' \
		-e 's/scriptencoding utf-8/scriptencoding iso-8859-2/' \
		-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
		$< | iconv -f UTF-8 -t ISO-8859-2 \
		> $@

# Convert menu_sr_rs.utf-8.vim to create menu_sr_rs.ascii.vim.
menu_sr_rs.ascii.vim: menu_sr_rs.utf-8.vim
	rm -f $@
	$(SED) -e 's/а/a/g' -e 's/б/b/g' -e 's/в/v/g' -e 's/г/g/g' -e 's/д/d/g' \
		-e 's/ђ/dj/g' -e 's/е/e/g' -e 's/ж/z/g' -e 's/з/z/g' -e 's/и/i/g' \
		-e 's/ј/j/g' -e 's/к/k/g' -e 's/л/l/g' -e 's/љ/lj/g' -e 's/м/m/g' \
		-e 's/н/n/g' -e 's/њ/nj/g' -e 's/о/o/g' -e 's/п/p/g' -e 's/р/r/g' \
		-e 's/с/s/g' -e 's/т/t/g' -e 's/ћ/c/g' -e 's/у/u/g' -e 's/ф/f/g' \
		-e 's/х/h/g' -e 's/ц/c/g' -e 's/ч/c/g' -e 's/џ/dz/g' -e 's/ш/s/g' \
		-e 's/А/A/g' -e 's/Б/B/g' -e 's/В/V/g' -e 's/Г/G/g' -e 's/Д/D/g' \
		-e 's/Ђ/Đ/g' -e 's/Е/E/g' -e 's/Ж/Z/g' -e 's/З/Z/g' -e 's/И/I/g' \
		-e 's/Ј/J/g' -e 's/К/K/g' -e 's/Л/L/g' -e 's/Љ/Lj/g' -e 's/М/M/g' \
		-e 's/Н/N/g' -e 's/Њ/Nj/g' -e 's/О/O/g' -e 's/П/P/g' -e 's/Р/R/g' \
		-e 's/С/S/g' -e 's/Т/T/g' -e 's/Ћ/C/g' -e 's/У/U/g' -e 's/Ф/F/g' \
		-e 's/Х/H/g' -e 's/Ц/C/g' -e 's/Ч/C/g' -e 's/Џ/Dz/g' -e 's/Ш/S/g' \
		-e 's/scriptencoding utf-8/scriptencoding latin1/' \
		-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
		$< > $@
	
# Convert menu_uk_ua.utf-8.vim to create menu_uk_ua.cp1251.vim.
menu_uk_ua.cp1251.vim: menu_uk_ua.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t CP1251 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding cp1251/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_uk_ua.utf-8.vim to create menu_uk_ua.koi8-u.vim.
menu_uk_ua.koi8-u.vim: menu_uk_ua.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t KOI8-U $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding koi8-u/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_hu_hu.utf-8.vim to create menu_hu_hu.iso_8859-2.vim.
menu_hu_hu.iso_8859-2.vim: menu_hu_hu.utf-8.vim
	rm -f $@
	iconv -f UTF-8 -t ISO-8859-2 $< | \
		$(SED) -e 's/scriptencoding utf-8/scriptencoding iso-8859-2/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

# Convert menu_slovak_slovak_republic.1250.vim to create menu_sk_sk.iso_8859-2.vim.
menu_sk_sk.iso_8859-2.vim: menu_slovak_slovak_republic.1250.vim
	rm -f $@
	iconv -f CP1250 -t ISO-8859-2 $< | \
		$(SED) -e 's/scriptencoding cp1250/scriptencoding iso-8859-2/' \
			-e 's/" Original translations/" Generated from $<, DO NOT EDIT/' \
			> $@

clean:
	for G in $(CONVERTED); do if [ -f $$G ]; then rm -f $$G; fi; done
#	-rm -f $(CONVERTED)
