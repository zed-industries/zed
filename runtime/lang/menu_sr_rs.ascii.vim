" Menu Translations: Serbian
" Maintainer: Aleksandar Jelenak <ajelenak AT yahoo.com>
" Last Change:	Fri, 30 May 2003 10:17:39 Eastern Daylight Time
" Adapted for VIM 8 by: Ivan Pesic on 2017-12-28 12:05+0400
" Generated from menu_sr_rs.utf-8.vim, DO NOT EDIT

" Quit when menu translations have already been done.
if exists("did_menu_trans")
  finish
endif
let did_menu_trans = 1
let s:keepcpo= &cpo
set cpo&vim

scriptencoding latin1

" Help menu
menutrans &Help		      Pomo&c
menutrans &Overview<Tab><F1>  &Pregled<Tab><F1>
menutrans &User\ Manual       &Uputstvo\ za\ korisnike
menutrans &How-to\ links      &Kako\ da\.\.\.
menutrans &Find		      &Nadji
menutrans &Credits	      &Zasluge
menutrans Co&pying	      P&reuzimanje
menutrans O&rphans	      &Sirocici
menutrans &Sponsor/Register   Sponzor/&Registrujte\ se
menutrans &Version	      &Verzija
menutrans &About	      &O\ programu

" File menu
menutrans &File			    &Fajl
menutrans &Open\.\.\.<Tab>:e	    &Otvori\.\.\.<Tab>:e
menutrans Sp&lit-Open\.\.\.<Tab>:sp &Podeli-otvori\.\.\.<Tab>:sp
menutrans Open\ &Tab\.\.\.<Tab>:tabnew	Otvori\ karticu\.\.\.<Tab>:tabnew
menutrans &New<Tab>:enew	    &Nov<Tab>:enew
menutrans &Close<Tab>:close	    &Zatvori<Tab>:close
menutrans &Save<Tab>:w		    &Sacuvaj<Tab>:w
menutrans Save\ &As\.\.\.<Tab>:sav  Sacuvaj\ &kao\.\.\.<Tab>:sav
menutrans Split\ &Diff\ with\.\.\.  Podeli\ i\ &uporedi\ sa\.\.\.
menutrans Split\ Patched\ &By\.\.\. Po&deli\ i\ prepravi\ sa\.\.\.
menutrans &Print		    Sta&mpaj
menutrans Sa&ve-Exit<Tab>:wqa	    Sacuvaj\ i\ za&vrsi<Tab>:wqa
menutrans E&xit<Tab>:qa		    K&raj<Tab>:qa

" Edit menu
menutrans &Edit			 &Uredjivanje
menutrans &Undo<Tab>u		 &Ponisti<Tab>u
menutrans &Redo<Tab>^R		 &Vrati\ izmenu<Tab>^R
menutrans Rep&eat<Tab>\.	 P&onovi<Tab>\.
menutrans Cu&t<Tab>"+x		 Ise&ci<Tab>"+x
menutrans &Copy<Tab>"+y		 &Kopiraj<Tab>"+y
menutrans &Paste<Tab>"+gP	 &Ubaci<Tab>"+gP
menutrans &Paste<Tab>"+P	&Ubaci<Tab>"+P
menutrans Put\ &Before<Tab>[p	 Stavi\ ispre&d<Tab>[p
menutrans Put\ &After<Tab>]p	 Stavi\ &iza<Tab>]p
menutrans &Delete<Tab>x		 Iz&brisi<Tab>x
menutrans &Select\ all<Tab>ggVG  Izaberi\ sv&e<Tab>ggVG
menutrans &Find\.\.\.		 &Nadji\.\.\.
menutrans Find\ and\ Rep&lace\.\.\. Nadji\ i\ &zameni\.\.\.
menutrans Settings\ &Window	 P&rozor\ podesavanja
menutrans Startup\ &Settings	 Po&desavanja\ pri\ pokretanju		
menutrans &Global\ Settings	 Op&sta\ podesavanja
menutrans F&ile\ Settings	 Podesavanja\ za\ faj&love
menutrans &Shiftwidth		 &Korak\ uvlacenja
menutrans Soft\ &Tabstop	 &Meka\ tabulacija
menutrans Te&xt\ Width\.\.\.	 &Sirina\ teksta\.\.\.
menutrans &File\ Format\.\.\.	 &Vrsta\ fajla\.\.\.
menutrans Show\ C&olor\ Schemes\ in\ Menu	Prikazi\ seme\ bo&ja\ u\ meniju
menutrans C&olor\ Scheme	\Seme\ bo&ja
menutrans Show\ &Keymaps\ in\ Menu	Prikazi\ pres&likavanja\ tastature\ u\ meniju
menutrans &Keymap	Pres&likavanja\ tastature
menutrans Select\ Fo&nt\.\.\.	 Izbor\ &fonta\.\.\.

