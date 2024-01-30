" Vim syntax file
" Language:	Vim 9.0 script
" Maintainer:	Charles E. Campbell <NcampObell@SdrPchip.AorgM-NOSPAM>
" Last Change:	May 09, 2023
" 	2023 Nov 12 by Vim Project (:let-heredoc improvements)
" 	2023 Nov 20 by Vim Project (:loadkeymap improvements)
" 	2023 Dec 06 by Vim Project (add missing assignment operators)
" 	2023 Dec 10 by Vim Project (improve variable matching)
" 	2023 Dec 21 by Vim Project (improve ex command matching)
" 	2023 Dec 30 by Vim Project (:syntax improvements)
" 	2024 Jan 14 by Vim Project (TermResponseAll autocommand)
" 	2024 Jan 15 by Vim Project (:hi ctermfont attribute)
" 	2024 Jan 23 by Vim Project (add :[23]match commands)
" 	2024 Jan 25 by Vim Project (WinNewPre autocommand)
" 	2024 Jan 27 by Vim Project (add foreach() function)
" 	2024 Jan 28 by Vim Project (improve line-continuation matching & string interpolation)
" Version:	9.0-25
" URL:	http://www.drchip.org/astronaut/vim/index.html#SYNTAX_VIM
" Automatically generated keyword lists: {{{1

" Quit when a syntax file was already loaded {{{2
if exists("b:current_syntax")
  finish
endif
let s:keepcpo= &cpo
set cpo&vim

" vimTodo: contains common special-notices for comments {{{2
" Use the vimCommentGroup cluster to add your own.
syn keyword vimTodo contained	COMBAK	FIXME	TODO	XXX
syn cluster vimCommentGroup	contains=vimTodo,@Spell

" regular vim commands {{{2
syn keyword vimCommand contained	a ar[gs] argl[ocal] bad[d] bn[ext] breakd[el] bw[ipeout] cabo[ve] cat[ch] ccl[ose] cfdo chd[ir] class cnf[ile] comc[lear] cp[revious] cstag debugg[reedy] delep dell diffg[et] dig[raphs] do dsp[lit] echoe[rr] em[enu] endfo[r] eval f[ile] fina[lly] foldd[oopen] gr[ep] helpc[lose] his[tory] ij[ump] inor j[oin] keepj[umps] lab[ove] lat lc[d] le[ft] lfir[st] lh[elpgrep] lmak[e] loadk lp[revious] luado ma[rk] mk[exrc] mz[scheme] new nore on[ly] pc[lose] pp[op] promptf[ind] ptj[ump] pu[t] py3f[ile] pyx r[ead] redrawt[abline] ri[ght] rundo sIl sal[l] sbf[irst] sc scp se[t] sg sgn sie sip sme snoremenu spelli[nfo] spr[evious] sri star[tinsert] sts[elect] sus[pend] syncbind tabN[ext] tabl[ast] tabr[ewind] tcld[o] tj[ump] tlu tno[remap] tu[nmenu] undol[ist] v vim9[cmd] vs[plit] win[size] wq xmapc[lear] xr[estore]
syn keyword vimCommand contained	ab arga[dd] argu[ment] balt bo[tright] breakl[ist] cN[ext] cad[dbuffer] cb[uffer] cd cfir[st] che[ckpath] cle[arjumps] cnor comp[iler] cpf[ile] cun def deletel delm[arks] diffo[ff] dir doau e[dit] echom[sg] en[dif] endinterface ex files fini[sh] folddoc[losed] grepa[dd] helpf[ind] hor[izontal] il[ist] interface ju[mps] keepp[atterns] lad[dexpr] later lch[dir] lefta[bove] lg[etfile] lhi[story] lmapc[lear] loadkeymap lpf[ile] luafile mak[e] mks[ession] mzf[ile] nmapc[lear] nos[wapfile] opt[ions] pe[rl] pre[serve] promptr[epl] ptl[ast] public py[thon] pyxdo rec[over] reg[isters] rightb[elow] rv[iminfo] sIn san[dbox] sbl[ast] scI scr[iptnames] setf[iletype] sgI sgp sig sir smenu so[urce] spellr[are] sr srl startg[replace] substitutepattern sv[iew] syntime tabc[lose] tabm[ove] tabs tclf[ile] tl[ast] tlunmenu to[pleft] tunma[p] unh[ide] ve[rsion] vim9s[cript] wN[ext] winc[md] wqa[ll] xme xunme
syn keyword vimCommand contained	abc[lear] argd[elete] as[cii] bd[elete] bp[revious] bro[wse] cNf[ile] cadde[xpr] cbe[fore] cdo cg[etfile] checkt[ime] clo[se] co[py] con[tinue] cq[uit] cuna[bbrev] defc[ompile] deletep delp diffp[atch] disa[ssemble] doaut ea echon endclass endt[ry] exi[t] filet fir[st] foldo[pen] gui helpg[rep] i imapc[lear] intro k lN[ext] laddb[uffer] lb[uffer] lcl[ose] leg[acy] lgetb[uffer] ll lne[xt] loc[kmarks] lr[ewind] lv[imgrep] marks mksp[ell] n[ext] noa nu[mber] ownsyntax ped[it] prev[ious] ps[earch] ptn[ext] pw[d] pydo pyxfile red[o] res[ize] ru[ntime] sI sIp sav[eas] sbm[odified] sce scripte[ncoding] setg[lobal] sgc sgr sign sl[eep] smile sor[t] spellr[epall] srI srn startr[eplace] substituterepeat sw[apname] t tabd[o] tabn[ext] tags te[aroff] tlm tm[enu] tp[revious] type unl verb[ose] vim[grep] w[rite] windo wundo xmenu xunmenu
syn keyword vimCommand contained	abo[veleft] argded[upe] au bel[owright] br[ewind] bufdo c[hange] caddf[ile] cbel[ow] ce[nter] cgetb[uffer] chi[story] cmapc[lear] col[der] conf[irm] cr[ewind] cw[indow] defer deletl dep diffpu[t] dj[ump] dp earlier echow[indow] enddef endw[hile] exp filetype fix[del] for gvim helpt[ags] ia imp is[earch] kee[pmarks] lNf[ile] laddf[ile] lbe[fore] lcs lex[pr] lgete[xpr] lla[st] lnew[er] lockv[ar] ls lvimgrepa[dd] mat[ch] mkv[imrc] nb[key] noautocmd o[pen] p[rint] perld[o] pro ptN[ext] ptp[revious] py3 pyf[ile] q[uit] redi[r] ret[ab] rub[y] sIc sIr sbN[ext] sbn[ext] scg scriptv[ersion] setl[ocal] sge sh[ell] sil[ent] sla[st] sn[ext] sp[lit] spellr[rare] src srp static sun[hide] sy tN[ext] tabe[dit] tabnew tc[d] ter[minal] tlmenu tma[p] tr[ewind] u[ndo] unlo[ckvar] vert[ical] vimgrepa[dd] wa[ll] winp[os] wv[iminfo] xnoreme xwininfo
syn keyword vimCommand contained	abstract argdo bN[ext] bf[irst] brea[k] buffers ca caf[ter] cbo[ttom] cex[pr] cgete[xpr] cl[ist] cn[ext] colo[rscheme] cons[t] cs d[elete] delc[ommand] deletp di[splay] diffs[plit] dl dr[op] ec el[se] endenum ene[w] export filt[er] fo[ld] fu[nction] h[elp] hi iabc[lear] import isp[lit] keepa l[ist] laf[ter] lbel[ow] lcscope lf[ile] lgr[ep] lli[st] lnf[ile] lol[der] lt[ag] lw[indow] menut[ranslate] mkvie[w] nbc[lose] noh[lsearch] ol[dfiles] pa[ckadd] po[p] prof[ile] pta[g] ptr[ewind] py3do python3 qa[ll] redr[aw] retu[rn] rubyd[o] sIe sN[ext] sb[uffer] sbp[revious] sci scs sf[ind] sgi si sim[alt] sm[agic] sno[magic] spe[llgood] spellu[ndo] sre[wind] st[op] stj[ump] sunme syn ta[g] tabf[ind] tabo[nly] tch[dir] tf[irst] tln tmapc[lear] try una[bbreviate] uns[ilent] vi[sual] viu[sage] wh[ile] wn[ext] x[it] xnoremenu y[ank]
syn keyword vimCommand contained	addd arge[dit] b[uffer] bl[ast] breaka[dd] bun[load] cabc[lear] cal[l] cc cf[ile] changes cla[st] cnew[er] com cope[n] cscope debug delel delf[unction] dif[fupdate] difft[his] dli[st] ds[earch] echoc[onsole] elsei[f] endf[unction] enum exu[sage] fin[d] foldc[lose] go[to] ha[rdcopy] hid[e] if in iuna[bbrev] keepalt la[st] lan[guage] lbo[ttom] ld[o] lfdo lgrepa[dd] lma lo[adview] lop[en] lua m[ove] mes[sages] mod[e] nbs[tart] nor omapc[lear] packl[oadall] popu[p] profd[el] ptf[irst] pts[elect] py3f[ile] pythonx quita[ll] redraws[tatus] rew[ind] rubyf[ile] sIg sa[rgument] sba[ll] sbr[ewind] scl scscope sfir[st] sgl sic sin sm[ap] snoreme spelld[ump] spellw[rong] srg sta[g] stopi[nsert] sunmenu sync tab tabfir[st] tabp[revious] tcl th[row] tlnoremenu tn[ext] ts[elect] undoj[oin] up[date] vie[w] vne[w] wi wp[revious] xa[ll] xprop z[^.=]
syn keyword vimCommand contained	al[l] argg[lobal] ba[ll] bm[odified]
syn match   vimCommand contained	"\<z[-+^.=]\=\>"
syn keyword vimStdPlugin contained	Arguments Asm Break Cfilter Clear Continue DiffOrig Evaluate Finish Gdb Lfilter Man N[ext] Over P[rint] Program Run S Source Step Stop Termdebug TermdebugCommand TOhtml Until Winbar XMLent XMLns

" vimOptions are caught only when contained in a vimSet {{{2
syn keyword vimOption contained	acd ambw arshape aw backupskip beval bk bri bufhidden cdh ci cinsd cms commentstring conceallevel cpt cscopetagorder csto cursorlineopt dg dir ed enc equalprg expandtab fdls fex fileignorecase fml foldlevel formatexpr gcr gli guifont guitabtooltip hidden hlg imactivatefunc imi inc inex isident keymap langmap linebreak lm lsp makeencoding maxmem mh mmp more mousemoveevent mzq numberwidth opfunc patchexpr pfn pp printfont pumwidth pythonthreehome re restorescreen ro rulerformat scl scs sft shellslash shortmess showtabline slm smoothscroll spell spl srr statusline sw sxq tag tal tenc termwintype tgst titleold tpm ttm tw udir ur verbose viminfofile warn wfh wildchar wim winminheight wmh write
syn keyword vimOption contained	ai anti asd awa balloondelay bevalterm bkc briopt buflisted cdhome cin cinw co compatible confirm crb cscopeverbose csverb cwh dict directory edcompatible encoding errorbells exrc fdm ff filetype fmr foldlevelstart formatlistpat gd go guifontset helpfile highlight hls imactivatekey iminsert include inf isk keymodel langmenu lines lmap luadll makeprg maxmempattern mis mmt mouse mouses mzquantum nuw osfiletype patchmode ph preserveindent printheader pvh pyx readonly revins rop runtimepath scr sect sh shelltemp shortname shq sloc sms spellcapcheck splitbelow ss stl swapfile syn tagbsearch tb term terse thesaurus titlestring tr tty twk ul ut verbosefile virtualedit wb wfw wildcharm winaltkeys winminwidth wmnu writeany
syn keyword vimOption contained	akm antialias autochdir background ballooneval bex bl brk buftype cdpath cindent cinwords cocu complete copyindent cryptmethod csl cuc debug dictionary display ef endoffile errorfile fcl fdn ffs fillchars fo foldmarker formatoptions gdefault gp guifontwide helpheight history hlsearch imaf ims includeexpr infercase iskeyword keyprotocol langnoremap linespace lnr lw mat maxmemtot mkspellmem mod mousef mouseshape mzschemedll odev pa path pheader previewheight printmbcharset pvp pyxversion redrawtime ri rs sb scroll sections shcf shelltype showbreak si sm sn spellfile splitkeep ssl stmp swapsync synmaxcol tagcase tbi termbidi textauto thesaurusfunc tl ts ttybuiltin tws undodir varsofttabstop vfile visualbell wc wh wildignore wincolor winptydll wmw writebackup
syn keyword vimOption contained	al ar autoindent backspace balloonevalterm bexpr bo browsedir casemap cedit cink clipboard cole completefunc cot cscopepathcomp cspc cul deco diff dy efm endofline errorformat fcs fdo fic fixendofline foldclose foldmethod formatprg gfm grepformat guiheadroom helplang hk ic imak imsearch incsearch insertmode isp keywordprg langremap lisp loadplugins lz matchpairs mco ml modeline mousefocus mouset mzschemegcdll oft packpath pdev pi previewpopup printmbfont pvw qe regexpengine rightleft rtp sbo scrollbind secure shell shellxescape showcmd sidescroll smartcase so spelllang splitright ssop sts swb syntax tagfunc tbidi termencoding textmode tildeop tm tsl ttyfast twsl undofile vartabstop vi vop wcm whichwrap wildignorecase window winwidth wop writedelay
syn keyword vimOption contained	aleph arab autoread backup balloonexpr bg bomb bs cb cf cinkeys cm colorcolumn completeopt cp cscopeprg csprg culopt def diffexpr ea ei eof esckeys fdc fdt fileencoding fixeol foldcolumn foldminlines fp gfn grepprg guiligatures hf hkmap icon imc imsf inde is isprint km laststatus lispoptions lop ma matchtime mef mle modelineexpr mousehide mousetime nf ofu para penc pm previewwindow printoptions pw qftf relativenumber rightleftcmd ru sbr scrollfocus sel shellcmdflag shellxquote showcmdloc sidescrolloff smartindent softtabstop spelloptions spo st su swf ta taglength tbis termguicolors textwidth timeout to tsr ttym twt undolevels vb viewdir vsts wcr wi wildmenu winfixheight wiv wrap ws
syn keyword vimOption contained	allowrevins arabic autoshelldir backupcopy bdir bh breakat bsdir cc cfu cino cmdheight columns completepopup cpo cscopequickfix csqf cursorbind define diffopt ead ek eol et fde fen fileencodings fk foldenable foldnestmax fs gfs gtl guioptions hh hkmapp iconstring imcmdline imst indentexpr isf joinspaces jumpoptions kmp lazyredraw lispwords lpl macatsui maxcombine menc mls modelines mousem mp nrformats omnifunc paragraphs perldll pmbcs printdevice prompt pythondll quickfixtextfunc remap rl rubydll sc scrolljump selection shellpipe shiftround showfulltag signcolumn smarttab sol spellsuggest spr sta sua switchbuf tabline tagrelative tbs termwinkey tf timeoutlen toolbar tsrfu ttymouse tx undoreload vbs viewoptions vts wd wic wildmode winfixwidth wiw wrapmargin ww
syn keyword vimOption contained	altkeymap arabicshape autowrite backupdir bdlay bin breakindent bsk ccv ch cinoptions cmdwinheight com completeslash cpoptions cscoperelative csre cursorcolumn delcombine digraph eadirection emo ep eventignore fdi fenc fileformat fkmap foldexpr foldopen fsync gfw gtt guipty hi hkp ignorecase imd imstatusfunc indentkeys isfname js jop kp lbr list lrm magic maxfuncdepth menuitems mm modifiable mousemev mps nu opendevice paste pex pmbfn printencoding pt pythonhome quoteescape renderoptions rlc ruf scb scrolloff selectmode shellquote shiftwidth showmatch siso smc sp spf sps stal suffixes sws tabpagemax tags tc termwinscroll tfu title toolbariconsize ttimeout ttyscroll uc updatecount vdir vif wa weirdinvert wig wildoptions winheight wm wrapscan xtermcodes
syn keyword vimOption contained	ambiwidth ari autowriteall backupext belloff binary breakindentopt bt cd charconvert cinscopedecls cmp comments concealcursor cpp cscopetag cst cursorline dex dip eb emoji equalalways ex fdl fencs fileformats flp foldignore foldtext ft ghr guicursor guitablabel hid hl im imdisable imstyle indk isi key kpc lcs listchars ls makeef maxmapdepth mfd mmd modified mousemodel msm number operatorfunc pastetoggle pexpr popt printexpr pumheight pythonthreedll rdt report rnu ruler scf scrollopt sessionoptions shellredir shm showmode sj smd spc spk sr startofline suffixesadd sxe tabstop tagstack tcldll termwinsize tgc titlelen top ttimeoutlen ttytype udf updatetime ve viminfo wak

