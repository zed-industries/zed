# vi:set ts=8 sts=4 sw=4 et fdm=marker:
#
# dutch.nsi : Dutch language strings for gvim NSIS installer.
#
# Locale ID    : 1043
# Locale Name  : nl
# fileencoding : UTF-8
# Author       : Peter Odding <peter@peterodding.com>

!insertmacro MUI_LANGUAGE "Dutch"


# Overwrite the default translation.
# These strings should be always English.  Otherwise dosinst.c fails.
LangString ^SetupCaption     ${LANG_DUTCH} \
        "$(^Name) Setup"
LangString ^UninstallCaption ${LANG_DUTCH} \
        "$(^Name) Uninstall"

##############################################################################
# MUI Configuration Strings                                               {{{1
##############################################################################

#LangString str_dest_folder          ${LANG_DUTCH} \
#    "Doelmap (moet eindigen op $\"vim$\")"

LangString str_show_readme          ${LANG_DUTCH} \
    "README weergeven na installatie"

# Install types:
LangString str_type_typical         ${LANG_DUTCH} \
    "Gebruikelijk"

LangString str_type_minimal         ${LANG_DUTCH} \
    "Minimaal"

LangString str_type_full            ${LANG_DUTCH} \
    "Volledig"


##############################################################################
# Section Titles & Description                                            {{{1
##############################################################################

LangString str_section_old_ver      ${LANG_DUTCH} \
    "Bestaande versie(s) de-installeren"
LangString str_desc_old_ver         ${LANG_DUTCH} \
    "Bestaande Vim versie(s) van je systeem verwijderen."

LangString str_section_exe          ${LANG_DUTCH} \
    "Vim GUI"
LangString str_desc_exe             ${LANG_DUTCH} \
    "Vim GUI uitvoerbare bestanden en runtime bestanden.  Dit component is vereist."

LangString str_section_console      ${LANG_DUTCH} \
    "Vim console programma"
LangString str_desc_console         ${LANG_DUTCH} \
    "Console versie van Vim (vim.exe)."

LangString str_section_batch        ${LANG_DUTCH} \
    "Creëer .bat bestanden"
LangString str_desc_batch           ${LANG_DUTCH} \
    "Creëer .bat bestanden voor Vim varianten in de Windows map voor \
     commando regel gebruik."

LangString str_group_icons          ${LANG_DUTCH} \
    "Creëer pictogrammen for Vim"
LangString str_desc_icons           ${LANG_DUTCH} \
    "Creëer pictogrammen voor Vim op verschillende locaties voor gemakkelijke toegang."

LangString str_section_desktop      ${LANG_DUTCH} \
    "Op het bureaublad"
LangString str_desc_desktop         ${LANG_DUTCH} \
    "Creëer pictogrammen voor Vim uitvoerbare bestanden op het bureaublad."

LangString str_section_start_menu   ${LANG_DUTCH} \
    "In de Programma's map in het start menu"
LangString str_desc_start_menu      ${LANG_DUTCH} \
    "Voeg Vim toe aan de programma's map in het start menu.  \
     Van toepassing op Windows 95 en later."

#LangString str_section_quick_launch ${LANG_DUTCH} \
#    "In de snel starten balk"
#LangString str_desc_quick_launch    ${LANG_DUTCH} \
#    "Voeg Vim snelkoppeling toe aan de snel starten balk."

LangString str_section_edit_with    ${LANG_DUTCH} \
    "Voeg Vim contextmenu toe"
LangString str_desc_edit_with       ${LANG_DUTCH} \
    "Voeg Vim toe aan de $\"Openen met...$\" contextmenu lijst."

#LangString str_section_edit_with32  ${LANG_DUTCH} \
#    "32-bit versie"
#LangString str_desc_edit_with32     ${LANG_DUTCH} \
#    "Voeg Vim toe aan de $\"Openen met...$\" contextmenu lijst \
#     voor 32-bit toepassingen."

#LangString str_section_edit_with64  ${LANG_DUTCH} \
#    "64-bit versie"
#LangString str_desc_edit_with64     ${LANG_DUTCH} \
#    "Voeg Vim toe aan de $\"Openen met...$\" contextmenu lijst \
#     voor 64-bit toepassingen."

