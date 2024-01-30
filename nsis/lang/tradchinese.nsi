# vi:set ts=8 sts=4 sw=4 et fdm=marker:
#
# tradchinese.nsi: Traditional Chinese language strings for gvim NSIS
# installer.
#
# Locale ID    : 1028
# fileencoding : UTF-8
# Author       : Guopeng Wen

!insertmacro MUI_LANGUAGE "TradChinese"


# Overwrite the default translation.
# These strings should be always English.  Otherwise dosinst.c fails.
LangString ^SetupCaption     ${LANG_TRADCHINESE} \
        "$(^Name) Setup"
LangString ^UninstallCaption ${LANG_TRADCHINESE} \
        "$(^Name) Uninstall"

##############################################################################
# MUI Configuration Strings                                               {{{1
##############################################################################

#LangString str_dest_folder          ${LANG_TRADCHINESE} \
#    "安裝資料夾 (必須以 vim 結尾)"

LangString str_show_readme          ${LANG_TRADCHINESE} \
    "安裝完成後顯示 README 檔案"

# Install types:
LangString str_type_typical         ${LANG_TRADCHINESE} \
    "典型安裝"

LangString str_type_minimal         ${LANG_TRADCHINESE} \
    "最小安裝"

LangString str_type_full            ${LANG_TRADCHINESE} \
    "完全安裝"


##############################################################################
# Section Titles & Description                                            {{{1
##############################################################################

LangString str_section_old_ver      ${LANG_TRADCHINESE} \
    "移除舊版本"
LangString str_desc_old_ver         ${LANG_TRADCHINESE} \
    "移除閣下電腦上舊版本的 Vim。"

LangString str_section_exe          ${LANG_TRADCHINESE} \
    "安裝 Vim 圖形界面程式"
LangString str_desc_exe             ${LANG_TRADCHINESE} \
    "安裝 Vim 圖形界面程式及腳本。此為必選安裝。"

LangString str_section_console      ${LANG_TRADCHINESE} \
    "安裝 Vim 命令行程式"
LangString str_desc_console         ${LANG_TRADCHINESE} \
    "安裝 Vim 命令行程式 (vim.exe)。該程式在控制臺窗口中運行。"

LangString str_section_batch        ${LANG_TRADCHINESE} \
    "安裝批次檔案"
LangString str_desc_batch           ${LANG_TRADCHINESE} \
    "為 Vim 的各種變體創建批次檔，以便在命令行下啟動 Vim。"

LangString str_group_icons          ${LANG_TRADCHINESE} \
    "建立 Vim 圖示"
LangString str_desc_icons           ${LANG_TRADCHINESE} \
    "建立若干 Vim 圖示，以便于使用 Vim。"

LangString str_section_desktop      ${LANG_TRADCHINESE} \
    "於桌面"
LangString str_desc_desktop         ${LANG_TRADCHINESE} \
    "建立若干 Vim 圖示於桌面上，以方便啟動 Vim。"

LangString str_section_start_menu   ${LANG_TRADCHINESE} \
    "於「開始」功能表的「程式」集"
LangString str_desc_start_menu      ${LANG_TRADCHINESE} \
    "在「開始」功能表的「程式」集中建立 Vim 啟動組。\
     適用于 Windows 95 及以上版本。"

#LangString str_section_quick_launch ${LANG_TRADCHINESE} \
#    "於快速啟動列"
#LangString str_desc_quick_launch    ${LANG_TRADCHINESE} \
#    "在快速啟動列中建立 Vim 圖示。"

LangString str_section_edit_with    ${LANG_TRADCHINESE} \
    "安裝快捷選單"
LangString str_desc_edit_with       ${LANG_TRADCHINESE} \
    "在「打開方式」快捷選單中添加 Vim 項。"

#LangString str_section_edit_with32  ${LANG_TRADCHINESE} \
#    "32 位元版本"
#LangString str_desc_edit_with32     ${LANG_TRADCHINESE} \
#    "在 32 位元程式的「打開方式」快捷選單中添加 Vim 項。"

#LangString str_section_edit_with64  ${LANG_TRADCHINESE} \
#    "64 位元版本"
#LangString str_desc_edit_with64     ${LANG_TRADCHINESE} \
#    "在 64 位元程式的「打開方式」快捷選單中添加 Vim 項。"

