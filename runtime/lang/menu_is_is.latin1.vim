" Menu Translations:	Icelandic / Íslenska
" Maintainer:		Jón Arnar Briem <jonbriem@gmail.com>
" Originally By:	Jón Arnar Briem <jonbriem@gmail.com>
" Last Change:	Sun, 24 Mar 2019 22:40:00 CEST
" Original translations
" vim:set foldmethod=marker tabstop=8:

" Quit when menu translations have already been done.
if exists("did_menu_trans")
  finish
endif
let did_menu_trans = 1
let s:keepcpo= &cpo
set cpo&vim

" The translations below are in latin1, but they work for cp1252 and
" iso-8859-15 without conversion as well.
if &enc != "cp1252" && &enc != "iso-8859-15"
  scriptencoding latin1
endif

" {{{ FILE / SKRÁ
menutrans &File				                        Skrá
menutrans &Open\.\.\.<Tab>:e		                Opna\.\.\.<Tab>:e
menutrans Sp&lit-Open\.\.\.<Tab>:sp	                Splitt\ opna\.\.\.<Tab>:sp
menutrans Open\ &Tab\.\.\.<Tab>:tabnew	            Opna\ flipa\.\.\.<Tab>:tabnew
menutrans &New<Tab>:enew		                    Ný\ Skrá<Tab>:enew
menutrans &Close<Tab>:close		                    Loka<Tab>:close
menutrans &Save<Tab>:w			                    Vista<Tab>:w
menutrans Save\ &As\.\.\.<Tab>:sav	                Vista\ sem\.\.\.<Tab>:sav
menutrans &Print			                        Prenta
menutrans Sa&ve-Exit<Tab>:wqa		                Vista\ og\ Loka<Tab>:wqa
menutrans E&xit<Tab>:qa			                    Loka<Tab>:qa

if has("diff")
    menutrans Split\ &Diff\ with\.\.\.	            Splitt\ opna\ mismun\ við\.\.\.
    menutrans Split\ Patched\ &By\.\.\.	            Splitt\ opna\ plástrað\ af\.\.\.
endif
" }}} FILE / SKRÁ

" {{{ EDIT / BREYTA
menutrans &Edit				                        Breyta
menutrans &Undo<Tab>u			                    Afturkalla<Tab>u
menutrans &Redo<Tab>^R			                    Endurkalla<Tab>^R
menutrans Rep&eat<Tab>\.		                    Endurtaka<Tab>\.
menutrans Cu&t<Tab>"+x			                    Klippa<Tab>"+x
menutrans &Copy<Tab>"+y			                    Afrita<Tab>"+y
menutrans &Paste<Tab>"+gP		                    Líma<Tab>"+gP
menutrans Put\ &Before<Tab>[p		                Líma\ Fyrir<Tab>[p
menutrans Put\ &After<Tab>]p		                Líma\ Eftir<Tab>]p
menutrans &Delete<Tab>x			                    Eyða<Tab>x
menutrans &Select\ All<Tab>ggVG		                Velja\ Allt<Tab>ggVG
menutrans &Find\.\.\.			                    Finna\.\.\.
menutrans Find\ and\ Rep&lace\.\.\.	                Finna\ og\ Skipta\.\.\.

" [-- SETTINGS --]
menutrans Settings\ &Window				            Stillingar\ Glugga
menutrans &Global\ Settings				            Víðværar\ Stillingar
menutrans Startup\ &Settings				        Ræsistillingar

menutrans Toggle\ Pattern\ &Highlight<Tab>:set\ hls!	    Munsturauðkenning\ á\/af<Tab>:set\ hls!
menutrans Toggle\ &Ignoring\ Case<Tab>:set\ ic!		        Hunsa\ há-lágstafi\ á\/af<Tab>:set\ ic!
menutrans Toggle\ &Showing\ Matched\ Pairs<Tab>:set\ sm!	Sýna\ Pörun\ á\/af<Tab>:set\ sm!

menutrans &Context\ lines				            Samhengislínur