LangString str_section_vim_rc       ${LANG_DUTCH} \
    "Creëer standaard configuratie"
LangString str_desc_vim_rc          ${LANG_DUTCH} \
    "Creëer een standaard configuratie bestand (_vimrc) als er nog geen bestaat."

LangString str_group_plugin         ${LANG_DUTCH} \
    "Creëer Plugin mappen"
LangString str_desc_plugin          ${LANG_DUTCH} \
    "Creëer plugin mappen.  Plugin mappen maken het mogelijk om \
     Vim uit te breiden door een bestand in een map te plaatsen."

LangString str_section_plugin_home  ${LANG_DUTCH} \
    "Privé"
LangString str_desc_plugin_home     ${LANG_DUTCH} \
    "Create plugin directories in HOME directory."

LangString str_section_plugin_vim   ${LANG_DUTCH} \
    "Gedeeld"
LangString str_desc_plugin_vim      ${LANG_DUTCH} \
    "Creëer plugin mappen in Vim installatie map, deze worden gebruikt \
     voor iedereen op het systeem."

LangString str_section_nls          ${LANG_DUTCH} \
    "Ondersteuning voor andere talen"
LangString str_desc_nls             ${LANG_DUTCH} \
    "Bestanden voor ondersteuning van andere talen dan Engels installeren."

LangString str_unsection_register   ${LANG_DUTCH} \
    "Vim afmelden"
LangString str_desc_unregister      ${LANG_DUTCH} \
    "Registratie van Vim in het systeem ongedaan maken."

LangString str_unsection_exe        ${LANG_DUTCH} \
    "Vim uitvoerbare/runtime bestanden verwijderen"
LangString str_desc_rm_exe          ${LANG_DUTCH} \
    "Verwijder alle Vim uitvoerbare bestanden en runtime bestanden."

LangString str_ungroup_plugin       ${LANG_DUTCH} \
    "Remove plugin directories"
LangString str_desc_rm_plugin       ${LANG_DUTCH} \
    "Remove the plugin directories if they are empty."

LangString str_unsection_plugin_home ${LANG_DUTCH} \
    "Privé"
LangString str_desc_rm_plugin_home  ${LANG_DUTCH} \
    "Remove the plugin directories from HOME directory."

LangString str_unsection_plugin_vim ${LANG_DUTCH} \
    "Gedeeld"
LangString str_desc_rm_plugin_vim   ${LANG_DUTCH} \
    "Remove the plugin directories from Vim install directory."

LangString str_unsection_rootdir    ${LANG_DUTCH} \
    "Remove the Vim root directory"
LangString str_desc_rm_rootdir      ${LANG_DUTCH} \
    "Remove the Vim root directory. It contains your Vim configuration files!"


##############################################################################
# Messages                                                                {{{1
##############################################################################

#LangString str_msg_too_many_ver  ${LANG_DUTCH} \
#    "Er zijn $vim_old_ver_count Vim versies op je systeem gevonden.$\r$\n\
#     Deze installatie kan omgaan met maximaal ${VIM_MAX_OLD_VER} versies.$\r$\n\
#     Verwijder a.u.b. wat versies en probeer het dan opnieuw."

#LangString str_msg_invalid_root  ${LANG_DUTCH} \
#    "Ongeldig installatiepad: $vim_install_root!$\r$\n\
#     Het moet eindelijk op $\"vim$\"."

#LangString str_msg_bin_mismatch  ${LANG_DUTCH} \
#    "Binair pad onjuist!$\r$\n$\r$\n\
#     Het binaire pad zou $\"$vim_bin_path$\" moeten zijn,$\r$\n\
#     maar het systeem geeft aan dat het binaire pad $\"$INSTDIR$\" is."

#LangString str_msg_vim_running   ${LANG_DUTCH} \
#    "Vim is nog actief op je systeem.$\r$\n\
#     Sluit a.u.b. alle instanties van Vim voordat je verder gaat."

#LangString str_msg_register_ole  ${LANG_DUTCH} \
#    "Bezig met proberen om Vim te registreren met OLE. \
#     Er is geen bericht dat aangeeft of deze operatie slaagt."

