vim9script

# Vim syntax file
# Language:           MetaPost
# Maintainer:         Nicola Vitacolonna <nvitacolonna@gmail.com>
# Former Maintainers: Andreas Scherer <andreas.scherer@pobox.com>
# Latest Revision:    2022 Aug 12

if exists("b:current_syntax")
  finish
endif

# Deprecation warnings: to be removed eventually
if exists("g:plain_mp_macros")
  echomsg "[mp] g:plain_mp_macros is deprecated: use g:mp_plain_macros instead."
endif
if exists("mfplain_mp_macros")
  echomsg "[mp] g:mfplain_mp_macros is deprecated: use g:mp_mfplain_macros instead."
endif
if exists("other_mp_macros")
  echomsg "[mp] g:other_mp_macros is deprecated: use g:mp_other_macros instead."
endif

# Store the current values of METAFONT global options
const mf_plain_macros = get(g:, "mf_plain_macros", get(g:, "plain_mf_macros", -1))
const mf_plain_modes  = get(g:, "mf_plain_modes",  get(g:, "plain_mf_modes",  -1))
const mf_other_macros = get(g:, "mf_other_macros", get(g:, "other_mf_macros", -1))

g:mf_plain_macros = 0 # plain.mf has no special meaning for MetaPost
g:mf_plain_modes  = 0 # No METAFONT modes
g:mf_other_macros = 0 # cmbase.mf, logo.mf, ... neither

# Read the METAFONT syntax to start with
runtime! syntax/mf.vim
unlet b:current_syntax # Necessary for syn include below

# Restore the value of existing global variables
if mf_plain_macros == -1
  unlet g:mf_plain_macros
else
  g:plain_mf_macros = mf_plain_macros
endif
if mf_plain_modes == -1
  unlet g:mf_plain_modes
else
  g:mf_plain_modes = mf_plain_modes
endif
if mf_other_macros == -1
  unlet g:mf_other_macros
else
  g:mf_other_macros = mf_other_macros
endif

# Use TeX highlighting inside verbatimtex/btex... etex
syn include @MPTeX syntax/tex.vim
unlet b:current_syntax
# These are defined as keywords rather than using matchgroup
# in order to make them available to syntaxcomplete.
syn keyword mpTeXdelim       btex etex verbatimtex contained
syn region mpTeXinsert matchgroup=mpTeXdelim start=/\<verbatimtex\>\|\<btex\>/ end=/\<etex\>/ keepend contains=@MPTeX,mpTeXdelim

# iskeyword must be set after the syn include above, because tex.vim sets `syn
# iskeyword`. Note that keywords do not contain numbers (numbers are
# subscripts)
syntax iskeyword @,_

# MetaPost primitives not found in METAFONT
syn keyword mpBoolExp        bounded clipped filled stroked textual arclength
syn keyword mpNumExp         arctime blackpart bluepart colormodel cyanpart
syn keyword mpNumExp         fontsize greenpart greypart magentapart redpart
syn keyword mpPairExp        yellowpart llcorner lrcorner ulcorner urcorner
syn keyword mpPathExp        envelope pathpart
syn keyword mpPenExp         penpart
syn keyword mpPicExp         dashpart glyph infont
syn keyword mpStringExp      fontpart readfrom textpart
syn keyword mpType           cmykcolor color rgbcolor
# Other MetaPost primitives listed in the manual
syn keyword mpPrimitive      mpxbreak within
# Internal quantities not found in METAFONT
# (Table 6 in MetaPost: A User's Manual)
syn keyword mpInternal       defaultcolormodel hour minute linecap linejoin
syn keyword mpInternal       miterlimit mpprocset mpversion numberprecision
syn keyword mpInternal       numbersystem outputfilename outputformat
syn keyword mpInternal       outputformatoptions outputtemplate prologues
syn keyword mpInternal       restoreclipcolor tracinglostchars troffmode
syn keyword mpInternal       truecorners
# List of commands not found in METAFONT (from MetaPost: A User's Manual)
syn keyword mpCommand        clip closefrom dashed filenametemplate fontmapfile
syn keyword mpCommand        fontmapline setbounds withcmykcolor withcolor
syn keyword mpCommand        withgreyscale withoutcolor withpostscript
syn keyword mpCommand        withprescript withrgbcolor write
# METAFONT internal variables not found in MetaPost
syn keyword notDefined       autorounding chardx chardy fillin granularity
syn keyword notDefined       proofing smoothing tracingedges tracingpens
syn keyword notDefined       turningcheck xoffset yoffset
# Suffix defined only in METAFONT:
syn keyword notDefined       nodot
# Other not implemented primitives (see MetaPost: A User's Manual, §C.1)
syn keyword notDefined       cull display openwindow numspecial totalweight
syn keyword notDefined       withweight