" Edit/Global Settings
menutrans Toggle\ Pattern\ &Highlight<Tab>:set\ hls! Isticanje\ &sablona\ (da/ne)<Tab>:set\ hls!
menutrans Toggle\ &Ignoring\ Case<Tab>:set\ ic! Zanemari\ velicinu\ &slova\ (da/ne)<Tab>:set\ ic!
menutrans Toggle\ &Showing\ Matched\ Pairs<Tab>:set\ sm! Prikazi\ uparenu\ &zagradu\ (da/ne)<Tab>:set\ sm!
menutrans &Context\ lines  Vidljivi\ &redovi
menutrans &Virtual\ Edit   Virtuelno\ &uredjivanje
menutrans Toggle\ Insert\ &Mode<Tab>:set\ im!   Rezim\ U&metanje\ (da/ne)<Tab>:set\ im!
menutrans Toggle\ Vi\ C&ompatibility<Tab>:set\ cp!     '&Vi'\ saglasno\ (da/ne)<Tab>:set\ cp!
menutrans Search\ &Path\.\.\. Putanja\ &pretrage\.\.\.
menutrans Ta&g\ Files\.\.\.   &Fajlovi\ oznaka\.\.\.
menutrans Toggle\ &Toolbar    Linija\ sa\ &alatkama\ (da/ne)
menutrans Toggle\ &Bottom\ Scrollbar   Donja\ l&inija\ klizanja\ (da/ne)
menutrans Toggle\ &Left\ Scrollbar  &Leva\ linija\ klizanja\ (da/ne)
menutrans Toggle\ &Right\ Scrollbar &Desna\ linija\ klizanja\ (da/ne)

" Edit/Global Settings/Virtual Edit
menutrans Never		      Nikad
menutrans Block\ Selection    Izbor\ bloka
menutrans Insert\ mode	      Rezim\ Umetanje
menutrans Block\ and\ Insert  Blok\ i\ Umetanje
menutrans Always	      Uvek

" Edit/File Settings
menutrans Toggle\ Line\ &Numbering<Tab>:set\ nu!   Prikazi\ &numeraciju\ linija\ (da/ne)<Tab>:set\ nu!
menutrans Toggle\ Relati&ve\ Line\ Numbering<Tab>:set\ rnu!   Prikazi\ Relati&vnu\ numeraciju\ linija\ (da/ne)<Tab>:set\ rnu!
menutrans Toggle\ &List\ Mode<Tab>:set\ list!	   Rezim\ &liste\ (da/ne)<Tab>:set\ list!
menutrans Toggle\ Line\ &Wrapping<Tab>:set\ wrap!	   Obavijanje\ &redova\ (da/ne)<Tab>:set\ wrap!
menutrans Toggle\ W&rapping\ at\ Word<Tab>:set\ lbr!   Prelomi\ &na\ rec\ (da/ne)<Tab>:set\ lbr!
menutrans Toggle\ Tab\ &Expanding<Tab>:set\ et!	   Razmaci\ umesto\ &tabulacije\ (da/ne)<Tab>:set\ et!
menutrans Toggle\ &Auto\ Indenting<Tab>:set\ ai!	Auto-&uvlacenje\ (da/ne)<Tab>:set\ ai!
menutrans Toggle\ &C-Style\ Indenting<Tab>:set\ cin!	   &C-uvlacenje\ (da/ne)<Tab>:set\ cin!

" Edit/Keymap
menutrans None Bez\ preslikavanja