" vimOptions: These are the turn-off setting variants {{{2
syn keyword vimOption contained	noacd noallowrevins noantialias noarabic noarshape noautoindent noautowrite noawa noballoonevalterm nobin nobl nobri nocdhome nocin noconfirm nocrb nocscopeverbose nocsverb nocursorbind nodeco nodiff noeb noek noendoffile noeol noesckeys noexpandtab nofic nofixeol nofoldenable nogd nohid nohkmap nohls noicon noimc noimdisable noinfercase nojoinspaces nolangremap nolinebreak nolnr nolrm nomacatsui noml nomodeline nomodified nomousefocus nomousemoveevent noodev nopi noprompt norelativenumber norevins norl nors noruler nosc noscf noscrollfocus nosecure noshellslash noshiftround noshowcmd noshowmatch nosi nosmartcase nosmarttab nosmoothscroll nosn nospell nosplitright nosr nosta nostmp noswf notagbsearch notagstack notbidi notermbidi noterse notextmode notgc notildeop notitle notop nottimeout nottyfast noudf novb nowa nowb nowfh nowic nowildmenu nowinfixwidth nowmnu nowrapscan nowriteany nows
syn keyword vimOption contained	noai noaltkeymap noar noarabicshape noasd noautoread noautowriteall nobackup nobeval nobinary nobomb nobuflisted nocf nocindent nocopyindent nocscoperelative nocsre nocuc nocursorcolumn nodelcombine nodigraph noed noemo noendofline noequalalways noet noexrc nofileignorecase nofk nofs nogdefault nohidden nohkmapp nohlsearch noignorecase noimcmdline noincsearch noinsertmode nojs nolazyredraw nolisp noloadplugins nolz nomagic nomle nomodelineexpr nomore nomousehide nonu noopendevice nopreserveindent nopvw noremap nori nornu noru nosb noscb noscrollbind noscs nosft noshelltemp noshortname noshowfulltag noshowmode nosm nosmartindent nosmd nosms nosol nosplitbelow nospr nossl nostartofline noswapfile nota notagrelative notbi notbs notermguicolors notextauto notf notgst notimeout noto notr nottybuiltin notx noundofile novisualbell nowarn noweirdinvert nowfw nowildignorecase nowinfixheight nowiv nowrap nowrite nowritebackup noxtermcodes
syn keyword vimOption contained	noakm noanti noarab noari noautochdir noautoshelldir noaw noballooneval nobevalterm nobk nobreakindent nocdh noci nocompatible nocp nocscopetag nocst nocul nocursorline nodg noea noedcompatible noemoji noeof noerrorbells noex nofen nofixendofline nofkmap nofsync noguipty nohk nohkp noic noim noimd noinf nois nolangnoremap nolbr nolist nolpl noma nomh nomod nomodifiable nomousef nomousemev nonumber nopaste nopreviewwindow noreadonly norestorescreen norightleft noro

" vimOptions: These are the invertible variants {{{2
syn keyword vimOption contained	invacd invallowrevins invantialias invarabic invarshape invautoindent invautowrite invawa invballoonevalterm invbin invbl invbri invcdhome invcin invconfirm invcrb invcscopeverbose invcsverb invcursorbind invdeco invdiff inveb invek invendoffile inveol invesckeys invexpandtab invfic invfixeol invfoldenable invgd invhid invhkmap invhls invicon invimc invimdisable invinfercase invjoinspaces invlangremap invlinebreak invlnr invlrm invmacatsui invml invmodeline invmodified invmousefocus invmousemoveevent invodev invpi invprompt invrelativenumber invrevins invrl invrs invruler invsc invscf invscrollfocus invsecure invshellslash invshiftround invshowcmd invshowmatch invsi invsmartcase invsmarttab invsmoothscroll invsn invspell invsplitright invsr invsta invstmp invswf invtagbsearch invtagstack invtbidi invtermbidi invterse invtextmode invtgc invtildeop invtitle invtop invttimeout invttyfast invudf invvb invwa invwb invwfh invwic invwildmenu invwinfixwidth invwmnu invwrapscan invwriteany invws
syn keyword vimOption contained	invai invaltkeymap invar invarabicshape invasd invautoread invautowriteall invbackup invbeval invbinary invbomb invbuflisted invcf invcindent invcopyindent invcscoperelative invcsre invcuc invcursorcolumn invdelcombine invdigraph inved invemo invendofline invequalalways invet invexrc invfileignorecase invfk invfs invgdefault invhidden invhkmapp invhlsearch invignorecase invimcmdline invincsearch invinsertmode invjs invlazyredraw invlisp invloadplugins invlz invmagic invmle invmodelineexpr invmore invmousehide invnu invopendevice invpreserveindent invpvw invremap invri invrnu invru invsb invscb invscrollbind invscs invsft invshelltemp invshortname invshowfulltag invshowmode invsm invsmartindent invsmd invsms invsol invsplitbelow invspr invssl invstartofline invswapfile invta invtagrelative invtbi invtbs invtermguicolors invtextauto invtf invtgst invtimeout invto invtr invttybuiltin invtx invundofile invvisualbell invwarn invweirdinvert invwfw invwildignorecase invwinfixheight invwiv invwrap invwrite invwritebackup invxtermcodes
syn keyword vimOption contained	invakm invanti invarab invari invautochdir invautoshelldir invaw invballooneval invbevalterm invbk invbreakindent invcdh invci invcompatible invcp invcscopetag invcst invcul invcursorline invdg invea invedcompatible invemoji inveof inverrorbells invex invfen invfixendofline invfkmap invfsync invguipty invhk invhkp invic invim invimd invinf invis invlangnoremap invlbr invlist invlpl invma invmh invmod invmodifiable invmousef invmousemev invnumber invpaste invpreviewwindow invreadonly invrestorescreen invrightleft invro

" termcap codes (which can also be set) {{{2
syn keyword vimOption contained	t_8b t_8u t_AF t_AL t_bc t_BE t_ce t_cl t_Co t_Cs t_CV t_db t_DL t_Ds t_EI t_F2 t_F4 t_F6 t_F8 t_fd t_fs t_IE t_k1 t_k2 t_K3 t_K4 t_K5 t_K6 t_K7 t_K8 t_K9 t_kb t_KB t_kd t_KD t_KE t_KG t_KH t_KI t_KK t_KL t_kN t_kP t_kr t_ks t_ku t_le t_mb t_md t_me t_mr t_ms t_nd t_op t_PE t_PS t_RB t_RC t_RF t_Ri t_RI t_RK t_RS t_RT t_RV t_Sb t_SC t_se t_Sf t_SH t_Si t_SI t_so t_sr t_SR t_ST t_te t_Te t_TE t_ti t_TI t_ts t_Ts t_u7 t_ue t_us t_Us t_ut t_vb t_ve t_vi t_vs t_VS t_WP t_WS t_XM t_xn t_xs t_ZH t_ZR
syn keyword vimOption contained	t_8f t_AB t_al t_AU t_BD t_cd t_Ce t_cm t_cs t_CS t_da t_dl t_ds t_EC t_F1 t_F3 t_F5 t_F7 t_F9 t_fe t_GP t_IS t_K1 t_k3 t_k4 t_k5 t_k6 t_k7 t_k8 t_k9 t_KA t_kB t_KC t_kD t_ke t_KF t_kh t_kI t_KJ t_kl
syn match   vimOption contained	"t_%1"
syn match   vimOption contained	"t_#2"
syn match   vimOption contained	"t_#4"
syn match   vimOption contained	"t_@7"
syn match   vimOption contained	"t_*7"
syn match   vimOption contained	"t_&8"
syn match   vimOption contained	"t_%i"
syn match   vimOption contained	"t_k;"

" unsupported settings: some were supported by vi but don't do anything in vim {{{2
" others have been dropped along with msdos support
syn keyword vimErrSetting contained	bioskey biosk conskey consk autoprint beautify flash graphic hardtabs mesg novice open op optimize redraw slow slowopen sourceany w300 w1200 w9600 hardtabs ht nobioskey nobiosk noconskey noconsk noautoprint nobeautify noflash nographic nohardtabs nomesg nonovice noopen noop nooptimize noredraw noslow noslowopen nosourceany now300 now1200 now9600 w1200 w300 w9600

" AutoCmd Events {{{2
syn case ignore
syn keyword vimAutoEvent contained	BufAdd BufDelete BufFilePost BufHidden BufNew BufRead BufReadPost BufUnload BufWinLeave BufWrite BufWritePost CmdlineChanged CmdlineLeave CmdwinEnter ColorScheme CompleteChanged CompleteDonePre CursorHoldI CursorMovedI DiffUpdated DirChanged DirChangedPre EncodingChanged ExitPre FileAppendCmd FileAppendPost FileAppendPre FileChangedRO FileChangedShell FileChangedShellPost FileEncoding FileExplorer FileReadCmd FileReadPost FileReadPre FileType FileWriteCmd FileWritePost FileWritePre FilterReadPost FilterReadPre FilterWritePost FilterWritePre FocusGained FocusLost FuncUndefined GUIEnter GUIFailed InsertChange InsertCharPre InsertEnter InsertLeave InsertLeavePre MenuPopup ModeChanged OptionSet QuickFixCmdPost QuickFixCmdPre QuitPre RemoteReply SafeState SafeStateAgain SessionLoadPost ShellCmdPost ShellFilterPost SigUSR1 SourceCmd SourcePost SourcePre SpellFileMissing StdinReadPost StdinReadPre SwapExists Syntax TabClosed TabEnter TabLeave TabNew TermChanged TerminalOpen TerminalWinOpen TermResponse TermResponseAll TextChanged TextChangedI TextChangedP TextChangedT TextYankPost User VimEnter VimLeave VimLeavePre VimResized VimResume VimSuspend WinClosed WinEnter WinLeave WinNewPre WinNew WinResized WinScrolled
syn keyword vimAutoEvent contained	BufCreate BufEnter BufFilePre BufLeave BufNewFile BufReadCmd BufReadPre BufWinEnter BufWipeout BufWriteCmd BufWritePre CmdlineEnter CmdUndefined CmdwinLeave ColorSchemePre CompleteDone CursorHold CursorMoved

" Highlight commonly used Groupnames {{{2
syn keyword vimGroup contained	Comment Constant String Character Number Boolean Float Identifier Function Statement Conditional Repeat Label Operator Keyword Exception PreProc Include Define Macro PreCondit Type StorageClass Structure Typedef Special SpecialChar Tag Delimiter SpecialComment Debug Underlined Ignore Error Todo

