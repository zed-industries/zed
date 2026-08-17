define_state_group!(rawtext_states_group = {

    rawtext_state {
        memchr(b'<') => ( emit_text?; mark_tag_start; --> #[inline] rawtext_less_than_sign_state )
        eoc  => ( emit_text?; )
        eof  => ( emit_text_and_eof?; )
    }

    rawtext_less_than_sign_state {
        b'/' => ( --> rawtext_end_tag_open_state )
        eof  => ( emit_text_and_eof?; )
        _    => ( unmark_tag_start; emit_text?; reconsume in rawtext_state )
    }

    rawtext_end_tag_open_state {
        alpha => ( create_end_tag; start_token_part; update_tag_name_hash; --> rawtext_end_tag_name_state )
        eof   => ( emit_text_and_eof?; )
        _     => ( unmark_tag_start; emit_text?; reconsume in rawtext_state )
    }

    rawtext_end_tag_name_state {
        whitespace => (
            if is_appropriate_end_tag
                ( finish_tag_name?; --> before_attribute_name_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in rawtext_state )
        )

        b'/' => (
            if is_appropriate_end_tag
                ( finish_tag_name?; --> self_closing_start_tag_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in rawtext_state )
        )

        b'>' => (
            if is_appropriate_end_tag
                ( finish_tag_name?; emit_tag?; --> dyn next_text_parsing_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in rawtext_state )
        )

        alpha => ( update_tag_name_hash; )
        eof   => ( emit_text_and_eof?; )
        _     => ( unmark_tag_start; emit_text?; reconsume in rawtext_state )
    }

});
