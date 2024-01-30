/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

// Use PLURAL_MSG() for messages that are passed to ngettext(), so that the
// second one uses msgid_plural.
#ifdef DO_INIT
# define PLURAL_MSG(var1, msg1, var2, msg2) \
	char var1[] = msg1; \
	char var2[] = msg2;
#else
# define PLURAL_MSG(var1, msg1, var2, msg2) \
	extern char var1[]; \
	extern char var2[];
#endif

/*
 * Definition of error messages, sorted on error number.
 */

EXTERN char e_interrupted[]
	INIT(= N_("Interrupted"));

EXTERN char e_backslash_should_be_followed_by[]
	INIT(= N_("E10: \\ should be followed by /, ? or &"));
EXTERN char e_invalid_in_cmdline_window[]
	INIT(= N_("E11: Invalid in command-line window; :q<CR> closes the window"));
EXTERN char e_command_not_allowed_from_vimrc_in_current_dir_or_tag_search[]
	INIT(= N_("E12: Command not allowed from exrc/vimrc in current dir or tag search"));
EXTERN char e_file_exists[]
	INIT(= N_("E13: File exists (add ! to override)"));
// E14 unused
#ifdef FEAT_EVAL
EXTERN char e_invalid_expression_str[]
	INIT(= N_("E15: Invalid expression: \"%s\""));
#endif
EXTERN char e_invalid_range[]
	INIT(= N_("E16: Invalid range"));
#if defined(UNIX) || defined(FEAT_SYN_HL) \
	    || defined(FEAT_SPELL) || defined(FEAT_EVAL)