# Keywords defined by plain.mp
if get(g:, "mp_plain_macros", get(g:, "plain_mp_macros", 1)) || get(b:, "mp_metafun", get(g:, "mp_metafun", 0))
  syn keyword mpDef          beginfig clear_pen_memory clearit clearpen clearpen
  syn keyword mpDef          clearxy colorpart cutdraw downto draw drawarrow
  syn keyword mpDef          drawdblarrow drawdot drawoptions endfig erase
  syn keyword mpDef          exitunless fill filldraw flex gobble hide interact
  syn keyword mpDef          label loggingall makelabel numtok penstroke pickup
  syn keyword mpDef          range reflectedabout rotatedaround shipit
  syn keyword mpDef          stop superellipse takepower tracingall tracingnone
  syn keyword mpDef          undraw undrawdot unfill unfilldraw upto
  syn match   mpDef          "???"
  syn keyword mpVardef       arrowhead bbox bot buildcycle byte ceiling center
  syn keyword mpVardef       counterclockwise decr dir direction directionpoint
  syn keyword mpVardef       dotlabel dotlabels image incr interpath inverse
  syn keyword mpVardef       labels lft magstep max min penlabels penpos round
  syn keyword mpVardef       rt savepen solve tensepath thelabel top unitvector
  syn keyword mpVardef       whatever z
  syn keyword mpPrimaryDef   div dotprod gobbled mod
  syn keyword mpSecondaryDef intersectionpoint
  syn keyword mpTertiaryDef  cutafter cutbefore softjoin thru
  syn keyword mpNewInternal  ahangle ahlength bboxmargin beveled butt defaultpen
  syn keyword mpNewInternal  defaultscale dotlabeldiam eps epsilon infinity
  syn keyword mpNewInternal  join_radius labeloffset mitered pen_bot pen_lft
  syn keyword mpNewInternal  pen_rt pen_top rounded squared tolerance
  # Predefined constants
  syn keyword mpConstant     EOF background base_name base_version black
  syn keyword mpConstant     blankpicture blue ditto down evenly fullcircle
  syn keyword mpConstant     green halfcircle identity left origin penrazor
  syn keyword mpConstant     penspeck pensquare quartercircle red right
  syn keyword mpConstant     unitsquare up white withdots
  # Other predefined variables
  syn keyword mpVariable     currentpen currentpen_path currentpicture cuttings
  syn keyword mpVariable     defaultfont extra_beginfig extra_endfig
  syn keyword mpVariable     laboff labxf labyf laboff labxf labyf
  syn match   mpVariable     /\.\%(lft\|rt\|bot\|top\|ulft\|urt\|llft\|lrt\)\>/
  # let statements:
  syn keyword mpnumExp       abs
  syn keyword mpDef          rotatedabout
  syn keyword mpCommand      bye relax
  # on and off are not technically keywords, but it is nice to highlight them
  # inside dashpattern().
  syn keyword mpOnOff        off on contained
  syn keyword mpDash         dashpattern contained
  syn region  mpDashPattern start="dashpattern\s*" end=")"he=e-1 contains=mfNumeric,mfLength,mpOnOff,mpDash
endif

# Keywords defined by mfplain.mp
if get(g:, "mp_mfplain_macros", get(g:, "mfplain_mp_macros", 0))
  syn keyword mpDef          beginchar capsule_def change_width
  syn keyword mpDef          define_blacker_pixels define_corrected_pixels
  syn keyword mpDef          define_good_x_pixels define_good_y_pixels
  syn keyword mpDef          define_horizontal_corrected_pixels define_pixels
  syn keyword mpDef          define_whole_blacker_pixels define_whole_pixels
  syn keyword mpDef          define_whole_vertical_blacker_pixels
  syn keyword mpDef          define_whole_vertical_pixels endchar
  syn keyword mpDef          font_coding_scheme font_extra_space font_identifier
  syn keyword mpDef          font_normal_shrink font_normal_space
  syn keyword mpDef          font_normal_stretch font_quad font_size font_slant
  syn keyword mpDef          font_x_height italcorr labelfont lowres_fix makebox
  syn keyword mpDef          makegrid maketicks mode_def mode_setup proofrule
  syn keyword mpDef          smode
  syn keyword mpVardef       hround proofrulethickness vround
  syn keyword mpNewInternal  blacker o_correction
  syn keyword mpVariable     extra_beginchar extra_endchar extra_setup rulepen
  # plus some no-ops, also from mfplain.mp
  syn keyword mpDef          cull cullit gfcorners imagerules nodisplays
  syn keyword mpDef          notransforms openit proofoffset screenchars
  syn keyword mpDef          screenrule screenstrokes showit
  syn keyword mpVardef       grayfont slantfont titlefont
  syn keyword mpVariable     currenttransform
  syn keyword mpConstant     unitpixel
  # These are not listed in the MetaPost manual, and some are ignored by
  # MetaPost, but are nonetheless defined in mfplain.mp
  syn keyword mpDef          killtext
  syn match   mpVardef       "\<good\.\%(x\|y\|lft\|rt\|top\|bot\)\>"
  syn keyword mpVariable     aspect_ratio localfont mag mode mode_name
  syn keyword mpVariable     proofcolor
  syn keyword mpConstant     lowres proof smoke
  syn keyword mpNewInternal  autorounding bp_per_pixel granularity
  syn keyword mpNewInternal  number_of_modes proofing smoothing turningcheck
