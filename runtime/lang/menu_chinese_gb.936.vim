" Menu Translations:    Simplified Chinese
" Maintainer:           Ada (Haowen) Yu <me@yuhaowen.com>
" Previous Maintainer:  Shun Bai <baishunde@gmail.com>, Yuheng Xie <elephant@linux.net.cn>
" Last Change:          2022 July 9
" Generated from menu_zh_cn.utf-8.vim, DO NOT EDIT
"
" Generated with the scripts from:
"
"       https://github.com/adaext/vim-menutrans-helper

" Quit when menu translations have already been done.

if exists("did_menu_trans")
  finish
endif
let did_menu_trans = 1
let s:keepcpo = &cpo
set cpo&vim

scriptencoding cp936

" Help menu
menutrans &Help 帮助(&H)
" Help menuitems and dialog {{{1
menutrans &Overview<Tab><F1> 概述(&O)<Tab><F1>
menutrans &User\ Manual 用户手册(&U)
menutrans &How-to\ Links 如何使用(&H)
menutrans &Find\.\.\. 查找(&F)\.\.\.
menutrans &Credits 致谢(&C)
menutrans Co&pying 版权(&P)
menutrans &Sponsor/Register 赞助/注册(&S)
menutrans O&rphans 拯救孤儿(&R)
menutrans &Version 版本(&V)
menutrans &About 关于(&A)

" fun! s:Helpfind()
if !exists("g:menutrans_help_dialog")
  let g:menutrans_help_dialog = "输入命令或单词以获得帮助:\n\n前缀 i_ 表示输入模式下的命令(如: i_CTRL-X)\n前缀 c_ 表示命令行下的编辑命令(如: c_<Del>)\n前缀 ' 表示选项名(如: 'shiftwidth')"
endif
" }}}

" File menu
menutrans &File 文件(&F)
" File menuitems {{{1
menutrans &Open\.\.\.<Tab>:e 打开(&O)\.\.\.<Tab>:e
menutrans Sp&lit-Open\.\.\.<Tab>:sp 在拆分窗口打开(&L)\.\.\.<Tab>:sp
menutrans Open\ &Tab\.\.\.<Tab>:tabnew 在标签页打开(&T)\.\.\.<Tab>:tabnew
menutrans &New<Tab>:enew 新建(&N)<Tab>:enew
menutrans &Close<Tab>:close 关闭(&C)<Tab>:close
menutrans &Save<Tab>:w 保存(&S)<Tab>:w
menutrans Save\ &As\.\.\.<Tab>:sav 另存为(&A)\.\.\.<Tab>:sav
menutrans Split\ &Diff\ With\.\.\. 拆分窗口以对比差异(Diff)(&D)\.\.\.
menutrans Split\ Patched\ &By\.\.\. 拆分窗口以进行修补(Patch)(&B)\.\.\.
menutrans &Print 打印(&P)
menutrans Sa&ve-Exit<Tab>:wqa 保存并退出(&V)<Tab>:wqa
menutrans E&xit<Tab>:qa 退出(&X)<Tab>:qa
" }}}

" Edit menu
menutrans &Edit 编辑(&E)
" Edit menuitems {{{1
menutrans &Undo<Tab>u 撤销(&U)<Tab>u
menutrans &Redo<Tab>^R 恢复(&R)<Tab>^R
menutrans Rep&eat<Tab>\. 重复(&E)<Tab>\.
menutrans Cu&t<Tab>"+x 剪切(&T)<Tab>"+x
menutrans &Copy<Tab>"+y 复制(&C)<Tab>"+y
menutrans &Paste<Tab>"+gP 粘贴(&P)<Tab>"+gP
menutrans Put\ &Before<Tab>[p 粘贴到光标前(&B)<Tab>[p
menutrans Put\ &After<Tab>]p 粘贴到光标后(&A)<Tab>]p
menutrans &Delete<Tab>x 删除(&D)<Tab>x
menutrans &Select\ All<Tab>ggVG 全选(&S)<Tab>ggVG
menutrans &Find\.\.\. 查找(&F)\.\.\.
menutrans Find\ and\ Rep&lace\.\.\. 查找和替换(&L)\.\.\.
menutrans &Find<Tab>/ 查找(&F)<Tab>/
menutrans Find\ and\ Rep&lace<Tab>:%s 查找和替换(&L)<Tab>:%s
menutrans Find\ and\ Rep&lace<Tab>:s 查找和替换(&L)<Tab>:s
menutrans Settings\ &Window 设置窗口(&W)
menutrans Startup\ &Settings 启动设置(&S)

