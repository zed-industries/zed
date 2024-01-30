" Menu Translations:	Danish
" Maintainer:		scootergrisen
" Last Change:		2022 Nov 17
" Original translations

" Quit when menu translations have already been done.
if exists("did_menu_trans")
  finish
endif
let did_menu_trans = 1
let s:keepcpo= &cpo
set cpo&vim

scriptencoding utf-8

" Help menu
menut &Help	Hjælp

menut &Overview<Tab><F1>	Overblik<Tab><F1>
menut &User\ Manual	Brugermanual
menut &How-to\ links	How-to-links
menut &Find\.\.\.	Find\.\.\.
" -SEP1-
menut &Credits	Anerkendelser
menut Co&pying	Kopiering
menut &Sponsor/Register	Sponsorer/registrer
menut O&rphans	Forældreløse\ børn
" -SEP2-
menut &Version	Version
menut &About	Om

let g:menutrans_help_dialog = "Indtast en kommando eller ord for at finde hjælp om:\n\nStart med i_ for kommandoer til inputtilstand (f.eks.: i_CTRL-X)\nStart med c_ for kommandoer til redigering af kommandolinje (f.eks.: c_<Del>)\nStart med ' for et tilvalgsnavn (f.eks.: 'shiftwidth')"

" File menu
menut &File	Fil

menut &Open\.\.\.<Tab>:e	Åbn\.\.\.<Tab>:e
menut Sp&lit-Open\.\.\.<Tab>:sp	Opdel-åbn\.\.\.<Tab>:sp
menut Open\ &Tab\.\.\.<Tab>:tabnew	Åbn\ faneblad\.\.\.<Tab>:tabnew
menut &New<Tab>:enew	Ny<Tab>:enew
menut &Close<Tab>:close	Luk<Tab>:close
" -SEP1-
menut &Save<Tab>:w	Gem<Tab>:w
menut Save\ &As\.\.\.<Tab>:sav	Gem\ som\.\.\.<Tab>:sav
" -SEP2-
menut Split\ &Diff\ with\.\.\.	Opdel\ diff\ med\.\.\.
menut Split\ Patched\ &By\.\.\.	Opdel\ patched\ af\.\.\.
" -SEP3-
menut &Print	Udskriv
" -SEP4-
menut Sa&ve-Exit<Tab>:wqa	Gem-afslut
menut E&xit<Tab>:qa	Afslut

" Edit menu
menut &Edit	Rediger

menut &Undo<Tab>u	Fortryd<Tab>u
menut &Redo<Tab>^R	Omgør<Tab>^R
menut Rep&eat<Tab>\.	Gentag<Tab>\.
" -SEP1-
menut Cu&t<Tab>"+x	Klip<Tab>"+x
menut &Copy<Tab>"+y	Kopiér<Tab>"+y
menut &Paste<Tab>"+gP	Indsæt<Tab>"+gP
menut Put\ &Before<Tab>[p	Indsæt\ inden\ (put)<Tab>[p
menut Put\ &After<Tab>]p	Indsæt\ efter\ (put)<Tab>]p
menut &Delete<Tab>x	Slet<Tab>x
menut &Select\ all<Tab>ggVG	Markér\ alt<Tab>ggVG
" -SEP2-
menut &Find\.\.\.	Find\.\.\.
menut &Find\.\.\.<Tab>/	Find\.\.\.<Tab>/
menut Find\ and\ Rep&lace\.\.\.	Find\ og\ erstat\.\.\.
menut Find\ and\ Rep&lace\.\.\.<Tab>:%s	Find\ og\ erstat\.\.\.<Tab>:%s
menut Find\ and\ Rep&lace\.\.\.<Tab>:s	Find\ og\ erstat\.\.\.<Tab>:s
" -SEP3-
menut Settings\ &Window	Indstillinger-vindue
menut Startup\ &Settings	Opstartsindstillinger
menut &Global\ Settings	Globale\ indstillinger
menut Question	Spørgsmål