endif

# Keywords defined by all base macro packages:
# - (r)boxes.mp
# - format.mp
# - graph.mp
# - marith.mp
# - sarith.mp
# - string.mp
# - TEX.mp
if get(g:, "mp_other_macros", get(g:, "other_mp_macros", 1))
  # boxes and rboxes
  syn keyword mpDef          boxjoin drawboxed drawboxes drawunboxed
  syn keyword mpNewInternal  circmargin defaultdx defaultdy rbox_radius
  syn keyword mpVardef       boxit bpath circleit fixpos fixsize generic_declare
  syn keyword mpVardef       generic_redeclare generisize pic rboxit str_prefix
  # format
  syn keyword mpVardef       Mformat format init_numbers roundd
  syn keyword mpVariable     Fe_base Fe_plus
  syn keyword mpConstant     Ten_to
  # graph
  syn keyword mpDef          Gfor Gxyscale OUT auto begingraph endgraph gdata
  syn keyword mpDef          gdraw gdrawarrow gdrawdblarrow gfill plot
  syn keyword mpVardef       augment autogrid frame gdotlabel glabel grid itick
  syn keyword mpVardef       otick
  syn keyword mpVardef       Mreadpath setcoords setrange
  syn keyword mpNewInternal  Gmarks Gminlog Gpaths linear log
  syn keyword mpVariable     Autoform Gemarks Glmarks Gumarks
  syn keyword mpConstant     Gtemplate
  syn match   mpVariable     /Gmargin\.\%(low\|high\)/
  # marith
  syn keyword mpVardef       Mabs Meform Mexp Mexp_str Mlog Mlog_Str Mlog_str
  syn keyword mpPrimaryDef   Mdiv Mmul
  syn keyword mpSecondaryDef Madd Msub
  syn keyword mpTertiaryDef  Mleq
  syn keyword mpNewInternal  Mten Mzero
  # sarith
  syn keyword mpVardef       Sabs Scvnum
  syn keyword mpPrimaryDef   Sdiv Smul
  syn keyword mpSecondaryDef Sadd Ssub
  syn keyword mpTertiaryDef  Sleq Sneq
  # string
  syn keyword mpVardef       cspan isdigit loptok
  # TEX
  syn keyword mpVardef       TEX TEXPOST TEXPRE
endif

if get(b:, "mp_metafun", get(g:, "mp_metafun", 0))
  # MetaFun additions to MetaPost base file
  syn keyword mpConstant cyan magenta yellow
  syn keyword mpConstant penspec
  syn keyword mpNumExp   graypart greycolor graycolor

  # Highlight TeX keywords (for MetaPost embedded in ConTeXt documents)
  syn match   mpTeXKeyword  '\\[a-zA-Z@]\+'

  syn keyword mpPrimitive runscript

  runtime! syntax/shared/context-data-metafun.vim

  hi def link metafunCommands   Statement
  hi def link metafunInternals  Identifier
endif

# Define the default highlighting
hi def link mpTeXdelim     mpPrimitive
hi def link mpBoolExp      mfBoolExp
hi def link mpNumExp       mfNumExp
hi def link mpPairExp      mfPairExp
hi def link mpPathExp      mfPathExp
hi def link mpPenExp       mfPenExp
hi def link mpPicExp       mfPicExp
hi def link mpStringExp    mfStringExp
hi def link mpInternal     mfInternal
hi def link mpCommand      mfCommand
hi def link mpType         mfType
hi def link mpPrimitive    mfPrimitive
hi def link mpDef          mfDef
hi def link mpVardef       mpDef
hi def link mpPrimaryDef   mpDef
hi def link mpSecondaryDef mpDef
hi def link mpTertiaryDef  mpDef
hi def link mpNewInternal  mpInternal
hi def link mpVariable     mfVariable
hi def link mpConstant     mfConstant
hi def link mpOnOff        mpPrimitive
hi def link mpDash         mpPrimitive
hi def link mpTeXKeyword   Identifier

b:current_syntax = "mp"

# vim: sw=2 fdm=marker
