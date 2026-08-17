define_state_group!(data_states_group = {

    data_state  {
        memchr(b'<') => ( emit_text?; mark_tag_start; --> #[inline] tag_open_state )
        eoc  => ( emit_text?; )
        eof  => ( emit_text_and_eof?; )
    }

});