" Edit

menut Toggle\ Pattern\ &Highlight<Tab>:set\ hls!	Fremhævning\ af\ mønster\ til/fra<Tab>:set\ hls!
menut Toggle\ &Ignoring\ Case<Tab>:set\ ic!	Ignorerer\ forskel\ på\ store\ og\ små\ bogstaver\ til/fra<Tab>:set\ ic!
menut Toggle\ &Showing\ Matched\ Pairs<Tab>:set\ sm!	Viser\ matchende\ par\ til/fra<Tab>:set\ sm!

menut &Context\ lines	Kontekstlinjer

menut &Virtual\ Edit	Virtuel\ redigering
menut Never	Aldrig
menut Block\ Selection	Blokmarkering
menut Insert\ mode	Indsæt-tilstand
menut Block\ and\ Insert	Blok\ og\ indsæt
menut Always	Altid

menut Toggle\ Insert\ &Mode<Tab>:set\ im!	Indsæt-tilstand\ til/fra<Tab>:set\ im!
menut Toggle\ Vi\ C&ompatibility<Tab>:set\ cp!	Vi-kompatibel\ til/fra<Tab>:set\ cp!
menut Search\ &Path\.\.\.	Søgesti\.\.\.
menut Ta&g\ Files\.\.\.	Tag-filer\.\.\.
" -SEP1-
menut Toggle\ &Toolbar	Værktøjslinje\ til/fra
menut Toggle\ &Bottom\ Scrollbar	Nederste\ rullebjælke\ til/fra
menut Toggle\ &Left\ Scrollbar	Venstre\ rullebjælke\ til/fra
menut Toggle\ &Right\ Scrollbar	Højre\ rullebjælke\ til/fra

let g:menutrans_path_dialog = "Indtast søgesti til filer.\nSeparer mappenavne med et komma."
let g:menutrans_tags_dialog = "Indtast navne på tag-filer.\nSeparer navnene med et komma."

" Edit/File Settings
menut F&ile\ Settings	Filindstillinger

" Boolean options
menut Toggle\ Line\ &Numbering<Tab>:set\ nu!	Linjenummerering\ til/fra<Tab>:set\ nu!
menut Toggle\ Relati&ve\ Line\ Numbering<Tab>:set\ rnu!	Relativ\ linjenummerering\ til/fra<Tab>:set\ rnu!
menut Toggle\ &List\ Mode<Tab>:set\ list!	Listetilstand\ til/fra<Tab>:set\ list!
menut Toggle\ Line\ &Wrapping<Tab>:set\ wrap!	Linjeombrydning\ til/fra<Tab>:set\ wrap!
menut Toggle\ W&rapping\ at\ word<Tab>:set\ lbr!	Ombrydning\ ved\ ord\ til/fra<Tab>:set\ lbr!
menut Toggle\ Tab\ &expanding<Tab>:set\ et!	Udvidelse\ af\ tabulator\ til/fra<Tab>:set\ et!
menut Toggle\ &Auto\ Indenting<Tab>:set\ ai!	Automatisk\ indrykning\ til/fra<Tab>:set\ ai!
menut Toggle\ &C-Style\ Indenting<Tab>:set\ cin!	Indrykning\ i\ &C-stil\ til/fra<Tab>:set\ cin!
" -SEP2-
menut &Shiftwidth	Shiftwidth
" menut &Shiftwidth.2<Tab>:set\ sw=2\ sw?<CR>	Shiftwidth.2<Tab>:set\ sw=2\ sw?<CR>
" menut &Shiftwidth.3<Tab>:set\ sw=3\ sw?<CR>	Shiftwidth.3<Tab>:set\ sw=3\ sw?<CR>
" menut &Shiftwidth.4<Tab>:set\ sw=4\ sw?<CR>	Shiftwidth.4<Tab>:set\ sw=4\ sw?<CR>
" menut &Shiftwidth.5<Tab>:set\ sw=5\ sw?<CR>	Shiftwidth.5<Tab>:set\ sw=5\ sw?<CR>
" menut &Shiftwidth.6<Tab>:set\ sw=6\ sw?<CR>	Shiftwidth.6<Tab>:set\ sw=6\ sw?<CR>
" menut &Shiftwidth.8<Tab>:set\ sw=8\ sw?<CR>	Shiftwidth.8<Tab>:set\ sw=8\ sw?<CR>
menut Soft\ &Tabstop	Blødt\ tabulatorstop
" menut Soft\ &Tabstop.2<Tab>:set\ sts=2\ sts?	Blødt\ Tabstop.2<Tab>:set\ sts=2\ sts?
" menut Soft\ &Tabstop.3<Tab>:set\ sts=3\ sts?	Blødt\ Tabstop.3<Tab>:set\ sts=3\ sts?
" menut Soft\ &Tabstop.4<Tab>:set\ sts=4\ sts?	Blødt\ Tabstop.4<Tab>:set\ sts=4\ sts?
" menut Soft\ &Tabstop.5<Tab>:set\ sts=5\ sts?	Blødt\ Tabstop.5<Tab>:set\ sts=5\ sts?
" menut Soft\ &Tabstop.6<Tab>:set\ sts=6\ sts?	Blødt\ Tabstop.6<Tab>:set\ sts=6\ sts?
" menut Soft\ &Tabstop.8<Tab>:set\ sts=8\ sts?	Blødt\ Tabstop.8<Tab>:set\ sts=8\ sts?
menut Te&xt\ Width\.\.\.	Tekstbredde\.\.\.
menut &File\ Format\.\.\.	Filformat\.\.\.

