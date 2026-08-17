define_state_group!(script_data_double_escaped_states_group = {

    script_data_double_escaped_start_state {
        whitespace => ( --> script_data_double_escaped_state )
        b'/'       => ( --> script_data_double_escaped_state )
        b'>'       => ( --> script_data_double_escaped_state )
        eof        => ( emit_text_and_eof?; )
        _          => ( reconsume in script_data_escaped_state )
    }

    script_data_double_escaped_state {
        [ "--" ] => ( --> script_data_double_escaped_dash_dash_state )
        b'<'     => ( emit_text?; --> script_data_double_escaped_less_than_sign_state )
        eof      => ( emit_text_and_eof?; )
        _        => ()
    }

    script_data_double_escaped_dash_dash_state {
        b'-' => ()
        b'<' => ( --> #[inline] script_data_double_escaped_less_than_sign_state )
        b'>' => ( emit_text?; reconsume in script_data_state )
        eof  => ( emit_text_and_eof?; )
        _    => ( --> script_data_double_escaped_state )
    }

    script_data_double_escaped_less_than_sign_state {
        b'/' => ( --> script_data_double_escaped_end_tag_name_state )
        eof  => ( emit_text_and_eof?; )
        _    => ( reconsume in script_data_double_escaped_state )
    }

    script_data_double_escaped_end_tag_name_state {
        [ "SCRIPT"; ignore_case ] => ( --> script_data_double_escaped_end_state )
        eof                       => ( emit_text_and_eof?; )
        _                         => ( reconsume in script_data_double_escaped_state )
    }

    script_data_double_escaped_end_state {
        whitespace => ( --> script_data_escaped_state )
        b'/'       => ( --> script_data_escaped_state )
        b'>'       => ( --> script_data_escaped_state )
        eof        => ( emit_text_and_eof?; )
        _          => ( reconsume in script_data_double_escaped_state )
    }

});