" Default highlighting groups {{{2
syn keyword vimHLGroup contained	ColorColumn CurSearch Cursor CursorColumn CursorIM CursorLine CursorLineFold CursorLineNr CursorLineSign DiffAdd DiffChange DiffDelete DiffText Directory EndOfBuffer ErrorMsg FoldColumn Folded IncSearch LineNr LineNrAbove LineNrBelow MatchParen Menu MessageWindow ModeMsg MoreMsg NonText Normal Pmenu PmenuExtra PmenuExtraSel PmenuKind PmenuKindSel PmenuSbar PmenuSel PmenuThumb Question QuickFixLine Scrollbar Search SignColumn SpecialKey SpellBad SpellCap SpellLocal SpellRare StatusLine StatusLineNC StatusLineTerm StatusLineTermNC TabLine TabLineFill TabLineSel Terminal Title Tooltip VertSplit Visual VisualNOS WarningMsg WildMenu
syn match vimHLGroup contained	"Conceal"
syn case match

" Function Names {{{2
syn keyword vimFuncName contained	abs argc assert_equal assert_match atan balloon_show bufexists bufwinid ceil ch_canread ch_getbufnr ch_read ch_status complete_check count deletebufline digraph_set eval exists_compiled extendnew findfile fnameescape foldtextresult get getchangelist getcmdcompltype getcompletion getfperm getline getpid getscriptinfo getwininfo glob2regpat histadd hlID indexof inputsecret isinf job_setoptions js_encode libcall list2str log10 mapnew matchdelete matchstrpos mzeval popup_atcursor popup_filter_menu popup_getpos popup_move pow prompt_setinterrupt prop_find prop_type_delete py3eval readblob reg_executing remote_expr remote_startserver reverse screenchars search searchpos setcellwidths setcursorcharpos setmatches settabwinvar shiftwidth sign_place simplify sound_clear spellbadword state strcharpart stridx strridx substitute synID systemlist taglist term_dumpload term_getcursor term_getstatus term_scrape term_setrestore test_autochdir test_gui_event test_null_dict test_null_string test_settime timer_pause toupper typename values winbufnr win_getid win_id2win winnr win_splitmove
syn keyword vimFuncName contained	acos argidx assert_equalfile assert_nobeep atan2 balloon_split buflisted bufwinnr changenr ch_close ch_getjob ch_readblob cindent complete_info cscope_connection did_filetype digraph_setlist eventhandler exp feedkeys flatten fnamemodify foreground getbufinfo getchar getcmdline getcurpos getfsize getloclist getpos gettabinfo getwinpos globpath histdel hlset input insert islocked job_start json_decode libcallnr listener_add luaeval mapset matchend max nextnonblank popup_beval popup_filter_yesno popup_hide popup_notification prevnonblank prompt_setprompt prop_list prop_type_get pyeval readdir reg_recording remote_foreground remove round screencol searchcount server2client setcharpos setenv setpos settagstack sign_define sign_placelist sin soundfold spellsuggest str2float strchars string strtrans swapfilelist synIDattr tabpagebuflist tan term_dumpwrite term_getjob term_gettitle term_sendkeys term_setsize test_feedinput test_ignore_error test_null_function test_option_not_set test_srand_seed timer_start tr undofile virtcol wincol win_gettype winlayout winrestcmd winwidth
syn keyword vimFuncName contained	add arglistid assert_exception assert_notequal autocmd_add blob2list bufload byte2line char2nr ch_close_in ch_info ch_readraw clearmatches confirm cursor diff_filler echoraw executable expand filereadable flattennew foldclosed fullcommand getbufline getcharmod getcmdpos getcursorcharpos getftime getmarklist getqflist gettabvar getwinposx has histget hostname inputdialog interrupt isnan job_status json_encode line listener_flush map match matchfuzzy menu_info nr2char popup_clear popup_findecho popup_list popup_setoptions printf prop_add prop_remove prop_type_list pyxeval readdirex reltime remote_peek rename rubyeval screenpos searchdecl serverlist setcharsearch setfperm setqflist setwinvar sign_getdefined sign_undefine sinh sound_playevent split str2list strdisplaywidth strlen strutf16len swapinfo synIDtrans tabpagenr tanh term_getaltscreen term_getline term_gettty term_setansicolors term_start test_garbagecollect_now test_mswin_event test_null_job test_override test_unknown timer_stop trim undotree virtcol2col windowsversion win_gotoid winline winrestview wordcount
syn keyword vimFuncName contained	and argv assert_fails assert_notmatch autocmd_delete browse bufloaded byteidx charclass chdir ch_log ch_sendexpr col copy debugbreak diff_hlID empty execute expandcmd filewritable float2nr foldclosedend funcref getbufoneline getcharpos getcmdscreenpos getcwd getftype getmatches getreg gettabwinvar getwinposy has_key histnr iconv inputlist invert items job_stop keys line2byte listener_remove maparg matchadd matchfuzzypos min or popup_close popup_findinfo popup_locate popup_settext prompt_getprompt prop_add_list prop_type_add pum_getpos rand readfile reltimefloat remote_read repeat screenattr screenrow searchpair setbufline setcmdline setline setreg sha256 sign_getplaced sign_unplace slice sound_playfile sqrt str2nr strftime strpart strwidth swapname synstack tabpagewinnr tempname term_getansicolors term_getscrolled terminalprops term_setapi term_wait test_garbagecollect_soon test_null_blob test_null_list test_refcount test_void timer_stopall trunc uniq visualmode win_execute winheight win_move_separator winsaveview writefile
syn keyword vimFuncName contained	append asin assert_false assert_report autocmd_get browsedir bufname byteidxcomp charcol ch_evalexpr ch_logfile ch_sendraw complete cos deepcopy digraph_get environ exepath expr10 filter floor foldlevel function getbufvar getcharsearch getcmdtype getenv getimstatus getmousepos getreginfo gettagstack getwinvar haslocaldir hlexists indent inputrestore isabsolutepath job_getchannel join keytrans lispindent localtime mapcheck matchaddpos matchlist mkdir pathshorten popup_create popup_findpreview popup_menu popup_show prompt_setcallback prop_clear prop_type_change pumvisible range reduce reltimestr remote_send resolve screenchar screenstring searchpairpos setbufvar setcmdpos setloclist settabvar shellescape sign_jump sign_unplacelist sort sound_stop srand strcharlen strgetchar strptime submatch synconcealed system tagfiles term_dumpdiff term_getattr term_getsize term_list term_setkill test_alloc_fail test_getvalue test_null_channel test_null_partial test_setmouse timer_info tolower type utf16idx wildmenumode win_findbuf win_id2tabwin win_move_statusline win_screenpos xor
syn keyword vimFuncName contained	appendbufline assert_beeps assert_inrange assert_true balloon_gettext bufadd bufnr call charidx ch_evalraw ch_open ch_setoptions complete_add cosh delete digraph_getlist escape exists extend finddir fmod foldtext garbagecollect getcellwidths getcharstr getcmdwintype getfontname getjumplist getmouseshape getregtype gettext glob hasmapto hlget index inputsave isdirectory job_info js_decode len list2blob log maplist matcharg matchstr mode perleval popup_dialog popup_getoptions

"--- syntax here and above generated by mkvimvim ---

syn keyword vimCommand contained	2mat[ch] 3mat[ch]
syn keyword vimFuncName contained	foreach

" Special Vim Highlighting (not automatic) {{{1

" Set up folding commands for this syntax highlighting file {{{2
if exists("g:vimsyn_folding") && g:vimsyn_folding =~# '[afhlmpPrt]'
 if g:vimsyn_folding =~# 'a'
  com! -nargs=* VimFolda <args> fold
 else
  com! -nargs=* VimFolda <args>
 endif
 if g:vimsyn_folding =~# 'f'
  com! -nargs=* VimFoldf <args> fold
 else
  com! -nargs=* VimFoldf <args>
 endif
 if g:vimsyn_folding =~# 'h'
  com! -nargs=* VimFoldh <args> fold
 else
  com! -nargs=* VimFoldh <args>
 endif
 if g:vimsyn_folding =~# 'l'
  com! -nargs=* VimFoldl <args> fold
 else
  com! -nargs=* VimFoldl <args>
 endif
 if g:vimsyn_folding =~# 'm'
  com! -nargs=* VimFoldm <args> fold
 else
  com! -nargs=* VimFoldm <args>
 endif
 if g:vimsyn_folding =~# 'p'
  com! -nargs=* VimFoldp <args> fold
 else
  com! -nargs=* VimFoldp <args>
 endif
 if g:vimsyn_folding =~# 'P'
  com! -nargs=* VimFoldP <args> fold
 else
  com! -nargs=* VimFoldP <args>
 endif
 if g:vimsyn_folding =~# 'r'
  com! -nargs=* VimFoldr <args> fold
 else
  com! -nargs=* VimFoldr <args>
 endif
 if g:vimsyn_folding =~# 't'
  com! -nargs=* VimFoldt <args> fold
 else
  com! -nargs=* VimFoldt <args>
 endif
else
 com! -nargs=*	VimFolda	<args>
 com! -nargs=*	VimFoldf	<args>
 com! -nargs=*	VimFoldh	<args>
 com! -nargs=*	VimFoldl	<args>
 com! -nargs=*	VimFoldm	<args>
 com! -nargs=*	VimFoldp	<args>
 com! -nargs=*	VimFoldP	<args>
 com! -nargs=*	VimFoldr	<args>
 com! -nargs=*	VimFoldt	<args>
endif

" commands not picked up by the generator (due to non-standard format) {{{2
syn keyword vimCommand contained	py3

" Deprecated variable options {{{2
if exists("g:vim_minlines")
 let g:vimsyn_minlines= g:vim_minlines
endif
if exists("g:vim_maxlines")
 let g:vimsyn_maxlines= g:vim_maxlines
endif
if exists("g:vimsyntax_noerror")
 let g:vimsyn_noerror= g:vimsyntax_noerror
endif

" Variable options {{{2
if exists("g:vim_maxlines")
 let s:vimsyn_maxlines= g:vim_maxlines
else
 let s:vimsyn_maxlines= 60
endif

" Numbers {{{2
" =======
syn match vimNumber	'\<\d\+\%(\.\d\+\%([eE][+-]\=\d\+\)\=\)\=' skipwhite nextgroup=vimGlobal,vimSubst,vimCommand,vimComment,vim9Comment
syn match vimNumber	'-\d\+\%(\.\d\+\%([eE][+-]\=\d\+\)\=\)\='  skipwhite nextgroup=vimGlobal,vimSubst,vimCommand,vimComment,vim9Comment
syn match vimNumber	'\<0[xX]\x\+'		       skipwhite nextgroup=vimGlobal,vimSubst,vimCommand,vimComment,vim9Comment
syn match vimNumber	'\%(^\|\A\)\zs#\x\{6}'             	       skipwhite nextgroup=vimGlobal,vimSubst,vimCommand,vimComment,vim9Comment
syn match vimNumber	'\<0[zZ][a-zA-Z0-9.]\+'                    skipwhite nextgroup=vimGlobal,vimSubst,vimCommand,vimComment,vim9Comment
syn match vimNumber	'0[0-7]\+'		       skipwhite nextgroup=vimGlobal,vimSubst,vimCommand,vimComment,vim9Comment
syn match vimNumber	'0[bB][01]\+'		       skipwhite nextgroup=vimGlobal,vimSubst,vimCommand,vimComment,vim9Comment

" All vimCommands are contained by vimIsCommand. {{{2
syn match vimCmdSep	"[:|]\+"	skipwhite nextgroup=vimAbb,vimAddress,vimAutoCmd,vimAugroup,vimBehave,vimEcho,vimEchoHL,vimExecute,vimIsCommand,vimExtCmd,vimFilter,vimGlobal,vimHighlight,vimLet,vimMap,vimMark,vimNorm,vimSet,vimSyntax,vimUnlet,vimUnmap,vimUserCmd
syn match vimIsCommand	"\<\%(\h\w*\|[23]mat\%[ch]\)\>"	contains=vimCommand
syn match vimVar	      contained	"\<\h[a-zA-Z0-9#_]*\>"
syn match vimVar		"\<[bwglstav]:\h[a-zA-Z0-9#_]*\>"
syn match vimVar	      	"\s\zs&\%([lg]:\)\=\a\+\>"
syn match vimVar		"\s\zs&t_\S[a-zA-Z0-9]\>"
syn match vimVar        	"\s\zs&t_k;"
syn match vimFBVar      contained   "\<[bwglstav]:\h[a-zA-Z0-9#_]*\>"
syn keyword vimCommand  contained	in

" Insertions And Appends: insert append {{{2
"   (buftype != nofile test avoids having append, change, insert show up in the command window)
" =======================
if &buftype != 'nofile'
 syn region vimInsert	matchgroup=vimCommand start="^[: \t]*\(\d\+\(,\d\+\)\=\)\=a\%[ppend]$"		matchgroup=vimCommand end="^\.$""
 syn region vimInsert	matchgroup=vimCommand start="^[: \t]*\(\d\+\(,\d\+\)\=\)\=c\%[hange]$"		matchgroup=vimCommand end="^\.$""
 syn region vimInsert	matchgroup=vimCommand start="^[: \t]*\(\d\+\(,\d\+\)\=\)\=i\%[nsert]$"		matchgroup=vimCommand end="^\.$""
endif