menutrans &Virtual\ Edit				            Skinbreytihamur
menutrans Never						                Aldrei
menutrans Block\ Selection				            Bálkval
menutrans Insert\ mode					            Innskotshamur
menutrans Block\ and\ Insert				        Bálkval\ og\ Innskotshamur
menutrans Always					                Alltaf
menutrans Toggle\ Insert\ &Mode<Tab>:set\ im!		Innskotshamur\ á\/af<Tab>:set\ im!
menutrans Toggle\ Vi\ C&ompatibility<Tab>:set\ cp!	Vi\ Samhæfanleiki\ á\/af<Tab>:set\ cp!

menutrans Search\ &Path\.\.\.				        Leita\ í\ Slóð\.\.\.
menutrans Ta&g\ Files\.\.\.				            Merkja\ Skrár\.\.\.

menutrans Toggle\ &Toolbar				            Tólaborð\ á\/af
menutrans Toggle\ &Bottom\ Scrollbar			    Neðri\ Skrunborði\ á\/af
menutrans Toggle\ &Left\ Scrollbar                  Vinstri\ Skrunborði\ á\/af
menutrans Toggle\ &Right\ Scrollbar			        Hægri\ Skrunborði\ á\/af

" Edit/File Settings
menutrans F&ile\ Settings				            Skráar-Stilingar

" Boolean options
menutrans Toggle\ Line\ &Numbering<Tab>:set\ nu!		Línunúmering\ á\/af<Tab>:set\ nu!
menutrans Toggle\ Relati&ve\ Line\ Numbering<Tab>:set\ rnu!	Afstæð\ Línunúmering\ á\/af<Tab>:set\ rnu!
menutrans Toggle\ &List\ Mode<Tab>:set\ list!			Listahamur\ á\/af<Tab>:set\ list!
menutrans Toggle\ Line\ &Wrapping<Tab>:set\ wrap!		Línuhlaup\ á\/af<Tab>:set\ wrap!
menutrans Toggle\ W&rapping\ at\ word<Tab>:set\ lbr!	Línuhlaup\ á\ orði\ á\/af<Tab>:set\ lbr!
menutrans Toggle\ Tab\ &Expanding<Tab>:set\ et!			Tab-víkkun\ á\/af<Tab>:set\ et!
menutrans Toggle\ &Auto\ Indenting<Tab>:set\ ai!			Sjálfvirkur\ Inndráttur\ á\/af<Tab>:set\ ai!
menutrans Toggle\ &C-Style\ Indenting<Tab>:set\ cin!	C-Inndráttur\ á\/af<Tab>:set\ cin!

" other options
menutrans &Shiftwidth					            Shiftbreidd
menutrans Soft\ &Tabstop			                Mjúk\ Tabstopp
menutrans Te&xt\ Width\.\.\.		                Textabreidd\.\.\.
menutrans &File\ Format\.\.\.		                Skráarform\.\.\.
menutrans C&olor\ Scheme			                Litaþema\.\.\.
menutrans &Keymap					                Lyklaskipan
" }}} EDIT / BREYTA

