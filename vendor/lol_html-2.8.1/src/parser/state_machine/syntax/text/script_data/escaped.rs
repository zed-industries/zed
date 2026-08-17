define_state_group!(script_data_escaped_states_group = {

    script_data_escape_start_state {
        [ "--" ] => ( --> script_data_escaped_dash_dash_state )
        eof      => ( emit_text_and_eof?; )
        _        => ( emit_text?; reconsume in script_data_state )
    }

    script_data_escaped_dash_dash_state {
        b'-' => ()
        b'<' => ( emit_text?; mark_tag_start; --> script_data_escaped_less_than_sign_state )
        b'>' => ( emit_text?; reconsume in script_data_state )
        eof  => ( emit_text_and_eof?; )
        _    => ( --> script_data_escaped_state )
    }

    script_data_escaped_state {
        [ "--" ] => ( --> script_data_escaped_dash_dash_state )
        b'<'     => ( emit_text?; mark_tag_start; --> script_data_escaped_less_than_sign_state )
        eof      => ( emit_text_and_eof?; )
        _        => ()
    }

    script_data_escaped_less_than_sign_state {
        [ "SCRIPT"; ignore_case ] => ( unmark_tag_start; --> script_data_double_escaped_start_state )
        b'/'                      => ( --> #[inline] script_data_escaped_end_tag_open_state )
        eof                       => ( emit_text_and_eof?; )
        _                         => ( unmark_tag_start; emit_text?; reconsume in script_data_escaped_state )
    }

    script_data_escaped_end_tag_open_state {
        alpha => ( create_end_tag; start_token_part; update_tag_name_hash; --> script_data_escaped_end_tag_name_state )
        eof   => ( emit_text_and_eof?; )
        _     => ( unmark_tag_start; emit_text?; reconsume in script_data_escaped_state )
    }

    script_data_escaped_end_tag_name_state {
        whitespace => (
            if is_appropriate_end_tag
                ( finish_tag_name?; --> before_attribute_name_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in script_data_escaped_state )
        )

        b'/' => (
            if is_appropriate_end_tag
                ( finish_tag_name?; --> self_closing_start_tag_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in script_data_escaped_state )
        )

        b'>' => (
            if is_appropriate_end_tag
                ( finish_tag_name?; emit_tag?; --> dyn next_text_parsing_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in script_data_escaped_state )
        )

        alpha => ( update_tag_name_hash; )
        eof   => ( emit_text_and_eof?; )
        _     => ( emit_text?; reconsume in script_data_escaped_state )
    }

});
