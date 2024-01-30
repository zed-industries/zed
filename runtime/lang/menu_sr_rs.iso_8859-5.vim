" Menu Translations: Serbian
" Maintainer: Aleksandar Jelenak <ajelenak AT yahoo.com>
" Last Change:	Fri, 30 May 2003 10:17:39 Eastern Daylight Time
" Adapted for VIM 8 by: Иван Пешић on 2017-12-28 12:05+0400
" Generated from menu_sr_rs.utf-8.vim, DO NOT EDIT

" Quit when menu translations have already been done.
if exists("did_menu_trans")
  finish
endif
let did_menu_trans = 1
let s:keepcpo= &cpo
set cpo&vim

scriptencoding iso-8859-5

" Help menu
menutrans &Help		      Помо&ћ
menutrans &Overview<Tab><F1>  &Преглед<Tab><F1>
menutrans &User\ Manual       &Упутство\ за\ кориснике
menutrans &How-to\ links      &Како\ да\.\.\.
menutrans &Find		      &Нађи
menutrans &Credits	      &Заслуге
menutrans Co&pying	      П&реузимање
menutrans O&rphans	      &Сирочићи
menutrans &Sponsor/Register   Спонзор/&Региструјте\ се
menutrans &Version	      &Верзија
menutrans &About	      &О\ програму

" File menu
menutrans &File			    &Фајл
menutrans &Open\.\.\.<Tab>:e	    &Отвори\.\.\.<Tab>:e
menutrans Sp&lit-Open\.\.\.<Tab>:sp &Подели-отвори\.\.\.<Tab>:sp
menutrans Open\ &Tab\.\.\.<Tab>:tabnew	Отвори\ картицу\.\.\.<Tab>:tabnew
menutrans &New<Tab>:enew	    &Нов<Tab>:enew
menutrans &Close<Tab>:close	    &Затвори<Tab>:close
menutrans &Save<Tab>:w		    &Сачувај<Tab>:w
menutrans Save\ &As\.\.\.<Tab>:sav  Сачувај\ &као\.\.\.<Tab>:sav
menutrans Split\ &Diff\ with\.\.\.  Подели\ и\ &упореди\ са\.\.\.
menutrans Split\ Patched\ &By\.\.\. По&дели\ и\ преправи\ са\.\.\.
menutrans &Print		    Шта&мпај
menutrans Sa&ve-Exit<Tab>:wqa	    Сачувај\ и\ за&врши<Tab>:wqa
menutrans E&xit<Tab>:qa		    К&рај<Tab>:qa

" Edit menu
menutrans &Edit			 &Уређивање
menutrans &Undo<Tab>u		 &Поништи<Tab>u
menutrans &Redo<Tab>^R		 &Врати\ измену<Tab>^R
menutrans Rep&eat<Tab>\.	 П&онови<Tab>\.
menutrans Cu&t<Tab>"+x		 Исе&ци<Tab>"+x
menutrans &Copy<Tab>"+y		 &Копирај<Tab>"+y
menutrans &Paste<Tab>"+gP	 &Убаци<Tab>"+gP
menutrans &Paste<Tab>"+P	&Убаци<Tab>"+P
menutrans Put\ &Before<Tab>[p	 Стави\ испре&д<Tab>[p
menutrans Put\ &After<Tab>]p	 Стави\ &иза<Tab>]p
menutrans &Delete<Tab>x		 Из&бриши<Tab>x
menutrans &Select\ all<Tab>ggVG  Изабери\ св&е<Tab>ggVG
menutrans &Find\.\.\.		 &Нађи\.\.\.
menutrans Find\ and\ Rep&lace\.\.\. Нађи\ и\ &замени\.\.\.
menutrans Settings\ &Window	 П&розор\ подешавања
menutrans Startup\ &Settings	 По&дешавања\ при\ покретању		
menutrans &Global\ Settings	 Оп&шта\ подешавања
menutrans F&ile\ Settings	 Подешавања\ за\ фај&лове
menutrans &Shiftwidth		 &Корак\ увлачења
menutrans Soft\ &Tabstop	 &Мека\ табулација
menutrans Te&xt\ Width\.\.\.	 &Ширина\ текста\.\.\.
menutrans &File\ Format\.\.\.	 &Врста\ фајла\.\.\.
menutrans Show\ C&olor\ Schemes\ in\ Menu	Прикажи\ шеме\ бо&ја\ у\ менију
menutrans C&olor\ Scheme	\Шеме\ бо&ја
menutrans Show\ &Keymaps\ in\ Menu	Прикажи\ прес&ликавања\ тастатуре\ у\ менију
menutrans &Keymap	Прес&ликавања\ тастатуре
menutrans Select\ Fo&nt\.\.\.	 Избор\ &фонта\.\.\.