" {{{  TOOLS / TÓl
if has("spell")
    menutrans &Spelling					            Stafsetning
    menutrans &Spell\ Check\ On				        Villuleit\ á
    menutrans Spell\ Check\ &Off			        Villuleit\ af
    menutrans To\ &Next\ error<Tab>]s			    Næsta\ Villa<Tab>]s
    menutrans To\ &Previous\ error<Tab>[s		    Fyrri\ Villa<Tab>[s
    menutrans Suggest\ &Corrections<Tab>z=		    Leggja\ til\ Leiðréttingar<Tab>z=
    menutrans &Repeat\ correction<Tab>:spellrepall	Endurtaka\ Leiðréttingu<Tab>:spellrepall
    menutrans Set\ language\ to\ "en"			    Stilla\ Orðabók\ á "en"
    menutrans Set\ language\ to\ "en_au"		    Stilla\ Orðabók\ á "en_au"
    menutrans Set\ language\ to\ "en_ca"		    Stilla\ Orðabók\ á "en_ca"
    menutrans Set\ language\ to\ "en_gb"		    Stilla\ Orðabók\ á "en_gb"
    menutrans Set\ language\ to\ "en_nz"		    Stilla\ Orðabók\ á "en_nz"
    menutrans Set\ language\ to\ "en_us"		    Stilla\ Orðabók\ á "en_us"
    menutrans &Find\ More\ Languages			    Finna\ fleiri\ Orðabækur
endif
if has("folding")
  menutrans &Folding					            Földun
  " open close folds
  menutrans &Enable/Disable\ folds<Tab>zi		    Földun\ á\/af<Tab>zi
  menutrans &View\ Cursor\ Line<Tab>zv			    Sjá\ Línu\ Bendils<Tab>zv
  menutrans Vie&w\ Cursor\ Line\ only<Tab>zMzx	    Sjá\ Eingöngu\ Línu\ Bendils<Tab>zMzx
  menutrans C&lose\ more\ folds<Tab>zm			    Loka\ Fleiri\ Földunum<Tab>zm
  menutrans &Close\ all\ folds<Tab>zM			    Loka\ Öllum\ Földunum<Tab>zM
  menutrans O&pen\ more\ folds<Tab>zr			    Opna\ Fleiri\ Faldanir<Tab>zr
  menutrans &Open\ all\ folds<Tab>zR			    Opna\ Allar\ Faldanir<Tab>zR
  " fold method
  menutrans Fold\ Met&hod			                Földunar-háttur
  menutrans M&anual					                Handvirkur
  menutrans I&ndent					                Inndráttur
  menutrans E&xpression				                Segð
  menutrans S&yntax					                Málskipan
  menutrans &Diff					                Mismunur
  menutrans Ma&rker					                Merking
  " create and delete folds
  menutrans Create\ &Fold<Tab>zf			        Búa\ til\ Földun<Tab>zf
  menutrans &Delete\ Fold<Tab>zd			        Eyða\ Földun<Tab>zd
  menutrans Delete\ &All\ Folds<Tab>zD		        Eyða\ Öllum\ Földunum<Tab>zD
  " moving around in folds
  menutrans Fold\ Col&umn\ Width			        Breidd\ Földunar
endif  " has folding

if has("diff")
  menutrans &Diff					                Mismunur
  menutrans &Update					                Uppfæra
  menutrans &Get\ Block				                Sækja\ Bálk
  menutrans &Put\ Block				                Setja\ Bálk
endif

menutrans &Tools					                Tól
menutrans &Jump\ to\ this\ tag<Tab>g^]	    		Stökkva\ í\ Merki<Tab>g^]
menutrans Jump\ &back<Tab>^T			        	Stökkva\ til\ baka<Tab>^T
menutrans Build\ &Tags\ File			        	Búa\ til\ Merkjaskrá
menutrans &Make<Tab>:make				            Smíða<Tab>:make
menutrans &List\ Errors<Tab>:cl			        	Birta\ Villur<Tab>:cl
menutrans L&ist\ Messages<Tab>:cl!		        	Birta\ Skilaboð<Tab>:cl!
menutrans &Next\ Error<Tab>:cn			        	Næsta\ Villa<Tab>:cn
menutrans &Previous\ Error<Tab>:cp		        	Fyrri\ Villa<Tab>:cp
menutrans &Older\ List<Tab>:cold		        	Eldri\ Listi<Tab>:cold
menutrans N&ewer\ List<Tab>:cnew		        	Nýrri\ Listi<Tab>:cnew

menutrans Error\ &Window				            Villugluggi
menutrans Se&t\ Compiler				            Smiður
menutrans &Update<Tab>:cwin				            Uppfæra<Tab>:cwin
menutrans &Open<Tab>:copen				            Opna<Tab>:copen
menutrans &Close<Tab>:cclose				        Loka<Tab>:cclose

menutrans &Convert\ to\ HEX<Tab>:%!xxd			    Breyta\ í\ HEX<Tab>:%!xxd
menutrans Conve&rt\ back<Tab>:%!xxd\ -r			    Breyta\ til\ baka<Tab>:%!xxd\ -r
" }}}  TOOLS / TÓL

