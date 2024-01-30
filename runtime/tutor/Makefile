# Makefile for the Vim tutor.
#
# The Japanese tutor exists in three encodings.  Use the UTF-8 version as the
# original and create the others with conversion.
#
# Similarly for Russian and Korean

# Common components
include Make_all.mak

.PHONY: all clean

all: $(CONVERTED)

tutor.utf-8: tutor
	iconv -f ISO-8859-1 -t UTF-8 $< >$@

tutor.bar: tutor.bar.utf-8
	iconv -f UTF-8 -t ISO-8859-1 $< >$@

tutor.ca.utf-8: tutor.ca
	iconv -f ISO-8859-1 -t UTF-8 $< >$@

tutor.eo: tutor.eo.utf-8
	iconv -f UTF-8 -t ISO-8859-3 $< >$@

tutor.de.utf-8: tutor.de
	iconv -f ISO-8859-1 -t UTF-8 $< >$@

tutor.el: tutor.el.utf-8
	iconv -f UTF-8 -t ISO-8859-7 $< >$@

tutor.el.cp737: tutor.el.utf-8
	iconv -f UTF-8 -t CP737 $< >$@

tutor.es: tutor.es.utf-8
	iconv -f UTF-8 -t ISO-8859-1 $< >$@

tutor.fr.utf-8: tutor.fr
	iconv -f ISO-8859-1 -t UTF-8 $< >$@

tutor.hu: tutor.hu.utf-8
	iconv -f UTF-8 -t ISO-8859-2 $< >$@

tutor.hu.cp1250: tutor.hu.utf-8
	iconv -f UTF-8 -t CP1250 $< >$@

tutor.it.utf-8: tutor.it
	iconv -f ISO-8859-1 -t UTF-8 $< >$@

tutor.hr: tutor.hr.utf-8
	iconv -f UTF-8 -t ISO-8859-2 $< >$@

tutor.hr.cp1250: tutor.hr.utf-8
	iconv -f UTF-8 -t CP1250 $< >$@

tutor.ja.sjis: tutor.ja.utf-8
	iconv -f UTF-8 -t CP932 $< >$@

tutor.ja.euc: tutor.ja.utf-8
	iconv -f UTF-8 -t EUC-JP $< >$@

tutor.ko.euc: tutor.ko.utf-8
	iconv -f UTF-8 -t EUC-KR $< >$@

tutor.nl: tutor.nl.utf-8
	iconv -f UTF-8 -t ISO-8859-1 $< >$@

tutor.no.utf-8: tutor.no
	iconv -f ISO-8859-1 -t UTF-8 $< >$@

# nb is an alias for no
tutor.nb: tutor.no
	cp $< $@

tutor.nb.utf-8: tutor.no.utf-8
	cp $< $@

tutor.ru: tutor.ru.utf-8
	iconv -f UTF-8 -t KOI8-R $< >$@

tutor.ru.cp1251: tutor.ru.utf-8
	iconv -f UTF-8 -t CP1251 $< >$@

tutor.tr.iso9: tutor.tr.utf-8
	iconv -f UTF-8 -t ISO-8859-9 $< >$@

tutor.sv.utf-8: tutor.sv
	iconv -f ISO-8859-1 -t UTF-8 $< >$@

tutor.zh.utf-8: tutor.zh.big5
	iconv -f BIG-5 -t UTF-8 $< >$@

clean:
	for G in $(CONVERTED); do if [ -f $$G ]; then rm -f $$G; fi; done
