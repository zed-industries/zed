" Menu Translations:	Turkish
" Maintainer:		Emir SARI <bitigchi@me.com>
" Generated from menu_tr_tr.utf-8.vim, DO NOT EDIT

if exists("did_menu_trans")
   finish
endif

let did_menu_trans = 1
let s:keepcpo= &cpo
set cpo&vim
scriptencoding cp1254

" Top
menutrans &File	&Dosya
menutrans &Edit	Dü&zen
menutrans &Tools	&Araçlar
menutrans &Syntax	&Sözdizim
menutrans &Buffers	A&rabellekler
menutrans &Window	&Pencere
menutrans &Help	&Yardým

" Help menu
menutrans &Overview<Tab><F1>	&Genel\ Bakýþ<Tab><F1>
menutrans &User\ Manual	&Kullaným\ Kýlavuzu
menutrans &How-To\ Links	&Nasýl\ Yapýlýr?
menutrans &Find\.\.\.	        &Bul\.\.\.
"--------------------
menutrans &Credits		&Teþekkürler
menutrans Co&pying		&Daðýtým
menutrans &Sponsor/Register	&Sponsorluk/Kayýt
menutrans O&rphans		&Yetimler
"--------------------
menutrans &Version	Sürüm\ &Bilgisi
menutrans &About	&Hakkýnda

" File menu
menutrans &Open\.\.\.<Tab>:e		&Aç\.\.\.<Tab>:e
menutrans Sp&lit-Open\.\.\.<Tab>:sp	&Yeni\ Bölümde\ Aç\.\.\.<Tab>:sp
menutrans Open\ &Tab\.\.\.<Tab>:tabnew	S&ekme\ Aç\.\.\.<Tab>:tabnew
menutrans &New<Tab>:enew	        Yeni\ &Sekme<Tab>:enew
menutrans &Close<Tab>:close		Ka&pat<Tab>:close
"--------------------
menutrans &Save<Tab>:w			Kayde&t<Tab>:w
menutrans Save\ &As\.\.\.<Tab>:sav	&Farklý\ Kaydet\.\.\.<Tab>:sav
"--------------------
menutrans Split\ &Diff\ With\.\.\.	Ka&rþýlaþtýr\.\.\.
menutrans Split\ Patched\ &By\.\.\.	Ya&ma\ Ýle\ Karþýlaþtýr\.\.\.
"--------------------
menutrans &Print		Ya&zdýr
menutrans Sa&ve-Exit<Tab>:wqa	        Kaydet\ &ve\ Çýk<Tab>:wqa
menutrans E&xit<Tab>:qa		Çý&k<Tab>:qa

" Edit menu
menutrans &Undo<Tab>u		        &Geri\ Al<Tab>u
menutrans &Redo<Tab>^R		        &Yinele<Tab>^R
menutrans Rep&eat<Tab>\.        	Son\ Komutu\ Y&inele<Tab>\.
"--------------------
menutrans Cu&t<Tab>"+x                 &Kes<Tab>"+x
menutrans &Copy<Tab>"+y 	        K&opyala<Tab>"+y
menutrans &Paste<Tab>"+gP              Ya&pýþtýr<Tab>"+gP
menutrans Put\ &Before<Tab>[p	        Ö&nüne\ Koy<Tab>[p
menutrans Put\ &After<Tab>]p	        A&rkasýna\ Koy<Tab>]p
menutrans &Delete<Tab>x 	        Si&l<Tab>x
menutrans &Select\ All<Tab>ggVG	Tü&münü\ Seç<Tab>ggVG
"--------------------
" Athena GUI only
menutrans &Find<Tab>/		                &Bul<Tab>/
menutrans Find\ and\ Rep&lace<Tab>:%s	        Bul\ &ve\ Deðiþtir<Tab>:%s
" End Athena GUI only
menutrans &Find\.\.\.<Tab>/		        &Bul\.\.\.<Tab>/
menutrans Find\ and\ Rep&lace\.\.\.	        Bul\ ve\ &Deðiþtir\.\.\.
menutrans Find\ and\ Rep&lace\.\.\.<Tab>:%s	Bul\ ve\ &Deðiþtir\.\.\.<Tab>:%s
menutrans Find\ and\ Rep&lace\.\.\.<Tab>:s	Bul\ ve\ &Deðiþtir\.\.\.<Tab>:s
"--------------------
menutrans Settings\ &Window	&Ayarlar\ Penceresi
menutrans Startup\ &Settings	Baþlan&gýç\ Ayarlarý
menutrans &Global\ Settings	Ge&nel\ Ayarlar
menutrans F&ile\ Settings	&Dosya\ Ayarlarý
menutrans C&olor\ Scheme	&Renk\ Düzeni
menutrans &Keymap		Düðme\ &Eþlem
menutrans Select\ Fo&nt\.\.\.	Ya&zýtipi\ Seç\.\.\.