" {{{ SYNTAX / MÁLSKIPAN
menutrans &Syntax				                    Málskipan
menutrans &Show\ filetypes\ in\ menu		        Sýna\ Skráartegundir
menutrans Set\ '&syntax'\ only			            Stilla\ aðeins\ 'málskipan'\
menutrans Set\ '&filetype'\ too			            Stilla\ einnig\ 'skráartegund'\
menutrans &Off					                    Af
menutrans &Manual				                    Handvirkt
menutrans A&utomatic				                Sjálfvirkt
menutrans on/off\ for\ &This\ file		            á/af\ fyrir\ þessa\ skrá
menutrans Co&lor\ test				                Litaprófun
menutrans &Highlight\ test			                Auðkenningarprófun
menutrans &Convert\ to\ HTML			            Breyta\ í\ HTML
" }}} SYNTAX / MÁLSKIPAN

" {{{ BUFFERS / BIÐMINNI
menutrans &Buffers					                Biðminni
menutrans &Refresh\ menu			                Uppfæra\ valmynd
menutrans Delete					                Eyða
menutrans &Alternate				                Skipta
menutrans &Next						                Næsta
menutrans &Previous					                Fyrra
" }}} BUFFERS / BIÐMINNI

" {{{ WINDOW / GLUGGI
menutrans &Window			                        Gluggi
menutrans &New<Tab>^Wn			                    Nýr<Tab>^Wn
menutrans S&plit<Tab>^Ws		                    Splitta<Tab>^Ws
menutrans Split\ &Vertically<Tab>^Wv	            Splitta\ Lóðrétt<Tab>^Wv
menutrans Split\ File\ E&xplorer	                Splitta\ Skráarvafra
menutrans Sp&lit\ To\ #<Tab>^W^^	                Splitta\ í\ Flipa\ #<Tab>^W^^
menutrans &Close<Tab>^Wc		                    Loka\ Flipa<Tab>^Wc
menutrans Close\ &Other(s)<Tab>^Wo	                Loka\ Öðrum\ Flipum<Tab>^Wo
menutrans Ne&xt<Tab>^Ww			                    Næsti<Tab>^Ww
menutrans P&revious<Tab>^WW		                    Fyrri<Tab>^WW
menutrans &Equal\ Size<Tab>^W=		                Jafn\ Stór<Tab>^W=
menutrans &Max\ Height<Tab>^W_		                Hámarkshæð<Tab>^W_
menutrans M&in\ Height<Tab>^W1_		                Lágmarkshæð<Tab>^W1_
menutrans Max\ &Width<Tab>^W\|		                Hámarksbreidd<Tab>^W\|
menutrans Min\ Widt&h<Tab>^W1\|		                Lágmarksbreidd<Tab>^W1\|
menutrans Move\ &To			                        Færa
menutrans &Top<Tab>^WK			                    Upp<Tab>^WK
menutrans &Bottom<Tab>^WJ		                    Niður<Tab>^WJ
menutrans &Left\ side<Tab>^WH		                Til\ Vinstri<Tab>^WH
menutrans &Right\ side<Tab>^WL		                Til\ Hægri<Tab>^WL
menutrans Rotate\ &Up<Tab>^WR		                Rúlla\ upp<Tab>^WR
menutrans Rotate\ &Down<Tab>^Wr		                Rúlla\ niður<Tab>^Wr
menutrans Select\ Fo&nt\.\.\.		                Velja\ Leturgerð\.\.\.
" }}} WINDOW / GLUGGI

" {{{ HELP / HJÁLP
menutrans &Help			                            Hjálp
menutrans &Overview<Tab><F1>	                    Yfirlit<Tab><F1>
menutrans &User\ Manual		                        Notendahandbók
menutrans &How-to\ links	                        Hjálparhlekkir
menutrans &GUI			                            Myndrænt\ Viðmót
menutrans &Credits		                            Höfundar
menutrans Co&pying		                            Afritun
menutrans &Sponsor/Register                         Styrkja/Skráning
menutrans O&rphans		                            Góðgerðarstarf
menutrans &Find\.\.\.		                        Leit\.\.\.	" conflicts with Edit.Find
menutrans &Version		                            Útgáfa
menutrans &About		                            Um\ Forritið
" }}} HELP / HJÁLP