LangString str_section_vim_rc       ${LANG_TRADCHINESE} \
    "建立默認設定檔"
LangString str_desc_vim_rc          ${LANG_TRADCHINESE} \
    "在安裝資料夾下建立默認的 Vim 設定檔(_vimrc)。\
     若該設定檔已經存在，則略過此項。"

LangString str_group_plugin         ${LANG_TRADCHINESE} \
    "建立插件資料夾"
LangString str_desc_plugin          ${LANG_TRADCHINESE} \
    "建立(空的)插件資料夾結構。插件資料夾用于安裝 Vim 的擴展插件，\
     只要將檔案復制到相關的子資料夾中即可。"

LangString str_section_plugin_home  ${LANG_TRADCHINESE} \
    "建立插件資料夾"
LangString str_desc_plugin_home     ${LANG_TRADCHINESE} \
    "Create plugin directories in HOME directory."

LangString str_section_plugin_vim   ${LANG_TRADCHINESE} \
    "建立共享插件資料夾"
LangString str_desc_plugin_vim      ${LANG_TRADCHINESE} \
    "在 Vim 安裝資料夾下建立(空的)插件資料夾結構，電腦上所有用戶都能使用安裝\
     在該資料夾里的擴展插件。"

LangString str_section_nls          ${LANG_TRADCHINESE} \
    "安裝本地語言支持"
LangString str_desc_nls             ${LANG_TRADCHINESE} \
    "安裝用于支持本地語言的檔案。"

LangString str_unsection_register   ${LANG_TRADCHINESE} \
    "移除 Vim 系統設定"
LangString str_desc_unregister      ${LANG_TRADCHINESE} \
    "移除與 Vim 相關的系統設定。"

LangString str_unsection_exe        ${LANG_TRADCHINESE} \
    "移除 Vim 程式及腳本"
LangString str_desc_rm_exe          ${LANG_TRADCHINESE} \
    "移除所有的 Vim 程式及腳本。"

LangString str_ungroup_plugin       ${LANG_TRADCHINESE} \
    "Remove plugin directories"
LangString str_desc_rm_plugin       ${LANG_TRADCHINESE} \
    "Remove the plugin directories if they are empty."

LangString str_unsection_plugin_home ${LANG_TRADCHINESE} \
    "Private"
LangString str_desc_rm_plugin_home  ${LANG_TRADCHINESE} \
    "Remove the vimfiles directory in HOME directory."

LangString str_unsection_plugin_vim ${LANG_TRADCHINESE} \
    "Shared"
LangString str_desc_rm_plugin_vim   ${LANG_TRADCHINESE} \
    "Remove the vimfiles directory in Vim install directory."

LangString str_unsection_rootdir    ${LANG_TRADCHINESE} \
    "Remove the Vim root directory"
LangString str_desc_rm_rootdir      ${LANG_TRADCHINESE} \
    "Remove the Vim root directory. It contains your Vim configuration files!"


##############################################################################
# Messages                                                                {{{1
##############################################################################

#LangString str_msg_too_many_ver  ${LANG_TRADCHINESE} \
#    "閣下的電腦上安裝了 $vim_old_ver_count 個不同版本的 Vim，$\r$\n\
#     但是本安裝程式最多只能處理 ${VIM_MAX_OLD_VER} 個版本。$\r$\n\
#     煩請閣下手工移除一些版本以后再運行本安裝程式。"

#LangString str_msg_invalid_root  ${LANG_TRADCHINESE} \
#    "安裝資料夾「$vim_install_root」無效！$\r$\n\
#     該資料夾必須以「vim」結尾。"

#LangString str_msg_bin_mismatch  ${LANG_TRADCHINESE} \
#    "Vim 執行程式安裝路徑異常！$\r$\n$\r$\n\
#     該版本 Vim 的執行程式安裝路徑應該是「$vim_bin_path」,$\r$\n\
#     而系統卻指示該路徑為「$INSTDIR」。"

#LangString str_msg_vim_running   ${LANG_TRADCHINESE} \
#    "閣下的電腦上尚有正在運行之 Vim，$\r$\n\
#     煩請閣下在執行后續步驟前將其全部退出。"

#LangString str_msg_register_ole  ${LANG_TRADCHINESE} \
#    "試圖注冊 Vim OLE 伺服程式。請注意不論成功與否都不再顯示進一步的信息。"