" Edit/Global Settings
menutrans Toggle\ Pattern\ &Highlight<Tab>:set\ hls! Истицање\ &шаблона\ (да/не)<Tab>:set\ hls!
menutrans Toggle\ &Ignoring\ Case<Tab>:set\ ic! Занемари\ величину\ &слова\ (да/не)<Tab>:set\ ic!
menutrans Toggle\ &Showing\ Matched\ Pairs<Tab>:set\ sm! Прикажи\ упарену\ &заграду\ (да/не)<Tab>:set\ sm!
menutrans &Context\ lines  Видљиви\ &редови
menutrans &Virtual\ Edit   Виртуелно\ &уређивање
menutrans Toggle\ Insert\ &Mode<Tab>:set\ im!   Режим\ У&метање\ (да/не)<Tab>:set\ im!
menutrans Toggle\ Vi\ C&ompatibility<Tab>:set\ cp!     '&Vi'\ сагласно\ (да/не)<Tab>:set\ cp!
menutrans Search\ &Path\.\.\. Путања\ &претраге\.\.\.
menutrans Ta&g\ Files\.\.\.   &Фајлови\ ознака\.\.\.
menutrans Toggle\ &Toolbar    Линија\ са\ &алаткама\ (да/не)
menutrans Toggle\ &Bottom\ Scrollbar   Доња\ л&инија\ клизања\ (да/не)
menutrans Toggle\ &Left\ Scrollbar  &Лева\ линија\ клизања\ (да/не)
menutrans Toggle\ &Right\ Scrollbar &Десна\ линија\ клизања\ (да/не)

" Edit/Global Settings/Virtual Edit
menutrans Never		      Никад
menutrans Block\ Selection    Избор\ блока
menutrans Insert\ mode	      Режим\ Уметање
menutrans Block\ and\ Insert  Блок\ и\ Уметање
menutrans Always	      Увек

" Edit/File Settings
menutrans Toggle\ Line\ &Numbering<Tab>:set\ nu!   Прикажи\ &нумерацију\ линија\ (да/не)<Tab>:set\ nu!
menutrans Toggle\ Relati&ve\ Line\ Numbering<Tab>:set\ rnu!   Прикажи\ Релати&вну\ нумерацију\ линија\ (да/не)<Tab>:set\ rnu!
menutrans Toggle\ &List\ Mode<Tab>:set\ list!	   Режим\ &листе\ (да/не)<Tab>:set\ list!
menutrans Toggle\ Line\ &Wrapping<Tab>:set\ wrap!	   Обавијање\ &редова\ (да/не)<Tab>:set\ wrap!
menutrans Toggle\ W&rapping\ at\ Word<Tab>:set\ lbr!   Преломи\ &на\ реч\ (да/не)<Tab>:set\ lbr!
menutrans Toggle\ Tab\ &Expanding<Tab>:set\ et!	   Размаци\ уместо\ &табулације\ (да/не)<Tab>:set\ et!
menutrans Toggle\ &Auto\ Indenting<Tab>:set\ ai!	Ауто-&увлачење\ (да/не)<Tab>:set\ ai!
menutrans Toggle\ &C-Style\ Indenting<Tab>:set\ cin!	   &C-увлачење\ (да/не)<Tab>:set\ cin!