" {{{ POPUP
menutrans &Undo				                        Til\ Baka
menutrans Cu&t				                        Klippa
menutrans &Copy				                        Afrita
menutrans &Paste			                        Líma
menutrans &Delete			                        Eyða
menutrans Select\ Blockwise		                    Velja\ Bálkvíst
menutrans Select\ &Word			                    Velja\ Orð
menutrans Select\ &Sentence		                    Velja\ Setningu
menutrans Select\ Pa&ragraph	                    Velja\ Efnisgrein
menutrans Select\ &Line			                    Velja\ Línu
menutrans Select\ &Block		                    Velja\ Bálk
menutrans Select\ &All			                    Velja\ Allt
" }}} POPUP

" {{{ TOOLBAR
if has("toolbar")
  if exists("*Do_toolbar_tmenu")
    delfun Do_toolbar_tmenu
  endif
  fun Do_toolbar_tmenu()
    tmenu ToolBar.Open		                        Opna Skrá
    tmenu ToolBar.Save		                        Vista Skrá
    tmenu ToolBar.SaveAll	                        Vista Allar Skrár
    tmenu ToolBar.Print		                        Prenta
    tmenu ToolBar.Undo		                        Afturkalla
    tmenu ToolBar.Redo		                        Endurkalla
    tmenu ToolBar.Cut		                        Klippa
    tmenu ToolBar.Copy		                        Afrita
    tmenu ToolBar.Paste		                        Líma
    tmenu ToolBar.Find		                        Finna...
    tmenu ToolBar.FindNext	                        Finna Næsta
    tmenu ToolBar.FindPrev	                        Finna fyrri
    tmenu ToolBar.Replace	                        Finna og Skipta...
    if 0	" disabled; These are in the Windows menu
      tmenu ToolBar.New		                        Nýr
      tmenu ToolBar.WinSplit	                    Splitta Glugga
      tmenu ToolBar.WinMax	                        Hámarksstærð Glugga
      tmenu ToolBar.WinMin	                        Lágmarksstærð Glugga
      tmenu ToolBar.WinClose	                    Loka Glugga
    endif
    tmenu ToolBar.LoadSesn	                        Hlaða Setu
    tmenu ToolBar.SaveSesn	                        Vista Setu
    tmenu ToolBar.RunScript	                        Keyra Skriptu
    tmenu ToolBar.Make		                        Smíða
    tmenu ToolBar.Shell		                        Opna Skel
    tmenu ToolBar.RunCtags	                        Smíða Merki
    tmenu ToolBar.TagJump	                        Hoppa í Merki
    tmenu ToolBar.Help		                        Hjálp
    tmenu ToolBar.FindHelp	                        Finna Hjálp...
  endfun
endif
" }}} TOOLBAR

" {{{ DIALOG TEXTS
let g:menutrans_no_file =                           "[Engin Skrá]"
let g:menutrans_help_dialog =                       "Sláið inn skipun eða orða til að leita upplýsinga um:\n\nForskeytið i_ fyrir ílagshamsskipanir (t.d. i_CTRL-X)\nForskeytið c_ fyrir skipanalínuskipanir (t.d. c_<Del>)\nForskeytið ' fyrir nafn á valmöguleika (t.d. 'shiftbreidd')"
let g:menutrans_path_dialog =                       "Sláið inn leitarslóð fyrir skrár.\nAðskiljið möppur með kommu"
let g:menutrans_tags_dialog =                       "Sláið inn nafn Merkjaskráa.\nAðskiljið nöfnin með kommu"
let g:menutrans_textwidth_dialog =                  "Sláið inn nýja textabreidd (0 til að óvirkja sniðmátun): "
let g:menutrans_fileformat_dialog =                 "Veljið Skráarsnið"
" }}}

let &cpo = s:keepcpo
unlet s:keepcpo
