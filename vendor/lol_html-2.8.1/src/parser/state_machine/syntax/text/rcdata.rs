define_state_group!(rcdata_states_group = {

    rcdata_state {
        memchr(b'<') => ( emit_text?; mark_tag_start; --> rcdata_less_than_sign_state )
        eoc  => ( emit_text?; )
        eof  => ( emit_text_and_eof?; )
    }

    rcdata_less_than_sign_state {
        b'/' => ( --> rcdata_end_tag_open_state )
        eof  => ( emit_text_and_eof?; )
        _    => ( unmark_tag_start; emit_text?; reconsume in rcdata_state )
    }

    rcdata_end_tag_open_state {
        alpha => ( create_end_tag; start_token_part; update_tag_name_hash; --> rcdata_end_tag_name_state )
        eof   => ( emit_text_and_eof?; )
        _     => ( unmark_tag_start; emit_text?; reconsume in rcdata_state )
    }

    rcdata_end_tag_name_state {
        whitespace => (
            if is_appropriate_end_tag
                ( finish_tag_name?; --> before_attribute_name_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in rcdata_state )
        )

        b'/' => (
            if is_appropriate_end_tag
                ( finish_tag_name?; --> self_closing_start_tag_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in rcdata_state )
        )

        b'>' => (
            if is_appropriate_end_tag
                ( finish_tag_name?; emit_tag?; --> dyn next_text_parsing_state )
            else
                ( unmark_tag_start; emit_text?; reconsume in rcdata_state )
        )

        alpha => ( update_tag_name_hash; )
        eof   => ( emit_text_and_eof?; )
        _     => ( unmark_tag_start; emit_text?; reconsume in rcdata_state )
    }

});
