define_state_group!(comment_states_group = {
    #[cold]
    bogus_comment_state {
        memchr(b'>') => ( mark_comment_text_end; emit_current_token?; --> data_state )
        eof  => ( mark_comment_text_end; emit_current_token_and_eof?; )
    }

    comment_start_state <-- ( create_comment; start_token_part; ) {
        b'-' => ( mark_comment_text_end; --> #[inline] comment_start_dash_state )
        b'>' => ( mark_comment_text_end; emit_current_token?; --> data_state )
        eof  => ( reconsume in comment_state )
        _    => ( reconsume in comment_state )
    }

    comment_state {
        b'-' => ( mark_comment_text_end; --> #[inline] comment_end_dash_state )
        b'<' => ( --> comment_less_than_sign_state )
        eof  => ( mark_comment_text_end; emit_current_token_and_eof?; )
        _    => ( mark_comment_text_end; )
    }

    comment_start_dash_state {
        b'-' => ( --> comment_end_state )
        b'>' => ( emit_current_token?; --> data_state )
        eof  => ( emit_current_token_and_eof?; )
        _    => ( reconsume in comment_state )
    }

    comment_end_dash_state {
        b'-' => ( --> comment_end_state )
        eof  => ( emit_current_token_and_eof?; )
        _    => ( reconsume in comment_state )
    }

    comment_end_state {
        b'>' => ( emit_current_token?; --> data_state )
        b'!' => ( --> comment_end_bang_state )
        b'-' => ( shift_comment_text_end_by 1; )
        eof  => ( emit_current_token_and_eof?; )
        _    => ( shift_comment_text_end_by 2; reconsume in comment_state )
    }

    comment_less_than_sign_state {
        b'!' => ( mark_comment_text_end; --> #[inline] comment_less_than_sign_bang_state )
        b'<' => ( mark_comment_text_end; )
        eof  => ( mark_comment_text_end; reconsume in comment_state )
        _    => ( mark_comment_text_end; reconsume in comment_state )
    }

    comment_less_than_sign_bang_state {
        // careful! mark_comment_text_end ends the comment at the previous character! all of those
        // states run mark_comment_text_end because the only transition leading to this state (! in
        // comment_less_than_sign_state) "appends the current input character"
        b'-' => ( mark_comment_text_end; --> #[inline] comment_less_than_sign_bang_dash_state )
        eof  => ( mark_comment_text_end; reconsume in comment_state )
        _    => ( mark_comment_text_end; reconsume in comment_state )
    }

    comment_less_than_sign_bang_dash_state {
        b'-' => ( --> #[inline] comment_less_than_sign_bang_dash_dash_state )
        eof  => ( reconsume in comment_end_dash_state )
        _    => ( reconsume in comment_end_dash_state )
    }

    comment_less_than_sign_bang_dash_dash_state {
        eof  => ( reconsume in comment_end_state )
        _    => ( reconsume in comment_end_state )
    }

    comment_end_bang_state {
        b'-' => ( shift_comment_text_end_by 3; --> comment_end_dash_state )
        b'>' => ( emit_current_token?; --> data_state )
        eof  => ( emit_current_token_and_eof?; )
        _    => ( shift_comment_text_end_by 3; reconsume in comment_state )
    }

});
