" Menu Translations:	Russian
" Maintainer:		Restorer, <restorer@mail2k.ru>
" Previous Maintainer:	Sergey Alyoshin, <alyoshin.s@gmail.com>
"			vassily ragosin, <vrr[at]users.sourceforge.net>
" Last Change:		23 Aug 2023
" Original translations
" URL:			https://github.com/RestorerZ/RuVim
"
"
" Adopted for RuVim project by Vassily Ragosin.
" First translation: Tim Alexeevsky, <realtim [at] mail.ru>,
" based on ukrainian translation by Bohdan Vlasyuk, <bohdan@vstu.edu.ua>
"
"
" Quit when menu translations have already been done.
"
" Check is
"
if exists("did_menu_trans")
   finish
endif
let g:did_menu_trans = 1
let s:keepcpo= &cpo
set cpo&vim

scriptencoding utf-8

" Top
menutrans &File				&Файл
menutrans &Edit				&Правка
menutrans &Tools			С&ервис
menutrans &Syntax			Син&таксис
menutrans &Buffers			&Буферы
menutrans &Window			&Окно
menutrans &Help				&Справка
"
"
"
" Submenu of menu Help
menutrans &Overview<Tab><F1>		О&бщий\ обзор<Tab>F1
menutrans &User\ Manual			&Руководство\ пользователя
menutrans &How-to\ links		&Инструкции
menutrans &Find\.\.\.			&Найти\.\.\.
"--------------------
menutrans &Credits			Со&авторы
menutrans Co&pying			&Лицензия
menutrans &Sponsor/Register		Сод&ействие\ и\ регистрация
menutrans O&rphans			&Благотворительность
"--------------------
menutrans &Version			&Текущая\ версия
menutrans &About			&О\ программе
"
"
" Submenu of File menu
menutrans &Open\.\.\.<Tab>:e		&Открыть\.\.\.<Tab>:e
menutrans Sp&lit-Open\.\.\.<Tab>:sp	От&крыть\ в\ новом\ окне\.\.\.<Tab>:sp
menutrans Open\ &Tab\.\.\.<Tab>:tabnew	Откры&ть\ в\ новой\ вкладке\.\.\.<Tab>:tabnew
menutrans &New<Tab>:enew		Созд&ать<Tab>:enew
menutrans &Close<Tab>:close		&Закрыть<Tab>:close
"--------------------
menutrans &Save<Tab>:w			&Сохранить<Tab>:w
menutrans Save\ &As\.\.\.<Tab>:sav	Со&хранить\ как\.\.\.<Tab>:sav
"--------------------
menutrans Split\ &Diff\ with\.\.\.	Сра&внить\ с\.\.\.
menutrans Split\ Patched\ &By\.\.\.	Сравн&ить\ и\ исправить\.\.\.
"--------------------
menutrans &Print			&Печать\.\.\.
menutrans Sa&ve-Exit<Tab>:wqa		Сохра&нить\ и\ выйти<Tab>:wqa
menutrans E&xit<Tab>:qa			В&ыход<Tab>:qa
"
"
" Submenu of Edit menu
menutrans &Undo<Tab>u			&Отменить<Tab>u
menutrans &Redo<Tab>^R			В&ернуть<Tab>Ctrl+R
menutrans Rep&eat<Tab>\.		Повторит&ь<Tab>\.
"--------------------
menutrans Cu&t<Tab>"+x			&Вырезать<Tab>"+x
menutrans &Copy<Tab>"+y			&Копировать<Tab>"+y
menutrans &Paste<Tab>"+gP		Вст&авить<Tab>"+g\ Shift+P
menutrans Put\ &Before<Tab>[p		Поместить\ п&еред<Tab>[p
menutrans Put\ &After<Tab>]p		Поместить\ по&сле<Tab>]p
menutrans &Delete<Tab>x			&Удалить<Tab>x
menutrans &Select\ All<Tab>ggVG		В&ыделить\ всё<Tab>gg\ Shift+V\ Shift+G
"--------------------
" if has("win32") || has("gui_gtk") || has("gui_kde") || has("gui_motif")
menutrans &Find\.\.\.			&Найти\.\.\.
menutrans Find\ and\ Rep&lace\.\.\.	&Заменить\.\.\.
" else
menutrans &Find<Tab>/			&Найти<Tab>/
menutrans Find\ and\ Rep&lace<Tab>:%s	&Заменить<Tab>:%s
menutrans Find\ and\ Rep&lace<Tab>:s	&Заменить<Tab>:s
"--------------------
menutrans Settings\ &Window			Все\ &параметры\.\.\.
menutrans Startup\ &Settings			Параметры\ запус&ка
menutrans &Global\ Settings			О&бщие\ параметры
menutrans F&ile\ Settings			Пара&метры\ текущего\ буфера
menutrans Show\ C&olor\ Schemes\ in\ Menu	Показать\ меню\ выбора\ цве&товой\ схемы
menutrans C&olor\ Scheme			Цветовая\ с&хема
menutrans Show\ &Keymaps\ in\ Menu		Показать\ меню\ выбора\ раскладки\ к&лавиатуры
menutrans &Keymap				&Раскладка\ клавиатуры
menutrans None					Не\ использовать
menutrans Select\ Fo&nt\.\.\.			&Шрифт\.\.\.
">>>----------------- Edit/Global settings
menutrans Toggle\ Pattern\ &Highlight<Tab>:set\ hls!		Подсветка\ сов&падений<Tab>:set\ hls!
menutrans Toggle\ &Ignoring\ Case<Tab>:set\ ic!			&Регистронезависимый\ поиск<Tab>:set\ ic!
menutrans Toggle\ &Showing\ Matched\ Pairs<Tab>:set\ sm!	Подсветка\ парных\ &элементов<Tab>:set\ sm!
menutrans &Context\ lines					Контекстных\ стр&ок
menutrans &Virtual\ Edit					Вир&туальное\ редактирование
menutrans Toggle\ Insert\ &Mode<Tab>:set\ im!			Режим\ &вставки<Tab>:set\ im!
menutrans Toggle\ Vi\ C&ompatibility<Tab>:set\ cp!		&Совместимость\ с\ редактором\ Vi<Tab>:set\ cp!
menutrans Search\ &Path\.\.\.					&Каталоги\ для\ поиска\ файлов\.\.\.
menutrans Ta&g\ Files\.\.\.					И&ндексные\ файлы\.\.\.
"
menutrans Toggle\ &Toolbar		Показ\ панели\ &инструментов
menutrans Toggle\ &Bottom\ Scrollbar	Показ\ полосы\ прокрутки\ вни&зу
menutrans Toggle\ &Left\ Scrollbar	Показ\ полосы\ прокрутки\ с&лева
menutrans Toggle\ &Right\ Scrollbar	Показ\ полосы\ прокрутки\ спр&ава
">>>->>>------------- Edit/Global settings/Virtual edit
menutrans Never				Выключено\ во\ всех\ режимах
menutrans Block\ Selection		Включено\ в\ режиме\ визуального\ блока
menutrans Insert\ mode			Включено\ в\ режиме\ вставки
menutrans Block\ and\ Insert		Включено\ в\ режимах\ визуального\ блока\ и\ вставки
menutrans Always			Включено\ во\ всех\ режимах
">>>----------------- Edit/File settings
menutrans Toggle\ Line\ &Numbering<Tab>:set\ nu!		Показ\ &нумерации\ строк<Tab>:set\ nu!
menutrans Toggle\ relati&ve\ Line\ Numbering<Tab>:set\ rnu!	Показ\ относите&льной\ нумерации\ строк<Tab>:set\ nru!
menutrans Toggle\ &List\ Mode<Tab>:set\ list!			Показ\ не&печатаемых\ знаков<Tab>:set\ list!
menutrans Toggle\ Line\ &Wrapping<Tab>:set\ wrap!		&Разбивка\ строк\ по\ границе\ окна<Tab>:set\ wrap!
menutrans Toggle\ W&rapping\ at\ word<Tab>:set\ lbr!		Разбивка\ строк\ по\ &границе\ слов<Tab>:set\ lbr!
menutrans Toggle\ Tab\ &Expanding<Tab>:set\ et!			Замена\ символов\ &табуляции\ на\ пробелы<Tab>:set\ et!
menutrans Toggle\ &Auto\ Indenting<Tab>:set\ ai!		Установка\ отступа\ как\ у\ текущей\ &строки<Tab>:set\ ai!
menutrans Toggle\ &C-Style\ Indenting<Tab>:set\ cin!		Установка\ отступа\ как\ в\ &языке\ Си<Tab>:set\ cin!
">>>---
menutrans &Shiftwidth				Вели&чина\ отступа
menutrans Soft\ &Tabstop			Ширина\ &табуляции
menutrans Te&xt\ Width\.\.\.			&Ширина\ текста\.\.\.
menutrans &File\ Format\.\.\.			&Формат\ файла\.\.\.
"
"
"
" Submenu of Tools menu
menutrans &Jump\ to\ this\ tag<Tab>g^]		&Перейти\ по\ указателю<Tab>g\ Ctrl+]
menutrans Jump\ &back<Tab>^T			&Вернуться\ назад<Tab>Ctrl+T
menutrans Build\ &Tags\ File			Создать\ файл\ с\ &индексами
"-------------------
menutrans &Folding				С&труктура\ текста
menutrans &Spelling				Пр&авописание
menutrans &Diff					&Сравнение\ текста
"-------------------
menutrans &Make<Tab>:make			Ко&мпиляция<Tab>:make
menutrans &List\ Errors<Tab>:cl			Распознанные\ о&шибки<Tab>:cl
menutrans L&ist\ Messages<Tab>:cl!		Вес&ь\ список\ результатов<Tab>:cl!
menutrans &Next\ Error<Tab>:cn			Следу&ющая\ запись\ из\ списка<Tab>:cn
menutrans &Previous\ Error<Tab>:cp		Пр&едыдущая\ запись\ из\ списка<Tab>:cp
menutrans &Older\ List<Tab>:cold		Пред&ыдущий\ список\ результатов<Tab>:cold
menutrans N&ewer\ List<Tab>:cnew		С&ледующий\ список\ результатов<Tab>:cnew
menutrans Error\ &Window			Ок&но\ со\ списком\ результатов
menutrans Show\ Compiler\ Se&ttings\ in\ Menu	Показать\ меню\ выбора\ &компилятора
menutrans Se&T\ Compiler			Выбрать\ &компилятор
"-------------------
menutrans &Convert\ to\ HEX<Tab>:%!xxd		Прео&бразовать\ в\ HEX<Tab>:%!xxd
menutrans Conve&rt\ back<Tab>:%!xxd\ -r		Преобразовать\ и&з\ HEX<Tab>:%!xxd\ -r
">>>---------------- Tools/Spelling
menutrans &Spell\ Check\ On			Выполнять\ &проверку
menutrans Spell\ Check\ &Off			&Не\ выполнять\ проверку
menutrans To\ &Next\ error<Tab>]s		С&ледующая\ ошибка<Tab>]s
menutrans To\ &Previous\ error<Tab>[s		Пр&едыдущая\ ошибка<Tab>[s
menutrans Suggest\ &Corrections<Tab>z=		Вариант&ы\ написания<Tab>z=
menutrans &Repeat\ correction<Tab>:spellrepall	Заменить\ &все<Tab>:spellrepall
"-------------------
menutrans Set\ language\ to\ "en"		Проверка\ для\ языка\ "en"
menutrans Set\ language\ to\ "en_au"		Проверка\ для\ языка\ "en_au"
menutrans Set\ language\ to\ "en_ca"		Проверка\ для\ языка\ "en_ca"
menutrans Set\ language\ to\ "en_gb"		Проверка\ для\ языка\ "en_gb"
menutrans Set\ language\ to\ "en_nz"		Проверка\ для\ языка\ "en_nz"
menutrans Set\ language\ to\ "en_us"		Проверка\ для\ языка\ "en_us"
menutrans &Find\ More\ Languages		Найти\ для\ других\ &языков
let g:menutrans_set_lang_to =			'Проверка для языка'
">>>---------------- Folds
menutrans &Enable/Disable\ folds<Tab>zi		&Показать\ или\ убрать\ структуру<Tab>zi
menutrans &View\ Cursor\ Line<Tab>zv		Просмотр\ строки\ под\ &курсором<Tab>zv
menutrans Vie&w\ Cursor\ Line\ only<Tab>zMzx	Просмотр\ &только\ строки\ под\ курсором<Tab>z\ Shift+M\ zx
menutrans C&lose\ more\ folds<Tab>zm		Свернуть\ вло&женные\ блоки\ структуры<Tab>zm
menutrans &Close\ all\ folds<Tab>zM		Свернуть\ &все\ блоки\ структуры<Tab>z\ Shift+M
menutrans &Open\ all\ folds<Tab>zR		Развернуть\ в&се\ блоки\ структуры<Tab>z\ Shift+R
menutrans O&pen\ more\ folds<Tab>zr		Ра&звернуть\ вложенный\ блок\ структуры<Tab>zr
menutrans Fold\ Met&hod				&Метод\ разметки\ структуры
menutrans Create\ &Fold<Tab>zf			Со&здать\ блок\ структуры<Tab>zf
menutrans &Delete\ Fold<Tab>zd			&Убрать\ блок\ структуры<Tab>zd
menutrans Delete\ &All\ Folds<Tab>zD		Убрать\ вс&е\ блоки\ структуры<Tab>z\ Shift+D
menutrans Fold\ col&umn\ width			&Ширина\ столбца\ со\ значками\ структуры
">>>->>>----------- Tools/Folds/Fold Method
menutrans M&anual				Разметка\ вру&чную
menutrans I&ndent				На\ основе\ о&тступов
menutrans E&xpression				На\ основе\ р&асчётов
menutrans S&yntax				На\ основе\ &синтаксиса
menutrans &Diff					На\ основе\ различий\ в\ текстах
menutrans Ma&rker				На\ основе\ &маркеров
">>>--------------- Sub of Tools/Diff
menutrans &Update				О&бновить\ содержимое\ окон
menutrans &Get\ Block				Перенести\ &в\ текущий\ буфер
menutrans &Put\ Block				Перенести\ &из\ текущего\ буфера
">>>--------------- Tools/Error window
menutrans &Update<Tab>:cwin			О&бновить<Tab>:cwin
menutrans &Close<Tab>:cclose			&Закрыть<Tab>:cclose
menutrans &Open<Tab>:copen			&Открыть<Tab>:copen
"
"
" Syntax menu
"
menutrans &Show\ File\ Types\ in\ menu		&Показать\ меню\ выбора\ типа\ файла
menutrans Set\ '&syntax'\ only			А&ктивировать\ параметр\ 'syntax'
menutrans Set\ '&filetype'\ too			Активировать\ пара&метр\ 'filetype'
menutrans &Off					&Отключить\ подсветку
menutrans &Manual				Включение\ подсветки\ вру&чную
menutrans A&utomatic				Включение\ подсветки\ &автоматически
menutrans on/off\ for\ &This\ file		Изменить\ режим\ для\ &текущего\ файла
menutrans Co&lor\ test				Проверить\ поддер&живаемые\ цвета
menutrans &Highlight\ test			Показать\ группы\ под&светки
menutrans &Convert\ to\ HTML			Прео&бразовать\ текущий\ файл\ в\ HTML
"
"
" Buffers menu
"
menutrans &Refresh\ menu			&Обновить\ список\ буферов
menutrans &Delete				&Закрыть\ буфер
menutrans &Alternate				&Соседний\ буфер
menutrans &Next					С&ледующий\ буфер
menutrans &Previous				&Предыдущий\ буфер
"
"
" Submenu of Window menu
"
menutrans &New<Tab>^Wn				&Создать<Tab>Ctrl+W\ n
menutrans S&plit<Tab>^Ws			Разделить\ по\ &горизонтали<Tab>Ctrl+W\ s
menutrans Split\ &Vertically<Tab>^Wv		Разделить\ по\ &вертикали<Tab>Ctrl+W\ v
menutrans Sp&lit\ To\ #<Tab>^W^^		С&оседний\ файл\ в\ новом\ окне<Tab>Ctrl+W\ Ctrl+^
menutrans Split\ File\ E&xplorer		Диспетчер\ файлов
"
menutrans &Close<Tab>^Wc			&Закрыть\ текущее\ окно<Tab>Ctrl+W\ c
menutrans Close\ &Other(s)<Tab>^Wo		З&акрыть\ другие\ окна<Tab>Ctrl+W\ o
"
menutrans Move\ &To				&Переместить
menutrans Rotate\ &Up<Tab>^WR			Сдвинуть\ ввер&х<Tab>Ctrl+W\ Shift+R
menutrans Rotate\ &Down<Tab>^Wr			Сдвинуть\ в&низ<Tab>Ctrl+W\ r
"
menutrans &Equal\ Size<Tab>^W=			Выравнивание\ раз&мера<Tab>Ctrl+W\ =
menutrans &Max\ Height<Tab>^W_			Максимальная\ в&ысота<Tab>Ctrl+W\ _
menutrans M&in\ Height<Tab>^W1_			Минимальная\ высо&та<Tab>Ctrl+W\ 1_
menutrans Max\ &Width<Tab>^W\|			Максимальная\ &ширина<Tab>Ctrl+W\ \|
menutrans Min\ Widt&h<Tab>^W1\|			Минимальная\ ш&ирина<Tab>Ctrl+W\ 1\|
">>>----------------- Submenu of Window/Move To
menutrans &Top<Tab>^WK				В&верх<Tab>Ctrl+W\ Shift+K
menutrans &Bottom<Tab>^WJ			В&низ<Tab>Ctrl+W\ Shift+J
menutrans &Left\ side<Tab>^WH			В&лево<Tab>Ctrl+W\ Shift+H
menutrans &Right\ side<Tab>^WL			В&право<Tab>Ctrl+W\ Shift+L
"
"
" The popup menu
"
"
menutrans &Undo					&Отменить
menutrans Cu&t					&Вырезать
menutrans &Copy					&Копировать
menutrans &Paste				Вст&авить
menutrans &Delete				&Удалить
menutrans Select\ Blockwise			Блоковое\ выделение
menutrans Select\ &Word				Выделить\ с&лово
menutrans Select\ &Line				Выделить\ с&троку
menutrans Select\ &Block			Выделить\ &блок
menutrans Select\ &All				В&ыделить\ всё
menutrans Select\ &Sentence			Выделить\ предло&жение
menutrans Select\ Pa&ragraph			Выделить\ аб&зац
"
" The Spelling popup menu
"
let g:menutrans_spell_change_ARG_to =		'Исправить\ "%s"'
let g:menutrans_spell_add_ARG_to_word_list =	'Добавить\ "%s"\ в\ словарь'
let g:menutrans_spell_ignore_ARG =		'Пропустить\ "%s"'
"
" The GUI toolbar
"
if has("toolbar")
  if exists("*Do_toolbar_tmenu")
    delfun Do_toolbar_tmenu
  endif
  def g:Do_toolbar_tmenu()
    tmenu ToolBar.New				Создать документ
    tmenu ToolBar.Open				Открыть файл
    tmenu ToolBar.Save				Сохранить файл
    tmenu ToolBar.SaveAll			Сохранить все файлы
    tmenu ToolBar.Print				Печать
    tmenu ToolBar.Undo				Отменить
    tmenu ToolBar.Redo				Вернуть
    tmenu ToolBar.Cut				Вырезать
    tmenu ToolBar.Copy				Копировать
    tmenu ToolBar.Paste				Вставить
    tmenu ToolBar.Find				Найти...
    tmenu ToolBar.FindNext			Найти следующее
    tmenu ToolBar.FindPrev			Найти предыдущее
    tmenu ToolBar.Replace			Заменить...
    tmenu ToolBar.NewSesn			Создать сеанс редактирования
    tmenu ToolBar.LoadSesn			Загрузить сеанс редактирования
    tmenu ToolBar.SaveSesn			Сохранить сеанс редактирования
    tmenu ToolBar.RunScript			Выполнить командный файл программы Vim
    tmenu ToolBar.Shell				Командная оболочка
    tmenu ToolBar.Make				Компиляция
    tmenu ToolBar.RunCtags			Создать файл с индексами
    tmenu ToolBar.TagJump			Перейти по указателю
    tmenu ToolBar.Help				Справка
    tmenu ToolBar.FindHelp			Поиск в документации
    tmenu ToolBar.WinClose			Закрыть текущее окно
    tmenu ToolBar.WinMax			Максимальная высота текущего окна
    tmenu ToolBar.WinMin			Минимальная высота текущего окна
    tmenu ToolBar.WinSplit			Разделить окно по горизонтали
    tmenu ToolBar.WinVSplit			Разделить окно по вертикали
    tmenu ToolBar.WinMaxWidth			Максимальная ширина текущего окна
    tmenu ToolBar.WinMinWidth			Минимальная ширина текущего окна
  enddef
endif
"
"
" Dialog texts
"
" Find in help dialog
"
let g:menutrans_help_dialog = "Наберите команду или слово, которые требуется найти в документации.\n\nЧтобы найти команды режима вставки, используйте приставку i_ (например, i_CTRL-X)\nЧтобы найти команды командной строки, используйте приставку c_ (например, c_<Del>)\nЧтобы найти информацию о параметрах, используйте символ ' (например, 'shftwidth')"
"
" Search path dialog
"
let g:menutrans_path_dialog = "Укажите через запятую наименования каталогов, где будет выполняться поиск файлов"
"
" Tag files dialog
"
let g:menutrans_tags_dialog = "Укажите через запятую наименования файлов индексов"
"
" Text width dialog
"
let g:menutrans_textwidth_dialog = "Укажите количество символов для установки ширины текста\nЧтобы отменить форматирование, укажите 0"
"
" File format dialog
"
let g:menutrans_fileformat_dialog = "Выберите формат файла"
let g:menutrans_fileformat_choices = "&1. Unix\n&2. Dos\n&3. Mac\nОтмена (&C)"
"
let menutrans_no_file = "[Безымянный]"

" Menus to handle Russian encodings
" Thanks to Pavlo Bohmat for the idea
" vassily ragosin <vrr[at]users.sourceforge.net>
"
an 10.355 &File.-SEP-					<Nop>
an 10.360.20 &File.Открыть\ в\ кодировке\.\.\..CP1251	:browse e ++enc=cp1251<CR>
an 10.360.30 &File.Открыть\ в\ кодировке\.\.\..CP866	:browse e ++enc=cp866<CR>
an 10.360.30 &File.Открыть\ в\ кодировке\.\.\..KOI8-R	:browse e ++enc=koi8-r<CR>
an 10.360.40 &File.Открыть\ в\ кодировке\.\.\..UTF-8	:browse e ++enc=utf-8<CR>
an 10.365.20 &File.Сохранить\ с\ кодировкой\.\.\..CP1251 :browse w ++enc=cp1251<CR>
an 10.365.30 &File.Сохранить\ с\ кодировкой\.\.\..CP866	:browse w ++enc=cp866<CR>
an 10.365.30 &File.Сохранить\ с\ кодировкой\.\.\..KOI8-R :browse w ++enc=koi8-r<CR>
an 10.365.40 &File.Сохранить\ с\ кодировкой\.\.\..UTF-8	:browse w ++enc=utf-8<CR>
"

let &cpo = s:keepcpo
unlet s:keepcpo