EXTERN char e_str_is_directory[]
	INIT(= N_("E17: \"%s\" is a directory"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_unexpected_characters_in_let[]
	INIT(= N_("E18: Unexpected characters in :let"));
EXTERN char e_unexpected_characters_in_assignment[]
	INIT(= N_("E18: Unexpected characters in assignment"));
#endif
EXTERN char e_mark_has_invalid_line_number[]
	INIT(= N_("E19: Mark has invalid line number"));
EXTERN char e_mark_not_set[]
	INIT(= N_("E20: Mark not set"));
EXTERN char e_cannot_make_changes_modifiable_is_off[]
	INIT(= N_("E21: Cannot make changes, 'modifiable' is off"));
EXTERN char e_scripts_nested_too_deep[]
	INIT(= N_("E22: Scripts nested too deep"));
EXTERN char e_no_alternate_file[]
	INIT(= N_("E23: No alternate file"));
EXTERN char e_no_such_abbreviation[]
	INIT(= N_("E24: No such abbreviation"));
#if !defined(FEAT_GUI) || defined(VIMDLL)
EXTERN char e_gui_cannot_be_used_not_enabled_at_compile_time[]
	INIT(= N_("E25: GUI cannot be used: Not enabled at compile time"));
#endif
#ifndef FEAT_RIGHTLEFT
EXTERN char e_hebrew_cannot_be_used_not_enabled_at_compile_time[]
	INIT(= N_("E26: Hebrew cannot be used: Not enabled at compile time\n"));
#endif
EXTERN char e_farsi_support_has_been_removed[]
	INIT(= N_("E27: Farsi support has been removed\n"));
#if defined(FEAT_SEARCH_EXTRA) || defined(FEAT_SYN_HL)
EXTERN char e_no_such_highlight_group_name_str[]
	INIT(= N_("E28: No such highlight group name: %s"));
#endif
EXTERN char e_no_inserted_text_yet[]
	INIT(= N_("E29: No inserted text yet"));
EXTERN char e_no_previous_command_line[]
	INIT(= N_("E30: No previous command line"));
EXTERN char e_no_such_mapping[]
	INIT(= N_("E31: No such mapping"));
EXTERN char e_no_file_name[]
	INIT(= N_("E32: No file name"));
EXTERN char e_no_previous_substitute_regular_expression[]
	INIT(= N_("E33: No previous substitute regular expression"));
EXTERN char e_no_previous_command[]
	INIT(= N_("E34: No previous command"));
EXTERN char e_no_previous_regular_expression[]
	INIT(= N_("E35: No previous regular expression"));
EXTERN char e_not_enough_room[]
	INIT(= N_("E36: Not enough room"));
EXTERN char e_no_write_since_last_change[]
	INIT(= N_("E37: No write since last change"));
EXTERN char e_no_write_since_last_change_add_bang_to_override[]
	INIT(= N_("E37: No write since last change (add ! to override)"));
EXTERN char e_null_argument[]
	INIT(= "E38: Null argument");
#if defined(FEAT_DIGRAPHS) || defined(FEAT_TIMERS) || defined(FEAT_EVAL)
EXTERN char e_number_expected[]
	INIT(= N_("E39: Number expected"));
#endif
#ifdef FEAT_QUICKFIX
EXTERN char e_cant_open_errorfile_str[]
	INIT(= N_("E40: Can't open errorfile %s"));
#endif
EXTERN char e_out_of_memory[]
	INIT(= N_("E41: Out of memory!"));
#ifdef FEAT_QUICKFIX
EXTERN char e_no_errors[]
	INIT(= N_("E42: No Errors"));
#endif
EXTERN char e_damaged_match_string[]
	INIT(= "E43: Damaged match string");
EXTERN char e_corrupted_regexp_program[]
	INIT(= "E44: Corrupted regexp program");
EXTERN char e_readonly_option_is_set_add_bang_to_override[]
	INIT(= N_("E45: 'readonly' option is set (add ! to override)"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_change_readonly_variable[]
	INIT(= N_("E46: Cannot change read-only variable"));
EXTERN char e_cannot_change_readonly_variable_str[]
	INIT(= N_("E46: Cannot change read-only variable \"%s\""));
#endif
#ifdef FEAT_QUICKFIX
EXTERN char e_error_while_reading_errorfile[]
	INIT(= N_("E47: Error while reading errorfile"));
#endif
#ifdef HAVE_SANDBOX
EXTERN char e_not_allowed_in_sandbox[]
	INIT(= N_("E48: Not allowed in sandbox"));
#endif
EXTERN char e_invalid_scroll_size[]
	INIT(= N_("E49: Invalid scroll size"));
#ifdef FEAT_SYN_HL
EXTERN char e_too_many_z[]
	INIT(= N_("E50: Too many \\z("));
#endif
EXTERN char e_too_many_str_open[]
	INIT(= N_("E51: Too many %s("));
#ifdef FEAT_SYN_HL
EXTERN char e_unmatched_z[]
	INIT(= N_("E52: Unmatched \\z("));
#endif
EXTERN char e_unmatched_str_percent_open[]
	INIT(= N_("E53: Unmatched %s%%("));
EXTERN char e_unmatched_str_open[]
	INIT(= N_("E54: Unmatched %s("));
EXTERN char e_unmatched_str_close[]
	INIT(= N_("E55: Unmatched %s)"));
// E56 unused
// E57 unused
// E58 unused
EXTERN char e_invalid_character_after_str_at[]
	INIT(= N_("E59: Invalid character after %s@"));
EXTERN char e_too_many_complex_str_curly[]
	INIT(= N_("E60: Too many complex %s{...}s"));
EXTERN char e_nested_str[]
	INIT(= N_("E61: Nested %s*"));
EXTERN char e_nested_str_chr[]
	INIT(= N_("E62: Nested %s%c"));
EXTERN char e_invalid_use_of_underscore[]
	INIT(= N_("E63: Invalid use of \\_"));
EXTERN char e_str_chr_follows_nothing[]
	INIT(= N_("E64: %s%c follows nothing"));
EXTERN char e_illegal_back_reference[]
	INIT(= N_("E65: Illegal back reference"));
#ifdef FEAT_SYN_HL
EXTERN char e_z_not_allowed_here[]
	INIT(= N_("E66: \\z( not allowed here"));
EXTERN char e_z1_z9_not_allowed_here[]
	INIT(= N_("E67: \\z1 - \\z9 not allowed here"));
#endif
EXTERN char e_invalid_character_after_bsl_z[]
	INIT(= N_("E68: Invalid character after \\z"));
EXTERN char e_missing_sb_after_str[]
	INIT(= N_("E69: Missing ] after %s%%["));
EXTERN char e_empty_str_brackets[]
	INIT(= N_("E70: Empty %s%%[]"));
EXTERN char e_invalid_character_after_str[]
	INIT(= N_("E71: Invalid character after %s%%"));
EXTERN char e_close_error_on_swap_file[]
	INIT(= N_("E72: Close error on swap file"));
EXTERN char e_tag_stack_empty[]
	INIT(= N_("E73: Tag stack empty"));
EXTERN char e_command_too_complex[]
	INIT(= N_("E74: Command too complex"));
EXTERN char e_name_too_long[]
	INIT(= N_("E75: Name too long"));
EXTERN char e_too_many_brackets[]
	INIT(= N_("E76: Too many ["));
EXTERN char e_too_many_file_names[]
	INIT(= N_("E77: Too many file names"));
EXTERN char e_unknown_mark[]
	INIT(= N_("E78: Unknown mark"));
EXTERN char e_cannot_expand_wildcards[]
	INIT(= N_("E79: Cannot expand wildcards"));
EXTERN char e_error_while_writing[]
	INIT(= N_("E80: Error while writing"));
#ifdef FEAT_EVAL
EXTERN char e_using_sid_not_in_script_context[]
	INIT(= N_("E81: Using <SID> not in a script context"));
#endif
EXTERN char e_cannot_allocate_any_buffer_exiting[]
	INIT(= N_("E82: Cannot allocate any buffer, exiting..."));
EXTERN char e_cannot_allocate_buffer_using_other_one[]
	INIT(= N_("E83: Cannot allocate buffer, using other one..."));
EXTERN char e_no_modified_buffer_found[]
	INIT(= N_("E84: No modified buffer found"));
EXTERN char e_there_is_no_listed_buffer[]
	INIT(= N_("E85: There is no listed buffer"));
EXTERN char e_buffer_nr_does_not_exist[]
	INIT(= N_("E86: Buffer %ld does not exist"));
EXTERN char e_cannot_go_beyond_last_buffer[]
	INIT(= N_("E87: Cannot go beyond last buffer"));
EXTERN char e_cannot_go_before_first_buffer[]
	INIT(= N_("E88: Cannot go before first buffer"));
EXTERN char e_no_write_since_last_change_for_buffer_nr_add_bang_to_override[]
	INIT(= N_("E89: No write since last change for buffer %d (add ! to override)"));
EXTERN char e_cannot_unload_last_buffer[]
	INIT(= N_("E90: Cannot unload last buffer"));
EXTERN char e_shell_option_is_empty[]
	INIT(= N_("E91: 'shell' option is empty"));
EXTERN char e_buffer_nr_not_found[]
	INIT(= N_("E92: Buffer %d not found"));
EXTERN char e_more_than_one_match_for_str[]
	INIT(= N_("E93: More than one match for %s"));
EXTERN char e_no_matching_buffer_for_str[]
	INIT(= N_("E94: No matching buffer for %s"));
EXTERN char e_buffer_with_this_name_already_exists[]
	INIT(= N_("E95: Buffer with this name already exists"));
#if defined(FEAT_DIFF)
EXTERN char e_cannot_diff_more_than_nr_buffers[]
	INIT(= N_("E96: Cannot diff more than %d buffers"));
EXTERN char e_cannot_create_diffs[]
	INIT(= N_("E97: Cannot create diffs"));
EXTERN char e_cannot_read_diff_output[]
	INIT(= N_("E98: Cannot read diff output"));
EXTERN char e_current_buffer_is_not_in_diff_mode[]
	INIT(= N_("E99: Current buffer is not in diff mode"));
EXTERN char e_no_other_buffer_in_diff_mode[]
	INIT(= N_("E100: No other buffer in diff mode"));
EXTERN char e_more_than_two_buffers_in_diff_mode_dont_know_which_one_to_use[]
	INIT(= N_("E101: More than two buffers in diff mode, don't know which one to use"));
EXTERN char e_cant_find_buffer_str[]
	INIT(= N_("E102: Can't find buffer \"%s\""));
EXTERN char e_buffer_str_is_not_in_diff_mode[]
	INIT(= N_("E103: Buffer \"%s\" is not in diff mode"));
#endif
#ifdef FEAT_DIGRAPHS
EXTERN char e_escape_not_allowed_in_digraph[]
	INIT(= N_("E104: Escape not allowed in digraph"));
#endif
#ifdef FEAT_KEYMAP
EXTERN char e_using_loadkeymap_not_in_sourced_file[]
	INIT(= N_("E105: Using :loadkeymap not in a sourced file"));
#endif
// E106 unused
#ifdef FEAT_EVAL
EXTERN char e_missing_parenthesis_str[]
	INIT(= N_("E107: Missing parentheses: %s"));
EXTERN char e_no_such_variable_str[]
	INIT(= N_("E108: No such variable: \"%s\""));
EXTERN char e_missing_colon_after_questionmark[]
	INIT(= N_("E109: Missing ':' after '?'"));
EXTERN char e_missing_closing_paren[]
	INIT(= N_("E110: Missing ')'"));
EXTERN char e_missing_closing_square_brace[]
	INIT(= N_("E111: Missing ']'"));
EXTERN char e_option_name_missing_str[]
	INIT(= N_("E112: Option name missing: %s"));
EXTERN char e_unknown_option_str[]
	INIT(= N_("E113: Unknown option: %s"));
EXTERN char e_missing_double_quote_str[]
	INIT(= N_("E114: Missing double quote: %s"));
EXTERN char e_missing_single_quote_str[]
	INIT(= N_("E115: Missing single quote: %s"));
EXTERN char e_invalid_arguments_for_function_str[]
	INIT(= N_("E116: Invalid arguments for function %s"));
EXTERN char e_unknown_function_str[]
	INIT(= N_("E117: Unknown function: %s"));
EXTERN char e_too_many_arguments_for_function_str[]
	INIT(= N_("E118: Too many arguments for function: %s"));
EXTERN char e_not_enough_arguments_for_function_str[]
	INIT(= N_("E119: Not enough arguments for function: %s"));
EXTERN char e_using_sid_not_in_script_context_str[]
	INIT(= N_("E120: Using <SID> not in a script context: %s"));
EXTERN char e_undefined_variable_str[]
	INIT(= N_("E121: Undefined variable: %s"));
EXTERN char e_undefined_variable_char_str[]
	INIT(= N_("E121: Undefined variable: %c:%s"));
EXTERN char e_function_str_already_exists_add_bang_to_replace[]
	INIT(= N_("E122: Function %s already exists, add ! to replace it"));
EXTERN char e_undefined_function_str[]
	INIT(= N_("E123: Undefined function: %s"));
EXTERN char e_missing_paren_str[]
	INIT(= N_("E124: Missing '(': %s"));
EXTERN char e_illegal_argument_str[]
	INIT(= N_("E125: Illegal argument: %s"));
EXTERN char e_missing_endfunction[]
	INIT(= N_("E126: Missing :endfunction"));
EXTERN char e_cannot_redefine_function_str_it_is_in_use[]
	INIT(= N_("E127: Cannot redefine function %s: It is in use"));
EXTERN char e_function_name_must_start_with_capital_or_s_str[]
	INIT(= N_("E128: Function name must start with a capital or \"s:\": %s"));
EXTERN char e_function_name_required[]
	INIT(= N_("E129: Function name required"));
// E130 unused
EXTERN char e_cannot_delete_function_str_it_is_in_use[]
	INIT(= N_("E131: Cannot delete function %s: It is in use"));
EXTERN char e_function_call_depth_is_higher_than_macfuncdepth[]
	INIT(= N_("E132: Function call depth is higher than 'maxfuncdepth'"));
EXTERN char e_return_not_inside_function[]
	INIT(= N_("E133: :return not inside a function"));
#endif
EXTERN char e_cannot_move_range_of_lines_into_itself[]
	INIT(= N_("E134: Cannot move a range of lines into itself"));
EXTERN char e_filter_autocommands_must_not_change_current_buffer[]
	INIT(= N_("E135: *Filter* Autocommands must not change current buffer"));
#if defined(FEAT_VIMINFO)
EXTERN char e_viminfo_too_many_errors_skipping_rest_of_file[]
	INIT(= N_("E136: viminfo: Too many errors, skipping rest of file"));
EXTERN char e_viminfo_file_is_not_writable_str[]
	INIT(= N_("E137: Viminfo file is not writable: %s"));
EXTERN char e_cant_write_viminfo_file_str[]
	INIT(= N_("E138: Can't write viminfo file %s!"));
#endif
EXTERN char e_file_is_loaded_in_another_buffer[]
	INIT(= N_("E139: File is loaded in another buffer"));
EXTERN char e_use_bang_to_write_partial_buffer[]
	INIT(= N_("E140: Use ! to write partial buffer"));
EXTERN char e_no_file_name_for_buffer_nr[]
	INIT(= N_("E141: No file name for buffer %ld"));
EXTERN char e_file_not_written_writing_is_disabled_by_write_option[]
	INIT(= N_("E142: File not written: Writing is disabled by 'write' option"));
EXTERN char e_autocommands_unexpectedly_deleted_new_buffer_str[]
	INIT(= N_("E143: Autocommands unexpectedly deleted new buffer %s"));
EXTERN char e_non_numeric_argument_to_z[]
	INIT(= N_("E144: Non-numeric argument to :z"));
EXTERN char e_shell_commands_and_some_functionality_not_allowed_in_rvim[]
	INIT(= N_("E145: Shell commands and some functionality not allowed in rvim"));
EXTERN char e_regular_expressions_cant_be_delimited_by_letters[]
	INIT(= N_("E146: Regular expressions can't be delimited by letters"));
EXTERN char e_cannot_do_global_recursive_with_range[]
	INIT(= N_("E147: Cannot do :global recursive with a range"));
EXTERN char e_regular_expression_missing_from_global[]
	INIT(= N_("E148: Regular expression missing from :global"));
EXTERN char e_sorry_no_help_for_str[]
	INIT(= N_("E149: Sorry, no help for %s"));
EXTERN char e_not_a_directory_str[]
	INIT(= N_("E150: Not a directory: %s"));
EXTERN char e_no_match_str_1[]
	INIT(= N_("E151: No match: %s"));
EXTERN char e_cannot_open_str_for_writing_1[]
	INIT(= N_("E152: Cannot open %s for writing"));
EXTERN char e_unable_to_open_str_for_reading[]
	INIT(= N_("E153: Unable to open %s for reading"));
EXTERN char e_duplicate_tag_str_in_file_str_str[]
	INIT(= N_("E154: Duplicate tag \"%s\" in file %s/%s"));
#ifdef FEAT_SIGNS
EXTERN char e_unknown_sign_str[]
	INIT(= N_("E155: Unknown sign: %s"));
EXTERN char e_missing_sign_name[]
	INIT(= N_("E156: Missing sign name"));
EXTERN char e_invalid_sign_id_nr[]
	INIT(= N_("E157: Invalid sign ID: %d"));
#endif
#if defined(FEAT_SIGNS) || defined(FEAT_EVAL)
EXTERN char e_invalid_buffer_name_str[]
	INIT(= N_("E158: Invalid buffer name: %s"));
#endif
#ifdef FEAT_SIGNS
EXTERN char e_missing_sign_number[]
	INIT(= N_("E159: Missing sign number"));
EXTERN char e_unknown_sign_command_str[]
	INIT(= N_("E160: Unknown sign command: %s"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_breakpoint_not_found_str[]
	INIT(= N_("E161: Breakpoint not found: %s"));
#endif
EXTERN char e_no_write_since_last_change_for_buffer_str[]
	INIT(= N_("E162: No write since last change for buffer \"%s\""));
EXTERN char e_there_is_only_one_file_to_edit[]
	INIT(= N_("E163: There is only one file to edit"));
EXTERN char e_cannot_go_before_first_file[]
	INIT(= N_("E164: Cannot go before first file"));
EXTERN char e_cannot_go_beyond_last_file[]
	INIT(= N_("E165: Cannot go beyond last file"));
EXTERN char e_cant_open_linked_file_for_writing[]
	INIT(= N_("E166: Can't open linked file for writing"));
EXTERN char e_scriptencoding_used_outside_of_sourced_file[]
	INIT(= N_("E167: :scriptencoding used outside of a sourced file"));
#ifdef FEAT_EVAL
EXTERN char e_finish_used_outside_of_sourced_file[]
	INIT(= N_("E168: :finish used outside of a sourced file"));
#endif
EXTERN char e_command_too_recursive[]
	INIT(= N_("E169: Command too recursive"));
#ifdef FEAT_EVAL
EXTERN char e_missing_endwhile[]
	INIT(= N_("E170: Missing :endwhile"));
EXTERN char e_missing_endfor[]
	INIT(= N_("E170: Missing :endfor"));
EXTERN char e_missing_endif[]
	INIT(= N_("E171: Missing :endif"));
EXTERN char e_missing_marker[]
	INIT(= N_("E172: Missing marker"));
#endif

PLURAL_MSG(e_nr_more_file_to_edit, "E173: %d more file to edit",
		e_nr_more_files_to_edit, "E173: %d more files to edit")

EXTERN char e_command_already_exists_add_bang_to_replace_it_str[]
	INIT(= N_("E174: Command already exists: add ! to replace it: %s"));
EXTERN char e_no_attribute_specified[]
	INIT(= N_("E175: No attribute specified"));
EXTERN char e_invalid_number_of_arguments[]
	INIT(= N_("E176: Invalid number of arguments"));
EXTERN char e_count_cannot_be_specified_twice[]
	INIT(= N_("E177: Count cannot be specified twice"));
EXTERN char e_invalid_default_value_for_count[]
	INIT(= N_("E178: Invalid default value for count"));
EXTERN char e_argument_required_for_str[]
	INIT(= N_("E179: Argument required for %s"));
EXTERN char e_invalid_complete_value_str[]
	INIT(= N_("E180: Invalid complete value: %s"));
EXTERN char e_invalid_address_type_value_str[]
	INIT(= N_("E180: Invalid address type value: %s"));
EXTERN char e_invalid_attribute_str[]
	INIT(= N_("E181: Invalid attribute: %s"));
EXTERN char e_invalid_command_name[]
	INIT(= N_("E182: Invalid command name"));
EXTERN char e_user_defined_commands_must_start_with_an_uppercase_letter[]
	INIT(= N_("E183: User defined commands must start with an uppercase letter"));
EXTERN char e_no_such_user_defined_command_str[]
	INIT(= N_("E184: No such user-defined command: %s"));
EXTERN char e_cannot_find_color_scheme_str[]
	INIT(= N_("E185: Cannot find color scheme '%s'"));
EXTERN char e_no_previous_directory[]
	INIT(= N_("E186: No previous directory"));
EXTERN char e_directory_unknown[]
	INIT(= N_("E187: Directory unknown"));
EXTERN char e_obtaining_window_position_not_implemented_for_this_platform[]
	INIT(= N_("E188: Obtaining window position not implemented for this platform"));
EXTERN char e_str_exists_add_bang_to_override[]
	INIT(= N_("E189: \"%s\" exists (add ! to override)"));
EXTERN char e_cannot_open_str_for_writing_2[]
	INIT(= N_("E190: Cannot open \"%s\" for writing"));
EXTERN char e_argument_must_be_letter_or_forward_backward_quote[]
	INIT(= N_("E191: Argument must be a letter or forward/backward quote"));
EXTERN char e_recursive_use_of_normal_too_deep[]
	INIT(= N_("E192: Recursive use of :normal too deep"));
#ifdef FEAT_EVAL
EXTERN char e_str_not_inside_function[]
	INIT(= N_("E193: %s not inside a function"));
#endif
EXTERN char e_no_alternate_file_name_to_substitute_for_hash[]
	INIT(= N_("E194: No alternate file name to substitute for '#'"));
#ifdef FEAT_VIMINFO
EXTERN char e_cannot_open_viminfo_file_for_reading[]
	INIT(= N_("E195: Cannot open viminfo file for reading"));
#endif
#ifndef FEAT_DIGRAPHS
EXTERN char e_no_digraphs_version[]
	INIT(= N_("E196: No digraphs in this version"));
#endif
EXTERN char e_cannot_set_language_to_str[]
	INIT(= N_("E197: Cannot set language to \"%s\""));
// E198 unused
EXTERN char e_active_window_or_buffer_changed_or_deleted[]
	INIT(= N_("E199: Active window or buffer changed or deleted"));
EXTERN char e_readpre_autocommands_made_file_unreadable[]
	INIT(= N_("E200: *ReadPre autocommands made the file unreadable"));
EXTERN char e_readpre_autocommands_must_not_change_current_buffer[]
	INIT(= N_("E201: *ReadPre autocommands must not change current buffer"));
#ifdef FEAT_EVAL
EXTERN char e_conversion_mad_file_unreadable[]
	INIT(= N_("E202: Conversion made file unreadable!"));
#endif
EXTERN char e_autocommands_deleted_or_unloaded_buffer_to_be_written[]
	INIT(= N_("E203: Autocommands deleted or unloaded buffer to be written"));
EXTERN char e_autocommands_changed_number_of_lines_in_unexpected_way[]
	INIT(= N_("E204: Autocommand changed number of lines in unexpected way"));
EXTERN char e_patchmode_cant_save_original_file[]
	INIT(= N_("E205: Patchmode: can't save original file"));
EXTERN char e_patchmode_cant_touch_empty_original_file[]
	INIT(= N_("E206: Patchmode: can't touch empty original file"));
EXTERN char e_cant_delete_backup_file[]
	INIT(= N_("E207: Can't delete backup file"));
EXTERN char e_error_writing_to_str[]
	INIT(= N_("E208: Error writing to \"%s\""));
EXTERN char e_error_closing_str[]
	INIT(= N_("E209: Error closing \"%s\""));
EXTERN char e_error_reading_str[]
	INIT(= N_("E210: Error reading \"%s\""));
EXTERN char e_file_str_no_longer_available[]
	INIT(= N_("E211: File \"%s\" no longer available"));
EXTERN char e_cant_open_file_for_writing[]
	INIT(= N_("E212: Can't open file for writing"));
EXTERN char e_cannot_convert_add_bang_to_write_without_conversion[]
	INIT(= N_("E213: Cannot convert (add ! to write without conversion)"));
#ifdef FEAT_EVAL
EXTERN char e_cant_find_temp_file_for_writing[]
	INIT(= N_("E214: Can't find temp file for writing"));
#endif
EXTERN char e_illegal_character_after_star_str[]
	INIT(= N_("E215: Illegal character after *: %s"));
EXTERN char e_no_such_event_str[]
	INIT(= N_("E216: No such event: %s"));
EXTERN char e_no_such_group_or_event_str[]
	INIT(= N_("E216: No such group or event: %s"));
EXTERN char e_cant_execute_autocommands_for_all_events[]
	INIT(= N_("E217: Can't execute autocommands for ALL events"));
EXTERN char e_autocommand_nesting_too_deep[]
	INIT(= N_("E218: Autocommand nesting too deep"));
EXTERN char e_missing_open_curly[]
	INIT(= N_("E219: Missing {."));
EXTERN char e_missing_close_curly[]
	INIT(= N_("E220: Missing }."));
#ifdef FEAT_EVAL
EXTERN char e_marker_cannot_start_with_lower_case_letter[]
	INIT(= N_("E221: Marker cannot start with lower case letter"));
#endif
EXTERN char e_add_to_internal_buffer_that_was_already_read_from[]
	INIT(= "E222: Add to internal buffer that was already read from");
EXTERN char e_recursive_mapping[]
	INIT(= N_("E223: Recursive mapping"));
EXTERN char e_global_abbreviation_already_exists_for_str[]
	INIT(= N_("E224: Global abbreviation already exists for %s"));
EXTERN char e_global_mapping_already_exists_for_str[]
	INIT(= N_("E225: Global mapping already exists for %s"));
EXTERN char e_abbreviation_already_exists_for_str[]
	INIT(= N_("E226: Abbreviation already exists for %s"));
EXTERN char e_mapping_already_exists_for_str[]
	INIT(= N_("E227: Mapping already exists for %s"));
EXTERN char e_makemap_illegal_mode[]
	INIT(= "E228: makemap: Illegal mode");
#ifdef FEAT_GUI
EXTERN char e_cannot_start_the_GUI[]
	INIT(= N_("E229: Cannot start the GUI"));
EXTERN char e_cannot_read_from_str[]
	INIT(= N_("E230: Cannot read from \"%s\""));
EXTERN char e_guifontwide_invalid[]
	INIT(= N_("E231: 'guifontwide' invalid"));
#ifdef FEAT_BEVAL_GUI
EXTERN char e_cannot_create_ballooneval_with_both_message_and_callback[]
	INIT(= "E232: Cannot create BalloonEval with both message and callback");
#endif
# if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_X11)
EXTERN char e_cannot_open_display[]
	INIT(= N_("E233: Cannot open display"));
# endif
# if defined(FEAT_XFONTSET)
EXTERN char e_unknown_fontset_str[]
	INIT(= N_("E234: Unknown fontset: %s"));
# endif
# if defined(FEAT_GUI_X11) || defined(FEAT_GUI_GTK) \
	|| defined(FEAT_GUI_PHOTON) || defined(FEAT_GUI_MSWIN) || defined(FEAT_GUI_HAIKU)
EXTERN char e_unknown_font_str[]
	INIT(= N_("E235: Unknown font: %s"));
# endif
# if defined(FEAT_GUI_X11) && !defined(FEAT_GUI_GTK)
EXTERN char e_font_str_is_not_fixed_width[]
	INIT(= N_("E236: Font \"%s\" is not fixed-width"));
# endif
#endif
#ifdef MSWIN
EXTERN char e_printer_selection_failed[]
	INIT(= N_("E237: Printer selection failed"));
EXTERN char e_print_error_str[]
	INIT(= N_("E238: Print error: %s"));
#endif
#ifdef FEAT_SIGNS
EXTERN char e_invalid_sign_text_str[]
	INIT(= N_("E239: Invalid sign text: %s"));
#endif
#if defined(FEAT_CLIENTSERVER) && defined(FEAT_X11)
EXTERN char e_no_connection_to_x_server[]
	INIT(= N_("E240: No connection to the X server"));
#endif
#ifdef FEAT_CLIENTSERVER
EXTERN char e_unable_to_send_to_str[]
	INIT(= N_("E241: Unable to send to %s"));
#endif
EXTERN char e_cant_split_window_while_closing_another[]
	INIT(= N_("E242: Can't split a window while closing another"));
#if defined(FEAT_GUI_MSWIN) && !defined(FEAT_OLE)
EXTERN char e_argument_not_supported_str_use_ole_version[]
	INIT(= N_("E243: Argument not supported: \"-%s\"; Use the OLE version."));
#endif
#ifdef MSWIN
EXTERN char e_illegal_str_name_str_in_font_name_str[]
	INIT(= N_("E244: Illegal %s name \"%s\" in font name \"%s\""));
EXTERN char e_illegal_char_nr_in_font_name_str[]
	INIT(= N_("E245: Illegal char '%c' in font name \"%s\""));
#endif
EXTERN char e_filechangedshell_autocommand_deleted_buffer[]
	INIT(= N_("E246: FileChangedShell autocommand deleted buffer"));
#ifdef FEAT_CLIENTSERVER
EXTERN char e_no_registered_server_named_str[]
	INIT(= N_("E247: No registered server named \"%s\""));
EXTERN char e_failed_to_send_command_to_destination_program[]
	INIT(= N_("E248: Failed to send command to the destination program"));
#endif
EXTERN char e_window_layout_changed_unexpectedly[]
	INIT(= N_("E249: Window layout changed unexpectedly"));
#ifdef FEAT_XFONTSET
EXTERN char e_fonts_for_the_following_charsets_are_missing_in_fontset[]
	INIT(= N_("E250: Fonts for the following charsets are missing in fontset %s:"));
#endif
#ifdef FEAT_CLIENTSERVER
EXTERN char e_vim_instance_registry_property_is_badly_formed_deleted[]
	INIT(= N_("E251: VIM instance registry property is badly formed.  Deleted!"));
#endif
#ifdef FEAT_GUI_X11
EXTERN char e_fontsent_name_str_font_str_is_not_fixed_width[]
	INIT(= N_("E252: Fontset name: %s - Font '%s' is not fixed-width"));
EXTERN char e_fontset_name_str[]
	INIT(= N_("E253: Fontset name: %s"));
#endif
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
EXTERN char e_cannot_allocate_color_str[]
	INIT(= N_("E254: Cannot allocate color %s"));
#endif
#if defined(FEAT_SIGN_ICONS) && !defined(FEAT_GUI_GTK)
EXTERN char e_couldnt_read_in_sign_data[]
	INIT(= N_("E255: Couldn't read in sign data"));
#endif
// E256 unused
#ifdef FEAT_CSCOPE
EXTERN char e_cstag_tag_not_founc[]
	INIT(= N_("E257: cstag: Tag not found"));
#endif
#ifdef FEAT_CLIENTSERVER
EXTERN char e_unable_to_send_to_client[]
	INIT(= N_("E258: Unable to send to client"));
#endif
#ifdef FEAT_CSCOPE
EXTERN char e_no_matches_found_for_cscope_query_str_of_str[]
	INIT(= N_("E259: No matches found for cscope query %s of %s"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_missing_name_after_method[]
	INIT(= N_("E260: Missing name after ->"));
#endif
#ifdef FEAT_CSCOPE
EXTERN char e_cscope_connection_str_not_founc[]
	INIT(= N_("E261: Cscope connection %s not found"));
EXTERN char e_error_reading_cscope_connection_nr[]
	INIT(= N_("E262: Error reading cscope connection %d"));
#endif
#if defined(DYNAMIC_PYTHON) || defined(DYNAMIC_PYTHON3)
EXTERN char e_sorry_this_command_is_disabled_python_library_could_not_be_found[]
	INIT(= N_("E263: Sorry, this command is disabled, the Python library could not be loaded."));
#endif
#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
EXTERN char e_python_error_initialising_io_object[]
	INIT(= N_("E264: Python: Error initialising I/O objects"));
#endif
#ifdef FEAT_RUBY
EXTERN char e_dollar_must_be_an_instance_of_string[]
	INIT(= N_("E265: $_ must be an instance of String"));
#endif
#ifdef DYNAMIC_RUBY
EXTERN char e_sorry_this_command_is_disabled_the_ruby_library_could_not_be_loaded[]
	INIT(= N_("E266: Sorry, this command is disabled, the Ruby library could not be loaded."));
#endif
#ifdef FEAT_RUBY
EXTERN char e_unexpected_return[]
	INIT(= N_("E267: Unexpected return"));
EXTERN char e_unexpected_next[]
	INIT(= N_("E268: Unexpected next"));
EXTERN char e_unexpected_break[]
	INIT(= N_("E269: Unexpected break"));
EXTERN char e_unexpected_redo[]
	INIT(= N_("E270: Unexpected redo"));
EXTERN char e_retry_outside_of_rescue_clause[]
	INIT(= N_("E271: Retry outside of rescue clause"));
EXTERN char e_unhandled_exception[]
	INIT(= N_("E272: Unhandled exception"));
EXTERN char e_unknown_longjmp_status_nr[]
	INIT(= N_("E273: Unknown longjmp status %d"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_no_white_space_allowed_before_parenthesis[]
	INIT(= N_("E274: No white space allowed before parenthesis"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_cannot_add_text_property_to_unloaded_buffer[]
	INIT(= N_("E275: Cannot add text property to unloaded buffer"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_function_as_method_str[]
	INIT(= N_("E276: Cannot use function as a method: %s"));
#endif
#ifdef FEAT_CLIENTSERVER
EXTERN char e_unable_to_read_server_reply[]
	INIT(= N_("E277: Unable to read a server reply"));
#endif
// E278 unused
#if defined(FEAT_TERMINAL) && !defined(UNIX) && !defined(MSWIN)
EXTERN char e_sorry_plusplusshell_not_supported_on_this_system[]
	INIT(= N_("E279: Sorry, ++shell is not supported on this system"));
#endif
#ifdef FEAT_TCL
EXTERN char e_tcl_fatal_error_reflist_corrupt_please_report_this[]
	INIT(= "E280: TCL FATAL ERROR: reflist corrupt!? Please report this to vim-dev@vim.org");
#endif
// E281 unused
EXTERN char e_cannot_read_from_str_2[]
	INIT(= N_("E282: Cannot read from \"%s\""));
EXTERN char e_no_marks_matching_str[]
	INIT(= N_("E283: No marks matching \"%s\""));
#ifdef FEAT_XIM
# ifndef FEAT_GUI_GTK
EXTERN char e_cannot_set_ic_values[]
	INIT(= N_("E284: Cannot set IC values"));
# endif
# ifdef FEAT_GUI_X11
EXTERN char e_failed_to_create_input_context[]
	INIT(= N_("E285: Failed to create input context"));
EXTERN char e_failed_to_open_input_method[]
	INIT(= N_("E286: Failed to open input method"));
EXTERN char e_warning_could_not_set_destroy_callback_to_im[]
	INIT(= N_("E287: Warning: Could not set destroy callback to IM"));
EXTERN char e_input_method_doesnt_support_any_style[]
	INIT(= N_("E288: Input method doesn't support any style"));
EXTERN char e_input_method_doesnt_support_my_preedit_type[]
	INIT(= N_("E289: Input method doesn't support my preedit type"));
# endif
#endif
#ifdef FEAT_SEARCH_EXTRA
EXTERN char e_list_or_number_required[]
	INIT(= N_("E290: List or number required"));
#endif
// E291 unused
EXTERN char e_invalid_count_for_del_bytes_nr[]
	INIT(= "E292: Invalid count for del_bytes(): %ld");
EXTERN char e_block_was_not_locked[]
	INIT(= "E293: Block was not locked");
EXTERN char e_seek_error_in_swap_file_read[]
	INIT(= N_("E294: Seek error in swap file read"));
EXTERN char e_read_error_in_swap_file[]
	INIT(= N_("E295: Read error in swap file"));
EXTERN char e_seek_error_in_swap_file_write[]
	INIT(= N_("E296: Seek error in swap file write"));
EXTERN char e_write_error_in_swap_file[]
	INIT(= N_("E297: Write error in swap file"));
EXTERN char e_didnt_get_block_nr_zero[]
	INIT(= "E298: Didn't get block nr 0?");
EXTERN char e_didnt_get_block_nr_one[]
	INIT(= "E298: Didn't get block nr 1?");
EXTERN char e_didnt_get_block_nr_two[]
	INIT(= "E298: Didn't get block nr 2?");
#ifdef FEAT_PERL
EXTERN char e_perl_evaluation_forbidden_in_sandbox_without_safe_module[]
	INIT(= N_("E299: Perl evaluation forbidden in sandbox without the Safe module"));
#endif
EXTERN char e_swap_file_already_exists_symlink_attack[]
	INIT(= N_("E300: Swap file already exists (symlink attack?)"));
EXTERN char e_oops_lost_the_swap_file[]
	INIT(= N_("E301: Oops, lost the swap file!!!"));
EXTERN char e_could_not_rename_swap_file[]
	INIT(= N_("E302: Could not rename swap file"));
EXTERN char e_unable_to_open_swap_file_for_str_recovery_impossible[]
	INIT(= N_("E303: Unable to open swap file for \"%s\", recovery impossible"));
EXTERN char e_ml_upd_block0_didnt_get_block_zero[]
	INIT(= "E304: ml_upd_block0(): Didn't get block 0??");
EXTERN char e_no_swap_file_found_for_str[]
	INIT(= N_("E305: No swap file found for %s"));
EXTERN char e_cannot_open_str[]
	INIT(= N_("E306: Cannot open %s"));
EXTERN char e_str_does_not_look_like_vim_swap_file[]
	INIT(= N_("E307: %s does not look like a Vim swap file"));
EXTERN char e_warning_original_file_may_have_been_changed[]
	INIT(= N_("E308: Warning: Original file may have been changed"));
EXTERN char e_unable_to_read_block_one_from_str[]
	INIT(= N_("E309: Unable to read block 1 from %s"));
EXTERN char e_block_one_id_wrong_str_not_swp_file[]
	INIT(= N_("E310: Block 1 ID wrong (%s not a .swp file?)"));
EXTERN char e_recovery_interrupted[]
	INIT(= N_("E311: Recovery Interrupted"));
EXTERN char e_errors_detected_while_recovering_look_for_lines_starting_with_questions[]
	INIT(= N_("E312: Errors detected while recovering; look for lines starting with ???"));
EXTERN char e_cannot_preserve_there_is_no_swap_file[]
	INIT(= N_("E313: Cannot preserve, there is no swap file"));
EXTERN char e_preserve_failed[]
	INIT(= N_("E314: Preserve failed"));
EXTERN char e_ml_get_invalid_lnum_nr[]
	INIT(= "E315: ml_get: Invalid lnum: %ld");
EXTERN char e_ml_get_cannot_find_line_nr_in_buffer_nr_str[]
	INIT(= "E316: ml_get: Cannot find line %ld in buffer %d %s");
EXTERN char e_pointer_block_id_wrong[]
	INIT(= "E317: Pointer block id wrong");
EXTERN char e_pointer_block_id_wrong_two[]
	INIT(= "E317: Pointer block id wrong 2");
EXTERN char e_pointer_block_id_wrong_three[]
	INIT(= "E317: Pointer block id wrong 3");
EXTERN char e_pointer_block_id_wrong_four[]
	INIT(= "E317: Pointer block id wrong 4");
EXTERN char e_updated_too_many_blocks[]
	INIT(= "E318: Updated too many blocks?");
EXTERN char e_sorry_command_is_not_available_in_this_version[]
	INIT(= N_("E319: Sorry, the command is not available in this version"));
EXTERN char e_cannot_find_line_nr[]
	INIT(= "E320: Cannot find line %ld");
EXTERN char e_could_not_reload_str[]
	INIT(= N_("E321: Could not reload \"%s\""));
EXTERN char e_line_number_out_of_range_nr_past_the_end[]
	INIT(= "E322: Line number out of range: %ld past the end");
EXTERN char e_line_count_wrong_in_block_nr[]
	INIT(= "E323: Line count wrong in block %ld");
#ifdef FEAT_POSTSCRIPT
EXTERN char e_cant_open_postscript_output_file[]
	INIT(= N_("E324: Can't open PostScript output file"));
#endif
EXTERN char e_attention[]
	INIT(= N_("E325: ATTENTION"));
EXTERN char e_too_many_swap_files_found[]
	INIT(= N_("E326: Too many swap files found"));
#ifdef FEAT_MENU
EXTERN char_u e_part_of_menu_item_path_is_not_sub_menu[]
	INIT(= N_("E327: Part of menu-item path is not sub-menu"));
EXTERN char e_menu_only_exists_in_another_mode[]
	INIT(= N_("E328: Menu only exists in another mode"));
EXTERN char_u e_no_menu_str[]
	INIT(= N_("E329: No menu \"%s\""));
EXTERN char e_menu_path_must_not_lead_to_sub_menu[]
	INIT(= N_("E330: Menu path must not lead to a sub-menu"));
EXTERN char e_must_not_add_menu_items_directly_to_menu_bar[]
	INIT(= N_("E331: Must not add menu items directly to menu bar"));
EXTERN char e_separator_cannot_be_part_of_menu_path[]
	INIT(= N_("E332: Separator cannot be part of a menu path"));
EXTERN char e_menu_path_must_lead_to_menu_item[]
	INIT(= N_("E333: Menu path must lead to a menu item"));
EXTERN char e_menu_not_found_str[]
	INIT(= N_("E334: Menu not found: %s"));
EXTERN char e_menu_not_defined_for_str_mode[]
	INIT(= N_("E335: Menu not defined for %s mode"));
EXTERN char e_menu_path_must_lead_to_sub_menu[]
	INIT(= N_("E336: Menu path must lead to a sub-menu"));
EXTERN char e_menu_not_found_check_menu_names[]
	INIT(= N_("E337: Menu not found - check menu names"));
#endif
#ifdef FEAT_BROWSE
EXTERN char e_sorry_no_file_browser_in_console_mode[]
	INIT(= N_("E338: Sorry, no file browser in console mode"));
#endif
EXTERN char e_pattern_too_long[]
	INIT(= N_("E339: Pattern too long"));
EXTERN char e_internal_error_please_report_a_bug[]
	INIT(= N_("E340: Internal error; if you can reproduce please report a bug"));
EXTERN char e_internal_error_lalloc_zero[]
	INIT(= "E341: Internal error: lalloc(0, )");
EXTERN char e_out_of_memory_allocating_nr_bytes[]
	INIT(= N_("E342: Out of memory!  (allocating %lu bytes)"));
EXTERN char e_invalid_path_number_must_be_at_end_of_path_or_be_followed_by_str[]
	INIT(= N_("E343: Invalid path: '**[number]' must be at the end of the path or be followed by '%s'."));
EXTERN char e_cant_find_directory_str_in_cdpath[]
	INIT(= N_("E344: Can't find directory \"%s\" in cdpath"));
EXTERN char e_cant_find_file_str_in_path[]
	INIT(= N_("E345: Can't find file \"%s\" in path"));
EXTERN char e_no_more_directory_str_found_in_cdpath[]
	INIT(= N_("E346: No more directory \"%s\" found in cdpath"));
EXTERN char e_no_more_file_str_found_in_path[]
	INIT(= N_("E347: No more file \"%s\" found in path"));
EXTERN char e_no_string_under_cursor[]
	INIT(= N_("E348: No string under cursor"));
EXTERN char e_no_identifier_under_cursor[]
	INIT(= N_("E349: No identifier under cursor"));
#ifdef FEAT_FOLDING
EXTERN char e_cannot_create_fold_with_current_foldmethod[]
	INIT(= N_("E350: Cannot create fold with current 'foldmethod'"));
EXTERN char e_cannot_delete_fold_with_current_foldmethod[]
	INIT(= N_("E351: Cannot delete fold with current 'foldmethod'"));
EXTERN char e_cannot_erase_folds_with_current_foldmethod[]
	INIT(= N_("E352: Cannot erase folds with current 'foldmethod'"));
#endif
EXTERN char e_nothing_in_register_str[]
	INIT(= N_("E353: Nothing in register %s"));
EXTERN char e_invalid_register_name_str[]
	INIT(= N_("E354: Invalid register name: '%s'"));
EXTERN char e_unknown_option_str_2[]
	INIT(= N_("E355: Unknown option: %s"));
EXTERN char e_get_varp_error[]
	INIT(= "E356: get_varp ERROR");
#ifdef FEAT_LANGMAP
EXTERN char e_langmap_matching_character_missing_for_str[]
	INIT(= N_("E357: 'langmap': Matching character missing for %s"));
EXTERN char e_langmap_extra_characters_after_semicolon_str[]
	INIT(= N_("E358: 'langmap': Extra characters after semicolon: %s"));
#endif
#if defined(AMIGA) || defined(MACOS_X) || defined(MSWIN)  \
	|| defined(UNIX) || defined(VMS)
EXTERN char e_screen_mode_setting_not_supported[]
	INIT(= N_("E359: Screen mode setting not supported"));
#endif
#ifdef AMIGA
EXTERN char e_cannot_execute_shell_with_f_option[]
	INIT(= N_("E360: Cannot execute shell with -f option"));
#endif
// E361 unused
#if defined(FEAT_EVAL)
EXTERN char e_using_boolean_value_as_float[]
	INIT(= N_("E362: Using a boolean value as a Float"));
#endif
EXTERN char e_pattern_uses_more_memory_than_maxmempattern[]
	INIT(= N_("E363: Pattern uses more memory than 'maxmempattern'"));
#ifdef FEAT_LIBCALL
EXTERN char e_library_call_failed_for_str[]
	INIT(= N_("E364: Library call failed for \"%s()\""));
#endif
#ifdef FEAT_POSTSCRIPT
EXTERN char e_failed_to_print_postscript_file[]
	INIT(= N_("E365: Failed to print PostScript file"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_not_allowed_to_enter_popup_window[]
	INIT(= N_("E366: Not allowed to enter a popup window"));
#endif
EXTERN char e_no_such_group_str[]
	INIT(= N_("E367: No such group: \"%s\""));
#ifdef FEAT_LIBCALL
EXTERN char e_got_sig_str_in_libcall[]
	INIT(= N_("E368: Got SIG%s in libcall()"));
#endif
EXTERN char e_invalid_item_in_str_brackets[]
	INIT(= N_("E369: Invalid item in %s%%[]"));
#ifdef USING_LOAD_LIBRARY
EXTERN char e_could_not_load_library_str_str[]
	INIT(= N_("E370: Could not load library %s: %s"));
#endif
#ifdef FEAT_GUI_MSWIN
EXTERN char e_command_not_found[]
	INIT(= N_("E371: Command not found"));
#endif
#ifdef FEAT_QUICKFIX
EXTERN char e_too_many_chr_in_format_string[]
	INIT(= N_("E372: Too many %%%c in format string"));
EXTERN char e_unexpected_chr_in_format_str[]
	INIT(= N_("E373: Unexpected %%%c in format string"));
EXTERN char e_missing_rsb_in_format_string[]
	INIT(= N_("E374: Missing ] in format string"));
EXTERN char e_unsupported_chr_in_format_string[]
	INIT(= N_("E375: Unsupported %%%c in format string"));
EXTERN char e_invalid_chr_in_format_string_prefix[]
	INIT(= N_("E376: Invalid %%%c in format string prefix"));
EXTERN char e_invalid_chr_in_format_string[]
	INIT(= N_("E377: Invalid %%%c in format string"));
EXTERN char e_errorformat_contains_no_pattern[]
	INIT(= N_("E378: 'errorformat' contains no pattern"));
EXTERN char e_missing_or_empty_directory_name[]
	INIT(= N_("E379: Missing or empty directory name"));
EXTERN char e_at_bottom_of_quickfix_stack[]
	INIT(= N_("E380: At bottom of quickfix stack"));
EXTERN char e_at_top_of_quickfix_stack[]
	INIT(= N_("E381: At top of quickfix stack"));
#endif
EXTERN char e_cannot_write_buftype_option_is_set[]
	INIT(= N_("E382: Cannot write, 'buftype' option is set"));
EXTERN char e_invalid_search_string_str[]
	INIT(= N_("E383: Invalid search string: %s"));
EXTERN char e_search_hit_top_without_match_for_str[]
	INIT(= N_("E384: Search hit TOP without match for: %s"));
EXTERN char e_search_hit_bottom_without_match_for_str[]
	INIT(= N_("E385: Search hit BOTTOM without match for: %s"));
EXTERN char e_expected_question_or_slash_after_semicolon[]
	INIT(= N_("E386: Expected '?' or '/'  after ';'"));
#ifdef FEAT_FIND_ID
EXTERN char e_match_is_on_current_line[]
	INIT(= N_("E387: Match is on current line"));
EXTERN char e_couldnt_find_definition[]
	INIT(= N_("E388: Couldn't find definition"));
EXTERN char e_couldnt_find_pattern[]
	INIT(= N_("E389: Couldn't find pattern"));
#endif
#ifdef FEAT_SYN_HL
EXTERN char e_illegal_argument_str_2[]
	INIT(= N_("E390: Illegal argument: %s"));
EXTERN char e_no_such_syntax_cluster_str_1[]
	INIT(= N_("E391: No such syntax cluster: %s"));
EXTERN char e_no_such_syntax_cluster_str_2[]
	INIT(= N_("E392: No such syntax cluster: %s"));
EXTERN char e_groupthere_not_accepted_here[]
	INIT(= N_("E393: group[t]here not accepted here"));
EXTERN char e_didnt_find_region_item_for_str[]
	INIT(= N_("E394: Didn't find region item for %s"));
EXTERN char e_contains_argument_not_accepted_here[]
	INIT(= N_("E395: Contains argument not accepted here"));
// E396 unused
EXTERN char e_filename_required[]
	INIT(= N_("E397: Filename required"));
EXTERN char e_missing_equal_str[]
	INIT(= N_("E398: Missing '=': %s"));
EXTERN char e_not_enough_arguments_syntax_region_str[]
	INIT(= N_("E399: Not enough arguments: syntax region %s"));
EXTERN char e_no_cluster_specified[]
	INIT(= N_("E400: No cluster specified"));
EXTERN char e_pattern_delimiter_not_found_str[]
	INIT(= N_("E401: Pattern delimiter not found: %s"));
EXTERN char e_garbage_after_pattern_str[]
	INIT(= N_("E402: Garbage after pattern: %s"));
EXTERN char e_syntax_sync_line_continuations_pattern_specified_twice[]
	INIT(= N_("E403: syntax sync: Line continuations pattern specified twice"));
EXTERN char e_illegal_arguments_str[]
	INIT(= N_("E404: Illegal arguments: %s"));
EXTERN char e_missing_equal_sign_str[]
	INIT(= N_("E405: Missing equal sign: %s"));
EXTERN char e_empty_argument_str[]
	INIT(= N_("E406: Empty argument: %s"));
EXTERN char e_str_not_allowed_here[]
	INIT(= N_("E407: %s not allowed here"));
EXTERN char e_str_must_be_first_in_contains_list[]
	INIT(= N_("E408: %s must be first in contains list"));
EXTERN char e_unknown_group_name_str[]
	INIT(= N_("E409: Unknown group name: %s"));
EXTERN char e_invalid_syntax_subcommand_str[]
	INIT(= N_("E410: Invalid :syntax subcommand: %s"));
#endif
EXTERN char e_highlight_group_name_not_found_str[]
	INIT(= N_("E411: Highlight group not found: %s"));
EXTERN char e_not_enough_arguments_highlight_link_str[]
	INIT(= N_("E412: Not enough arguments: \":highlight link %s\""));
EXTERN char e_too_many_arguments_highlight_link_str[]
	INIT(= N_("E413: Too many arguments: \":highlight link %s\""));
EXTERN char e_group_has_settings_highlight_link_ignored[]
	INIT(= N_("E414: Group has settings, highlight link ignored"));
EXTERN char e_unexpected_equal_sign_str[]
	INIT(= N_("E415: Unexpected equal sign: %s"));
EXTERN char e_missing_equal_sign_str_2[]
	INIT(= N_("E416: Missing equal sign: %s"));
EXTERN char e_missing_argument_str[]
	INIT(= N_("E417: Missing argument: %s"));
EXTERN char e_illegal_value_str[]
	INIT(= N_("E418: Illegal value: %s"));
EXTERN char e_im_a_teapot[]
	INIT(= N_("E418: I'm a teapot"));
EXTERN char e_fg_color_unknown[]
	INIT(= N_("E419: FG color unknown"));
EXTERN char e_bg_color_unknown[]
	INIT(= N_("E420: BG color unknown"));
EXTERN char e_color_name_or_number_not_recognized_str[]
	INIT(= N_("E421: Color name or number not recognized: %s"));
EXTERN char e_terminal_code_too_long_str[]
	INIT(= N_("E422: Terminal code too long: %s"));
EXTERN char e_illegal_argument_str_3[]
	INIT(= N_("E423: Illegal argument: %s"));
EXTERN char e_too_many_different_highlighting_attributes_in_use[]
	INIT(= N_("E424: Too many different highlighting attributes in use"));
EXTERN char e_cannot_go_before_first_matching_tag[]
	INIT(= N_("E425: Cannot go before first matching tag"));
EXTERN char e_tag_not_found_str[]
	INIT(= N_("E426: Tag not found: %s"));
EXTERN char e_there_is_only_one_matching_tag[]
	INIT(= N_("E427: There is only one matching tag"));
EXTERN char e_cannot_go_beyond_last_matching_tag[]
	INIT(= N_("E428: Cannot go beyond last matching tag"));
EXTERN char e_file_str_does_not_exist[]
	INIT(= N_("E429: File \"%s\" does not exist"));
#ifdef FEAT_EMACS_TAGS
EXTERN char e_tag_file_path_truncated_for_str[]
	INIT(= N_("E430: Tag file path truncated for %s\n"));
#endif
EXTERN char e_format_error_in_tags_file_str[]
	INIT(= N_("E431: Format error in tags file \"%s\""));
EXTERN char e_tags_file_not_sorted_str[]
	INIT(= N_("E432: Tags file not sorted: %s"));
EXTERN char e_no_tags_file[]
	INIT(= N_("E433: No tags file"));
EXTERN char e_cannot_find_tag_pattern[]
	INIT(= N_("E434: Can't find tag pattern"));
EXTERN char e_couldnt_find_tag_just_guessing[]
	INIT(= N_("E435: Couldn't find tag, just guessing!"));
EXTERN char e_no_str_entry_in_termcap[]
	INIT(= N_("E436: No \"%s\" entry in termcap"));
EXTERN char e_terminal_capability_cm_required[]
	INIT(= N_("E437: Terminal capability \"cm\" required"));
EXTERN char e_u_undo_line_numbers_wrong[]
	INIT(= "E438: u_undo: Line numbers wrong");
EXTERN char e_undo_list_corrupt[]
	INIT(= "E439: Undo list corrupt");
EXTERN char e_undo_line_missing[]
	INIT(= "E440: Undo line missing");
#ifdef FEAT_QUICKFIX
EXTERN char e_there_is_no_preview_window[]
	INIT(= N_("E441: There is no preview window"));
#endif
EXTERN char e_cant_split_topleft_and_botright_at_the_same_time[]
	INIT(= N_("E442: Can't split topleft and botright at the same time"));
EXTERN char e_cannot_rotate_when_another_window_is_split[]
	INIT(= N_("E443: Cannot rotate when another window is split"));
EXTERN char e_cannot_close_last_window[]
	INIT(= N_("E444: Cannot close last window"));
EXTERN char e_other_window_contains_changes[]
	INIT(= N_("E445: Other window contains changes"));
EXTERN char e_no_file_name_under_cursor[]
	INIT(= N_("E446: No file name under cursor"));
EXTERN char e_cant_find_file_str_in_path_2[]
	INIT(= N_("E447: Can't find file \"%s\" in path"));
#ifdef USING_LOAD_LIBRARY
EXTERN char e_could_not_load_library_function_str[]
	INIT(= N_("E448: Could not load library function %s"));
#endif
#ifdef FEAT_CLIENTSERVER
EXTERN char e_invalid_expression_received[]
	INIT(= N_("E449: Invalid expression received"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_buffer_number_text_or_list_required[]
	INIT(= N_("E450: Buffer number, text or a list required"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_expected_right_curly_str[]
	INIT(= N_("E451: Expected }: %s"));
EXTERN char e_double_semicolon_in_list_of_variables[]
	INIT(= N_("E452: Double ; in list of variables"));
#endif
EXTERN char e_ul_color_unknown[]
	INIT(= N_("E453: UL color unknown"));
#ifdef FEAT_EVAL
EXTERN char e_function_list_was_modified[]
	INIT(= N_("E454: Function list was modified"));
#endif
#ifdef FEAT_POSTSCRIPT
EXTERN char e_error_writing_to_postscript_output_file[]
	INIT(= N_("E455: Error writing to PostScript output file"));
EXTERN char e_cant_open_file_str_2[]
	INIT(= N_("E456: Can't open file \"%s\""));
EXTERN char e_cant_find_postscript_resource_file_str_ps[]
	INIT(= N_("E456: Can't find PostScript resource file \"%s.ps\""));
EXTERN char e_cant_read_postscript_resource_file_str[]
	INIT(= N_("E457: Can't read PostScript resource file \"%s\""));
#endif
#ifdef FEAT_GUI_X11
EXTERN char e_cannot_allocate_colormap_entry_some_colors_may_be_incorrect[]
	INIT(= N_("E458: Cannot allocate colormap entry, some colors may be incorrect"));
#endif
#if defined(UNIX) || defined(FEAT_SESSION)
EXTERN char e_cannot_go_back_to_previous_directory[]
	INIT(= N_("E459: Cannot go back to previous directory"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_entries_missing_in_mapset_dict_argument[]
	INIT(= N_("E460: Entries missing in mapset() dict argument"));
EXTERN char e_illegal_variable_name_str[]
	INIT(= N_("E461: Illegal variable name: %s"));
#endif
EXTERN char e_could_not_prepare_for_reloading_str[]
	INIT(= N_("E462: Could not prepare for reloading \"%s\""));
#ifdef FEAT_NETBEANS_INTG
EXTERN char e_region_is_guarded_cannot_modify[]
	INIT(= N_("E463: Region is guarded, cannot modify"));
#endif
EXTERN char e_ambiguous_use_of_user_defined_command[]
	INIT(= N_("E464: Ambiguous use of user-defined command"));
#ifdef FEAT_EVAL
EXTERN char e_ambiguous_use_of_user_defined_command_str[]
	INIT(= N_("E464: Ambiguous use of user-defined command: %s"));
#endif
EXTERN char e_winsize_requires_two_number_arguments[]
	INIT(= N_("E465: :winsize requires two number arguments"));
EXTERN char e_winpos_requires_two_number_arguments[]
	INIT(= N_("E466: :winpos requires two number arguments"));
#ifdef FEAT_EVAL
EXTERN char e_custom_completion_requires_function_argument[]
	INIT(= N_("E467: Custom completion requires a function argument"));
#endif
EXTERN char e_completion_argument_only_allowed_for_custom_completion[]
	INIT(= N_("E468: Completion argument only allowed for custom completion"));
#ifdef FEAT_CSCOPE
EXTERN char e_invalid_cscopequickfix_flag_chr_for_chr[]
	INIT(= N_("E469: Invalid cscopequickfix flag %c for %c"));
#endif
EXTERN char e_command_aborted[]
	INIT(= N_("E470: Command aborted"));
EXTERN char e_argument_required[]
	INIT(= N_("E471: Argument required"));
EXTERN char e_command_failed[]
	INIT(= N_("E472: Command failed"));
EXTERN char e_internal_error_in_regexp[]
	INIT(= "E473: Internal error in regexp");
EXTERN char e_invalid_argument[]
	INIT(= N_("E474: Invalid argument"));
EXTERN char e_invalid_argument_str[]
	INIT(= N_("E475: Invalid argument: %s"));
EXTERN char e_invalid_value_for_argument_str[]
	INIT(= N_("E475: Invalid value for argument %s"));
#if defined(FEAT_JOB_CHANNEL) || defined(FEAT_PROP_POPUP) || defined(FEAT_EVAL)
EXTERN char e_invalid_value_for_argument_str_str[]
	INIT(= N_("E475: Invalid value for argument %s: %s"));
#endif
EXTERN char e_invalid_command[]
	INIT(= N_("E476: Invalid command"));
EXTERN char e_invalid_command_str[]
	INIT(= N_("E476: Invalid command: %s"));
#ifdef FEAT_EVAL
EXTERN char e_invalid_command_str_expected_str[]
	INIT(= N_("E476: Invalid command: %s, expected %s"));
#endif
EXTERN char e_no_bang_allowed[]
	INIT(= N_("E477: No ! allowed"));
EXTERN char e_dont_panic[]
	INIT(= N_("E478: Don't panic!"));
EXTERN char e_no_match[]
	INIT(= N_("E479: No match"));
EXTERN char e_no_match_str_2[]
	INIT(= N_("E480: No match: %s"));
EXTERN char e_no_range_allowed[]
	INIT(= N_("E481: No range allowed"));
EXTERN char e_cant_create_file_str[]
	INIT(= N_("E482: Can't create file %s"));
EXTERN char e_cant_get_temp_file_name[]
	INIT(= N_("E483: Can't get temp file name"));
EXTERN char e_cant_open_file_str[]
	INIT(= N_("E484: Can't open file %s"));
EXTERN char e_cant_read_file_str[]
	INIT(= N_("E485: Can't read file %s"));
EXTERN char e_pattern_not_found[]
	INIT(= N_("E486: Pattern not found"));
EXTERN char e_pattern_not_found_str[]
	INIT(= N_("E486: Pattern not found: %s"));
EXTERN char e_argument_must_be_positive[]
	INIT(= N_("E487: Argument must be positive"));
#ifdef FEAT_PROP_POPUP
EXTERN char e_argument_must_be_positive_str[]
	INIT(= N_("E487: Argument must be positive: %s"));
#endif
EXTERN char e_trailing_characters[]
	INIT(= N_("E488: Trailing characters"));
EXTERN char e_trailing_characters_str[]
	INIT(= N_("E488: Trailing characters: %s"));
EXTERN char e_no_call_stack_to_substitute_for_stack[]
	INIT(= N_("E489: No call stack to substitute for \"<stack>\""));
#ifdef FEAT_FOLDING
EXTERN char e_no_fold_found[]
	INIT(= N_("E490: No fold found"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_json_decode_error_at_str[]
	INIT(= N_("E491: JSON decode error at '%s'"));
#endif
EXTERN char e_not_an_editor_command[]
	INIT(= N_("E492: Not an editor command"));
EXTERN char e_backwards_range_given[]
	INIT(= N_("E493: Backwards range given"));
EXTERN char e_use_w_or_w_gt_gt[]
	INIT(= N_("E494: Use w or w>>"));
EXTERN char e_no_autocommand_file_name_to_substitute_for_afile[]
	INIT(= N_("E495: No autocommand file name to substitute for \"<afile>\""));
EXTERN char e_no_autocommand_buffer_number_to_substitute_for_abuf[]
	INIT(= N_("E496: No autocommand buffer number to substitute for \"<abuf>\""));
EXTERN char e_no_autocommand_match_name_to_substitute_for_amatch[]
	INIT(= N_("E497: No autocommand match name to substitute for \"<amatch>\""));
EXTERN char e_no_source_file_name_to_substitute_for_sfile[]
	INIT(= N_("E498: No :source file name to substitute for \"<sfile>\""));
EXTERN char e_empty_file_name_for_percent_or_hash_only_works_with_ph[]
	// xgettext:no-c-format
	INIT(= N_("E499: Empty file name for '%' or '#', only works with \":p:h\""));
EXTERN char e_evaluates_to_an_empty_string[]
	INIT(= N_("E500: Evaluates to an empty string"));
EXTERN char e_at_end_of_file[]
	INIT(= N_("E501: At end-of-file"));
	// E502
EXTERN char e_is_a_directory[]
	INIT(= N_("is a directory"));
	// E503
EXTERN char e_is_not_file_or_writable_device[]
	INIT(= N_("is not a file or writable device"));
EXTERN char e_str_is_not_file_or_writable_device[]
	INIT(= N_("E503: \"%s\" is not a file or writable device"));
EXTERN char e_coffee_currently_not_available[]
	INIT(= N_("E503: Coffee is currently not available"));
	// E504
EXTERN char e_is_read_only_cannot_override_W_in_cpoptions[]
	INIT(= N_("is read-only (cannot override: \"W\" in 'cpoptions')"));
	// E505
EXTERN char e_is_read_only_add_bang_to_override[]
	INIT(= N_("is read-only (add ! to override)"));
EXTERN char e_str_is_read_only_add_bang_to_override[]
	INIT(= N_("E505: \"%s\" is read-only (add ! to override)"));
EXTERN char e_cant_write_to_backup_file_add_bang_to_override[]
	INIT(= N_("E506: Can't write to backup file (add ! to override)"));
EXTERN char e_close_error_for_backup_file_add_bang_to_write_anyway[]
	INIT(= N_("E507: Close error for backup file (add ! to write anyway)"));
EXTERN char e_cant_read_file_for_backup_add_bang_to_write_anyway[]
	INIT(= N_("E508: Can't read file for backup (add ! to write anyway)"));
EXTERN char e_cannot_create_backup_file_add_bang_to_write_anyway[]
	INIT(= N_("E509: Cannot create backup file (add ! to override)"));
EXTERN char e_cant_make_backup_file_add_bang_to_write_anyway[]
	INIT(= N_("E510: Can't make backup file (add ! to write anyway)"));
#ifdef FEAT_NETBEANS_INTG
EXTERN char e_netbeans_already_connected[]
	INIT(= N_("E511: NetBeans already connected"));
#endif
EXTERN char e_close_failed[]
	INIT(= N_("E512: Close failed"));
EXTERN char e_write_error_conversion_failed_make_fenc_empty_to_override[]
	INIT(= N_("E513: Write error, conversion failed (make 'fenc' empty to override)"));
EXTERN char e_write_error_conversion_failed_in_line_nr_make_fenc_empty_to_override[]
	INIT(= N_("E513: Write error, conversion failed in line %ld (make 'fenc' empty to override)"));
EXTERN char e_write_error_file_system_full[]
	INIT(= N_("E514: Write error (file system full?)"));
EXTERN char e_no_buffers_were_unloaded[]
	INIT(= N_("E515: No buffers were unloaded"));
EXTERN char e_no_buffers_were_deleted[]
	INIT(= N_("E516: No buffers were deleted"));
EXTERN char e_no_buffers_were_wiped_out[]
	INIT(= N_("E517: No buffers were wiped out"));
EXTERN char e_unknown_option[]
	INIT(= N_("E518: Unknown option"));
EXTERN char e_option_not_supported[]
	INIT(= N_("E519: Option not supported"));
EXTERN char e_not_allowed_in_modeline[]
	INIT(= N_("E520: Not allowed in a modeline"));
EXTERN char e_number_required_after_equal[]
	INIT(= N_("E521: Number required after ="));
EXTERN char e_number_required_after_str_equal_str[]
	INIT(= N_("E521: Number required: &%s = '%s'"));
EXTERN char e_not_found_in_termcap[]
	INIT(= N_("E522: Not found in termcap"));
EXTERN char e_not_allowed_here[]
	INIT(= N_("E523: Not allowed here"));
EXTERN char e_missing_colon[]
	INIT(= N_("E524: Missing colon"));
EXTERN char e_zero_length_string[]
	INIT(= N_("E525: Zero length string"));
#ifdef FEAT_VIMINFO
EXTERN char e_missing_number_after_angle_str_angle[]
	INIT(= N_("E526: Missing number after <%s>"));
EXTERN char e_missing_comma[]
	INIT(= N_("E527: Missing comma"));
EXTERN char e_must_specify_a_value[]
	INIT(= N_("E528: Must specify a ' value"));
#endif
EXTERN char e_cannot_set_term_to_empty_string[]
	INIT(= N_("E529: Cannot set 'term' to empty string"));
#ifdef FEAT_GUI
EXTERN char e_cannot_change_term_in_GUI[]
	INIT(= N_("E530: Cannot change 'term' in the GUI"));
EXTERN char e_use_gui_to_start_GUI[]
	INIT(= N_("E531: Use \":gui\" to start the GUI"));
#endif
#ifdef FEAT_NETBEANS_INTG
EXTERN char e_highlighting_color_name_too_long_in_defineAnnoType[]
	INIT(= N_("E532: Highlighting color name too long in defineAnnoType"));
#endif
#ifdef FEAT_GUI
EXTERN char e_cant_select_wide_font[]
	INIT(= N_("E533: Can't select wide font"));
EXTERN char e_invalid_wide_font[]
	INIT(= N_("E534: Invalid wide font"));
#endif
EXTERN char e_illegal_character_after_chr[]
	INIT(= N_("E535: Illegal character after <%c>"));
#ifdef FEAT_FOLDING
EXTERN char e_comma_required[]
	INIT(= N_("E536: Comma required"));
EXTERN char e_commentstring_must_be_empty_or_contain_str[]
	INIT(= N_("E537: 'commentstring' must be empty or contain %s"));
#endif
EXTERN char e_pattern_found_in_every_line_str[]
	INIT(= N_("E538: Pattern found in every line: %s"));
EXTERN char e_illegal_character_str[]
	INIT(= N_("E539: Illegal character <%s>"));
#ifdef FEAT_STL_OPT
EXTERN char e_unclosed_expression_sequence[]
	INIT(= N_("E540: Unclosed expression sequence"));
// E541 unused
EXTERN char e_unbalanced_groups[]
	INIT(= N_("E542: Unbalanced groups"));
#endif
#ifdef MSWIN
EXTERN char e_not_valid_codepage[]
	INIT(= N_("E543: Not a valid codepage"));
#endif
#ifdef FEAT_KEYMAP
EXTERN char e_keymap_file_not_found[]
	INIT(= N_("E544: Keymap file not found"));
#endif
#ifdef CURSOR_SHAPE
EXTERN char e_missing_colon_2[]
	INIT(= N_("E545: Missing colon"));
EXTERN char e_illegal_mode[]
	INIT(= N_("E546: Illegal mode"));
#endif
#ifdef FEAT_MOUSESHAPE
EXTERN char e_illegal_mouseshape[]
	INIT(= N_("E547: Illegal mouseshape"));
#endif
#ifdef CURSOR_SHAPE
EXTERN char e_digit_expected[]
	INIT(= N_("E548: Digit expected"));
EXTERN char e_illegal_percentage[]
	INIT(= N_("E549: Illegal percentage"));
#endif
#ifdef FEAT_PRINTER
EXTERN char e_missing_colon_3[]
	INIT(= N_("E550: Missing colon"));
EXTERN char e_illegal_component[]
	INIT(= N_("E551: Illegal component"));
EXTERN char e_digit_expected_2[]
	INIT(= N_("E552: Digit expected"));
#endif
#ifdef FEAT_QUICKFIX
EXTERN char e_no_more_items[]
	INIT(= N_("E553: No more items"));
#endif
EXTERN char e_syntax_error_in_str_curlies[]
	INIT(= N_("E554: Syntax error in %s{...}"));
EXTERN char e_at_bottom_of_tag_stack[]
	INIT(= N_("E555: At bottom of tag stack"));
EXTERN char e_at_top_of_tag_stack[]
	INIT(= N_("E556: At top of tag stack"));
EXTERN char e_cannot_open_termcap_file[]
	INIT(= N_("E557: Cannot open termcap file"));
EXTERN char e_terminal_entry_not_found_in_terminfo[]
	INIT(= N_("E558: Terminal entry not found in terminfo"));
#if defined(HAVE_TGETENT) && !defined(TERMINFO)
EXTERN char e_terminal_entry_not_found_in_termcap[]
	INIT(= N_("E559: Terminal entry not found in termcap"));
#endif
#ifdef FEAT_CSCOPE
EXTERN char e_usage_cscope_str[]
	INIT(= N_("E560: Usage: cs[cope] %s"));
EXTERN char e_unknown_cscope_search_type[]
	INIT(= N_("E561: Unknown cscope search type"));
EXTERN char e_usage_cstag_ident[]
	INIT(= N_("E562: Usage: cstag <ident>"));
EXTERN char e_stat_str_error_nr[]
	INIT(= N_("E563: stat(%s) error: %d"));
EXTERN char e_str_is_not_directory_or_valid_cscope_database[]
	INIT(= N_("E564: %s is not a directory or a valid cscope database"));
#endif
EXTERN char e_not_allowed_to_change_text_or_change_window[]
	INIT(= N_("E565: Not allowed to change text or change window"));
#ifdef FEAT_CSCOPE
EXTERN char e_could_not_create_cscope_pipes[]
	INIT(= N_("E566: Could not create cscope pipes"));
EXTERN char e_no_cscope_connections[]
	INIT(= N_("E567: No cscope connections"));
EXTERN char e_duplicate_cscope_database_not_added[]
	INIT(= N_("E568: Duplicate cscope database not added"));
// E569 unused
EXTERN char e_fatal_error_in_cs_manage_matches[]
	INIT(= "E570: Fatal error in cs_manage_matches");
#endif
#ifdef DYNAMIC_TCL
EXTERN char e_sorry_this_command_is_disabled_tcl_library_could_not_be_loaded[]
	INIT(= N_("E571: Sorry, this command is disabled: the Tcl library could not be loaded."));
#endif
#ifdef FEAT_TCL
EXTERN char e_exit_code_nr[]
	INIT(= N_("E572: Exit code %d"));
#endif
#ifdef FEAT_CLIENTSERVER
EXTERN char e_invalid_server_id_used_str[]
	INIT(= N_("E573: Invalid server id used: %s"));
#endif
#ifdef FEAT_VIMINFO
EXTERN char e_unknown_register_type_nr[]
	INIT(= N_("E574: Unknown register type %d"));
	// E575
EXTERN char e_illegal_starting_char[]
	INIT(= N_("Illegal starting char"));
	// E576
EXTERN char e_nonr_missing_gt[]
	INIT(= N_("Missing '>'"));
	// E577
EXTERN char e_illegal_register_name[]
	INIT(= N_("Illegal register name"));
#endif
// E578 unused
#ifdef FEAT_EVAL
EXTERN char e_if_nesting_too_deep[]
	INIT(= N_("E579: :if nesting too deep"));
EXTERN char e_block_nesting_too_deep[]
	INIT(= N_("E579: Block nesting too deep"));
EXTERN char e_endif_without_if[]
	INIT(= N_("E580: :endif without :if"));
EXTERN char e_else_without_if[]
	INIT(= N_("E581: :else without :if"));
EXTERN char e_elseif_without_if[]
	INIT(= N_("E582: :elseif without :if"));
EXTERN char e_multiple_else[]
	INIT(= N_("E583: Multiple :else"));
EXTERN char e_elseif_after_else[]
	INIT(= N_("E584: :elseif after :else"));
EXTERN char e_while_for_nesting_too_deep[]
	INIT(= N_("E585: :while/:for nesting too deep"));
EXTERN char e_continue_without_while_or_for[]
	INIT(= N_("E586: :continue without :while or :for"));
EXTERN char e_break_without_while_or_for[]
	INIT(= N_("E587: :break without :while or :for"));
EXTERN char e_endwhile_without_while[]
	INIT(= N_("E588: :endwhile without :while"));
EXTERN char e_endfor_without_for[]
	INIT(= N_("E588: :endfor without :for"));
#endif
EXTERN char e_backupext_and_patchmode_are_equal[]
	INIT(= N_("E589: 'backupext' and 'patchmode' are equal"));
#ifdef FEAT_QUICKFIX
EXTERN char e_preview_window_already_exists[]
	INIT(= N_("E590: A preview window already exists"));
#endif
EXTERN char e_winheight_cannot_be_smaller_than_winminheight[]
	INIT(= N_("E591: 'winheight' cannot be smaller than 'winminheight'"));
EXTERN char e_winwidth_cannot_be_smaller_than_winminwidth[]
	INIT(= N_("E592: 'winwidth' cannot be smaller than 'winminwidth'"));
EXTERN char e_need_at_least_nr_lines[]
	INIT(= N_("E593: Need at least %d lines"));
EXTERN char e_need_at_least_nr_columns[]
	INIT(= N_("E594: Need at least %d columns"));
#ifdef FEAT_LINEBREAK
EXTERN char e_showbreak_contains_unprintable_or_wide_character[]
	INIT(= N_("E595: 'showbreak' contains unprintable or wide character"));
#endif
#ifdef FEAT_GUI
EXTERN char e_invalid_fonts[]
	INIT(= N_("E596: Invalid font(s)"));
# ifdef FEAT_XFONTSET
EXTERN char e_cant_select_fontset[]
	INIT(= N_("E597: Can't select fontset"));
EXTERN char e_invalid_fontset[]
	INIT(= N_("E598: Invalid fontset"));
# endif
#endif
#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
EXTERN char e_value_of_imactivatekey_is_invalid[]
	INIT(= N_("E599: Value of 'imactivatekey' is invalid"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_missing_endtry[]
	INIT(= N_("E600: Missing :endtry"));
EXTERN char e_try_nesting_too_deep[]
	INIT(= N_("E601: :try nesting too deep"));
EXTERN char e_endtry_without_try[]
	INIT(= N_("E602: :endtry without :try"));
EXTERN char e_catch_without_try[]
	INIT(= N_("E603: :catch without :try"));
EXTERN char e_catch_after_finally[]
	INIT(= N_("E604: :catch after :finally"));
EXTERN char e_exception_not_caught_str[]
	INIT(= N_("E605: Exception not caught: %s"));
EXTERN char e_finally_without_try[]
	INIT(= N_("E606: :finally without :try"));
EXTERN char e_multiple_finally[]
	INIT(= N_("E607: Multiple :finally"));
EXTERN char e_cannot_throw_exceptions_with_vim_prefix[]
	INIT(= N_("E608: Cannot :throw exceptions with 'Vim' prefix"));
#endif
#ifdef FEAT_CSCOPE
EXTERN char e_cscope_error_str[]
	INIT(= N_("E609: Cscope error: %s"));
#endif
EXTERN char e_no_argument_to_delete[]
	INIT(= N_("E610: No argument to delete"));
#ifdef FEAT_EVAL
EXTERN char e_using_special_as_number[]
	INIT(= N_("E611: Using a Special as a Number"));
#endif
#ifdef FEAT_SIGNS
EXTERN char e_too_many_signs_defined[]
	INIT(= N_("E612: Too many signs defined"));
#endif
#if defined(MSWIN) && defined(FEAT_PRINTER)
EXTERN char e_unknown_printer_font_str[]
	INIT(= N_("E613: Unknown printer font: %s"));
#endif
EXTERN char e_class_required[]
	INIT(= N_("E614: Class required"));
// E615 unused
EXTERN char e_object_required_for_argument_nr[]
	INIT(= N_("E616: Object required for argument %d"));
#ifdef FEAT_GUI_GTK
EXTERN char e_cannot_be_changed_in_gtk_GUI[]
	INIT(= N_("E617: Cannot be changed in the GTK GUI"));
#endif
#ifdef FEAT_POSTSCRIPT
EXTERN char e_file_str_is_not_postscript_resource_file[]
	INIT(= N_("E618: File \"%s\" is not a PostScript resource file"));
EXTERN char e_file_str_is_not_supported_postscript_resource_file[]
	INIT(= N_("E619: File \"%s\" is not a supported PostScript resource file"));
EXTERN char e_unable_to_convert_to_print_encoding_str[]
	INIT(= N_("E620: Unable to convert to print encoding \"%s\""));
EXTERN char e_str_resource_file_has_wrong_version[]
	INIT(= N_("E621: \"%s\" resource file has wrong version"));
#endif
#ifdef FEAT_CSCOPE
EXTERN char e_could_not_fork_for_cscope[]
	INIT(= N_("E622: Could not fork for cscope"));
# ifndef UNIX
EXTERN char e_could_not_spawn_cscope_process[]
	INIT(= N_("E623: Could not spawn cscope process"));
# endif
#endif
#if defined(FEAT_PRINTER) && defined(FEAT_POSTSCRIPT)
EXTERN char e_cant_open_file_str_3[]
	INIT(= N_("E624: Can't open file \"%s\""));
#endif
#if defined(FEAT_CSCOPE) && !defined(UNIX)
EXTERN char e_cannot_open_cscope_database_str[]
	INIT(= N_("E625: Cannot open cscope database: %s"));
EXTERN char e_cannot_get_cscope_database_information[]
	INIT(= N_("E626: Cannot get cscope database information"));
#endif
#ifdef FEAT_NETBEANS_INTG
EXTERN char e_missing_colon_str[]
	INIT(= N_("E627: Missing colon: %s"));
EXTERN char e_missing_bang_or_slash_in_str[]
	INIT(= N_("E628: Missing ! or / in: %s"));
#ifdef NBDEBUG
EXTERN char e_bad_return_from_nb_do_cmd[]
	INIT(= "E629: Bad return from nb_do_cmd");
#endif
#endif
#ifdef FEAT_JOB_CHANNEL
EXTERN char e_str_write_while_not_connected[]
	INIT(= N_("E630: %s(): Write while not connected"));
EXTERN char e_str_write_failed[]
	INIT(= N_("E631: %s(): Write failed"));
#endif
#ifdef FEAT_NETBEANS_INTG
EXTERN char e_invalid_buffer_identifier_in_getlength[]
	INIT(= N_("E632: Invalid buffer identifier in getLength"));
EXTERN char e_invalid_buffer_identifier_in_gettext[]
	INIT(= N_("E633: Invalid buffer identifier in getText"));
EXTERN char e_invalid_buffer_identifier_in_remove[]
	INIT(= N_("E634: Invalid buffer identifier in remove"));
EXTERN char e_invalid_buffer_identifier_in_insert[]
	INIT(= N_("E635: Invalid buffer identifier in insert"));
EXTERN char e_invalid_buffer_identifier_in_create[]
	INIT(= N_("E636: Invalid buffer identifier in create"));
EXTERN char e_invalid_buffer_identifier_in_startdocumentlisten[]
	INIT(= N_("E637: Invalid buffer identifier in startDocumentListen"));
EXTERN char e_invalid_buffer_identifier_in_stopdocumentlisten[]
	INIT(= N_("E638: Invalid buffer identifier in stopDocumentListen"));
EXTERN char e_invalid_buffer_identifier_in_settitle[]
	INIT(= N_("E639: Invalid buffer identifier in setTitle"));
EXTERN char e_invalid_buffer_identifier_in_initdone[]
	INIT(= N_("E640: Invalid buffer identifier in initDone"));
EXTERN char e_invalid_buffer_identifier_in_setbuffernumber[]
	INIT(= N_("E641: Invalid buffer identifier in setBufferNumber"));
EXTERN char e_file_str_not_found_in_setbuffernumber[]
	INIT(= N_("E642: File %s not found in setBufferNumber"));
EXTERN char e_invalid_buffer_identifier_in_setfullname[]
	INIT(= N_("E643: Invalid buffer identifier in setFullName"));
EXTERN char e_invalid_buffer_identifier_in_editfile[]
	INIT(= N_("E644: Invalid buffer identifier in editFile"));
EXTERN char e_invalid_buffer_identifier_in_setvisible[]
	INIT(= N_("E645: Invalid buffer identifier in setVisible"));
EXTERN char e_invalid_buffer_identifier_in_setmodified[]
	INIT(= N_("E646: Invalid buffer identifier in setModified"));
EXTERN char e_invalid_buffer_identifier_in_setdot[]
	INIT(= N_("E647: Invalid buffer identifier in setDot"));
EXTERN char e_invalid_buffer_identifier_in_close[]
	INIT(= N_("E648: Invalid buffer identifier in close"));
// E649 unused
EXTERN char e_invalid_buffer_identifier_in_defineannotype[]
	INIT(= N_("E650: Invalid buffer identifier in defineAnnoType"));
EXTERN char e_invalid_buffer_identifier_in_addanno[]
	INIT(= N_("E651: Invalid buffer identifier in addAnno"));
EXTERN char e_invalid_buffer_identifier_in_getanno[]
	INIT(= N_("E652: Invalid buffer identifier in getAnno"));
#endif
// E653 unused
EXTERN char e_missing_delimiter_after_search_pattern_str[]
	INIT(= N_("E654: Missing delimiter after search pattern: %s"));
#ifdef FEAT_EVAL
EXTERN char e_too_many_symbolic_links_cycle[]
	INIT(= N_("E655: Too many symbolic links (cycle?)"));
#endif
#ifdef FEAT_NETBEANS_INTG
	// E656
EXTERN char e_netbeans_disallows_writes_of_unmodified_buffers[]
	INIT(= N_("NetBeans disallows writes of unmodified buffers"));
	// E657
EXTERN char e_partial_writes_disallowed_for_netbeans_buffers[]
	INIT(= N_("Partial writes disallowed for NetBeans buffers"));
EXTERN char e_netbeans_connection_lost_for_buffer_nr[]
	INIT(= N_("E658: NetBeans connection lost for buffer %d"));
#endif
#ifdef FEAT_PYTHON
EXTERN char e_cannot_invoke_python_recursively[]
	INIT(= N_("E659: Cannot invoke Python recursively"));
#endif
#ifdef FEAT_NETBEANS_INTG
EXTERN char e_cannot_open_netbeans_connection_info_file[]
	INIT(= "E660: Cannot open NetBeans connection info file");
#endif
#ifdef FEAT_MULTI_LANG
EXTERN char e_sorry_no_str_help_for_str[]
	INIT(= N_("E661: Sorry, no '%s' help for %s"));
#endif
EXTERN char e_at_start_of_changelist[]
	INIT(= N_("E662: At start of changelist"));
EXTERN char e_at_end_of_changelist[]
	INIT(= N_("E663: At end of changelist"));
EXTERN char e_changelist_is_empty[]
	INIT(= N_("E664: Changelist is empty"));
#ifdef FEAT_GUI
EXTERN char e_cannot_start_gui_no_valid_font_found[]
	INIT(= N_("E665: Cannot start GUI, no valid font found"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_compiler_not_supported_str[]
	INIT(= N_("E666: Compiler not supported: %s"));
#endif
#ifdef HAVE_FSYNC
EXTERN char e_fsync_failed[]
	INIT(= N_("E667: Fsync failed"));
#endif
#ifdef FEAT_NETBEANS_INTG
EXTERN char e_wrong_access_mode_for_netbeans_connection_info_file_str[]
	INIT(= N_("E668: Wrong access mode for NetBeans connection info file: \"%s\""));
#endif
EXTERN char e_unprintable_character_in_group_name[]
	INIT(= N_("E669: Unprintable character in group name"));
EXTERN char e_mix_of_help_file_encodings_within_language_str[]
	INIT(= N_("E670: Mix of help file encodings within a language: %s"));
#ifdef FEAT_GUI_MSWIN
EXTERN char e_cannot_find_window_title_str[]
	INIT(= N_("E671: Cannot find window title \"%s\""));
EXTERN char e_unable_to_open_window_inside_mdi_application[]
	INIT(= N_("E672: Unable to open window inside MDI application"));
#endif
#if defined(FEAT_PRINTER) && defined(FEAT_POSTSCRIPT)
EXTERN char e_incompatible_multi_byte_encoding_and_character_set[]
	INIT(= N_("E673: Incompatible multi-byte encoding and character set"));
EXTERN char e_printmbcharset_cannot_be_empty_with_multi_byte_encoding[]
	INIT(= N_("E674: printmbcharset cannot be empty with multi-byte encoding."));
EXTERN char e_no_default_font_specified_for_multi_byte_printing[]
	INIT(= N_("E675: No default font specified for multi-byte printing."));
#endif
EXTERN char e_no_matching_autocommands_for_buftype_str_buffer[]
	INIT(= N_("E676: No matching autocommands for buftype=%s buffer"));
#ifdef FEAT_SYN_HL
EXTERN char e_error_writing_temp_file[]
	INIT(= N_("E677: Error writing temp file"));
#endif
EXTERN char e_invalid_character_after_str_2[]
	INIT(= N_("E678: Invalid character after %s%%[dxouU]"));
#ifdef FEAT_SYN_HL
EXTERN char e_recursive_loop_loading_syncolor_vim[]
	INIT(= N_("E679: Recursive loop loading syncolor.vim"));
#endif
EXTERN char e_buffer_nr_invalid_buffer_number[]
	INIT(= N_("E680: <buffer=%d>: invalid buffer number"));
#if defined(FEAT_QUICKFIX) || defined(FEAT_EVAL)
EXTERN char e_buffer_is_not_loaded[]
	INIT(= N_("E681: Buffer is not loaded"));
#endif
#ifdef FEAT_QUICKFIX
EXTERN char e_invalid_search_pattern_or_delimiter[]
	INIT(= N_("E682: Invalid search pattern or delimiter"));
EXTERN char e_file_name_missing_or_invalid_pattern[]
	INIT(= N_("E683: File name missing or invalid pattern"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_list_index_out_of_range_nr[]
	INIT(= N_("E684: List index out of range: %ld"));
#endif
EXTERN char e_internal_error_str[]
	INIT(= N_("E685: Internal error: %s"));
#ifdef FEAT_EVAL
EXTERN char e_argument_of_str_must_be_list[]
	INIT(= N_("E686: Argument of %s must be a List"));
EXTERN char e_less_targets_than_list_items[]
	INIT(= N_("E687: Less targets than List items"));
EXTERN char e_more_targets_than_list_items[]
	INIT(= N_("E688: More targets than List items"));
EXTERN char e_index_not_allowed_after_str_str[]
	INIT(= N_("E689: Index not allowed after a %s: %s"));
EXTERN char e_missing_in_after_for[]
	INIT(= N_("E690: Missing \"in\" after :for"));
EXTERN char e_can_only_compare_list_with_list[]
	INIT(= N_("E691: Can only compare List with List"));
EXTERN char e_invalid_operation_for_list[]
	INIT(= N_("E692: Invalid operation for List"));
EXTERN char e_class_or_typealias_required_for_argument_nr[]
	INIT(= N_("E693: Class or class typealias required for argument %d"));
EXTERN char e_invalid_operation_for_funcrefs[]
	INIT(= N_("E694: Invalid operation for Funcrefs"));
EXTERN char e_cannot_index_a_funcref[]
	INIT(= N_("E695: Cannot index a Funcref"));
EXTERN char e_missing_comma_in_list_str[]
	INIT(= N_("E696: Missing comma in List: %s"));
EXTERN char e_missing_end_of_list_rsb_str[]
	INIT(= N_("E697: Missing end of List ']': %s"));
EXTERN char e_variable_nested_too_deep_for_making_copy[]
	INIT(= N_("E698: Variable nested too deep for making a copy"));
EXTERN char e_too_many_arguments[]
	INIT(= N_("E699: Too many arguments"));
EXTERN char e_unknown_function_str_2[]
	INIT(= N_("E700: Unknown function: %s"));
EXTERN char e_invalid_type_for_len[]
	INIT(= N_("E701: Invalid type for len()"));
EXTERN char e_sort_compare_function_failed[]
	INIT(= N_("E702: Sort compare function failed"));
EXTERN char e_using_funcref_as_number[]
	INIT(= N_("E703: Using a Funcref as a Number"));
EXTERN char e_funcref_variable_name_must_start_with_capital_str[]
	INIT(= N_("E704: Funcref variable name must start with a capital: %s"));
EXTERN char e_variable_name_conflicts_with_existing_function_str[]
	INIT(= N_("E705: Variable name conflicts with existing function: %s"));
EXTERN char e_argument_of_str_must_be_list_string_or_dictionary[]
	INIT(= N_("E706: Argument of %s must be a List, String or Dictionary"));
EXTERN char e_function_name_conflicts_with_variable_str[]
	INIT(= N_("E707: Function name conflicts with variable: %s"));
EXTERN char e_slice_must_come_last[]
	INIT(= N_("E708: [:] must come last"));
EXTERN char e_slice_requires_list_or_blob_value[]
	INIT(= N_("E709: [:] requires a List or Blob value"));
EXTERN char e_list_value_has_more_items_than_targets[]
	INIT(= N_("E710: List value has more items than targets"));
EXTERN char e_list_value_does_not_have_enough_items[]
	INIT(= N_("E711: List value does not have enough items"));
EXTERN char e_argument_of_str_must_be_list_or_dictionary[]
	INIT(= N_("E712: Argument of %s must be a List or Dictionary"));
EXTERN char e_cannot_use_empty_key_for_dictionary[]
	INIT(= N_("E713: Cannot use empty key for Dictionary"));
EXTERN char e_list_required[]
	INIT(= N_("E714: List required"));
EXTERN char e_dictionary_required[]
	INIT(= N_("E715: Dictionary required"));
EXTERN char e_key_not_present_in_dictionary_str[]
	INIT(= N_("E716: Key not present in Dictionary: \"%s\""));
EXTERN char e_dictionary_entry_already_exists[]
	INIT(= N_("E717: Dictionary entry already exists"));
EXTERN char e_funcref_required[]
	INIT(= N_("E718: Funcref required"));
EXTERN char e_cannot_slice_dictionary[]
	INIT(= N_("E719: Cannot slice a Dictionary"));
EXTERN char e_missing_colon_in_dictionary_str[]
	INIT(= N_("E720: Missing colon in Dictionary: %s"));
EXTERN char e_duplicate_key_in_dictionary_str[]
	INIT(= N_("E721: Duplicate key in Dictionary: \"%s\""));
EXTERN char e_missing_comma_in_dictionary_str[]
	INIT(= N_("E722: Missing comma in Dictionary: %s"));
EXTERN char e_missing_dict_end_str[]
	INIT(= N_("E723: Missing end of Dictionary '}': %s"));
EXTERN char e_variable_nested_too_deep_for_displaying[]
	INIT(= N_("E724: Variable nested too deep for displaying"));
EXTERN char e_calling_dict_function_without_dictionary_str[]
	INIT(= N_("E725: Calling dict function without Dictionary: %s"));
EXTERN char e_stride_is_zero[]
	INIT(= N_("E726: Stride is zero"));
EXTERN char e_start_past_end[]
	INIT(= N_("E727: Start past end"));
EXTERN char e_using_dictionary_as_number[]
	INIT(= N_("E728: Using a Dictionary as a Number"));
EXTERN char e_using_funcref_as_string[]
	INIT(= N_("E729: Using a Funcref as a String"));
EXTERN char e_using_list_as_string[]
	INIT(= N_("E730: Using a List as a String"));
EXTERN char e_using_dictionary_as_string[]
	INIT(= N_("E731: Using a Dictionary as a String"));
EXTERN char e_using_endfor_with_while[]
	INIT(= N_("E732: Using :endfor with :while"));
EXTERN char e_using_endwhile_with_for[]
	INIT(= N_("E733: Using :endwhile with :for"));
EXTERN char e_wrong_variable_type_for_str_equal[]
	INIT(= N_("E734: Wrong variable type for %s="));
EXTERN char e_can_only_compare_dictionary_with_dictionary[]
	INIT(= N_("E735: Can only compare Dictionary with Dictionary"));
EXTERN char e_invalid_operation_for_dictionary[]
	INIT(= N_("E736: Invalid operation for Dictionary"));
EXTERN char e_key_already_exists_str[]
	INIT(= N_("E737: Key already exists: %s"));
EXTERN char e_cant_list_variables_for_str[]
	INIT(= N_("E738: Can't list variables for %s"));
EXTERN char e_cannot_create_directory_str[]
	INIT(= N_("E739: Cannot create directory: %s"));
EXTERN char e_too_many_arguments_for_function_str_2[]
	INIT(= N_("E740: Too many arguments for function %s"));
EXTERN char e_value_is_locked[]
	INIT(= N_("E741: Value is locked"));
EXTERN char e_value_is_locked_str[]
	INIT(= N_("E741: Value is locked: %s"));
EXTERN char e_cannot_change_value[]
	INIT(= N_("E742: Cannot change value"));
EXTERN char e_cannot_change_value_of_str[]
	INIT(= N_("E742: Cannot change value of %s"));
EXTERN char e_variable_nested_too_deep_for_unlock[]
	INIT(= N_("E743: Variable nested too deep for (un)lock"));
#endif
#ifdef FEAT_NETBEANS_INTG
EXTERN char e_netbeans_does_not_allow_changes_in_read_only_files[]
	INIT(= N_("E744: NetBeans does not allow changes in read-only files"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_using_list_as_number[]
	INIT(= N_("E745: Using a List as a Number"));
EXTERN char e_function_name_does_not_match_script_file_name_str[]
	INIT(= N_("E746: Function name does not match script file name: %s"));
#endif
EXTERN char e_cannot_change_directory_buffer_is_modified_add_bang_to_override[]
	INIT(= N_("E747: Cannot change directory, buffer is modified (add ! to override)"));
EXTERN char e_no_previously_used_register[]
	INIT(= N_("E748: No previously used register"));
EXTERN char e_empty_buffer[]
	INIT(= N_("E749: Empty buffer"));
#ifdef FEAT_PROFILE
EXTERN char e_first_use_profile_start_fname[]
	INIT(= N_("E750: First use \":profile start {fname}\""));
#endif
#ifdef FEAT_SPELL
EXTERN char e_output_file_name_must_not_have_region_name[]
	INIT(= N_("E751: Output file name must not have region name"));
EXTERN char e_no_previous_spell_replacement[]
	INIT(= N_("E752: No previous spell replacement"));
EXTERN char e_not_found_str[]
	INIT(= N_("E753: Not found: %s"));
EXTERN char e_only_up_to_nr_regions_supported[]
	INIT(= N_("E754: Only up to %d regions supported"));
EXTERN char e_invalid_region_in_str[]
	INIT(= N_("E755: Invalid region in %s"));
EXTERN char e_spell_checking_is_not_possible[]
	INIT(= N_("E756: Spell checking is not possible"));
EXTERN char e_this_does_not_look_like_spell_file[]
	INIT(= N_("E757: This does not look like a spell file"));
EXTERN char e_truncated_spell_file[]
	INIT(= N_("E758: Truncated spell file"));
EXTERN char e_format_error_in_spell_file[]
	INIT(= N_("E759: Format error in spell file"));
EXTERN char e_no_word_count_in_str[]
	INIT(= N_("E760: No word count in %s"));
EXTERN char e_format_error_in_affix_file_fol_low_or_upp[]
	INIT(= N_("E761: Format error in affix file FOL, LOW or UPP"));
EXTERN char e_character_in_fol_low_or_upp_is_out_of_range[]
	INIT(= N_("E762: Character in FOL, LOW or UPP is out of range"));
EXTERN char e_word_characters_differ_between_spell_files[]
	INIT(= N_("E763: Word characters differ between spell files"));
#endif
#if defined(FEAT_SYN_HL) || defined(FEAT_COMPL_FUNC) || defined(FEAT_SPELL)
EXTERN char e_option_str_is_not_set[]
	INIT(= N_("E764: Option '%s' is not set"));
#endif
#ifdef FEAT_SPELL
EXTERN char e_spellfile_does_not_have_nr_entries[]
	INIT(= N_("E765: 'spellfile' does not have %d entries"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_insufficient_arguments_for_printf[]
	INIT(= N_("E766: Insufficient arguments for printf()"));
#endif
EXTERN char e_too_many_arguments_to_printf[]
	INIT(= N_("E767: Too many arguments for printf()"));
EXTERN char e_swap_file_exists_str_silent_overrides[]
	INIT(= N_("E768: Swap file exists: %s (:silent! overrides)"));
EXTERN char e_missing_rsb_after_str_lsb[]
	INIT(= N_("E769: Missing ] after %s["));
#ifdef FEAT_SPELL
EXTERN char e_unsupported_section_in_spell_file[]
	INIT(= N_("E770: Unsupported section in spell file"));
EXTERN char e_old_spell_file_needs_to_be_updated[]
	INIT(= N_("E771: Old spell file, needs to be updated"));
EXTERN char e_spell_file_is_for_newer_version_of_vim[]
	INIT(= N_("E772: Spell file is for newer version of Vim"));
#endif
EXTERN char e_symlink_loop_for_str[]
	INIT(= N_("E773: Symlink loop for \"%s\""));
#ifdef FEAT_EVAL
EXTERN char e_operatorfunc_is_empty[]
	INIT(= N_("E774: 'operatorfunc' is empty"));
#else
EXTERN char e_eval_feature_not_available[]
	INIT(= N_("E775: Eval feature not available"));
#endif
#ifdef FEAT_QUICKFIX
EXTERN char e_no_location_list[]
	INIT(= N_("E776: No location list"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_string_or_list_expected[]
	INIT(= N_("E777: String or List expected"));
#endif
#ifdef FEAT_SPELL
EXTERN char e_this_does_not_look_like_sug_file_str[]
	INIT(= N_("E778: This does not look like a .sug file: %s"));
EXTERN char e_old_sug_file_needs_to_be_updated_str[]
	INIT(= N_("E779: Old .sug file, needs to be updated: %s"));
EXTERN char e_sug_file_is_for_newer_version_of_vim_str[]
	INIT(= N_("E780: .sug file is for newer version of Vim: %s"));
EXTERN char e_sug_file_doesnt_match_spl_file_str[]
	INIT(= N_("E781: .sug file doesn't match .spl file: %s"));
EXTERN char e_error_while_reading_sug_file_str[]
	INIT(= N_("E782: Error while reading .sug file: %s"));
EXTERN char e_duplicate_char_in_map_entry[]
	INIT(= N_("E783: Duplicate char in MAP entry"));
#endif
EXTERN char e_cannot_close_last_tab_page[]
	INIT(= N_("E784: Cannot close last tab page"));
#ifdef FEAT_EVAL
# ifdef FEAT_COMPL_FUNC
EXTERN char e_complete_can_only_be_used_in_insert_mode[]
	INIT(= N_("E785: complete() can only be used in Insert mode"));
# endif
EXTERN char e_range_not_allowed[]
	INIT(= N_("E786: Range not allowed"));
#endif
#ifdef FEAT_DIFF
EXTERN char e_buffer_changed_unexpectedly[]
	INIT(= N_("E787: Buffer changed unexpectedly"));
#endif
EXTERN char e_not_allowed_to_edit_another_buffer_now[]
	INIT(= N_("E788: Not allowed to edit another buffer now"));
#ifdef FEAT_SYN_HL
EXTERN char e_error_missing_rsb_str[]
	INIT(= N_("E789: Missing ']': %s"));
#endif
EXTERN char e_undojoin_is_not_allowed_after_undo[]
	INIT(= N_("E790: undojoin is not allowed after undo"));
#ifdef FEAT_KEYMAP
EXTERN char e_empty_keymap_entry[]
	INIT(= N_("E791: Empty keymap entry"));
#endif
#ifdef FEAT_MENU
EXTERN char e_empty_menu_name[]
	INIT(= N_("E792: Empty menu name"));
#endif
#ifdef FEAT_DIFF
EXTERN char e_no_other_buffer_in_diff_mode_is_modifiable[]
	INIT(= N_("E793: No other buffer in diff mode is modifiable"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_cannot_set_variable_in_sandbox[]
	INIT(= N_("E794: Cannot set variable in the sandbox"));
EXTERN char e_cannot_set_variable_in_sandbox_str[]
	INIT(= N_("E794: Cannot set variable in the sandbox: \"%s\""));
EXTERN char e_cannot_delete_variable[]
	INIT(= N_("E795: Cannot delete variable"));
EXTERN char e_cannot_delete_variable_str[]
	INIT(= N_("E795: Cannot delete variable %s"));
#endif
#ifdef MSWIN
	// E796
EXTERN char e_writing_to_device_disabled_with_opendevice_option[]
	INIT(= N_("writing to device disabled with 'opendevice' option"));
#endif
#ifdef FEAT_SPELL
EXTERN char e_spellfilemising_autocommand_deleted_buffer[]
	INIT(= N_("E797: SpellFileMissing autocommand deleted buffer"));
#endif
#ifdef FEAT_SEARCH_EXTRA
EXTERN char e_id_is_reserved_for_match_nr[]
	INIT(= N_("E798: ID is reserved for \":match\": %d"));
EXTERN char e_invalid_id_nr_must_be_greater_than_or_equal_to_one_1[]
	INIT(= N_("E799: Invalid ID: %d (must be greater than or equal to 1)"));
#endif
#ifndef FEAT_ARABIC
EXTERN char e_arabic_cannot_be_used_not_enabled_at_compile_time[]
	INIT(= N_("E800: Arabic cannot be used: Not enabled at compile time\n"));
#endif
#ifdef FEAT_SEARCH_EXTRA
EXTERN char e_id_already_taken_nr[]
	INIT(= N_("E801: ID already taken: %d"));
EXTERN char e_invalid_id_nr_must_be_greater_than_or_equal_to_one_2[]
	INIT(= N_("E802: Invalid ID: %d (must be greater than or equal to 1)"));
EXTERN char e_id_not_found_nr[]
	INIT(= N_("E803: ID not found: %d"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_percent_with_float[]
	// xgettext:no-c-format
	INIT(= N_("E804: Cannot use '%' with Float"));
EXTERN char e_using_float_as_number[]
	INIT(= N_("E805: Using a Float as a Number"));
EXTERN char e_using_float_as_string[]
	INIT(= N_("E806: Using a Float as a String"));
EXTERN char e_expected_float_argument_for_printf[]
	INIT(= N_("E807: Expected Float argument for printf()"));
EXTERN char e_number_or_float_required[]
	INIT(= N_("E808: Number or Float required"));
#endif
#ifndef FEAT_EVAL
EXTERN char e_hashsmall_is_not_available_without_the_eval_feature[]
	INIT(= N_("E809: #< is not available without the +eval feature"));
#endif
#ifdef FEAT_DIFF
EXTERN char e_cannot_read_or_write_temp_files[]
	INIT(= N_("E810: Cannot read or write temp files"));
#endif
EXTERN char e_not_allowed_to_change_buffer_information_now[]
	INIT(= N_("E811: Not allowed to change buffer information now"));
EXTERN char e_autocommands_changed_buffer_or_buffer_name[]
	INIT(= N_("E812: Autocommands changed buffer or buffer name"));
EXTERN char e_cannot_close_autocmd_or_popup_window[]
	INIT(= N_("E813: Cannot close autocmd or popup window"));
EXTERN char e_cannot_close_window_only_autocmd_window_would_remain[]
	INIT(= N_("E814: Cannot close window, only autocmd window would remain"));
#ifdef FEAT_MZSCHEME
EXTERN char e_sorry_this_command_is_disabled_the_mzscheme_libraries_could_not_be_loaded[]
	INIT(= N_("E815: Sorry, this command is disabled, the MzScheme libraries could not be loaded."));
#endif
#ifdef FEAT_DIFF
EXTERN char e_cannot_read_patch_output[]
	INIT(= N_("E816: Cannot read patch output"));
#endif
#ifdef FEAT_CRYPT
EXTERN char e_blowfish_big_little_endian_use_wrong[]
	INIT(= N_("E817: Blowfish big/little endian use wrong"));
EXTERN char e_sha256_test_failed[]
	INIT(= N_("E818: sha256 test failed"));
EXTERN char e_blowfish_test_failed[]
	INIT(= N_("E819: Blowfish test failed"));
EXTERN char e_sizeof_uint32_isnot_four[]
	INIT(= N_("E820: sizeof(uint32_t) != 4"));
EXTERN char e_file_is_encrypted_with_unknown_method[]
	INIT(= N_("E821: File is encrypted with unknown method"));
#endif
#ifdef FEAT_PERSISTENT_UNDO
EXTERN char e_cannot_open_undo_file_for_reading_str[]
	INIT(= N_("E822: Cannot open undo file for reading: %s"));
EXTERN char e_not_an_undo_file_str[]
	INIT(= N_("E823: Not an undo file: %s"));
EXTERN char e_incompatible_undo_file_str[]
	INIT(= N_("E824: Incompatible undo file: %s"));
EXTERN char e_corrupted_undo_file_str_str[]
	INIT(= N_("E825: Corrupted undo file (%s): %s"));
# ifdef FEAT_CRYPT
EXTERN char e_undo_file_decryption_failed[]
	INIT(= N_("E826: Undo file decryption failed: %s"));
# else
EXTERN char e_undo_file_is_encrypted_str[]
	INIT(= N_("E827: Undo file is encrypted: %s"));
# endif
EXTERN char e_cannot_open_undo_file_for_writing_str[]
	INIT(= N_("E828: Cannot open undo file for writing: %s"));
EXTERN char e_write_error_in_undo_file_str[]
	INIT(= N_("E829: Write error in undo file: %s"));
#endif
EXTERN char e_undo_number_nr_not_found[]
	INIT(= N_("E830: Undo number %ld not found"));
#ifdef FEAT_CRYPT
EXTERN char e_bf_key_init_called_with_empty_password[]
	INIT(= "E831: bf_key_init() called with empty password");
# ifdef FEAT_PERSISTENT_UNDO
EXTERN char e_non_encrypted_file_has_encrypted_undo_file_str[]
	INIT(= N_("E832: Non-encrypted file has encrypted undo file: %s"));
# endif
#else
EXTERN char e_str_is_encrypted_and_this_version_of_vim_does_not_support_encryption[]
	INIT(= N_("E833: %s is encrypted and this version of Vim does not support encryption"));
#endif
EXTERN char e_conflicts_with_value_of_listchars[]
	INIT(= N_("E834: Conflicts with value of 'listchars'"));
EXTERN char e_conflicts_with_value_of_fillchars[]
	INIT(= N_("E835: Conflicts with value of 'fillchars'"));
#ifdef DYNAMIC_PYTHON
EXTERN char e_this_vim_cannot_execute_python_after_using_py3[]
	INIT(= N_("E836: This Vim cannot execute :python after using :py3"));
EXTERN char e_this_vim_cannot_execute_py3_after_using_python[]
	INIT(= N_("E837: This Vim cannot execute :py3 after using :python"));
#endif
#if defined(FEAT_NETBEANS_INTG) && defined(FEAT_GUI)
EXTERN char e_netbeans_is_not_supported_with_this_GUI[]
	INIT(= N_("E838: NetBeans is not supported with this GUI"));
#endif
// E839 unused
# ifdef FEAT_COMPL_FUNC
EXTERN char e_complete_function_deleted_text[]
	INIT(= N_("E840: Completion function deleted text"));
# endif
EXTERN char e_reserved_name_cannot_be_used_for_user_defined_command[]
	INIT(= N_("E841: Reserved name, cannot be used for user defined command"));
EXTERN char e_no_line_number_to_use_for_slnum[]
	INIT(= N_("E842: No line number to use for \"<slnum>\""));
#ifdef FEAT_CRYPT
EXTERN char e_error_while_updating_swap_file_crypt[]
	INIT(= N_("E843: Error while updating swap file crypt"));
#endif
#ifdef FEAT_CONCEAL
EXTERN char e_invalid_cchar_value[]
	INIT(= N_("E844: Invalid cchar value"));
#endif
#ifdef FEAT_SPELL
EXTERN char e_insufficient_memory_word_list_will_be_incomplete[]
	INIT(= N_("E845: Insufficient memory, word list will be incomplete"));
#endif
EXTERN char e_key_code_not_set[]
	INIT(= N_("E846: Key code not set"));
#ifdef FEAT_SYN_HL
EXTERN char e_too_many_syntax_includes[]
	INIT(= N_("E847: Too many syntax includes"));
EXTERN char e_too_many_syntax_clusters[]
	INIT(= N_("E848: Too many syntax clusters"));
#endif
EXTERN char e_too_many_highlight_and_syntax_groups[]
	INIT(= N_("E849: Too many highlight and syntax groups"));
#ifndef FEAT_CLIPBOARD
EXTERN char e_invalid_register_name[]
	INIT(= N_("E850: Invalid register name"));
#endif
#ifdef FEAT_GUI
EXTERN char e_failed_to_create_new_process_for_GUI[]
	INIT(= N_("E851: Failed to create a new process for the GUI"));
EXTERN char e_the_child_process_failed_to_start_GUI[]
	INIT(= N_("E852: The child process failed to start the GUI"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_duplicate_argument_name_str[]
	INIT(= N_("E853: Duplicate argument name: %s"));
#endif
EXTERN char e_path_too_long_for_completion[]
	INIT(= N_("E854: Path too long for completion"));
EXTERN char e_autocommands_caused_command_to_abort[]
	INIT(= N_("E855: Autocommands caused command to abort"));
#ifdef FEAT_EVAL
EXTERN char e_assert_fails_second_arg[]
	INIT(= N_("E856: \"assert_fails()\" second argument must be a string or a list with one or two strings"));
EXTERN char e_dictionary_key_str_required[]
	INIT(= N_("E857: Dictionary key \"%s\" required"));
#endif
#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
EXTERN char e_eval_did_not_return_valid_python_object[]
	INIT(= N_("E858: Eval did not return a valid python object"));
EXTERN char e_failed_to_convert_returned_python_object_to_vim_value[]
	INIT(= N_("E859: Failed to convert returned python object to a Vim value"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_need_id_and_type_or_types_with_both[]
	INIT(= N_("E860: Need 'id' and 'type' or 'types' with 'both'"));
# ifdef FEAT_TERMINAL
EXTERN char e_cannot_open_second_popup_with_terminal[]
	INIT(= N_("E861: Cannot open a second popup with a terminal"));
# endif
#endif
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_g_here[]
	INIT(= N_("E862: Cannot use g: here"));
#endif
#if defined(FEAT_PROP_POPUP) && defined(FEAT_TERMINAL)
EXTERN char e_not_allowed_for_terminal_in_popup_window[]
	INIT(= N_("E863: Not allowed for a terminal in a popup window"));
#endif
EXTERN char e_percent_hash_can_only_be_followed_by_zero_one_two_automatic_engine_will_be_used[]
	// xgettext:no-c-format
	INIT(= N_("E864: \\%#= can only be followed by 0, 1, or 2. The automatic engine will be used"));
EXTERN char e_nfa_regexp_end_encountered_prematurely[]
	INIT(= N_("E865: (NFA) Regexp end encountered prematurely"));
EXTERN char e_nfa_regexp_misplaced_chr[]
	INIT(= N_("E866: (NFA regexp) Misplaced %c"));
EXTERN char e_nfa_regexp_unknown_operator_z_chr[]
	INIT(= N_("E867: (NFA regexp) Unknown operator '\\z%c'"));
EXTERN char e_nfa_regexp_unknown_operator_percent_chr[]
	INIT(= N_("E867: (NFA regexp) Unknown operator '\\%%%c'"));
EXTERN char e_error_building_nfa_with_equivalence_class[]
	INIT(= N_("E868: Error building NFA with equivalence class!"));
EXTERN char e_nfa_regexp_unknown_operator_at_chr[]
	INIT(= N_("E869: (NFA regexp) Unknown operator '\\@%c'"));
EXTERN char e_nfa_regexp_error_reading_repetition_limits[]
	INIT(= N_("E870: (NFA regexp) Error reading repetition limits"));
EXTERN char e_nfa_regexp_cant_have_multi_follow_multi[]
	INIT(= N_("E871: (NFA regexp) Can't have a multi follow a multi"));
EXTERN char e_nfa_regexp_too_many_parens[]
	INIT(= N_("E872: (NFA regexp) Too many '('"));
EXTERN char e_nfa_regexp_proper_termination_error[]
	INIT(= N_("E873: (NFA regexp) proper termination error"));
EXTERN char e_nfa_regexp_could_not_pop_stack[]
	INIT(= N_("E874: (NFA regexp) Could not pop the stack!"));
EXTERN char e_nfa_regexp_while_converting_from_postfix_to_nfa_too_many_stats_left_on_stack[]
	INIT(= N_("E875: (NFA regexp) (While converting from postfix to NFA), too many states left on stack"));
EXTERN char e_nfa_regexp_not_enough_space_to_store_whole_nfa[]
	INIT(= N_("E876: (NFA regexp) Not enough space to store the whole NFA"));
EXTERN char e_nfa_regexp_invalid_character_class_nr[]
	INIT(= "E877: (NFA regexp) Invalid character class: %d");
EXTERN char e_nfa_regexp_could_not_allocate_memory_for_branch_traversal[]
	INIT(= N_("E878: (NFA regexp) Could not allocate memory for branch traversal!"));
#ifdef FEAT_SYN_HL
EXTERN char e_nfa_regexp_too_many_z[]
	INIT(= N_("E879: (NFA regexp) Too many \\z("));
#endif
#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
EXTERN char e_cant_handle_systemexit_of_python_exception_in_vim[]
	INIT(= N_("E880: Can't handle SystemExit of python exception in vim"));
#endif
EXTERN char e_line_count_changed_unexpectedly[]
	INIT(= N_("E881: Line count changed unexpectedly"));
#ifdef FEAT_EVAL
EXTERN char e_uniq_compare_function_failed[]
	INIT(= N_("E882: Uniq compare function failed"));
EXTERN char e_search_pattern_and_expression_register_may_not_contain_two_or_more_lines[]
	INIT(= N_("E883: Search pattern and expression register may not contain two or more lines"));
EXTERN char e_function_name_cannot_contain_colon_str[]
	INIT(= N_("E884: Function name cannot contain a colon: %s"));
#endif
#ifdef FEAT_SIGNS
EXTERN char e_not_possible_to_change_sign_str[]
	INIT(= N_("E885: Not possible to change sign %s"));
#endif
#ifdef FEAT_VIMINFO
EXTERN char e_cant_rename_viminfo_file_to_str[]
	INIT(= N_("E886: Can't rename viminfo file to %s!"));
#endif
#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
EXTERN char e_sorry_this_command_is_disabled_python_side_module_could_not_be_loaded[]
	INIT(= N_("E887: Sorry, this command is disabled, the Python's site module could not be loaded."));
#endif
EXTERN char e_nfa_regexp_cannot_repeat_str[]
	INIT(= N_("E888: (NFA regexp) cannot repeat %s"));
#ifdef FEAT_PROP_POPUP
EXTERN char e_number_required[]
	INIT(= N_("E889: Number required"));
#endif
#ifdef FEAT_SYN_HL
EXTERN char e_trailing_char_after_rsb_str_str[]
	INIT(= N_("E890: Trailing char after ']': %s]%s"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_using_funcref_as_float[]
	INIT(= N_("E891: Using a Funcref as a Float"));
EXTERN char e_using_string_as_float[]
	INIT(= N_("E892: Using a String as a Float"));
EXTERN char e_using_list_as_float[]
	INIT(= N_("E893: Using a List as a Float"));
EXTERN char e_using_dictionary_as_float[]
	INIT(= N_("E894: Using a Dictionary as a Float"));
#endif
#ifdef FEAT_MZSCHEME
EXTERN char e_sorry_this_command_is_disabled_the_mzscheme_racket_base_module_could_not_be_loaded[]
	INIT(= N_("E895: Sorry, this command is disabled, the MzScheme's racket/base module could not be loaded."));
#endif
#ifdef FEAT_EVAL
EXTERN char e_argument_of_str_must_be_list_dictionary_or_blob[]
	INIT(= N_("E896: Argument of %s must be a List, Dictionary or Blob"));
EXTERN char e_list_or_blob_required[]
	INIT(= N_("E897: List or Blob required"));
#endif
#ifdef FEAT_JOB_CHANNEL
EXTERN char e_socket_in_channel_connect[]
	INIT(= N_("E898: socket() in channel_connect()"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_argument_of_str_must_be_list_or_blob[]
	INIT(= N_("E899: Argument of %s must be a List or Blob"));
EXTERN char e_maxdepth_must_be_non_negative_number[]
	INIT(= N_("E900: maxdepth must be non-negative number"));
#endif
#ifdef FEAT_JOB_CHANNEL
EXTERN char e_getaddrinfo_in_channel_open_str[]
	INIT(= N_("E901: getaddrinfo() in channel_open(): %s"));
# ifndef FEAT_IPV6
EXTERN char e_gethostbyname_in_channel_open[]
	INIT(= N_("E901: gethostbyname() in channel_open()"));
# endif
EXTERN char e_cannot_connect_to_port[]
	INIT(= N_("E902: Cannot connect to port"));
EXTERN char e_received_command_with_non_string_argument[]
	INIT(= N_("E903: Received command with non-string argument"));
EXTERN char e_last_argument_for_expr_call_must_be_number[]
	INIT(= N_("E904: Last argument for expr/call must be a number"));
EXTERN char e_third_argument_for_call_must_be_list[]
	INIT(= N_("E904: Third argument for call must be a list"));
EXTERN char e_received_unknown_command_str[]
	INIT(= N_("E905: Received unknown command: %s"));
EXTERN char e_not_an_open_channel[]
	INIT(= N_("E906: Not an open channel"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_using_special_value_as_float[]
	INIT(= N_("E907: Using a special value as a Float"));
EXTERN char e_using_invalid_value_as_string_str[]
	INIT(= N_("E908: Using an invalid value as a String: %s"));
EXTERN char e_cannot_index_special_variable[]
	INIT(= N_("E909: Cannot index a special variable"));
#endif
#ifdef FEAT_JOB_CHANNEL
EXTERN char e_using_job_as_number[]
	INIT(= N_("E910: Using a Job as a Number"));
EXTERN char e_using_job_as_float[]
	INIT(= N_("E911: Using a Job as a Float"));
EXTERN char e_cannot_use_evalexpr_sendexpr_with_raw_or_nl_channel[]
	INIT(= N_("E912: Cannot use ch_evalexpr()/ch_sendexpr() with a raw or nl channel"));
EXTERN char e_using_channel_as_number[]
	INIT(= N_("E913: Using a Channel as a Number"));
EXTERN char e_using_channel_as_float[]
	INIT(= N_("E914: Using a Channel as a Float"));
EXTERN char e_in_io_buffer_requires_in_buf_or_in_name_to_be_set[]
	INIT(= N_("E915: in_io buffer requires in_buf or in_name to be set"));
EXTERN char e_not_valid_job[]
	INIT(= N_("E916: Not a valid job"));
EXTERN char e_cannot_use_callback_with_str[]
	INIT(= N_("E917: Cannot use a callback with %s()"));
EXTERN char e_buffer_must_be_loaded_str[]
	INIT(= N_("E918: Buffer must be loaded: %s"));
#endif
EXTERN char e_directory_not_found_in_str_str[]
	INIT(= N_("E919: Directory not found in '%s': \"%s\""));
#ifdef FEAT_JOB_CHANNEL
EXTERN char e_io_file_requires_name_to_be_set[]
	INIT(= N_("E920: _io file requires _name to be set"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_invalid_callback_argument[]
	INIT(= N_("E921: Invalid callback argument"));
// E922 unused
EXTERN char e_second_argument_of_function_must_be_list_or_dict[]
	INIT(= N_("E923: Second argument of function() must be a list or a dict"));
#endif
#ifdef FEAT_QUICKFIX
EXTERN char e_current_window_was_closed[]
	INIT(= N_("E924: Current window was closed"));
EXTERN char e_current_quickfix_list_was_changed[]
	INIT(= N_("E925: Current quickfix list was changed"));
EXTERN char e_current_location_list_was_changed[]
	INIT(= N_("E926: Current location list was changed"));
#endif
#ifdef FEAT_EVAL
# ifdef FEAT_QUICKFIX
EXTERN char e_invalid_action_str_1[]
	INIT(= N_("E927: Invalid action: '%s'"));
# endif
EXTERN char e_string_required[]
	INIT(= N_("E928: String required"));
#endif
#ifdef FEAT_VIMINFO
EXTERN char e_too_many_viminfo_temp_files_like_str[]
	INIT(= N_("E929: Too many viminfo temp files, like %s!"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_redir_inside_execute[]
	INIT(= N_("E930: Cannot use :redir inside execute()"));
#endif
EXTERN char e_buffer_cannot_be_registered[]
	INIT(= N_("E931: Buffer cannot be registered"));
#ifdef FEAT_EVAL
EXTERN char e_closure_function_should_not_be_at_top_level_str[]
	INIT(= N_("E932: Closure function should not be at top level: %s"));
EXTERN char e_function_was_deleted_str[]
	INIT(= N_("E933: Function was deleted: %s"));
#endif
#ifdef FEAT_SIGNS
EXTERN char e_cannot_jump_to_buffer_that_does_not_have_name[]
	INIT(= N_("E934: Cannot jump to a buffer that does not have a name"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_invalid_submatch_number_nr[]
	INIT(= N_("E935: Invalid submatch number: %d"));
#endif
EXTERN char e_cannot_delete_current_group[]
	INIT(= N_("E936: Cannot delete the current group"));
EXTERN char e_attempt_to_delete_buffer_that_is_in_use_str[]
	INIT(= N_("E937: Attempt to delete a buffer that is in use: %s"));
#ifdef FEAT_EVAL
EXTERN char e_duplicate_key_in_json_str[]
	INIT(= N_("E938: Duplicate key in JSON: \"%s\""));
#endif
EXTERN char e_positive_count_required[]
	INIT(= N_("E939: Positive count required"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_lock_or_unlock_variable_str[]
	INIT(= N_("E940: Cannot lock or unlock variable %s"));
# ifdef FEAT_CLIENTSERVER
EXTERN char e_already_started_server[]
	INIT(= N_("E941: Already started a server"));
# else
EXTERN char e_clientserver_feature_not_available[]
	INIT(= N_("E942: +clientserver feature not available"));
# endif
#endif
EXTERN char e_command_table_needs_to_be_updated_run_make_cmdidxs[]
	INIT(= "E943: Command table needs to be updated, run 'make cmdidxs'");
EXTERN char e_reverse_range_in_character_class[]
	INIT(= N_("E944: Reverse range in character class"));
EXTERN char e_range_too_large_in_character_class[]
	INIT(= N_("E945: Range too large in character class"));
#ifdef FEAT_TERMINAL
EXTERN char e_cannot_make_terminal_with_running_job_modifiable[]
	INIT(= N_("E946: Cannot make a terminal with running job modifiable"));
EXTERN char e_job_still_running_in_buffer_str[]
	INIT(= N_("E947: Job still running in buffer \"%s\""));
EXTERN char e_job_still_running[]
	INIT(= N_("E948: Job still running"));
EXTERN char e_job_still_running_add_bang_to_end_the_job[]
	INIT(= N_("E948: Job still running (add ! to end the job)"));
#endif
EXTERN char e_file_changed_while_writing[]
	INIT(= N_("E949: File changed while writing"));
EXTERN char e_cannot_convert_between_str_and_str[]
	INIT(= N_("E950: Cannot convert between %s and %s"));
EXTERN char e_percent_value_too_large[]
	// xgettext:no-c-format
	INIT(= N_("E951: \\% value too large"));
#if defined(FEAT_EVAL) && defined(FEAT_QUICKFIX)
EXTERN char e_autocommand_caused_recursive_behavior[]
	INIT(= N_("E952: Autocommand caused recursive behavior"));
#endif
#ifdef FEAT_TERMINAL
EXTERN char e_file_exists_str[]
	INIT(= N_("E953: File exists: %s"));
#endif
#if defined(FEAT_TERMGUICOLORS) && defined(FEAT_VTP)
EXTERN char e_24_bit_colors_are_not_supported_on_this_environment[]
	INIT(= N_("E954: 24-bit colors are not supported on this environment"));
#endif
#ifdef FEAT_TERMINAL
EXTERN char e_not_terminal_buffer[]
	INIT(= N_("E955: Not a terminal buffer"));
#endif
EXTERN char e_cannot_use_pattern_recursively[]
	INIT(= N_("E956: Cannot use pattern recursively"));
#ifdef FEAT_EVAL
EXTERN char e_invalid_window_number[]
	INIT(= N_("E957: Invalid window number"));
#endif
#ifdef FEAT_TERMINAL
EXTERN char e_job_already_finished[]
	INIT(= N_("E958: Job already finished"));
#endif
#ifdef FEAT_DIFF
EXTERN char e_invalid_diff_format[]
	INIT(= N_("E959: Invalid diff format."));
EXTERN char e_problem_creating_internal_diff[]
	INIT(= N_("E960: Problem creating the internal diff"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_no_line_number_to_use_for_sflnum[]
	INIT(= N_("E961: No line number to use for \"<sflnum>\""));
EXTERN char e_invalid_action_str_2[]
	INIT(= N_("E962: Invalid action: '%s'"));
EXTERN char e_setting_v_str_to_value_with_wrong_type[]
	INIT(= N_("E963: Setting v:%s to value with wrong type"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char_u e_invalid_column_number_nr[]
	INIT(= N_("E964: Invalid column number: %ld"));
EXTERN char e_missing_property_type_name[]
	INIT(= N_("E965: Missing property type name"));
#endif
#ifdef FEAT_EVAL
EXTERN char_u e_invalid_line_number_nr[]
	INIT(= N_("E966: Invalid line number: %ld"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_text_property_info_corrupted[]
	INIT(= "E967: Text property info corrupted");
EXTERN char e_need_at_least_one_of_id_or_type[]
	INIT(= N_("E968: Need at least one of 'id' or 'type'"));
EXTERN char e_property_type_str_already_defined[]
	INIT(= N_("E969: Property type %s already defined"));
EXTERN char e_unknown_highlight_group_name_str[]
	INIT(= N_("E970: Unknown highlight group name: '%s'"));
EXTERN char e_property_type_str_does_not_exist[]
	INIT(= N_("E971: Property type %s does not exist"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_blob_value_does_not_have_right_number_of_bytes[]
	INIT(= N_("E972: Blob value does not have the right number of bytes"));
EXTERN char e_blob_literal_should_have_an_even_number_of_hex_characters[]
	INIT(= N_("E973: Blob literal should have an even number of hex characters"));
EXTERN char e_using_blob_as_number[]
	INIT(= N_("E974: Using a Blob as a Number"));
EXTERN char e_using_blob_as_float[]
	INIT(= N_("E975: Using a Blob as a Float"));
EXTERN char e_using_blob_as_string[]
	INIT(= N_("E976: Using a Blob as a String"));
EXTERN char e_can_only_compare_blob_with_blob[]
	INIT(= N_("E977: Can only compare Blob with Blob"));
EXTERN char e_invalid_operation_for_blob[]
	INIT(= N_("E978: Invalid operation for Blob"));
EXTERN char e_blob_index_out_of_range_nr[]
	INIT(= N_("E979: Blob index out of range: %ld"));
# ifndef USE_INPUT_BUF
EXTERN char e_lowlevel_input_not_supported[]
	INIT(= N_("E980: Lowlevel input not supported"));
# endif
#endif
EXTERN char e_command_not_allowed_in_rvim[]
	INIT(= N_("E981: Command not allowed in rvim"));
#if defined(FEAT_TERMINAL) && defined(MSWIN)
EXTERN char e_conpty_is_not_available[]
	INIT(= N_("E982: ConPTY is not available"));
#endif
EXTERN char e_duplicate_argument_str[]
	INIT(= N_("E983: Duplicate argument: %s"));
EXTERN char e_scriptversion_used_outside_of_sourced_file[]
	INIT(= N_("E984: :scriptversion used outside of a sourced file"));
#ifdef FEAT_EVAL
EXTERN char e_dot_equal_not_supported_with_script_version_two[]
	INIT(= N_("E985: .= is not supported with script version >= 2"));
EXTERN char e_cannot_modify_tag_stack_within_tagfunc[]
	INIT(= N_("E986: Cannot modify the tag stack within tagfunc"));
EXTERN char e_invalid_return_value_from_tagfunc[]
	INIT(= N_("E987: Invalid return value from tagfunc"));
#endif
#ifdef GUI_MAY_SPAWN
EXTERN char e_gui_cannot_be_used_cannot_execute_gvim_exe[]
	INIT(= N_("E988: GUI cannot be used. Cannot execute gvim.exe."));
#endif
#ifdef FEAT_EVAL
EXTERN char e_non_default_argument_follows_default_argument[]
	INIT(= N_("E989: Non-default argument follows default argument"));
EXTERN char e_missing_end_marker_str[]
	INIT(= N_("E990: Missing end marker '%s'"));
EXTERN char e_cannot_use_heredoc_here[]
	INIT(= N_("E991: Cannot use =<< here"));
#endif
EXTERN char e_not_allowed_in_modeline_when_modelineexpr_is_off[]
	INIT(= N_("E992: Not allowed in a modeline when 'modelineexpr' is off"));
#ifdef FEAT_EVAL
EXTERN char e_window_nr_is_not_popup_window[]
	INIT(= N_("E993: Window %d is not a popup window"));
EXTERN char e_not_allowed_in_popup_window[]
	INIT(= N_("E994: Not allowed in a popup window"));
EXTERN char e_cannot_modify_existing_variable[]
	INIT(= N_("E995: Cannot modify existing variable"));
EXTERN char e_cannot_lock_range[]
	INIT(= N_("E996: Cannot lock a range"));
EXTERN char e_cannot_lock_option[]
	INIT(= N_("E996: Cannot lock an option"));
EXTERN char e_cannot_lock_list_or_dict[]
	INIT(= N_("E996: Cannot lock a list or dict"));
EXTERN char e_cannot_lock_environment_variable[]
	INIT(= N_("E996: Cannot lock an environment variable"));
EXTERN char e_cannot_lock_register[]
	INIT(= N_("E996: Cannot lock a register"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_tabpage_not_found_nr[]
	INIT(= N_("E997: Tabpage not found: %d"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_reduce_of_an_empty_str_with_no_initial_value[]
	INIT(= N_("E998: Reduce of an empty %s with no initial value"));
#endif
EXTERN char e_scriptversion_not_supported_nr[]
	INIT(= N_("E999: scriptversion not supported: %d"));
// E1000 unused
#ifdef FEAT_EVAL
EXTERN char e_variable_not_found_str[]
	INIT(= N_("E1001: Variable not found: %s"));
EXTERN char e_syntax_error_at_str[]
	INIT(= N_("E1002: Syntax error at %s"));
EXTERN char e_missing_return_value[]
	INIT(= N_("E1003: Missing return value"));
EXTERN char e_white_space_required_before_and_after_str_at_str[]
	INIT(= N_("E1004: White space required before and after '%s' at \"%s\""));
EXTERN char e_too_many_argument_types[]
	INIT(= N_("E1005: Too many argument types"));
EXTERN char e_str_is_used_as_argument[]
	INIT(= N_("E1006: %s is used as an argument"));
EXTERN char e_mandatory_argument_after_optional_argument[]
	INIT(= N_("E1007: Mandatory argument after optional argument"));
EXTERN char e_missing_type_after_str[]
	INIT(= N_("E1008: Missing <type> after %s"));
EXTERN char e_missing_gt_after_type_str[]
	INIT(= N_("E1009: Missing > after type: %s"));
EXTERN char e_type_not_recognized_str[]
	INIT(= N_("E1010: Type not recognized: %s"));
EXTERN char e_name_too_long_str[]
	INIT(= N_("E1011: Name too long: %s"));
EXTERN char e_type_mismatch_expected_str_but_got_str[]
	INIT(= N_("E1012: Type mismatch; expected %s but got %s"));
EXTERN char e_type_mismatch_expected_str_but_got_str_in_str[]
	INIT(= N_("E1012: Type mismatch; expected %s but got %s in %s"));
EXTERN char e_argument_nr_type_mismatch_expected_str_but_got_str[]
	INIT(= N_("E1013: Argument %d: type mismatch, expected %s but got %s"));
EXTERN char e_argument_nr_type_mismatch_expected_str_but_got_str_in_str[]
	INIT(= N_("E1013: Argument %d: type mismatch, expected %s but got %s in %s"));
EXTERN char e_invalid_key_str[]
	INIT(= N_("E1014: Invalid key: %s"));
EXTERN char e_name_expected_str[]
	INIT(= N_("E1015: Name expected: %s"));
EXTERN char e_cannot_declare_a_scope_variable_str[]
	INIT(= N_("E1016: Cannot declare a %s variable: %s"));
EXTERN char e_cannot_declare_an_environment_variable_str[]
	INIT(= N_("E1016: Cannot declare an environment variable: %s"));
EXTERN char e_variable_already_declared_str[]
	INIT(= N_("E1017: Variable already declared: %s"));
EXTERN char e_cannot_assign_to_constant_str[]
	INIT(= N_("E1018: Cannot assign to a constant: %s"));
EXTERN char e_can_only_concatenate_to_string[]
	INIT(= N_("E1019: Can only concatenate to string"));
EXTERN char e_cannot_use_operator_on_new_variable_str[]
	INIT(= N_("E1020: Cannot use an operator on a new variable: %s"));
EXTERN char e_const_requires_a_value[]
	INIT(= N_("E1021: Const requires a value"));
EXTERN char e_type_or_initialization_required[]
	INIT(= N_("E1022: Type or initialization required"));
EXTERN char e_using_number_as_bool_nr[]
	INIT(= N_("E1023: Using a Number as a Bool: %lld"));
EXTERN char e_using_number_as_string[]
	INIT(= N_("E1024: Using a Number as a String"));
EXTERN char e_using_rcurly_outside_if_block_scope[]
	INIT(= N_("E1025: Using } outside of a block scope"));
#endif
EXTERN char e_missing_rcurly[]
	INIT(= N_("E1026: Missing }"));
#ifdef FEAT_EVAL
EXTERN char e_missing_return_statement[]
	INIT(= N_("E1027: Missing return statement"));
EXTERN char e_compiling_def_function_failed[]
	INIT(= N_("E1028: Compiling :def function failed"));
EXTERN char e_expected_str_but_got_str[]
	INIT(= N_("E1029: Expected %s but got %s"));
EXTERN char e_using_string_as_number_str[]
	INIT(= N_("E1030: Using a String as a Number: \"%s\""));
EXTERN char e_cannot_use_void_value[]
	INIT(= N_("E1031: Cannot use void value"));
EXTERN char e_missing_catch_or_finally[]
	INIT(= N_("E1032: Missing :catch or :finally"));
EXTERN char e_catch_unreachable_after_catch_all[]
	INIT(= N_("E1033: Catch unreachable after catch-all"));
EXTERN char e_cannot_use_reserved_name_str[]
	INIT(= N_("E1034: Cannot use reserved name %s"));
EXTERN char e_percent_requires_number_arguments[]
	// xgettext:no-c-format
	INIT(= N_("E1035: % requires number arguments"));
EXTERN char e_char_requires_number_or_float_arguments[]
	INIT(= N_("E1036: %c requires number or float arguments"));
EXTERN char e_cannot_use_str_with_str[]
	INIT(= N_("E1037: Cannot use \"%s\" with %s"));
EXTERN char e_vim9script_can_only_be_used_in_script[]
	INIT(= N_("E1038: \"vim9script\" can only be used in a script"));
EXTERN char e_vim9script_must_be_first_command_in_script[]
	INIT(= N_("E1039: \"vim9script\" must be the first command in a script"));
#endif
EXTERN char e_cannot_use_scriptversion_after_vim9script[]
	INIT(= N_("E1040: Cannot use :scriptversion after :vim9script"));
#ifdef FEAT_EVAL
EXTERN char e_redefining_script_item_str[]
	INIT(= N_("E1041: Redefining script item: \"%s\""));
EXTERN char e_export_can_only_be_used_in_vim9script[]
	INIT(= N_("E1042: Export can only be used in vim9script"));
EXTERN char e_invalid_command_after_export[]
	INIT(= N_("E1043: Invalid command after :export"));
EXTERN char e_export_with_invalid_argument[]
	INIT(= N_("E1044: Export with invalid argument"));
// E1045 not used
// E1046 not used
EXTERN char e_syntax_error_in_import_str[]
	INIT(= N_("E1047: Syntax error in import: %s"));
EXTERN char e_item_not_found_in_script_str[]
	INIT(= N_("E1048: Item not found in script: %s"));
EXTERN char e_item_not_exported_in_script_str[]
	INIT(= N_("E1049: Item not exported in script: %s"));
#endif
EXTERN char e_colon_required_before_range_str[]
	INIT(= N_("E1050: Colon required before a range: %s"));
#ifdef FEAT_EVAL
EXTERN char e_wrong_argument_type_for_plus[]
	INIT(= N_("E1051: Wrong argument type for +"));
EXTERN char e_cannot_declare_an_option_str[]
	INIT(= N_("E1052: Cannot declare an option: %s"));
EXTERN char e_could_not_import_str[]
	INIT(= N_("E1053: Could not import \"%s\""));
EXTERN char e_variable_already_declared_in_script_str[]
	INIT(= N_("E1054: Variable already declared in the script: %s"));
EXTERN char e_missing_name_after_dots[]
	INIT(= N_("E1055: Missing name after ..."));
EXTERN char e_expected_type_str[]
	INIT(= N_("E1056: Expected a type: %s"));
EXTERN char e_missing_enddef[]
	INIT(= N_("E1057: Missing :enddef"));
EXTERN char e_function_nesting_too_deep[]
	INIT(= N_("E1058: Function nesting too deep"));
EXTERN char e_no_white_space_allowed_before_colon_str[]
	INIT(= N_("E1059: No white space allowed before colon: %s"));
EXTERN char e_expected_dot_after_name_str[]
	INIT(= N_("E1060: Expected dot after name: %s"));
EXTERN char e_cannot_find_function_str[]
	INIT(= N_("E1061: Cannot find function %s"));
EXTERN char e_cannot_index_number[]
	INIT(= N_("E1062: Cannot index a Number"));
EXTERN char e_type_mismatch_for_v_variable[]
	INIT(= N_("E1063: Type mismatch for v: variable"));
#endif
EXTERN char e_yank_register_changed_while_using_it[]
	INIT(= N_("E1064: Yank register changed while using it"));
EXTERN char e_command_cannot_be_shortened_str[]
	INIT(= N_("E1065: Command cannot be shortened: %s"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_declare_a_register_str[]
	INIT(= N_("E1066: Cannot declare a register: %s"));
EXTERN char e_separator_mismatch_str[]
	INIT(= N_("E1067: Separator mismatch: %s"));
EXTERN char e_no_white_space_allowed_before_str_str[]
	INIT(= N_("E1068: No white space allowed before '%s': %s"));
EXTERN char e_white_space_required_after_str_str[]
	INIT(= N_("E1069: White space required after '%s': %s"));
EXTERN char e_invalid_string_for_import_str[]
	INIT(= N_("E1071: Invalid string for :import: %s"));
EXTERN char e_cannot_compare_str_with_str[]
	INIT(= N_("E1072: Cannot compare %s with %s"));
EXTERN char e_name_already_defined_str[]
	INIT(= N_("E1073: Name already defined: %s"));
EXTERN char e_no_white_space_allowed_after_dot[]
	INIT(= N_("E1074: No white space allowed after dot"));
EXTERN char e_namespace_not_supported_str[]
	INIT(= N_("E1075: Namespace not supported: %s"));
// E1076 unused (was deleted)
EXTERN char e_missing_argument_type_for_str[]
	INIT(= N_("E1077: Missing argument type for %s"));
#endif
EXTERN char e_invalid_command_nested_did_you_mean_plusplus_nested[]
	INIT(= N_("E1078: Invalid command \"nested\", did you mean \"++nested\"?"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_declare_variable_on_command_line[]
	INIT(= N_("E1079: Cannot declare a variable on the command line"));
EXTERN char e_invalid_assignment[]
	INIT(= N_("E1080: Invalid assignment"));
EXTERN char e_cannot_unlet_str[]
	INIT(= N_("E1081: Cannot unlet %s"));
#endif
EXTERN char e_command_modifier_without_command[]
	INIT(= N_("E1082: Command modifier without command"));
#ifdef FEAT_EVAL
EXTERN char e_missing_backtick[]
	INIT(= N_("E1083: Missing backtick"));
EXTERN char e_cannot_delete_vim9_script_function_str[]
	INIT(= N_("E1084: Cannot delete Vim9 script function %s"));
EXTERN char e_not_callable_type_str[]
	INIT(= N_("E1085: Not a callable type: %s"));
// E1086 unused
EXTERN char e_cannot_use_index_when_declaring_variable[]
	INIT(= N_("E1087: Cannot use an index when declaring a variable"));
EXTERN char e_script_cannot_import_itself[]
	INIT(= N_("E1088: Script cannot import itself"));
EXTERN char e_unknown_variable_str[]
	INIT(= N_("E1089: Unknown variable: %s"));
EXTERN char e_cannot_assign_to_argument_str[]
	INIT(= N_("E1090: Cannot assign to argument %s"));
EXTERN char e_function_is_not_compiled_str[]
	INIT(= N_("E1091: Function is not compiled: %s"));
EXTERN char e_cannot_nest_redir[]
	INIT(= N_("E1092: Cannot nest :redir"));
EXTERN char e_expected_nr_items_but_got_nr[]
	INIT(= N_("E1093: Expected %d items but got %d"));
EXTERN char e_import_can_only_be_used_in_script[]
	INIT(= N_("E1094: Import can only be used in a script"));
EXTERN char e_unreachable_code_after_str[]
	INIT(= N_("E1095: Unreachable code after :%s"));
EXTERN char e_returning_value_in_function_without_return_type[]
	INIT(= N_("E1096: Returning a value in a function without a return type"));
EXTERN char e_line_incomplete[]
	INIT(= N_("E1097: Line incomplete"));
EXTERN char e_string_list_or_blob_required[]
	INIT(= N_("E1098: String, List or Blob required"));
EXTERN char e_unknown_error_while_executing_str[]
	INIT(= N_("E1099: Unknown error while executing %s"));
EXTERN char e_command_not_supported_in_vim9_script_missing_var_str[]
	INIT(= N_("E1100: Command not supported in Vim9 script (missing :var?): %s"));
EXTERN char e_cannot_declare_script_variable_in_function_str[]
	INIT(= N_("E1101: Cannot declare a script variable in a function: %s"));
EXTERN char e_lambda_function_not_found_str[]
	INIT(= N_("E1102: Lambda function not found: %s"));
EXTERN char e_dictionary_not_set[]
	INIT(= N_("E1103: Dictionary not set"));
EXTERN char e_missing_gt[]
	INIT(= N_("E1104: Missing >"));
EXTERN char e_cannot_convert_str_to_string[]
	INIT(= N_("E1105: Cannot convert %s to string"));

PLURAL_MSG(e_one_argument_too_many, "E1106: One argument too many",
		e_nr_arguments_too_many, "E1106: %d arguments too many")

EXTERN char e_string_list_dict_or_blob_required[]
	INIT(= N_("E1107: String, List, Dict or Blob required"));
// E1108 unused
EXTERN char e_list_item_nr_is_not_list[]
	INIT(= N_("E1109: List item %d is not a List"));
EXTERN char e_list_item_nr_does_not_contain_3_numbers[]
	INIT(= N_("E1110: List item %d does not contain 3 numbers"));
EXTERN char e_list_item_nr_range_invalid[]
	INIT(= N_("E1111: List item %d range invalid"));
EXTERN char e_list_item_nr_cell_width_invalid[]
	INIT(= N_("E1112: List item %d cell width invalid"));
EXTERN char e_overlapping_ranges_for_nr[]
	INIT(= N_("E1113: Overlapping ranges for 0x%lx"));
EXTERN char e_only_values_of_0x80_and_higher_supported[]
	INIT(= N_("E1114: Only values of 0x80 and higher supported"));
EXTERN char e_assert_fails_fourth_argument[]
	INIT(= N_("E1115: \"assert_fails()\" fourth argument must be a number"));
EXTERN char e_assert_fails_fifth_argument[]
	INIT(= N_("E1116: \"assert_fails()\" fifth argument must be a string"));
EXTERN char e_cannot_use_bang_with_nested_def[]
	INIT(= N_("E1117: Cannot use ! with nested :def"));
EXTERN char e_cannot_change_locked_list[]
	INIT(= N_("E1118: Cannot change locked list"));
EXTERN char e_cannot_change_locked_list_item[]
	INIT(= N_("E1119: Cannot change locked list item"));
EXTERN char e_cannot_change_dict[]
	INIT(= N_("E1120: Cannot change dict"));
EXTERN char e_cannot_change_dict_item[]
	INIT(= N_("E1121: Cannot change dict item"));
EXTERN char e_variable_is_locked_str[]
	INIT(= N_("E1122: Variable is locked: %s"));
EXTERN char e_missing_comma_before_argument_str[]
	INIT(= N_("E1123: Missing comma before argument: %s"));
EXTERN char e_str_cannot_be_used_in_legacy_vim_script[]
	INIT(= N_("E1124: \"%s\" cannot be used in legacy Vim script"));
EXTERN char e_final_requires_a_value[]
	INIT(= N_("E1125: Final requires a value"));
EXTERN char e_cannot_use_let_in_vim9_script[]
	INIT(= N_("E1126: Cannot use :let in Vim9 script"));
EXTERN char e_missing_name_after_dot[]
	INIT(= N_("E1127: Missing name after dot"));
EXTERN char e_endblock_without_block[]
	INIT(= N_("E1128: } without {"));
EXTERN char e_throw_with_empty_string[]
	INIT(= N_("E1129: Throw with empty string"));
EXTERN char e_cannot_add_to_null_list[]
	INIT(= N_("E1130: Cannot add to null list"));
EXTERN char e_cannot_add_to_null_blob[]
	INIT(= N_("E1131: Cannot add to null blob"));
EXTERN char e_missing_function_argument[]
	INIT(= N_("E1132: Missing function argument"));
EXTERN char e_cannot_extend_null_dict[]
	INIT(= N_("E1133: Cannot extend a null dict"));
EXTERN char e_cannot_extend_null_list[]
	INIT(= N_("E1134: Cannot extend a null list"));
EXTERN char e_using_string_as_bool_str[]
	INIT(= N_("E1135: Using a String as a Bool: \"%s\""));
#endif
EXTERN char e_cmd_mapping_must_end_with_cr_before_second_cmd[]
	INIT(= N_("E1136: <Cmd> mapping must end with <CR> before second <Cmd>"));
// E1137 unused
#ifdef FEAT_EVAL
EXTERN char e_using_bool_as_number[]
	INIT(= N_("E1138: Using a Bool as a Number"));
EXTERN char e_missing_matching_bracket_after_dict_key[]
	INIT(= N_("E1139: Missing matching bracket after dict key"));
EXTERN char e_for_argument_must_be_sequence_of_lists[]
	INIT(= N_("E1140: :for argument must be a sequence of lists"));
EXTERN char e_indexable_type_required[]
	INIT(= N_("E1141: Indexable type required"));
EXTERN char e_calling_test_garbagecollect_now_while_v_testing_is_not_set[]
	INIT(= N_("E1142: Calling test_garbagecollect_now() while v:testing is not set"));
EXTERN char e_empty_expression_str[]
	INIT(= N_("E1143: Empty expression: \"%s\""));
EXTERN char e_command_str_not_followed_by_white_space_str[]
	INIT(= N_("E1144: Command \"%s\" is not followed by white space: %s"));
EXTERN char e_missing_heredoc_end_marker_str[]
	INIT(= N_("E1145: Missing heredoc end marker: %s"));
EXTERN char e_command_not_recognized_str[]
	INIT(= N_("E1146: Command not recognized: %s"));
EXTERN char e_list_not_set[]
	INIT(= N_("E1147: List not set"));
EXTERN char e_cannot_index_str[]
	INIT(= N_("E1148: Cannot index a %s"));
EXTERN char e_script_variable_invalid_after_reload_in_function_str[]
	INIT(= N_("E1149: Script variable is invalid after reload in function %s"));
EXTERN char e_script_variable_type_changed[]
	INIT(= N_("E1150: Script variable type changed"));
EXTERN char e_mismatched_endfunction[]
	INIT(= N_("E1151: Mismatched endfunction"));
EXTERN char e_mismatched_enddef[]
	INIT(= N_("E1152: Mismatched enddef"));
EXTERN char e_invalid_operation_for_str[]
	INIT(= N_("E1153: Invalid operation for %s"));
EXTERN char e_divide_by_zero[]
	INIT(= N_("E1154: Divide by zero"));
#endif
EXTERN char e_cannot_define_autocommands_for_all_events[]
	INIT(= N_("E1155: Cannot define autocommands for ALL events"));
EXTERN char e_cannot_change_arglist_recursively[]
	INIT(= N_("E1156: Cannot change the argument list recursively"));
#ifdef FEAT_EVAL
EXTERN char e_missing_return_type[]
	INIT(= N_("E1157: Missing return type"));
EXTERN char e_cannot_use_flatten_in_vim9_script[]
	INIT(= N_("E1158: Cannot use flatten() in Vim9 script, use flattennew()"));
#endif
EXTERN char e_cannot_split_window_when_closing_buffer[]
	INIT(= N_("E1159: Cannot split a window when closing the buffer"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_default_for_variable_arguments[]
	INIT(= N_("E1160: Cannot use a default for variable arguments"));
EXTERN char e_cannot_json_encode_str[]
	INIT(= N_("E1161: Cannot json encode a %s"));
EXTERN char e_register_name_must_be_one_char_str[]
	INIT(= N_("E1162: Register name must be one character: %s"));
EXTERN char e_variable_nr_type_mismatch_expected_str_but_got_str[]
	INIT(= N_("E1163: Variable %d: type mismatch, expected %s but got %s"));
EXTERN char e_variable_nr_type_mismatch_expected_str_but_got_str_in_str[]
	INIT(= N_("E1163: Variable %d: type mismatch, expected %s but got %s in %s"));
#endif
EXTERN char e_vim9cmd_must_be_followed_by_command[]
	INIT(= N_("E1164: vim9cmd must be followed by a command"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_range_with_assignment_str[]
	INIT(= N_("E1165: Cannot use a range with an assignment: %s"));
EXTERN char e_cannot_use_range_with_dictionary[]
	INIT(= N_("E1166: Cannot use a range with a dictionary"));
EXTERN char e_argument_name_shadows_existing_variable_str[]
	INIT(= N_("E1167: Argument name shadows existing variable: %s"));
EXTERN char e_argument_already_declared_in_script_str[]
	INIT(= N_("E1168: Argument already declared in the script: %s"));
EXTERN char e_expression_too_recursive_str[]
	INIT(= N_("E1169: Expression too recursive: %s"));
EXTERN char e_cannot_use_hash_curly_to_start_comment[]
	INIT(= N_("E1170: Cannot use #{ to start a comment"));
EXTERN char e_missing_end_block[]
	INIT(= N_("E1171: Missing } after inline function"));
EXTERN char e_cannot_use_default_values_in_lambda[]
	INIT(= N_("E1172: Cannot use default values in a lambda"));
EXTERN char e_text_found_after_str_str[]
	INIT(= N_("E1173: Text found after %s: %s"));
EXTERN char e_string_required_for_argument_nr[]
	INIT(= N_("E1174: String required for argument %d"));
EXTERN char e_non_empty_string_required_for_argument_nr[]
	INIT(= N_("E1175: Non-empty string required for argument %d"));
EXTERN char e_misplaced_command_modifier[]
	INIT(= N_("E1176: Misplaced command modifier"));
EXTERN char e_for_loop_on_str_not_supported[]
	INIT(= N_("E1177: For loop on %s not supported"));
EXTERN char e_cannot_lock_unlock_local_variable[]
	INIT(= N_("E1178: Cannot lock or unlock a local variable"));
#endif
#ifdef FEAT_TERMINAL
EXTERN char e_failed_to_extract_pwd_from_str_check_your_shell_config[]
	INIT(= N_("E1179: Failed to extract PWD from %s, check your shell's config related to OSC 7"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_variable_arguments_type_must_be_list_str[]
	INIT(= N_("E1180: Variable arguments type must be a list: %s"));
EXTERN char e_cannot_use_underscore_here[]
	INIT(= N_("E1181: Cannot use an underscore here"));
EXTERN char e_cannot_define_dict_func_in_vim9_script_str[]
	INIT(= N_("E1182: Cannot define a dict function in Vim9 script: %s"));
EXTERN char e_cannot_use_range_with_assignment_operator_str[]
	INIT(= N_("E1183: Cannot use a range with an assignment operator: %s"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_blob_not_set[]
	INIT(= N_("E1184: Blob not set"));
EXTERN char e_missing_redir_end[]
	INIT(= N_("E1185: Missing :redir END"));
EXTERN char e_expression_does_not_result_in_value_str[]
	INIT(= N_("E1186: Expression does not result in a value: %s"));
#endif
EXTERN char e_failed_to_source_defaults[]
	INIT(= N_("E1187: Failed to source defaults.vim"));
#if defined(FEAT_TERMINAL)
EXTERN char e_cannot_open_terminal_from_command_line_window[]
	INIT(= N_("E1188: Cannot open a terminal from the command line window"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_legacy_with_command_str[]
	INIT(= N_("E1189: Cannot use :legacy with this command: %s"));

PLURAL_MSG(e_one_argument_too_few, "E1190: One argument too few",
		e_nr_arguments_too_few, "E1190: %d arguments too few")

EXTERN char e_call_to_function_that_failed_to_compile_str[]
	INIT(= N_("E1191: Call to function that failed to compile: %s"));
EXTERN char e_empty_function_name[]
	INIT(= N_("E1192: Empty function name"));
#endif
// libsodium
#ifdef FEAT_CRYPT
# ifndef FEAT_SODIUM
EXTERN char e_libsodium_not_built_in[]
	INIT(= N_("E1193: cryptmethod xchacha20 not built into this Vim"));
# else
#  if 0
EXTERN char e_libsodium_cannot_encrypt_header[]
	INIT(= N_("E1194: Cannot encrypt header, not enough space"));
EXTERN char e_libsodium_cannot_encrypt_buffer[]
	INIT(= N_("E1195: Cannot encrypt buffer, not enough space"));
EXTERN char e_libsodium_cannot_decrypt_header[]
	INIT(= N_("E1196: Cannot decrypt header, not enough space"));
#  endif
EXTERN char e_libsodium_cannot_allocate_buffer[]
	INIT(= N_("E1197: Cannot allocate_buffer for encryption"));
EXTERN char e_libsodium_decryption_failed_header_incomplete[]
	INIT(= N_("E1198: Decryption failed: Header incomplete!"));
#  if 0
EXTERN char e_libsodium_cannot_decrypt_buffer[]
	INIT(= N_("E1199: Cannot decrypt buffer, not enough space"));
#  endif
EXTERN char e_libsodium_decryption_failed[]
	INIT(= N_("E1200: Decryption failed!"));
EXTERN char e_libsodium_decryption_failed_premature[]
	INIT(= N_("E1201: Decryption failed: pre-mature end of file!"));
# endif
#endif
#ifdef FEAT_EVAL
EXTERN char e_no_white_space_allowed_after_str_str[]
	INIT(= N_("E1202: No white space allowed after '%s': %s"));
EXTERN char e_dot_not_allowed_after_str_str[]
	INIT(= N_("E1203: Dot not allowed after a %s: %s"));
#endif
EXTERN char e_regexp_number_after_dot_pos_search_chr[]
	INIT(= N_("E1204: No Number allowed after .: '\\%%%c'"));
EXTERN char e_no_white_space_allowed_between_option_and[]
	INIT(= N_("E1205: No white space allowed between option and"));
#ifdef FEAT_EVAL
EXTERN char e_dict_required_for_argument_nr[]
	INIT(= N_("E1206: Dictionary required for argument %d"));
EXTERN char e_expression_without_effect_str[]
	INIT(= N_("E1207: Expression without an effect: %s"));
#endif
EXTERN char e_complete_used_without_allowing_arguments[]
	INIT(= N_("E1208: -complete used without allowing arguments"));
#ifdef FEAT_EVAL
EXTERN char e_invalid_value_for_line_number_str[]
	INIT(= N_("E1209: Invalid value for a line number: \"%s\""));
EXTERN char e_number_required_for_argument_nr[]
	INIT(= N_("E1210: Number required for argument %d"));
EXTERN char e_list_required_for_argument_nr[]
	INIT(= N_("E1211: List required for argument %d"));
EXTERN char e_bool_required_for_argument_nr[]
	INIT(= N_("E1212: Bool required for argument %d"));
EXTERN char e_redefining_imported_item_str[]
	INIT(= N_("E1213: Redefining imported item \"%s\""));
#endif
#if defined(FEAT_DIGRAPHS)
EXTERN char e_digraph_must_be_just_two_characters_str[]
	INIT(= N_("E1214: Digraph must be just two characters: %s"));
EXTERN char e_digraph_argument_must_be_one_character_str[]
	INIT(= N_("E1215: Digraph must be one character: %s"));
EXTERN char e_digraph_setlist_argument_must_be_list_of_lists_with_two_items[]
	INIT(= N_("E1216: digraph_setlist() argument must be a list of lists with two items"));
#endif
#ifdef FEAT_EVAL
# ifdef FEAT_JOB_CHANNEL
EXTERN char e_chan_or_job_required_for_argument_nr[]
	INIT(= N_("E1217: Channel or Job required for argument %d"));
EXTERN char e_job_required_for_argument_nr[]
	INIT(= N_("E1218: Job required for argument %d"));
# endif
EXTERN char e_float_or_number_required_for_argument_nr[]
	INIT(= N_("E1219: Float or Number required for argument %d"));
EXTERN char e_string_or_number_required_for_argument_nr[]
	INIT(= N_("E1220: String or Number required for argument %d"));
# ifdef FEAT_JOB_CHANNEL
EXTERN char e_string_or_blob_required_for_argument_nr[]
	INIT(= N_("E1221: String or Blob required for argument %d"));
# endif
EXTERN char e_string_or_list_required_for_argument_nr[]
	INIT(= N_("E1222: String or List required for argument %d"));
EXTERN char e_string_or_dict_required_for_argument_nr[]
	INIT(= N_("E1223: String or Dictionary required for argument %d"));
EXTERN char e_string_number_or_list_required_for_argument_nr[]
	INIT(= N_("E1224: String, Number or List required for argument %d"));
EXTERN char e_string_list_or_dict_required_for_argument_nr[]
	INIT(= N_("E1225: String, List or Dictionary required for argument %d"));
EXTERN char e_list_or_blob_required_for_argument_nr[]
	INIT(= N_("E1226: List or Blob required for argument %d"));
EXTERN char e_list_or_dict_required_for_argument_nr[]
	INIT(= N_("E1227: List or Dictionary required for argument %d"));
EXTERN char e_list_dict_or_blob_required_for_argument_nr[]
	INIT(= N_("E1228: List, Dictionary or Blob required for argument %d"));
EXTERN char e_expected_dictionary_for_using_key_str_but_got_str[]
	INIT(= N_("E1229: Expected dictionary for using key \"%s\", but got %s"));
#endif
#ifdef FEAT_SODIUM
EXTERN char e_encryption_sodium_mlock_failed[]
	INIT(= N_("E1230: Encryption: sodium_mlock() failed"));
#endif
EXTERN char e_cannot_use_bar_to_separate_commands_here_str[]
	INIT(= N_("E1231: Cannot use a bar to separate commands here: %s"));
#ifdef FEAT_EVAL
EXTERN char e_argument_of_exists_compiled_must_be_literal_string[]
	INIT(= N_("E1232: Argument of exists_compiled() must be a literal string"));
EXTERN char e_exists_compiled_can_only_be_used_in_def_function[]
	INIT(= N_("E1233: exists_compiled() can only be used in a :def function"));
#endif
EXTERN char e_legacy_must_be_followed_by_command[]
	INIT(= N_("E1234: legacy must be followed by a command"));
#ifdef FEAT_EVAL
// E1235 unused
EXTERN char e_cannot_use_str_itself_it_is_imported[]
	INIT(= N_("E1236: Cannot use %s itself, it is imported"));
#endif
EXTERN char e_no_such_user_defined_command_in_current_buffer_str[]
	INIT(= N_("E1237: No such user-defined command in current buffer: %s"));
#ifdef FEAT_EVAL
EXTERN char e_blob_required_for_argument_nr[]
	INIT(= N_("E1238: Blob required for argument %d"));
EXTERN char e_invalid_value_for_blob_nr[]
	INIT(= N_("E1239: Invalid value for blob: %d"));
#endif
EXTERN char e_resulting_text_too_long[]
	INIT(= N_("E1240: Resulting text too long"));
#ifdef FEAT_EVAL
EXTERN char e_separator_not_supported_str[]
	INIT(= N_("E1241: Separator not supported: %s"));
EXTERN char e_no_white_space_allowed_before_separator_str[]
	INIT(= N_("E1242: No white space allowed before separator: %s"));
#endif
#ifdef FEAT_GUI_GTK
EXTERN char e_ascii_code_not_in_range[]
	INIT(= N_("E1243: ASCII code not in 32-127 range"));
#endif
#ifdef FEAT_EVAL
# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
EXTERN char e_bad_color_string_str[]
	INIT(= N_("E1244: Bad color string: %s"));
# endif
EXTERN char e_cannot_expand_sfile_in_vim9_function[]
	INIT(= N_("E1245: Cannot expand <sfile> in a Vim9 function"));
EXTERN char e_cannot_find_variable_to_unlock_str[]
	INIT(= N_("E1246: Cannot find variable to (un)lock: %s"));
#endif
EXTERN char e_line_number_out_of_range[]
	INIT(= N_("E1247: Line number out of range"));
#ifdef FEAT_EVAL
EXTERN char e_closure_called_from_invalid_context[]
	INIT(= N_("E1248: Closure called from invalid context"));
#endif
EXTERN char e_highlight_group_name_too_long[]
	INIT(= N_("E1249: Highlight group name too long"));
#ifdef FEAT_EVAL
EXTERN char e_argument_of_str_must_be_list_string_dictionary_or_blob[]
	INIT(= N_("E1250: Argument of %s must be a List, String, Dictionary or Blob"));
EXTERN char e_list_dict_blob_or_string_required_for_argument_nr[]
	INIT(= N_("E1251: List, Dictionary, Blob or String required for argument %d"));
EXTERN char e_string_list_or_blob_required_for_argument_nr[]
	INIT(= N_("E1252: String, List or Blob required for argument %d"));
// E1253 unused
EXTERN char e_cannot_use_script_variable_in_for_loop[]
	INIT(= N_("E1254: Cannot use script variable in for loop"));
#endif
EXTERN char e_cmd_mapping_must_end_with_cr[]
	INIT(= N_("E1255: <Cmd> mapping must end with <CR>"));
#ifdef FEAT_EVAL
EXTERN char e_string_or_function_required_for_argument_nr[]
	INIT(= N_("E1256: String or function required for argument %d"));
EXTERN char e_imported_script_must_use_as_or_end_in_dot_vim_str[]
	INIT(= N_("E1257: Imported script must use \"as\" or end in .vim: %s"));
EXTERN char e_no_dot_after_imported_name_str[]
	INIT(= N_("E1258: No '.' after imported name: %s"));
EXTERN char e_missing_name_after_imported_name_str[]
	INIT(= N_("E1259: Missing name after imported name: %s"));
EXTERN char e_cannot_unlet_imported_item_str[]
	INIT(= N_("E1260: Cannot unlet an imported item: %s"));
EXTERN char e_cannot_import_dot_vim_without_using_as[]
	INIT(= N_("E1261: Cannot import .vim without using \"as\""));
EXTERN char e_cannot_import_same_script_twice_str[]
	INIT(= N_("E1262: Cannot import the same script twice: %s"));
EXTERN char e_cannot_use_name_with_hash_in_vim9_script_use_export_instead[]
	INIT(= N_("E1263: Cannot use name with # in Vim9 script, use export instead"));
EXTERN char e_autoload_import_cannot_use_absolute_or_relative_path[]
	INIT(= N_("E1264: Autoload import cannot use absolute or relative path: %s"));
EXTERN char e_cannot_use_partial_here[]
	INIT(= N_("E1265: Cannot use a partial here"));
#endif
#if defined(FEAT_PYTHON3) && defined(MSWIN)
EXTERN char e_critical_error_in_python3_initialization_check_your_installation[]
	INIT(= N_("E1266: Critical error in python3 initialization, check your python3 installation"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_function_name_must_start_with_capital_str[]
	INIT(= N_("E1267: Function name must start with a capital: %s"));
EXTERN char e_cannot_use_s_colon_in_vim9_script_str[]
	INIT(= N_("E1268: Cannot use s: in Vim9 script: %s"));
EXTERN char e_cannot_create_vim9_script_variable_in_function_str[]
	INIT(= N_("E1269: Cannot create a Vim9 script variable in a function: %s"));
#endif
EXTERN char e_cannot_use_s_backslash_in_vim9_script[]
	INIT(= N_("E1270: Cannot use :s\\/sub/ in Vim9 script"));
#ifdef FEAT_EVAL
EXTERN char e_compiling_closure_without_context_str[]
	INIT(= N_("E1271: Compiling closure without context: %s"));
EXTERN char e_using_type_not_in_script_context_str[]
	INIT(= N_("E1272: Using type not in a script context: %s"));
#endif
EXTERN char e_nfa_regexp_missing_value_in_chr[]
	INIT(= N_("E1273: (NFA regexp) missing value in '\\%%%c'"));
EXTERN char e_no_script_file_name_to_substitute_for_script[]
	INIT(= N_("E1274: No script file name to substitute for \"<script>\""));
#ifdef FEAT_EVAL
EXTERN char e_string_or_function_required_for_arrow_parens_expr[]
	INIT(= N_("E1275: String or function required for ->(expr)"));
EXTERN char e_illegal_map_mode_string_str[]
	INIT(= N_("E1276: Illegal map mode string: '%s'"));
# if !defined(FEAT_JOB_CHANNEL)
EXTERN char e_channel_job_feature_not_available[]
	INIT(= N_("E1277: Channel and job feature is not available"));
# endif
EXTERN char e_stray_closing_curly_str[]
	INIT(= N_("E1278: Stray '}' without a matching '{': %s"));
EXTERN char e_missing_close_curly_str[]
	INIT(= N_("E1279: Missing '}': %s"));
#endif
#ifdef FEAT_SPELL
EXTERN char e_illegal_character_in_word[]
	INIT(= N_("E1280: Illegal character in word"));
#endif
EXTERN char e_atom_engine_must_be_at_start_of_pattern[]
	INIT(= N_("E1281: Atom '\\%%#=%c' must be at the start of the pattern"));
#ifdef FEAT_EVAL
EXTERN char e_bitshift_ops_must_be_number[]
	INIT(= N_("E1282: Bitshift operands must be numbers"));
EXTERN char e_bitshift_ops_must_be_positive[]
	INIT(= N_("E1283: Bitshift amount must be a positive number"));
#endif
#if defined(FEAT_PROP_POPUP)
EXTERN char e_argument_1_list_item_nr_dictionary_required[]
	INIT(= N_("E1284: Argument 1, list item %d: Dictionary required"));
#endif
#ifdef FEAT_RELTIME
EXTERN char e_could_not_clear_timeout_str[]
	INIT(= N_("E1285: Could not clear timeout: %s"));
EXTERN char e_could_not_set_timeout_str[]
	INIT(= N_("E1286: Could not set timeout: %s"));
#ifndef PROF_NSEC
EXTERN char e_could_not_set_handler_for_timeout_str[]
	INIT(= N_("E1287: Could not set handler for timeout: %s"));
EXTERN char e_could_not_reset_handler_for_timeout_str[]
	INIT(= N_("E1288: Could not reset handler for timeout: %s"));
EXTERN char e_could_not_check_for_pending_sigalrm_str[]
	INIT(= N_("E1289: Could not check for pending SIGALRM: %s"));
#endif
#endif
#ifdef FEAT_EVAL
EXTERN char e_substitute_nesting_too_deep[]
	INIT(= N_("E1290: substitute nesting too deep"));
EXTERN char e_invalid_argument_nr[]
	INIT(= N_("E1291: Invalid argument: %ld"));
#endif
EXTERN char e_cmdline_window_already_open[]
	INIT(= N_("E1292: Command-line window is already open"));
#ifdef FEAT_PROP_POPUP
EXTERN char e_cannot_use_negative_id_after_adding_textprop_with_text[]
	INIT(= N_("E1293: Cannot use a negative id after adding a textprop with text"));
EXTERN char e_can_only_use_text_align_when_column_is_zero[]
	INIT(= N_("E1294: Can only use text_align when column is zero"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_cannot_specify_both_type_and_types[]
	INIT(= N_("E1295: Cannot specify both 'type' and 'types'"));
EXTERN char e_can_only_use_left_padding_when_column_is_zero[]
	INIT(= N_("E1296: Can only use left padding when column is zero"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_non_null_dict_required_for_argument_nr[]
	INIT(= N_("E1297: Non-NULL Dictionary required for argument %d"));
EXTERN char e_non_null_list_required_for_argument_nr[]
	INIT(= N_("E1298: Non-NULL List required for argument %d"));
#endif
EXTERN char e_window_unexpectedly_close_while_searching_for_tags[]
	INIT(= N_("E1299: Window unexpectedly closed while searching for tags"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_partial_with_dictionary_for_defer[]
	INIT(= N_("E1300: Cannot use a partial with dictionary for :defer"));
EXTERN char e_string_number_list_or_blob_required_for_argument_nr[]
	INIT(= N_("E1301: String, Number, List or Blob required for argument %d"));
EXTERN char e_script_variable_was_deleted[]
	INIT(= N_("E1302: Script variable was deleted"));
EXTERN char e_custom_list_completion_function_does_not_return_list_but_str[]
	INIT(= N_("E1303: Custom list completion function does not return a List but a %s"));
EXTERN char e_cannot_use_type_with_this_variable_str[]
	INIT(= N_("E1304: Cannot use type with this variable: %s"));
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_cannot_use_length_endcol_and_endlnum_with_text[]
	INIT(= N_("E1305: Cannot use \"length\", \"end_col\" and \"end_lnum\" with \"text\""));
#endif
#ifdef FEAT_EVAL
EXTERN char e_loop_nesting_too_deep[]
	INIT(= N_("E1306: Loop nesting too deep"));
EXTERN char e_argument_nr_trying_to_modify_const_str[]
	INIT(= N_("E1307: Argument %d: Trying to modify a const %s"));
EXTERN char e_cannot_resize_window_in_another_tab_page[]
	INIT(= N_("E1308: Cannot resize a window in another tab page"));
#endif
EXTERN char e_cannot_change_mappings_while_listing[]
	INIT(= N_("E1309: Cannot change mappings while listing"));
#if defined(FEAT_MENU)
EXTERN char e_cannot_change_menus_while_listing[]
	INIT(= N_("E1310: Cannot change menus while listing"));
#endif
EXTERN char e_cannot_change_user_commands_while_listing[]
	INIT(= N_("E1311: Cannot change user commands while listing"));
EXTERN char e_not_allowed_to_change_window_layout_in_this_autocmd[]
	INIT(= N_("E1312: Not allowed to change the window layout in this autocmd"));
EXTERN char e_not_allowed_to_add_or_remove_entries_str[]
	INIT(= N_("E1313: Not allowed to add or remove entries (%s)"));
#ifdef FEAT_EVAL
EXTERN char e_class_name_must_start_with_uppercase_letter_str[]
	INIT(= N_("E1314: Class name must start with an uppercase letter: %s"));
EXTERN char e_white_space_required_after_name_str[]
	INIT(= N_("E1315: White space required after name: %s"));
EXTERN char e_class_can_only_be_defined_in_vim9_script[]
	INIT(= N_("E1316: Class can only be defined in Vim9 script"));
EXTERN char e_invalid_object_variable_declaration_str[]
	INIT(= N_("E1317: Invalid object variable declaration: %s"));
EXTERN char e_not_valid_command_in_class_str[]
	INIT(= N_("E1318: Not a valid command in a class: %s"));
EXTERN char e_using_class_as_number[]
	INIT(= N_("E1319: Using a Class as a Number"));
EXTERN char e_using_object_as_number[]
	INIT(= N_("E1320: Using an Object as a Number"));
EXTERN char e_using_class_as_float[]
	INIT(= N_("E1321: Using a Class as a Float"));
EXTERN char e_using_object_as_float[]
	INIT(= N_("E1322: Using an Object as a Float"));
EXTERN char e_using_class_as_string[]
	INIT(= N_("E1323: Using a Class as a String"));
EXTERN char e_using_object_as_string[]
	INIT(= N_("E1324: Using an Object as a String"));
EXTERN char e_method_not_found_on_class_str_str[]
	INIT(= N_("E1325: Method \"%s\" not found in class \"%s\""));
EXTERN char e_variable_not_found_on_object_str_str[]
	INIT(= N_("E1326: Variable \"%s\" not found in object \"%s\""));
EXTERN char e_object_required_found_str[]
	INIT(= N_("E1327: Object required, found %s"));
EXTERN char e_constructor_default_value_must_be_vnone_str[]
	INIT(= N_("E1328: Constructor default value must be v:none: %s"));
EXTERN char e_invalid_class_variable_declaration_str[]
	INIT(= N_("E1329: Invalid class variable declaration: %s"));
EXTERN char e_invalid_type_for_object_variable_str[]
	INIT(= N_("E1330: Invalid type for object variable: %s"));
EXTERN char e_public_must_be_followed_by_var_static_final_or_const[]
	INIT(= N_("E1331: Public must be followed by \"var\" or \"static\" or \"final\" or \"const\""));
EXTERN char e_public_variable_name_cannot_start_with_underscore_str[]
	INIT(= N_("E1332: Public variable name cannot start with underscore: %s"));
EXTERN char e_cannot_access_protected_variable_str[]
	INIT(= N_("E1333: Cannot access protected variable \"%s\" in class \"%s\""));
// E1334 unused
EXTERN char e_variable_is_not_writable_str[]
	INIT(= N_("E1335: Variable \"%s\" in class \"%s\" is not writable"));
#endif
EXTERN char e_internal_error_shortmess_too_long[]
	INIT(= "E1336: Internal error: shortmess too long");
#ifdef FEAT_EVAL
EXTERN char e_class_variable_str_not_found_in_class_str[]
	INIT(= N_("E1337: Class variable \"%s\" not found in class \"%s\""));
// E1338 unused
#endif
#ifdef FEAT_PROP_POPUP
EXTERN char e_cannot_add_textprop_with_text_after_using_textprop_with_negative_id[]
	INIT(= N_("E1339: Cannot add a textprop with text after using a textprop with a negative id"));
#endif
#ifdef FEAT_EVAL
EXTERN char e_argument_already_declared_in_class_str[]
	INIT(= N_("E1340: Argument already declared in the class: %s"));
EXTERN char e_variable_already_declared_in_class_str[]
	INIT(= N_("E1341: Variable already declared in the class: %s"));
EXTERN char e_interface_can_only_be_defined_in_vim9_script[]
	INIT(= N_("E1342: Interface can only be defined in Vim9 script"));
EXTERN char e_interface_name_must_start_with_uppercase_letter_str[]
	INIT(= N_("E1343: Interface name must start with an uppercase letter: %s"));
EXTERN char e_cannot_initialize_variable_in_interface[]
	INIT(= N_("E1344: Cannot initialize a variable in an interface"));
EXTERN char e_not_valid_command_in_interface_str[]
	INIT(= N_("E1345: Not a valid command in an interface: %s"));
EXTERN char e_interface_name_not_found_str[]
	INIT(= N_("E1346: Interface name not found: %s"));
EXTERN char e_not_valid_interface_str[]
	INIT(= N_("E1347: Not a valid interface: %s"));
EXTERN char e_variable_str_of_interface_str_not_implemented[]
	INIT(= N_("E1348: Variable \"%s\" of interface \"%s\" is not implemented"));
EXTERN char e_method_str_of_interface_str_not_implemented[]
	INIT(= N_("E1349: Method \"%s\" of interface \"%s\" is not implemented"));
EXTERN char e_duplicate_implements[]
	INIT(= N_("E1350: Duplicate \"implements\""));
EXTERN char e_duplicate_interface_after_implements_str[]
	INIT(= N_("E1351: Duplicate interface after \"implements\": %s"));
EXTERN char e_duplicate_extends[]
	INIT(= N_("E1352: Duplicate \"extends\""));
EXTERN char e_class_name_not_found_str[]
	INIT(= N_("E1353: Class name not found: %s"));
EXTERN char e_cannot_extend_str[]
	INIT(= N_("E1354: Cannot extend %s"));
EXTERN char e_duplicate_function_str[]
	INIT(= N_("E1355: Duplicate function: %s"));
EXTERN char e_super_must_be_followed_by_dot[]
	INIT(= N_("E1356: \"super\" must be followed by a dot"));
EXTERN char e_using_super_not_in_class_method[]
	INIT(= N_("E1357: Using \"super\" not in a class method"));
EXTERN char e_using_super_not_in_child_class[]
	INIT(= N_("E1358: Using \"super\" not in a child class"));
EXTERN char e_cannot_define_new_method_in_abstract_class[]
	INIT(= N_("E1359: Cannot define a \"new\" method in an abstract class"));
EXTERN char e_using_null_object[]
	INIT(= N_("E1360: Using a null object"));
#endif
EXTERN char e_cannot_use_color_none_did_you_mean_none[]
	INIT(= N_("E1361: Cannot use color \"none\", did you mean \"NONE\"?"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_non_null_object[]
	INIT(= N_("E1362: Cannot use a non-null object"));
EXTERN char e_incomplete_type[]
	INIT(= N_("E1363: Incomplete type"));
#endif
EXTERN char e_warning_pointer_block_corrupted[]
	INIT(= N_("E1364: Warning: Pointer block corrupted"));
#ifdef FEAT_EVAL
EXTERN char e_cannot_use_a_return_type_with_new_method[]
	INIT(= N_("E1365: Cannot use a return type with the \"new\" method"));
EXTERN char e_cannot_access_protected_method_str[]
	INIT(= N_("E1366: Cannot access protected method: %s"));
EXTERN char e_variable_str_of_interface_str_has_different_access[]
	INIT(= N_("E1367: Access level of variable \"%s\" of interface \"%s\" is different"));
EXTERN char e_static_must_be_followed_by_var_def_final_or_const[]
	INIT(= N_("E1368: Static must be followed by \"var\" or \"def\" or \"final\" or \"const\""));
EXTERN char e_duplicate_variable_str[]
	INIT(= N_("E1369: Duplicate variable: %s"));
EXTERN char e_cannot_define_new_method_as_static[]
	INIT(= N_("E1370: Cannot define a \"new\" method as static"));
EXTERN char e_abstract_must_be_followed_by_def[]
	INIT(= N_("E1371: Abstract must be followed by \"def\""));
EXTERN char e_abstract_method_in_concrete_class[]
	INIT(= N_("E1372: Abstract method \"%s\" cannot be defined in a concrete class"));
EXTERN char e_abstract_method_str_not_found[]
	INIT(= N_("E1373: Abstract method \"%s\" is not implemented"));
EXTERN char e_class_variable_str_accessible_only_inside_class_str[]
	INIT(= N_("E1374: Class variable \"%s\" accessible only inside class \"%s\""));
EXTERN char e_class_variable_str_accessible_only_using_class_str[]
	INIT(= N_("E1375: Class variable \"%s\" accessible only using class \"%s\""));
EXTERN char e_object_variable_str_accessible_only_using_object_str[]
	INIT(= N_("E1376: Object variable \"%s\" accessible only using class \"%s\" object"));
EXTERN char e_method_str_of_class_str_has_different_access[]
	INIT(= N_("E1377: Access level of method \"%s\" is different in class \"%s\""));
EXTERN char e_static_member_not_supported_in_interface[]
	INIT(= N_("E1378: Static member not supported in an interface"));
EXTERN char e_protected_variable_not_supported_in_interface[]
	INIT(= N_("E1379: Protected variable not supported in an interface"));
EXTERN char e_protected_method_not_supported_in_interface[]
	INIT(= N_("E1380: Protected method not supported in an interface"));
EXTERN char e_interface_cannot_use_implements[]
	INIT(= N_("E1381: Interface cannot use \"implements\""));
EXTERN char e_variable_str_type_mismatch_expected_str_but_got_str[]
	INIT(= N_("E1382: Variable \"%s\": type mismatch, expected %s but got %s"));
EXTERN char e_method_str_type_mismatch_expected_str_but_got_str[]
	INIT(= N_("E1383: Method \"%s\": type mismatch, expected %s but got %s"));
EXTERN char e_class_method_str_accessible_only_inside_class_str[]
	INIT(= N_("E1384: Class method \"%s\" accessible only inside class \"%s\""));
EXTERN char e_class_method_str_accessible_only_using_class_str[]
	INIT(= N_("E1385: Class method \"%s\" accessible only using class \"%s\""));
EXTERN char e_object_method_str_accessible_only_using_object_str[]
	INIT(= N_("E1386: Object method \"%s\" accessible only using class \"%s\" object"));
EXTERN char e_public_variable_not_supported_in_interface[]
	INIT(= N_("E1387: Public variable not supported in an interface"));
EXTERN char e_public_keyword_not_supported_for_method[]
	INIT(= N_("E1388: Public keyword not supported for a method"));
EXTERN char e_missing_name_after_implements[]
	INIT(= N_("E1389: Missing name after implements"));
EXTERN char e_cannot_use_an_object_variable_except_with_the_new_method_str[]
	INIT(= N_("E1390: Cannot use an object variable \"this.%s\" except with the \"new\" method"));
EXTERN char e_cannot_lock_object_variable_str[]
	INIT(= N_("E1391: Cannot (un)lock variable \"%s\" in class \"%s\""));
EXTERN char e_cannot_lock_class_variable_str[]
	INIT(= N_("E1392: Cannot (un)lock class variable \"%s\" in class \"%s\""));
EXTERN char e_type_can_only_be_defined_in_vim9_script[]
	INIT(= N_("E1393: Type can only be defined in Vim9 script"));
EXTERN char e_type_name_must_start_with_uppercase_letter_str[]
	INIT(= N_("E1394: Type name must start with an uppercase letter: %s"));
EXTERN char e_cannot_modify_typealias[]
	INIT(= N_("E1395: Type alias \"%s\" cannot be modified"));
EXTERN char e_typealias_already_exists_for_str[]
	INIT(= N_("E1396: Type alias \"%s\" already exists"));
EXTERN char e_missing_typealias_name[]
	INIT(= N_("E1397: Missing type alias name"));
EXTERN char e_missing_typealias_type[]
	INIT(= N_("E1398: Missing type alias type"));
EXTERN char e_type_can_only_be_used_in_script[]
	INIT(= N_("E1399: Type can only be used in a script"));
EXTERN char e_using_typealias_as_number[]
	INIT(= N_("E1400: Using type alias \"%s\" as a Number"));
EXTERN char e_using_typealias_as_float[]
	INIT(= N_("E1401: Using type alias \"%s\" as a Float"));
EXTERN char e_using_typealias_as_string[]
	INIT(= N_("E1402: Using type alias \"%s\" as a String"));
EXTERN char e_using_typealias_as_value_str[]
	INIT(= N_("E1403: Type alias \"%s\" cannot be used as a value"));
EXTERN char e_abstract_cannot_be_used_in_interface[]
	INIT(= N_("E1404: Abstract cannot be used in an interface"));
EXTERN char e_using_class_as_value_str[]
	INIT(= N_("E1405: Class \"%s\" cannot be used as a value"));
EXTERN char e_using_class_as_var_val[]
	INIT(= N_("E1406: Cannot use a Class as a variable or value"));
EXTERN char e_using_typealias_as_var_val[]
	INIT(= N_("E1407: Cannot use a Typealias as a variable or value"));
EXTERN char e_final_variable_not_supported_in_interface[]
	INIT(= N_("E1408: Final variable not supported in an interface"));
EXTERN char e_cannot_change_readonly_variable_str_in_class_str[]
	INIT(= N_("E1409: Cannot change read-only variable \"%s\" in class \"%s\""));
EXTERN char e_const_variable_not_supported_in_interface[]
	INIT(= N_("E1410: Const variable not supported in an interface"));
EXTERN char e_missing_dot_after_object_str[]
	INIT(= N_("E1411: Missing dot after object \"%s\""));
#endif
// E1412 - E1499 unused (reserved for Vim9 class support)
EXTERN char e_cannot_mix_positional_and_non_positional_str[]
	INIT(= N_("E1500: Cannot mix positional and non-positional arguments: %s"));
EXTERN char e_fmt_arg_nr_unused_str[]
	INIT(= N_("E1501: format argument %d unused in $-style format: %s"));
EXTERN char e_positional_num_field_spec_reused_str_str[]
	INIT(= N_("E1502: Positional argument %d used as field width reused as different type: %s/%s"));
EXTERN char e_positional_nr_out_of_bounds_str[]
	INIT(= N_("E1503: Positional argument %d out of bounds: %s"));
EXTERN char e_positional_arg_num_type_inconsistent_str_str[]
	INIT(= N_("E1504: Positional argument %d type used inconsistently: %s/%s"));
EXTERN char e_invalid_format_specifier_str[]
	INIT(= N_("E1505: Invalid format specifier: %s"));
EXTERN char e_xattr_erange[]
	INIT(= N_("E1506: Buffer too small to copy xattr value or key"));
EXTERN char e_aptypes_is_null_nr_str[]
	INIT(= "E1507: Internal error: ap_types or ap_types[idx] is NULL: %d: %s");
EXTERN char e_xattr_e2big[]
	INIT(= N_("E1508: Size of the extended attribute value is larger than the maximum size allowed"));
EXTERN char e_xattr_other[]
	INIT(= N_("E1509: Error occurred when reading or writing extended attribute"));
EXTERN char e_val_too_large[]
	INIT(= N_("E1510: Value too large: %s"));
EXTERN char e_wrong_number_of_characters_for_field_str[]
	INIT(= N_("E1511: Wrong number of characters for field \"%s\""));
EXTERN char e_wrong_character_width_for_field_str[]
	INIT(= N_("E1512: Wrong character width for field \"%s\""));