" Behave! {{{2
" =======
syn match   vimBehave	"\<be\%[have]\>" skipwhite nextgroup=vimBehaveModel,vimBehaveError
syn keyword vimBehaveModel contained	mswin	xterm
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_nobehaveerror")
 syn match   vimBehaveError contained	"[^ ]\+"
endif

" Filetypes {{{2
" =========
syn match   vimFiletype	"\<filet\%[ype]\(\s\+\I\i*\)*"	skipwhite contains=vimFTCmd,vimFTOption,vimFTError
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_vimFTError")
 syn match   vimFTError  contained	"\I\i*"
endif
syn keyword vimFTCmd    contained	filet[ype]
syn keyword vimFTOption contained	detect indent off on plugin

" Augroup : vimAugroupError removed because long augroups caused sync'ing problems. {{{2
" ======= : Trade-off: Increasing synclines with slower editing vs augroup END error checking.
syn cluster vimAugroupList	contains=vimAugroup,vimIsCommand,vimUserCmd,vimExecute,vimNotFunc,vimFuncName,vimFunction,vimFunctionError,vimLineComment,vimNotFunc,vimMap,vimSpecFile,vimOper,vimNumber,vimOperParen,vimComment,vim9Comment,vimString,vimSubst,vimMark,vimRegister,vimAddress,vimFilter,vimCmplxRepeat,vimComment,vim9Comment,vimLet,vimSet,vimAutoCmd,vimRegion,vimSynLine,vimNotation,vimCtrlChar,vimFuncVar,vimContinue,vimOption
if exists("g:vimsyn_folding") && g:vimsyn_folding =~# 'a'
 syn region  vimAugroup	fold matchgroup=vimAugroupKey start="\<aug\%[roup]\>\ze\s\+\K\k*" end="\<aug\%[roup]\>\ze\s\+[eE][nN][dD]\>"	contains=vimAutoCmd,@vimAugroupList
else
 syn region  vimAugroup	matchgroup=vimAugroupKey start="\<aug\%[roup]\>\ze\s\+\K\k*" end="\<aug\%[roup]\>\ze\s\+[eE][nN][dD]\>"		contains=vimAutoCmd,@vimAugroupList
endif
syn match   vimAugroup	"aug\%[roup]!"	contains=vimAugroupKey
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_noaugrouperror")
 syn match   vimAugroupError	"\<aug\%[roup]\>\s\+[eE][nN][dD]\>"
endif
syn keyword vimAugroupKey contained	aug[roup]

" Operators: {{{2
" =========
syn cluster	vimOperGroup	contains=vimEnvvar,vimFunc,vimFuncVar,vimOper,vimOperParen,vimNumber,vimString,vimType,vimRegister,@vimContinue,vim9Comment,vimVar
syn match	vimOper	"||\|&&\|[-+*/%.!]"				skipwhite nextgroup=vimString,vimSpecFile
syn match	vimOper	"\%#=1\(==\|!=\|>=\|<=\|=\~\|!\~\|>\|<\|=\|!\~#\)[?#]\{0,2}"	skipwhite nextgroup=vimString,vimSpecFile
syn match	vimOper	"\(\<is\|\<isnot\)[?#]\{0,2}\>"			skipwhite nextgroup=vimString,vimSpecFile
syn region	vimOperParen 	matchgroup=vimParenSep	start="(" end=")" contains=vimoperStar,@vimOperGroup
syn region	vimOperParen	matchgroup=vimSep		start="#\={" end="}" contains=@vimOperGroup nextgroup=vimVar,vimFuncVar
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_noopererror")
 syn match	vimOperError	")"
endif

" Functions : Tag is provided for those who wish to highlight tagged functions {{{2
" =========
syn cluster	vimFuncList	contains=vimCommand,vimFunctionError,vimFuncKey,Tag,vimFuncSID
syn cluster	vimFuncBodyList	contains=vimAbb,vimAddress,vimAugroupKey,vimAutoCmd,vimCmplxRepeat,vimComment,vim9Comment,vimContinue,vimCtrlChar,vimEcho,vimEchoHL,vimEnvvar,vimExecute,vimIsCommand,vimFBVar,vimFunc,vimFunction,vimFuncVar,vimGlobal,vimHighlight,vimIsCommand,vimLet,vimLetHereDoc,vimLineComment,vimMap,vimMark,vimNorm,vimNotation,vimNotFunc,vimNumber,vimOper,vimOperParen,vimRegion,vimRegister,vimSearch,vimSet,vimSpecFile,vimString,vimSubst,vimSynLine,vimUnmap,vimUserCommand
syn match	vimFunction	"\<\(fu\%[nction]\)!\=\s\+\%(<[sS][iI][dD]>\|[sSgGbBwWtTlL]:\)\=\%(\i\|[#.]\|{.\{-1,}}\)*\ze\s*("	contains=@vimFuncList nextgroup=vimFuncBody
syn match	vimFunction	"\<def!\=\s\+\%(\i\|[#.]\|{.\{-1,}}\)*\ze\s*(" contains=@vimFuncList nextgroup=vimFuncBody
"syn match	vimFunction	"\<def!\=\ze\s*(" contains=@vimFuncList nextgroup=vimFuncBody

if exists("g:vimsyn_folding") && g:vimsyn_folding =~# 'f'
 syn region	vimFuncBody  contained	fold start="\ze\s*("	matchgroup=vimCommand end="\<\(endf\>\|endfu\%[nction]\>\|enddef\>\)"		contains=@vimFuncBodyList
else
 syn region	vimFuncBody  contained	start="\ze\s*("		matchgroup=vimCommand end="\<\(endf\>\|endfu\%[nction]\>\|enddef\>\)"		contains=@vimFuncBodyList
endif
syn match	vimFuncVar   contained	"a:\(\K\k*\|\d\+\)"
syn match	vimFuncSID   contained	"\c<sid>\|\<s:"
syn keyword	vimFuncKey   contained	fu[nction]
syn keyword	vimFuncKey   contained	def
syn match	vimFuncBlank contained	"\s\+"

syn keyword	vimPattern   contained	start	skip	end

" vimTypes : new for vim9
syn match	vimType	":\s*\zs\<\(bool\|number\|float\|string\|blob\|list<\|dict<\|job\|channel\|func\)\>"

" Keymaps: (Vim Project Addition) {{{2
" =======

" TODO: autogenerated vimCommand keyword list does not handle all abbreviations
"     : handle Vim9 script comments when something like #13104 is merged
syn match  vimKeymapStart	"^"	contained skipwhite nextgroup=vimKeymapLhs,vimKeymapLineComment
syn match  vimKeymapLhs	"\S\+"	contained skipwhite nextgroup=vimKeymapRhs contains=vimNotation
syn match  vimKeymapRhs	"\S\+"	contained skipwhite nextgroup=vimKeymapTailComment contains=vimNotation
syn match  vimKeymapTailComment	"\S.*"	contained
syn match  vimKeymapLineComment	+".*+	contained contains=@vimCommentGroup,vimCommentString,vimCommentTitle

syn region vimKeymap matchgroup=vimCommand start="\<loadk\%[eymap]\>" end="\%$" contains=vimKeymapStart

" Special Filenames, Modifiers, Extension Removal: {{{2
" ===============================================
syn match	vimSpecFile	"<c\(word\|WORD\)>"	nextgroup=vimSpecFileMod,vimSubst
syn match	vimSpecFile	"<\([acs]file\|amatch\|abuf\)>"	nextgroup=vimSpecFileMod,vimSubst
syn match	vimSpecFile	"\s%[ \t:]"ms=s+1,me=e-1	nextgroup=vimSpecFileMod,vimSubst
syn match	vimSpecFile	"\s%$"ms=s+1	nextgroup=vimSpecFileMod,vimSubst
syn match	vimSpecFile	"\s%<"ms=s+1,me=e-1	nextgroup=vimSpecFileMod,vimSubst
syn match	vimSpecFile	"#\d\+\|[#%]<\>"	nextgroup=vimSpecFileMod,vimSubst
syn match	vimSpecFileMod	"\(:[phtre]\)\+"	contained

" User-Specified Commands: {{{2
" =======================
syn cluster	vimUserCmdList	contains=vimAddress,vimSyntax,vimHighlight,vimAutoCmd,vimCmplxRepeat,vimComment,vim9Comment,vimCtrlChar,vimEscapeBrace,vimFunc,vimFuncName,vimFunction,vimFunctionError,vimIsCommand,vimMark,vimNotation,vimNumber,vimOper,vimRegion,vimRegister,vimLet,vimSet,vimSetEqual,vimSetString,vimSpecFile,vimString,vimSubst,vimSubstRep,vimSubstRange,vimSynLine
syn keyword	vimUserCommand	contained	com[mand]
syn match	vimUserCmd	"\<com\%[mand]!\=\>.*$"	contains=vimUserAttrb,vimUserAttrbError,vimUserCommand,@vimUserCmdList,vimComFilter
syn match	vimUserAttrbError	contained	"-\a\+\ze\s"
syn match	vimUserAttrb	contained	"-nargs=[01*?+]"	contains=vimUserAttrbKey,vimOper
syn match	vimUserAttrb	contained	"-complete="		contains=vimUserAttrbKey,vimOper nextgroup=vimUserAttrbCmplt,vimUserCmdError
syn match	vimUserAttrb	contained	"-range\(=%\|=\d\+\)\="	contains=vimNumber,vimOper,vimUserAttrbKey
syn match	vimUserAttrb	contained	"-count\(=\d\+\)\="	contains=vimNumber,vimOper,vimUserAttrbKey
syn match	vimUserAttrb	contained	"-bang\>"		contains=vimOper,vimUserAttrbKey
syn match	vimUserAttrb	contained	"-bar\>"		contains=vimOper,vimUserAttrbKey
syn match	vimUserAttrb	contained	"-buffer\>"		contains=vimOper,vimUserAttrbKey
syn match	vimUserAttrb	contained	"-register\>"		contains=vimOper,vimUserAttrbKey
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_nousercmderror")
 syn match	vimUserCmdError	contained	"\S\+\>"
endif
syn case ignore
syn keyword	vimUserAttrbKey   contained	bar	ban[g]	cou[nt]	ra[nge] com[plete]	n[args]	re[gister]
syn keyword	vimUserAttrbCmplt contained	augroup buffer behave color command compiler cscope dir environment event expression file file_in_path filetype function help highlight history locale mapping menu option packadd shellcmd sign syntax syntime tag tag_listfiles user var
syn keyword	vimUserAttrbCmplt contained	custom customlist nextgroup=vimUserAttrbCmpltFunc,vimUserCmdError
syn match	vimUserAttrbCmpltFunc contained	",\%([sS]:\|<[sS][iI][dD]>\)\=\%(\h\w*\%([.#]\h\w*\)\+\|\h\w*\)"hs=s+1 nextgroup=vimUserCmdError

syn case match
syn match	vimUserAttrbCmplt contained	"custom,\u\w*"

" Lower Priority Comments: after some vim commands... {{{2
" =======================
syn match	vimComment	excludenl +\s"[^\-:.%#=*].*$+lc=1	contains=@vimCommentGroup,vimCommentString
syn match	vimComment	+\<endif\s\+".*$+lc=5	contains=@vimCommentGroup,vimCommentString
syn match	vimComment	+\<else\s\+".*$+lc=4	contains=@vimCommentGroup,vimCommentString
syn region	vimCommentString	contained oneline start='\S\s\+"'ms=e	end='"'
" Vim9 comments - TODO: might be highlighted while they don't work
syn match	vim9Comment	excludenl +\s#[^{].*$+lc=1	contains=@vimCommentGroup,vimCommentString
syn match	vim9Comment	+\<endif\s\+#[^{].*$+lc=5	contains=@vimCommentGroup,vimCommentString
syn match	vim9Comment	+\<else\s\+#[^{].*$+lc=4	contains=@vimCommentGroup,vimCommentString
" Vim9 comment inside expression
syn match	vim9Comment	+\s\zs#[^{].*$+ms=s+1	contains=@vimCommentGroup,vimCommentString
syn match	vim9Comment	+^\s*#[^{].*$+	contains=@vimCommentGroup,vimCommentString
syn match	vim9Comment	+^\s*#$+	contains=@vimCommentGroup,vimCommentString

" Environment Variables: {{{2
" =====================
syn match	vimEnvvar	"\$\I\i*"
syn match	vimEnvvar	"\${\I\i*}"

" In-String Specials: {{{2
" Try to catch strings, if nothing else matches (therefore it must precede the others!)
"  vimEscapeBrace handles ["]  []"] (ie. "s don't terminate string inside [])
syn region	vimEscapeBrace	oneline   contained transparent start="[^\\]\(\\\\\)*\[\zs\^\=\]\=" skip="\\\\\|\\\]" end="]"me=e-1
syn match	vimPatSepErr	contained	"\\)"
syn match	vimPatSep	contained	"\\|"
syn region	vimPatSepZone	oneline   contained   matchgroup=vimPatSepZ start="\\%\=\ze(" skip="\\\\" end="\\)\|[^\\]['"]"	contains=@vimStringGroup
syn region	vimPatRegion	contained transparent matchgroup=vimPatSepR start="\\[z%]\=(" end="\\)"	contains=@vimSubstList oneline
syn match	vimNotPatSep	contained	"\\\\"
syn cluster	vimStringGroup	contains=vimEscape,vimEscapeBrace,vimPatSep,vimNotPatSep,vimPatSepErr,vimPatSepZone,@Spell
syn region	vimString	oneline keepend	start=+[^a-zA-Z>!\\@]"+lc=1 skip=+\\\\\|\\"+ matchgroup=vimStringEnd end=+"+	contains=@vimStringGroup
syn region	vimString	oneline keepend	start=+[^a-zA-Z>!\\@]'+lc=1 end=+'+
syn region	vimString	oneline	start=+=!+lc=1	skip=+\\\\\|\\!+ end=+!+	contains=@vimStringGroup
syn region	vimString	oneline	start="=+"lc=1	skip="\\\\\|\\+" end="+"	contains=@vimStringGroup
"syn region	vimString	oneline	start="\s/\s*\A"lc=1 skip="\\\\\|\\+" end="/"	contains=@vimStringGroup  " see tst45.vim
syn match	vimString	contained	+"[^"]*\\$+	skipnl nextgroup=vimStringCont
syn match	vimStringCont	contained	+\(\\\\\|.\)\{-}[^\\]"+
syn match	vimEscape	contained	"\\."