" Edit/Global Settings
menutrans &Global\ Settings 全局设置(&G)
" Edit.Global Settings menuitems and dialogs {{{2
menutrans Toggle\ Pattern\ &Highlight<Tab>:set\ hls! 开/关高亮查找内容(&H)<Tab>:set\ hls!
menutrans Toggle\ &Ignoring\ Case<Tab>:set\ ic! 开/关忽略大小写(&I)<Tab>:set\ ic!
menutrans Toggle\ &Showing\ Matched\ Pairs<Tab>:set\ sm! 开/关显示括号匹配(&S)<Tab>:set\ sm!
menutrans &Context\ Lines 上下文行数(&C)
menutrans &Virtual\ Edit 虚拟编辑(&V)
" Edit.Global Settings.Virtual Edit menuitems {{{3
menutrans Never 从不
menutrans Block\ Selection 只在选定矩形块时
menutrans Insert\ Mode 只在插入模式时
menutrans Block\ and\ Insert 在选定矩形块和插入模式时
menutrans Always 始终
" }}}
menutrans Toggle\ Insert\ &Mode<Tab>:set\ im! 开/关插入模式(&M)<Tab>:set\ im!
menutrans Toggle\ Vi\ C&ompatibility<Tab>:set\ cp! 开/关\ Vi\ 兼容性(&O)<Tab>:set\ cp!
menutrans Search\ &Path\.\.\. 搜索路径(&P)\.\.\.
menutrans Ta&g\ Files\.\.\. 标记文件(Tags)(&G)\.\.\.

" GUI options
menutrans Toggle\ &Toolbar 开/关工具栏(&T)
menutrans Toggle\ &Bottom\ Scrollbar 开/关底部滚动条(&B)
menutrans Toggle\ &Left\ Scrollbar 开/关左侧滚动条(&L)
menutrans Toggle\ &Right\ Scrollbar 开/关右侧滚动条(&R)

" fun! s:SearchP()
if !exists("g:menutrans_path_dialog")
  let g:menutrans_path_dialog = "输入搜索路径。\n用逗号分隔目录名。"
endif

" fun! s:TagFiles()
if !exists("g:menutrans_tags_dialog")
  let g:menutrans_tags_dialog = "输入标记文件(Tags)名称。\n用逗号分隔文件名。"
endif
" }}}

" Edit/File Settings
menutrans F&ile\ Settings 文件设置(&I)
" Edit.File Settings menuitems and dialogs {{{2
" Boolean options
menutrans Toggle\ Line\ &Numbering<Tab>:set\ nu! 开/关行号(&N)<Tab>:set\ nu!
menutrans Toggle\ Relati&ve\ Line\ Numbering<Tab>:set\ rnu! 开/关相对行号(&V)<Tab>:set\ rnu!
menutrans Toggle\ &List\ Mode<Tab>:set\ list! 开/关列表模式(&L)<Tab>:set\ list!
menutrans Toggle\ Line\ &Wrapping<Tab>:set\ wrap! 开/关换行(&W)<Tab>:set\ wrap!
menutrans Toggle\ W&rapping\ at\ Word<Tab>:set\ lbr! 开/关词尾换行(&R)<Tab>:set\ lbr!
menutrans Toggle\ Tab\ &Expanding<Tab>:set\ et! 开/关制表符扩展(&E)<Tab>:set\ et!
menutrans Toggle\ &Auto\ Indenting<Tab>:set\ ai! 开/关自动缩进(&A)<Tab>:set\ ai!
menutrans Toggle\ &C-Style\ Indenting<Tab>:set\ cin! 开/关\ C\ 语言式缩进(&C)<Tab>:set\ cin!

" other options
menutrans &Shiftwidth 缩进宽度(&S)
menutrans Soft\ &Tabstop 软制表位宽度(Soft\ Tabstop)(&T)
menutrans Te&xt\ Width\.\.\. 文本宽度(&X)\.\.\.
menutrans &File\ Format\.\.\. 文件格式(&F)\.\.\.