">>>----------------- Edit/Global settings
menutrans Toggle\ Pattern\ &Highlight<Tab>:set\ hls!    	Dizgi\ &Vurgulamasýný\ Aç/Kapat<Tab>:set\ hls!
menutrans Toggle\ &Ignoring\ Case<Tab>:set\ ic!		BÜYÜK/küçük\ Harf\ &Duyarlý\ Aç/Kapat<Tab>:set\ ic!
menutrans Toggle\ &Showing\ Matched\ Pairs<Tab>:set\ sm!	Eþ&leþen\ Ýkilileri\ Aç/Kapat<Tab>:set\ sm!
menutrans &Context\ Lines					Ý&mleçle\ Oynayan\ Satýrlar
menutrans &Virtual\ Edit					&Sanal\ Düzenleme
menutrans Toggle\ Insert\ &Mode<Tab>:set\ im!			Ekleme\ &Kipini\ Aç/Kapat<Tab>:set\ im!
menutrans Toggle\ Vi\ C&ompatibility<Tab>:set\ cp!		&Vi\ Uyumlu\ Kipi\ Aç/Kapat<Tab>:set\ cp!
menutrans Search\ &Path\.\.\.					&Arama\ Yolu\.\.\.
menutrans Ta&g\ Files\.\.\.					&Etiket\ Dosyalarý\.\.\.
"
menutrans Toggle\ &Toolbar		&Araç\ Çubuðunu\ Aç/Kapat
menutrans Toggle\ &Bottom\ Scrollbar	A&lt\ Kaydýrma\ Çubuðunu\ Aç/Kapat
menutrans Toggle\ &Left\ Scrollbar	&Sol\ Kaydýrma\ Çubuðunu\ Aç/Kapat
menutrans Toggle\ &Right\ Scrollbar	S&að\ Kaydýrma\ Çubuðunu\ Aç/Kapat

">>>->>>------------- Edit/Global settings/Virtual edit
menutrans Never		Kapalý
menutrans Block\ Selection	Blok\ Seçimi
menutrans Insert\ Mode		Ekleme\ Kipi
menutrans Block\ and\ Insert	Blok\ Seçiminde\ ve\ Ekleme\ Kipinde
menutrans Always		Her\ Zaman\ Açýk
">>>----------------- Edit/File settings
menutrans Toggle\ Line\ &Numbering<Tab>:set\ nu!		&Satýr\ Numaralandýrmayý\ Aç/Kapat<Tab>:set\ nu!
menutrans Toggle\ Relati&ve\ Line\ Numbering<Tab>:set\ rnu!	&Göreceli\ Satýr\ Numaralandýrmayý\ Aç/Kapat<Tab>:set\ nru!
menutrans Toggle\ &List\ Mode<Tab>:set\ list!			Gö&rünmeyen\ Karakterleri\ Aç/Kapat<Tab>:set\ list!
menutrans Toggle\ Line\ &Wrapping<Tab>:set\ wrap!		Sa&týr\ Kaydýrmayý\ Aç/Kapat<Tab>:set\ wrap!
menutrans Toggle\ W&rapping\ at\ Word<Tab>:set\ lbr!		Sö&zcük\ Kaydýrmayý\ Aç/Kapat<Tab>:set\ lbr!
menutrans Toggle\ Tab\ &Expanding-tab<Tab>:set\ et!		S&ekmeleri\ Boþluklara\ Dönüþtürmeyi\ Aç/Kapat<Tab>:set\ et!
menutrans Toggle\ &Auto\ Indenting<Tab>:set\ ai!		&Otomatik\ Girintilemeyi\ Aç/Kapat<Tab>:set\ ai!
menutrans Toggle\ &C-Style\ Indenting<Tab>:set\ cin!		&C\ Tarzý\ Girintilemeyi\ Aç/Kapat<Tab>:set\ cin!
">>>---
menutrans &Shiftwidth		&Girinti\ Düzeyi
menutrans Soft\ &Tabstop	&Sekme\ Geniþliði
menutrans Te&xt\ Width\.\.\.	&Metin\ Geniþliði\.\.\.
menutrans &File\ Format\.\.\.	&Dosya\ Biçimi\.\.\.