syn region	vimString start=+$'+ end=+'+ skip=+''+ oneline contains=vimStringInterpolationBrace,vimStringInterpolationExpr
syn region	vimString start=+$"+ end=+"+ oneline contains=@vimStringGroup,vimStringInterpolationBrace,vimStringInterpolationExpr
syn region	vimStringInterpolationExpr matchgroup=vimOperParen start=+{+ end=+}+ oneline contains=vimFunc,vimFuncVar,vimOper,vimNotation,vimOperParen,vimString,vimVar
syn match	vimStringInterpolationBrace "{{"
syn match	vimStringInterpolationBrace "}}"

" Substitutions: {{{2
" =============
syn cluster	vimSubstList	contains=vimPatSep,vimPatRegion,vimPatSepErr,vimSubstTwoBS,vimSubstRange,vimNotation
syn cluster	vimSubstRepList	contains=vimSubstSubstr,vimSubstTwoBS,vimNotation
syn cluster	vimSubstList	add=vimCollection
syn match	vimSubst	"\(:\+\s*\|^\s*\||\s*\)\<\%(\<s\%[ubstitute]\>\|\<sm\%[agic]\>\|\<sno\%[magic]\>\)[:#[:alpha:]]\@!" nextgroup=vimSubstPat
"syn match	vimSubst	"\%(^\|[^\\]\)\<s\%[ubstitute]\>[:#[:alpha:]]\@!"	nextgroup=vimSubstPat contained
syn match	vimSubst	"\%(^\|[^\\\"']\)\<s\%[ubstitute]\>[:#[:alpha:]\"']\@!"	nextgroup=vimSubstPat contained
syn match	vimSubst	"/\zs\<s\%[ubstitute]\>\ze/"		nextgroup=vimSubstPat
syn match	vimSubst	"\(:\+\s*\|^\s*\)s\ze#.\{-}#.\{-}#"		nextgroup=vimSubstPat
syn match	vimSubst1       contained	"\<s\%[ubstitute]\>"	nextgroup=vimSubstPat
syn match	vimSubst2       contained	"s\%[ubstitute]\>"	nextgroup=vimSubstPat
syn region	vimSubstPat     contained	matchgroup=vimSubstDelim start="\z([^a-zA-Z( \t[\]&]\)"rs=s+1 skip="\\\\\|\\\z1" end="\z1"re=e-1,me=e-1	 contains=@vimSubstList	nextgroup=vimSubstRep4	oneline
syn region	vimSubstRep4    contained	matchgroup=vimSubstDelim start="\z(.\)" skip="\\\\\|\\\z1" end="\z1" matchgroup=vimNotation end="<[cC][rR]>" contains=@vimSubstRepList	nextgroup=vimSubstFlagErr	oneline
syn region	vimCollection   contained transparent	start="\\\@<!\[" skip="\\\[" end="\]"	contains=vimCollClass
syn match	vimCollClassErr contained	"\[:.\{-\}:\]"
syn match	vimCollClass    contained transparent	"\%#=1\[:\(alnum\|alpha\|blank\|cntrl\|digit\|graph\|lower\|print\|punct\|space\|upper\|xdigit\|retu\%[rn]\|tab\|escape\|backspace\):\]"
syn match	vimSubstSubstr  contained	"\\z\=\d"
syn match	vimSubstTwoBS   contained	"\\\\"
syn match	vimSubstFlagErr contained	"[^< \t\r|]\+" contains=vimSubstFlags
syn match	vimSubstFlags   contained	"[&cegiIlnpr#]\+"

" 'String': {{{2
syn match	vimString	"[^(,]'[^']\{-}\zs'"

" Marks, Registers, Addresses, Filters: {{{2
syn match	vimMark	"'[a-zA-Z0-9]\ze[-+,!]"	nextgroup=vimFilter,vimMarkNumber,vimSubst
syn match	vimMark	"'[<>]\ze[-+,!]"		nextgroup=vimFilter,vimMarkNumber,vimSubst
syn match	vimMark	",\zs'[<>]\ze"		nextgroup=vimFilter,vimMarkNumber,vimSubst
syn match	vimMark	"[!,:]\zs'[a-zA-Z0-9]"	nextgroup=vimFilter,vimMarkNumber,vimSubst
syn match	vimMark	"\<norm\%[al]\s\zs'[a-zA-Z0-9]"	nextgroup=vimFilter,vimMarkNumber,vimSubst
syn match	vimMarkNumber	"[-+]\d\+"		contained contains=vimOper nextgroup=vimSubst2
syn match	vimPlainMark contained	"'[a-zA-Z0-9]"
syn match	vimRange	"[`'][a-zA-Z0-9],[`'][a-zA-Z0-9]"	contains=vimMark	skipwhite nextgroup=vimFilter

syn match	vimRegister	'[^,;[{: \t]\zs"[a-zA-Z0-9.%#:_\-/]\ze[^a-zA-Z_":0-9]'
syn match	vimRegister	'\<norm\s\+\zs"[a-zA-Z0-9]'
syn match	vimRegister	'\<normal\s\+\zs"[a-zA-Z0-9]'
syn match	vimRegister	'@"'
syn match	vimPlainRegister contained	'"[a-zA-Z0-9\-:.%#*+=]'
syn match	vimLetRegister	contained	'@["0-9\-a-zA-Z#=*+_/]'

syn match	vimAddress	",\zs[.$]"	skipwhite nextgroup=vimSubst1
syn match	vimAddress	"%\ze\a"	skipwhite nextgroup=vimString,vimSubst1

syn match	vimFilter 		"^!!\=[^"]\{-}\(|\|\ze\"\|$\)"	contains=vimOper,vimSpecFile
syn match	vimFilter    contained	"!!\=[^"]\{-}\(|\|\ze\"\|$\)"	contains=vimOper,vimSpecFile
syn match	vimComFilter contained	"|!!\=[^"]\{-}\(|\|\ze\"\|$\)"      contains=vimOper,vimSpecFile

" Complex Repeats: (:h complex-repeat) {{{2
" ===============
syn match	vimCmplxRepeat	'[^a-zA-Z_/\\()]q[0-9a-zA-Z"]\>'lc=1
syn match	vimCmplxRepeat	'@[0-9a-z".=@:]\ze\($\|[^a-zA-Z]\>\)'

" Set command and associated set-options (vimOptions) with comment {{{2
syn region	vimSet		matchgroup=vimCommand start="\<\%(setl\%[ocal]\|setg\%[lobal]\|se\%[t]\)\>" skip="\%(\\\\\)*\\.\n\@!" end="$" end="|" matchgroup=vimNotation end="<[cC][rR]>" keepend contains=vimSetEqual,vimOption,vimErrSetting,vimComment,vim9Comment,vimSetString,vimSetMod
syn region	vimSetEqual	contained	start="[=:]\|[-+^]=" skip="\\\\\|\\\s" end="[| \t]"me=e-1 end="$"	contains=vimCtrlChar,vimSetSep,vimNotation,vimEnvvar
syn region	vimSetString	contained	start=+="+hs=s+1	skip=+\\\\\|\\"+  end=+"+		contains=vimCtrlChar
syn match	vimSetSep	contained	"[,:]"
syn match	vimSetMod	contained	"&vim\=\|[!&?<]\|all&"

" Let And Var: {{{2
" ===========
syn keyword	vimLet	let		skipwhite nextgroup=vimVar,vimFuncVar,vimLetHereDoc,vimLetRegister,vimVarList
syn keyword	vimConst	cons[t]		skipwhite nextgroup=vimVar,vimLetHereDoc,vimVarList
syn region	vimVarList	contained	start="\[" end="]" contains=vimVar,vimContinue

syn keyword	vimUnlet	unl[et]		skipwhite nextgroup=vimUnletBang,vimUnletVars
syn match	vimUnletBang	contained	"!"	skipwhite nextgroup=vimUnletVars
syn region	vimUnletVars	contained	start="$\I\|\h" skip="\n\s*\\" end="$" end="|" contains=vimVar,vimEnvvar,vimContinue,vimString,vimNumber

VimFoldh syn region vimLetHereDoc	matchgroup=vimLetHereDocStart start='=<<\s*\%(trim\s\+\%(eval\s\+\)\=\|eval\s\+\%(trim\s\+\)\=\)\=\z(\L\S*\)' matchgroup=vimLetHereDocStop end='^\s*\z1\s*$'
syn keyword	vimLet	var		skipwhite nextgroup=vimVar,vimFuncVar,vimLetHereDoc

" For: {{{2
" ===
syn keyword	vimFor	for	skipwhite nextgroup=vimVar,vimVarList
" Abbreviations: {{{2
" =============
syn keyword vimAbb	ab[breviate] ca[bbrev] inorea[bbrev] cnorea[bbrev] norea[bbrev] ia[bbrev] skipwhite nextgroup=vimMapMod,vimMapLhs

" Autocmd: {{{2
" =======
syn match	vimAutoEventList	contained	"\(!\s\+\)\=\(\a\+,\)*\a\+"	contains=vimAutoEvent nextgroup=vimAutoCmdSpace
syn match	vimAutoCmdSpace	contained	"\s\+"	nextgroup=vimAutoCmdSfxList
syn match	vimAutoCmdSfxList	contained	"\S*"	skipwhite nextgroup=vimAutoCmdMod
syn keyword	vimAutoCmd	au[tocmd] do[autocmd] doautoa[ll]	skipwhite nextgroup=vimAutoEventList
syn match	vimAutoCmdMod	"\(++\)\=\(once\|nested\)"

" Echo And Execute: -- prefer strings! {{{2
" ================
syn region	vimEcho	oneline excludenl matchgroup=vimCommand start="\<ec\%[ho]\>" skip="\(\\\\\)*\\|" end="$\||" contains=vimFunc,vimFuncVar,vimString,vimVar
syn region	vimExecute	oneline excludenl matchgroup=vimCommand start="\<exe\%[cute]\>" skip="\(\\\\\)*\\|" end="$\||\|<[cC][rR]>" contains=vimFuncVar,vimIsCommand,vimOper,vimNotation,vimOperParen,vimString,vimVar
syn match	vimEchoHL	"echohl\="	skipwhite nextgroup=vimGroup,vimHLGroup,vimEchoHLNone
syn case ignore
syn keyword	vimEchoHLNone	none
syn case match

" Maps: {{{2
" ====
syn match	vimMap		"\<map\>!\=\ze\s*[^(]" skipwhite nextgroup=vimMapMod,vimMapLhs
syn keyword	vimMap		cm[ap] cno[remap] im[ap] ino[remap] lm[ap] ln[oremap] nm[ap] nn[oremap] no[remap] om[ap] ono[remap] smap snor[emap] tno[remap] tm[ap] vm[ap] vmapc[lear] vn[oremap] xm[ap] xn[oremap] skipwhite nextgroup=vimMapBang,vimMapMod,vimMapLhs
syn keyword	vimMap		mapc[lear] smapc[lear]
syn keyword	vimUnmap		cu[nmap] iu[nmap] lu[nmap] nun[map] ou[nmap] sunm[ap] tunma[p] unm[ap] unm[ap] vu[nmap] xu[nmap] skipwhite nextgroup=vimMapBang,vimMapMod,vimMapLhs
syn match	vimMapLhs	contained	"\S\+"			contains=vimNotation,vimCtrlChar skipwhite nextgroup=vimMapRhs
syn match	vimMapBang	contained	"!"			skipwhite nextgroup=vimMapMod,vimMapLhs
syn match	vimMapMod	contained	"\%#=1\c<\(buffer\|expr\|\(local\)\=leader\|nowait\|plug\|script\|sid\|unique\|silent\)\+>" contains=vimMapModKey,vimMapModErr skipwhite nextgroup=vimMapMod,vimMapLhs
syn match	vimMapRhs	contained	".*" contains=vimNotation,vimCtrlChar	skipnl nextgroup=vimMapRhsExtend
syn match	vimMapRhsExtend	contained	"^\s*\\.*$"			contains=vimContinue
syn case ignore
syn keyword	vimMapModKey	contained	buffer	expr	leader	localleader	nowait	plug	script	sid	silent	unique
syn case match

" Menus: {{{2
" =====
syn cluster	vimMenuList contains=vimMenuBang,vimMenuPriority,vimMenuName,vimMenuMod
syn keyword	vimCommand	am[enu] an[oremenu] aun[menu] cme[nu] cnoreme[nu] cunme[nu] ime[nu] inoreme[nu] iunme[nu] me[nu] nme[nu] nnoreme[nu] noreme[nu] nunme[nu] ome[nu] onoreme[nu] ounme[nu] unme[nu] vme[nu] vnoreme[nu] vunme[nu] skipwhite nextgroup=@vimMenuList
syn match	vimMenuName	"[^ \t\\<]\+"	contained nextgroup=vimMenuNameMore,vimMenuMap
syn match	vimMenuPriority	"\d\+\(\.\d\+\)*"	contained skipwhite nextgroup=vimMenuName
syn match	vimMenuNameMore	"\c\\\s\|<tab>\|\\\."	contained nextgroup=vimMenuName,vimMenuNameMore contains=vimNotation
syn match	vimMenuMod    contained	"\c<\(script\|silent\)\+>"  skipwhite contains=vimMapModKey,vimMapModErr nextgroup=@vimMenuList
syn match	vimMenuMap	"\s"	contained skipwhite nextgroup=vimMenuRhs
syn match	vimMenuRhs	".*$"	contained contains=vimString,vimComment,vim9Comment,vimIsCommand
syn match	vimMenuBang	"!"	contained skipwhite nextgroup=@vimMenuList

" Angle-Bracket Notation: (tnx to Michael Geddes) {{{2
" ======================
syn case ignore
syn match	vimNotation	"\%#=1\(\\\|<lt>\)\=<\([scamd]-\)\{0,4}x\=\(f\d\{1,2}\|[^ \t:]\|cmd\|scriptcmd\|cr\|lf\|linefeed\|retu\%[rn]\|k\=del\%[ete]\|bs\|backspace\|tab\|esc\|right\|left\|help\|undo\|insert\|ins\|mouse\|k\=home\|k\=end\|kplus\|kminus\|kdivide\|kmultiply\|kenter\|kpoint\|space\|k\=\(page\)\=\(\|down\|up\|k\d\>\)\)>" contains=vimBracket
syn match	vimNotation	"\%#=1\(\\\|<lt>\)\=<\([scam2-4]-\)\{0,4}\(right\|left\|middle\)\(mouse\)\=\(drag\|release\)\=>"	contains=vimBracket
syn match	vimNotation	"\%#=1\(\\\|<lt>\)\=<\(bslash\|plug\|sid\|space\|bar\|nop\|nul\|lt\)>"			contains=vimBracket
syn match	vimNotation	'\(\\\|<lt>\)\=<C-R>[0-9a-z"%#:.\-=]'he=e-1				contains=vimBracket
syn match	vimNotation	'\%#=1\(\\\|<lt>\)\=<\%(q-\)\=\(line[12]\|count\|bang\|reg\|args\|mods\|f-args\|f-mods\|lt\)>'	contains=vimBracket
syn match	vimNotation	"\%#=1\(\\\|<lt>\)\=<\([cas]file\|abuf\|amatch\|cword\|cWORD\|client\)>"		contains=vimBracket
syn match	vimNotation	"\%#=1\(\\\|<lt>\)\=<\%([scamd]-\)\{0,4}char-\%(\d\+\|0\o\+\|0x\x\+\)>"			contains=vimBracket
syn match	vimBracket contained	"[\\<>]"
syn case match

" User Function Highlighting: {{{2
" (following Gautam Iyer's suggestion)
" ==========================
syn match vimFunc		"\%(\%([sSgGbBwWtTlL]:\|<[sS][iI][dD]>\)\=\%(\w\+\.\)*\I[a-zA-Z0-9_.]*\)\ze\s*("		contains=vimFuncEcho,vimFuncName,vimUserFunc,vimExecute
syn match vimUserFunc contained	"\%(\%([sSgGbBwWtTlL]:\|<[sS][iI][dD]>\)\=\%(\w\+\.\)*\I[a-zA-Z0-9_.]*\)\|\<\u[a-zA-Z0-9.]*\>\|\<if\>"	contains=vimNotation
syn keyword vimFuncEcho contained	ec ech echo

" User Command Highlighting: {{{2
syn match vimUsrCmd	'^\s*\zs\u\%(\w*\)\@>\%([(#[]\|\s\+\%([-+*/%]\=\|\.\.\)=\)\@!'

" Errors And Warnings: {{{2
" ====================
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_novimfunctionerror")
 syn match	vimFunctionError	"\s\zs[a-z0-9]\i\{-}\ze\s*("			contained contains=vimFuncKey,vimFuncBlank
 syn match	vimFunctionError	"\s\zs\%(<[sS][iI][dD]>\|[sSgGbBwWtTlL]:\)\d\i\{-}\ze\s*("	contained contains=vimFuncKey,vimFuncBlank
 syn match	vimElseIfErr	"\<else\s\+if\>"
 syn match	vimBufnrWarn	/\<bufnr\s*(\s*["']\.['"]\s*)/
endif

syn match vimNotFunc	"\<if\>\|\<el\%[seif]\>\|\<retu\%[rn]\>\|\<while\>"	skipwhite nextgroup=vimOper,vimOperParen,vimVar,vimFunc,vimNotation

" Norm: {{{2
" ====
syn match	vimNorm		"\<norm\%[al]!\=" skipwhite nextgroup=vimNormCmds
syn match	vimNormCmds contained	".*$"

" Syntax: {{{2
"=======
syn match	vimGroupList	contained	"[^[:space:],]\+\%(\s*,\s*[^[:space:],]\+\)*" contains=vimGroupSpecial
syn region	vimGroupList	contained	start=/^\s*["#]\\ \|^\s*\\\|[^[:space:],]\+\s*,/ skip=/\s*\n\s*\\\|\s*\n\s*["#]\\ \|^\s*\\\|^\s*["#]\\ / end=/[^[:space:],]\s*$\|[^[:space:],]\ze\s\+\w/ contains=@vimContinue,vimGroupSpecial
syn keyword	vimGroupSpecial	contained	ALL	ALLBUT	CONTAINED	TOP

if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_novimsynerror")
 syn match	vimSynError	contained	"\i\+"
 syn match	vimSynError	contained	"\i\+="	nextgroup=vimGroupList
endif
syn match	vimSynContains	contained	"\<contain\%(s\|edin\)="	skipwhite skipnl nextgroup=vimGroupList
syn match	vimSynKeyContainedin	contained	"\<containedin="	skipwhite skipnl nextgroup=vimGroupList
syn match	vimSynNextgroup	contained	"\<nextgroup="		skipwhite skipnl nextgroup=vimGroupList
if has("conceal")
 " no whitespace allowed after '='
 syn match	vimSynCchar	contained	"\<cchar="	nextgroup=vimSynCcharValue
 syn match	vimSynCcharValue	contained	"\S"
endif

syn match	vimSyntax	"\<sy\%[ntax]\>"	contains=vimCommand skipwhite nextgroup=vimSynType,vimComment,vim9Comment
syn match	vimAuSyntax	contained	"\s+sy\%[ntax]"	contains=vimCommand skipwhite nextgroup=vimSynType,vimComment,vim9Comment
syn cluster vimFuncBodyList add=vimSyntax

" Syntax: case {{{2
syn keyword	vimSynType	contained	case	skipwhite nextgroup=vimSynCase,vimSynCaseError
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_novimsyncaseerror")
 syn match	vimSynCaseError	contained	"\i\+"
endif
syn keyword	vimSynCase	contained	ignore	match

" Syntax: clear {{{2
syn keyword	vimSynType	contained	clear	skipwhite nextgroup=vimGroupList

" Syntax: cluster {{{2
syn keyword	vimSynType	contained	cluster	skipwhite nextgroup=vimClusterName
syn region	vimClusterName	contained keepend	matchgroup=vimGroupName start="\h\w*\>" skip=+\\\\\|\\\|\n\s*\\\|\n\s*"\\ + matchgroup=vimCmdSep end="$\||" contains=@vimContinue,vimGroupAdd,vimGroupRem,vimSynContains,vimSynError
syn match	vimGroupAdd	contained keepend	"\<add="	skipwhite skipnl nextgroup=vimGroupList
syn match	vimGroupRem	contained keepend	"\<remove="	skipwhite skipnl nextgroup=vimGroupList
syn cluster vimFuncBodyList add=vimSynType,vimGroupAdd,vimGroupRem

" Syntax: foldlevel {{{2
syn keyword	vimSynType	contained	foldlevel	skipwhite nextgroup=vimSynFoldMethod,vimSynFoldMethodError
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_novimsynfoldmethoderror")
 syn match	vimSynFoldMethodError	contained	"\i\+"
endif
syn keyword	vimSynFoldMethod	contained	start	minimum

" Syntax: iskeyword {{{2
syn keyword	vimSynType	contained	iskeyword	skipwhite nextgroup=vimIskList
syn match	vimIskList	contained	'\S\+'	contains=vimIskSep
syn match	vimIskSep	contained	','

" Syntax: include {{{2
syn keyword	vimSynType	contained	include	skipwhite nextgroup=vimGroupList
syn cluster vimFuncBodyList add=vimSynType

" Syntax: keyword {{{2
syn cluster	vimSynKeyGroup	contains=@vimContinue,vimSynCchar,vimSynNextgroup,vimSynKeyOpt,vimSynKeyContainedin
syn keyword	vimSynType	contained	keyword	skipwhite nextgroup=vimSynKeyRegion
syn region	vimSynKeyRegion	contained         keepend	matchgroup=vimGroupName start="\h\w*\>" skip=+\\\\\|\\|\|\n\s*\\\|\n\s*"\\ + matchgroup=vimCmdSep end="|\|$" contains=@vimSynKeyGroup
syn match	vimSynKeyOpt	contained	"\%#=1\<\(conceal\|contained\|transparent\|skipempty\|skipwhite\|skipnl\)\>"
syn cluster vimFuncBodyList add=vimSynType

" Syntax: match {{{2
syn cluster	vimSynMtchGroup	contains=@vimContinue,vimSynCchar,vimSynContains,vimSynError,vimSynMtchOpt,vimSynNextgroup,vimSynRegPat,vimNotation,vimMtchComment
syn keyword	vimSynType	contained	match	skipwhite nextgroup=vimSynMatchRegion
syn region	vimSynMatchRegion	contained keepend	matchgroup=vimGroupName start="\h\w*\>" skip=+\\\\\|\\|\|\n\s*\\\|\n\s*"\\ + matchgroup=vimCmdSep end="|\|$" contains=@vimSynMtchGroup
syn match	vimSynMtchOpt	contained	"\%#=1\<\(conceal\|transparent\|contained\|excludenl\|keepend\|skipempty\|skipwhite\|display\|extend\|skipnl\|fold\)\>"
syn cluster vimFuncBodyList add=vimSynMtchGroup

" Syntax: off and on {{{2
syn keyword	vimSynType	contained	enable	list	manual	off	on	reset

" Syntax: region {{{2
syn cluster	vimSynRegPatGroup	contains=@vimContinue,vimPatSep,vimNotPatSep,vimSynPatRange,vimSynNotPatRange,vimSubstSubstr,vimPatRegion,vimPatSepErr,vimNotation
syn cluster	vimSynRegGroup	contains=@vimContinue,vimSynCchar,vimSynContains,vimSynNextgroup,vimSynRegOpt,vimSynReg,vimSynMtchGrp
syn keyword	vimSynType	contained	region	skipwhite nextgroup=vimSynRegion
syn region	vimSynRegion	contained keepend	matchgroup=vimGroupName start="\h\w*" skip=+\\\\\|\\\|\n\s*\\\|\n\s*"\\ + end="|\|$" contains=@vimSynRegGroup
syn match	vimSynRegOpt	contained	"\%#=1\<\(conceal\(ends\)\=\|transparent\|contained\|excludenl\|skipempty\|skipwhite\|display\|keepend\|oneline\|extend\|skipnl\|fold\)\>"
syn match	vimSynReg	contained	"\<\%(start\|skip\|end\)="	nextgroup=vimSynRegPat
syn match	vimSynMtchGrp	contained	"matchgroup="	nextgroup=vimGroup,vimHLGroup
syn region	vimSynRegPat	contained extend	start="\z([-`~!@#$%^&*_=+;:'",./?]\)"  skip=/\\\\\|\\\z1\|\n\s*\\\|\n\s*"\\ /  end="\z1"  contains=@vimSynRegPatGroup skipwhite nextgroup=vimSynPatMod,vimSynReg
syn match	vimSynPatMod	contained	"\%#=1\(hs\|ms\|me\|hs\|he\|rs\|re\)=[se]\([-+]\d\+\)\="
syn match	vimSynPatMod	contained	"\%#=1\(hs\|ms\|me\|hs\|he\|rs\|re\)=[se]\([-+]\d\+\)\=," nextgroup=vimSynPatMod
syn match	vimSynPatMod	contained	"lc=\d\+"
syn match	vimSynPatMod	contained	"lc=\d\+," nextgroup=vimSynPatMod
syn region	vimSynPatRange	contained	start="\["	skip="\\\\\|\\]"   end="]"
syn match	vimSynNotPatRange	contained	"\\\\\|\\\["
syn match	vimMtchComment	contained	'"[^"]\+$'
syn cluster vimFuncBodyList add=vimSynType

