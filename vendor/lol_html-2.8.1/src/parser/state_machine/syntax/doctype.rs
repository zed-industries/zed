define_state_group!(doctype_states_group = {

    doctype_state {
        whitespace => ( --> #[inline] before_doctype_name_state )
        b'>'       => ( create_doctype; set_force_quirks; emit_current_token?; --> data_state )
        eof        => ( create_doctype; set_force_quirks; emit_current_token_and_eof?; )
        _          => ( reconsume in before_doctype_name_state )
    }

    before_doctype_name_state {
        whitespace => ()
        b'>'       => ( create_doctype; set_force_quirks; emit_current_token?; --> data_state )
        eof        => ( create_doctype; set_force_quirks; emit_current_token_and_eof?; )
        _          => ( create_doctype; start_token_part; --> #[inline] doctype_name_state )
    }

    doctype_name_state {
        whitespace => ( finish_doctype_name; --> after_doctype_name_state )
        b'>'       => ( finish_doctype_name; emit_current_token?; --> data_state )
        eof        => ( finish_doctype_name; set_force_quirks; emit_current_token_and_eof?; )
        _          => ()
    }

    after_doctype_name_state {
        whitespace                => ()
        b'>'                      => ( emit_current_token?; --> data_state )
        eof                       => ( set_force_quirks; emit_current_token_and_eof?; )
        [ "PUBLIC"; ignore_case ] => ( --> after_doctype_public_keyword_state )
        [ "SYSTEM"; ignore_case ] => ( --> after_doctype_system_keyword_state )
        _                         => ( set_force_quirks; --> bogus_doctype_state )
    }

    after_doctype_public_keyword_state {
        whitespace => ( --> before_doctype_public_identifier_state )
        b'"'       => ( set_closing_quote_to_double; --> doctype_public_identifier_state )
        b'\''      => ( set_closing_quote_to_single; --> doctype_public_identifier_state )
        b'>'       => ( set_force_quirks; emit_current_token?; --> data_state )
        eof        => ( set_force_quirks; emit_current_token_and_eof?; )
        _          => ( set_force_quirks; --> bogus_doctype_state )
    }

    after_doctype_system_keyword_state {
        whitespace => ( --> before_doctype_system_identifier_state )
        b'"'       => ( set_closing_quote_to_double; --> doctype_system_identifier_state )
        b'\''      => ( set_closing_quote_to_single; --> doctype_system_identifier_state )
        b'>'       => ( set_force_quirks; emit_current_token?; --> data_state )
        eof        => ( set_force_quirks; emit_current_token_and_eof?; )
        _          => ( set_force_quirks; --> bogus_doctype_state )
    }

    before_doctype_public_identifier_state {
        whitespace => ()
        b'"'       => ( set_closing_quote_to_double; --> doctype_public_identifier_state )
        b'\''      => ( set_closing_quote_to_single; --> doctype_public_identifier_state )
        b'>'       => ( set_force_quirks; emit_current_token?; --> data_state )
        eof        => ( set_force_quirks; emit_current_token_and_eof?; )
        _          => ( set_force_quirks; --> bogus_doctype_state )
    }

    before_doctype_system_identifier_state {
        whitespace => ()
        b'"'       => ( set_closing_quote_to_double; --> doctype_system_identifier_state )
        b'\''      => ( set_closing_quote_to_single; --> doctype_system_identifier_state )
        b'>'       => ( set_force_quirks; emit_current_token?; --> data_state )
        eof        => ( set_force_quirks; emit_current_token_and_eof?; )
        _          => ( set_force_quirks; --> bogus_doctype_state )
    }

    doctype_public_identifier_state <-- ( start_token_part; ) {
        closing_quote => ( finish_doctype_public_id; --> after_doctype_public_identifier_state )
        b'>'          => ( finish_doctype_public_id; set_force_quirks; emit_current_token?; --> data_state )
        eof           => ( finish_doctype_public_id; set_force_quirks; emit_current_token_and_eof?; )
        _             => ()
    }

    doctype_system_identifier_state <-- ( start_token_part; ) {
        closing_quote => ( finish_doctype_system_id; --> after_doctype_system_identifier_state )
        b'>'          => ( finish_doctype_system_id; set_force_quirks; emit_current_token?; --> data_state )
        eof           => ( finish_doctype_system_id; set_force_quirks; emit_current_token_and_eof?; )
        _             => ()
    }

    after_doctype_public_identifier_state {
        whitespace => ( --> between_doctype_public_and_system_identifiers_state )
        b'>'       => ( emit_current_token?; --> data_state )
        b'"'       => ( set_closing_quote_to_double; --> doctype_system_identifier_state )
        b'\''      => ( set_closing_quote_to_single; --> doctype_system_identifier_state )
        eof        => ( set_force_quirks; emit_current_token_and_eof?; )
        _          => ( set_force_quirks; --> bogus_doctype_state )
    }

    after_doctype_system_identifier_state {
        whitespace => ()
        b'>'       => ( emit_current_token?; --> data_state )
        eof        => ( set_force_quirks; emit_current_token_and_eof?; )
        _          => ( --> bogus_doctype_state )
    }

    between_doctype_public_and_system_identifiers_state {
        whitespace => ()
        b'>'       => ( emit_current_token?; --> data_state )
        b'"'       => ( set_closing_quote_to_double; --> doctype_system_identifier_state )
        b'\''      => ( set_closing_quote_to_single; --> doctype_system_identifier_state )
        eof        => ( set_force_quirks; emit_current_token_and_eof?; )
        _          => ( set_force_quirks; --> bogus_doctype_state )
    }

    #[cold]
    bogus_doctype_state {
        memchr(b'>') => ( emit_current_token?; --> data_state )
        eof  => ( emit_current_token_and_eof?; )
    }

});