#LangString str_msg_unreg_ole     ${LANG_DUTCH} \
#    "Bezig met proberen om Vim te de-registreren met OLE. \
#     Er is geen bericht dat aangeeft of deze operatie slaagt."

#LangString str_msg_rm_start      ${LANG_DUTCH} \
#    "De volgende versies worden verwijderd:"

#LangString str_msg_rm_fail       ${LANG_DUTCH} \
#    "De volgende versies konden niet worden verwijderd:"

#LangString str_msg_no_rm_key     ${LANG_DUTCH} \
#    "Kan de uninstaller register sleutel niet vinden."

#LangString str_msg_no_rm_reg     ${LANG_DUTCH} \
#    "Kan de uninstaller niet vinden via het register."

#LangString str_msg_no_rm_exe     ${LANG_DUTCH} \
#    "Kan geen toegang krijgen tot de uninstaller."

#LangString str_msg_rm_copy_fail  ${LANG_DUTCH} \
#    "Kon de uninstaller niet naar een tijdelijke map kopiëren."

#LangString str_msg_rm_run_fail   ${LANG_DUTCH} \
#    "Kon de uninstaller niet uitvoeren."

#LangString str_msg_abort_install ${LANG_DUTCH} \
#    "Installatie wordt gestopt."

LangString str_msg_install_fail  ${LANG_DUTCH} \
    "Installatie is mislukt."

LangString str_msg_rm_exe_fail   ${LANG_DUTCH} \
    "Sommige bestanden in $0 zijn niet verwijderd!$\r$\n\
     Dit moet je handmatig doen."

#LangString str_msg_rm_root_fail  ${LANG_DUTCH} \
#    "WAARSCHUWING: Kan $\"$vim_install_root$\" niet verwijderen omdat het niet leeg is!"

LangString str_msg_uninstalling  ${LANG_DUTCH} \
    "Uninstalling the old version..."

LangString str_msg_registering   ${LANG_DUTCH} \
    "Registering..."

LangString str_msg_unregistering ${LANG_DUTCH} \
    "Unregistering..."


##############################################################################
# Dialog Box                                                              {{{1
##############################################################################

LangString str_vimrc_page_title    ${LANG_DUTCH} \
    "Choose _vimrc settings"
LangString str_vimrc_page_subtitle ${LANG_DUTCH} \
    "Choose the settings for enhancement, keyboard and mouse."

LangString str_msg_compat_title    ${LANG_DUTCH} \
    " Vi / Vim behavior "
LangString str_msg_compat_desc     ${LANG_DUTCH} \
    "&Compatibility and enhancements"
LangString str_msg_compat_vi       ${LANG_DUTCH} \
    "Vi compatible"
LangString str_msg_compat_vim      ${LANG_DUTCH} \
    "Vim original"
LangString str_msg_compat_defaults ${LANG_DUTCH} \
    "Vim with some enhancements (load defaults.vim)"
LangString str_msg_compat_all      ${LANG_DUTCH} \
    "Vim with all enhancements (load vimrc_example.vim) (Default)"

LangString str_msg_keymap_title   ${LANG_DUTCH} \
    " Mappings "
LangString str_msg_keymap_desc    ${LANG_DUTCH} \
    "&Remap a few keys for Windows (Ctrl-V, Ctrl-C, Ctrl-A, Ctrl-S, Ctrl-F, etc)"
LangString str_msg_keymap_default ${LANG_DUTCH} \
    "Do not remap keys (Default)"
LangString str_msg_keymap_windows ${LANG_DUTCH} \
    "Remap a few keys"

LangString str_msg_mouse_title   ${LANG_DUTCH} \
    " Mouse "
LangString str_msg_mouse_desc    ${LANG_DUTCH} \
    "&Behavior of right and left buttons"
LangString str_msg_mouse_default ${LANG_DUTCH} \
    "Right: popup menu, Left: visual mode (Default)"
LangString str_msg_mouse_windows ${LANG_DUTCH} \
    "Right: popup menu, Left: select mode (Windows)"
LangString str_msg_mouse_unix    ${LANG_DUTCH} \
    "Right: extends selection, Left: visual mode (Unix)"
