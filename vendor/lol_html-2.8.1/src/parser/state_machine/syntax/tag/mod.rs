#[macro_use]
mod attributes;

define_state_group!(tag_states_group = {

    tag_open_state {
        alpha => ( create_start_tag; start_token_part; update_tag_name_hash; --> #[inline] tag_name_state )
        b'!'  => ( unmark_tag_start; --> markup_declaration_open_state )
        b'/'  => ( --> #[inline] end_tag_open_state )
        b'?'  => ( unmark_tag_start; create_comment; start_token_part; --> bogus_comment_state )
        eof   => ( emit_text_and_eof?; )
        _     => ( unmark_tag_start; emit_text?; reconsume in data_state )
    }

    end_tag_open_state {
        alpha => ( create_end_tag; start_token_part; update_tag_name_hash; --> #[inline] tag_name_state )
        b'>'  => ( unmark_tag_start; emit_raw_without_token?; --> data_state )
        eof   => ( emit_text_and_eof?; )
        _     => ( create_comment; start_token_part; reconsume in bogus_comment_state )
    }

    markup_declaration_open_state <-- ( start_token_part; ) {
        [ "--" ]                   => ( --> comment_start_state )
        [ "DOCTYPE"; ignore_case ] => ( --> doctype_state )

        [ "[CDATA[" ] => (
            if cdata_allowed
                ( emit_raw_without_token?; enter_cdata; --> cdata_section_state )
            else
                ( create_comment; --> bogus_comment_state )
        )

        eof => ( create_comment; reconsume in bogus_comment_state )
        _   => ( create_comment; reconsume in bogus_comment_state )
    }

    tag_name_state {
        whitespace => ( finish_tag_name?; --> before_attribute_name_state )
        b'>'       => ( finish_tag_name?; emit_tag?; --> dyn next_text_parsing_state )
        b'/'       => ( finish_tag_name?; --> self_closing_start_tag_state )
        eof        => ( emit_raw_without_token_and_eof?; )
        _          => ( update_tag_name_hash; )
    }

    self_closing_start_tag_state {
        b'>' => ( mark_as_self_closing; emit_tag?; --> dyn next_text_parsing_state )
        eof  => ( emit_raw_without_token_and_eof?; )
        _    => ( reconsume in before_attribute_name_state )
    }
});