let g:menutrans_textwidth_dialog = "Indtast ny tekstbredde (0 for at deaktivere formatering): "
let g:menutrans_fileformat_dialog = "Vælg format til skrivning af filen"
let g:menutrans_fileformat_choices = "&Unix\n&Dos\n&Mac\n&Annuller"

menut Show\ C&olor\ Schemes\ in\ Menu	Vis\ farveskemaer\ i\ menu
menut C&olor\ Scheme	Farveskema

" menut blue			blå
" menut darkblue		mørkeblå
" menut desert		ørken
" menut elflord		elverherre
" menut evening		aften
" menut industry		industri
" menut morning		morgen
" menut peachpuff	fersken
" menut shine		skær
" menut slate		skiffer
" menut default		standard
" menut torte		tærte
" menut zellner		???
" menut delek		???
" menut koehler		???
" menut murphy		???
" menut pablo		???
" menut ron			ron

menut Show\ &Keymaps\ in\ Menu	Vis\ tastaturlayouts\ i\ menu
menut &Keymap	Tastaturlayout

menut None	Intet
" menut accents	Diakritiske\ tegn
" menut arabic	arabisk
" menut armenian-eastern	armensk\ (østlig)
" menut armenian-western	armensk\ (vestlig)
" menut belarusian-jcuken	hviderussisk\ [belarusian-jcuken]
" menut czech	tjekkisk
" menut greek	græsk
" menut hebrew	hebraisk
" menut hebrewp	hebraisk\ [hebrewp]
" menut magyar	ungarsk
" menut persian	persisk
" menut serbian	serbisk
" menut serbian-latin	serbisk\ (latinsk)
" menut slovak	slovakisk

menut Select\ Fo&nt\.\.\.	Vælg\ skrifttype\.\.\.

" Programming menu
menut &Tools	Værktøjer