" Syntax: sync {{{2
" ============
syn keyword vimSynType	contained	sync	skipwhite	nextgroup=vimSyncC,vimSyncLines,vimSyncMatch,vimSyncError,vimSyncLinebreak,vimSyncLinecont,vimSyncRegion
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_novimsyncerror")
 syn match	vimSyncError	contained	"\i\+"
endif
syn keyword	vimSyncC	contained	ccomment	clear	fromstart
syn keyword	vimSyncMatch	contained	match	skipwhite	nextgroup=vimSyncGroupName
syn keyword	vimSyncRegion	contained	region	skipwhite	nextgroup=vimSynReg
syn match	vimSyncLinebreak	contained	"\<linebreaks="	skipwhite	nextgroup=vimNumber
syn keyword	vimSyncLinecont	contained	linecont	skipwhite	nextgroup=vimSynRegPat
syn match	vimSyncLines	contained	"\(min\|max\)\=lines="	nextgroup=vimNumber
syn match	vimSyncGroupName	contained	"\h\w*"	skipwhite	nextgroup=vimSyncKey
syn match	vimSyncKey	contained	"\<groupthere\|grouphere\>"	skipwhite nextgroup=vimSyncGroup
syn match	vimSyncGroup	contained	"\h\w*"	skipwhite	nextgroup=vimSynRegPat,vimSyncNone
syn keyword	vimSyncNone	contained	NONE