#LangString str_msg_unreg_ole     ${LANG_TRADCHINESE} \
#    "試圖注銷 Vim OLE 伺服程式。請注意不論成功與否都不再顯示進一步的信息。"

#LangString str_msg_rm_start      ${LANG_TRADCHINESE} \
#    "正移除如下版本："

#LangString str_msg_rm_fail       ${LANG_TRADCHINESE} \
#    "以下版本移除失敗："

#LangString str_msg_no_rm_key     ${LANG_TRADCHINESE} \
#    "找不到反安裝程式的登錄檔入口。"

#LangString str_msg_no_rm_reg     ${LANG_TRADCHINESE} \
#    "在登錄檔中未找到反安裝程式路徑。"

#LangString str_msg_no_rm_exe     ${LANG_TRADCHINESE} \
#    "找不到反安裝程式。"

#LangString str_msg_rm_copy_fail  ${LANG_TRADCHINESE} \
#    "無法將法將反安裝程式复制到臨時目錄。"

#LangString str_msg_rm_run_fail   ${LANG_TRADCHINESE} \
#    "執行反安裝程式失敗。"

#LangString str_msg_abort_install ${LANG_TRADCHINESE} \
#    "安裝程式將退出。"

LangString str_msg_install_fail  ${LANG_TRADCHINESE} \
    "安裝失敗。預祝下次好運。"

LangString str_msg_rm_exe_fail   ${LANG_TRADCHINESE} \
    "資料夾「$0」下有部分檔案未能移除！$\r$\n\
     閣下只能手工移除該資料夾。"

#LangString str_msg_rm_root_fail  ${LANG_TRADCHINESE} \
#    "警告：無法刪除 Vim 安裝資料夾「$vim_install_root」，\
#     該資料夾下仍有其他檔案。"

LangString str_msg_uninstalling  ${LANG_TRADCHINESE} \
    "Uninstalling the old version..."

LangString str_msg_registering   ${LANG_TRADCHINESE} \
    "Registering..."

LangString str_msg_unregistering ${LANG_TRADCHINESE} \
    "Unregistering..."


##############################################################################
# Dialog Box                                                              {{{1
##############################################################################

LangString str_vimrc_page_title    ${LANG_TRADCHINESE} \
    "Choose _vimrc settings"
LangString str_vimrc_page_subtitle ${LANG_TRADCHINESE} \
    "Choose the settings for enhancement, keyboard and mouse."

LangString str_msg_compat_title    ${LANG_TRADCHINESE} \
    " Vi / Vim behavior "
LangString str_msg_compat_desc     ${LANG_TRADCHINESE} \
    "&Compatibility and enhancements"
LangString str_msg_compat_vi       ${LANG_TRADCHINESE} \
    "Vi compatible"
LangString str_msg_compat_vim      ${LANG_TRADCHINESE} \
    "Vim original"
LangString str_msg_compat_defaults ${LANG_TRADCHINESE} \
    "Vim with some enhancements (load defaults.vim)"
LangString str_msg_compat_all      ${LANG_TRADCHINESE} \
    "Vim with all enhancements (load vimrc_example.vim) (Default)"

LangString str_msg_keymap_title   ${LANG_TRADCHINESE} \
    " Mappings "
LangString str_msg_keymap_desc    ${LANG_TRADCHINESE} \
    "&Remap a few keys for Windows (Ctrl-V, Ctrl-C, Ctrl-A, Ctrl-S, Ctrl-F, etc)"
LangString str_msg_keymap_default ${LANG_TRADCHINESE} \
    "Do not remap keys (Default)"
LangString str_msg_keymap_windows ${LANG_TRADCHINESE} \
    "Remap a few keys"

LangString str_msg_mouse_title   ${LANG_TRADCHINESE} \
    " Mouse "
LangString str_msg_mouse_desc    ${LANG_TRADCHINESE} \
    "&Behavior of right and left buttons"
LangString str_msg_mouse_default ${LANG_TRADCHINESE} \
    "Right: popup menu, Left: visual mode (Default)"
LangString str_msg_mouse_windows ${LANG_TRADCHINESE} \
    "Right: popup menu, Left: select mode (Windows)"
LangString str_msg_mouse_unix    ${LANG_TRADCHINESE} \
    "Right: extends selection, Left: visual mode (Unix)"
