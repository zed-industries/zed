#
# Common po Makefile, defines the list of languages.
#

LANGUAGES = \
		af \
		ca \
		cs \
		cs.cp1250 \
		da \
		de \
		en_GB \
		eo \
		es \
		fi \
		fr \
		ga \
		hu \
		it \
		ja \
		ja.euc-jp \
		ja.sjis \
		ko \
		ko.UTF-8 \
		lv \
		nb \
		nl \
		no \
		pl \
		pl.UTF-8 \
		pl.cp1250 \
		pt_BR \
		ru \
		ru.cp1251 \
		sk \
		sk.cp1250 \
		sr \
		sv \
		tr \
		uk \
		uk.cp1251 \
		vi \
		zh_CN \
		zh_CN.UTF-8 \
		zh_CN.cp936 \
		zh_TW \
		zh_TW.UTF-8 \


POFILES = \
		af.po \
		ca.po \
		cs.po \
		cs.cp1250.po \
		da.po \
		de.po \
		en_GB.po \
		eo.po \
		es.po \
		fi.po \
		fr.po \
		ga.po \
		hu.po \
		it.po \
		ja.po \
		ja.euc-jp.po \
		ja.sjis.po \
		ko.po \
		ko.UTF-8.po \
		lv.po \
		nb.po \
		nl.po \
		no.po \
		pl.po \
		pl.UTF-8.po \
		pl.cp1250.po \
		pt_BR.po \
		ru.po \
		ru.cp1251.po \
		sk.po \
		sk.cp1250.po \
		sr.po \
		sv.po \
		tr.po \
		uk.po \
		uk.cp1251.po \
		vi.po \
		zh_CN.po \
		zh_CN.UTF-8.po \
		zh_CN.cp936.po \
		zh_TW.po \
		zh_TW.UTF-8.po \


MOFILES = \
		af.mo \
		ca.mo \
		cs.mo \
		da.mo \
		de.mo \
		en_GB.mo \
		eo.mo \
		es.mo \
		fi.mo \
		fr.mo \
		ga.mo \
		hu.mo \
		it.mo \
		ja.mo \
		ko.UTF-8.mo \
		lv.mo \
		nb.mo \
		nl.mo \
		no.mo \
		pl.mo \
		pt_BR.mo \
		ru.mo \
		sk.mo \
		sr.mo \
		sv.mo \
		tr.mo \
		uk.mo \
		vi.mo \
		zh_CN.UTF-8.mo \
		zh_TW.UTF-8.mo \


MOCONVERTED = \
		cs.cp1250.mo \
		ja.euc-jp.mo \
		ja.sjis.mo \
		ko.mo \
		pl.UTF-8.mo \
		pl.cp1250.mo \
		ru.cp1251.mo \
		sk.cp1250.mo \
		uk.cp1251.mo \
		zh_CN.mo \
		zh_CN.cp936.mo \
		zh_TW.mo \


CHECKFILES = \
		af.ck \
		ca.ck \
		cs.ck \
		cs.cp1250.ck \
		da.ck \
		de.ck \
		en_GB.ck \
		eo.ck \
		es.ck \
		fi.ck \
		fr.ck \
		ga.ck \
		hu.ck \
		it.ck \
		ja.ck \
		ja.euc-jp.ck \
		ja.sjis.ck \
		ko.UTF-8.ck \
		ko.ck \
		lv.ck \
		nb.ck \
		nl.ck \
		no.ck \
		pl.UTF-8.ck \
		pl.ck \
		pl.cp1250.ck \
		pt_BR.ck \
		ru.ck \
		ru.cp1251.ck \
		sk.ck \
		sk.cp1250.ck \
		sr.ck \
		sv.ck \
		tr.ck \
		uk.ck \
		uk.cp1251.ck \
		vi.ck \
		zh_CN.UTF-8.ck \
		zh_CN.ck \
		zh_CN.cp936.ck \
		zh_TW.UTF-8.ck \
		zh_TW.ck \

PO_VIM_INPUTLIST = \
	../../runtime/optwin.vim \
	../../runtime/defaults.vim

PO_VIM_JSLIST = \
	optwin.js \
	defaults.js

# Arguments for xgettext to pick up messages to translate from the source code.
XGETTEXT_KEYWORDS = --keyword=_ --keyword=N_ --keyword=NGETTEXT:1,2 --keyword=PLURAL_MSG:2,4
