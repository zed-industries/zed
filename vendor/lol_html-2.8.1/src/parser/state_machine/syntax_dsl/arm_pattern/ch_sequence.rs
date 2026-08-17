macro_rules! ch_sequence_arm_pattern {

    // Sequences
    //--------------------------------------------------------------------
    ( | $scope_vars:tt |> "--", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'-', b'-' ], $($rest_args)*
        );
    };

    ( | $scope_vars:tt |> "]>", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b']', b'>' ], $($rest_args)*
        );
    };

    ( | $scope_vars:tt |> "DOCTYPE", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'D', b'O', b'C', b'T', b'Y', b'P', b'E' ], $($rest_args)*
        );
    };

    ( | $scope_vars:tt |> "[CDATA[", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'[', b'C', b'D', b'A', b'T', b'A', b'[' ], $($rest_args)*
        );
    };

    ( | $scope_vars:tt |> "PUBLIC", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'P', b'U', b'B', b'L', b'I', b'C' ], $($rest_args)*
        );
    };

    ( | $scope_vars:tt |> "SYSTEM", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'S', b'Y', b'S', b'T', b'E', b'M' ], $($rest_args)*
        );
    };

    ( | $scope_vars:tt |> "SCRIPT", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'S', b'C', b'R', b'I', b'P', b'T' ], $($rest_args)*
        );
    };

    // Character comparison expression
    //--------------------------------------------------------------------
    ( @cmp_exp $ch:ident, $exp_ch:expr ) => ( $ch == $exp_ch );
    ( @cmp_exp $ch:ident, $exp_ch:expr, ignore_case ) => ( $ch == $exp_ch || $ch == $exp_ch ^ 0x20 );


    // Match block expansion
    //--------------------------------------------------------------------
    ( @match_block
        | [$self:tt, $ctx:tt, $input:ident, $ch:ident] |> $exp_ch:expr, $body:tt, $($case_mod:ident)*
    ) => {
        match $ch {
            Some(ch) if ch_sequence_arm_pattern!(@cmp_exp ch, $exp_ch $(, $case_mod)*) => {
               $body
            },
            None if !$self.is_last_input() => {
                return $self.break_on_end_of_input($input);
            },
            _ => $self.leave_ch_sequence_matching(),
        }
    };

    // Expand check for the first character
    //--------------------------------------------------------------------
    ( @first | [$self:tt, $ctx:tt, $input:ident, $ch:ident] |>
        [ $exp_ch:expr, $($rest_chs:tt)* ], $actions:tt, $($case_mod:ident)*
    ) => {
        $self.enter_ch_sequence_matching();
        ch_sequence_arm_pattern!(@match_block |[$self, $ctx, $input, $ch]|> $exp_ch, {
            ch_sequence_arm_pattern!(
                @iter |[$self, $ctx, $input, $ch]|> 1, [ $($rest_chs)* ], $actions, $($case_mod)*
            );
        }, $($case_mod)*);
    };


    // Recursively expand checks for the remaining characters
    //--------------------------------------------------------------------
    ( @iter | [$self:tt, $ctx:tt, $input:ident, $ch:ident] |>
        $depth:expr, [ $exp_ch:expr, $($rest_chs:tt)* ], $actions:tt, $($case_mod:ident)*
    ) => {{
        let ch = $self.lookahead($input, $depth);

        ch_sequence_arm_pattern!(@match_block |[$self, $ctx, $input, ch]|> $exp_ch, {
            ch_sequence_arm_pattern!(
                @iter |[$self, $ctx, $input, $ch]|> $depth + 1, [ $($rest_chs)* ], $actions, $($case_mod)*
            );
        }, $($case_mod)*);
    }};

    // NOTE: end of recursion
    ( @iter | [$self:tt, $ctx:tt, $input:ident, $ch:ident] |>
        $depth:expr, [$exp_ch:expr], ( $($actions:tt)* ), $($case_mod:ident)*
    ) => {{
        let ch = $self.lookahead($input, $depth);

        ch_sequence_arm_pattern!(@match_block |[$self, $ctx, $input, ch]|> $exp_ch, {
            $self.consume_several($depth);
            $self.leave_ch_sequence_matching();
            action_list!(|$self, $ctx, $input|> $($actions)*);

            // NOTE: this may be unreachable on expansion, e.g. if
            // we have state transition in the action list.
            #[allow(unreachable_code)] { continue; }
        }, $($case_mod)*);
    }};
}