" Edit/Keymap
menutrans None Без\ пресликавања

" Tools menu
menutrans &Tools	&Алатке
menutrans &Jump\ to\ this\ tag<Tab>g^] Скочи\ на\ &ову\ ознаку<Tab>g^]
menutrans Jump\ &back<Tab>^T	 Скочи\ &натраг<Tab>^T
menutrans Build\ &Tags\ File	 Изгради\ &фајл\ ознака
menutrans &Spelling	 Пра&вопис
menutrans &Folding	      &Подвијање
menutrans Create\ &Fold<Tab>zf		  С&твори\ свијутак<Tab>zf
menutrans &Delete\ Fold<Tab>zd		  О&бриши\ свијутак<Tab>zd
menutrans Delete\ &All\ Folds<Tab>zD	  Обриши\ све\ св&ијутке<Tab>zD
menutrans Fold\ column\ &width		  Ширина\ &реда\ цвијутка
"menutrans &Diff		      &Упоређивање
menutrans &Make<Tab>:make     'mak&е'<Tab>:make
menutrans &List\ Errors<Tab>:cl     Списак\ &грешака<Tab>:cl
menutrans L&ist\ Messages<Tab>:cl!  Сп&исак\ порука<Tab>:cl!
menutrans &Next\ Error<Tab>:cn	    С&ледећа\ грешка<Tab>:cn
menutrans &Previous\ Error<Tab>:cp  Пре&тходна\ грешка<Tab>:cp
menutrans &Older\ List<Tab>:cold    Стари\ списа&к<Tab>:cold
menutrans N&ewer\ List<Tab>:cnew    Но&ви\ списак<Tab>:cnew
menutrans Error\ &Window	    Прозор\ са\ г&решкама
menutrans Se&t\ Compiler	    И&забери\ преводиоца
menutrans &Convert\ to\ HEX<Tab>:%!xxd	   Претвори\ у\ &ХЕКС<Tab>:%!xxd
menutrans Conve&rt\ back<Tab>:%!xxd\ -r    Вр&ати\ у\ првобитан\ облик<Tab>:%!xxd\ -r
menutrans Show\ Compiler\ Se&ttings\ in\ Menu	Прикажи\ поде&шавања\ преводиоца\ у\ менију