" Additional IsCommand: here by reasons of precedence {{{2
" ====================
syn match	vimIsCommand	"<Bar>\s*\a\+"	transparent contains=vimCommand,vimNotation

" Highlighting: {{{2
" ============
syn cluster	vimHighlightCluster		contains=vimHiLink,vimHiClear,vimHiKeyList,vimComment,vim9Comment
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_novimhictermerror")
 syn match	vimHiCtermError	contained	"\D\i*"
endif
syn match	vimHighlight	"\<hi\%[ghlight]\>"	skipwhite nextgroup=vimHiBang,@vimHighlightCluster
syn match	vimHiBang	contained	"!"	skipwhite nextgroup=@vimHighlightCluster

syn match	vimHiGroup	contained	"\i\+"
syn case ignore
syn keyword	vimHiAttrib	contained	none bold inverse italic nocombine reverse standout strikethrough underline undercurl
syn keyword	vimFgBgAttrib	contained	none bg background fg foreground
syn case match
syn match	vimHiAttribList	contained	"\i\+"	contains=vimHiAttrib
syn match	vimHiAttribList	contained	"\i\+,"he=e-1	contains=vimHiAttrib nextgroup=vimHiAttribList
syn case ignore
syn keyword	vimHiCtermColor	contained	black blue brown cyan darkblue darkcyan darkgray darkgreen darkgrey darkmagenta darkred darkyellow gray green grey grey40 grey50 grey90 lightblue lightcyan lightgray lightgreen lightgrey lightmagenta lightred lightyellow magenta red seagreen white yellow
syn match	vimHiCtermColor	contained	"\<color\d\{1,3}\>"

syn case match
syn match	vimHiFontname	contained	"[a-zA-Z\-*]\+"
syn match	vimHiGuiFontname	contained	"'[a-zA-Z\-* ]\+'"
syn match	vimHiGuiRgb	contained	"#\x\{6}"

" Highlighting: hi group key=arg ... {{{2
syn cluster	vimHiCluster contains=vimGroup,vimHiGroup,vimHiTerm,vimHiCTerm,vimHiStartStop,vimHiCtermFgBg,vimHiCtermul,vimHiCtermfont,vimHiGui,vimHiGuiFont,vimHiGuiFgBg,vimHiKeyError,vimNotation
syn region	vimHiKeyList	contained oneline start="\i\+" skip="\\\\\|\\|" end="$\||"	contains=@vimHiCluster
if !exists("g:vimsyn_noerror") && !exists("g:vimsyn_vimhikeyerror")
 syn match	vimHiKeyError	contained	"\i\+="he=e-1
endif
syn match	vimHiTerm	contained	"\cterm="he=e-1		nextgroup=vimHiAttribList
syn match	vimHiStartStop	contained	"\c\(start\|stop\)="he=e-1	nextgroup=vimHiTermcap,vimOption
syn match	vimHiCTerm	contained	"\ccterm="he=e-1		nextgroup=vimHiAttribList
syn match	vimHiCtermFgBg	contained	"\ccterm[fb]g="he=e-1	nextgroup=vimHiNmbr,vimHiCtermColor,vimFgBgAttrib,vimHiCtermError
syn match	vimHiCtermul	contained	"\cctermul="he=e-1	nextgroup=vimHiNmbr,vimHiCtermColor,vimFgBgAttrib,vimHiCtermError
syn match	vimHiCtermfont	contained	"\cctermfont="he=e-1	nextgroup=vimHiNmbr,vimHiCtermColor,vimFgBgAttrib,vimHiCtermError
syn match	vimHiGui	contained	"\cgui="he=e-1		nextgroup=vimHiAttribList
syn match	vimHiGuiFont	contained	"\cfont="he=e-1		nextgroup=vimHiFontname
syn match	vimHiGuiFgBg	contained	"\cgui\%([fb]g\|sp\)="he=e-1	nextgroup=vimHiGroup,vimHiGuiFontname,vimHiGuiRgb,vimFgBgAttrib
syn match	vimHiTermcap	contained	"\S\+"		contains=vimNotation
syn match	vimHiNmbr	contained	'\d\+'

" Highlight: clear {{{2
syn keyword	vimHiClear	contained	clear	nextgroup=vimHiGroup

" Highlight: link {{{2
" see tst24 (hi def vs hi) (Jul 06, 2018)
"syn region	vimHiLink	contained oneline matchgroup=vimCommand start="\(\<hi\%[ghlight]\s\+\)\@<=\(\(def\%[ault]\s\+\)\=link\>\|\<def\>\)" end="$"	contains=vimHiGroup,vimGroup,vimHLGroup,vimNotation
syn region	vimHiLink	contained oneline matchgroup=vimCommand start="\(\<hi\%[ghlight]\s\+\)\@<=\(\(def\%[ault]\s\+\)\=link\>\|\<def\>\)" end="$"	contains=@vimHiCluster
syn cluster vimFuncBodyList add=vimHiLink

" Control Characters: {{{2
" ==================
syn match	vimCtrlChar	"[--]"

" Beginners - Patterns that involve ^ {{{2
" =========
syn match	vimLineComment	+^[ \t:]*".*$+	contains=@vimCommentGroup,vimCommentString,vimCommentTitle,vimComment
syn match	vimLineComment	+^[ \t:]*"\("[^"]*"\|[^"]\)*$+	contains=@vimCommentGroup,vimCommentString,vimCommentTitle
syn match	vim9LineComment	+^[ \t:]\+#.*$+	contains=@vimCommentGroup,vimCommentString,vimCommentTitle
syn match	vimCommentTitle	'"\s*\%([sS]:\|\h\w*#\)\=\u\w*\(\s\+\u\w*\)*:'hs=s+1	contained contains=vimCommentTitleLeader,vimTodo,@vimCommentGroup
" Note: Look-behind to work around nextgroup skipnl consuming leading whitespace and preventing a match
syn match	vimContinue		"^\s*\zs\\"
syn match         vimContinueComment	'^\s*\zs["#]\\ .*' contained
syn cluster	vimContinue contains=vimContinue,vimContinueComment
syn region	vimString	start="^\s*\\\z(['"]\)" skip='\\\\\|\\\z1' end="\z1" oneline keepend contains=@vimStringGroup,vimContinue
syn match	vimCommentTitleLeader	'"\s\+'ms=s+1	contained

" Searches And Globals: {{{2
" ====================
syn match	vimSearch	'^\s*[/?].*'		contains=vimSearchDelim
syn match	vimSearchDelim	'^\s*\zs[/?]\|[/?]$'	contained
syn region	vimGlobal	matchgroup=Statement start='\<g\%[lobal]!\=/'  skip='\\.' end='/'	skipwhite nextgroup=vimSubst
syn region	vimGlobal	matchgroup=Statement start='\<v\%[global]!\=/' skip='\\.' end='/'	skipwhite nextgroup=vimSubst

" Embedded Scripts:  {{{2
" ================
"   perl,ruby     : Benoit Cerrina
"   python,tcl    : Johannes Zellner
"   mzscheme, lua : Charles Campbell

" Allows users to specify the type of embedded script highlighting
" they want:  (perl/python/ruby/tcl support)
"   g:vimsyn_embed == 0   : don't embed any scripts
"   g:vimsyn_embed =~# 'l' : embed lua      (but only if vim supports it)
"   g:vimsyn_embed =~# 'm' : embed mzscheme (but only if vim supports it)
"   g:vimsyn_embed =~# 'p' : embed perl     (but only if vim supports it)
"   g:vimsyn_embed =~# 'P' : embed python   (but only if vim supports it)
"   g:vimsyn_embed =~# 'r' : embed ruby     (but only if vim supports it)
"   g:vimsyn_embed =~# 't' : embed tcl      (but only if vim supports it)
if !exists("g:vimsyn_embed")
 let g:vimsyn_embed= "lmpPr"
endif

" [-- lua --] {{{3
let s:luapath= fnameescape(expand("<sfile>:p:h")."/lua.vim")
if !filereadable(s:luapath)
 for s:luapath in split(globpath(&rtp,"syntax/lua.vim"),"\n")
  if filereadable(fnameescape(s:luapath))
   let s:luapath= fnameescape(s:luapath)
   break
  endif
 endfor
