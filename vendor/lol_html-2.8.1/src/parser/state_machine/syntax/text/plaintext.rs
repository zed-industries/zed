define_state_group!(plaintext_states_group = {

    #[cold]
    plaintext_state {
        eoc => ( emit_text?; )
        eof => ( emit_text_and_eof?; )
        _   => ()
    }

});