" Tools/Spelling
menutrans &Spell\ Check\ On	&Укључи\ проверу\ правописа
menutrans Spell\ Check\ &Off	&Искључи\ проверу\ правописа
menutrans To\ &Next\ Error<Tab>]s	Иди\ на\ &следећу\ грешку<Tab>]s
menutrans To\ &Previous\ Error<Tab>[s	Иди\ на\ &претходну\ грешку<Tab>[s
menutrans Suggest\ &Corrections<Tab>z=	Предложи\ исп&равке<Tab>z=
menutrans &Repeat\ Correction<Tab>:spellrepall	П&онови\ исправку<Tab>:spellrepall
menutrans Set\ Language\ to\ "en"	Постави\ језик\ на\ "en"
menutrans Set\ Language\ to\ "en_au" 	Постави\ језик\ на\ "en_au"
menutrans Set\ Language\ to\ "en_ca" 	Постави\ језик\ на\ "en_ca"
menutrans Set\ Language\ to\ "en_gb" 	Постави\ језик\ на\ "en_gb"
menutrans Set\ Language\ to\ "en_nz" 	Постави\ језик\ на\ "en_nz"
menutrans Set\ Language\ to\ "en_us" 	Постави\ језик\ на\ "en_us"
menutrans &Find\ More\ Languages	Пронађи\ још\ језика 

" Tools/Folding
menutrans &Enable/Disable\ folds<Tab>zi   &Омогући/прекини\ свијање<Tab>zi
menutrans &View\ Cursor\ Line<Tab>zv	  &Покажи\ ред\ са\ курсором<Tab>zv
menutrans Vie&w\ Cursor\ Line\ only<Tab>zMzx Покажи\ &само\ ред\ са\ курсором<Tab>zMzx
menutrans C&lose\ more\ folds<Tab>zm   &Затвори\ више\ свијутака<Tab>zm
menutrans &Close\ all\ folds<Tab>zM    Затвори\ с&ве\ свијутке<Tab>zM
menutrans O&pen\ more\ folds<Tab>zr    Отвори\ виш&е\ свијутака<Tab>zr
menutrans &Open\ all\ folds<Tab>zR     О&твори\ све\ свијутке<Tab>zR
menutrans Fold\ Met&hod		       &Начин\ подвијања
menutrans Fold\ Col&umn\ Width	Ширина\ колоне\ испред\ свијутака

" Tools/Folding/Fold Method
menutrans M&anual	&Ручно
menutrans I&ndent	&Увученост
menutrans E&xpression	&Израз
menutrans S&yntax	&Синтакса
"menutrans &Diff
menutrans Ma&rker	&Ознака

" Tools/Diff
menutrans &Update	&Ажурирај
menutrans &Get\ Block	&Прихвати\ блок\ изменa
menutrans &Put\ Block	Пре&баци\ блок\ измена

" Tools/Error Window
menutrans &Update<Tab>:cwin   &Ажурирај<Tab>:cwin
menutrans &Open<Tab>:copen    &Отвори<Tab>:copen
menutrans &Close<Tab>:cclose  &Затвори<Tab>:cclose

" Bufers menu
menutrans &Buffers	   &Бафери
menutrans &Refresh\ menu   &Ажурирај
menutrans Delete	   &Обриши
menutrans &Alternate	   А&лтернативни
menutrans &Next		   &Следећи
menutrans &Previous	   &Претходни
menutrans [No\ File]	   [Нема\ фајла]

" Window menu
menutrans &Window		    &Прозор
menutrans &New<Tab>^Wn		    &Нови<Tab>^Wn
menutrans S&plit<Tab>^Ws	    &Подели<Tab>^Ws
menutrans Sp&lit\ To\ #<Tab>^W^^    Подели\ са\ &алтернативним<Tab>^W^^
menutrans Split\ &Vertically<Tab>^Wv   Подели\ &усправно<Tab>^Wv
menutrans Split\ File\ E&xplorer    Подели\ за\ преглед\ &фајлова
menutrans &Close<Tab>^Wc	    &Затвори<Tab>^Wc
menutrans Close\ &Other(s)<Tab>^Wo  Затвори\ &остале<Tab>^Wo
"menutrans Ne&xt<Tab>^Ww       &Следећи<Tab>^Ww
"menutrans P&revious<Tab>^WW	  П&ретходни<Tab>^WW
menutrans Move\ &To		    Пре&мести
menutrans Rotate\ &Up<Tab>^WR	    &Кружно\ нагоре<Tab>^WR
menutrans Rotate\ &Down<Tab>^Wr     Кружно\ надол&е<Tab>^Wr
menutrans &Equal\ Size<Tab>^W=	    &Исте\ величине<Tab>^W=
menutrans &Max\ Height<Tab>^W_	    Максимална\ &висина<Tab>^W_
menutrans M&in\ Height<Tab>^W1_     Минима&лна\ висина<Tab>^W1_
menutrans Max\ &Width<Tab>^W\|	    Максимална\ &ширина<Tab>^W\|
menutrans Min\ Widt&h<Tab>^W1\|     Минимална\ ши&рина<Tab>^W1\|

" Window/Move To
menutrans &Top<Tab>^WK		 &Врх<Tab>^WK
menutrans &Bottom<Tab>^WJ	 &Подножје<Tab>^WJ
menutrans &Left\ side<Tab>^WH	 У&лево<Tab>^WH
menutrans &Right\ side<Tab>^WL	 У&десно<Tab>^WL

" The popup menu
menutrans &Undo		      &Поништи
menutrans Cu&t		      &Исеци
menutrans &Copy		      &Копирај
menutrans &Paste	      &Убаци
menutrans &Delete	      И&збриши
menutrans Select\ Blockwise   Бирај\ б&локовски
menutrans Select\ &Word       Изабери\ &реч
menutrans Select\ &Sentence       Изабери\ р&еченицу
menutrans Select\ Pa&ragraph       Изабери\ &пасус
menutrans Select\ &Line       Изабери\ р&ед
menutrans Select\ &Block      Изабери\ &блок
menutrans Select\ &All	      Изабери\ &све

" The GUI toolbar
if has("toolbar")
  if exists("*Do_toolbar_tmenu")
    delfun Do_toolbar_tmenu
  endif
  fun Do_toolbar_tmenu()
    tmenu ToolBar.Open     Учитај
    tmenu ToolBar.Save     Сачувај
    tmenu ToolBar.SaveAll  Сачувај све
    tmenu ToolBar.Print    Штампај
    tmenu ToolBar.Undo     Врати
    tmenu ToolBar.Redo     Поврати
    tmenu ToolBar.Cut      Исеци
    tmenu ToolBar.Copy     Копирај
    tmenu ToolBar.Paste    Убаци
    tmenu ToolBar.Find     Нађи
    tmenu ToolBar.FindNext Нађи следећи
    tmenu ToolBar.FindPrev Нађи претходни
    tmenu ToolBar.Replace  Замени
    tmenu ToolBar.New      Нови
    tmenu ToolBar.WinSplit Подели прозор
    tmenu ToolBar.WinMax   Максимална висина
    tmenu ToolBar.WinMin   Минимална висина
    tmenu ToolBar.WinVSplit   Подели усправно
    tmenu ToolBar.WinMaxWidth Максимална ширина
    tmenu ToolBar.WinMinWidth Минимална ширина
    tmenu ToolBar.WinClose Затвори прозор
    tmenu ToolBar.LoadSesn Учитај сеансу
    tmenu ToolBar.SaveSesn Сачувај сеансу
    tmenu ToolBar.RunScript   Изврши спис
    tmenu ToolBar.Make     'make'
    tmenu ToolBar.Shell    Оперативно окружење
    tmenu ToolBar.RunCtags Направи ознаке
    tmenu ToolBar.TagJump  Иди на ознаку
    tmenu ToolBar.Help     Помоћ
    tmenu ToolBar.FindHelp Нађи објашњење
  endfun
endif

" Syntax menu
menutrans &Syntax &Синтакса
menutrans &Show\ File\ Types\ in\ Menu  Прикажи\ типове\ фајлова\ у\ &менију
menutrans Set\ '&syntax'\ only   Поде&си\ само\ 'syntax' 
menutrans Set\ '&filetype'\ too  Подеси\ &такође\ и\ 'filetype'
menutrans &Off       &Искључено
menutrans &Manual    &Ручно
menutrans A&utomatic    &Аутоматски
menutrans on/off\ for\ &This\ file     Да/не\ за\ овај\ &фајл
menutrans Co&lor\ test     Провера\ &боја
menutrans &Highlight\ test Провера\ исти&цања
menutrans &Convert\ to\ HTML  Претвори\ &у\ HTML

" dialog texts
let menutrans_help_dialog = "Унесите наредбу или реч чије појашњење тражите:\n\nДодајте i_ за наредбе уноса (нпр. i_CTRL-X)\nДодајте c_ за наредбе командног режима (нпр. с_<Del>)\nДодајте ' за имена опција (нпр. 'shiftwidth')"

let g:menutrans_path_dialog = "Унесите путању претраге за фајлове\nРаздвојите зарезима имена директоријума."

let g:menutrans_tags_dialog = "Унесите имена фајлова са ознакама\nРаздвојите зарезима имена."

let g:menutrans_textwidth_dialog = "Унесите нову ширину текста (0 спречава прелом)"

let g:menutrans_fileformat_dialog = "Изаберите формат записа фајла"

let g:menutrans_fileformat_choices = "&Unix\n&Dos\n&Mac\n&Откажи"

let menutrans_no_file = "[Нема фајла]"

let &cpo = s:keepcpo
unlet s:keepcpo

" vim: tw=0 keymap=serbian