" Tools menu
menutrans &Tools	&Alatke
menutrans &Jump\ to\ this\ tag<Tab>g^] Skoci\ na\ &ovu\ oznaku<Tab>g^]
menutrans Jump\ &back<Tab>^T	 Skoci\ &natrag<Tab>^T
menutrans Build\ &Tags\ File	 Izgradi\ &fajl\ oznaka
menutrans &Spelling	 Pra&vopis
menutrans &Folding	      &Podvijanje
menutrans Create\ &Fold<Tab>zf		  S&tvori\ svijutak<Tab>zf
menutrans &Delete\ Fold<Tab>zd		  O&brisi\ svijutak<Tab>zd
menutrans Delete\ &All\ Folds<Tab>zD	  Obrisi\ sve\ sv&ijutke<Tab>zD
menutrans Fold\ column\ &width		  Sirina\ &reda\ cvijutka
"menutrans &Diff		      &Uporedjivanje
menutrans &Make<Tab>:make     'mak&e'<Tab>:make
menutrans &List\ Errors<Tab>:cl     Spisak\ &gresaka<Tab>:cl
menutrans L&ist\ Messages<Tab>:cl!  Sp&isak\ poruka<Tab>:cl!
menutrans &Next\ Error<Tab>:cn	    S&ledeca\ greska<Tab>:cn
menutrans &Previous\ Error<Tab>:cp  Pre&thodna\ greska<Tab>:cp
menutrans &Older\ List<Tab>:cold    Stari\ spisa&k<Tab>:cold
menutrans N&ewer\ List<Tab>:cnew    No&vi\ spisak<Tab>:cnew
menutrans Error\ &Window	    Prozor\ sa\ g&reskama
menutrans Se&t\ Compiler	    I&zaberi\ prevodioca
menutrans &Convert\ to\ HEX<Tab>:%!xxd	   Pretvori\ u\ &HEKS<Tab>:%!xxd
menutrans Conve&rt\ back<Tab>:%!xxd\ -r    Vr&ati\ u\ prvobitan\ oblik<Tab>:%!xxd\ -r
menutrans Show\ Compiler\ Se&ttings\ in\ Menu	Prikazi\ pode&savanja\ prevodioca\ u\ meniju