" fun! s:TextWidth()
if !exists("g:menutrans_textwidth_dialog")
  let g:menutrans_textwidth_dialog = "输入文本宽度(每行最大字符数，0 表示禁用):"
endif

" fun! s:FileFormat()
if !exists("g:menutrans_fileformat_dialog")
  let g:menutrans_fileformat_dialog = "选择文件的保存格式:"
endif
if !exists("g:menutrans_fileformat_choices")
  let g:menutrans_fileformat_choices = "&Unix\n&Dos\n&Mac\n取消(&C)"
endif
" }}}
menutrans Show\ C&olor\ Schemes\ in\ Menu 在菜单中显示配色方案(&O)
menutrans C&olor\ Scheme 配色方案(&O)
menutrans Show\ &Keymaps\ in\ Menu 在菜单中显示键盘映射(&K)
menutrans &Keymap 键盘映射(&K)
menutrans Select\ Fo&nt\.\.\. 选择字体(&N)\.\.\.
" }}}

" Programming menu
menutrans &Tools 工具(&T)
" Tools menuitems {{{1
menutrans &Jump\ to\ This\ Tag<Tab>g^] 跳转到这个标记(Tag)(&J)<Tab>g^]
menutrans Jump\ &Back<Tab>^T 跳转回(&B)<Tab>^T
menutrans Build\ &Tags\ File 生成标记文件(Tags)(&T)