" Tools menu
menutrans &Jump\ to\ This\ Tag<Tab>g^]	Þ&u\ Etikete\ Atla<Tab>g^]
menutrans Jump\ &Back<Tab>^T		&Geri\ Dön<Tab>^T
menutrans Build\ &Tags\ File		&Etiket\ Dosyasý\ Oluþtur
"-------------------
menutrans &Folding	&Kývýrmalar
menutrans &Spelling	&Yazým\ Denetimi
menutrans &Diff	K&arþýlaþtýrma\ (diff)
"-------------------
menutrans &Make<Tab>:make			&Derle<Tab>:make
menutrans &List\ Errors<Tab>:cl		&Hatalarý\ Listele<Tab>:cl
menutrans L&ist\ Messages<Tab>:cl!		Ý&letileri\ Listele<Tab>:cl!
menutrans &Next\ Error<Tab>:cn			Bir\ &Sonraki\ Hata<Tab>:cn
menutrans &Previous\ Error<Tab>:cp		Bir\ Ö&nceki\ Hata<Tab>:cp
menutrans &Older\ List<Tab>:cold		Daha\ &Eski\ Hatalar<Tab>:cold
menutrans N&ewer\ List<Tab>:cnew		Daha\ &Yeni\ Hatalar<Tab>:cnew
menutrans Error\ &Window			Hatalar\ &Penceresi
menutrans Se&t\ Compiler			De&rleyici\ Seç
menutrans Show\ Compiler\ Se&ttings\ in\ Menu	Derleyici\ Ayarlarýný\ Menüde\ &Göster 
"-------------------
menutrans &Convert\ to\ HEX<Tab>:%!xxd	    	HEX'e\ Dö&nüþtür<Tab>:%!xxd
menutrans Conve&rt\ Back<Tab>:%!xxd\ -r	HEX'&ten\ Dönüþtür<Tab>:%!xxd\ -r
">>>---------------- Tools/Spelling
menutrans &Spell\ Check\ On			Yazým\ Denetimini\ &Aç
menutrans Spell\ Check\ &Off			Yazým\ Denetimini\ &Kapat
menutrans To\ &Next\ Error<Tab>]s		Bir\ &Sonraki\ Hata<Tab>]s
menutrans To\ &Previous\ Error<Tab>[s		Bir\ Ö&nceki\ Hata<Tab>[s
menutrans Suggest\ &Corrections<Tab>z=		Dü&zeltme\ Öner<Tab>z=
menutrans &Repeat\ Correction<Tab>:spellrepall	Düzeltmeyi\ &Yinele<Tab>spellrepall
"-------------------
menutrans Set\ Language\ to\ "en"	Dili\ "en"\ yap
menutrans Set\ Language\ to\ "en_au"	Dili\ "en_au"\ yap
menutrans Set\ Language\ to\ "en_ca"	Dili\ "en_ca"\ yap
menutrans Set\ Language\ to\ "en_gb"	Dili\ "en_gb"\ yap
menutrans Set\ Language\ to\ "en_nz"	Dili\ "en_nz"\ yap
menutrans Set\ Language\ to\ "en_us"	Dili\ "en_us"\ yap
menutrans &Find\ More\ Languages	&Baþka\ Diller\ Bul
let g:menutrans_set_lang_to =		'Dil Yükle'

" The Spelling popup menu
let g:menutrans_spell_change_ARG_to =		'Düzeltilecek:\ "%s"\ ->'
let g:menutrans_spell_add_ARG_to_word_list =	'"%s"\ sözcüðünü\ sözlüðe\ ekle'
let g:menutrans_spell_ignore_ARG =		'"%s"\ sözcüðünü\ yoksay'
">>>---------------- Folds
menutrans &Enable/Disable\ Folds<Tab>zi		&Kývýrmalarý\ Aç/Kapat<Tab>zi
menutrans &View\ Cursor\ Line<Tab>zv		Ý&mlecin\ Olduðu\ Satýrý\ Görüntüle<Tab>zv
menutrans Vie&w\ Cursor\ Line\ Only<Tab>zMzx	Ya&lnýzca\ Ýmlecin\ Olduðu\ Satýrý\ Görüntüle<Tab>zMzx
menutrans C&lose\ More\ Folds<Tab>zm		&Daha\ Fazla\ Kývýrma\ Kapat<Tab>zm
menutrans &Close\ All\ Folds<Tab>zM		Bütün\ Ký&výrmalarý\ Kapat<Tab>zM
menutrans &Open\ All\ Folds<Tab>zR		Bü&tün\ Kývýrmalarý\ Aç<Tab>zR
menutrans O&pen\ More\ Folds<Tab>zr		D&aha\ Fazla\ Kývýrma\ Aç<Tab>zr
menutrans Fold\ Met&hod				Kývý&rma\ Yöntemi
menutrans Create\ &Fold<Tab>zf			Kývýrma\ &Oluþtur<Tab>zf
menutrans &Delete\ Fold<Tab>zd			Kývýrma\ &Sil<Tab>zd
menutrans Delete\ &All\ Folds<Tab>zD		Tü&m\ Kývýrmalarý\ Sil<Tab>zD
menutrans Fold\ col&umn\ Width			Kývýrma\ Sütunu\ &Geniþliði
">>>->>>----------- Tools/Folds/Fold Method
menutrans M&anual	&El\ Ýle
menutrans I&ndent	&Girinti
menutrans E&xpression	Ý&fade
menutrans S&yntax	&Sözdizim
menutrans Ma&rker	Ý&mleyici
">>>--------------- Tools/Diff
menutrans &Update	&Güncelle
menutrans &Get\ Block	Bloðu\ &Al
menutrans &Put\ Block	Bloðu\ &Koy
">>>--------------- Tools/Diff/Error window
menutrans &Update<Tab>:cwin	&Güncelle<Tab>:cwin
menutrans &Close<Tab>:cclose	&Kapat<Tab>:cclose
menutrans &Open<Tab>:copen	&Aç<Tab>:copen

" Syntax menu
menutrans &Show\ File\ Types\ in\ Menu	Dosya\ Türlerini\ Menüde\ &Göster
menutrans Set\ '&syntax'\ only		Yalnýzca\ 'syntax'\ &Ayarla
menutrans Set\ '&filetype'\ too	'filetype'\ Ýçin\ &de\ Ayarla
menutrans &Off				&Kapat
menutrans &Manual			&El\ Ýle
menutrans A&utomatic			&Otomatik
menutrans On/Off\ for\ &This\ File	&Bu\ Dosya\ Ýçin\ Aç/Kapat
menutrans Co&lor\ Test			&Renk\ Testi
menutrans &Highlight\ Test		&Vurgulama\ Testi
menutrans &Convert\ to\ HTML		&HTML'ye\ Dönüþtür

" Buffers menu
menutrans &Refresh\ menu	&Menüyü\ Güncelle
menutrans Delete		&Sil
menutrans &Alternate		Ö&teki
menutrans &Next		So&nraki
menutrans &Previous		Ön&ceki
menutrans [No\ File]		[Dosya\ Yok]

" Window menu
menutrans &New<Tab>^Wn			Yeni\ &Pencere<Tab>^Wn
menutrans S&plit<Tab>^Ws		Pencereyi\ &Böl<Tab>^Ws
menutrans Sp&lit\ To\ #<Tab>^W^^	Pencereyi\ Baþkasýna\ Bö&l<Tab>^W^^
menutrans Split\ &Vertically<Tab>^Wv	Pencereyi\ &Dikey\ Olarak\ Böl<Tab>^Wv
menutrans Split\ File\ E&xplorer	Yeni\ Bölü&mde\ Dosya\ Gezginini\ Aç
"
menutrans &Close<Tab>^Wc		Pen&cereyi\ Kapat<Tab>^Wc
menutrans Close\ &Other(s)<Tab>^Wo	Diðer\ Pencerele&ri\ Kapat<Tab>^Wo
"
menutrans Move\ &To		&Taþý
menutrans Rotate\ &Up<Tab>^WR	&Yukarý\ Taþý<Tab>^WR
menutrans Rotate\ &Down<Tab>^Wr	&Aþaðý\ Taþý<Tab>^Wr
"
menutrans &Equal\ Size<Tab>^W=	&Eþit\ Boyut<Tab>^W=
menutrans &Max\ Height<Tab>^W_	E&n\ Büyük\ Yükseklik<Tab>^W_
menutrans M&in\ Height<Tab>^W1_	En\ Küçük\ Yüksekl&ik<Tab>^W1_
menutrans Max\ &Width<Tab>^W\|	En\ Büyük\ Gen&iþlik<Tab>^W\|
menutrans Min\ Widt&h<Tab>^W1\|	En\ Küçük\ Geniþli&k<Tab>^W1\|

">>>----------------- Window/Move To
menutrans &Top<Tab>^WK		&Yukarý<Tab>^WK
menutrans &Bottom<Tab>^WJ	&Aþaðý<Tab>^WJ
menutrans &Left\ Side<Tab>^WH	So&la<Tab>^WH
menutrans &Right\ Side<Tab>^WL	&Saða<Tab>^WL

" The popup menu
menutrans &Undo		&Geri\ Al
menutrans Cu&t			&Kes
menutrans &Copy		K&opyala
menutrans &Paste		&Yapýþtýr
menutrans &Delete		&Sil
menutrans Select\ Blockwise	&Blok\ Biçiminde\ Seç
menutrans Select\ &Word	Sö&zcük\ Seç
menutrans Select\ &Sentence	&Tümce\ Seç
menutrans Select\ Pa&ragraph	&Paragraf\ Seç
menutrans Select\ &Line	S&atýr\ Seç
menutrans Select\ &Block	Bl&ok\ Seç
menutrans Select\ &All		Tümü&nü\ Seç

" The GUI toolbar
if has("toolbar")
	if exists("*Do_toolbar_tmenu")
		delfun Do_toolbar_tmenu
	endif

	fun Do_toolbar_tmenu()
		tmenu ToolBar.Open	Dosya Aç
		tmenu ToolBar.Save	Dosya Kaydet
		tmenu ToolBar.SaveAll	Tüm Dosyalarý Kaydet
		tmenu ToolBar.Print	Yazdýr
		tmenu ToolBar.Undo	Geri Al
		tmenu ToolBar.Redo	Yinele
		tmenu ToolBar.Cut	Kes
		tmenu ToolBar.Copy	Kopyala
		tmenu ToolBar.Paste	Yapýþtýr
		tmenu ToolBar.Find	Bul...
		tmenu ToolBar.FindNext	Sonrakini Bul
		tmenu ToolBar.FindPrev	Öncekini Bul
		tmenu ToolBar.Replace	Bul ve Deðiþtir...
		if 0	" disabled; These are in the Windows menu
			tmenu ToolBar.New	Yeni Pencere
			tmenu ToolBar.WinSplit	Pencereyi Böl
			tmenu ToolBar.WinMax	En Büyük Pencere Yüksekliði
			tmenu ToolBar.WinMin	En Küçük Pencere Yüksekliði
			tmenu ToolBar.WinClose	Pencereyi Kapat
		endif
		tmenu ToolBar.LoadSesn	Oturum Yükle
		tmenu ToolBar.SaveSesn	Oturum Kaydet
		tmenu ToolBar.RunScript	Betik Çalýþtýr
		tmenu ToolBar.Make	Derle
		tmenu ToolBar.Shell	Kabuk
		tmenu ToolBar.RunCtags	Etiket Dosyasý Oluþtur
		tmenu ToolBar.TagJump	Etikete Atla
		tmenu ToolBar.Help	Yardým
		tmenu ToolBar.FindHelp	Yardým Bul
	endfun
endif

" Dialog texts
" Find in help dialog
let g:menutrans_help_dialog = "Yardým için komut veya sözcük girin:\n\nEkleme Kipi komutlarýný aramak için i_ ekleyin (örneðin i_CTRL-X)\nNormal Kip komutlarýný aramak için _c ekleyin (örneðin c_<Del>)\nSeçenekler hakkýnda yardým almak için ' ekleyin (örneðin 'shiftwidth')"

" Searh path dialog
let g:menutrans_path_dialog = "Dosya aramasý için yol belirtin.\nDizin adlarý virgüllerle ayrýlýr."

" Tag files dialog
let g:menutrans_tags_dialog = "Etiket dosyasý adlarý belirtin (virgülle ayýrarak).\n"

" Text width dialog
let g:menutrans_textwidth_dialog = "Biçimlendirme için metin geniþliðini belirtin.\nBiçimlendirme iptali için 0 girin."

" File format dialog
let g:menutrans_fileformat_dialog = "Dosya biçimi seçin"
let g:menutrans_fileformat_choices = "&Unix\n&Dos\n&Mac\nÝ&ptal"
let menutrans_no_file = "[Dosya Yok]"
let &cpo = s:keepcpo
unlet s:keepcpo
