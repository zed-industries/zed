define_state_group!(cdata_section_states_group = {

    #[cold]
    cdata_section_state {
        memchr(b']') => ( emit_text?; --> #[inline] cdata_section_bracket_state )
        eoc  => ( emit_text?; )
        eof  => ( emit_text_and_eof?; )
    }

    cdata_section_bracket_state {
        [ "]>" ] => ( emit_raw_without_token?; leave_cdata; --> data_state )
        eof      => ( emit_text_and_eof?; )
        _        => ( emit_text?; reconsume in cdata_section_state )
    }
});