" Tools/Spelling
menutrans &Spell\ Check\ On	&Ukljuci\ proveru\ pravopisa
menutrans Spell\ Check\ &Off	&Iskljuci\ proveru\ pravopisa
menutrans To\ &Next\ Error<Tab>]s	Idi\ na\ &sledecu\ gresku<Tab>]s
menutrans To\ &Previous\ Error<Tab>[s	Idi\ na\ &prethodnu\ gresku<Tab>[s
menutrans Suggest\ &Corrections<Tab>z=	Predlozi\ isp&ravke<Tab>z=
menutrans &Repeat\ Correction<Tab>:spellrepall	P&onovi\ ispravku<Tab>:spellrepall
menutrans Set\ Language\ to\ "en"	Postavi\ jezik\ na\ "en"
menutrans Set\ Language\ to\ "en_au" 	Postavi\ jezik\ na\ "en_au"
menutrans Set\ Language\ to\ "en_ca" 	Postavi\ jezik\ na\ "en_ca"
menutrans Set\ Language\ to\ "en_gb" 	Postavi\ jezik\ na\ "en_gb"
menutrans Set\ Language\ to\ "en_nz" 	Postavi\ jezik\ na\ "en_nz"
menutrans Set\ Language\ to\ "en_us" 	Postavi\ jezik\ na\ "en_us"
menutrans &Find\ More\ Languages	Pronadji\ jos\ jezika 

" Tools/Folding
menutrans &Enable/Disable\ folds<Tab>zi   &Omoguci/prekini\ svijanje<Tab>zi
menutrans &View\ Cursor\ Line<Tab>zv	  &Pokazi\ red\ sa\ kursorom<Tab>zv
menutrans Vie&w\ Cursor\ Line\ only<Tab>zMzx Pokazi\ &samo\ red\ sa\ kursorom<Tab>zMzx
menutrans C&lose\ more\ folds<Tab>zm   &Zatvori\ vise\ svijutaka<Tab>zm
menutrans &Close\ all\ folds<Tab>zM    Zatvori\ s&ve\ svijutke<Tab>zM
menutrans O&pen\ more\ folds<Tab>zr    Otvori\ vis&e\ svijutaka<Tab>zr
menutrans &Open\ all\ folds<Tab>zR     O&tvori\ sve\ svijutke<Tab>zR
menutrans Fold\ Met&hod		       &Nacin\ podvijanja
menutrans Fold\ Col&umn\ Width	Sirina\ kolone\ ispred\ svijutaka

" Tools/Folding/Fold Method
menutrans M&anual	&Rucno
menutrans I&ndent	&Uvucenost
menutrans E&xpression	&Izraz
menutrans S&yntax	&Sintaksa
"menutrans &Diff
menutrans Ma&rker	&Oznaka

" Tools/Diff
menutrans &Update	&Azuriraj
menutrans &Get\ Block	&Prihvati\ blok\ izmena
menutrans &Put\ Block	Pre&baci\ blok\ izmena

" Tools/Error Window
menutrans &Update<Tab>:cwin   &Azuriraj<Tab>:cwin
menutrans &Open<Tab>:copen    &Otvori<Tab>:copen
menutrans &Close<Tab>:cclose  &Zatvori<Tab>:cclose

" Bufers menu
menutrans &Buffers	   &Baferi
menutrans &Refresh\ menu   &Azuriraj
menutrans Delete	   &Obrisi
menutrans &Alternate	   A&lternativni
menutrans &Next		   &Sledeci
menutrans &Previous	   &Prethodni
menutrans [No\ File]	   [Nema\ fajla]

" Window menu
menutrans &Window		    &Prozor
menutrans &New<Tab>^Wn		    &Novi<Tab>^Wn
menutrans S&plit<Tab>^Ws	    &Podeli<Tab>^Ws
menutrans Sp&lit\ To\ #<Tab>^W^^    Podeli\ sa\ &alternativnim<Tab>^W^^
menutrans Split\ &Vertically<Tab>^Wv   Podeli\ &uspravno<Tab>^Wv
menutrans Split\ File\ E&xplorer    Podeli\ za\ pregled\ &fajlova
menutrans &Close<Tab>^Wc	    &Zatvori<Tab>^Wc
menutrans Close\ &Other(s)<Tab>^Wo  Zatvori\ &ostale<Tab>^Wo
"menutrans Ne&xt<Tab>^Ww       &Sledeci<Tab>^Ww
"menutrans P&revious<Tab>^WW	  P&rethodni<Tab>^WW
menutrans Move\ &To		    Pre&mesti
menutrans Rotate\ &Up<Tab>^WR	    &Kruzno\ nagore<Tab>^WR
menutrans Rotate\ &Down<Tab>^Wr     Kruzno\ nadol&e<Tab>^Wr
menutrans &Equal\ Size<Tab>^W=	    &Iste\ velicine<Tab>^W=
menutrans &Max\ Height<Tab>^W_	    Maksimalna\ &visina<Tab>^W_
menutrans M&in\ Height<Tab>^W1_     Minima&lna\ visina<Tab>^W1_
menutrans Max\ &Width<Tab>^W\|	    Maksimalna\ &sirina<Tab>^W\|
menutrans Min\ Widt&h<Tab>^W1\|     Minimalna\ si&rina<Tab>^W1\|

" Window/Move To
menutrans &Top<Tab>^WK		 &Vrh<Tab>^WK
menutrans &Bottom<Tab>^WJ	 &Podnozje<Tab>^WJ
menutrans &Left\ side<Tab>^WH	 U&levo<Tab>^WH
menutrans &Right\ side<Tab>^WL	 U&desno<Tab>^WL

" The popup menu
menutrans &Undo		      &Ponisti
menutrans Cu&t		      &Iseci
menutrans &Copy		      &Kopiraj
menutrans &Paste	      &Ubaci
menutrans &Delete	      I&zbrisi
menutrans Select\ Blockwise   Biraj\ b&lokovski
menutrans Select\ &Word       Izaberi\ &rec
menutrans Select\ &Sentence       Izaberi\ r&ecenicu
menutrans Select\ Pa&ragraph       Izaberi\ &pasus
menutrans Select\ &Line       Izaberi\ r&ed
menutrans Select\ &Block      Izaberi\ &blok
menutrans Select\ &All	      Izaberi\ &sve

" The GUI toolbar
if has("toolbar")
  if exists("*Do_toolbar_tmenu")
    delfun Do_toolbar_tmenu
  endif
  fun Do_toolbar_tmenu()
    tmenu ToolBar.Open     Ucitaj
    tmenu ToolBar.Save     Sacuvaj
    tmenu ToolBar.SaveAll  Sacuvaj sve
    tmenu ToolBar.Print    Stampaj
    tmenu ToolBar.Undo     Vrati
    tmenu ToolBar.Redo     Povrati
    tmenu ToolBar.Cut      Iseci
    tmenu ToolBar.Copy     Kopiraj
    tmenu ToolBar.Paste    Ubaci
    tmenu ToolBar.Find     Nadji
    tmenu ToolBar.FindNext Nadji sledeci
    tmenu ToolBar.FindPrev Nadji prethodni
    tmenu ToolBar.Replace  Zameni
    tmenu ToolBar.New      Novi
    tmenu ToolBar.WinSplit Podeli prozor
    tmenu ToolBar.WinMax   Maksimalna visina
    tmenu ToolBar.WinMin   Minimalna visina
    tmenu ToolBar.WinVSplit   Podeli uspravno
    tmenu ToolBar.WinMaxWidth Maksimalna sirina
    tmenu ToolBar.WinMinWidth Minimalna sirina
    tmenu ToolBar.WinClose Zatvori prozor
    tmenu ToolBar.LoadSesn Ucitaj seansu
    tmenu ToolBar.SaveSesn Sacuvaj seansu
    tmenu ToolBar.RunScript   Izvrsi spis
    tmenu ToolBar.Make     'make'
    tmenu ToolBar.Shell    Operativno okruzenje
    tmenu ToolBar.RunCtags Napravi oznake
    tmenu ToolBar.TagJump  Idi na oznaku
    tmenu ToolBar.Help     Pomoc
    tmenu ToolBar.FindHelp Nadji objasnjenje
  endfun
endif

" Syntax menu
menutrans &Syntax &Sintaksa
menutrans &Show\ File\ Types\ in\ Menu  Prikazi\ tipove\ fajlova\ u\ &meniju
menutrans Set\ '&syntax'\ only   Pode&si\ samo\ 'syntax' 
menutrans Set\ '&filetype'\ too  Podesi\ &takodje\ i\ 'filetype'
menutrans &Off       &Iskljuceno
menutrans &Manual    &Rucno
menutrans A&utomatic    &Automatski
menutrans on/off\ for\ &This\ file     Da/ne\ za\ ovaj\ &fajl
menutrans Co&lor\ test     Provera\ &boja
menutrans &Highlight\ test Provera\ isti&canja
menutrans &Convert\ to\ HTML  Pretvori\ &u\ HTML

" dialog texts
let menutrans_help_dialog = "Unesite naredbu ili rec cije pojasnjenje trazite:\n\nDodajte i_ za naredbe unosa (npr. i_CTRL-X)\nDodajte c_ za naredbe komandnog rezima (npr. s_<Del>)\nDodajte ' za imena opcija (npr. 'shiftwidth')"

let g:menutrans_path_dialog = "Unesite putanju pretrage za fajlove\nRazdvojite zarezima imena direktorijuma."

let g:menutrans_tags_dialog = "Unesite imena fajlova sa oznakama\nRazdvojite zarezima imena."

let g:menutrans_textwidth_dialog = "Unesite novu sirinu teksta (0 sprecava prelom)"

let g:menutrans_fileformat_dialog = "Izaberite format zapisa fajla"

let g:menutrans_fileformat_choices = "&Unix\n&Dos\n&Mac\n&Otkazi"

let menutrans_no_file = "[Nema fajla]"

let &cpo = s:keepcpo
unlet s:keepcpo

" vim: tw=0 keymap=serbian
