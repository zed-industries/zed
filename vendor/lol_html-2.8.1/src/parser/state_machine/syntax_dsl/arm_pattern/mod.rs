#[macro_use]
mod ch_sequence;

macro_rules! arm_pattern {
    ( | $cb_args:tt |>
         alpha => $actions:tt
    ) => {
        state_body!(@callback | $cb_args |> Some(b'a'..=b'z' | b'A'..=b'Z') => $actions);
    };

    ( | $cb_args:tt |>
        whitespace => $actions:tt
    ) => {
        state_body!(@callback | $cb_args |>
            Some(b' ' | b'\n' | b'\r' | b'\t' | b'\x0C') => $actions
        );
    };

    ( | [ [$self:tt, $ctx:tt, $input_chunk:ident, $ch:ident ], $($rest_cb_args:tt)+ ] |>
        closing_quote => $actions:tt
    ) => {
        state_body!(@callback | [ [$self, $ctx, $input_chunk, $ch], $($rest_cb_args)+ ] |>
            Some(ch) if ch == $self.closing_quote() => $actions
        );
    };


    ( | [ [$self:tt, $ctx:tt, $input:ident, $ch:ident ], $($rest_cb_args:tt)+ ] |>
        eoc => ( $($actions:tt)* )
    ) => {
        state_body!(@callback | [ [$self, $ctx, $input, $ch], $($rest_cb_args)+ ] |>
            None if !$self.is_last_input() => ({
                action_list!(|$self, $ctx, $input|> $($actions)* );

                return $self.break_on_end_of_input($input);
            })
        );
    };

    // NOTE: this arm is always enforced by the compiler to make match exhaustive,
    // so it's safe to break parsing loop here, since we don't have any input left
    // to parse. We execute EOF actions only if it's a last input, otherwise we just
    // break the parsing loop if it hasn't been done by the explicit EOC arm.
    ( | [ [$self:tt, $ctx:tt, $input:ident, $ch:ident ], $($rest_cb_args:tt)+ ] |>
        eof => ( $($actions:tt)* )
    ) => {
        state_body!(@callback | [ [$self, $ctx, $input, $ch], $($rest_cb_args)+ ] |>
            None => ({
                if $self.is_last_input() {
                    action_list!(|$self, $ctx, $input|> $($actions)* );
                }

                return $self.break_on_end_of_input($input);
            })
        );
    };

    ( | [ $scope_vars:tt, $($rest_cb_args:tt)+ ] |>
        [ $seq_pat:tt $(; $case_mod:ident)* ] => $actions:tt
    ) => {
        // NOTE: character sequence arm should be expanded in
        // place before we hit the character match block.
        ch_sequence_arm_pattern!(|$scope_vars|> $seq_pat, $actions, $($case_mod)* );
        state_body!(@callback | [ $scope_vars, $($rest_cb_args)+ ] |>);
    };

    ( | $cb_args:tt |> $pat:pat => $actions:tt ) => {
        state_body!(@callback | $cb_args |> Some($pat) => $actions);
    };
}