" Tools.Spelling Menu
menutrans &Spelling 拼写检查(&S)
" Tools.Spelling menuitems and dialog {{{2
menutrans &Spell\ Check\ On 打开拼写检查(&S)
menutrans Spell\ Check\ &Off 关闭拼写检查(&O)
menutrans To\ &Next\ Error<Tab>]s 上一个错误(&N)<Tab>]s
menutrans To\ &Previous\ Error<Tab>[s 下一个错误(&P)<Tab>[s
menutrans Suggest\ &Corrections<Tab>z= 更正建议(&C)<Tab>z=
menutrans &Repeat\ Correction<Tab>:spellrepall 更正全部同类错误(&R)<Tab>:spellrepall
menutrans Set\ Language\ to\ "en" 设置语言为\ "en"
menutrans Set\ Language\ to\ "en_au" 设置语言为\ "en_au"
menutrans Set\ Language\ to\ "en_ca" 设置语言为\ "en_ca"
menutrans Set\ Language\ to\ "en_gb" 设置语言为\ "en_gb"
menutrans Set\ Language\ to\ "en_nz" 设置语言为\ "en_nz"
menutrans Set\ Language\ to\ "en_us" 设置语言为\ "en_us"
menutrans &Find\ More\ Languages 查找更多语言(&F)

" func! s:SpellLang()
if !exists("g:menutrans_set_lang_to")
  let g:menutrans_set_lang_to = "设置语言为"
endif
" }}}

" Tools.Fold Menu
menutrans &Folding 折叠(&F)
" Tools.Fold menuitems {{{2
" open close folds
menutrans &Enable/Disable\ Folds<Tab>zi 启用/禁用折叠(&E)<Tab>zi
menutrans &View\ Cursor\ Line<Tab>zv 展开光标所在行(&V)<Tab>zv
menutrans Vie&w\ Cursor\ Line\ Only<Tab>zMzx 只展开光标所在行(&W)<Tab>zMzx
menutrans C&lose\ More\ Folds<Tab>zm 折叠一级(&L)<Tab>zm
menutrans &Close\ All\ Folds<Tab>zM 折叠全部(&C)<Tab>zM
menutrans O&pen\ More\ Folds<Tab>zr 展开一级(&P)<Tab>zr
menutrans &Open\ All\ Folds<Tab>zR 展开全部(&O)<Tab>zR
" fold method
menutrans Fold\ Met&hod 折叠方式(&H)
" Tools.Fold.Fold Method menuitems {{{3
menutrans M&anual 手动(&A)
menutrans I&ndent 缩进(&N)
menutrans E&xpression 表达式(&X)
menutrans S&yntax 语法(&Y)
menutrans &Diff 差异(Diff)(&D)
menutrans Ma&rker 记号(Marker)(&R)
" }}}
" create and delete folds
menutrans Create\ &Fold<Tab>zf 创建折叠(&F)<Tab>zf
menutrans &Delete\ Fold<Tab>zd 删除折叠(&D)<Tab>zd
menutrans Delete\ &All\ Folds<Tab>zD 删除全部折叠(&A)<Tab>zD
" moving around in folds
menutrans Fold\ Col&umn\ Width 折叠操作栏宽度(&W)
" }}}

" Tools.Diff Menu
menutrans &Diff 差异(Diff)(&D)
" Tools.Diff menuitems {{{2
menutrans &Update 刷新(&U)
menutrans &Get\ Block 采用对侧文本块(&G)
menutrans &Put\ Block 采用本侧文本块(&P)
" }}}

menutrans &Make<Tab>:make 生成(Make)(&M)<Tab>:make
menutrans &List\ Errors<Tab>:cl 列出错误(&L)<Tab>:cl
menutrans L&ist\ Messages<Tab>:cl! 列出消息(&I)<Tab>:cl!
menutrans &Next\ Error<Tab>:cn 下一个错误(&N)<Tab>:cn
menutrans &Previous\ Error<Tab>:cp 上一个错误(&P)<Tab>:cp
menutrans &Older\ List<Tab>:cold 较旧的错误列表(&O)<Tab>:cold
menutrans N&ewer\ List<Tab>:cnew 较新的错误列表(&E)<Tab>:cnew
menutrans Error\ &Window 错误窗口(&W)
" Tools.Error Window menuitems {{{2
menutrans &Update<Tab>:cwin 刷新(&U)<Tab>:cwin
menutrans &Open<Tab>:copen 打开(&O)<Tab>:copen
menutrans &Close<Tab>:cclose 关闭(&C)<Tab>:cclose
" }}}
menutrans Show\ Compiler\ Se&ttings\ in\ Menu 在菜单中显示编译器设置(&T)
menutrans Se&t\ Compiler 设置编译器(&T)
menutrans &Convert\ to\ HEX<Tab>:%!xxd 转换成十六进制(&C)<Tab>:%!xxd
menutrans Conve&rt\ Back<Tab>:%!xxd\ -r 转换回(&R)<Tab>:%!xxd\ -r
" }}}

" Buffer menu
menutrans &Buffers 缓冲区(&B)
" Buffer menuitems and dialog {{{1
menutrans &Refresh\ Menu 刷新本菜单(&R)
menutrans &Delete 删除(&D)
menutrans &Alternate 切换(&A)
menutrans &Next 下一个(&N)
menutrans &Previous 上一个(&P)

" func! s:BMMunge(fname, bnum)
if !exists("g:menutrans_no_file")
  let g:menutrans_no_file = "[无文件]"
endif
" }}}

" Window menu
menutrans &Window 窗口(&W)
" Window menuitems {{{1
menutrans &New<Tab>^Wn 新建(&N)<Tab>^Wn
menutrans S&plit<Tab>^Ws 拆分(&P)<Tab>^Ws
menutrans Sp&lit\ To\ #<Tab>^W^^ 拆分并显示缓冲区\ #(&L)<Tab>^W^^
menutrans Split\ &Vertically<Tab>^Wv 垂直拆分(&V)<Tab>^Wv
menutrans Split\ File\ E&xplorer 拆分并打开文件浏览器(&X)
menutrans &Close<Tab>^Wc 关闭(&C)<Tab>^Wc
menutrans Close\ &Other(s)<Tab>^Wo 除此之外全部关闭(&O)<Tab>^Wo
menutrans Move\ &To 移动到(&T)
menutrans &Top<Tab>^WK 顶端(&T)<Tab>^WK
menutrans &Bottom<Tab>^WJ 底端(&B)<Tab>^WJ
menutrans &Left\ Side<Tab>^WH 左边(&L)<Tab>^WH
menutrans &Right\ Side<Tab>^WL 右边(&R)<Tab>^WL
menutrans Rotate\ &Up<Tab>^WR 向上轮换(&U)<Tab>^WR
menutrans Rotate\ &Down<Tab>^Wr 向下轮换(&D)<Tab>^Wr
menutrans &Equal\ Size<Tab>^W= 平均分布(&E)<Tab>^W=
menutrans &Max\ Height<Tab>^W_ 最大高度(&M)<Tab>^W
menutrans M&in\ Height<Tab>^W1_ 最小高度(&I)<Tab>^W1_
menutrans Max\ &Width<Tab>^W\| 最大宽度(&W)<Tab>^W\|
menutrans Min\ Widt&h<Tab>^W1\| 最小宽度(&H)<Tab>^W1\|
" }}}

" The popup menu {{{1
menutrans &Undo 撤销(&U)
menutrans Cu&t 剪切(&T)
menutrans &Copy 复制(&C)
menutrans &Paste 粘贴(&P)
menutrans &Delete 删除(&D)
menutrans Select\ Blockwise 改为选定矩形块
menutrans Select\ &Word 选定单词(&W)
menutrans Select\ &Sentence 选定句(&S)
menutrans Select\ Pa&ragraph 选定段落(&R)
menutrans Select\ &Line 选定行(&L)
menutrans Select\ &Block 选定矩形块(&B)
menutrans Select\ &All 全选(&A)

" func! <SID>SpellPopup()
if !exists("g:menutrans_spell_change_ARG_to")
  let g:menutrans_spell_change_ARG_to = '将\ "%s"\ 更改为'
endif
if !exists("g:menutrans_spell_add_ARG_to_word_list")
  let g:menutrans_spell_add_ARG_to_word_list = '将\ "%s"\ 添加到词典'
endif
if !exists("g:menutrans_spell_ignore_ARG")
  let g:menutrans_spell_ignore_ARG = '忽略\ "%s"'
endif
" }}}

" The GUI toolbar {{{1
if has("toolbar")
  if exists("*Do_toolbar_tmenu")
    delfun Do_toolbar_tmenu
  endif
  fun Do_toolbar_tmenu()
    let did_toolbar_tmenu = 1
    tmenu ToolBar.Open 打开文件
    tmenu ToolBar.Save 保存当前文件
    tmenu ToolBar.SaveAll 全部保存
    tmenu ToolBar.Print 打印
    tmenu ToolBar.Undo 撤销
    tmenu ToolBar.Redo 恢复
    tmenu ToolBar.Cut 剪切到剪贴板
    tmenu ToolBar.Copy 复制到剪贴板
    tmenu ToolBar.Paste 从剪贴板粘贴
    if !has("gui_athena")
      tmenu ToolBar.Replace 查找和替换...
      tmenu ToolBar.FindNext 查找下一个
      tmenu ToolBar.FindPrev 查找上一个
    endif
    tmenu ToolBar.LoadSesn 加载会话
    tmenu ToolBar.SaveSesn 保存当前会话
    tmenu ToolBar.RunScript 运行 Vim 脚本
    tmenu ToolBar.Make 生成当前项目 (:make)
    tmenu ToolBar.RunCtags 在当前目录生成标记(Tags) (!ctags -R .)
    tmenu ToolBar.TagJump 跳转到光标所在标记(Tag)
    tmenu ToolBar.Help Vim 帮助
    tmenu ToolBar.FindHelp 在 Vim 帮助中查找
  endfun
endif
" }}}

" Syntax menu
menutrans &Syntax 语法(&S)
" Syntax menuitems {{{1
menutrans &Show\ File\ Types\ in\ Menu 在菜单中显示文件类型(&S)
menutrans &Off 关闭(&O)
menutrans &Manual 手动(&M)
menutrans A&utomatic 自动(&U)
menutrans On/Off\ for\ &This\ File 只对这个文件开/关(&T)
menutrans Co&lor\ Test 色彩测试(&L)
menutrans &Highlight\ Test 高亮测试(&H)
menutrans &Convert\ to\ HTML 转换成\ HTML(&C)

" From synmenu.vim
menutrans Set\ '&syntax'\ Only 只设置\ 'syntax'(&S)
menutrans Set\ '&filetype'\ Too 也设置\ 'filetype'(&F)
menutrans Oracle\ config Oracle\ 配置文件
menutrans Vim\ help\ file Vim\ 帮助文件
menutrans Vim\ script Vim\ 脚本
menutrans Viminfo\ file Vim\ 信息文件
menutrans Virata\ config Virata\ 配置文件
menutrans Whitespace\ (add) 增加加亮空格
" }}}

" Netrw menu {{{1
" Plugin loading may be after menu translation
" So giveup testing if Netrw Plugin is loaded
" if exists("g:loaded_netrwPlugin")
  menutrans Help<tab><F1> 帮助<tab><F1>
  menutrans Bookmarks 书签
  menutrans History 历史记录
  menutrans Go\ Up\ Directory<tab>- 向上一级<tab>-
  menutrans Apply\ Special\ Viewer<tab>x 用默认程序打开<tab>x
  menutrans Bookmarks\ and\ History 书签和历史记录
  " Netrw.Bookmarks and History menuitems {{{2
  menutrans Bookmark\ Current\ Directory<tab>mb 添加书签<tab>mb
  menutrans Bookmark\ Delete 移除书签
  menutrans Goto\ Prev\ Dir\ (History)<tab>u 后退(历史记录)<tab>u
  menutrans Goto\ Next\ Dir\ (History)<tab>U 前进(历史记录)<tab>U
  menutrans List<tab>qb 完整列表<tab>qb
  " }}}
  menutrans Browsing\ Control 控制
  " Netrw.Browsing Control menuitems {{{2
  menutrans Horizontal\ Split<tab>o 在拆分窗口打开<tab>o
  menutrans Vertical\ Split<tab>v 在垂直拆分窗口打开<tab>v
  menutrans New\ Tab<tab>t 在标签页打开<tab>t
  menutrans Preview<tab>p 预览<tab>p
  menutrans Edit\ File\ Hiding\ List<tab><ctrl-h> 编辑隐藏条件(Hiding\ List)<tab><ctrl-h>
  menutrans Edit\ Sorting\ Sequence<tab>S 编辑排序条件(Sorting\ Sequence)<tab>S
  menutrans Quick\ Hide/Unhide\ Dot\ Files<tab>gh 快速隐藏/显示以\.开头的文件<tab>gh
  menutrans Refresh\ Listing<tab><ctrl-l> 刷新<tab><ctrl-l>
  menutrans Settings/Options<tab>:NetrwSettings 设置/选项<tab>:NetrwSettings
  " }}}
  menutrans Delete\ File/Directory<tab>D 删除文件/目录<tab>D
  menutrans Edit\ File/Dir 编辑文件/目录
  " Netrw.Edit File menuitems {{{2
  menutrans Create\ New\ File<tab>% 新建文件<tab>%
  menutrans In\ Current\ Window<tab><cr> 在当前窗口<tab><cr>
  menutrans Preview\ File/Directory<tab>p 预览文件/目录<tab>p
  menutrans In\ Previous\ Window<tab>P 在上一个窗口<tab>P
  menutrans In\ New\ Window<tab>o 在新窗口<tab>o
  menutrans In\ New\ Tab<tab>t 在新标签页<tab>t
  menutrans In\ New\ Vertical\ Window<tab>v 在新垂直窗口<tab>v
  " }}}
  menutrans Explore 浏览
  " Netrw.Explore menuitems {{{2
  menutrans Directory\ Name 指定目录名
  menutrans Filenames\ Matching\ Pattern\ (curdir\ only)<tab>:Explore\ */ 匹配指定文件名模式(当前目录)<tab>:Explore\ */
  menutrans Filenames\ Matching\ Pattern\ (+subdirs)<tab>:Explore\ **/ 匹配指定文件名模式(含子目录)<tab>:Explore\ **/
  menutrans Files\ Containing\ String\ Pattern\ (curdir\ only)<tab>:Explore\ *// 内容包含指定字符串模式(当前目录)<tab>:Explore\ *//
  menutrans Files\ Containing\ String\ Pattern\ (+subdirs)<tab>:Explore\ **// 内容包含指定字符串模式(含子目录)<tab>:Explore\ **//
  menutrans Next\ Match<tab>:Nexplore 下一个匹配项<tab>:Nexplore
  menutrans Prev\ Match<tab>:Pexplore 上一个匹配项<tab>:Pexplore
  " }}}
  menutrans Make\ Subdirectory<tab>d 新建子目录<tab>d
  menutrans Marked\ Files 选定的(Marked)文件
  " Netrw.Marked Files menuitems {{{2
  menutrans Mark\ File<tab>mf 选定(Mark)/取消<tab>mf
  menutrans Mark\ Files\ by\ Regexp<tab>mr 用正则表达式(Regexp)选定<tab>mr
  menutrans Hide-Show-List\ Control<tab>a 隐藏/显示<tab>a
  menutrans Copy\ To\ Target<tab>mc 复制到目标<tab>mc
  menutrans Delete<tab>D 删除<tab>D
  menutrans Diff<tab>md 差异(Diff)<tab>md
  menutrans Edit<tab>me 编辑<tab>me
  menutrans Exe\ Cmd<tab>mx 作为参数运行命令<tab>mx
  menutrans Move\ To\ Target<tab>mm 移动到目标<tab>mm
  menutrans Obtain<tab>O 获取<tab>O
  menutrans Print<tab>mp 打印<tab>mp
  menutrans Replace<tab>R 替换<tab>R
  menutrans Set\ Target<tab>mt 设置目标<tab>mt
  menutrans Tag<tab>mT 生成标记文件(Tags)<tab>mT
  menutrans Zip/Unzip/Compress/Uncompress<tab>mz 压缩/解压缩<tab>mz
  " }}}
  menutrans Obtain\ File<tab>O 获取文件<tab>O
  menutrans Style 显示风格
  " Netrw.Style menuitems {{{2
  menutrans Listing 列表形式
  " Netrw.Style.Listing menuitems {{{3
  menutrans thin<tab>i 紧凑<thin)<tab>i
  menutrans long<tab>i 详细(long)<tab>i
  menutrans wide<tab>i 多列(wide)<tab>i
  menutrans tree<tab>i 树状(tree)<tab>i
  " }}}
  menutrans Normal-Hide-Show 显示/隐藏
  " Netrw.Style.Normal-Hide_show menuitems {{{3
  menutrans Show\ All<tab>a 显示全部
  menutrans Normal<tab>a 不显示隐藏文件
  menutrans Hidden\ Only<tab>a 只显示隐藏文件
  " }}}
  menutrans Reverse\ Sorting\ Order<tab>r 升序/降序<tab>r
  menutrans Sorting\ Method 排序方式
  " Netrw.Style.Sorting Method menuitems {{{3
  menutrans Name<tab>s 文件名<tab>s
  menutrans Time<tab>s 修改时间<tab>s
  menutrans Size<tab>s 大小<tab>s
  menutrans Exten<tab>s 扩展名<tab>s
  " }}}
  " }}}
  menutrans Rename\ File/Directory<tab>R 重命名文件/目录<tab>R
  menutrans Set\ Current\ Directory<tab>c 设置\ Vim\ 工作目录<tab>c
  menutrans Targets 目标
" endif
" }}}

" Shellmenu menu
" Shellmenu menuitems {{{1
" From shellmenu.vim
menutrans ShellMenu Shell\ 菜单
menutrans Statements 语句
menutrans Test 测试
menutrans Existence 存在
menutrans Existence\ -\ file 存在\ -\ 文件
menutrans Existence\ -\ file\ (not\ empty) 存在\ -\ 文件(非空)
menutrans Existence\ -\ directory 存在\ -\ 目录
menutrans Existence\ -\ executable 存在\ -\ 可执行
menutrans Existence\ -\ readable 存在\ -\ 可读
menutrans Existence\ -\ writable 存在\ -\ 可写
menutrans String\ is\ empty 字符串为空
menutrans String\ is\ not\ empty 字符串非空
menutrans Strings\ are\ equal 字符串值相等
menutrans Strings\ are\ not\ equal 字符串值不相等
menutrans Value\ is\ greater\ than 值大于
menutrans Value\ is\ greater\ equal 值大于等于
menutrans Values\ are\ equal 值相等
menutrans Values\ are\ not\ equal 值不相等
menutrans Value\ is\ less\ than 值小于
menutrans Value\ is\ less\ equal 值小于等于
menutrans ParmSub 参数替换
menutrans Substitute\ word\ if\ parm\ not\ set 如果参数没设置就替换该词
menutrans Set\ parm\ to\ word\ if\ not\ set 参数未设置就设为该词
menutrans Substitute\ word\ if\ parm\ set\ else\ nothing 如果参数设置就替换该词，否则什么都不做
menutrans If\ parm\ not\ set\ print\ word\ and\ exit 如果参数没有设置就打印该词并退出
menutrans SpShVars Shell\ 特殊变量
menutrans Number\ of\ positional\ parameters 位置参数的数目
menutrans All\ positional\ parameters\ (quoted\ spaces) 所有位置参数(quoted\ spaces)
menutrans All\ positional\ parameters\ (unquoted\ spaces) 所有位置参数(unquoted\ spaces)
menutrans Flags\ set 设置标志
menutrans Return\ code\ of\ last\ command 返回前一条命令的代码
menutrans Process\ number\ of\ this\ shell shell\ 自身进程号
menutrans Process\ number\ of\ last\ background\ command 前一条后台命令的进程号
menutrans Environ 环境变量
menutrans Mark\ created\ or\ modified\ variables\ for\ export 标记修改的或者创建的变量为导出
menutrans Exit\ when\ command\ returns\ non-zero\ status 当命令返回非零状态时退出
menutrans Disable\ file\ name\ expansion 禁用文件名拓展
menutrans Locate\ and\ remember\ commands\ when\ being\ looked\ up 当查询命令时定位并记住该命令
menutrans All\ assignment\ statements\ are\ placed\ in\ the\ environment\ for\ a\ command 所有的赋值参数被放在命令的环境中
menutrans Read\ commands\ but\ do\ not\ execute\ them 读命令但是不要执行
menutrans Exit\ after\ reading\ and\ executing\ one\ command 读并执行一个命令之后退出
menutrans Treat\ unset\ variables\ as\ an\ error\ when\ substituting 替换时把未设置命令视为错误
menutrans Print\ shell\ input\ lines\ as\ they\ are\ read 读\ shell\ 输入行的时候打印
menutrans Print\ commands\ and\ their\ arguments\ as\ they\ are\ executed 被执行时打印命令和参数
" }}}

" termdebug menu
" termdebug menuitems {{{1
" From termdebug.vim
menutrans Set\ breakpoint 设置断点
menutrans Clear\ breakpoint 清除断点
menutrans Run\ until 运行到
menutrans Evaluate 求值
menutrans WinBar 工具条
menutrans Step 单步
menutrans Next 下一个
menutrans Finish 结束
menutrans Cont 继续
menutrans Stop 停止
" }}}

" debchangelog menu
" debchangelog menuitems {{{1
" From debchangelog.vim
menutrans &Changelog 更新日志(&C)
menutrans &New\ Version 新版本(&N)
menutrans &Add\ Entry 添加条目(&A)
menutrans &Close\ Bug 关闭\ Bug(&C)
menutrans Set\ &Distribution 设置发行版(&D)
menutrans &unstable 不稳定(&U)
menutrans Set\ &Urgency 设置紧急(&U)
menutrans &low 低(&L)
menutrans &medium 中(&M)
menutrans &high 高(&H)
menutrans U&nfinalise 未完成(&N)
menutrans &Finalise 完成(&F)
" }}}

" ada menu
" ada menuitems {{{1
" From ada.vim
menutrans Tag 标签
menutrans List 列表
menutrans Jump 跳转
menutrans Create\ File 创建文件
menutrans Create\ Dir 创建目录
menutrans Highlight 高亮
menutrans Toggle\ Space\ Errors 切换空格错误
menutrans Toggle\ Lines\ Errors 切换行错误
menutrans Toggle\ Rainbow\ Color 切换彩虹颜色
menutrans Toggle\ Standard\ Types 切换标准类型
" }}}

" gnat menu
" gnat menuitems {{{1
" From gnat.vim
menutrans Build 构建
menutrans Pretty\ Print 重新格式化代码
menutrans Find 查找
menutrans Set\ Projectfile\.\.\. 设置项目文件\.\.\.
" }}}

let &cpo = s:keepcpo
unlet s:keepcpo

" vim: set ts=4 sw=4 noet fdm=marker fdc=4 :