endif
if (g:vimsyn_embed =~# 'l' && has("lua")) && filereadable(s:luapath)
 unlet! b:current_syntax
 syn cluster vimFuncBodyList	add=vimLuaRegion
 exe "syn include @vimLuaScript ".s:luapath
 VimFoldl syn region vimLuaRegion matchgroup=vimScriptDelim start=+lua\s*<<\s*\z(.*\)$+ end=+^\z1$+	contains=@vimLuaScript
 VimFoldl syn region vimLuaRegion matchgroup=vimScriptDelim start=+lua\s*<<\s*$+ end=+\.$+	contains=@vimLuaScript
 syn cluster vimFuncBodyList	add=vimLuaRegion
else
 syn region vimEmbedError start=+lua\s*<<\s*\z(.*\)$+ end=+^\z1$+
 syn region vimEmbedError start=+lua\s*<<\s*$+ end=+\.$+
endif
unlet s:luapath

" [-- perl --] {{{3
let s:perlpath= fnameescape(expand("<sfile>:p:h")."/perl.vim")
if !filereadable(s:perlpath)
 for s:perlpath in split(globpath(&rtp,"syntax/perl.vim"),"\n")
  if filereadable(fnameescape(s:perlpath))
   let s:perlpath= fnameescape(s:perlpath)
   break
  endif
 endfor
endif
if (g:vimsyn_embed =~# 'p' && has("perl")) && filereadable(s:perlpath)
 unlet! b:current_syntax
 syn cluster vimFuncBodyList	add=vimPerlRegion
 exe "syn include @vimPerlScript ".s:perlpath
 VimFoldp syn region vimPerlRegion  matchgroup=vimScriptDelim start=+pe\%[rl]\s*<<\s*\z(\S*\)\ze\(\s*["#].*\)\=$+ end=+^\z1\ze\(\s*[#"].*\)\=$+	contains=@vimPerlScript
 VimFoldp syn region vimPerlRegion	matchgroup=vimScriptDelim start=+pe\%[rl]\s*<<\s*$+ end=+\.$+			contains=@vimPerlScript
 syn cluster vimFuncBodyList	add=vimPerlRegion
else
 syn region vimEmbedError start=+pe\%[rl]\s*<<\s*\z(.*\)$+ end=+^\z1$+
 syn region vimEmbedError start=+pe\%[rl]\s*<<\s*$+ end=+\.$+
endif
unlet s:perlpath

" [-- ruby --] {{{3
let s:rubypath= fnameescape(expand("<sfile>:p:h")."/ruby.vim")
if !filereadable(s:rubypath)
 for s:rubypath in split(globpath(&rtp,"syntax/ruby.vim"),"\n")
  if filereadable(fnameescape(s:rubypath))
   let s:rubypath= fnameescape(s:rubypath)
   break
  endif
 endfor
endif
if (g:vimsyn_embed =~# 'r' && has("ruby")) && filereadable(s:rubypath)
 syn cluster vimFuncBodyList	add=vimRubyRegion
 unlet! b:current_syntax
 exe "syn include @vimRubyScript ".s:rubypath
 VimFoldr syn region vimRubyRegion	matchgroup=vimScriptDelim start=+rub[y]\s*<<\s*\z(\S*\)\ze\(\s*#.*\)\=$+ end=+^\z1\ze\(\s*".*\)\=$+	contains=@vimRubyScript
 syn region vimRubyRegion	matchgroup=vimScriptDelim start=+rub[y]\s*<<\s*$+ end=+\.$+			contains=@vimRubyScript
 syn cluster vimFuncBodyList	add=vimRubyRegion
else
 syn region vimEmbedError start=+rub[y]\s*<<\s*\z(.*\)$+ end=+^\z1$+
 syn region vimEmbedError start=+rub[y]\s*<<\s*$+ end=+\.$+
endif
unlet s:rubypath

" [-- python --] {{{3
let s:pythonpath= fnameescape(expand("<sfile>:p:h")."/python.vim")
if !filereadable(s:pythonpath)
 for s:pythonpath in split(globpath(&rtp,"syntax/python.vim"),"\n")
  if filereadable(fnameescape(s:pythonpath))
   let s:pythonpath= fnameescape(s:pythonpath)
   break
  endif
 endfor
endif
if g:vimsyn_embed =~# 'P' && has("pythonx") && filereadable(s:pythonpath)
 unlet! b:current_syntax
 syn cluster vimFuncBodyList	add=vimPythonRegion
 exe "syn include @vimPythonScript ".s:pythonpath
 VimFoldP syn region vimPythonRegion matchgroup=vimScriptDelim start=+py\%[thon][3x]\=\s*<<\s*\%(trim\s*\)\=\z(\S*\)\ze\(\s*#.*\)\=$+ end=+^\z1\ze\(\s*".*\)\=$+	contains=@vimPythonScript
 VimFoldP syn region vimPythonRegion matchgroup=vimScriptDelim start=+py\%[thon][3x]\=\s*<<\s*\%(trim\s*\)\=$+ end=+\.$+				contains=@vimPythonScript
 VimFoldP syn region vimPythonRegion matchgroup=vimScriptDelim start=+Py\%[thon]2or3\s*<<\s*\%(trim\s*\)\=\z(\S*\)\ze\(\s*#.*\)\=$+ end=+^\z1\ze\(\s*".*\)\=$+	contains=@vimPythonScript
 VimFoldP syn region vimPythonRegion matchgroup=vimScriptDelim start=+Py\%[thon]2or3\=\s*<<\s*\%(trim\s*\)\=$+ end=+\.$+				contains=@vimPythonScript
 syn cluster vimFuncBodyList	add=vimPythonRegion
else
 syn region vimEmbedError start=+py\%[thon]3\=\s*<<\s*\z(.*\)$+ end=+^\z1$+
 syn region vimEmbedError start=+py\%[thon]3\=\s*<<\s*$+ end=+\.$+
endif
unlet s:pythonpath

" [-- tcl --] {{{3
if has("win32") || has("win95") || has("win64") || has("win16")
 " apparently has("tcl") has been hanging vim on some windows systems with cygwin
 let s:trytcl= (&shell !~ '\<\%(bash\>\|4[nN][tT]\|\<zsh\)\>\%(\.exe\)\=$')
else
 let s:trytcl= 1
endif
if s:trytcl
 let s:tclpath= fnameescape(expand("<sfile>:p:h")."/tcl.vim")
 if !filereadable(s:tclpath)
  for s:tclpath in split(globpath(&rtp,"syntax/tcl.vim"),"\n")
   if filereadable(fnameescape(s:tclpath))
    let s:tclpath= fnameescape(s:tclpath)
    break
   endif
  endfor
 endif
 if (g:vimsyn_embed =~# 't' && has("tcl")) && filereadable(s:tclpath)
  unlet! b:current_syntax
  syn cluster vimFuncBodyList	add=vimTclRegion
  exe "syn include @vimTclScript ".s:tclpath
  VimFoldt syn region vimTclRegion matchgroup=vimScriptDelim start=+tc[l]\=\s*<<\s*\z(.*\)$+ end=+^\z1$+	contains=@vimTclScript
  VimFoldt syn region vimTclRegion matchgroup=vimScriptDelim start=+tc[l]\=\s*<<\s*$+ end=+\.$+	contains=@vimTclScript
  syn cluster vimFuncBodyList	add=vimTclScript
 else
  syn region vimEmbedError start=+tc[l]\=\s*<<\s*\z(.*\)$+ end=+^\z1$+
  syn region vimEmbedError start=+tc[l]\=\s*<<\s*$+ end=+\.$+
 endif
 unlet s:tclpath
else
 syn region vimEmbedError start=+tc[l]\=\s*<<\s*\z(.*\)$+ end=+^\z1$+
 syn region vimEmbedError start=+tc[l]\=\s*<<\s*$+ end=+\.$+
endif
unlet s:trytcl

" [-- mzscheme --] {{{3
let s:mzschemepath= fnameescape(expand("<sfile>:p:h")."/scheme.vim")
if !filereadable(s:mzschemepath)
 for s:mzschemepath in split(globpath(&rtp,"syntax/mzscheme.vim"),"\n")
  if filereadable(fnameescape(s:mzschemepath))
   let s:mzschemepath= fnameescape(s:mzschemepath)
   break
  endif
 endfor
endif
if (g:vimsyn_embed =~# 'm' && has("mzscheme")) && filereadable(s:mzschemepath)
 unlet! b:current_syntax
 let s:iskKeep= &isk
 syn cluster vimFuncBodyList	add=vimMzSchemeRegion
 exe "syn include @vimMzSchemeScript ".s:mzschemepath
 let &isk= s:iskKeep
 unlet s:iskKeep
 VimFoldm syn region vimMzSchemeRegion matchgroup=vimScriptDelim start=+mz\%[scheme]\s*<<\s*\z(.*\)$+ end=+^\z1$+	contains=@vimMzSchemeScript
 VimFoldm syn region vimMzSchemeRegion matchgroup=vimScriptDelim start=+mz\%[scheme]\s*<<\s*$+ end=+\.$+		contains=@vimMzSchemeScript
 syn cluster vimFuncBodyList	add=vimMzSchemeRegion
else
 syn region vimEmbedError start=+mz\%[scheme]\s*<<\s*\z(.*\)$+ end=+^\z1$+
 syn region vimEmbedError start=+mz\%[scheme]\s*<<\s*$+ end=+\.$+
endif
unlet s:mzschemepath

" Synchronize (speed) {{{2
"============
if exists("g:vimsyn_minlines")
 exe "syn sync minlines=".g:vimsyn_minlines
endif
exe "syn sync maxlines=".s:vimsyn_maxlines
syn sync linecont	"^\s\+\\"
syn sync match vimAugroupSyncA	groupthere NONE	"\<aug\%[roup]\>\s\+[eE][nN][dD]"

" ====================
" Highlighting Settings {{{2
" ====================

if !exists("skip_vim_syntax_inits")
 if !exists("g:vimsyn_noerror")
  hi def link vimBehaveError	vimError
  hi def link vimCollClassErr	vimError
  hi def link vimErrSetting	vimError
  hi def link vimEmbedError	vimError
  hi def link vimFTError	vimError
  hi def link vimFunctionError	vimError
  hi def link vimFunc         	vimError
  hi def link vimHiAttribList	vimError
  hi def link vimHiCtermError	vimError
  hi def link vimHiKeyError	vimError
  hi def link vimKeyCodeError	vimError
  hi def link vimMapModErr	vimError
  hi def link vimSubstFlagErr	vimError
  hi def link vimSynCaseError	vimError
  hi def link vimSynFoldMethodError	vimError
  hi def link vimBufnrWarn	vimWarn
 endif

 hi def link vimAbb	vimCommand
 hi def link vimAddress	vimMark
 hi def link vimAugroupError	vimError
 hi def link vimAugroupKey	vimCommand
 hi def link vimAuHighlight	vimHighlight
 hi def link vimAutoCmdOpt	vimOption
 hi def link vimAutoCmd	vimCommand
 hi def link vimAutoEvent	Type
 hi def link vimAutoCmdMod	Special
 hi def link vimAutoSet	vimCommand
 hi def link vimBehaveModel	vimBehave
 hi def link vimBehave	vimCommand
 hi def link vimBracket	Delimiter
 hi def link vimCmplxRepeat	SpecialChar
 hi def link vimCommand	Statement
 hi def link vimComment	Comment
 hi def link vim9Comment	Comment
 hi def link vimCommentString	vimString
 hi def link vimCommentTitle	PreProc
 hi def link vimCondHL	vimCommand
 hi def link vimConst	vimCommand
 hi def link vimContinue	Special
 hi def link vimContinueComment	vimComment
 hi def link vimCtrlChar	SpecialChar
 hi def link vimEchoHLNone	vimGroup
 hi def link vimEchoHL	vimCommand
 hi def link vimElseIfErr	Error
 hi def link vimElseif	vimCondHL
 hi def link vimEnvvar	PreProc
 hi def link vimError	Error
 hi def link vimEscape	Special
 hi def link vimFBVar	vimVar
 hi def link vimFgBgAttrib	vimHiAttrib
 hi def link vimFuncEcho	vimCommand
 hi def link vimHiCtermul	vimHiTerm
 hi def link vimHiCtermfont	vimHiTerm
 hi def link vimFold	Folded
 hi def link vimFor	vimCommand
 hi def link vimFTCmd	vimCommand
 hi def link vimFTOption	vimSynType
 hi def link vimFuncKey	vimCommand
 hi def link vimFuncName	Function
 hi def link vimFuncSID	Special
 hi def link vimFuncVar	Identifier
 hi def link vimGroupAdd	vimSynOption
 hi def link vimGroupName	vimGroup
 hi def link vimGroupRem	vimSynOption
 hi def link vimGroupSpecial	Special
 hi def link vimGroup	Type
 hi def link vimHiAttrib	PreProc
 hi def link vimHiClear	vimHighlight
 hi def link vimHiCtermFgBg	vimHiTerm
 hi def link vimHiCTerm	vimHiTerm
 hi def link vimHighlight	vimCommand
 hi def link vimHiGroup	vimGroupName
 hi def link vimHiGuiFgBg	vimHiTerm
 hi def link vimHiGuiFont	vimHiTerm
 hi def link vimHiGuiRgb	vimNumber
 hi def link vimHiGui	vimHiTerm
 hi def link vimHiNmbr	Number
 hi def link vimHiStartStop	vimHiTerm
 hi def link vimHiTerm	Type
 hi def link vimHLGroup	vimGroup
 hi def link vimHLMod	PreProc
 hi def link vimInsert	vimString
 hi def link vimIskSep	Delimiter
 hi def link vimKeyCode	vimSpecFile
 hi def link vimKeymapLineComment	vimComment
 hi def link vimKeymapTailComment	vimComment
 hi def link vimKeyword	Statement
 hi def link vimLet	vimCommand
 hi def link vimLetHereDoc	vimString
 hi def link vimLetHereDocStart	Special
 hi def link vimLetHereDocStop	Special
 hi def link vimLetRegister	Special
 hi def link vimLineComment	vimComment
 hi def link vim9LineComment	vimComment
 hi def link vimMapBang	vimCommand
 hi def link vimMapModKey	vimFuncSID
 hi def link vimMapMod	vimBracket
 hi def link vimMap	vimCommand
 hi def link vimMark	Number
 hi def link vimMarkNumber	vimNumber
 hi def link vimMenuMod	vimMapMod
 hi def link vimMenuNameMore	vimMenuName
 hi def link vimMenuName	PreProc
 hi def link vimMtchComment	vimComment
 hi def link vimNorm	vimCommand
 hi def link vimNotation	Special
 hi def link vimNotFunc	vimCommand
 hi def link vimNotPatSep	vimString
 hi def link vimNumber	Number
 hi def link vimOperError	Error
 hi def link vimOper	Operator
 hi def link vimOperStar	vimOper
 hi def link vimOption	PreProc
 hi def link vimParenSep	Delimiter
 hi def link vimPatSepErr	vimError
 hi def link vimPatSepR	vimPatSep
 hi def link vimPatSep	SpecialChar
 hi def link vimPatSepZone	vimString
 hi def link vimPatSepZ	vimPatSep
 hi def link vimPattern	Type
 hi def link vimPlainMark	vimMark
 hi def link vimPlainRegister	vimRegister
 hi def link vimRegister	SpecialChar
 hi def link vimScriptDelim	Comment
 hi def link vimSearchDelim	Statement
 hi def link vimSearch	vimString
 hi def link vimSep	Delimiter
 hi def link vimSetMod	vimOption
 hi def link vimSetSep	Statement
 hi def link vimSetString	vimString
 hi def link vimSpecFile	Identifier
 hi def link vimSpecFileMod	vimSpecFile
 hi def link vimSpecial	Type
 hi def link vimStatement	Statement
 hi def link vimStringCont	vimString
 hi def link vimString	String
 hi def link vimStringEnd	vimString
 hi def link vimStringInterpolationBrace	vimEscape
 hi def link vimSubst1	vimSubst
 hi def link vimSubstDelim	Delimiter
 hi def link vimSubstFlags	Special
 hi def link vimSubstSubstr	SpecialChar
 hi def link vimSubstTwoBS	vimString
 hi def link vimSubst	vimCommand
 hi def link vimSynCaseError	Error
 hi def link vimSynCase	Type
 hi def link vimSyncC	Type
 hi def link vimSyncError	Error
 hi def link vimSyncGroupName	vimGroupName
 hi def link vimSyncGroup	vimGroupName
 hi def link vimSyncKey	Type
 hi def link vimSyncNone	Type
 hi def link vimSynContains	vimSynOption
 hi def link vimSynError	Error
 hi def link vimSynFoldMethodError	Error
 hi def link vimSynFoldMethod	Type
 hi def link vimSynKeyContainedin	vimSynContains
 hi def link vimSynKeyOpt	vimSynOption
 hi def link vimSynCchar	vimSynOption
 hi def link vimSynCcharValue	Character
 hi def link vimSynMtchGrp	vimSynOption
 hi def link vimSynMtchOpt	vimSynOption
 hi def link vimSynNextgroup	vimSynOption
 hi def link vimSynNotPatRange	vimSynRegPat
 hi def link vimSynOption	Special
 hi def link vimSynPatRange	vimString
 hi def link vimSynRegOpt	vimSynOption
 hi def link vimSynRegPat	vimString
 hi def link vimSynReg	Type
 hi def link vimSyntax	vimCommand
 hi def link vimSynType	vimSpecial
 hi def link vimTodo	Todo
 hi def link vimType	Type
 hi def link vimUnlet	vimCommand
 hi def link vimUnletBang	vimCommand
 hi def link vimUnmap	vimMap
 hi def link vimUserAttrbCmpltFunc	Special
 hi def link vimUserAttrbCmplt	vimSpecial
 hi def link vimUserAttrbKey	vimOption
 hi def link vimUserAttrb	vimSpecial
 hi def link vimUserAttrbError	Error
 hi def link vimUserCmdError	Error
 hi def link vimUserCommand	vimCommand
 hi def link vimUserFunc	Normal
 hi def link vimVar	Identifier
 hi def link vimWarn	WarningMsg
endif

" Current Syntax Variable: {{{2
let b:current_syntax = "vim"

" ---------------------------------------------------------------------
" Cleanup: {{{1
delc VimFolda
delc VimFoldf
delc VimFoldl
delc VimFoldm
delc VimFoldp
delc VimFoldP
delc VimFoldr
delc VimFoldt
let &cpo = s:keepcpo
unlet s:keepcpo
" vim:ts=18  fdm=marker