menut &Jump\ to\ this\ tag<Tab>g^]	Hop\ til\ tagget<Tab>g^]
menut Jump\ &back<Tab>^T	Hop\ tilbage<Tab>^T
menut Build\ &Tags\ File	Build\ tags-fil
" -SEP1-
" Tools.Spelling Menu
menut &Spelling	Stavning
menut &Spell\ Check\ On	Stavekontrol\ til
menut Spell\ Check\ &Off	Stavekontrol\ fra
menut To\ &Next\ error<Tab>]s	Til\ næste\ fejl<Tab>]s
menut To\ &Previous\ error<Tab>[s	Til\ forrige\ fejl<Tab>[s
menut Suggest\ &Corrections<Tab>z=	Foreslå\ rettelse<Tab>z=
menut &Repeat\ correction<Tab>:spellrepall	Gentag\ rettelse<Tab>:spellrepall
menut Set\ language\ to\ "en"	Sæt\ sprog\ til\ "en"
menut Set\ language\ to\ "en_au"	Sæt\ sprog\ til\ "en_au"
menut Set\ language\ to\ "en_ca"	Sæt\ sprog\ til\ "en_ca"
menut Set\ language\ to\ "en_gb"	Sæt\ sprog\ til\ "en_gb"
menut Set\ language\ to\ "en_nz"	Sæt\ sprog\ til\ "en_nz"
menut Set\ language\ to\ "en_us"	Sæt\ sprog\ til\ "en_us"
menut &Find\ More\ Languages	Find\ flere\ sprog

" Tools.Fold Menu
menut &Folding	Foldning
" open close folds
menut &Enable/Disable\ folds<Tab>zi	Aktivér/deaktivér\ sammenfoldninger<Tab>zi
menut &View\ Cursor\ Line<Tab>zv	Vis\ markørlinje<Tab>zv
menut Vie&w\ Cursor\ Line\ only<Tab>zMzx	Vis\ kun\ markørlinje<Tab>zMzx
menut C&lose\ more\ folds<Tab>zm	Luk\ flere\ sammenfoldninger<Tab>zm
menut &Close\ all\ folds<Tab>zM	Luk\ alle\ sammenfoldninger<Tab>zM
menut O&pen\ more\ folds<Tab>zr	Åbn\ flere\ sammenfoldninger<Tab>zr
menut &Open\ all\ folds<Tab>zR	Åbn\ alle\ sammenfoldninger<Tab>zR
" fold method
" -SEP1-
menut Fold\ Met&hod	Sammenfoldningsmetode
menut M&anual	Manuelt
menut I&ndent	Indryk
menut E&xpression	Udtryk
menut S&yntax	Syntaks
menut &Diff	Diff
menut Ma&rker	Markør
" create and delete folds
menut Create\ &Fold<Tab>zf	Opret\ sammenfoldning<Tab>zf
menut &Delete\ Fold<Tab>zd	Slet\ sammenfoldning<Tab>zd
menut Delete\ &All\ Folds<Tab>zD	Slet\ alle\ sammenfoldninger<Tab>zD
" moving around in folds
" -SEP2-
menut Fold\ col&umn\ width	Kolonnebredde\ for\ sammenfoldning

menut &Diff	Diff
"
menut &Update	Opdater
menut &Get\ Block	Hent\ blok\ (get)
menut &Put\ Block	Indsæt\ blok\ (put)

" -SEP2-
menut &Make<Tab>:make	&Make<Tab>:make

menut &List\ Errors<Tab>:cl	Oplist\ fejl<Tab>:cl
menut L&ist\ Messages<Tab>:cl!	Oplist\ meddelelser<Tab>:cl!
menut &Next\ Error<Tab>:cn	Næste\ fejl<Tab>:cn
menut &Previous\ Error<Tab>:cp	Forrige\ fejl<Tab>:cp
menut &Older\ List<Tab>:cold	Ældre\ liste<Tab>:cold
menut N&ewer\ List<Tab>:cnew	Nyere\ liste<Tab>:cnew

menut Error\ &Window	Fejl-vindue

menut &Update<Tab>:cwin	Opdater<Tab>:cwin
menut &Open<Tab>:copen	Åbn<Tab>:copen
menut &Close<Tab>:cclose	Luk<Tab>:cclose

" -SEP3-
menut &Convert\ to\ HEX<Tab>:%!xxd	Konvertér\ til\ HEX<Tab>:%!xxd
menut Conve&rt\ back<Tab>:%!xxd\ -r	Konvertér\ tilbage<Tab>:%!xxd\ -r

menut Se&T\ Compiler	Sæt\ kompiler

" Buffers menu
menut &Buffers	Buffere

menut &Refresh\ menu	Genopfrisk\ menu
menut &Delete	Slet
menut &Alternate	Skift
menut &Next	Næste
menut &Previous	Forrige
menut [No\ File]	[Ingen\ fil]

" Syntax menu
menut &Syntax	Syntaks

menut &Show\ File\ Types\ in\ menu	Vis\ filtyper\ i\ menu
menut Set\ '&syntax'\ only	Sæt\ kun\ 'syntax'
menut Set\ '&filetype'\ too	Sæt\ også\ 'filetype'
menut &Off	Fra
menut &Manual	Manuelt
menut A&utomatic	Automatisk
menut On/Off\ for\ &This\ File	Til/fra\ for\ denne\ fil
menut Co&lor\ test	Farvetest
menut &Highlight\ test	Fremhævningstest
menut &Convert\ to\ HTML	Konvertér\ til\ HTML

let g:menutrans_no_file = "[Ingen fil]"

" Window menu
menut &Window	Vindue

menut &New<Tab>^Wn	Nyt<Tab>^Wn
menut S&plit<Tab>^Ws	Opdel<Tab>^Ws
menut Sp&lit\ To\ #<Tab>^W^^	Opdel\ til\ #<Tab>^W^^
menut Split\ &Vertically<Tab>^Wv	Opdel\ lodret<Tab>^Wv
menut Split\ File\ E&xplorer	Opdel\ filbrowser
" -SEP1-
menut &Close<Tab>^Wc	Luk<Tab>^Wc
menut Close\ &Other(s)<Tab>^Wo	Luk\ andre<Tab>^Wo
" -SEP2-
menut Move\ &To	Flyt\ til

menut &Top<Tab>^WK	Øverst<Tab>^WK
menut &Bottom<Tab>^WJ	Nederst<Tab>^WJ
menut &Left\ side<Tab>^WH	Venstre\ side<Tab>^WH
menut &Right\ side<Tab>^WL	Højre\ side<Tab>^WL
menut Rotate\ &Up<Tab>^WR	Roter\ op<Tab>^WR
menut Rotate\ &Down<Tab>^Wr	Roter\ ned<Tab>^Wr
" -SEP3-
menut &Equal\ Size<Tab>^W=	Samme\ størrelse<Tab>^W=
menut &Max\ Height<Tab>^W_	Maks\.\ højde<Tab>^W_
menut M&in\ Height<Tab>^W1_	Min\.\ højde<Tab>^W1_
menut Max\ &Width<Tab>^W\|	Maks\.\ bredde<Tab>^W\|
menut Min\ Widt&h<Tab>^W1\|	Min\.\ bredde<Tab>^W1\|

" The popup menu
menut &Undo	Fortryd
" -SEP1-
menut Cu&t	Klip
menut &Copy	Kopiér
menut &Paste	Indsæt
menut &Delete	Slet
" -SEP2-
menut Select\ Blockwise		Markér\ blokvis
menut Select\ &Word	Markér\ ord

menut Select\ &Sentence	Markér\ sætning
menut Select\ Pa&ragraph	Markér\ afsnit

menut Select\ &Line	Markér\ linje
menut Select\ &Block		Markér\ blok
menut Select\ &All	Markér\ alt

" The GUI toolbar
if has("toolbar")
  if exists("*Do_toolbar_tmenu")
    delfun Do_toolbar_tmenu
  endif
  fun Do_toolbar_tmenu()
  tmenu ToolBar.Open		Åbn fil
  tmenu ToolBar.Save		Gem nuværende fil
  tmenu ToolBar.SaveAll		Gem alle filer
  tmenu ToolBar.Print		Udskriv
  tmenu ToolBar.Undo		Fortryd
  tmenu ToolBar.Redo		Omgør
  tmenu ToolBar.Cut		Klip til udklipsholder
  tmenu ToolBar.Copy		Kopiér til udklipsholder
  tmenu ToolBar.Paste		Indsæt fra udklipsholder
  if !has("gui_athena")
    tmenu ToolBar.Replace	Find/erstat...
    tmenu ToolBar.FindNext	Find næste
    tmenu ToolBar.FindPrev	Find forrige
  endif
  tmenu ToolBar.LoadSesn	Vælg en session som skal indlæses
  tmenu ToolBar.SaveSesn	Gem nuværende session
  tmenu ToolBar.RunScript	Vælg et Vim-script som skal køres
  tmenu ToolBar.Make		Make nuværende projekt (:make)
  tmenu ToolBar.RunCtags	Build tags i nuværende mappetræ (!ctags -R .)
  tmenu ToolBar.TagJump		Hop til tag under markør
  tmenu ToolBar.Help		Vim hjælp
  tmenu ToolBar.FindHelp	Søg i Vim hjælp
  endfun
endif

let g:menutrans_set_lang_to = "Sæt sprog til"

" stavegenvejsmenu pop op ting
let g:menutrans_spell_change_ARG_to = 'Ændr\ "%s"\ til'
let g:menutrans_spell_add_ARG_to_word_list = 'Tilføj\ "%s"\ til\ ordliste'
let g:menutrans_spell_ignore_ARG = 'Ignorer "%s"'



" Forsøg på at oversætte netrw-menuen
menut Help<tab><F1>					Hjælp<tab><F1>
" -Sep1-
menut Go\ Up\ Directory<tab>-					Gå\ mappe\ op<tab>-
menut Apply\ Special\ Viewer<tab>x					Anvend\ speciel\ fremviser<tab>x

menut Bookmarks\ and\ History					Bogmærker\ og\ historik<tab>:echo "(disabled)"
menut Bookmark\ Current\ Directory<tab>mb					Sæt\ bogmærke\ for\ nuværende\ mappe<tab>mb
menut Goto\ Prev\ Dir\ (History)<tab>u					Gå\ til\ forrige\ mappe\ (historik)<tab>u
menut Goto\ Next\ Dir\ (History)<tab>U					Gå\ til\ næste\ mappe\ (historik)<tab>U
menut List<tab>qb					Oplist<tab>qb

menut Browsing\ Control					Gennemgangskontol
menut Horizontal\ Split<tab>o					Vandret\ opdeling<tab>o
menut Vertical\ Split<tab>v					Lodret\ opdeling<tab>v
menut New\ Tab<tab>t					Nyt\ faneblad<tab>t
menut Preview<tab>p					Forhåndsvis<tab>p
menut Edit\ File\ Hiding\ List<tab><ctrl-h>               Rediger\ liste\ til\ filskjulning
menut Edit\ Sorting\ Sequence<tab>S					Rediger\ sorteringssekvens<tab>S
menut Quick\ Hide/Unhide\ Dot\ Files<tab>gh					Hurtig\ skjul/vis\ punktum-filer<tab>gh
menut Refresh\ Listing<tab><ctrl-l>							Genopfrisk\ oplistning<tab>\<c-l>\ ikke\ sikker\ det\ med\ er\ korrekt
menut Settings/Options<tab>:NetrwSettings					Indstillinger/valgmuligheder<tab>

menut Delete\ File/Directory<tab>D	Slet\ fil/mappe<tab>D

menut Edit\ File/Dir					Rediger\ fil/mappe
menut Create\ New\ File<tab>%					Opret\ ny\ fil<tab>%
menut In\ Current\ Window<tab><cr>					I\ nuværende\ vindue<tab>
menut Preview\ File/Directory<tab>p					Forhåndsvis\ fil/mappe<tab>p
menut In\ Previous\ Window<tab>P					I\ forrige\ vindue<tab>P
menut In\ New\ Window<tab>o					I\ nyt\ vindue<tab>o
menut In\ New\ Tab<tab>t					I\ nyt\ faneblad<tab>t
menut In\ New\ Vertical\ Window<tab>v					I\ nyt\ lodret\ vindue<tab>v

menut Explore					Gennemse
menut Directory\ Name					Mappenavn<tab>:Explore
menut Filenames\ Matching\ Pattern\ (curdir\ only)<tab>:Explore\ */					test29<tab>:Explore\ */
menut Filenames\ Matching\ Pattern\ (+subdirs)<tab>:Explore\ **/					test30<tab>:Explore\ **/
menut Files\ Containing\ String\ Pattern\ (curdir\ only)<tab>:Explore\ *//					test31<tab>:Explore\ *//
menut Files\ Containing\ String\ Pattern\ (+subdirs)<tab>:Explore\ **//					test32<tab>:Explore\ **//
menut Next\ Match<tab>:Nexplore					Næste\ match<tab>:Nexplore<cr>
menut Prev\ Match<tab>:Pexplore					Forrige\ match<tab>:Pexplore<cr>

menut Make\ Subdirectory<tab>d					Opret\ undermappe<tab>d

menut Marked\ Files					Mærkede\ filer
menut Mark\ File<tab>mf					Mærk\ fil<tab>mf
menut Mark\ Files\ by\ Regexp<tab>mr					Mærk\ filer\ efter\ regulært\ udtrk<tab>mr
menut Hide-Show-List\ Control<tab>a					test38<tab>a
menut Copy\ To\ Target<tab>mc					Kopiér\ til\ mål<tab>mc
menut Delete<tab>D					Slet<tab>D
menut Diff<tab>md					Diff<tab>md
menut Edit<tab>me					Rediger<tab>me
menut Exe\ Cmd<tab>mx					test43<tab>mx
menut Move\ To\ Target<tab>mm					Flyt\ til\ mål<tab>mm
menut Obtain<tab>O					Indhent<tab>O
menut Print<tab>mp					Udskriv<tab>mp
menut Replace<tab>R					Erstat<tab>R
menut Set\ Target<tab>mt					Sæt\ mål<tab>mt
menut Tag<tab>mT					test49<tab>mT
menut Zip/Unzip/Compress/Uncompress<tab>mz					Zip/unzip/komprimér/udpak<tab>mz

menut Obtain\ File<tab>O					Indhent\ fil<tab>O

menut Style					Stile
menut Listing				Oplisting
menut thin<tab>i	tynd
menut long<tab>i	lang
menut wide<tab>i	bred
menut tree<tab>i	træ
menut Normal-Hide-Show					Normal-skjul-vis
menut Show\ All<tab>a					Vis\ alle<tab>
menut Normal<tab>a					Normal<tab>
menut Hidden\ Only<tab>a					Kun\ skulte<tab>
menut Reverse\ Sorting\ Order<tab>         Omvendt\ sorteringsrækkefølge
menut Sorting\ Method					Sorteringsmetode
menut Name<tab>s       Navn
menut Time<tab>s       Tidspunkt
menut Size<tab>s       Størrelse
menut Exten<tab>s      Endelse
menut Rename\ File/Directory<tab>R	Omdøb\ fil/mappe<tab>R
menut Set\ Current\ Directory<tab>c	Sæt\ nuværende\ mappe<tab>c

menut History					Historik

menut Targets					Mål

let &cpo = s:keepcpo
unlet s:keepcpo

" vim: set sw=2 :
